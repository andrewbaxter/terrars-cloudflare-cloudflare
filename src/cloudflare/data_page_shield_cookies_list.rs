#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataPageShieldCookiesListData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_page: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    same_site: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataPageShieldCookiesList_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPageShieldCookiesListData_>,
}

#[derive(Clone)]
pub struct DataPageShieldCookiesList(std::rc::Rc<DataPageShieldCookiesList_>);

impl DataPageShieldCookiesList {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc =
        "Set the field `direction`.\nThe direction used to sort returned cookies.'\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\nFilters the returned cookies that match the specified domain attribute"]
    pub fn set_domain(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().domain = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `export`.\nExport the list of cookies as a file, limited to 50000 entries.\nAvailable values: \"csv\"."]
    pub fn set_export(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().export = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hosts`.\nIncludes cookies that match one or more URL-encoded hostnames separated by commas.\n\nWildcards are supported at the start and end of each hostname to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn set_hosts(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_only`.\nFilters the returned cookies that are set with HttpOnly"]
    pub fn set_http_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().http_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nFilters the returned cookies that match the specified name.\nWildcards are supported at the start and end to support starts with, ends with\nand contains. e.g. session*"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order_by`.\nThe field used to sort returned cookies.\nAvailable values: \"first_seen_at\", \"last_seen_at\"."]
    pub fn set_order_by(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order_by = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `page`.\nThe current page number of the paginated results.\n\nWe additionally support a special value \"all\". When \"all\" is used, the API will return all the cookies\nwith the applied filters in a single page. This feature is best-effort and it may only work for zones with\na low number of cookies"]
    pub fn set_page(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().page = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `page_url`.\nIncludes connections that match one or more page URLs (separated by commas) where they were last seen\n\nWildcards are supported at the start and end of each page URL to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn set_page_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().page_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nFilters the returned cookies that match the specified path attribute"]
    pub fn set_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_page`.\nThe number of results per page."]
    pub fn set_per_page(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().per_page = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `same_site`.\nFilters the returned cookies that match the specified same_site attribute\nAvailable values: \"lax\", \"strict\", \"none\"."]
    pub fn set_same_site(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().same_site = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secure`.\nFilters the returned cookies that are set with Secure"]
    pub fn set_secure(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().secure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nFilters the returned cookies that match the specified type attribute\nAvailable values: \"first_party\", \"unknown\"."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier"]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe direction used to sort returned cookies.'\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nFilters the returned cookies that match the specified domain attribute"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `export` after provisioning.\nExport the list of cookies as a file, limited to 50000 entries.\nAvailable values: \"csv\"."]
    pub fn export(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nIncludes cookies that match one or more URL-encoded hostnames separated by commas.\n\nWildcards are supported at the start and end of each hostname to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn hosts(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `http_only` after provisioning.\nFilters the returned cookies that are set with HttpOnly"]
    pub fn http_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFilters the returned cookies that match the specified name.\nWildcards are supported at the start and end to support starts with, ends with\nand contains. e.g. session*"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nThe field used to sort returned cookies.\nAvailable values: \"first_seen_at\", \"last_seen_at\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page` after provisioning.\nThe current page number of the paginated results.\n\nWe additionally support a special value \"all\". When \"all\" is used, the API will return all the cookies\nwith the applied filters in a single page. This feature is best-effort and it may only work for zones with\na low number of cookies"]
    pub fn page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page_url` after provisioning.\nIncludes connections that match one or more page URLs (separated by commas) where they were last seen\n\nWildcards are supported at the start and end of each page URL to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nFilters the returned cookies that match the specified path attribute"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nThe number of results per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataPageShieldCookiesList_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `same_site` after provisioning.\nFilters the returned cookies that match the specified same_site attribute\nAvailable values: \"lax\", \"strict\", \"none\"."]
    pub fn same_site(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.same_site", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secure` after provisioning.\nFilters the returned cookies that are set with Secure"]
    pub fn secure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nFilters the returned cookies that match the specified type attribute\nAvailable values: \"first_party\", \"unknown\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataPageShieldCookiesList {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPageShieldCookiesList { }

impl terrars::ToListMappable for DataPageShieldCookiesList {
    type O = terrars::ListRef<DataPageShieldCookiesListRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPageShieldCookiesList_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_page_shield_cookies_list".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPageShieldCookiesList {
    pub tf_id: std::string::String,
}

impl BuildDataPageShieldCookiesList {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPageShieldCookiesList {
        let out = DataPageShieldCookiesList(std::rc::Rc::new(DataPageShieldCookiesList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPageShieldCookiesListData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                direction: core::default::Default::default(),
                domain: core::default::Default::default(),
                export: core::default::Default::default(),
                hosts: core::default::Default::default(),
                http_only: core::default::Default::default(),
                max_items: core::default::Default::default(),
                name: core::default::Default::default(),
                order_by: core::default::Default::default(),
                page: core::default::Default::default(),
                page_url: core::default::Default::default(),
                path: core::default::Default::default(),
                per_page: core::default::Default::default(),
                same_site: core::default::Default::default(),
                secure: core::default::Default::default(),
                type_: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPageShieldCookiesListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPageShieldCookiesListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPageShieldCookiesListRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe direction used to sort returned cookies.'\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nFilters the returned cookies that match the specified domain attribute"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `export` after provisioning.\nExport the list of cookies as a file, limited to 50000 entries.\nAvailable values: \"csv\"."]
    pub fn export(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nIncludes cookies that match one or more URL-encoded hostnames separated by commas.\n\nWildcards are supported at the start and end of each hostname to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn hosts(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `http_only` after provisioning.\nFilters the returned cookies that are set with HttpOnly"]
    pub fn http_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFilters the returned cookies that match the specified name.\nWildcards are supported at the start and end to support starts with, ends with\nand contains. e.g. session*"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nThe field used to sort returned cookies.\nAvailable values: \"first_seen_at\", \"last_seen_at\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page` after provisioning.\nThe current page number of the paginated results.\n\nWe additionally support a special value \"all\". When \"all\" is used, the API will return all the cookies\nwith the applied filters in a single page. This feature is best-effort and it may only work for zones with\na low number of cookies"]
    pub fn page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page_url` after provisioning.\nIncludes connections that match one or more page URLs (separated by commas) where they were last seen\n\nWildcards are supported at the start and end of each page URL to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nFilters the returned cookies that match the specified path attribute"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nThe number of results per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataPageShieldCookiesList_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `same_site` after provisioning.\nFilters the returned cookies that match the specified same_site attribute\nAvailable values: \"lax\", \"strict\", \"none\"."]
    pub fn same_site(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.same_site", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secure` after provisioning.\nFilters the returned cookies that are set with Secure"]
    pub fn secure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nFilters the returned cookies that match the specified type attribute\nAvailable values: \"first_party\", \"unknown\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPageShieldCookiesList_Result {}

impl DataPageShieldCookiesList_Result { }

impl terrars::ToListMappable for DataPageShieldCookiesList_Result {
    type O = terrars::BlockAssignable<DataPageShieldCookiesList_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPageShieldCookiesList_Result {}

impl BuildDataPageShieldCookiesList_Result {
    pub fn build(self) -> DataPageShieldCookiesList_Result {
        DataPageShieldCookiesList_Result {}
    }
}

#[allow(dead_code)]
pub struct DataPageShieldCookiesList_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPageShieldCookiesList_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPageShieldCookiesList_ResultRef {
        DataPageShieldCookiesList_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPageShieldCookiesList_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_attribute` after provisioning.\n"]
    pub fn domain_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `expires_attribute` after provisioning.\n"]
    pub fn expires_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `first_seen_at` after provisioning.\n"]
    pub fn first_seen_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_seen_at", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `http_only_attribute` after provisioning.\n"]
    pub fn http_only_attribute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_only_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `last_seen_at` after provisioning.\n"]
    pub fn last_seen_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_seen_at", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age_attribute` after provisioning.\n"]
    pub fn max_age_attribute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `page_urls` after provisioning.\n"]
    pub fn page_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.page_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `path_attribute` after provisioning.\n"]
    pub fn path_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_attribute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `same_site_attribute` after provisioning.\nAvailable values: \"lax\", \"strict\", \"none\"."]
    pub fn same_site_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.same_site_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `secure_attribute` after provisioning.\n"]
    pub fn secure_attribute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure_attribute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"first_party\", \"unknown\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
