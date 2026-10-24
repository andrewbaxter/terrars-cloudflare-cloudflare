use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct LoadBalancerData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    default_pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    fallback_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxied: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_affinity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_affinity_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steering_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
    zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adaptive_routing: std::option::Option<std::vec::Vec<LoadBalancerAdaptiveRoutingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_pools: std::option::Option<std::vec::Vec<LoadBalancerCountryPoolsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_strategy: std::option::Option<std::vec::Vec<LoadBalancerLocationStrategyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pop_pools: std::option::Option<std::vec::Vec<LoadBalancerPopPoolsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    random_steering: std::option::Option<std::vec::Vec<LoadBalancerRandomSteeringEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_pools: std::option::Option<std::vec::Vec<LoadBalancerRegionPoolsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<LoadBalancerRulesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_affinity_attributes: std::option::Option<std::vec::Vec<LoadBalancerSessionAffinityAttributesEl>>,
    dynamic: LoadBalancerDynamic,
}

struct LoadBalancer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LoadBalancerData>,
}

#[derive(Clone)]
pub struct LoadBalancer(std::rc::Rc<LoadBalancer_>);

impl LoadBalancer {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `description`.\nFree text description."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nEnable or disable the load balancer. Defaults to `true`."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxied`.\nWhether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`."]
    pub fn set_proxied(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().proxied = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_affinity`.\nSpecifies the type of session affinity the load balancer should use unless specified as `none` or `\"\"` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `\"\"`, `none`, `cookie`, `ip_cookie`. Defaults to `none`."]
    pub fn set_session_affinity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().session_affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_affinity_ttl`.\nTime, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless [`session_affinity_ttl`](#session_affinity_ttl) is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`."]
    pub fn set_session_affinity_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().session_affinity_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `steering_policy`.\nThe method the load balancer uses to determine the route to your origin. Value `off` uses [`default_pool_ids`](#default_pool_ids). Value `geo` uses [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools). For non-proxied requests, the [`country`](#country) for [`country_pools`](#country_pools) is determined by [`location_strategy`](#location_strategy). Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in [`default_pool_ids`](#default_pool_ids) (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by [`location_strategy`](#location_strategy) for non-proxied requests. Value `\"\"` maps to `geo` if you use [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `\"\"` Defaults to `\"\"`."]
    pub fn set_steering_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().steering_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ttl`.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`."]
    pub fn set_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `adaptive_routing`.\n"]
    pub fn set_adaptive_routing(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerAdaptiveRoutingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().adaptive_routing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.adaptive_routing = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `country_pools`.\n"]
    pub fn set_country_pools(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerCountryPoolsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().country_pools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.country_pools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `location_strategy`.\n"]
    pub fn set_location_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerLocationStrategyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().location_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.location_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pop_pools`.\n"]
    pub fn set_pop_pools(self, v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerPopPoolsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pop_pools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pop_pools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `random_steering`.\n"]
    pub fn set_random_steering(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRandomSteeringEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().random_steering = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.random_steering = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `region_pools`.\n"]
    pub fn set_region_pools(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRegionPoolsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().region_pools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.region_pools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRulesEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `session_affinity_attributes`.\n"]
    pub fn set_session_affinity_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerSessionAffinityAttributesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().session_affinity_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.session_affinity_attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_pool_ids` after provisioning.\nA list of pool IDs ordered by their failover priority. Used whenever [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) are not defined."]
    pub fn default_pool_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_pool_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nFree text description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable or disable the load balancer. Defaults to `true`."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fallback_pool_id` after provisioning.\nThe pool ID to use when all other pools are detected as unhealthy."]
    pub fn fallback_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxied` after provisioning.\nWhether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`."]
    pub fn proxied(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity` after provisioning.\nSpecifies the type of session affinity the load balancer should use unless specified as `none` or `\"\"` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `\"\"`, `none`, `cookie`, `ip_cookie`. Defaults to `none`."]
    pub fn session_affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity_ttl` after provisioning.\nTime, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless [`session_affinity_ttl`](#session_affinity_ttl) is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`."]
    pub fn session_affinity_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity_ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `steering_policy` after provisioning.\nThe method the load balancer uses to determine the route to your origin. Value `off` uses [`default_pool_ids`](#default_pool_ids). Value `geo` uses [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools). For non-proxied requests, the [`country`](#country) for [`country_pools`](#country_pools) is determined by [`location_strategy`](#location_strategy). Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in [`default_pool_ids`](#default_pool_ids) (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by [`location_strategy`](#location_strategy) for non-proxied requests. Value `\"\"` maps to `geo` if you use [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `\"\"` Defaults to `\"\"`."]
    pub fn steering_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.steering_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<LoadBalancerRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl terrars::Referable for LoadBalancer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LoadBalancer { }

impl terrars::ToListMappable for LoadBalancer {
    type O = terrars::ListRef<LoadBalancerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LoadBalancer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_load_balancer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoadBalancer {
    pub tf_id: std::string::String,
    #[doc =
        "A list of pool IDs ordered by their failover priority. Used whenever [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) are not defined."]
    pub default_pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The pool ID to use when all other pools are detected as unhealthy."]
    pub fallback_pool_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildLoadBalancer {
    pub fn build(self, stack: &mut terrars::Stack) -> LoadBalancer {
        let out = LoadBalancer(std::rc::Rc::new(LoadBalancer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LoadBalancerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoadBalancerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoadBalancerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_pool_ids` after provisioning.\nA list of pool IDs ordered by their failover priority. Used whenever [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) are not defined."]
    pub fn default_pool_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_pool_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nFree text description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable or disable the load balancer. Defaults to `true`."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fallback_pool_id` after provisioning.\nThe pool ID to use when all other pools are detected as unhealthy."]
    pub fn fallback_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxied` after provisioning.\nWhether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`."]
    pub fn proxied(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxied", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity` after provisioning.\nSpecifies the type of session affinity the load balancer should use unless specified as `none` or `\"\"` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `\"\"`, `none`, `cookie`, `ip_cookie`. Defaults to `none`."]
    pub fn session_affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity_ttl` after provisioning.\nTime, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless [`session_affinity_ttl`](#session_affinity_ttl) is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`."]
    pub fn session_affinity_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity_ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `steering_policy` after provisioning.\nThe method the load balancer uses to determine the route to your origin. Value `off` uses [`default_pool_ids`](#default_pool_ids). Value `geo` uses [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools). For non-proxied requests, the [`country`](#country) for [`country_pools`](#country_pools) is determined by [`location_strategy`](#location_strategy). Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in [`default_pool_ids`](#default_pool_ids) (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by [`location_strategy`](#location_strategy) for non-proxied requests. Value `\"\"` maps to `geo` if you use [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `\"\"` Defaults to `\"\"`."]
    pub fn steering_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.steering_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<LoadBalancerRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerAdaptiveRoutingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_across_pools: std::option::Option<terrars::PrimField<bool>>,
}

impl LoadBalancerAdaptiveRoutingEl {
    #[doc =
        "Set the field `failover_across_pools`.\nExtends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool. Defaults to `false`."]
    pub fn set_failover_across_pools(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.failover_across_pools = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerAdaptiveRoutingEl {
    type O = terrars::BlockAssignable<LoadBalancerAdaptiveRoutingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerAdaptiveRoutingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerAdaptiveRoutingElRef {
        LoadBalancerAdaptiveRoutingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerAdaptiveRoutingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failover_across_pools` after provisioning.\nExtends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool. Defaults to `false`."]
    pub fn failover_across_pools(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_across_pools", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerCountryPoolsEl {
    country: terrars::PrimField<std::string::String>,
    pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl LoadBalancerCountryPoolsEl { }

impl terrars::ToListMappable for LoadBalancerCountryPoolsEl {
    type O = terrars::BlockAssignable<LoadBalancerCountryPoolsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerCountryPoolsEl {
    #[doc =
        "A country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country."]
    pub country: terrars::PrimField<std::string::String>,
    #[doc = "A list of pool IDs in failover priority to use in the given country."]
    pub pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerCountryPoolsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerCountryPoolsElRef {
        LoadBalancerCountryPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerCountryPoolsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `country` after provisioning.\nA country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use in the given country."]
    pub fn pool_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerLocationStrategyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefer_ecs: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LoadBalancerLocationStrategyEl {
    #[doc =
        "Set the field `mode`.\nDetermines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`. Defaults to `pop`."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefer_ecs`.\nWhether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when [`steering_policy=\"proximity\"`](#steering_policy), and `geo` will prefer ECS only when [`steering_policy=\"geo\"`](#steering_policy). Available values: `always`, `never`, `proximity`, `geo`. Defaults to `proximity`."]
    pub fn set_prefer_ecs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefer_ecs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerLocationStrategyEl {
    type O = terrars::BlockAssignable<LoadBalancerLocationStrategyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerLocationStrategyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerLocationStrategyElRef {
        LoadBalancerLocationStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerLocationStrategyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nDetermines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`. Defaults to `pop`."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefer_ecs` after provisioning.\nWhether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when [`steering_policy=\"proximity\"`](#steering_policy), and `geo` will prefer ECS only when [`steering_policy=\"geo\"`](#steering_policy). Available values: `always`, `never`, `proximity`, `geo`. Defaults to `proximity`."]
    pub fn prefer_ecs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefer_ecs", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPopPoolsEl {
    pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    pop: terrars::PrimField<std::string::String>,
}

impl LoadBalancerPopPoolsEl { }

impl terrars::ToListMappable for LoadBalancerPopPoolsEl {
    type O = terrars::BlockAssignable<LoadBalancerPopPoolsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPopPoolsEl {
    #[doc = "A list of pool IDs in failover priority to use for traffic reaching the given PoP."]
    pub pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "A 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP."]
    pub pop: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerPopPoolsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerPopPoolsElRef {
        LoadBalancerPopPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPopPoolsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use for traffic reaching the given PoP."]
    pub fn pool_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pop` after provisioning.\nA 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP."]
    pub fn pop(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pop", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRandomSteeringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_weights: std::option::Option<terrars::RecField<terrars::PrimField<f64>>>,
}

impl LoadBalancerRandomSteeringEl {
    #[doc =
        "Set the field `default_weight`.\nThe default weight for pools in the load balancer that are not specified in the [`pool_weights`](#pool_weights) map."]
    pub fn set_default_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_weight = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pool_weights`.\nA mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer."]
    pub fn set_pool_weights(mut self, v: impl std::convert::Into<terrars::RecField<terrars::PrimField<f64>>>) -> Self {
        self.pool_weights = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerRandomSteeringEl {
    type O = terrars::BlockAssignable<LoadBalancerRandomSteeringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRandomSteeringElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerRandomSteeringElRef {
        LoadBalancerRandomSteeringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRandomSteeringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_weight` after provisioning.\nThe default weight for pools in the load balancer that are not specified in the [`pool_weights`](#pool_weights) map."]
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
pub struct LoadBalancerRegionPoolsEl {
    pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
}

impl LoadBalancerRegionPoolsEl { }

impl terrars::ToListMappable for LoadBalancerRegionPoolsEl {
    type O = terrars::BlockAssignable<LoadBalancerRegionPoolsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRegionPoolsEl {
    #[doc = "A list of pool IDs in failover priority to use in the given region."]
    pub pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "A region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region."]
    pub region: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRegionPoolsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerRegionPoolsElRef {
        LoadBalancerRegionPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRegionPoolsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use in the given region."]
    pub fn pool_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElFixedResponseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
}

impl LoadBalancerRulesElFixedResponseEl {
    #[doc = "Set the field `content_type`.\nThe value of the HTTP context-type header for this fixed response."]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe value of the HTTP location header for this fixed response."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_body`.\nThe text used as the html body for this fixed response."]
    pub fn set_message_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_body = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\nThe HTTP status code used for this fixed response."]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerRulesElFixedResponseEl {
    type O = terrars::BlockAssignable<LoadBalancerRulesElFixedResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRulesElFixedResponseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerRulesElFixedResponseElRef {
        LoadBalancerRulesElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElFixedResponseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nThe value of the HTTP context-type header for this fixed response."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe value of the HTTP location header for this fixed response."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `message_body` after provisioning.\nThe text used as the html body for this fixed response."]
    pub fn message_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status_code` after provisioning.\nThe HTTP status code used for this fixed response."]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElAdaptiveRoutingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_across_pools: std::option::Option<terrars::PrimField<bool>>,
}

impl LoadBalancerRulesElOverridesElAdaptiveRoutingEl {
    #[doc =
        "Set the field `failover_across_pools`.\nExtends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool."]
    pub fn set_failover_across_pools(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.failover_across_pools = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerRulesElOverridesElAdaptiveRoutingEl {
    type O = terrars::BlockAssignable<LoadBalancerRulesElOverridesElAdaptiveRoutingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRulesElOverridesElAdaptiveRoutingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LoadBalancerRulesElOverridesElAdaptiveRoutingElRef {
        LoadBalancerRulesElOverridesElAdaptiveRoutingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElAdaptiveRoutingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failover_across_pools` after provisioning.\nExtends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool."]
    pub fn failover_across_pools(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_across_pools", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElCountryPoolsEl {
    country: terrars::PrimField<std::string::String>,
    pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl LoadBalancerRulesElOverridesElCountryPoolsEl { }

impl terrars::ToListMappable for LoadBalancerRulesElOverridesElCountryPoolsEl {
    type O = terrars::BlockAssignable<LoadBalancerRulesElOverridesElCountryPoolsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesElCountryPoolsEl {
    #[doc =
        "A country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country."]
    pub country: terrars::PrimField<std::string::String>,
    #[doc = "A list of pool IDs in failover priority to use in the given country."]
    pub pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRulesElOverridesElCountryPoolsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LoadBalancerRulesElOverridesElCountryPoolsElRef {
        LoadBalancerRulesElOverridesElCountryPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElCountryPoolsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `country` after provisioning.\nA country code which can be determined with the Load Balancing Regions API described [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/). Multiple entries should not be specified with the same country."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use in the given country."]
    pub fn pool_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElLocationStrategyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefer_ecs: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LoadBalancerRulesElOverridesElLocationStrategyEl {
    #[doc =
        "Set the field `mode`.\nDetermines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefer_ecs`.\nWhether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when [`steering_policy=\"proximity\"`](#steering_policy), and `geo` will prefer ECS only when [`steering_policy=\"geo\"`](#steering_policy). Available values: `always`, `never`, `proximity`, `geo`."]
    pub fn set_prefer_ecs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefer_ecs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerRulesElOverridesElLocationStrategyEl {
    type O = terrars::BlockAssignable<LoadBalancerRulesElOverridesElLocationStrategyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRulesElOverridesElLocationStrategyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LoadBalancerRulesElOverridesElLocationStrategyElRef {
        LoadBalancerRulesElOverridesElLocationStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElLocationStrategyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nDetermines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefer_ecs` after provisioning.\nWhether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when [`steering_policy=\"proximity\"`](#steering_policy), and `geo` will prefer ECS only when [`steering_policy=\"geo\"`](#steering_policy). Available values: `always`, `never`, `proximity`, `geo`."]
    pub fn prefer_ecs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefer_ecs", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElPopPoolsEl {
    pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    pop: terrars::PrimField<std::string::String>,
}

impl LoadBalancerRulesElOverridesElPopPoolsEl { }

impl terrars::ToListMappable for LoadBalancerRulesElOverridesElPopPoolsEl {
    type O = terrars::BlockAssignable<LoadBalancerRulesElOverridesElPopPoolsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesElPopPoolsEl {
    #[doc = "A list of pool IDs in failover priority to use for traffic reaching the given PoP."]
    pub pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "A 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP."]
    pub pop: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRulesElOverridesElPopPoolsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerRulesElOverridesElPopPoolsElRef {
        LoadBalancerRulesElOverridesElPopPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElPopPoolsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use for traffic reaching the given PoP."]
    pub fn pool_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pop` after provisioning.\nA 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP."]
    pub fn pop(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pop", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElRandomSteeringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_weights: std::option::Option<terrars::RecField<terrars::PrimField<f64>>>,
}

impl LoadBalancerRulesElOverridesElRandomSteeringEl {
    #[doc =
        "Set the field `default_weight`.\nThe default weight for pools in the load balancer that are not specified in the [`pool_weights`](#pool_weights) map."]
    pub fn set_default_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_weight = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pool_weights`.\nA mapping of pool IDs to custom weights. The weight is relative to other pools in the load balancer."]
    pub fn set_pool_weights(mut self, v: impl std::convert::Into<terrars::RecField<terrars::PrimField<f64>>>) -> Self {
        self.pool_weights = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerRulesElOverridesElRandomSteeringEl {
    type O = terrars::BlockAssignable<LoadBalancerRulesElOverridesElRandomSteeringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRulesElOverridesElRandomSteeringElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LoadBalancerRulesElOverridesElRandomSteeringElRef {
        LoadBalancerRulesElOverridesElRandomSteeringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElRandomSteeringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_weight` after provisioning.\nThe default weight for pools in the load balancer that are not specified in the [`pool_weights`](#pool_weights) map."]
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
pub struct LoadBalancerRulesElOverridesElRegionPoolsEl {
    pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
}

impl LoadBalancerRulesElOverridesElRegionPoolsEl { }

impl terrars::ToListMappable for LoadBalancerRulesElOverridesElRegionPoolsEl {
    type O = terrars::BlockAssignable<LoadBalancerRulesElOverridesElRegionPoolsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesElOverridesElRegionPoolsEl {
    #[doc = "A list of pool IDs in failover priority to use in the given region."]
    pub pool_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "A region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region."]
    pub region: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRulesElOverridesElRegionPoolsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LoadBalancerRulesElOverridesElRegionPoolsElRef {
        LoadBalancerRulesElOverridesElRegionPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElRegionPoolsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pool_ids` after provisioning.\nA list of pool IDs in failover priority to use in the given region."]
    pub fn pool_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pool_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesElSessionAffinityAttributesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    samesite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zero_downtime_failover: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LoadBalancerRulesElOverridesElSessionAffinityAttributesEl {
    #[doc =
        "Set the field `samesite`.\nConfigures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set [`secure=\"Never\"`](#secure). Available values: `Auto`, `Lax`, `None`, `Strict`."]
    pub fn set_samesite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.samesite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secure`.\nConfigures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`."]
    pub fn set_secure(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zero_downtime_failover`.\nConfigures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`."]
    pub fn set_zero_downtime_failover(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.zero_downtime_failover = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerRulesElOverridesElSessionAffinityAttributesEl {
    type O = terrars::BlockAssignable<LoadBalancerRulesElOverridesElSessionAffinityAttributesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRulesElOverridesElSessionAffinityAttributesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LoadBalancerRulesElOverridesElSessionAffinityAttributesElRef {
        LoadBalancerRulesElOverridesElSessionAffinityAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElSessionAffinityAttributesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `samesite` after provisioning.\nConfigures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set [`secure=\"Never\"`](#secure). Available values: `Auto`, `Lax`, `None`, `Strict`."]
    pub fn samesite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.samesite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secure` after provisioning.\nConfigures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`."]
    pub fn secure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zero_downtime_failover` after provisioning.\nConfigures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`."]
    pub fn zero_downtime_failover(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zero_downtime_failover", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoadBalancerRulesElOverridesElDynamic {
    adaptive_routing: std::option::Option<terrars::DynamicBlock<LoadBalancerRulesElOverridesElAdaptiveRoutingEl>>,
    country_pools: std::option::Option<terrars::DynamicBlock<LoadBalancerRulesElOverridesElCountryPoolsEl>>,
    location_strategy: std::option::Option<terrars::DynamicBlock<LoadBalancerRulesElOverridesElLocationStrategyEl>>,
    pop_pools: std::option::Option<terrars::DynamicBlock<LoadBalancerRulesElOverridesElPopPoolsEl>>,
    random_steering: std::option::Option<terrars::DynamicBlock<LoadBalancerRulesElOverridesElRandomSteeringEl>>,
    region_pools: std::option::Option<terrars::DynamicBlock<LoadBalancerRulesElOverridesElRegionPoolsEl>>,
    session_affinity_attributes: std
    ::option
    ::Option<terrars::DynamicBlock<LoadBalancerRulesElOverridesElSessionAffinityAttributesEl>>,
}

#[derive(Serialize)]
pub struct LoadBalancerRulesElOverridesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_pools: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_affinity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_affinity_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steering_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adaptive_routing: std::option::Option<std::vec::Vec<LoadBalancerRulesElOverridesElAdaptiveRoutingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_pools: std::option::Option<std::vec::Vec<LoadBalancerRulesElOverridesElCountryPoolsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_strategy: std::option::Option<std::vec::Vec<LoadBalancerRulesElOverridesElLocationStrategyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pop_pools: std::option::Option<std::vec::Vec<LoadBalancerRulesElOverridesElPopPoolsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    random_steering: std::option::Option<std::vec::Vec<LoadBalancerRulesElOverridesElRandomSteeringEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_pools: std::option::Option<std::vec::Vec<LoadBalancerRulesElOverridesElRegionPoolsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_affinity_attributes: std
    ::option
    ::Option<std::vec::Vec<LoadBalancerRulesElOverridesElSessionAffinityAttributesEl>>,
    dynamic: LoadBalancerRulesElOverridesElDynamic,
}

impl LoadBalancerRulesElOverridesEl {
    #[doc =
        "Set the field `default_pools`.\nA list of pool IDs ordered by their failover priority. Used whenever [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) are not defined."]
    pub fn set_default_pools(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.default_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fallback_pool`.\nThe pool ID to use when all other pools are detected as unhealthy."]
    pub fn set_fallback_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fallback_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_affinity`.\nConfigure cookie attributes for session affinity cookie."]
    pub fn set_session_affinity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.session_affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_affinity_ttl`.\nTime, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless [`session_affinity_ttl`](#session_affinity_ttl) is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`."]
    pub fn set_session_affinity_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.session_affinity_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `steering_policy`.\nThe method the load balancer uses to determine the route to your origin. Value `off` uses [`default_pool_ids`](#default_pool_ids). Value `geo` uses [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools). For non-proxied requests, the [`country`](#country) for [`country_pools`](#country_pools) is determined by [`location_strategy`](#location_strategy). Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in [`default_pool_ids`](#default_pool_ids) (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by [`location_strategy`](#location_strategy) for non-proxied requests. Value `\"\"` maps to `geo` if you use [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `\"\"` Defaults to `\"\"`."]
    pub fn set_steering_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.steering_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ttl`.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`."]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `adaptive_routing`.\n"]
    pub fn set_adaptive_routing(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRulesElOverridesElAdaptiveRoutingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.adaptive_routing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.adaptive_routing = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `country_pools`.\n"]
    pub fn set_country_pools(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRulesElOverridesElCountryPoolsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.country_pools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.country_pools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `location_strategy`.\n"]
    pub fn set_location_strategy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRulesElOverridesElLocationStrategyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.location_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.location_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pop_pools`.\n"]
    pub fn set_pop_pools(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRulesElOverridesElPopPoolsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pop_pools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pop_pools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `random_steering`.\n"]
    pub fn set_random_steering(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRulesElOverridesElRandomSteeringEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.random_steering = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.random_steering = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `region_pools`.\n"]
    pub fn set_region_pools(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRulesElOverridesElRegionPoolsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.region_pools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.region_pools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `session_affinity_attributes`.\n"]
    pub fn set_session_affinity_attributes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LoadBalancerRulesElOverridesElSessionAffinityAttributesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.session_affinity_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.session_affinity_attributes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LoadBalancerRulesElOverridesEl {
    type O = terrars::BlockAssignable<LoadBalancerRulesElOverridesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LoadBalancerRulesElOverridesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRulesElOverridesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerRulesElOverridesElRef {
        LoadBalancerRulesElOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElOverridesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_pools` after provisioning.\nA list of pool IDs ordered by their failover priority. Used whenever [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) are not defined."]
    pub fn default_pools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_pools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fallback_pool` after provisioning.\nThe pool ID to use when all other pools are detected as unhealthy."]
    pub fn fallback_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity` after provisioning.\nConfigure cookie attributes for session affinity cookie."]
    pub fn session_affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_affinity_ttl` after provisioning.\nTime, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless [`session_affinity_ttl`](#session_affinity_ttl) is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`."]
    pub fn session_affinity_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `steering_policy` after provisioning.\nThe method the load balancer uses to determine the route to your origin. Value `off` uses [`default_pool_ids`](#default_pool_ids). Value `geo` uses [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools). For non-proxied requests, the [`country`](#country) for [`country_pools`](#country_pools) is determined by [`location_strategy`](#location_strategy). Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in [`default_pool_ids`](#default_pool_ids) (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by [`location_strategy`](#location_strategy) for non-proxied requests. Value `\"\"` maps to `geo` if you use [`pop_pools`](#pop_pools)/[`country_pools`](#country_pools)/[`region_pools`](#region_pools) otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `\"\"` Defaults to `\"\"`."]
    pub fn steering_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.steering_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nTime to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoadBalancerRulesElDynamic {
    fixed_response: std::option::Option<terrars::DynamicBlock<LoadBalancerRulesElFixedResponseEl>>,
    overrides: std::option::Option<terrars::DynamicBlock<LoadBalancerRulesElOverridesEl>>,
}

#[derive(Serialize)]
pub struct LoadBalancerRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminates: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<std::vec::Vec<LoadBalancerRulesElFixedResponseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overrides: std::option::Option<std::vec::Vec<LoadBalancerRulesElOverridesEl>>,
    dynamic: LoadBalancerRulesElDynamic,
}

impl LoadBalancerRulesEl {
    #[doc =
        "Set the field `condition`.\nThe statement to evaluate to determine if this rule's effects should be applied. An empty condition is always true. See [load balancing rules](https://developers.cloudflare.com/load-balancing/understand-basics/load-balancing-rules)."]
    pub fn set_condition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\nA disabled rule will not be executed."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `priority`.\nPriority used when determining the order of rule execution. Lower values are executed first. If not provided, the list order will be used."]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `terminates`.\nTerminates indicates that if this rule is true no further rules should be executed. Note: setting a [`fixed_response`](#fixed_response) forces this field to `true`."]
    pub fn set_terminates(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.terminates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRulesElFixedResponseEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fixed_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoadBalancerRulesElOverridesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LoadBalancerRulesEl {
    type O = terrars::BlockAssignable<LoadBalancerRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerRulesEl {
    #[doc = "Human readable name for this rule."]
    pub name: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LoadBalancerRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerRulesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerRulesElRef {
        LoadBalancerRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nThe statement to evaluate to determine if this rule's effects should be applied. An empty condition is always true. See [load balancing rules](https://developers.cloudflare.com/load-balancing/understand-basics/load-balancing-rules)."]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nA disabled rule will not be executed."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nHuman readable name for this rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nPriority used when determining the order of rule execution. Lower values are executed first. If not provided, the list order will be used."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `terminates` after provisioning.\nTerminates indicates that if this rule is true no further rules should be executed. Note: setting a [`fixed_response`](#fixed_response) forces this field to `true`."]
    pub fn terminates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminates", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<LoadBalancerRulesElFixedResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> terrars::ListRef<LoadBalancerRulesElOverridesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerSessionAffinityAttributesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drain_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    samesite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zero_downtime_failover: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LoadBalancerSessionAffinityAttributesEl {
    #[doc =
        "Set the field `drain_duration`.\nConfigures the drain duration in seconds. This field is only used when session affinity is enabled on the load balancer. Defaults to `0`."]
    pub fn set_drain_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.drain_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `samesite`.\nConfigures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set [`secure=\"Never\"`](#secure). Available values: `Auto`, `Lax`, `None`, `Strict`. Defaults to `Auto`."]
    pub fn set_samesite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.samesite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secure`.\nConfigures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`. Defaults to `Auto`."]
    pub fn set_secure(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zero_downtime_failover`.\nConfigures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`. Defaults to `none`."]
    pub fn set_zero_downtime_failover(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.zero_downtime_failover = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerSessionAffinityAttributesEl {
    type O = terrars::BlockAssignable<LoadBalancerSessionAffinityAttributesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerSessionAffinityAttributesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerSessionAffinityAttributesElRef {
        LoadBalancerSessionAffinityAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerSessionAffinityAttributesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `drain_duration` after provisioning.\nConfigures the drain duration in seconds. This field is only used when session affinity is enabled on the load balancer. Defaults to `0`."]
    pub fn drain_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drain_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `samesite` after provisioning.\nConfigures the SameSite attribute on session affinity cookie. Value `Auto` will be translated to `Lax` or `None` depending if Always Use HTTPS is enabled. Note: when using value `None`, then you can not set [`secure=\"Never\"`](#secure). Available values: `Auto`, `Lax`, `None`, `Strict`. Defaults to `Auto`."]
    pub fn samesite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.samesite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secure` after provisioning.\nConfigures the Secure attribute on session affinity cookie. Value `Always` indicates the Secure attribute will be set in the Set-Cookie header, `Never` indicates the Secure attribute will not be set, and `Auto` will set the Secure attribute depending if Always Use HTTPS is enabled. Available values: `Auto`, `Always`, `Never`. Defaults to `Auto`."]
    pub fn secure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zero_downtime_failover` after provisioning.\nConfigures the zero-downtime failover between origins within a pool when session affinity is enabled. Value `none` means no failover takes place for sessions pinned to the origin. Value `temporary` means traffic will be sent to another other healthy origin until the originally pinned origin is available; note that this can potentially result in heavy origin flapping. Value `sticky` means the session affinity cookie is updated and subsequent requests are sent to the new origin. This feature is currently incompatible with Argo, Tiered Cache, and Bandwidth Alliance. Available values: `none`, `temporary`, `sticky`. Defaults to `none`."]
    pub fn zero_downtime_failover(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zero_downtime_failover", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoadBalancerDynamic {
    adaptive_routing: std::option::Option<terrars::DynamicBlock<LoadBalancerAdaptiveRoutingEl>>,
    country_pools: std::option::Option<terrars::DynamicBlock<LoadBalancerCountryPoolsEl>>,
    location_strategy: std::option::Option<terrars::DynamicBlock<LoadBalancerLocationStrategyEl>>,
    pop_pools: std::option::Option<terrars::DynamicBlock<LoadBalancerPopPoolsEl>>,
    random_steering: std::option::Option<terrars::DynamicBlock<LoadBalancerRandomSteeringEl>>,
    region_pools: std::option::Option<terrars::DynamicBlock<LoadBalancerRegionPoolsEl>>,
    rules: std::option::Option<terrars::DynamicBlock<LoadBalancerRulesEl>>,
    session_affinity_attributes: std
    ::option
    ::Option<terrars::DynamicBlock<LoadBalancerSessionAffinityAttributesEl>>,
}
