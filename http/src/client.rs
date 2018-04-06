use std::fmt;
use std::mem;

use futures::future;
use futures::prelude::*;
use futures::sync::{mpsc, oneshot};
use hyper;
use hyper::header::{Authorization, Headers};
use hyper_tls::HttpsConnector;
use juniper::FieldError;
use serde::de::Deserialize;
use serde_json;
use tokio_core::reactor::Handle;

use errors::ErrorMessage;

pub type ClientResult = Result<String, Error>;
pub type HyperClient = hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;

pub struct Config {
    pub http_client_retries: usize,
    pub http_client_buffer_size: usize,
}

pub struct Client {
    client: HyperClient,
    tx: mpsc::Sender<Payload>,
    rx: mpsc::Receiver<Payload>,
    max_retries: usize,
}

impl Client {
    pub fn new(config: &Config, handle: &Handle) -> Self {
        let max_retries = config.http_client_retries;
        let (tx, rx) = mpsc::channel::<Payload>(config.http_client_buffer_size);
        let client = hyper::Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(handle);

        Client {
            client,
            tx,
            rx,
            max_retries,
        }
    }

    pub fn stream(self) -> Box<Stream<Item = (), Error = ()>> {
        let Self { client, rx, .. } = self;

        Box::new(rx.and_then(move |payload| Self::send_request(&client, payload).map(|_| ()).map_err(|_| ())))
    }

    pub fn handle(&self) -> ClientHandle {
        ClientHandle {
            tx: self.tx.clone(),
            max_retries: self.max_retries,
        }
    }

    fn send_request(client: &HyperClient, payload: Payload) -> Box<Future<Item = (), Error = ()>> {
        let Payload {
            url,
            method,
            body: maybe_body,
            headers: maybe_headers,
            callback,
        } = payload;

        let uri = match url.parse() {
            Ok(val) => val,
            Err(err) => {
                error!("Url `{}` passed to http client cannot be parsed: `{}`", url, err);
                return Box::new(
                    callback
                        .send(Err(Error::Parse(format!("Cannot parse url `{}`", url))))
                        .into_future()
                        .map(|_| ())
                        .map_err(|_| ()),
                );
            }
        };
        let mut req = hyper::Request::new(method, uri);

        if let Some(headers) = maybe_headers {
            mem::replace(req.headers_mut(), headers);
        }

        for body in maybe_body.iter() {
            req.set_body(body.clone());
        }

        let task = client
            .request(req)
            .map_err(Error::Network)
            .and_then(move |res| {
                let status = res.status();
                let body_future: Box<Future<Item = String, Error = Error>> = Box::new(Self::read_body(res.body()).map_err(Error::Network));
                match status.as_u16() {
                    200...299 => body_future,

                    _ => Box::new(body_future.and_then(move |body| {
                        let message = serde_json::from_str::<ErrorMessage>(&body).ok();
                        let error = Error::Api(status, message.or(Some(ErrorMessage { code: 422, message: body })));
                        future::err(error)
                    })),
                }
            })
            .then(|result| callback.send(result))
            .map(|_| ())
            .map_err(|_| ());

        Box::new(task)
    }

    fn read_body(body: hyper::Body) -> Box<Future<Item = String, Error = hyper::Error> + Send> {
        Box::new(body.fold(Vec::new(), |mut acc, chunk| {
            acc.extend_from_slice(&*chunk);
            future::ok::<_, hyper::Error>(acc)
        }).and_then(|bytes| match String::from_utf8(bytes) {
            Ok(data) => future::ok(data),
            Err(err) => future::err(hyper::Error::Utf8(err.utf8_error())),
        }))
    }
}

#[derive(Clone)]
pub struct ClientHandle {
    tx: mpsc::Sender<Payload>,
    max_retries: usize,
}

impl ClientHandle {
    pub fn request_with_auth_header<T>(
        &self,
        method: hyper::Method,
        url: String,
        body: Option<String>,
        auth_data: Option<String>,
    ) -> Box<Future<Item = T, Error = Error> + Send>
    where
        T: for<'a> Deserialize<'a> + 'static + Send,
    {
        let headers = auth_data.and_then(|s| {
            let mut headers = Headers::new();
            headers.set(Authorization(s));
            Some(headers)
        });
        self.request(method, url, body, headers)
    }

