use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct LoadBalancerPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_regions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latitude: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    longitude: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_origins: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitor: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_shedding: Option<Vec<LoadBalancerPoolLoadSheddingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_steering: Option<Vec<LoadBalancerPoolOriginSteeringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origins: Option<Vec<LoadBalancerPoolOriginsEl>>,
    dynamic: LoadBalancerPoolDynamic,
}

struct LoadBalancerPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LoadBalancerPoolData>,
}

#[derive(Clone)]
pub struct LoadBalancerPool(Rc<LoadBalancerPool_>);

impl LoadBalancerPool {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `check_regions`.\nA list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api)."]
    pub fn set_check_regions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().check_regions = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nFree text description."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nWhether to enable (the default) this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any). Defaults to `true`."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `latitude`.\nThe latitude this pool is physically located at; used for proximity steering."]
    pub fn set_latitude(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().latitude = Some(v.into());
        self
    }

    #[doc= "Set the field `longitude`.\nThe longitude this pool is physically located at; used for proximity steering."]
    pub fn set_longitude(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().longitude = Some(v.into());
        self
    }

    #[doc= "Set the field `minimum_origins`.\nThe minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`."]
    pub fn set_minimum_origins(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().minimum_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `monitor`.\nThe ID of the Monitor to use for health checking origins within this pool."]
    pub fn set_monitor(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().monitor = Some(v.into());
        self
    }

    #[doc= "Set the field `notification_email`.\nThe email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list."]
    pub fn set_notification_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().notification_email = Some(v.into());
        self
    }

    #[doc= "Set the field `load_shedding`.\n"]
    pub fn set_load_shedding(self, v: impl Into<BlockAssignable<LoadBalancerPoolLoadSheddingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_shedding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_shedding = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `origin_steering`.\n"]
    pub fn set_origin_steering(self, v: impl Into<BlockAssignable<LoadBalancerPoolOriginSteeringEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_steering = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_steering = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `origins`.\n"]
    pub fn set_origins(self, v: impl Into<BlockAssignable<LoadBalancerPoolOriginsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origins = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origins = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `check_regions` after provisioning.\nA list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api)."]
    pub fn check_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.check_regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nFree text description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable (the default) this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any). Defaults to `true`."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `latitude` after provisioning.\nThe latitude this pool is physically located at; used for proximity steering."]
    pub fn latitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `longitude` after provisioning.\nThe longitude this pool is physically located at; used for proximity steering."]
    pub fn longitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `minimum_origins` after provisioning.\nThe minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`."]
    pub fn minimum_origins(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_origins", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitor` after provisioning.\nThe ID of the Monitor to use for health checking origins within this pool."]
    pub fn monitor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA short name (tag) for the pool."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notification_email` after provisioning.\nThe email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list."]
    pub fn notification_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notification_email", self.extract_ref()))
    }
}

impl Referable for LoadBalancerPool {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LoadBalancerPool { }

impl ToListMappable for LoadBalancerPool {
    type O = ListRef<LoadBalancerPoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LoadBalancerPool_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_load_balancer_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoadBalancerPool {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "A short name (tag) for the pool."]
    pub name: PrimField<String>,
}

impl BuildLoadBalancerPool {
    pub fn build(self, stack: &mut Stack) -> LoadBalancerPool {
        let out = LoadBalancerPool(Rc::new(LoadBalancerPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LoadBalancerPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                check_regions: core::default::Default::default(),
                description: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                latitude: core::default::Default::default(),
                longitude: core::default::Default::default(),
                minimum_origins: core::default::Default::default(),
                monitor: core::default::Default::default(),
                name: self.name,
                notification_email: core::default::Default::default(),
                load_shedding: core::default::Default::default(),
                origin_steering: core::default::Default::default(),
                origins: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoadBalancerPoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoadBalancerPoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `check_regions` after provisioning.\nA list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api)."]
    pub fn check_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.check_regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nFree text description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable (the default) this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any). Defaults to `true`."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `latitude` after provisioning.\nThe latitude this pool is physically located at; used for proximity steering."]
    pub fn latitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `longitude` after provisioning.\nThe longitude this pool is physically located at; used for proximity steering."]
    pub fn longitude(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `minimum_origins` after provisioning.\nThe minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`."]
    pub fn minimum_origins(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_origins", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitor` after provisioning.\nThe ID of the Monitor to use for health checking origins within this pool."]
    pub fn monitor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA short name (tag) for the pool."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notification_email` after provisioning.\nThe email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list."]
    pub fn notification_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notification_email", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPoolLoadSheddingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_policy: Option<PrimField<String>>,
}

impl LoadBalancerPoolLoadSheddingEl {
    #[doc= "Set the field `default_percent`.\nPercent of traffic to shed 0 - 100. Defaults to `0`."]
    pub fn set_default_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_percent = Some(v.into());
        self
    }

