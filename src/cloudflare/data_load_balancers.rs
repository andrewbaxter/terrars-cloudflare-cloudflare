#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataLoadBalancersData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLoadBalancers_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLoadBalancersData_>,
}

#[derive(Clone)]
pub struct DataLoadBalancers(std::rc::Rc<DataLoadBalancers_>);

impl DataLoadBalancers {
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

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\n"]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataLoadBalancers_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataLoadBalancers {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLoadBalancers { }

impl terrars::ToListMappable for DataLoadBalancers {
    type O = terrars::ListRef<DataLoadBalancersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLoadBalancers_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_load_balancers".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLoadBalancers {
    pub tf_id: std::string::String,
}

impl BuildDataLoadBalancers {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLoadBalancers {
        let out = DataLoadBalancers(std::rc::Rc::new(DataLoadBalancers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLoadBalancersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                max_items: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLoadBalancersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLoadBalancersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataLoadBalancers_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_AdaptiveRouting {}

impl DataLoadBalancers_Result_AdaptiveRouting { }

impl terrars::ToListMappable for DataLoadBalancers_Result_AdaptiveRouting {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_AdaptiveRouting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_AdaptiveRouting {}

impl BuildDataLoadBalancers_Result_AdaptiveRouting {
    pub fn build(self) -> DataLoadBalancers_Result_AdaptiveRouting {
        DataLoadBalancers_Result_AdaptiveRouting {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_AdaptiveRoutingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_AdaptiveRoutingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancers_Result_AdaptiveRoutingRef {
        DataLoadBalancers_Result_AdaptiveRoutingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_AdaptiveRoutingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failover_across_pools` after provisioning.\nExtends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set false (the default) zero-downtime failover will only occur between origins within the same pool. See `session_affinity_attributes` for control over when sessions are broken or reassigned."]
    pub fn failover_across_pools(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_across_pools", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_LocationStrategy {}

impl DataLoadBalancers_Result_LocationStrategy { }

impl terrars::ToListMappable for DataLoadBalancers_Result_LocationStrategy {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_LocationStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_LocationStrategy {}

impl BuildDataLoadBalancers_Result_LocationStrategy {
    pub fn build(self) -> DataLoadBalancers_Result_LocationStrategy {
        DataLoadBalancers_Result_LocationStrategy {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_LocationStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_LocationStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancers_Result_LocationStrategyRef {
        DataLoadBalancers_Result_LocationStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_LocationStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nDetermines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful.\n- `\"pop\"`: Use the Cloudflare PoP location.\n- `\"resolver_ip\"`: Use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, use the Cloudflare PoP location.\nAvailable values: \"pop\", \"resolver_ip\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefer_ecs` after provisioning.\nWhether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location.\n- `\"always\"`: Always prefer ECS.\n- `\"never\"`: Never prefer ECS.\n- `\"proximity\"`: Prefer ECS only when `steering_policy=\"proximity\"`.\n- `\"geo\"`: Prefer ECS only when `steering_policy=\"geo\"`.\nAvailable values: \"always\", \"never\", \"proximity\", \"geo\"."]
    pub fn prefer_ecs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefer_ecs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_RandomSteering {}

impl DataLoadBalancers_Result_RandomSteering { }

impl terrars::ToListMappable for DataLoadBalancers_Result_RandomSteering {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_RandomSteering>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_RandomSteering {}

impl BuildDataLoadBalancers_Result_RandomSteering {
    pub fn build(self) -> DataLoadBalancers_Result_RandomSteering {
        DataLoadBalancers_Result_RandomSteering {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_RandomSteeringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_RandomSteeringRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancers_Result_RandomSteeringRef {
        DataLoadBalancers_Result_RandomSteeringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_RandomSteeringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_weight` after provisioning.\nThe default weight for pools in the load balancer that are not specified in the pool_weights map."]
    pub fn default_weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_weight", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pool_weights` after provisioning.\nA mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer."]
    pub fn pool_weights(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.pool_weights", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_Rules_FixedResponse {}

impl DataLoadBalancers_Result_Rules_FixedResponse { }

impl terrars::ToListMappable for DataLoadBalancers_Result_Rules_FixedResponse {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_Rules_FixedResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_Rules_FixedResponse {}

impl BuildDataLoadBalancers_Result_Rules_FixedResponse {
    pub fn build(self) -> DataLoadBalancers_Result_Rules_FixedResponse {
        DataLoadBalancers_Result_Rules_FixedResponse {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_Rules_FixedResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_Rules_FixedResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancers_Result_Rules_FixedResponseRef {
        DataLoadBalancers_Result_Rules_FixedResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_Rules_FixedResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nThe http 'Content-Type' header to include in the response."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe http 'Location' header to include in the response."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `message_body` after provisioning.\nText to include as the http body."]
    pub fn message_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nThe http status code to respond with."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_Rules_Overrides_AdaptiveRouting {}

impl DataLoadBalancers_Result_Rules_Overrides_AdaptiveRouting { }

impl terrars::ToListMappable for DataLoadBalancers_Result_Rules_Overrides_AdaptiveRouting {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_Rules_Overrides_AdaptiveRouting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_Rules_Overrides_AdaptiveRouting {}

impl BuildDataLoadBalancers_Result_Rules_Overrides_AdaptiveRouting {
    pub fn build(self) -> DataLoadBalancers_Result_Rules_Overrides_AdaptiveRouting {
        DataLoadBalancers_Result_Rules_Overrides_AdaptiveRouting {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_Rules_Overrides_AdaptiveRoutingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_Rules_Overrides_AdaptiveRoutingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancers_Result_Rules_Overrides_AdaptiveRoutingRef {
        DataLoadBalancers_Result_Rules_Overrides_AdaptiveRoutingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_Rules_Overrides_AdaptiveRoutingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failover_across_pools` after provisioning.\nExtends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set false (the default) zero-downtime failover will only occur between origins within the same pool. See `session_affinity_attributes` for control over when sessions are broken or reassigned."]
    pub fn failover_across_pools(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_across_pools", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_Rules_Overrides_LocationStrategy {}

impl DataLoadBalancers_Result_Rules_Overrides_LocationStrategy { }

impl terrars::ToListMappable for DataLoadBalancers_Result_Rules_Overrides_LocationStrategy {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_Rules_Overrides_LocationStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_Rules_Overrides_LocationStrategy {}

impl BuildDataLoadBalancers_Result_Rules_Overrides_LocationStrategy {
    pub fn build(self) -> DataLoadBalancers_Result_Rules_Overrides_LocationStrategy {
        DataLoadBalancers_Result_Rules_Overrides_LocationStrategy {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_Rules_Overrides_LocationStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_Rules_Overrides_LocationStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancers_Result_Rules_Overrides_LocationStrategyRef {
        DataLoadBalancers_Result_Rules_Overrides_LocationStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_Rules_Overrides_LocationStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nDetermines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful.\n- `\"pop\"`: Use the Cloudflare PoP location.\n- `\"resolver_ip\"`: Use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, use the Cloudflare PoP location.\nAvailable values: \"pop\", \"resolver_ip\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefer_ecs` after provisioning.\nWhether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location.\n- `\"always\"`: Always prefer ECS.\n- `\"never\"`: Never prefer ECS.\n- `\"proximity\"`: Prefer ECS only when `steering_policy=\"proximity\"`.\n- `\"geo\"`: Prefer ECS only when `steering_policy=\"geo\"`.\nAvailable values: \"always\", \"never\", \"proximity\", \"geo\"."]
    pub fn prefer_ecs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefer_ecs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_Rules_Overrides_RandomSteering {}

impl DataLoadBalancers_Result_Rules_Overrides_RandomSteering { }

impl terrars::ToListMappable for DataLoadBalancers_Result_Rules_Overrides_RandomSteering {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_Rules_Overrides_RandomSteering>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_Rules_Overrides_RandomSteering {}

impl BuildDataLoadBalancers_Result_Rules_Overrides_RandomSteering {
    pub fn build(self) -> DataLoadBalancers_Result_Rules_Overrides_RandomSteering {
        DataLoadBalancers_Result_Rules_Overrides_RandomSteering {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_Rules_Overrides_RandomSteeringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_Rules_Overrides_RandomSteeringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancers_Result_Rules_Overrides_RandomSteeringRef {
        DataLoadBalancers_Result_Rules_Overrides_RandomSteeringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_Rules_Overrides_RandomSteeringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_weight` after provisioning.\nThe default weight for pools in the load balancer that are not specified in the pool_weights map."]
    pub fn default_weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_weight", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pool_weights` after provisioning.\nA mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer."]
    pub fn pool_weights(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.pool_weights", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drain_duration: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributes {
    #[doc =
        "Set the field `drain_duration`.\nConfigures the drain duration in seconds. This field is only used when session affinity is enabled on the load balancer."]
    pub fn set_drain_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.drain_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributes {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributes {}

impl BuildDataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributes {
    pub fn build(self) -> DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributes {
        DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributes {
            drain_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributesRef {
        DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `drain_duration` after provisioning.\nConfigures the drain duration in seconds. This field is only used when session affinity is enabled on the load balancer."]
    pub fn drain_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drain_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nConfigures the names of HTTP headers to base session affinity on when header `session_affinity` is enabled. At least one HTTP header name must be provided. To specify the exact cookies to be used, include an item in the following format: `\"cookie:<cookie-name-1>,<cookie-name-2>\"` (example) where everything after the colon is a comma-separated list of cookie names. Providing only `\"cookie\"` will result in all cookies being used. The default max number of HTTP header names that can be provided depends on your plan: 5 for Enterprise, 1 for all other plans."]
    pub fn headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_all_headers` after provisioning.\nWhen header `session_affinity` is enabled, this option can be used to specify how HTTP headers on load balancing requests will be used. The supported values are: - `\"true\"`: Load balancing requests must contain *all* of the HTTP headers specified by the `headers` session affinity attribute, otherwise sessions aren't created. - `\"false\"`: Load balancing requests must contain *at least one* of the HTTP headers specified by the `headers` session affinity attribute, otherwise sessions aren't created."]
    pub fn require_all_headers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_all_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `samesite` after provisioning.\nConfigures the SameSite attribute on session affinity cookie. Value \"Auto\" will be translated to \"Lax\" or \"None\" depending if Always Use HTTPS is enabled. Note: when using value \"None\", the secure attribute can not be set to \"Never\".\nAvailable values: \"Auto\", \"Lax\", \"None\", \"Strict\"."]
    pub fn samesite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.samesite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secure` after provisioning.\nConfigures the Secure attribute on session affinity cookie. Value \"Always\" indicates the Secure attribute will be set in the Set-Cookie header, \"Never\" indicates the Secure attribute will not be set, and \"Auto\" will set the Secure attribute depending if Always Use HTTPS is enabled.\nAvailable values: \"Auto\", \"Always\", \"Never\"."]
    pub fn secure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zero_downtime_failover` after provisioning.\nConfigures the zero-downtime failover between origins within a pool when session affinity is enabled. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. The supported values are: - `\"none\"`: No failover takes place for sessions pinned to the origin (default). - `\"temporary\"`: Traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. - `\"sticky\"`: The session affinity cookie is updated and subsequent requests are sent to the new origin. Note: Zero-downtime failover with sticky sessions is currently not supported for session affinity by header.\nAvailable values: \"none\", \"temporary\", \"sticky\"."]
    pub fn zero_downtime_failover(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zero_downtime_failover", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_Rules_Overrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_pools: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pop_pools: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_pools: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
}

impl DataLoadBalancers_Result_Rules_Overrides {
    #[doc =
        "Set the field `country_pools`.\nA mapping of country codes to a list of pool IDs (ordered by their failover priority) for the given country. Any country not explicitly defined will fall back to using the corresponding region_pool mapping if it exists else to default_pools."]
    pub fn set_country_pools(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.country_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pop_pools`.\nEnterprise only: A mapping of Cloudflare PoP identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). Any PoPs not explicitly defined will fall back to using the corresponding country_pool, then region_pool mapping if it exists else to default_pools."]
    pub fn set_pop_pools(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.pop_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region_pools`.\nA mapping of region codes to a list of pool IDs (ordered by their failover priority) for the given region. Any regions not explicitly defined will fall back to using default_pools."]
    pub fn set_region_pools(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.region_pools = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadBalancers_Result_Rules_Overrides {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_Rules_Overrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_Rules_Overrides {}

impl BuildDataLoadBalancers_Result_Rules_Overrides {
    pub fn build(self) -> DataLoadBalancers_Result_Rules_Overrides {
        DataLoadBalancers_Result_Rules_Overrides {
            country_pools: core::default::Default::default(),
            pop_pools: core::default::Default::default(),
            region_pools: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_Rules_OverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_Rules_OverridesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancers_Result_Rules_OverridesRef {
        DataLoadBalancers_Result_Rules_OverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_Rules_OverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `adaptive_routing` after provisioning.\nControls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests. For example, zero-downtime failover occurs immediately when an origin becomes unavailable due to HTTP 521, 522, or 523 response codes. If there is another healthy origin in the same pool, the request is retried once against this alternate origin."]
    pub fn adaptive_routing(&self) -> DataLoadBalancers_Result_Rules_Overrides_AdaptiveRoutingRef {
        DataLoadBalancers_Result_Rules_Overrides_AdaptiveRoutingRef::new(
            self.shared().clone(),
            format!("{}.adaptive_routing", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `country_pools` after provisioning.\nA mapping of country codes to a list of pool IDs (ordered by their failover priority) for the given country. Any country not explicitly defined will fall back to using the corresponding region_pool mapping if it exists else to default_pools."]
    pub fn country_pools(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.country_pools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_pools` after provisioning.\nA list of pool IDs ordered by their failover priority. Pools defined here are used by default, or when region_pools are not configured for a given region."]
    pub fn default_pools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_pools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fallback_pool` after provisioning.\nThe pool ID to use when all other pools are detected as unhealthy."]
    pub fn fallback_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location_strategy` after provisioning.\nControls location-based steering for non-proxied requests. See `steering_policy` to learn how steering is affected."]
    pub fn location_strategy(&self) -> DataLoadBalancers_Result_Rules_Overrides_LocationStrategyRef {
        DataLoadBalancers_Result_Rules_Overrides_LocationStrategyRef::new(
            self.shared().clone(),
            format!("{}.location_strategy", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `pop_pools` after provisioning.\nEnterprise only: A mapping of Cloudflare PoP identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). Any PoPs not explicitly defined will fall back to using the corresponding country_pool, then region_pool mapping if it exists else to default_pools."]
    pub fn pop_pools(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.pop_pools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `random_steering` after provisioning.\nConfigures pool weights.\n- `steering_policy=\"random\"`: A random pool is selected with probability proportional to pool weights.\n- `steering_policy=\"least_outstanding_requests\"`: Use pool weights to scale each pool's outstanding requests.\n- `steering_policy=\"least_connections\"`: Use pool weights to scale each pool's open connections."]
    pub fn random_steering(&self) -> DataLoadBalancers_Result_Rules_Overrides_RandomSteeringRef {
        DataLoadBalancers_Result_Rules_Overrides_RandomSteeringRef::new(
            self.shared().clone(),
            format!("{}.random_steering", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `region_pools` after provisioning.\nA mapping of region codes to a list of pool IDs (ordered by their failover priority) for the given region. Any regions not explicitly defined will fall back to using default_pools."]
    pub fn region_pools(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.region_pools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity` after provisioning.\nSpecifies the type of session affinity the load balancer should use unless specified as `\"none\"`. The supported types are: - `\"cookie\"`: On the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy, then a new origin server is calculated and used. - `\"ip_cookie\"`: Behaves the same as `\"cookie\"` except the initial origin selection is stable and based on the client's ip address. - `\"header\"`: On the first request to a proxied load balancer, a session key based on the configured HTTP headers (see `session_affinity_attributes.headers`) is generated, encoding the request headers used for storing in the load balancer session state which origin the request will be forwarded to. Subsequent requests to the load balancer with the same headers will be sent to the same origin server, for the duration of the session and as long as the origin server remains healthy. If the session has been idle for the duration of `session_affinity_ttl` seconds or the origin server is unhealthy, then a new origin server is calculated and used. See `headers` in `session_affinity_attributes` for additional required configuration.\nAvailable values: \"none\", \"cookie\", \"ip_cookie\", \"header\"."]
    pub fn session_affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity_attributes` after provisioning.\nConfigures attributes for session affinity."]
    pub fn session_affinity_attributes(&self) -> DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributesRef {
        DataLoadBalancers_Result_Rules_Overrides_SessionAffinityAttributesRef::new(
            self.shared().clone(),
            format!("{}.session_affinity_attributes", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `session_affinity_ttl` after provisioning.\nTime, in seconds, until a client's session expires after being created. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. The accepted ranges per `session_affinity` policy are: - `\"cookie\"` / `\"ip_cookie\"`: The current default of 23 hours will be used unless explicitly set. The accepted range of values is between [1800, 604800]. - `\"header\"`: The current default of 1800 seconds will be used unless explicitly set. The accepted range of values is between [30, 3600]. Note: With session affinity by header, sessions only expire after they haven't been used for the number of seconds specified."]
    pub fn session_affinity_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `steering_policy` after provisioning.\nSteering Policy for this load balancer.\n- `\"off\"`: Use `default_pools`.\n- `\"geo\"`: Use `region_pools`/`country_pools`/`pop_pools`. For non-proxied requests, the country for `country_pools` is determined by `location_strategy`.\n- `\"random\"`: Select a pool randomly.\n- `\"dynamic_latency\"`: Use round trip time to select the closest pool in default_pools (requires pool health checks).\n- `\"proximity\"`: Use the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by `location_strategy` for non-proxied requests.\n- `\"least_outstanding_requests\"`: Select a pool by taking into consideration `random_steering` weights, as well as each pool's number of outstanding requests. Pools with more pending requests are weighted proportionately less relative to others.\n- `\"least_connections\"`: Select a pool by taking into consideration `random_steering` weights, as well as each pool's number of open connections. Pools with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections.\n- `\"\"`: Will map to `\"geo\"` if you use `region_pools`/`country_pools`/`pop_pools` otherwise `\"off\"`.\nAvailable values: \"off\", \"geo\", \"random\", \"dynamic_latency\", \"proximity\", \"least_outstanding_requests\", \"least_connections\", \"\"."]
    pub fn steering_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.steering_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This only applies to gray-clouded (unproxied) load balancers."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_Rules {}

impl DataLoadBalancers_Result_Rules { }

impl terrars::ToListMappable for DataLoadBalancers_Result_Rules {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_Rules {}

impl BuildDataLoadBalancers_Result_Rules {
    pub fn build(self) -> DataLoadBalancers_Result_Rules {
        DataLoadBalancers_Result_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancers_Result_RulesRef {
        DataLoadBalancers_Result_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nThe condition expressions to evaluate. If the condition evaluates to true, the overrides or fixed_response in this rule will be applied. An empty condition is always true. For more details on condition expressions, please see https://developers.cloudflare.com/load-balancing/understand-basics/load-balancing-rules/expressions."]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDisable this specific rule. It will no longer be evaluated by this load balancer."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fixed_response` after provisioning.\nA collection of fields used to directly respond to the eyeball instead of routing to a pool. If a fixed_response is supplied the rule will be marked as terminates."]
    pub fn fixed_response(&self) -> DataLoadBalancers_Result_Rules_FixedResponseRef {
        DataLoadBalancers_Result_Rules_FixedResponseRef::new(
            self.shared().clone(),
            format!("{}.fixed_response", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of this rule. Only used for human readability."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `overrides` after provisioning.\nA collection of overrides to apply to the load balancer when this rule's condition is true. All fields are optional."]
    pub fn overrides(&self) -> DataLoadBalancers_Result_Rules_OverridesRef {
        DataLoadBalancers_Result_Rules_OverridesRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nThe order in which rules should be executed in relation to each other. Lower values are executed first. Values do not need to be sequential. If no value is provided for any rule the array order of the rules field will be used to assign a priority."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `terminates` after provisioning.\nIf this rule's condition is true, this causes rule evaluation to stop after processing this rule."]
    pub fn terminates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminates", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result_SessionAffinityAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drain_duration: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLoadBalancers_Result_SessionAffinityAttributes {
    #[doc =
        "Set the field `drain_duration`.\nConfigures the drain duration in seconds. This field is only used when session affinity is enabled on the load balancer."]
    pub fn set_drain_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.drain_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadBalancers_Result_SessionAffinityAttributes {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result_SessionAffinityAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result_SessionAffinityAttributes {}

impl BuildDataLoadBalancers_Result_SessionAffinityAttributes {
    pub fn build(self) -> DataLoadBalancers_Result_SessionAffinityAttributes {
        DataLoadBalancers_Result_SessionAffinityAttributes { drain_duration: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_Result_SessionAffinityAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_Result_SessionAffinityAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancers_Result_SessionAffinityAttributesRef {
        DataLoadBalancers_Result_SessionAffinityAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_Result_SessionAffinityAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `drain_duration` after provisioning.\nConfigures the drain duration in seconds. This field is only used when session affinity is enabled on the load balancer."]
    pub fn drain_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drain_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nConfigures the names of HTTP headers to base session affinity on when header `session_affinity` is enabled. At least one HTTP header name must be provided. To specify the exact cookies to be used, include an item in the following format: `\"cookie:<cookie-name-1>,<cookie-name-2>\"` (example) where everything after the colon is a comma-separated list of cookie names. Providing only `\"cookie\"` will result in all cookies being used. The default max number of HTTP header names that can be provided depends on your plan: 5 for Enterprise, 1 for all other plans."]
    pub fn headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_all_headers` after provisioning.\nWhen header `session_affinity` is enabled, this option can be used to specify how HTTP headers on load balancing requests will be used. The supported values are: - `\"true\"`: Load balancing requests must contain *all* of the HTTP headers specified by the `headers` session affinity attribute, otherwise sessions aren't created. - `\"false\"`: Load balancing requests must contain *at least one* of the HTTP headers specified by the `headers` session affinity attribute, otherwise sessions aren't created."]
    pub fn require_all_headers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_all_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `samesite` after provisioning.\nConfigures the SameSite attribute on session affinity cookie. Value \"Auto\" will be translated to \"Lax\" or \"None\" depending if Always Use HTTPS is enabled. Note: when using value \"None\", the secure attribute can not be set to \"Never\".\nAvailable values: \"Auto\", \"Lax\", \"None\", \"Strict\"."]
    pub fn samesite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.samesite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secure` after provisioning.\nConfigures the Secure attribute on session affinity cookie. Value \"Always\" indicates the Secure attribute will be set in the Set-Cookie header, \"Never\" indicates the Secure attribute will not be set, and \"Auto\" will set the Secure attribute depending if Always Use HTTPS is enabled.\nAvailable values: \"Auto\", \"Always\", \"Never\"."]
    pub fn secure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zero_downtime_failover` after provisioning.\nConfigures the zero-downtime failover between origins within a pool when session affinity is enabled. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. The supported values are: - `\"none\"`: No failover takes place for sessions pinned to the origin (default). - `\"temporary\"`: Traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. - `\"sticky\"`: The session affinity cookie is updated and subsequent requests are sent to the new origin. Note: Zero-downtime failover with sticky sessions is currently not supported for session affinity by header.\nAvailable values: \"none\", \"temporary\", \"sticky\"."]
    pub fn zero_downtime_failover(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zero_downtime_failover", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancers_Result {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_pools: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pop_pools: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_pools: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
}

impl DataLoadBalancers_Result {
    #[doc =
        "Set the field `country_pools`.\nA mapping of country codes to a list of pool IDs (ordered by their failover priority) for the given country. Any country not explicitly defined will fall back to using the corresponding region_pool mapping if it exists else to default_pools."]
    pub fn set_country_pools(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.country_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pop_pools`.\nEnterprise only: A mapping of Cloudflare PoP identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). Any PoPs not explicitly defined will fall back to using the corresponding country_pool, then region_pool mapping if it exists else to default_pools."]
    pub fn set_pop_pools(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.pop_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region_pools`.\nA mapping of region codes to a list of pool IDs (ordered by their failover priority) for the given region. Any regions not explicitly defined will fall back to using default_pools."]
    pub fn set_region_pools(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.region_pools = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadBalancers_Result {
    type O = terrars::BlockAssignable<DataLoadBalancers_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancers_Result {}

impl BuildDataLoadBalancers_Result {
    pub fn build(self) -> DataLoadBalancers_Result {
        DataLoadBalancers_Result {
            country_pools: core::default::Default::default(),
            pop_pools: core::default::Default::default(),
            region_pools: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancers_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancers_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancers_ResultRef {
        DataLoadBalancers_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancers_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `adaptive_routing` after provisioning.\nControls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests. For example, zero-downtime failover occurs immediately when an origin becomes unavailable due to HTTP 521, 522, or 523 response codes. If there is another healthy origin in the same pool, the request is retried once against this alternate origin."]
    pub fn adaptive_routing(&self) -> DataLoadBalancers_Result_AdaptiveRoutingRef {
        DataLoadBalancers_Result_AdaptiveRoutingRef::new(
            self.shared().clone(),
            format!("{}.adaptive_routing", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `country_pools` after provisioning.\nA mapping of country codes to a list of pool IDs (ordered by their failover priority) for the given country. Any country not explicitly defined will fall back to using the corresponding region_pool mapping if it exists else to default_pools."]
    pub fn country_pools(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.country_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_pools` after provisioning.\nA list of pool IDs ordered by their failover priority. Pools defined here are used by default, or when region_pools are not configured for a given region."]
    pub fn default_pools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nObject description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable (the default) this load balancer."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fallback_pool` after provisioning.\nThe pool ID to use when all other pools are detected as unhealthy."]
    pub fn fallback_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location_strategy` after provisioning.\nControls location-based steering for non-proxied requests. See `steering_policy` to learn how steering is affected."]
    pub fn location_strategy(&self) -> DataLoadBalancers_Result_LocationStrategyRef {
        DataLoadBalancers_Result_LocationStrategyRef::new(
            self.shared().clone(),
            format!("{}.location_strategy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe DNS hostname to associate with your Load Balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the Load Balancer will take precedence and the DNS record will not be used."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nList of networks where Load Balancer or Pool is enabled."]
    pub fn networks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pop_pools` after provisioning.\nEnterprise only: A mapping of Cloudflare PoP identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). Any PoPs not explicitly defined will fall back to using the corresponding country_pool, then region_pool mapping if it exists else to default_pools."]
    pub fn pop_pools(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.pop_pools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxied` after provisioning.\nWhether the hostname should be gray clouded (false) or orange clouded (true)."]
    pub fn proxied(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.base))
    }

    #[doc =
        "Get a reference to the value of field `random_steering` after provisioning.\nConfigures pool weights.\n- `steering_policy=\"random\"`: A random pool is selected with probability proportional to pool weights.\n- `steering_policy=\"least_outstanding_requests\"`: Use pool weights to scale each pool's outstanding requests.\n- `steering_policy=\"least_connections\"`: Use pool weights to scale each pool's open connections."]
    pub fn random_steering(&self) -> DataLoadBalancers_Result_RandomSteeringRef {
        DataLoadBalancers_Result_RandomSteeringRef::new(
            self.shared().clone(),
            format!("{}.random_steering", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `region_pools` after provisioning.\nA mapping of region codes to a list of pool IDs (ordered by their failover priority) for the given region. Any regions not explicitly defined will fall back to using default_pools."]
    pub fn region_pools(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.region_pools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nBETA Field Not General Access: A list of rules for this load balancer to execute."]
    pub fn rules(&self) -> terrars::ListRef<DataLoadBalancers_Result_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity` after provisioning.\nSpecifies the type of session affinity the load balancer should use unless specified as `\"none\"`. The supported types are: - `\"cookie\"`: On the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy, then a new origin server is calculated and used. - `\"ip_cookie\"`: Behaves the same as `\"cookie\"` except the initial origin selection is stable and based on the client's ip address. - `\"header\"`: On the first request to a proxied load balancer, a session key based on the configured HTTP headers (see `session_affinity_attributes.headers`) is generated, encoding the request headers used for storing in the load balancer session state which origin the request will be forwarded to. Subsequent requests to the load balancer with the same headers will be sent to the same origin server, for the duration of the session and as long as the origin server remains healthy. If the session has been idle for the duration of `session_affinity_ttl` seconds or the origin server is unhealthy, then a new origin server is calculated and used. See `headers` in `session_affinity_attributes` for additional required configuration.\nAvailable values: \"none\", \"cookie\", \"ip_cookie\", \"header\"."]
    pub fn session_affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity_attributes` after provisioning.\nConfigures attributes for session affinity."]
    pub fn session_affinity_attributes(&self) -> DataLoadBalancers_Result_SessionAffinityAttributesRef {
        DataLoadBalancers_Result_SessionAffinityAttributesRef::new(
            self.shared().clone(),
            format!("{}.session_affinity_attributes", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `session_affinity_ttl` after provisioning.\nTime, in seconds, until a client's session expires after being created. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. The accepted ranges per `session_affinity` policy are: - `\"cookie\"` / `\"ip_cookie\"`: The current default of 23 hours will be used unless explicitly set. The accepted range of values is between [1800, 604800]. - `\"header\"`: The current default of 1800 seconds will be used unless explicitly set. The accepted range of values is between [30, 3600]. Note: With session affinity by header, sessions only expire after they haven't been used for the number of seconds specified."]
    pub fn session_affinity_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `steering_policy` after provisioning.\nSteering Policy for this load balancer.\n- `\"off\"`: Use `default_pools`.\n- `\"geo\"`: Use `region_pools`/`country_pools`/`pop_pools`. For non-proxied requests, the country for `country_pools` is determined by `location_strategy`.\n- `\"random\"`: Select a pool randomly.\n- `\"dynamic_latency\"`: Use round trip time to select the closest pool in default_pools (requires pool health checks).\n- `\"proximity\"`: Use the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by `location_strategy` for non-proxied requests.\n- `\"least_outstanding_requests\"`: Select a pool by taking into consideration `random_steering` weights, as well as each pool's number of outstanding requests. Pools with more pending requests are weighted proportionately less relative to others.\n- `\"least_connections\"`: Select a pool by taking into consideration `random_steering` weights, as well as each pool's number of open connections. Pools with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections.\n- `\"\"`: Will map to `\"geo\"` if you use `region_pools`/`country_pools`/`pop_pools` otherwise `\"off\"`.\nAvailable values: \"off\", \"geo\", \"random\", \"dynamic_latency\", \"proximity\", \"least_outstanding_requests\", \"least_connections\", \"\"."]
    pub fn steering_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.steering_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This only applies to gray-clouded (unproxied) load balancers."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_name` after provisioning.\n"]
    pub fn zone_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_name", self.base))
    }
}
