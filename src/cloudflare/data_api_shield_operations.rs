#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataApiShieldOperationsData_ {
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
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataApiShieldOperations_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataApiShieldOperationsData_>,
}

#[derive(Clone)]
pub struct DataApiShieldOperations(std::rc::Rc<DataApiShieldOperations_>);

impl DataApiShieldOperations {
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

    #[doc =
        "Set the field `feature`.\nAdd feature(s) to the results. The feature name that is given here corresponds to the resulting feature object. Have a look at the top-level object description for more details on the specific meaning."]
    pub fn set_feature(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().feature = std::option::Option::Some(v.into());
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
        "Set the field `order`.\nField to order by. When requesting a feature, the feature keys are available for ordering as well, e.g., `thresholds.suggested_threshold`.\nAvailable values: \"method\", \"host\", \"endpoint\", \"thresholds.$key\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
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
        "Get a reference to the value of field `feature` after provisioning.\nAdd feature(s) to the results. The feature name that is given here corresponds to the resulting feature object. Have a look at the top-level object description for more details on the specific meaning."]
    pub fn feature(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature", self.extract_ref()))
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
        "Get a reference to the value of field `order` after provisioning.\nField to order by. When requesting a feature, the feature keys are available for ordering as well, e.g., `thresholds.suggested_threshold`.\nAvailable values: \"method\", \"host\", \"endpoint\", \"thresholds.$key\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataApiShieldOperations_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataApiShieldOperations {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataApiShieldOperations { }

impl terrars::ToListMappable for DataApiShieldOperations {
    type O = terrars::ListRef<DataApiShieldOperationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataApiShieldOperations_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_api_shield_operations".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApiShieldOperations {
    pub tf_id: std::string::String,
}

impl BuildDataApiShieldOperations {
    pub fn build(self, stack: &mut terrars::Stack) -> DataApiShieldOperations {
        let out = DataApiShieldOperations(std::rc::Rc::new(DataApiShieldOperations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataApiShieldOperationsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                direction: core::default::Default::default(),
                endpoint: core::default::Default::default(),
                feature: core::default::Default::default(),
                host: core::default::Default::default(),
                max_items: core::default::Default::default(),
                method: core::default::Default::default(),
                order: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataApiShieldOperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataApiShieldOperationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `feature` after provisioning.\nAdd feature(s) to the results. The feature name that is given here corresponds to the resulting feature object. Have a look at the top-level object description for more details on the specific meaning."]
    pub fn feature(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature", self.extract_ref()))
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
        "Get a reference to the value of field `order` after provisioning.\nField to order by. When requesting a feature, the feature keys are available for ordering as well, e.g., `thresholds.suggested_threshold`.\nAvailable values: \"method\", \"host\", \"endpoint\", \"thresholds.$key\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataApiShieldOperations_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_ApiRouting {}

impl DataApiShieldOperations_Result_Features_ApiRouting { }

impl terrars::ToListMappable for DataApiShieldOperations_Result_Features_ApiRouting {
    type O = terrars::BlockAssignable<DataApiShieldOperations_Result_Features_ApiRouting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_ApiRouting {}

impl BuildDataApiShieldOperations_Result_Features_ApiRouting {
    pub fn build(self) -> DataApiShieldOperations_Result_Features_ApiRouting {
        DataApiShieldOperations_Result_Features_ApiRouting {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_ApiRoutingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperations_Result_Features_ApiRoutingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_ApiRoutingRef {
        DataApiShieldOperations_Result_Features_ApiRoutingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_ApiRoutingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc = "Get a reference to the value of field `route` after provisioning.\nTarget route."]
    pub fn route(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 { }

impl terrars
::ToListMappable for DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {
    type O =
        terrars
        ::BlockAssignable<
            DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {}

impl BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {
    pub fn build(
        self,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower` after provisioning.\nLower bound for percentile estimate"]
    pub fn lower(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower", self.base))
    }

    #[doc = "Get a reference to the value of field `upper` after provisioning.\nUpper bound for percentile estimate"]
    pub fn upper(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 { }

impl terrars
::ToListMappable for DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {
    type O =
        terrars
        ::BlockAssignable<
            DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {}

impl BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {
    pub fn build(
        self,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower` after provisioning.\nLower bound for percentile estimate"]
    pub fn lower(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower", self.base))
    }

    #[doc = "Get a reference to the value of field `upper` after provisioning.\nUpper bound for percentile estimate"]
    pub fn upper(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 { }

impl terrars
::ToListMappable for DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {
    type O =
        terrars
        ::BlockAssignable<
            DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {}

impl BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {
    pub fn build(
        self,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower` after provisioning.\nLower bound for percentile estimate"]
    pub fn lower(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower", self.base))
    }

    #[doc = "Get a reference to the value of field `upper` after provisioning.\nUpper bound for percentile estimate"]
    pub fn upper(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals { }

impl terrars
::ToListMappable for DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {
    type O =
        terrars
        ::BlockAssignable<
            DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {}

impl BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {
    pub fn build(
        self,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `p90` after provisioning.\nUpper and lower bound for percentile estimate"]
    pub fn p90(
        &self,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref::new(
            self.shared().clone(),
            format!("{}.p90", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `p95` after provisioning.\nUpper and lower bound for percentile estimate"]
    pub fn p95(
        &self,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref::new(
            self.shared().clone(),
            format!("{}.p95", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `p99` after provisioning.\nUpper and lower bound for percentile estimate"]
    pub fn p99(
        &self,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref::new(
            self.shared().clone(),
            format!("{}.p99", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold {}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold { }

impl terrars::ToListMappable for DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold {
    type O =
        terrars::BlockAssignable<DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold {}

impl BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold {
    pub fn build(self) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThresholdRef {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `confidence_intervals` after provisioning.\n"]
    pub fn confidence_intervals(
        &self,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef::new(
            self.shared().clone(),
            format!("{}.confidence_intervals", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `mean` after provisioning.\nSuggested threshold."]
    pub fn mean(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mean", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervals {}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervals { }

impl terrars::ToListMappable for DataApiShieldOperations_Result_Features_ConfidenceIntervals {
    type O = terrars::BlockAssignable<DataApiShieldOperations_Result_Features_ConfidenceIntervals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals {}

impl BuildDataApiShieldOperations_Result_Features_ConfidenceIntervals {
    pub fn build(self) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_ConfidenceIntervalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperations_Result_Features_ConfidenceIntervalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervalsRef {
        DataApiShieldOperations_Result_Features_ConfidenceIntervalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_ConfidenceIntervalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc = "Get a reference to the value of field `suggested_threshold` after provisioning.\n"]
    pub fn suggested_threshold(
        &self,
    ) -> DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThresholdRef {
        DataApiShieldOperations_Result_Features_ConfidenceIntervals_SuggestedThresholdRef::new(
            self.shared().clone(),
            format!("{}.suggested_threshold", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemas {}

impl DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemas { }

impl terrars::ToListMappable for DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemas {
    type O = terrars::BlockAssignable<DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemas {}

impl BuildDataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemas {
    pub fn build(self) -> DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemas {
        DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemas {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemasRef {
        DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nAn array containing the learned parameter schemas."]
    pub fn parameters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `responses` after provisioning.\nAn empty response object. This field is required to yield a valid operation schema."]
    pub fn responses(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.responses", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_ParameterSchemas {}

impl DataApiShieldOperations_Result_Features_ParameterSchemas { }

impl terrars::ToListMappable for DataApiShieldOperations_Result_Features_ParameterSchemas {
    type O = terrars::BlockAssignable<DataApiShieldOperations_Result_Features_ParameterSchemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_ParameterSchemas {}

impl BuildDataApiShieldOperations_Result_Features_ParameterSchemas {
    pub fn build(self) -> DataApiShieldOperations_Result_Features_ParameterSchemas {
        DataApiShieldOperations_Result_Features_ParameterSchemas {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_ParameterSchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperations_Result_Features_ParameterSchemasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_ParameterSchemasRef {
        DataApiShieldOperations_Result_Features_ParameterSchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_ParameterSchemasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameter_schemas` after provisioning.\nAn operation schema object containing a response."]
    pub fn parameter_schemas(&self) -> DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemasRef {
        DataApiShieldOperations_Result_Features_ParameterSchemas_ParameterSchemasRef::new(
            self.shared().clone(),
            format!("{}.parameter_schemas", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchema {}

impl DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchema { }

impl terrars::ToListMappable for DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchema {
    type O = terrars::BlockAssignable<DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchema {}

impl BuildDataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchema {
    pub fn build(self) -> DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchema {
        DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchema {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchemaRef {
        DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSchema file name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_SchemaInfo {}

impl DataApiShieldOperations_Result_Features_SchemaInfo { }

impl terrars::ToListMappable for DataApiShieldOperations_Result_Features_SchemaInfo {
    type O = terrars::BlockAssignable<DataApiShieldOperations_Result_Features_SchemaInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_SchemaInfo {}

impl BuildDataApiShieldOperations_Result_Features_SchemaInfo {
    pub fn build(self) -> DataApiShieldOperations_Result_Features_SchemaInfo {
        DataApiShieldOperations_Result_Features_SchemaInfo {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_SchemaInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperations_Result_Features_SchemaInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_SchemaInfoRef {
        DataApiShieldOperations_Result_Features_SchemaInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_SchemaInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_schema` after provisioning.\nSchema active on endpoint."]
    pub fn active_schema(&self) -> DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchemaRef {
        DataApiShieldOperations_Result_Features_SchemaInfo_ActiveSchemaRef::new(
            self.shared().clone(),
            format!("{}.active_schema", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `mitigation_action` after provisioning.\nAction taken on requests failing validation.\nAvailable values: \"none\", \"log\", \"block\"."]
    pub fn mitigation_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mitigation_action", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features_Thresholds {}

impl DataApiShieldOperations_Result_Features_Thresholds { }

impl terrars::ToListMappable for DataApiShieldOperations_Result_Features_Thresholds {
    type O = terrars::BlockAssignable<DataApiShieldOperations_Result_Features_Thresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features_Thresholds {}

impl BuildDataApiShieldOperations_Result_Features_Thresholds {
    pub fn build(self) -> DataApiShieldOperations_Result_Features_Thresholds {
        DataApiShieldOperations_Result_Features_Thresholds {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_Features_ThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperations_Result_Features_ThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiShieldOperations_Result_Features_ThresholdsRef {
        DataApiShieldOperations_Result_Features_ThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_Features_ThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_id_tokens` after provisioning.\nThe total number of auth-ids seen across this calculation."]
    pub fn auth_id_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_id_tokens", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_points` after provisioning.\nThe number of data points used for the threshold suggestion calculation."]
    pub fn data_points(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_points", self.base))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `p50` after provisioning.\nThe p50 quantile of requests (in period_seconds)."]
    pub fn p50(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p50", self.base))
    }

    #[doc =
        "Get a reference to the value of field `p90` after provisioning.\nThe p90 quantile of requests (in period_seconds)."]
    pub fn p90(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p90", self.base))
    }

    #[doc =
        "Get a reference to the value of field `p99` after provisioning.\nThe p99 quantile of requests (in period_seconds)."]
    pub fn p99(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p99", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nThe period over which this threshold is suggested."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests` after provisioning.\nThe estimated number of requests covered by these calculations."]
    pub fn requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suggested_threshold` after provisioning.\nThe suggested threshold in requests done by the same auth_id or period_seconds."]
    pub fn suggested_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suggested_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result_Features {}

impl DataApiShieldOperations_Result_Features { }

impl terrars::ToListMappable for DataApiShieldOperations_Result_Features {
    type O = terrars::BlockAssignable<DataApiShieldOperations_Result_Features>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result_Features {}

impl BuildDataApiShieldOperations_Result_Features {
    pub fn build(self) -> DataApiShieldOperations_Result_Features {
        DataApiShieldOperations_Result_Features {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_Result_FeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperations_Result_FeaturesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApiShieldOperations_Result_FeaturesRef {
        DataApiShieldOperations_Result_FeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_Result_FeaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_routing` after provisioning.\nAPI Routing settings on endpoint."]
    pub fn api_routing(&self) -> DataApiShieldOperations_Result_Features_ApiRoutingRef {
        DataApiShieldOperations_Result_Features_ApiRoutingRef::new(
            self.shared().clone(),
            format!("{}.api_routing", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `confidence_intervals` after provisioning.\n"]
    pub fn confidence_intervals(&self) -> DataApiShieldOperations_Result_Features_ConfidenceIntervalsRef {
        DataApiShieldOperations_Result_Features_ConfidenceIntervalsRef::new(
            self.shared().clone(),
            format!("{}.confidence_intervals", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_schemas` after provisioning.\n"]
    pub fn parameter_schemas(&self) -> DataApiShieldOperations_Result_Features_ParameterSchemasRef {
        DataApiShieldOperations_Result_Features_ParameterSchemasRef::new(
            self.shared().clone(),
            format!("{}.parameter_schemas", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `schema_info` after provisioning.\n"]
    pub fn schema_info(&self) -> DataApiShieldOperations_Result_Features_SchemaInfoRef {
        DataApiShieldOperations_Result_Features_SchemaInfoRef::new(
            self.shared().clone(),
            format!("{}.schema_info", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `thresholds` after provisioning.\n"]
    pub fn thresholds(&self) -> DataApiShieldOperations_Result_Features_ThresholdsRef {
        DataApiShieldOperations_Result_Features_ThresholdsRef::new(
            self.shared().clone(),
            format!("{}.thresholds", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataApiShieldOperations_Result {}

impl DataApiShieldOperations_Result { }

impl terrars::ToListMappable for DataApiShieldOperations_Result {
    type O = terrars::BlockAssignable<DataApiShieldOperations_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiShieldOperations_Result {}

impl BuildDataApiShieldOperations_Result {
    pub fn build(self) -> DataApiShieldOperations_Result {
        DataApiShieldOperations_Result {}
    }
}

#[allow(dead_code)]
pub struct DataApiShieldOperations_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiShieldOperations_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApiShieldOperations_ResultRef {
        DataApiShieldOperations_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiShieldOperations_ResultRef {
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
    pub fn features(&self) -> DataApiShieldOperations_Result_FeaturesRef {
        DataApiShieldOperations_Result_FeaturesRef::new(self.shared().clone(), format!("{}.features", self.base))
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

    #[doc = "Get a reference to the value of field `operation_id` after provisioning.\nUUID."]
    pub fn operation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation_id", self.base))
    }
}