    #[doc= "Set the field `default_policy`.\nMethod of shedding traffic. Available values: `\"\"`, `hash`, `random`. Defaults to `\"\"`."]
    pub fn set_default_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `session_percent`.\nPercent of session traffic to shed 0 - 100. Defaults to `0`."]
    pub fn set_session_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_percent = Some(v.into());
        self
    }

    #[doc= "Set the field `session_policy`.\nMethod of shedding traffic. Available values: `\"\"`, `hash`. Defaults to `\"\"`."]
    pub fn set_session_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_policy = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerPoolLoadSheddingEl {
    type O = BlockAssignable<LoadBalancerPoolLoadSheddingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPoolLoadSheddingEl {}

impl BuildLoadBalancerPoolLoadSheddingEl {
    pub fn build(self) -> LoadBalancerPoolLoadSheddingEl {
        LoadBalancerPoolLoadSheddingEl {
            default_percent: core::default::Default::default(),
            default_policy: core::default::Default::default(),
            session_percent: core::default::Default::default(),
            session_policy: core::default::Default::default(),
        }
    }
}

pub struct LoadBalancerPoolLoadSheddingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerPoolLoadSheddingElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerPoolLoadSheddingElRef {
        LoadBalancerPoolLoadSheddingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPoolLoadSheddingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_percent` after provisioning.\nPercent of traffic to shed 0 - 100. Defaults to `0`."]
    pub fn default_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_percent", self.base))
    }

    #[doc= "Get a reference to the value of field `default_policy` after provisioning.\nMethod of shedding traffic. Available values: `\"\"`, `hash`, `random`. Defaults to `\"\"`."]
    pub fn default_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `session_percent` after provisioning.\nPercent of session traffic to shed 0 - 100. Defaults to `0`."]
    pub fn session_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_percent", self.base))
    }

    #[doc= "Get a reference to the value of field `session_policy` after provisioning.\nMethod of shedding traffic. Available values: `\"\"`, `hash`. Defaults to `\"\"`."]
    pub fn session_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPoolOriginSteeringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
}

impl LoadBalancerPoolOriginSteeringEl {
    #[doc= "Set the field `policy`.\nOrigin steering policy to be used. Available values: `\"\"`, `hash`, `random`. Defaults to `random`."]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerPoolOriginSteeringEl {
    type O = BlockAssignable<LoadBalancerPoolOriginSteeringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPoolOriginSteeringEl {}

impl BuildLoadBalancerPoolOriginSteeringEl {
    pub fn build(self) -> LoadBalancerPoolOriginSteeringEl {
        LoadBalancerPoolOriginSteeringEl { policy: core::default::Default::default() }
    }
}

pub struct LoadBalancerPoolOriginSteeringElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerPoolOriginSteeringElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerPoolOriginSteeringElRef {
        LoadBalancerPoolOriginSteeringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPoolOriginSteeringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `policy` after provisioning.\nOrigin steering policy to be used. Available values: `\"\"`, `hash`, `random`. Defaults to `random`."]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPoolOriginsElHeaderEl {
    header: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl LoadBalancerPoolOriginsElHeaderEl { }

impl ToListMappable for LoadBalancerPoolOriginsElHeaderEl {
    type O = BlockAssignable<LoadBalancerPoolOriginsElHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPoolOriginsElHeaderEl {
    #[doc= "HTTP Header name."]
    pub header: PrimField<String>,
    #[doc= "Values for the HTTP headers."]
    pub values: SetField<PrimField<String>>,
}

impl BuildLoadBalancerPoolOriginsElHeaderEl {
    pub fn build(self) -> LoadBalancerPoolOriginsElHeaderEl {
        LoadBalancerPoolOriginsElHeaderEl {
            header: self.header,
            values: self.values,
        }
    }
}

pub struct LoadBalancerPoolOriginsElHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerPoolOriginsElHeaderElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerPoolOriginsElHeaderElRef {
        LoadBalancerPoolOriginsElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPoolOriginsElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header` after provisioning.\nHTTP Header name."]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\nValues for the HTTP headers."]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoadBalancerPoolOriginsElDynamic {
    header: Option<DynamicBlock<LoadBalancerPoolOriginsElHeaderEl>>,
}

#[derive(Serialize)]
pub struct LoadBalancerPoolOriginsEl {
    address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<LoadBalancerPoolOriginsElHeaderEl>>,
    dynamic: LoadBalancerPoolOriginsElDynamic,
}

impl LoadBalancerPoolOriginsEl {
    #[doc= "Set the field `enabled`.\nWhether this origin is enabled. Disabled origins will not receive traffic and are excluded from health checks. Defaults to `true`."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\nThe weight (0.01 - 1.00) of this origin, relative to other origins in the pool. Equal values mean equal weighting. A weight of 0 means traffic will not be sent to this origin, but health is still checked. Defaults to `1`."]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }

    #[doc= "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<BlockAssignable<LoadBalancerPoolOriginsElHeaderEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for LoadBalancerPoolOriginsEl {
    type O = BlockAssignable<LoadBalancerPoolOriginsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPoolOriginsEl {
    #[doc= "The IP address (IPv4 or IPv6) of the origin, or the publicly addressable hostname."]
    pub address: PrimField<String>,
    #[doc= "A human-identifiable name for the origin."]
    pub name: PrimField<String>,
}

impl BuildLoadBalancerPoolOriginsEl {
    pub fn build(self) -> LoadBalancerPoolOriginsEl {
        LoadBalancerPoolOriginsEl {
            address: self.address,
            enabled: core::default::Default::default(),
            name: self.name,
            weight: core::default::Default::default(),
            header: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LoadBalancerPoolOriginsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerPoolOriginsElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerPoolOriginsElRef {
        LoadBalancerPoolOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPoolOriginsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\nThe IP address (IPv4 or IPv6) of the origin, or the publicly addressable hostname."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether this origin is enabled. Disabled origins will not receive traffic and are excluded from health checks. Defaults to `true`."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA human-identifiable name for the origin."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nThe weight (0.01 - 1.00) of this origin, relative to other origins in the pool. Equal values mean equal weighting. A weight of 0 means traffic will not be sent to this origin, but health is still checked. Defaults to `1`."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoadBalancerPoolDynamic {
    load_shedding: Option<DynamicBlock<LoadBalancerPoolLoadSheddingEl>>,
    origin_steering: Option<DynamicBlock<LoadBalancerPoolOriginSteeringEl>>,
    origins: Option<DynamicBlock<LoadBalancerPoolOriginsEl>>,
}
