use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct LoadBalancerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    default_pool_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    fallback_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxied: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_affinity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_affinity_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    steering_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    adaptive_routing: Option<Vec<LoadBalancerAdaptiveRoutingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_pools: Option<Vec<LoadBalancerCountryPoolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_strategy: Option<Vec<LoadBalancerLocationStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pop_pools: Option<Vec<LoadBalancerPopPoolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    random_steering: Option<Vec<LoadBalancerRandomSteeringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_pools: Option<Vec<LoadBalancerRegionPoolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<LoadBalancerRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_affinity_attributes: Option<Vec<LoadBalancerSessionAffinityAttributesEl>>,
    dynamic: LoadBalancerDynamic,
}

struct LoadBalancer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LoadBalancerData>,
}

#[derive(Clone)]
pub struct LoadBalancer(Rc<LoadBalancer_>);

impl LoadBalancer {
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

    #[doc= "Set the field `description`.\nFree text description."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nEnable or disable the load balancer. Defaults to `true`."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `proxied`.\nWhether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`."]
    pub fn set_proxied(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().proxied = Some(v.into());
        self
    }

    #[doc= "Set the field `session_affinity`.\nSpecifies the type of session affinity the load balancer should use unless specified as `none` or `\"\"` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `\"\"`, `none`, `cookie`, `ip_cookie`. Defaults to `none`."]
    pub fn set_session_affinity(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().session_affinity = Some(v.into());
        self
    }

