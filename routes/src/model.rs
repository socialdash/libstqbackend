extern crate juniper;
extern crate std;

use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

pub enum Model {
    User,
    JWT,
    Store,
    Product,
    BaseProduct,
    UserRoles,
    Attribute,
    Category,
    CartProduct,
    CartStore,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::User => write!(f, "user"),
            Model::JWT => write!(f, "jwt"),
            Model::Store => write!(f, "store"),
            Model::Product => write!(f, "product"),
            Model::BaseProduct => write!(f, "base_product"),
            Model::UserRoles => write!(f, "user_roles"),
            Model::Attribute => write!(f, "attribute"),
            Model::Category => write!(f, "category"),
            Model::CartProduct => write!(f, "cart_product"),
            Model::CartStore => write!(f, "cart_store"),
        }
    }
}

impl FromStr for Model {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Model::User),
            "jwt" => Ok(Model::JWT),
            "store" => Ok(Model::Store),
            "product" => Ok(Model::Product),
            "base_product" => Ok(Model::BaseProduct),
            "user_roles" => Ok(Model::UserRoles),
            "attribute" => Ok(Model::Attribute),
            "category" => Ok(Model::Category),
            "cart_product" => Ok(Model::CartProduct),
            "cart_store" => Ok(Model::CartStore),
            _ => Err(FieldError::new(
                "Unknown model",
                graphql_value!({ "code": 300, "details": {
                        format!("Can not resolve model name. Unknown model: '{}'", s)
                        }}),
            )),
        }
    }
}

impl Model {
    pub fn to_url(&self) -> String {
        match *self {
            Model::User => "users".to_string(),
            Model::JWT => "jwt".to_string(),
            Model::Store => "stores".to_string(),
            Model::Product => "products".to_string(),
            Model::BaseProduct => "base_products".to_string(),
            Model::UserRoles => "user_roles".to_string(),
            Model::Attribute => "attributes".to_string(),
            Model::Category => "categories".to_string(),
            Model::CartProduct => "cart_products".to_string(),
            Model::CartStore => "cart_store".to_string(),
        }
    }
}
