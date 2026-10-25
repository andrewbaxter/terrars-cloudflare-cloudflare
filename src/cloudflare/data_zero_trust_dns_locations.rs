#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDnsLocationsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustDnsLocations_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDnsLocationsData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDnsLocations(std::rc::Rc<DataZeroTrustDnsLocations_>);

impl DataZeroTrustDnsLocations {
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

    #[doc = "Set the field `account_id`.\nSpecify the Cloudflare account identifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `direction`.\nSort direction. Only takes effect when `order_by` is also provided; it\nis ignored otherwise. When `direction` is omitted the effective\ndirection is field-specific: `created_at` and `updated_at` default to\ndescending (newest first); `name` defaults to ascending.\n  * `asc` — ascending.\n  * `desc` — descending.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nFilter the returned locations by one or more `field:value` pairs.\nRepeat the parameter to apply multiple filters; they are combined with\nlogical AND (a location must satisfy every filter to be returned).\n\nSupported fields and their matching behaviour:\n  * `name` — case-insensitive substring match on the location name.\n  * `id` — substring match on the location ID (UUID), with or without dashes.\n  * `is_default` — whether it is the default for the account.\n\nEach entry must match one of the per-field patterns below:\n  * the field must be one of `name`, `id`, or `is_default`;\n  * `name`/`id` accept any value;\n  * `is_default` only accepts `true` or `false`; any other value returns `400`"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order_by`.\nField to sort the returned locations by. When omitted, the order of\nresults is unspecified. Supported values:\n  * `name` — sort alphabetically by location name.\n  * `created_at` — sort by creation time; defaults to descending unless `direction` is set.\n  * `updated_at` — sort by last-modified time; defaults to descending unless `direction` is set.\nAvailable values: \"name\", \"created_at\", \"updated_at\"."]
    pub fn set_order_by(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order_by = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `search`.\nCase-insensitive substring match on the location name. When combined\nwith `filter`, both must match (logical AND)."]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nSort direction. Only takes effect when `order_by` is also provided; it\nis ignored otherwise. When `direction` is omitted the effective\ndirection is field-specific: `created_at` and `updated_at` default to\ndescending (newest first); `name` defaults to ascending.\n  * `asc` — ascending.\n  * `desc` — descending.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter the returned locations by one or more `field:value` pairs.\nRepeat the parameter to apply multiple filters; they are combined with\nlogical AND (a location must satisfy every filter to be returned).\n\nSupported fields and their matching behaviour:\n  * `name` — case-insensitive substring match on the location name.\n  * `id` — substring match on the location ID (UUID), with or without dashes.\n  * `is_default` — whether it is the default for the account.\n\nEach entry must match one of the per-field patterns below:\n  * the field must be one of `name`, `id`, or `is_default`;\n  * `name`/`id` accept any value;\n  * `is_default` only accepts `true` or `false`; any other value returns `400`"]
    pub fn filter(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nField to sort the returned locations by. When omitted, the order of\nresults is unspecified. Supported values:\n  * `name` — sort alphabetically by location name.\n  * `created_at` — sort by creation time; defaults to descending unless `direction` is set.\n  * `updated_at` — sort by last-modified time; defaults to descending unless `direction` is set.\nAvailable values: \"name\", \"created_at\", \"updated_at\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustDnsLocations_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nCase-insensitive substring match on the location name. When combined\nwith `filter`, both must match (logical AND)."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustDnsLocations {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDnsLocations { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations {
    type O = terrars::ListRef<DataZeroTrustDnsLocationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDnsLocations_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dns_locations".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDnsLocations {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustDnsLocations {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDnsLocations {
        let out = DataZeroTrustDnsLocations(std::rc::Rc::new(DataZeroTrustDnsLocations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDnsLocationsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                direction: core::default::Default::default(),
                filter: core::default::Default::default(),
                max_items: core::default::Default::default(),
                order_by: core::default::Default::default(),
                search: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDnsLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDnsLocationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nSort direction. Only takes effect when `order_by` is also provided; it\nis ignored otherwise. When `direction` is omitted the effective\ndirection is field-specific: `created_at` and `updated_at` default to\ndescending (newest first); `name` defaults to ascending.\n  * `asc` — ascending.\n  * `desc` — descending.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter the returned locations by one or more `field:value` pairs.\nRepeat the parameter to apply multiple filters; they are combined with\nlogical AND (a location must satisfy every filter to be returned).\n\nSupported fields and their matching behaviour:\n  * `name` — case-insensitive substring match on the location name.\n  * `id` — substring match on the location ID (UUID), with or without dashes.\n  * `is_default` — whether it is the default for the account.\n\nEach entry must match one of the per-field patterns below:\n  * the field must be one of `name`, `id`, or `is_default`;\n  * `name`/`id` accept any value;\n  * `is_default` only accepts `true` or `false`; any other value returns `400`"]
    pub fn filter(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nField to sort the returned locations by. When omitted, the order of\nresults is unspecified. Supported values:\n  * `name` — sort alphabetically by location name.\n  * `created_at` — sort by creation time; defaults to descending unless `direction` is set.\n  * `updated_at` — sort by last-modified time; defaults to descending unless `direction` is set.\nAvailable values: \"name\", \"created_at\", \"updated_at\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustDnsLocations_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nCase-insensitive substring match on the location name. When combined\nwith `filter`, both must match (logical AND)."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_Doh_Networks {}

impl DataZeroTrustDnsLocations_Result_Endpoints_Doh_Networks { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations_Result_Endpoints_Doh_Networks {
    type O = terrars::BlockAssignable<DataZeroTrustDnsLocations_Result_Endpoints_Doh_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDnsLocations_Result_Endpoints_Doh_Networks {}

impl BuildDataZeroTrustDnsLocations_Result_Endpoints_Doh_Networks {
    pub fn build(self) -> DataZeroTrustDnsLocations_Result_Endpoints_Doh_Networks {
        DataZeroTrustDnsLocations_Result_Endpoints_Doh_Networks {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_Doh_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocations_Result_Endpoints_Doh_NetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDnsLocations_Result_Endpoints_Doh_NetworksRef {
        DataZeroTrustDnsLocations_Result_Endpoints_Doh_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDnsLocations_Result_Endpoints_Doh_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\nSpecify the IP address or IP CIDR."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_Doh {}

impl DataZeroTrustDnsLocations_Result_Endpoints_Doh { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations_Result_Endpoints_Doh {
    type O = terrars::BlockAssignable<DataZeroTrustDnsLocations_Result_Endpoints_Doh>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDnsLocations_Result_Endpoints_Doh {}

impl BuildDataZeroTrustDnsLocations_Result_Endpoints_Doh {
    pub fn build(self) -> DataZeroTrustDnsLocations_Result_Endpoints_Doh {
        DataZeroTrustDnsLocations_Result_Endpoints_Doh {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_DohRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocations_Result_Endpoints_DohRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDnsLocations_Result_Endpoints_DohRef {
        DataZeroTrustDnsLocations_Result_Endpoints_DohRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDnsLocations_Result_Endpoints_DohRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicate whether the DOH endpoint is enabled for this location."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nSpecify the list of allowed source IP network ranges for this endpoint. When the list is empty, the endpoint allows all source IPs. The list takes effect only if the endpoint is enabled for this location."]
    pub fn networks(&self) -> terrars::ListRef<DataZeroTrustDnsLocations_Result_Endpoints_Doh_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_token` after provisioning.\nSpecify whether the DOH endpoint requires user identity authentication."]
    pub fn require_token(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_Dot {}

impl DataZeroTrustDnsLocations_Result_Endpoints_Dot { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations_Result_Endpoints_Dot {
    type O = terrars::BlockAssignable<DataZeroTrustDnsLocations_Result_Endpoints_Dot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDnsLocations_Result_Endpoints_Dot {}

impl BuildDataZeroTrustDnsLocations_Result_Endpoints_Dot {
    pub fn build(self) -> DataZeroTrustDnsLocations_Result_Endpoints_Dot {
        DataZeroTrustDnsLocations_Result_Endpoints_Dot {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_DotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocations_Result_Endpoints_DotRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDnsLocations_Result_Endpoints_DotRef {
        DataZeroTrustDnsLocations_Result_Endpoints_DotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDnsLocations_Result_Endpoints_DotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicate whether the DOT endpoint is enabled for this location."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nSpecify the list of allowed source IP network ranges for this endpoint. When the list is empty, the endpoint allows all source IPs. The list takes effect only if the endpoint is enabled for this location."]
    pub fn networks(&self) -> terrars::ListRef<DataZeroTrustDnsLocations_Result_Endpoints_Doh_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_Ipv4 {}

impl DataZeroTrustDnsLocations_Result_Endpoints_Ipv4 { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations_Result_Endpoints_Ipv4 {
    type O = terrars::BlockAssignable<DataZeroTrustDnsLocations_Result_Endpoints_Ipv4>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDnsLocations_Result_Endpoints_Ipv4 {}

impl BuildDataZeroTrustDnsLocations_Result_Endpoints_Ipv4 {
    pub fn build(self) -> DataZeroTrustDnsLocations_Result_Endpoints_Ipv4 {
        DataZeroTrustDnsLocations_Result_Endpoints_Ipv4 {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_Ipv4Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocations_Result_Endpoints_Ipv4Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDnsLocations_Result_Endpoints_Ipv4Ref {
        DataZeroTrustDnsLocations_Result_Endpoints_Ipv4Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDnsLocations_Result_Endpoints_Ipv4Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicate whether the IPv4 endpoint is enabled for this location."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_Networks {}

impl DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_Networks { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_Networks {
    type O = terrars::BlockAssignable<DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDnsLocations_Result_Endpoints_Ipv6_Networks {}

impl BuildDataZeroTrustDnsLocations_Result_Endpoints_Ipv6_Networks {
    pub fn build(self) -> DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_Networks {
        DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_Networks {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_NetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_NetworksRef {
        DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nSpecify the IPv6 address or IPv6 CIDR."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_Ipv6 {}

impl DataZeroTrustDnsLocations_Result_Endpoints_Ipv6 { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations_Result_Endpoints_Ipv6 {
    type O = terrars::BlockAssignable<DataZeroTrustDnsLocations_Result_Endpoints_Ipv6>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDnsLocations_Result_Endpoints_Ipv6 {}

impl BuildDataZeroTrustDnsLocations_Result_Endpoints_Ipv6 {
    pub fn build(self) -> DataZeroTrustDnsLocations_Result_Endpoints_Ipv6 {
        DataZeroTrustDnsLocations_Result_Endpoints_Ipv6 {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints_Ipv6Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocations_Result_Endpoints_Ipv6Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDnsLocations_Result_Endpoints_Ipv6Ref {
        DataZeroTrustDnsLocations_Result_Endpoints_Ipv6Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDnsLocations_Result_Endpoints_Ipv6Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicate whether the IPV6 endpoint is enabled for this location."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nSpecify the list of allowed source IPv6 network ranges for this endpoint. When the list is empty, the endpoint allows all source IPs. The list takes effect only if the endpoint is enabled for this location."]
    pub fn networks(&self) -> terrars::ListRef<DataZeroTrustDnsLocations_Result_Endpoints_Ipv6_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDnsLocations_Result_Endpoints {}

impl DataZeroTrustDnsLocations_Result_Endpoints { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations_Result_Endpoints {
    type O = terrars::BlockAssignable<DataZeroTrustDnsLocations_Result_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDnsLocations_Result_Endpoints {}

impl BuildDataZeroTrustDnsLocations_Result_Endpoints {
    pub fn build(self) -> DataZeroTrustDnsLocations_Result_Endpoints {
        DataZeroTrustDnsLocations_Result_Endpoints {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDnsLocations_Result_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocations_Result_EndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDnsLocations_Result_EndpointsRef {
        DataZeroTrustDnsLocations_Result_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDnsLocations_Result_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `doh` after provisioning.\n"]
    pub fn doh(&self) -> DataZeroTrustDnsLocations_Result_Endpoints_DohRef {
        DataZeroTrustDnsLocations_Result_Endpoints_DohRef::new(self.shared().clone(), format!("{}.doh", self.base))
    }

    #[doc = "Get a reference to the value of field `dot` after provisioning.\n"]
    pub fn dot(&self) -> DataZeroTrustDnsLocations_Result_Endpoints_DotRef {
        DataZeroTrustDnsLocations_Result_Endpoints_DotRef::new(self.shared().clone(), format!("{}.dot", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4` after provisioning.\n"]
    pub fn ipv4(&self) -> DataZeroTrustDnsLocations_Result_Endpoints_Ipv4Ref {
        DataZeroTrustDnsLocations_Result_Endpoints_Ipv4Ref::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> DataZeroTrustDnsLocations_Result_Endpoints_Ipv6Ref {
        DataZeroTrustDnsLocations_Result_Endpoints_Ipv6Ref::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDnsLocations_Result_MaxTtl {}

impl DataZeroTrustDnsLocations_Result_MaxTtl { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations_Result_MaxTtl {
    type O = terrars::BlockAssignable<DataZeroTrustDnsLocations_Result_MaxTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDnsLocations_Result_MaxTtl {}

impl BuildDataZeroTrustDnsLocations_Result_MaxTtl {
    pub fn build(self) -> DataZeroTrustDnsLocations_Result_MaxTtl {
        DataZeroTrustDnsLocations_Result_MaxTtl {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDnsLocations_Result_MaxTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocations_Result_MaxTtlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDnsLocations_Result_MaxTtlRef {
        DataZeroTrustDnsLocations_Result_MaxTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDnsLocations_Result_MaxTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\n`inherit` uses the account `max_ttl_secs`. `override` uses this location's `ttl_secs`. `disabled` leaves returned TTLs unchanged.\nAvailable values: \"inherit\", \"override\", \"disabled\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl_secs` after provisioning.\nLocation-specific cap on DNS response TTLs, in seconds. Required when `mode` is `override`. Must be omitted when `mode` is `inherit` or `disabled`."]
    pub fn ttl_secs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl_secs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDnsLocations_Result_Networks {}

impl DataZeroTrustDnsLocations_Result_Networks { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations_Result_Networks {
    type O = terrars::BlockAssignable<DataZeroTrustDnsLocations_Result_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDnsLocations_Result_Networks {}

impl BuildDataZeroTrustDnsLocations_Result_Networks {
    pub fn build(self) -> DataZeroTrustDnsLocations_Result_Networks {
        DataZeroTrustDnsLocations_Result_Networks {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDnsLocations_Result_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocations_Result_NetworksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDnsLocations_Result_NetworksRef {
        DataZeroTrustDnsLocations_Result_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDnsLocations_Result_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nSpecify the IPv4 address or IPv4 CIDR. Limit IPv4 CIDRs to a maximum of /24."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDnsLocations_Result {}

impl DataZeroTrustDnsLocations_Result { }

impl terrars::ToListMappable for DataZeroTrustDnsLocations_Result {
    type O = terrars::BlockAssignable<DataZeroTrustDnsLocations_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDnsLocations_Result {}

impl BuildDataZeroTrustDnsLocations_Result {
    pub fn build(self) -> DataZeroTrustDnsLocations_Result {
        DataZeroTrustDnsLocations_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDnsLocations_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDnsLocations_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDnsLocations_ResultRef {
        DataZeroTrustDnsLocations_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDnsLocations_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_default` after provisioning.\nIndicate whether this location is the default location."]
    pub fn client_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_default", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_destination_ips_id` after provisioning.\nIndicate the identifier of the pair of IPv4 addresses assigned to this location."]
    pub fn dns_destination_ips_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_destination_ips_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_destination_ipv6_block_id` after provisioning.\nSpecify the UUID of the IPv6 block brought to the gateway so that this location's IPv6 address is allocated from the Bring Your Own IPv6 (BYOIPv6) block rather than the standard Cloudflare IPv6 block."]
    pub fn dns_destination_ipv6_block_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_destination_ipv6_block_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `doh_subdomain` after provisioning.\nSpecify the DNS over HTTPS domain that receives DNS requests. Gateway automatically generates this value."]
    pub fn doh_subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.doh_subdomain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ecs_support` after provisioning.\nIndicate whether the location must resolve EDNS queries."]
    pub fn ecs_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_support", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endpoints` after provisioning.\nConfigure the destination endpoints for this location."]
    pub fn endpoints(&self) -> DataZeroTrustDnsLocations_Result_EndpointsRef {
        DataZeroTrustDnsLocations_Result_EndpointsRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nDefines the automatically generated IPv6 destination IP assigned to this location. Gateway counts all DNS requests sent to this IP as requests under this location."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_destination` after provisioning.\nShow the primary destination IPv4 address from the pair identified dns_destination_ips_id. This field read-only."]
    pub fn ipv4_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_destination", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_destination_backup` after provisioning.\nShow the backup destination IPv4 address from the pair identified dns_destination_ips_id. This field read-only."]
    pub fn ipv4_destination_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_destination_backup", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_ttl` after provisioning.\nControls how DNS response TTLs are capped for this location relative to the account `max_ttl_secs` setting. Omitting `max_ttl` on update resets it to `inherit`."]
    pub fn max_ttl(&self) -> DataZeroTrustDnsLocations_Result_MaxTtlRef {
        DataZeroTrustDnsLocations_Result_MaxTtlRef::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecify the location name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nSpecify the list of network ranges from which requests at this location originate. The list takes effect only if it is non-empty and the IPv4 endpoint is enabled for this location."]
    pub fn networks(&self) -> terrars::ListRef<DataZeroTrustDnsLocations_Result_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
