extern crate juniper;
extern crate std;

use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

#[derive(Clone, Copy, Debug, EnumIterator, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Model {
    Attribute,
    AttributeValue,
    AvailablePackageForUser,
    CustomAttribute,
    BaseProduct,
    Cart,
    CartProduct,
    CartStore,
    Category,
    Company,
    CompanyPackage,
    Country,
    Coupon,
    JWT,
    ModeratorProductComment,
    ModeratorStoreComment,
    Order,
    Package,
    Product,
    Role,
    SearchCategory,
    ShippingRates,
    Stock,
    Store,
    User,
    UserDeliveryAddress,
    UserRoles,
    Warehouse,
    WizardStore,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Model::Attribute => "attribute",
                Model::AttributeValue => "attribute_value",
                Model::AvailablePackageForUser => "available_package_for_user",
                Model::CustomAttribute => "custom_attribute",
                Model::BaseProduct => "base_product",
                Model::Cart => "cart",
                Model::CartProduct => "cart_product",
                Model::CartStore => "cart_store",
                Model::Category => "category",
                Model::Company => "company",
                Model::CompanyPackage => "company_package",
                Model::Country => "country",
                Model::Coupon => "coupon",
                Model::JWT => "jwt",
                Model::ModeratorProductComment => "moderator_product_comment",
                Model::ModeratorStoreComment => "moderator_store_comment",
                Model::Order => "order",
                Model::Package => "package",
                Model::Product => "product",
                Model::Role => "role",
                Model::SearchCategory => "search_category",
                Model::ShippingRates => "shipping_rates",
                Model::Stock => "stock",
                Model::Store => "store",
                Model::User => "user",
                Model::UserDeliveryAddress => "user_delivery_address",
                Model::UserRoles => "user_roles",
                Model::Warehouse => "warehouse",
                Model::WizardStore => "wizard_store",
            }
        )
    }
}

impl FromStr for Model {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "attribute" => Ok(Model::Attribute),
            "attribute_value" => Ok(Model::AttributeValue),
            "available_package_for_user" => Ok(Model::AvailablePackageForUser),
            "custom_attribute" => Ok(Model::CustomAttribute),
            "base_product" => Ok(Model::BaseProduct),
            "cart_product" => Ok(Model::CartProduct),
            "cart_store" => Ok(Model::CartStore),
            "cart" => Ok(Model::Cart),
            "category" => Ok(Model::Category),
            "company_package" => Ok(Model::CompanyPackage),
            "company" => Ok(Model::Company),
            "country" => Ok(Model::Country),
            "coupon" => Ok(Model::Coupon),
            "jwt" => Ok(Model::JWT),
            "moderator_product_comment" => Ok(Model::ModeratorProductComment),
            "moderator_store_comment" => Ok(Model::ModeratorStoreComment),
            "order" => Ok(Model::Order),
            "package" => Ok(Model::Package),
            "product" => Ok(Model::Product),
            "role" => Ok(Model::Role),
            "search_category" => Ok(Model::SearchCategory),
            "shipping_rates" => Ok(Model::ShippingRates),
            "stock" => Ok(Model::Stock),
            "store" => Ok(Model::Store),
            "user_delivery_address" => Ok(Model::UserDeliveryAddress),
            "user_roles" => Ok(Model::UserRoles),
            "user" => Ok(Model::User),
            "warehouse" => Ok(Model::Warehouse),
            "wizard_store" => Ok(Model::WizardStore),
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
            Model::Attribute => "attributes".to_string(),
            Model::AttributeValue => "values".to_string(),
            Model::AvailablePackageForUser => "available_packages_for_user".to_string(),
            Model::CustomAttribute => "custom_attributes".to_string(),
            Model::BaseProduct => "base_products".to_string(),
            Model::Cart => "cart".to_string(),
            Model::CartProduct => "cart_products".to_string(),
            Model::CartStore => "cart_store".to_string(),
            Model::Category => "categories".to_string(),
            Model::Company => "companies".to_string(),
            Model::CompanyPackage => "companies_packages".to_string(),
            Model::Country => "countries".to_string(),
            Model::Coupon => "coupons".to_string(),
            Model::JWT => "jwt".to_string(),
            Model::ModeratorProductComment => "moderator_product_comments".to_string(),
            Model::ModeratorStoreComment => "moderator_store_comments".to_string(),
            Model::Order => "orders".to_string(),
            Model::Package => "packages".to_string(),
            Model::Product => "products".to_string(),
            Model::Role => "roles".to_string(),
            Model::SearchCategory => "search_category".to_string(),
            Model::ShippingRates => "shipping_rates".to_string(),
            Model::Stock => "stocks".to_string(),
            Model::Store => "stores".to_string(),
            Model::User => "users".to_string(),
            Model::UserDeliveryAddress => "user_delivery_address".to_string(),
            Model::UserRoles => "user_roles".to_string(),
            Model::Warehouse => "warehouses".to_string(),
            Model::WizardStore => "wizard_stores".to_string(),
        }
    }
}
