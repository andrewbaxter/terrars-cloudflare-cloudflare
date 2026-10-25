#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataApiShieldDiscoveryOperationsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    diff: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataApiShieldDiscoveryOperations_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataApiShieldDiscoveryOperationsData_>,
}

#[derive(Clone)]
pub struct DataApiShieldDiscoveryOperations(std::rc::Rc<DataApiShieldDiscoveryOperations_>);

impl DataApiShieldDiscoveryOperations {
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
        "Set the field `diff`.\nWhen `true`, only return API Discovery results that are not saved into API Shield Endpoint Management"]
    pub fn set_diff(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().diff = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\nFilter results to only include endpoints containing this pattern."]
    pub fn set_endpoint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nFilter results to only include the specified hosts."]
    pub fn set_host(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method`.\nFilter results to only include the specified HTTP methods."]
    pub fn set_method(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nField to order by\nAvailable values: \"host\", \"method\", \"endpoint\", \"traffic_stats.requests\", \"traffic_stats.last_updated\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin`.\nFilter results to only include discovery results sourced from a particular discovery engine\n  * `ML` - Discovered operations that were sourced using ML API Discovery\n  * `SessionIdentifier` - Discovered operations that were sourced using Session Identifier API Discovery\nAvailable values: \"ML\", \"SessionIdentifier\", \"LabelDiscovery\"."]
    pub fn set_origin(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().origin = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `state`.\nFilter results to only include discovery results in a particular state. States are as follows\n  * `review` - Discovered operations that are not saved into API Shield Endpoint Management\n  * `saved` - Discovered operations that are already saved into API Shield Endpoint Management\n  * `ignored` - Discovered operations that have been marked as ignored\nAvailable values: \"review\", \"saved\", \"ignored\"."]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `diff` after provisioning.\nWhen `true`, only return API Discovery results that are not saved into API Shield Endpoint Management"]
    pub fn diff(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.diff", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nFilter results to only include endpoints containing this pattern."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nFilter results to only include the specified hosts."]
    pub fn host(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nFilter results to only include the specified HTTP methods."]
    pub fn method(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order by\nAvailable values: \"host\", \"method\", \"endpoint\", \"traffic_stats.requests\", \"traffic_stats.last_updated\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nFilter results to only include discovery results sourced from a particular discovery engine\n  * `ML` - Discovered operations that were sourced using ML API Discovery\n  * `SessionIdentifier` - Discovered operations that were sourced using Session Identifier API Discovery\nAvailable values: \"ML\", \"SessionIdentifier\", \"LabelDiscovery\"."]
    pub fn origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataApiShieldDiscoveryOperations_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nFilter results to only include discovery results in a particular state. States are as follows\n  * `review` - Discovered operations that are not saved into API Shield Endpoint Management\n  * `saved` - Discovered operations that are already saved into API Shield Endpoint Management\n  * `ignored` - Discovered operations that have been marked as ignored\nAvailable values: \"review\", \"saved\", \"ignored\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataApiShieldDiscoveryOperations {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataApiShieldDiscoveryOperations { }

impl terrars::ToListMappable for DataApiShieldDiscoveryOperations {
    type O = terrars::ListRef<DataApiShieldDiscoveryOperationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataApiShieldDiscoveryOperations_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_api_shield_discovery_operations".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApiShieldDiscoveryOperations {
    pub tf_id: std::string::String,
}

impl BuildDataApiShieldDiscoveryOperations {
    pub fn build(self, stack: &mut terrars::Stack) -> DataApiShieldDiscoveryOperations {
        let out = DataApiShieldDiscoveryOperations(std::rc::Rc::new(DataApiShieldDiscoveryOperations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataApiShieldDiscoveryOperationsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                diff: core::default::Default::default(),
                direction: core::default::Default::default(),
                endpoint: core::default::Default::default(),
                host: core::default::Default::default(),
                max_items: core::default::Default::default(),
                method: core::default::Default::default(),
                order: core::default::Default::default(),
                origin: core::default::Default::default(),
                state: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataApiShieldDiscoveryOperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldDiscoveryOperationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataApiShieldDiscoveryOperationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `diff` after provisioning.\nWhen `true`, only return API Discovery results that are not saved into API Shield Endpoint Management"]
    pub fn diff(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.diff", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nFilter results to only include endpoints containing this pattern."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nFilter results to only include the specified hosts."]
    pub fn host(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nFilter results to only include the specified HTTP methods."]
    pub fn method(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order by\nAvailable values: \"host\", \"method\", \"endpoint\", \"traffic_stats.requests\", \"traffic_stats.last_updated\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nFilter results to only include discovery results sourced from a particular discovery engine\n  * `ML` - Discovered operations that were sourced using ML API Discovery\n  * `SessionIdentifier` - Discovered operations that were sourced using Session Identifier API Discovery\nAvailable values: \"ML\", \"SessionIdentifier\", \"LabelDiscovery\"."]
    pub fn origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataApiShieldDiscoveryOperations_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nFilter results to only include discovery results in a particular state. States are as follows\n  * `review` - Discovered operations that are not saved into API Shield Endpoint Management\n  * `saved` - Discovered operations that are already saved into API Shield Endpoint Management\n  * `ignored` - Discovered operations that have been marked as ignored\nAvailable values: \"review\", \"saved\", \"ignored\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldDiscoveryOperations_Result_Features_TrafficStats {}

impl DataApiShieldDiscoveryOperations_Result_Features_TrafficStats { }

impl terrars::ToListMappable for DataApiShieldDiscoveryOperations_Result_Features_TrafficStats {
    type O = terrars::BlockAssignable<DataApiShieldDiscoveryOperations_Result_Features_TrafficStats>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldDiscoveryOperations_Result_Features_TrafficStats {}

impl BuildDataApiShieldDiscoveryOperations_Result_Features_TrafficStats {
    pub fn build(self) -> DataApiShieldDiscoveryOperations_Result_Features_TrafficStats {
        DataApiShieldDiscoveryOperations_Result_Features_TrafficStats {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldDiscoveryOperations_Result_Features_TrafficStatsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldDiscoveryOperations_Result_Features_TrafficStatsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldDiscoveryOperations_Result_Features_TrafficStatsRef {
        DataApiShieldDiscoveryOperations_Result_Features_TrafficStatsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldDiscoveryOperations_Result_Features_TrafficStatsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nThe period in seconds these statistics were computed over"]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests` after provisioning.\nThe average number of requests seen during this period"]
    pub fn requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldDiscoveryOperations_Result_Features {}

impl DataApiShieldDiscoveryOperations_Result_Features { }

impl terrars::ToListMappable for DataApiShieldDiscoveryOperations_Result_Features {
    type O = terrars::BlockAssignable<DataApiShieldDiscoveryOperations_Result_Features>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldDiscoveryOperations_Result_Features {}

impl BuildDataApiShieldDiscoveryOperations_Result_Features {
    pub fn build(self) -> DataApiShieldDiscoveryOperations_Result_Features {
        DataApiShieldDiscoveryOperations_Result_Features {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldDiscoveryOperations_Result_FeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldDiscoveryOperations_Result_FeaturesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldDiscoveryOperations_Result_FeaturesRef {
        DataApiShieldDiscoveryOperations_Result_FeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldDiscoveryOperations_Result_FeaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `traffic_stats` after provisioning.\n"]
    pub fn traffic_stats(&self) -> DataApiShieldDiscoveryOperations_Result_Features_TrafficStatsRef {
        DataApiShieldDiscoveryOperations_Result_Features_TrafficStatsRef::new(
            self.shared().clone(),
            format!("{}.traffic_stats", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataApiShieldDiscoveryOperations_Result {}

impl DataApiShieldDiscoveryOperations_Result { }

impl terrars::ToListMappable for DataApiShieldDiscoveryOperations_Result {
    type O = terrars::BlockAssignable<DataApiShieldDiscoveryOperations_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldDiscoveryOperations_Result {}

impl BuildDataApiShieldDiscoveryOperations_Result {
    pub fn build(self) -> DataApiShieldDiscoveryOperations_Result {
        DataApiShieldDiscoveryOperations_Result {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldDiscoveryOperations_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldDiscoveryOperations_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApiShieldDiscoveryOperations_ResultRef {
        DataApiShieldDiscoveryOperations_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldDiscoveryOperations_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with {varN}, starting with {var1}, during insertion. This will further be Cloudflare-normalized upon insertion. See: https://developers.cloudflare.com/rules/normalization/how-it-works/."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> DataApiShieldDiscoveryOperations_Result_FeaturesRef {
        DataApiShieldDiscoveryOperations_Result_FeaturesRef::new(
            self.shared().clone(),
            format!("{}.features", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nRFC3986-compliant host."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nThe HTTP method used to access the endpoint.\nAvailable values: \"GET\", \"POST\", \"HEAD\", \"OPTIONS\", \"PUT\", \"DELETE\", \"CONNECT\", \"PATCH\", \"TRACE\"."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nAPI discovery engine(s) that discovered this operation"]
    pub fn origin(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nState of operation in API Discovery\n  * `review` - Operation is not saved into API Shield Endpoint Management\n  * `saved` - Operation is saved into API Shield Endpoint Management\n  * `ignored` - Operation is marked as ignored\nAvailable values: \"review\", \"saved\", \"ignored\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