    pub fn request<T>(
        &self,
        method: hyper::Method,
        url: String,
        body: Option<String>,
        headers: Option<Headers>,
    ) -> Box<Future<Item = T, Error = Error> + Send>
    where
        T: for<'a> Deserialize<'a> + 'static + Send,
    {
        Box::new(
            self.send_request_with_retries(method, url, body, headers, None, self.max_retries)
                .and_then(|response| serde_json::from_str::<T>(&response).map_err(|err| Error::Parse(format!("{}", err)))),
        )
    }

    fn send_request_with_retries(
        &self,
        method: hyper::Method,
        url: String,
        body: Option<String>,
        headers: Option<Headers>,
        last_err: Option<Error>,
        retries: usize,
    ) -> Box<Future<Item = String, Error = Error> + Send> {
        if retries == 0 {
            let error = last_err.unwrap_or_else(|| Error::Unknown("Unexpected missing error in send_request_with_retries".to_string()));
            Box::new(future::err(error))
        } else {
            let self_clone = self.clone();
            let method_clone = method.clone();
            let body_clone = body.clone();
            let url_clone = url.clone();
            let headers_clone = headers.clone();
            Box::new(self.send_request(method, url, body, headers).or_else(move |err| match err {
                Error::Network(err) => {
                    warn!(
                        "Failed to fetch `{}` with error `{}`, retrying... Retries left {}",
                        url_clone, err, retries
                    );
                    self_clone.send_request_with_retries(
                        method_clone,
                        url_clone,
                        body_clone,
                        headers_clone,
                        Some(Error::Network(err)),
                        retries - 1,
                    )
                }
                _ => Box::new(future::err(err)),
            }))
        }
    }

    fn send_request(
        &self,
        method: hyper::Method,
        url: String,
        body: Option<String>,
        headers: Option<hyper::Headers>,
    ) -> Box<Future<Item = String, Error = Error> + Send> {
        info!(
            "Starting outbound http request: {} {} with body {} and headers {}",
            method,
            url,
            body.clone().unwrap_or_default(),
            headers.clone().unwrap_or_default()
        );
        let url_clone = url.clone();
        let method_clone = method.clone();

        let (tx, rx) = oneshot::channel::<ClientResult>();
        let payload = Payload {
            url,
            method,
            body,
            headers,
            callback: tx,
        };

        let future = self.tx
            .clone()
            .send(payload)
            .map_err(|err| Error::Unknown(format!("Unexpected error sending http client request params to channel: {}", err)))
            .and_then(|_| {
                rx.map_err(|err| Error::Unknown(format!("Unexpected error receiving http client response from channel: {}", err)))
            })
            .and_then(|result| result)
            .map_err(move |err| {
                error!("{} {} : {}", method_clone, url_clone, err);
                err
            });

        Box::new(future)
    }
}

struct Payload {
    pub url: String,
    pub method: hyper::Method,
    pub body: Option<String>,
    pub headers: Option<hyper::Headers>,
    pub callback: oneshot::Sender<ClientResult>,
}

#[derive(Debug, Fail)]
pub enum Error {
    Api(hyper::StatusCode, Option<ErrorMessage>),
    Network(hyper::Error),
    Parse(String),
    Unknown(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Api(ref status, Some(ref error_message)) => write!(
                f,
                "Http client 100: Api error: status: {}, code: {}, message: {}",
                status, error_message.code, error_message.message
            ),
            Error::Api(status, None) => write!(f, "Http client 100: Api error: status: {}", status),
            Error::Network(ref err) => write!(f, "Http client 200: Network error: {}", err),
            Error::Parse(ref err) => write!(f, "Http client 300: Parse error: {}", err),
            Error::Unknown(ref err) => write!(f, "Http client 400: Unknown error: {}", err),
        }
    }
}

impl Error {
    pub fn into_graphql(self) -> FieldError {
        match self {
            Error::Api(status, Some(ErrorMessage { code, message })) => {
                let code = code.to_string();
                let status = status.to_string();
                FieldError::new(
                    "Error response from microservice",
                    graphql_value!({ "code": 100, "details": {"status": status, "code": code, "message": message }}),
                )
            }
            Error::Api(status, None) => {
                let status = status.to_string();
                FieldError::new(
                    "Error response from microservice",
                    graphql_value!({ "code": 100, "details": { "status": status }}),
                )
            }
            Error::Network(_) => FieldError::new(
                "Network error for microservice",
                graphql_value!({ "code": 200, "details": { "See server logs for details." }}),
            ),
            Error::Parse(message) => FieldError::new("Unexpected parsing error", graphql_value!({ "code": 300, "details": { message }})),
            _ => FieldError::new(
                "Unknown error for microservice",
                graphql_value!({ "code": 400, "details": { "See server logs for details." }}),
            ),
        }
    }
}
