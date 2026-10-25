#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataPageShieldConnectionsListData_ {
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
    exclude_cdn_cgi: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_urls: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_page: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prioritize_malicious: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    urls: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataPageShieldConnectionsList_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPageShieldConnectionsListData_>,
}

#[derive(Clone)]
pub struct DataPageShieldConnectionsList(std::rc::Rc<DataPageShieldConnectionsList_>);

impl DataPageShieldConnectionsList {
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
        "Set the field `direction`.\nThe direction used to sort returned connections.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclude_cdn_cgi`.\nWhen true, excludes connections seen in a `/cdn-cgi` path from the returned connections. The default value is true."]
    pub fn set_exclude_cdn_cgi(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_cdn_cgi = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclude_urls`.\nExcludes connections whose URL contains one of the URL-encoded URLs separated by commas."]
    pub fn set_exclude_urls(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().exclude_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `export`.\nExport the list of connections as a file, limited to 50000 entries.\nAvailable values: \"csv\"."]
    pub fn set_export(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().export = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hosts`.\nIncludes connections that match one or more URL-encoded hostnames separated by commas.\n\nWildcards are supported at the start and end of each hostname to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn set_hosts(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order_by`.\nThe field used to sort returned connections.\nAvailable values: \"first_seen_at\", \"last_seen_at\"."]
    pub fn set_order_by(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order_by = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `page`.\nThe current page number of the paginated results.\n\nWe additionally support a special value \"all\". When \"all\" is used, the API will return all the connections\nwith the applied filters in a single page. This feature is best-effort and it may only work for zones with\na low number of connections"]
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

    #[doc = "Set the field `per_page`.\nThe number of results per page."]
    pub fn set_per_page(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().per_page = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prioritize_malicious`.\nWhen true, malicious connections appear first in the returned connections."]
    pub fn set_prioritize_malicious(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prioritize_malicious = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nFilters the returned connections using a comma-separated list of connection statuses. Accepted values: `active`, `infrequent`, and `inactive`. The default value is `active`."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `urls`.\nIncludes connections whose URL contain one or more URL-encoded URLs separated by commas."]
    pub fn set_urls(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier"]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe direction used to sort returned connections.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_cdn_cgi` after provisioning.\nWhen true, excludes connections seen in a `/cdn-cgi` path from the returned connections. The default value is true."]
    pub fn exclude_cdn_cgi(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_cdn_cgi", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_urls` after provisioning.\nExcludes connections whose URL contains one of the URL-encoded URLs separated by commas."]
    pub fn exclude_urls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `export` after provisioning.\nExport the list of connections as a file, limited to 50000 entries.\nAvailable values: \"csv\"."]
    pub fn export(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nIncludes connections that match one or more URL-encoded hostnames separated by commas.\n\nWildcards are supported at the start and end of each hostname to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn hosts(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nThe field used to sort returned connections.\nAvailable values: \"first_seen_at\", \"last_seen_at\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page` after provisioning.\nThe current page number of the paginated results.\n\nWe additionally support a special value \"all\". When \"all\" is used, the API will return all the connections\nwith the applied filters in a single page. This feature is best-effort and it may only work for zones with\na low number of connections"]
    pub fn page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page_url` after provisioning.\nIncludes connections that match one or more page URLs (separated by commas) where they were last seen\n\nWildcards are supported at the start and end of each page URL to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nThe number of results per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `prioritize_malicious` after provisioning.\nWhen true, malicious connections appear first in the returned connections."]
    pub fn prioritize_malicious(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prioritize_malicious", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataPageShieldConnectionsList_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nFilters the returned connections using a comma-separated list of connection statuses. Accepted values: `active`, `infrequent`, and `inactive`. The default value is `active`."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urls` after provisioning.\nIncludes connections whose URL contain one or more URL-encoded URLs separated by commas."]
    pub fn urls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataPageShieldConnectionsList {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPageShieldConnectionsList { }

impl terrars::ToListMappable for DataPageShieldConnectionsList {
    type O = terrars::ListRef<DataPageShieldConnectionsListRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPageShieldConnectionsList_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_page_shield_connections_list".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPageShieldConnectionsList {
    pub tf_id: std::string::String,
}

impl BuildDataPageShieldConnectionsList {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPageShieldConnectionsList {
        let out = DataPageShieldConnectionsList(std::rc::Rc::new(DataPageShieldConnectionsList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPageShieldConnectionsListData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                direction: core::default::Default::default(),
                exclude_cdn_cgi: core::default::Default::default(),
                exclude_urls: core::default::Default::default(),
                export: core::default::Default::default(),
                hosts: core::default::Default::default(),
                max_items: core::default::Default::default(),
                order_by: core::default::Default::default(),
                page: core::default::Default::default(),
                page_url: core::default::Default::default(),
                per_page: core::default::Default::default(),
                prioritize_malicious: core::default::Default::default(),
                status: core::default::Default::default(),
                urls: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPageShieldConnectionsListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPageShieldConnectionsListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPageShieldConnectionsListRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe direction used to sort returned connections.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_cdn_cgi` after provisioning.\nWhen true, excludes connections seen in a `/cdn-cgi` path from the returned connections. The default value is true."]
    pub fn exclude_cdn_cgi(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_cdn_cgi", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_urls` after provisioning.\nExcludes connections whose URL contains one of the URL-encoded URLs separated by commas."]
    pub fn exclude_urls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `export` after provisioning.\nExport the list of connections as a file, limited to 50000 entries.\nAvailable values: \"csv\"."]
    pub fn export(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nIncludes connections that match one or more URL-encoded hostnames separated by commas.\n\nWildcards are supported at the start and end of each hostname to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn hosts(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nThe field used to sort returned connections.\nAvailable values: \"first_seen_at\", \"last_seen_at\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page` after provisioning.\nThe current page number of the paginated results.\n\nWe additionally support a special value \"all\". When \"all\" is used, the API will return all the connections\nwith the applied filters in a single page. This feature is best-effort and it may only work for zones with\na low number of connections"]
    pub fn page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page_url` after provisioning.\nIncludes connections that match one or more page URLs (separated by commas) where they were last seen\n\nWildcards are supported at the start and end of each page URL to support starts with, ends with\nand contains. If no wildcards are used, results will be filtered by exact match"]
    pub fn page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nThe number of results per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `prioritize_malicious` after provisioning.\nWhen true, malicious connections appear first in the returned connections."]
    pub fn prioritize_malicious(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prioritize_malicious", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataPageShieldConnectionsList_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nFilters the returned connections using a comma-separated list of connection statuses. Accepted values: `active`, `infrequent`, and `inactive`. The default value is `active`."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urls` after provisioning.\nIncludes connections whose URL contain one or more URL-encoded URLs separated by commas."]
    pub fn urls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPageShieldConnectionsList_Result {}

impl DataPageShieldConnectionsList_Result { }

impl terrars::ToListMappable for DataPageShieldConnectionsList_Result {
    type O = terrars::BlockAssignable<DataPageShieldConnectionsList_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPageShieldConnectionsList_Result {}

impl BuildDataPageShieldConnectionsList_Result {
    pub fn build(self) -> DataPageShieldConnectionsList_Result {
        DataPageShieldConnectionsList_Result {}
    }
}

#[allow(dead_code)]
pub struct DataPageShieldConnectionsList_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPageShieldConnectionsList_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPageShieldConnectionsList_ResultRef {
        DataPageShieldConnectionsList_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPageShieldConnectionsList_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `added_at` after provisioning.\n"]
    pub fn added_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.added_at", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_reported_malicious` after provisioning.\n"]
    pub fn domain_reported_malicious(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_reported_malicious", self.base))
    }

    #[doc = "Get a reference to the value of field `first_page_url` after provisioning.\n"]
    pub fn first_page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_page_url", self.base))
    }

    #[doc = "Get a reference to the value of field `first_seen_at` after provisioning.\n"]
    pub fn first_seen_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_seen_at", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `last_seen_at` after provisioning.\n"]
    pub fn last_seen_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_seen_at", self.base))
    }

    #[doc = "Get a reference to the value of field `malicious_domain_categories` after provisioning.\n"]
    pub fn malicious_domain_categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.malicious_domain_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `malicious_url_categories` after provisioning.\n"]
    pub fn malicious_url_categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.malicious_url_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `page_urls` after provisioning.\n"]
    pub fn page_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.page_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `url_contains_cdn_cgi_path` after provisioning.\n"]
    pub fn url_contains_cdn_cgi_path(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_contains_cdn_cgi_path", self.base))
    }

    #[doc = "Get a reference to the value of field `url_reported_malicious` after provisioning.\n"]
    pub fn url_reported_malicious(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_reported_malicious", self.base))
    }
}
