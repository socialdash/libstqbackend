//! This crate provides common ACL facilities, namely the common groups and traits.
#[macro_use]
extern crate failure;
extern crate futures;

use futures::future;
use futures::prelude::*;

pub type Verdict<Context, E> = Box<Future<Item = (bool, Context), Error = (E, Context)>>;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Unauthorized")]
pub struct UnauthorizedError;

/// Access control layer for repos. It tells if a user can do a certain action with
/// certain resource. All logic for roles and permissions should be hardcoded into implementation
/// of this trait.
pub trait AclEngine<Context, Error>
where
    Context: 'static,
    Error: From<UnauthorizedError> + 'static,
{
    /// Tells if a user with id `user_id` can do `action` on `resource`.
    /// `resource_with_scope` can tell if this resource is in some scope, which is also a part of `acl` for some
    /// permissions. E.g. You can say that a user can do `Create` (`Action`) on `Store` (`Resource`) only if he's the
    /// `Owner` (`Scope`) of the store.
    fn allows(&self, ctx: Context) -> Verdict<Context, Error>;

    fn ensure_access(&self, ctx: Context) -> Box<Future<Item = Context, Error = (Error, Context)>> {
        Box::new(self.allows(ctx).and_then(|(allowed, ctx)| {
            future::result(if allowed {
                Ok(ctx)
            } else {
                Err((Error::from(UnauthorizedError), ctx))
            })
        }))
    }
}

impl<Context, Error, T> AclEngine<Context, Error> for T
where
    Context: 'static,
    Error: From<UnauthorizedError> + 'static,
    T: Fn(Context) -> Verdict<Context, Error>,
{
    fn allows(&self, ctx: Context) -> Verdict<Context, Error> {
        (self)(ctx)
    }
}

/// `SystemACL` allows all manipulation with resources in all cases.
#[derive(Clone, Debug, Default)]
pub struct SystemACL;

#[allow(unused)]
impl<Context, Error> AclEngine<Context, Error> for SystemACL
where
    Context: 'static,
    Error: From<UnauthorizedError> + 'static,
{
    fn allows(&self, ctx: Context) -> Verdict<Context, Error> {
        Box::new(future::ok((true, ctx)))
    }
}

/// `ForbiddenACL` denies all manipulation with resources in all cases.
#[derive(Clone, Debug, Default)]
pub struct ForbiddenACL;

#[allow(unused)]
impl<Context, Error> AclEngine<Context, Error> for ForbiddenACL
where
    Context: 'static,
    Error: From<UnauthorizedError> + 'static,
{
    fn allows(&self, ctx: Context) -> Verdict<Context, Error> {
        Box::new(future::ok((false, ctx)))
    }
}