    #[doc= "Set the field `session_affinity_ttl`.\nTime, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless [`session_affinity_ttl`](#session_affinity_ttl) is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`."]
    pub fn set_session_affinity_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().session_affinity_ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `steering_policy`.\nThe method the load balancer uses to determine the route to your origin. Value `off` uses [`default_pool_ids`](#default_pool_ids). Value `geo` uses [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools). For non-proxied requests, the [`country`](#country) for [`country_pools`](#country_pools) is determined by [`location_strategy`](#location_strategy). Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in [`default_pool_ids`](#default_pool_ids) (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by [`location_strategy`](#location_strategy) for non-proxied requests. Value `\"\"` maps to `geo` if you use [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `\"\"` Defaults to `\"\"`."]
    pub fn set_steering_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().steering_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `ttl`.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`."]
    pub fn set_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `adaptive_routing`.\n"]
    pub fn set_adaptive_routing(self, v: impl Into<BlockAssignable<LoadBalancerAdaptiveRoutingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().adaptive_routing = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.adaptive_routing = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `country_pools`.\n"]
    pub fn set_country_pools(self, v: impl Into<BlockAssignable<LoadBalancerCountryPoolsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().country_pools = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.country_pools = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `location_strategy`.\n"]
    pub fn set_location_strategy(self, v: impl Into<BlockAssignable<LoadBalancerLocationStrategyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().location_strategy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.location_strategy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `pop_pools`.\n"]
    pub fn set_pop_pools(self, v: impl Into<BlockAssignable<LoadBalancerPopPoolsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pop_pools = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pop_pools = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `random_steering`.\n"]
    pub fn set_random_steering(self, v: impl Into<BlockAssignable<LoadBalancerRandomSteeringEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().random_steering = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.random_steering = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `region_pools`.\n"]
    pub fn set_region_pools(self, v: impl Into<BlockAssignable<LoadBalancerRegionPoolsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().region_pools = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.region_pools = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<LoadBalancerRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `session_affinity_attributes`.\n"]
    pub fn set_session_affinity_attributes(
        self,
        v: impl Into<BlockAssignable<LoadBalancerSessionAffinityAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().session_affinity_attributes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.session_affinity_attributes = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_pool_ids` after provisioning.\nA list of pool IDs ordered by their failover priority. Used whenever [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) are not defined."]
    pub fn default_pool_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.default_pool_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nFree text description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nEnable or disable the load balancer. Defaults to `true`."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fallback_pool_id` after provisioning.\nThe pool ID to use when all other pools are detected as unhealthy."]
    pub fn fallback_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fallback_pool_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxied` after provisioning.\nWhether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`."]
    pub fn proxied(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_affinity` after provisioning.\nSpecifies the type of session affinity the load balancer should use unless specified as `none` or `\"\"` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `\"\"`, `none`, `cookie`, `ip_cookie`. Defaults to `none`."]
    pub fn session_affinity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_affinity_ttl` after provisioning.\nTime, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless [`session_affinity_ttl`](#session_affinity_ttl) is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`."]
    pub fn session_affinity_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_affinity_ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `steering_policy` after provisioning.\nThe method the load balancer uses to determine the route to your origin. Value `off` uses [`default_pool_ids`](#default_pool_ids). Value `geo` uses [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools). For non-proxied requests, the [`country`](#country) for [`country_pools`](#country_pools) is determined by [`location_strategy`](#location_strategy). Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in [`default_pool_ids`](#default_pool_ids) (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by [`location_strategy`](#location_strategy) for non-proxied requests. Value `\"\"` maps to `geo` if you use [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `\"\"` Defaults to `\"\"`."]
    pub fn steering_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.steering_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<LoadBalancerRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl Referable for LoadBalancer {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LoadBalancer { }

impl ToListMappable for LoadBalancer {
    type O = ListRef<LoadBalancerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LoadBalancer_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_load_balancer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoadBalancer {
    pub tf_id: String,
    #[doc= "A list of pool IDs ordered by their failover priority. Used whenever [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) are not defined."]
    pub default_pool_ids: ListField<PrimField<String>>,
    #[doc= "The pool ID to use when all other pools are detected as unhealthy."]
    pub fallback_pool_id: PrimField<String>,
    #[doc= "The DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used."]
    pub name: PrimField<String>,
    #[doc= "The zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildLoadBalancer {
    pub fn build(self, stack: &mut Stack) -> LoadBalancer {
        let out = LoadBalancer(Rc::new(LoadBalancer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LoadBalancerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_pool_ids: self.default_pool_ids,
                description: core::default::Default::default(),
                enabled: core::default::Default::default(),
                fallback_pool_id: self.fallback_pool_id,
                id: core::default::Default::default(),
                name: self.name,
                proxied: core::default::Default::default(),
                session_affinity: core::default::Default::default(),
                session_affinity_ttl: core::default::Default::default(),
                steering_policy: core::default::Default::default(),
                ttl: core::default::Default::default(),
                zone_id: self.zone_id,
                adaptive_routing: core::default::Default::default(),
                country_pools: core::default::Default::default(),
                location_strategy: core::default::Default::default(),
                pop_pools: core::default::Default::default(),
                random_steering: core::default::Default::default(),
                region_pools: core::default::Default::default(),
                rules: core::default::Default::default(),
                session_affinity_attributes: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoadBalancerRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoadBalancerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_pool_ids` after provisioning.\nA list of pool IDs ordered by their failover priority. Used whenever [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) are not defined."]
    pub fn default_pool_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.default_pool_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nFree text description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nEnable or disable the load balancer. Defaults to `true`."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fallback_pool_id` after provisioning.\nThe pool ID to use when all other pools are detected as unhealthy."]
    pub fn fallback_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fallback_pool_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proxied` after provisioning.\nWhether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`."]
    pub fn proxied(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_affinity` after provisioning.\nSpecifies the type of session affinity the load balancer should use unless specified as `none` or `\"\"` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `\"\"`, `none`, `cookie`, `ip_cookie`. Defaults to `none`."]
    pub fn session_affinity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_affinity_ttl` after provisioning.\nTime, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless [`session_affinity_ttl`](#session_affinity_ttl) is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`."]
    pub fn session_affinity_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_affinity_ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `steering_policy` after provisioning.\nThe method the load balancer uses to determine the route to your origin. Value `off` uses [`default_pool_ids`](#default_pool_ids). Value `geo` uses [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools). For non-proxied requests, the [`country`](#country) for [`country_pools`](#country_pools) is determined by [`location_strategy`](#location_strategy). Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in [`default_pool_ids`](#default_pool_ids) (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by [`location_strategy`](#location_strategy) for non-proxied requests. Value `\"\"` maps to `geo` if you use [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `\"\"` Defaults to `\"\"`."]
    pub fn steering_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.steering_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<LoadBalancerRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerAdaptiveRoutingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_across_pools: Option<PrimField<bool>>,
}

impl LoadBalancerAdaptiveRoutingEl {
    #[doc= "Set the field `failover_across_pools`.\nExtends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool. Defaults to `false`."]
    pub fn set_failover_across_pools(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.failover_across_pools = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerAdaptiveRoutingEl {
    type O = BlockAssignable<LoadBalancerAdaptiveRoutingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerAdaptiveRoutingEl {}

impl BuildLoadBalancerAdaptiveRoutingEl {
    pub fn build(self) -> LoadBalancerAdaptiveRoutingEl {
        LoadBalancerAdaptiveRoutingEl { failover_across_pools: core::default::Default::default() }
    }
}

pub struct LoadBalancerAdaptiveRoutingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerAdaptiveRoutingElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerAdaptiveRoutingElRef {
        LoadBalancerAdaptiveRoutingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerAdaptiveRoutingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `failover_across_pools` after provisioning.\nExtends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool. Defaults to `false`."]
    pub fn failover_across_pools(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.failover_across_pools", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerCountryPoolsEl {
    country: PrimField<String>,
    pool_ids: ListField<PrimField<String>>,
}

impl LoadBalancerCountryPoolsEl { }

impl ToListMappable for LoadBalancerCountryPoolsEl {
    type O = BlockAssignable<LoadBalancerCountryPoolsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerCountryPoolsEl {
    #[doc= "A country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country."]
    pub country: PrimField<String>,
    #[doc= "A list of pool IDs in failover priority to use in the given country."]
    pub pool_ids: ListField<PrimField<String>>,
}

impl BuildLoadBalancerCountryPoolsEl {
    pub fn build(self) -> LoadBalancerCountryPoolsEl {
        LoadBalancerCountryPoolsEl {
            country: self.country,
            pool_ids: self.pool_ids,
        }
    }
}

pub struct LoadBalancerCountryPoolsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerCountryPoolsElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerCountryPoolsElRef {
        LoadBalancerCountryPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerCountryPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `country` after provisioning.\nA country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country."]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc= "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use in the given country."]
    pub fn pool_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerLocationStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefer_ecs: Option<PrimField<String>>,
}

impl LoadBalancerLocationStrategyEl {
    #[doc= "Set the field `mode`.\nDetermines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`. Defaults to `pop`."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `prefer_ecs`.\nWhether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when [`steering_policy=\"proximity\"`](#steering_policy), and `geo` will prefer ECS only when [`steering_policy=\"geo\"`](#steering_policy). Available values: `always`, `never`, `proximity`, `geo`. Defaults to `proximity`."]
    pub fn set_prefer_ecs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefer_ecs = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerLocationStrategyEl {
    type O = BlockAssignable<LoadBalancerLocationStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerLocationStrategyEl {}

impl BuildLoadBalancerLocationStrategyEl {
    pub fn build(self) -> LoadBalancerLocationStrategyEl {
        LoadBalancerLocationStrategyEl {
            mode: core::default::Default::default(),
            prefer_ecs: core::default::Default::default(),
        }
    }
}

pub struct LoadBalancerLocationStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerLocationStrategyElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerLocationStrategyElRef {
        LoadBalancerLocationStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerLocationStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nDetermines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`. Defaults to `pop`."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `prefer_ecs` after provisioning.\nWhether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when [`steering_policy=\"proximity\"`](#steering_policy), and `geo` will prefer ECS only when [`steering_policy=\"geo\"`](#steering_policy). Available values: `always`, `never`, `proximity`, `geo`. Defaults to `proximity`."]
    pub fn prefer_ecs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefer_ecs", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPopPoolsEl {
    pool_ids: ListField<PrimField<String>>,
    pop: PrimField<String>,
}

impl LoadBalancerPopPoolsEl { }

impl ToListMappable for LoadBalancerPopPoolsEl {
    type O = BlockAssignable<LoadBalancerPopPoolsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPopPoolsEl {
    #[doc= "A list of pool IDs in failover priority to use for traffic reaching the given PoP."]
    pub pool_ids: ListField<PrimField<String>>,
    #[doc= "A 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP."]
    pub pop: PrimField<String>,
}

impl BuildLoadBalancerPopPoolsEl {
    pub fn build(self) -> LoadBalancerPopPoolsEl {
        LoadBalancerPopPoolsEl {
            pool_ids: self.pool_ids,
            pop: self.pop,
        }
    }
}

pub struct LoadBalancerPopPoolsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerPopPoolsElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerPopPoolsElRef {
        LoadBalancerPopPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPopPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use for traffic reaching the given PoP."]
    pub fn pool_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `pop` after provisioning.\nA 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP."]
    pub fn pop(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pop", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRandomSteeringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_weight: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_weights: Option<RecField<PrimField<f64>>>,
}

impl LoadBalancerRandomSteeringEl {
    #[doc= "Set the field `default_weight`.\nThe default weight for pools in the load balancer that are not specified in the [`pool_weights`](#pool_weights) map."]
    pub fn set_default_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_weight = Some(v.into());
        self
    }

    #[doc= "Set the field `pool_weights`.\nA mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer."]
    pub fn set_pool_weights(mut self, v: impl Into<RecField<PrimField<f64>>>) -> Self {
        self.pool_weights = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerRandomSteeringEl {
    type O = BlockAssignable<LoadBalancerRandomSteeringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRandomSteeringEl {}

impl BuildLoadBalancerRandomSteeringEl {
    pub fn build(self) -> LoadBalancerRandomSteeringEl {
        LoadBalancerRandomSteeringEl {
            default_weight: core::default::Default::default(),
            pool_weights: core::default::Default::default(),
        }
    }
}

pub struct LoadBalancerRandomSteeringElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRandomSteeringElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRandomSteeringElRef {
        LoadBalancerRandomSteeringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRandomSteeringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_weight` after provisioning.\nThe default weight for pools in the load balancer that are not specified in the [`pool_weights`](#pool_weights) map."]
    pub fn default_weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_weight", self.base))
    }

    #[doc= "Get a reference to the value of field `pool_weights` after provisioning.\nA mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer."]
    pub fn pool_weights(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(self.shared().clone(), format!("{}.pool_weights", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRegionPoolsEl {
    pool_ids: ListField<PrimField<String>>,
    region: PrimField<String>,
}

impl LoadBalancerRegionPoolsEl { }

impl ToListMappable for LoadBalancerRegionPoolsEl {
    type O = BlockAssignable<LoadBalancerRegionPoolsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRegionPoolsEl {
    #[doc= "A list of pool IDs in failover priority to use in the given region."]
    pub pool_ids: ListField<PrimField<String>>,
    #[doc= "A region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region."]
    pub region: PrimField<String>,
}

impl BuildLoadBalancerRegionPoolsEl {
    pub fn build(self) -> LoadBalancerRegionPoolsEl {
        LoadBalancerRegionPoolsEl {
            pool_ids: self.pool_ids,
            region: self.region,
        }
    }
}

pub struct LoadBalancerRegionPoolsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRegionPoolsElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRegionPoolsElRef {
        LoadBalancerRegionPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRegionPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use in the given region."]
    pub fn pool_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nA region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElFixedResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
}

impl LoadBalancerRulesElFixedResponseEl {
    #[doc= "Set the field `content_type`.\nThe value of the HTTP context-type header for this fixed response."]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\nThe value of the HTTP location header for this fixed response."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }

    #[doc= "Set the field `message_body`.\nThe text used as the html body for this fixed response."]
    pub fn set_message_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_body = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code`.\nThe HTTP status code used for this fixed response."]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerRulesElFixedResponseEl {
    type O = BlockAssignable<LoadBalancerRulesElFixedResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElFixedResponseEl {}

impl BuildLoadBalancerRulesElFixedResponseEl {
    pub fn build(self) -> LoadBalancerRulesElFixedResponseEl {
        LoadBalancerRulesElFixedResponseEl {
            content_type: core::default::Default::default(),
            location: core::default::Default::default(),
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct LoadBalancerRulesElFixedResponseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRulesElFixedResponseElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRulesElFixedResponseElRef {
        LoadBalancerRulesElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElFixedResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nThe value of the HTTP context-type header for this fixed response."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe value of the HTTP location header for this fixed response."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc= "Get a reference to the value of field `message_body` after provisioning.\nThe text used as the html body for this fixed response."]
    pub fn message_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_body", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\nThe HTTP status code used for this fixed response."]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElAdaptiveRoutingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_across_pools: Option<PrimField<bool>>,
}

impl LoadBalancerRulesElOverridesElAdaptiveRoutingEl {
    #[doc= "Set the field `failover_across_pools`.\nExtends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool."]
    pub fn set_failover_across_pools(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.failover_across_pools = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerRulesElOverridesElAdaptiveRoutingEl {
    type O = BlockAssignable<LoadBalancerRulesElOverridesElAdaptiveRoutingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesElAdaptiveRoutingEl {}

impl BuildLoadBalancerRulesElOverridesElAdaptiveRoutingEl {
    pub fn build(self) -> LoadBalancerRulesElOverridesElAdaptiveRoutingEl {
        LoadBalancerRulesElOverridesElAdaptiveRoutingEl { failover_across_pools: core::default::Default::default() }
    }
}

pub struct LoadBalancerRulesElOverridesElAdaptiveRoutingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRulesElOverridesElAdaptiveRoutingElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRulesElOverridesElAdaptiveRoutingElRef {
        LoadBalancerRulesElOverridesElAdaptiveRoutingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElAdaptiveRoutingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `failover_across_pools` after provisioning.\nExtends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool."]
    pub fn failover_across_pools(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.failover_across_pools", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElCountryPoolsEl {
    country: PrimField<String>,
    pool_ids: ListField<PrimField<String>>,
}

impl LoadBalancerRulesElOverridesElCountryPoolsEl { }

impl ToListMappable for LoadBalancerRulesElOverridesElCountryPoolsEl {
    type O = BlockAssignable<LoadBalancerRulesElOverridesElCountryPoolsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesElCountryPoolsEl {
    #[doc= "A country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country."]
    pub country: PrimField<String>,
    #[doc= "A list of pool IDs in failover priority to use in the given country."]
    pub pool_ids: ListField<PrimField<String>>,
}

impl BuildLoadBalancerRulesElOverridesElCountryPoolsEl {
    pub fn build(self) -> LoadBalancerRulesElOverridesElCountryPoolsEl {
        LoadBalancerRulesElOverridesElCountryPoolsEl {
            country: self.country,
            pool_ids: self.pool_ids,
        }
    }
}

pub struct LoadBalancerRulesElOverridesElCountryPoolsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRulesElOverridesElCountryPoolsElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRulesElOverridesElCountryPoolsElRef {
        LoadBalancerRulesElOverridesElCountryPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElCountryPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `country` after provisioning.\nA country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country."]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc= "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use in the given country."]
    pub fn pool_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElLocationStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefer_ecs: Option<PrimField<String>>,
}

impl LoadBalancerRulesElOverridesElLocationStrategyEl {
    #[doc= "Set the field `mode`.\nDetermines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `prefer_ecs`.\nWhether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when [`steering_policy=\"proximity\"`](#steering_policy), and `geo` will prefer ECS only when [`steering_policy=\"geo\"`](#steering_policy). Available values: `always`, `never`, `proximity`, `geo`."]
    pub fn set_prefer_ecs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefer_ecs = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerRulesElOverridesElLocationStrategyEl {
    type O = BlockAssignable<LoadBalancerRulesElOverridesElLocationStrategyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesElLocationStrategyEl {}

impl BuildLoadBalancerRulesElOverridesElLocationStrategyEl {
    pub fn build(self) -> LoadBalancerRulesElOverridesElLocationStrategyEl {
        LoadBalancerRulesElOverridesElLocationStrategyEl {
            mode: core::default::Default::default(),
            prefer_ecs: core::default::Default::default(),
        }
    }
}

pub struct LoadBalancerRulesElOverridesElLocationStrategyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRulesElOverridesElLocationStrategyElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRulesElOverridesElLocationStrategyElRef {
        LoadBalancerRulesElOverridesElLocationStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElLocationStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nDetermines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `prefer_ecs` after provisioning.\nWhether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when [`steering_policy=\"proximity\"`](#steering_policy), and `geo` will prefer ECS only when [`steering_policy=\"geo\"`](#steering_policy). Available values: `always`, `never`, `proximity`, `geo`."]
    pub fn prefer_ecs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefer_ecs", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElPopPoolsEl {
    pool_ids: ListField<PrimField<String>>,
    pop: PrimField<String>,
}

impl LoadBalancerRulesElOverridesElPopPoolsEl { }

impl ToListMappable for LoadBalancerRulesElOverridesElPopPoolsEl {
    type O = BlockAssignable<LoadBalancerRulesElOverridesElPopPoolsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesElPopPoolsEl {
    #[doc= "A list of pool IDs in failover priority to use for traffic reaching the given PoP."]
    pub pool_ids: ListField<PrimField<String>>,
    #[doc= "A 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP."]
    pub pop: PrimField<String>,
}

impl BuildLoadBalancerRulesElOverridesElPopPoolsEl {
    pub fn build(self) -> LoadBalancerRulesElOverridesElPopPoolsEl {
        LoadBalancerRulesElOverridesElPopPoolsEl {
            pool_ids: self.pool_ids,
            pop: self.pop,
        }
    }
}

pub struct LoadBalancerRulesElOverridesElPopPoolsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRulesElOverridesElPopPoolsElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRulesElOverridesElPopPoolsElRef {
        LoadBalancerRulesElOverridesElPopPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElPopPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use for traffic reaching the given PoP."]
    pub fn pool_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `pop` after provisioning.\nA 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP."]
    pub fn pop(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pop", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElRandomSteeringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_weight: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_weights: Option<RecField<PrimField<f64>>>,
}

impl LoadBalancerRulesElOverridesElRandomSteeringEl {
    #[doc= "Set the field `default_weight`.\nThe default weight for pools in the load balancer that are not specified in the [`pool_weights`](#pool_weights) map."]
    pub fn set_default_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_weight = Some(v.into());
        self
    }

    #[doc= "Set the field `pool_weights`.\nA mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer."]
    pub fn set_pool_weights(mut self, v: impl Into<RecField<PrimField<f64>>>) -> Self {
        self.pool_weights = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerRulesElOverridesElRandomSteeringEl {
    type O = BlockAssignable<LoadBalancerRulesElOverridesElRandomSteeringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesElRandomSteeringEl {}

impl BuildLoadBalancerRulesElOverridesElRandomSteeringEl {
    pub fn build(self) -> LoadBalancerRulesElOverridesElRandomSteeringEl {
        LoadBalancerRulesElOverridesElRandomSteeringEl {
            default_weight: core::default::Default::default(),
            pool_weights: core::default::Default::default(),
        }
    }
}

pub struct LoadBalancerRulesElOverridesElRandomSteeringElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRulesElOverridesElRandomSteeringElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRulesElOverridesElRandomSteeringElRef {
        LoadBalancerRulesElOverridesElRandomSteeringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElRandomSteeringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_weight` after provisioning.\nThe default weight for pools in the load balancer that are not specified in the [`pool_weights`](#pool_weights) map."]
    pub fn default_weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_weight", self.base))
    }

    #[doc= "Get a reference to the value of field `pool_weights` after provisioning.\nA mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer."]
    pub fn pool_weights(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(self.shared().clone(), format!("{}.pool_weights", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElRegionPoolsEl {
    pool_ids: ListField<PrimField<String>>,
    region: PrimField<String>,
}

impl LoadBalancerRulesElOverridesElRegionPoolsEl { }

impl ToListMappable for LoadBalancerRulesElOverridesElRegionPoolsEl {
    type O = BlockAssignable<LoadBalancerRulesElOverridesElRegionPoolsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesElRegionPoolsEl {
    #[doc= "A list of pool IDs in failover priority to use in the given region."]
    pub pool_ids: ListField<PrimField<String>>,
    #[doc= "A region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region."]
    pub region: PrimField<String>,
}

impl BuildLoadBalancerRulesElOverridesElRegionPoolsEl {
    pub fn build(self) -> LoadBalancerRulesElOverridesElRegionPoolsEl {
        LoadBalancerRulesElOverridesElRegionPoolsEl {
            pool_ids: self.pool_ids,
            region: self.region,
        }
    }
}

pub struct LoadBalancerRulesElOverridesElRegionPoolsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRulesElOverridesElRegionPoolsElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRulesElOverridesElRegionPoolsElRef {
        LoadBalancerRulesElOverridesElRegionPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElRegionPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use in the given region."]
    pub fn pool_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nA region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElSessionAffinityAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    samesite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_downtime_failover: Option<PrimField<String>>,
}

impl LoadBalancerRulesElOverridesElSessionAffinityAttributesEl {
    #[doc= "Set the field `samesite`.\nConfigures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set [`secure=\"Never\"`](#secure). Available values: `Auto`, `Lax`, `None`, `Strict`."]
    pub fn set_samesite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.samesite = Some(v.into());
        self
    }

    #[doc= "Set the field `secure`.\nConfigures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`."]
    pub fn set_secure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secure = Some(v.into());
        self
    }

    #[doc= "Set the field `zero_downtime_failover`.\nConfigures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`."]
    pub fn set_zero_downtime_failover(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zero_downtime_failover = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerRulesElOverridesElSessionAffinityAttributesEl {
    type O = BlockAssignable<LoadBalancerRulesElOverridesElSessionAffinityAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesElSessionAffinityAttributesEl {}

impl BuildLoadBalancerRulesElOverridesElSessionAffinityAttributesEl {
    pub fn build(self) -> LoadBalancerRulesElOverridesElSessionAffinityAttributesEl {
        LoadBalancerRulesElOverridesElSessionAffinityAttributesEl {
            samesite: core::default::Default::default(),
            secure: core::default::Default::default(),
            zero_downtime_failover: core::default::Default::default(),
        }
    }
}

pub struct LoadBalancerRulesElOverridesElSessionAffinityAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRulesElOverridesElSessionAffinityAttributesElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRulesElOverridesElSessionAffinityAttributesElRef {
        LoadBalancerRulesElOverridesElSessionAffinityAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElSessionAffinityAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `samesite` after provisioning.\nConfigures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set [`secure=\"Never\"`](#secure). Available values: `Auto`, `Lax`, `None`, `Strict`."]
    pub fn samesite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.samesite", self.base))
    }

    #[doc= "Get a reference to the value of field `secure` after provisioning.\nConfigures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`."]
    pub fn secure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secure", self.base))
    }

    #[doc= "Get a reference to the value of field `zero_downtime_failover` after provisioning.\nConfigures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`."]
    pub fn zero_downtime_failover(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zero_downtime_failover", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoadBalancerRulesElOverridesElDynamic {
    adaptive_routing: Option<DynamicBlock<LoadBalancerRulesElOverridesElAdaptiveRoutingEl>>,
    country_pools: Option<DynamicBlock<LoadBalancerRulesElOverridesElCountryPoolsEl>>,
    location_strategy: Option<DynamicBlock<LoadBalancerRulesElOverridesElLocationStrategyEl>>,
    pop_pools: Option<DynamicBlock<LoadBalancerRulesElOverridesElPopPoolsEl>>,
    random_steering: Option<DynamicBlock<LoadBalancerRulesElOverridesElRandomSteeringEl>>,
    region_pools: Option<DynamicBlock<LoadBalancerRulesElOverridesElRegionPoolsEl>>,
    session_affinity_attributes: Option<DynamicBlock<LoadBalancerRulesElOverridesElSessionAffinityAttributesEl>>,
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_pools: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fallback_pool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_affinity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_affinity_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    steering_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    adaptive_routing: Option<Vec<LoadBalancerRulesElOverridesElAdaptiveRoutingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_pools: Option<Vec<LoadBalancerRulesElOverridesElCountryPoolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_strategy: Option<Vec<LoadBalancerRulesElOverridesElLocationStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pop_pools: Option<Vec<LoadBalancerRulesElOverridesElPopPoolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    random_steering: Option<Vec<LoadBalancerRulesElOverridesElRandomSteeringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_pools: Option<Vec<LoadBalancerRulesElOverridesElRegionPoolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_affinity_attributes: Option<Vec<LoadBalancerRulesElOverridesElSessionAffinityAttributesEl>>,
    dynamic: LoadBalancerRulesElOverridesElDynamic,
}

impl LoadBalancerRulesElOverridesEl {
    #[doc= "Set the field `default_pools`.\nA list of pool IDs ordered by their failover priority. Used whenever [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) are not defined."]
    pub fn set_default_pools(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.default_pools = Some(v.into());
        self
    }

    #[doc= "Set the field `fallback_pool`.\nThe pool ID to use when all other pools are detected as unhealthy."]
    pub fn set_fallback_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fallback_pool = Some(v.into());
        self
    }

    #[doc= "Set the field `session_affinity`.\nConfigure cookie attributes for session affinity cookie."]
    pub fn set_session_affinity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_affinity = Some(v.into());
        self
    }

    #[doc= "Set the field `session_affinity_ttl`.\nTime, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless [`session_affinity_ttl`](#session_affinity_ttl) is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`."]
    pub fn set_session_affinity_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_affinity_ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `steering_policy`.\nThe method the load balancer uses to determine the route to your origin. Value `off` uses [`default_pool_ids`](#default_pool_ids). Value `geo` uses [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools). For non-proxied requests, the [`country`](#country) for [`country_pools`](#country_pools) is determined by [`location_strategy`](#location_strategy). Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in [`default_pool_ids`](#default_pool_ids) (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by [`location_strategy`](#location_strategy) for non-proxied requests. Value `\"\"` maps to `geo` if you use [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `\"\"` Defaults to `\"\"`."]
    pub fn set_steering_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.steering_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `ttl`.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`."]
    pub fn set_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `adaptive_routing`.\n"]
    pub fn set_adaptive_routing(
        mut self,
        v: impl Into<BlockAssignable<LoadBalancerRulesElOverridesElAdaptiveRoutingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.adaptive_routing = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.adaptive_routing = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `country_pools`.\n"]
    pub fn set_country_pools(
        mut self,
        v: impl Into<BlockAssignable<LoadBalancerRulesElOverridesElCountryPoolsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.country_pools = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.country_pools = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `location_strategy`.\n"]
    pub fn set_location_strategy(
        mut self,
        v: impl Into<BlockAssignable<LoadBalancerRulesElOverridesElLocationStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.location_strategy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.location_strategy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `pop_pools`.\n"]
    pub fn set_pop_pools(mut self, v: impl Into<BlockAssignable<LoadBalancerRulesElOverridesElPopPoolsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pop_pools = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pop_pools = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `random_steering`.\n"]
    pub fn set_random_steering(
        mut self,
        v: impl Into<BlockAssignable<LoadBalancerRulesElOverridesElRandomSteeringEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.random_steering = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.random_steering = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `region_pools`.\n"]
    pub fn set_region_pools(
        mut self,
        v: impl Into<BlockAssignable<LoadBalancerRulesElOverridesElRegionPoolsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.region_pools = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.region_pools = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `session_affinity_attributes`.\n"]
    pub fn set_session_affinity_attributes(
        mut self,
        v: impl Into<BlockAssignable<LoadBalancerRulesElOverridesElSessionAffinityAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.session_affinity_attributes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.session_affinity_attributes = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for LoadBalancerRulesElOverridesEl {
    type O = BlockAssignable<LoadBalancerRulesElOverridesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesEl {}

impl BuildLoadBalancerRulesElOverridesEl {
    pub fn build(self) -> LoadBalancerRulesElOverridesEl {
        LoadBalancerRulesElOverridesEl {
            default_pools: core::default::Default::default(),
            fallback_pool: core::default::Default::default(),
            session_affinity: core::default::Default::default(),
            session_affinity_ttl: core::default::Default::default(),
            steering_policy: core::default::Default::default(),
            ttl: core::default::Default::default(),
            adaptive_routing: core::default::Default::default(),
            country_pools: core::default::Default::default(),
            location_strategy: core::default::Default::default(),
            pop_pools: core::default::Default::default(),
            random_steering: core::default::Default::default(),
            region_pools: core::default::Default::default(),
            session_affinity_attributes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LoadBalancerRulesElOverridesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRulesElOverridesElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRulesElOverridesElRef {
        LoadBalancerRulesElOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_pools` after provisioning.\nA list of pool IDs ordered by their failover priority. Used whenever [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) are not defined."]
    pub fn default_pools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.default_pools", self.base))
    }

    #[doc= "Get a reference to the value of field `fallback_pool` after provisioning.\nThe pool ID to use when all other pools are detected as unhealthy."]
    pub fn fallback_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fallback_pool", self.base))
    }

    #[doc= "Get a reference to the value of field `session_affinity` after provisioning.\nConfigure cookie attributes for session affinity cookie."]
    pub fn session_affinity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.base))
    }

    #[doc= "Get a reference to the value of field `session_affinity_ttl` after provisioning.\nTime, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless [`session_affinity_ttl`](#session_affinity_ttl) is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`."]
    pub fn session_affinity_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_affinity_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `steering_policy` after provisioning.\nThe method the load balancer uses to determine the route to your origin. Value `off` uses [`default_pool_ids`](#default_pool_ids). Value `geo` uses [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools). For non-proxied requests, the [`country`](#country) for [`country_pools`](#country_pools) is determined by [`location_strategy`](#location_strategy). Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in [`default_pool_ids`](#default_pool_ids) (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by [`location_strategy`](#location_strategy) for non-proxied requests. Value `\"\"` maps to `geo` if you use [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `\"\"` Defaults to `\"\"`."]
    pub fn steering_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.steering_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoadBalancerRulesElDynamic {
    fixed_response: Option<DynamicBlock<LoadBalancerRulesElFixedResponseEl>>,
    overrides: Option<DynamicBlock<LoadBalancerRulesElOverridesEl>>,
}

#[derive(Serialize)]
pub struct LoadBalancerRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminates: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_response: Option<Vec<LoadBalancerRulesElFixedResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<Vec<LoadBalancerRulesElOverridesEl>>,
    dynamic: LoadBalancerRulesElDynamic,
}

impl LoadBalancerRulesEl {
    #[doc= "Set the field `condition`.\nThe statement to evaluate to determine if this rule's effects should be applied. An empty condition is always true. See [load balancing rules](https://developers.cloudflare.com/load-balancing/understand-basics/load-balancing-rules)."]
    pub fn set_condition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\nA disabled rule will not be executed."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `priority`.\nPriority used when determining the order of rule execution. Lower values are executed first. If not provided, the list order will be used."]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }

    #[doc= "Set the field `terminates`.\nTerminates indicates that if this rule is true no further rules should be executed. Note: setting a [`fixed_response`](#fixed_response) forces this field to `true`."]
    pub fn set_terminates(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.terminates = Some(v.into());
        self
    }

    #[doc= "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(mut self, v: impl Into<BlockAssignable<LoadBalancerRulesElFixedResponseEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_response = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_response = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `overrides`.\n"]
    pub fn set_overrides(mut self, v: impl Into<BlockAssignable<LoadBalancerRulesElOverridesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.overrides = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.overrides = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for LoadBalancerRulesEl {
    type O = BlockAssignable<LoadBalancerRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesEl {
    #[doc= "Human readable name for this rule."]
    pub name: PrimField<String>,
}

impl BuildLoadBalancerRulesEl {
    pub fn build(self) -> LoadBalancerRulesEl {
        LoadBalancerRulesEl {
            condition: core::default::Default::default(),
            disabled: core::default::Default::default(),
            name: self.name,
            priority: core::default::Default::default(),
            terminates: core::default::Default::default(),
            fixed_response: core::default::Default::default(),
            overrides: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LoadBalancerRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerRulesElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerRulesElRef {
        LoadBalancerRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `condition` after provisioning.\nThe statement to evaluate to determine if this rule's effects should be applied. An empty condition is always true. See [load balancing rules](https://developers.cloudflare.com/load-balancing/understand-basics/load-balancing-rules)."]
    pub fn condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nA disabled rule will not be executed."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nHuman readable name for this rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nPriority used when determining the order of rule execution. Lower values are executed first. If not provided, the list order will be used."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc= "Get a reference to the value of field `terminates` after provisioning.\nTerminates indicates that if this rule is true no further rules should be executed. Note: setting a [`fixed_response`](#fixed_response) forces this field to `true`."]
    pub fn terminates(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.terminates", self.base))
    }

    #[doc= "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> ListRef<LoadBalancerRulesElFixedResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc= "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> ListRef<LoadBalancerRulesElOverridesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerSessionAffinityAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    drain_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    samesite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_downtime_failover: Option<PrimField<String>>,
}

impl LoadBalancerSessionAffinityAttributesEl {
    #[doc= "Set the field `drain_duration`.\nConfigures the drain duration in seconds. This field is only used when session affinity is enabled on the load balancer. Defaults to `0`."]
    pub fn set_drain_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.drain_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `samesite`.\nConfigures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set [`secure=\"Never\"`](#secure). Available values: `Auto`, `Lax`, `None`, `Strict`. Defaults to `Auto`."]
    pub fn set_samesite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.samesite = Some(v.into());
        self
    }

    #[doc= "Set the field `secure`.\nConfigures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`. Defaults to `Auto`."]
    pub fn set_secure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secure = Some(v.into());
        self
    }

    #[doc= "Set the field `zero_downtime_failover`.\nConfigures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`. Defaults to `none`."]
    pub fn set_zero_downtime_failover(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zero_downtime_failover = Some(v.into());
        self
    }
}

impl ToListMappable for LoadBalancerSessionAffinityAttributesEl {
    type O = BlockAssignable<LoadBalancerSessionAffinityAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerSessionAffinityAttributesEl {}

impl BuildLoadBalancerSessionAffinityAttributesEl {
    pub fn build(self) -> LoadBalancerSessionAffinityAttributesEl {
        LoadBalancerSessionAffinityAttributesEl {
            drain_duration: core::default::Default::default(),
            samesite: core::default::Default::default(),
            secure: core::default::Default::default(),
            zero_downtime_failover: core::default::Default::default(),
        }
    }
}

pub struct LoadBalancerSessionAffinityAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerSessionAffinityAttributesElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerSessionAffinityAttributesElRef {
        LoadBalancerSessionAffinityAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerSessionAffinityAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `drain_duration` after provisioning.\nConfigures the drain duration in seconds. This field is only used when session affinity is enabled on the load balancer. Defaults to `0`."]
    pub fn drain_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.drain_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `samesite` after provisioning.\nConfigures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set [`secure=\"Never\"`](#secure). Available values: `Auto`, `Lax`, `None`, `Strict`. Defaults to `Auto`."]
    pub fn samesite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.samesite", self.base))
    }

    #[doc= "Get a reference to the value of field `secure` after provisioning.\nConfigures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`. Defaults to `Auto`."]
    pub fn secure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secure", self.base))
    }

    #[doc= "Get a reference to the value of field `zero_downtime_failover` after provisioning.\nConfigures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`. Defaults to `none`."]
    pub fn zero_downtime_failover(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zero_downtime_failover", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoadBalancerDynamic {
    adaptive_routing: Option<DynamicBlock<LoadBalancerAdaptiveRoutingEl>>,
    country_pools: Option<DynamicBlock<LoadBalancerCountryPoolsEl>>,
    location_strategy: Option<DynamicBlock<LoadBalancerLocationStrategyEl>>,
    pop_pools: Option<DynamicBlock<LoadBalancerPopPoolsEl>>,
    random_steering: Option<DynamicBlock<LoadBalancerRandomSteeringEl>>,
    region_pools: Option<DynamicBlock<LoadBalancerRegionPoolsEl>>,
    rules: Option<DynamicBlock<LoadBalancerRulesEl>>,
    session_affinity_attributes: Option<DynamicBlock<LoadBalancerSessionAffinityAttributesEl>>,
}
