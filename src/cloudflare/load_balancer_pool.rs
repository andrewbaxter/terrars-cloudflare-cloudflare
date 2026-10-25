#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct LoadBalancerPoolData_ {
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
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_sources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latitude: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_shedding: std::option::Option<LoadBalancerPool_LoadShedding>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    longitude: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_origins: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitor_group: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_filter: std::option::Option<LoadBalancerPool_NotificationFilter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_steering: std::option::Option<LoadBalancerPool_OriginSteering>,
    origins: std::vec::Vec<LoadBalancerPool_Origins>,
}

struct LoadBalancerPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LoadBalancerPoolData_>,
}

#[derive(Clone)]
pub struct LoadBalancerPool(std::rc::Rc<LoadBalancerPool_>);

impl LoadBalancerPool {
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

    #[doc =
        "Set the field `check_regions`.\nA list of regions from which to run health checks. Null means every Cloudflare data center."]
    pub fn set_check_regions(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().check_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA human-readable description of the pool."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enabled`.\nWhether to enable (the default) or disable this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any)."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `health_sources`.\nA list of health sources, ordered from highest to lowest priority, used to evaluate individual origin health and overall pool health. The load balancer uses the first source that has data and falls back to the next. Currently accepted values are null or the exact array [\"regional\", \"global\"]; any other combination is rejected. Null (the default) behaves like [\"local\", \"global\"]. [\"regional\", \"global\"] makes each region steer on its own health, falling back to the global decision when a region has no fresh data. Setting regional requires at least one region in check_regions."]
    pub fn set_health_sources(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().health_sources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `latitude`.\nThe latitude of the data center containing the origins used in this pool in decimal degrees. If this is set, longitude must also be set."]
    pub fn set_latitude(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().latitude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_shedding`.\nConfigures load shedding policies and percentages for the pool."]
    pub fn set_load_shedding(self, v: impl std::convert::Into<LoadBalancerPool_LoadShedding>) -> Self {
        self.0.data.borrow_mut().load_shedding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `longitude`.\nThe longitude of the data center containing the origins used in this pool in decimal degrees. If this is set, latitude must also be set."]
    pub fn set_longitude(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().longitude = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minimum_origins`.\nThe minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and will failover to the next available pool."]
    pub fn set_minimum_origins(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().minimum_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `monitor`.\nThe ID of the Monitor to use for checking the health of origins within this pool."]
    pub fn set_monitor(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().monitor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `monitor_group`.\nThe ID of the Monitor Group to use for checking the health of origins within this pool."]
    pub fn set_monitor_group(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().monitor_group = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notification_email`.\nThis field is now deprecated. It has been moved to Cloudflare's Centralized Notification service https://developers.cloudflare.com/fundamentals/notifications/. The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list."]
    pub fn set_notification_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().notification_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notification_filter`.\nFilter pool and origin health notifications by resource type or health status. Use null to reset."]
    pub fn set_notification_filter(self, v: impl std::convert::Into<LoadBalancerPool_NotificationFilter>) -> Self {
        self.0.data.borrow_mut().notification_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `origin_steering`.\nConfigures origin steering for the pool. Controls how origins are selected for new sessions and traffic without session affinity."]
    pub fn set_origin_steering(self, v: impl std::convert::Into<LoadBalancerPool_OriginSteering>) -> Self {
        self.0.data.borrow_mut().origin_steering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `check_regions` after provisioning.\nA list of regions from which to run health checks. Null means every Cloudflare data center."]
    pub fn check_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the pool."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled_at` after provisioning.\nThis field shows up only if the pool is disabled. This field is set with the time the pool was disabled at."]
    pub fn disabled_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable (the default) or disable this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any)."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_sources` after provisioning.\nA list of health sources, ordered from highest to lowest priority, used to evaluate individual origin health and overall pool health. The load balancer uses the first source that has data and falls back to the next. Currently accepted values are null or the exact array [\"regional\", \"global\"]; any other combination is rejected. Null (the default) behaves like [\"local\", \"global\"]. [\"regional\", \"global\"] makes each region steer on its own health, falling back to the global decision when a region has no fresh data. Setting regional requires at least one region in check_regions."]
    pub fn health_sources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latitude` after provisioning.\nThe latitude of the data center containing the origins used in this pool in decimal degrees. If this is set, longitude must also be set."]
    pub fn latitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `load_shedding` after provisioning.\nConfigures load shedding policies and percentages for the pool."]
    pub fn load_shedding(&self) -> LoadBalancerPool_LoadSheddingRef {
        LoadBalancerPool_LoadSheddingRef::new(self.shared().clone(), format!("{}.load_shedding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `longitude` after provisioning.\nThe longitude of the data center containing the origins used in this pool in decimal degrees. If this is set, latitude must also be set."]
    pub fn longitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `minimum_origins` after provisioning.\nThe minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and will failover to the next available pool."]
    pub fn minimum_origins(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_origins", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitor` after provisioning.\nThe ID of the Monitor to use for checking the health of origins within this pool."]
    pub fn monitor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitor_group` after provisioning.\nThe ID of the Monitor Group to use for checking the health of origins within this pool."]
    pub fn monitor_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA short name (tag) for the pool. Only alphanumeric characters, hyphens, and underscores are allowed."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nList of networks where Load Balancer or Pool is enabled."]
    pub fn networks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_email` after provisioning.\nThis field is now deprecated. It has been moved to Cloudflare's Centralized Notification service https://developers.cloudflare.com/fundamentals/notifications/. The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list."]
    pub fn notification_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_filter` after provisioning.\nFilter pool and origin health notifications by resource type or health status. Use null to reset."]
    pub fn notification_filter(&self) -> LoadBalancerPool_NotificationFilterRef {
        LoadBalancerPool_NotificationFilterRef::new(
            self.shared().clone(),
            format!("{}.notification_filter", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `origin_steering` after provisioning.\nConfigures origin steering for the pool. Controls how origins are selected for new sessions and traffic without session affinity."]
    pub fn origin_steering(&self) -> LoadBalancerPool_OriginSteeringRef {
        LoadBalancerPool_OriginSteeringRef::new(
            self.shared().clone(),
            format!("{}.origin_steering", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `origins` after provisioning.\nThe list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy."]
    pub fn origins(&self) -> terrars::ListRef<LoadBalancerPool_OriginsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origins", self.extract_ref()))
    }
}

impl terrars::Referable for LoadBalancerPool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LoadBalancerPool { }

impl terrars::ToListMappable for LoadBalancerPool {
    type O = terrars::ListRef<LoadBalancerPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LoadBalancerPool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_load_balancer_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoadBalancerPool {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "A short name (tag) for the pool. Only alphanumeric characters, hyphens, and underscores are allowed."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy."]
    pub origins: std::vec::Vec<LoadBalancerPool_Origins>,
}

impl BuildLoadBalancerPool {
    pub fn build(self, stack: &mut terrars::Stack) -> LoadBalancerPool {
        let out = LoadBalancerPool(std::rc::Rc::new(LoadBalancerPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LoadBalancerPoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                check_regions: core::default::Default::default(),
                description: core::default::Default::default(),
                enabled: core::default::Default::default(),
                health_sources: core::default::Default::default(),
                latitude: core::default::Default::default(),
                load_shedding: core::default::Default::default(),
                longitude: core::default::Default::default(),
                minimum_origins: core::default::Default::default(),
                monitor: core::default::Default::default(),
                monitor_group: core::default::Default::default(),
                name: self.name,
                notification_email: core::default::Default::default(),
                notification_filter: core::default::Default::default(),
                origin_steering: core::default::Default::default(),
                origins: self.origins,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoadBalancerPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoadBalancerPoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `check_regions` after provisioning.\nA list of regions from which to run health checks. Null means every Cloudflare data center."]
    pub fn check_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the pool."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled_at` after provisioning.\nThis field shows up only if the pool is disabled. This field is set with the time the pool was disabled at."]
    pub fn disabled_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable (the default) or disable this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any)."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_sources` after provisioning.\nA list of health sources, ordered from highest to lowest priority, used to evaluate individual origin health and overall pool health. The load balancer uses the first source that has data and falls back to the next. Currently accepted values are null or the exact array [\"regional\", \"global\"]; any other combination is rejected. Null (the default) behaves like [\"local\", \"global\"]. [\"regional\", \"global\"] makes each region steer on its own health, falling back to the global decision when a region has no fresh data. Setting regional requires at least one region in check_regions."]
    pub fn health_sources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latitude` after provisioning.\nThe latitude of the data center containing the origins used in this pool in decimal degrees. If this is set, longitude must also be set."]
    pub fn latitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `load_shedding` after provisioning.\nConfigures load shedding policies and percentages for the pool."]
    pub fn load_shedding(&self) -> LoadBalancerPool_LoadSheddingRef {
        LoadBalancerPool_LoadSheddingRef::new(self.shared().clone(), format!("{}.load_shedding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `longitude` after provisioning.\nThe longitude of the data center containing the origins used in this pool in decimal degrees. If this is set, latitude must also be set."]
    pub fn longitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `minimum_origins` after provisioning.\nThe minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and will failover to the next available pool."]
    pub fn minimum_origins(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_origins", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitor` after provisioning.\nThe ID of the Monitor to use for checking the health of origins within this pool."]
    pub fn monitor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitor_group` after provisioning.\nThe ID of the Monitor Group to use for checking the health of origins within this pool."]
    pub fn monitor_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA short name (tag) for the pool. Only alphanumeric characters, hyphens, and underscores are allowed."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nList of networks where Load Balancer or Pool is enabled."]
    pub fn networks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_email` after provisioning.\nThis field is now deprecated. It has been moved to Cloudflare's Centralized Notification service https://developers.cloudflare.com/fundamentals/notifications/. The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list."]
    pub fn notification_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_filter` after provisioning.\nFilter pool and origin health notifications by resource type or health status. Use null to reset."]
    pub fn notification_filter(&self) -> LoadBalancerPool_NotificationFilterRef {
        LoadBalancerPool_NotificationFilterRef::new(
            self.shared().clone(),
            format!("{}.notification_filter", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `origin_steering` after provisioning.\nConfigures origin steering for the pool. Controls how origins are selected for new sessions and traffic without session affinity."]
    pub fn origin_steering(&self) -> LoadBalancerPool_OriginSteeringRef {
        LoadBalancerPool_OriginSteeringRef::new(
            self.shared().clone(),
            format!("{}.origin_steering", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `origins` after provisioning.\nThe list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy."]
    pub fn origins(&self) -> terrars::ListRef<LoadBalancerPool_OriginsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origins", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPool_LoadShedding {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LoadBalancerPool_LoadShedding {
    #[doc =
        "Set the field `default_percent`.\nThe percent of traffic to shed from the pool, according to the default policy. Applies to new sessions and traffic without session affinity."]
    pub fn set_default_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_policy`.\nThe default policy to use when load shedding. A random policy randomly sheds a given percent of requests. A hash policy computes a hash over the CF-Connecting-IP address and sheds all requests originating from a percent of IPs.\nAvailable values: \"random\", \"hash\"."]
    pub fn set_default_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_percent`.\nThe percent of existing sessions to shed from the pool, according to the session policy."]
    pub fn set_session_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.session_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_policy`.\nOnly the hash policy is supported for existing sessions (to avoid exponential decay).\nAvailable values: \"hash\"."]
    pub fn set_session_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.session_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerPool_LoadShedding {
    type O = terrars::BlockAssignable<LoadBalancerPool_LoadShedding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPool_LoadShedding {}

impl BuildLoadBalancerPool_LoadShedding {
    pub fn build(self) -> LoadBalancerPool_LoadShedding {
        LoadBalancerPool_LoadShedding {
            default_percent: core::default::Default::default(),
            default_policy: core::default::Default::default(),
            session_percent: core::default::Default::default(),
            session_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoadBalancerPool_LoadSheddingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerPool_LoadSheddingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerPool_LoadSheddingRef {
        LoadBalancerPool_LoadSheddingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPool_LoadSheddingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_percent` after provisioning.\nThe percent of traffic to shed from the pool, according to the default policy. Applies to new sessions and traffic without session affinity."]
    pub fn default_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_policy` after provisioning.\nThe default policy to use when load shedding. A random policy randomly sheds a given percent of requests. A hash policy computes a hash over the CF-Connecting-IP address and sheds all requests originating from a percent of IPs.\nAvailable values: \"random\", \"hash\"."]
    pub fn default_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_percent` after provisioning.\nThe percent of existing sessions to shed from the pool, according to the session policy."]
    pub fn session_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_policy` after provisioning.\nOnly the hash policy is supported for existing sessions (to avoid exponential decay).\nAvailable values: \"hash\"."]
    pub fn session_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPool_NotificationFilter_Origin {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy: std::option::Option<terrars::PrimField<bool>>,
}

impl LoadBalancerPool_NotificationFilter_Origin {
    #[doc = "Set the field `disable`.\nIf set true, disable notifications for this type of resource (pool or origin)."]
    pub fn set_disable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `healthy`.\nIf present, send notifications only for this health status (e.g. false for only DOWN events). Use null to reset (all events)."]
    pub fn set_healthy(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.healthy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerPool_NotificationFilter_Origin {
    type O = terrars::BlockAssignable<LoadBalancerPool_NotificationFilter_Origin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPool_NotificationFilter_Origin {}

impl BuildLoadBalancerPool_NotificationFilter_Origin {
    pub fn build(self) -> LoadBalancerPool_NotificationFilter_Origin {
        LoadBalancerPool_NotificationFilter_Origin {
            disable: core::default::Default::default(),
            healthy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoadBalancerPool_NotificationFilter_OriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerPool_NotificationFilter_OriginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerPool_NotificationFilter_OriginRef {
        LoadBalancerPool_NotificationFilter_OriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPool_NotificationFilter_OriginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable` after provisioning.\nIf set true, disable notifications for this type of resource (pool or origin)."]
    pub fn disable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `healthy` after provisioning.\nIf present, send notifications only for this health status (e.g. false for only DOWN events). Use null to reset (all events)."]
    pub fn healthy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPool_NotificationFilter_Pool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy: std::option::Option<terrars::PrimField<bool>>,
}

impl LoadBalancerPool_NotificationFilter_Pool {
    #[doc = "Set the field `disable`.\nIf set true, disable notifications for this type of resource (pool or origin)."]
    pub fn set_disable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `healthy`.\nIf present, send notifications only for this health status (e.g. false for only DOWN events). Use null to reset (all events)."]
    pub fn set_healthy(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.healthy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerPool_NotificationFilter_Pool {
    type O = terrars::BlockAssignable<LoadBalancerPool_NotificationFilter_Pool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPool_NotificationFilter_Pool {}

impl BuildLoadBalancerPool_NotificationFilter_Pool {
    pub fn build(self) -> LoadBalancerPool_NotificationFilter_Pool {
        LoadBalancerPool_NotificationFilter_Pool {
            disable: core::default::Default::default(),
            healthy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoadBalancerPool_NotificationFilter_PoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerPool_NotificationFilter_PoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerPool_NotificationFilter_PoolRef {
        LoadBalancerPool_NotificationFilter_PoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPool_NotificationFilter_PoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable` after provisioning.\nIf set true, disable notifications for this type of resource (pool or origin)."]
    pub fn disable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `healthy` after provisioning.\nIf present, send notifications only for this health status (e.g. false for only DOWN events). Use null to reset (all events)."]
    pub fn healthy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPool_NotificationFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<LoadBalancerPool_NotificationFilter_Origin>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool: std::option::Option<LoadBalancerPool_NotificationFilter_Pool>,
}

impl LoadBalancerPool_NotificationFilter {
    #[doc =
        "Set the field `origin`.\nFilter options for a particular resource type (pool or origin). Use null to reset."]
    pub fn set_origin(mut self, v: impl std::convert::Into<LoadBalancerPool_NotificationFilter_Origin>) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool`.\nFilter options for a particular resource type (pool or origin). Use null to reset."]
    pub fn set_pool(mut self, v: impl std::convert::Into<LoadBalancerPool_NotificationFilter_Pool>) -> Self {
        self.pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerPool_NotificationFilter {
    type O = terrars::BlockAssignable<LoadBalancerPool_NotificationFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPool_NotificationFilter {}

impl BuildLoadBalancerPool_NotificationFilter {
    pub fn build(self) -> LoadBalancerPool_NotificationFilter {
        LoadBalancerPool_NotificationFilter {
            origin: core::default::Default::default(),
            pool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoadBalancerPool_NotificationFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerPool_NotificationFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerPool_NotificationFilterRef {
        LoadBalancerPool_NotificationFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPool_NotificationFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nFilter options for a particular resource type (pool or origin). Use null to reset."]
    pub fn origin(&self) -> LoadBalancerPool_NotificationFilter_OriginRef {
        LoadBalancerPool_NotificationFilter_OriginRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pool` after provisioning.\nFilter options for a particular resource type (pool or origin). Use null to reset."]
    pub fn pool(&self) -> LoadBalancerPool_NotificationFilter_PoolRef {
        LoadBalancerPool_NotificationFilter_PoolRef::new(self.shared().clone(), format!("{}.pool", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPool_OriginSteering {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LoadBalancerPool_OriginSteering {
    #[doc =
        "Set the field `policy`.\nThe type of origin steering policy to use.\n- `\"random\"`: Select an origin randomly.\n- `\"hash\"`: Select an origin by computing a hash over the CF-Connecting-IP address.\n- `\"least_outstanding_requests\"`: Select an origin by taking into consideration origin weights, as well as each origin's number of outstanding requests. Origins with more pending requests are weighted proportionately less relative to others.\n- `\"least_connections\"`: Select an origin by taking into consideration origin weights, as well as each origin's number of open connections. Origins with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections.\nAvailable values: \"random\", \"hash\", \"least_outstanding_requests\", \"least_connections\"."]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerPool_OriginSteering {
    type O = terrars::BlockAssignable<LoadBalancerPool_OriginSteering>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPool_OriginSteering {}

impl BuildLoadBalancerPool_OriginSteering {
    pub fn build(self) -> LoadBalancerPool_OriginSteering {
        LoadBalancerPool_OriginSteering { policy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LoadBalancerPool_OriginSteeringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerPool_OriginSteeringRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerPool_OriginSteeringRef {
        LoadBalancerPool_OriginSteeringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPool_OriginSteeringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `policy` after provisioning.\nThe type of origin steering policy to use.\n- `\"random\"`: Select an origin randomly.\n- `\"hash\"`: Select an origin by computing a hash over the CF-Connecting-IP address.\n- `\"least_outstanding_requests\"`: Select an origin by taking into consideration origin weights, as well as each origin's number of outstanding requests. Origins with more pending requests are weighted proportionately less relative to others.\n- `\"least_connections\"`: Select an origin by taking into consideration origin weights, as well as each origin's number of open connections. Origins with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections.\nAvailable values: \"random\", \"hash\", \"least_outstanding_requests\", \"least_connections\"."]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPool_Origins_Header {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl LoadBalancerPool_Origins_Header {
    #[doc =
        "Set the field `host`.\nThe 'Host' header allows to override the hostname set in the HTTP request. Current support is 1 'Host' header override per origin."]
    pub fn set_host(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerPool_Origins_Header {
    type O = terrars::BlockAssignable<LoadBalancerPool_Origins_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPool_Origins_Header {}

impl BuildLoadBalancerPool_Origins_Header {
    pub fn build(self) -> LoadBalancerPool_Origins_Header {
        LoadBalancerPool_Origins_Header { host: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LoadBalancerPool_Origins_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerPool_Origins_HeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerPool_Origins_HeaderRef {
        LoadBalancerPool_Origins_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPool_Origins_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe 'Host' header allows to override the hostname set in the HTTP request. Current support is 1 'Host' header override per origin."]
    pub fn host(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerPool_Origins {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flatten_cname: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<LoadBalancerPool_Origins_Header>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl LoadBalancerPool_Origins {
    #[doc =
        "Set the field `address`.\nThe IP address (IPv4 or IPv6) of the origin, or its publicly addressable hostname. Hostnames entered here should resolve directly to the origin, and not be a hostname proxied by Cloudflare. To set an internal/reserved address, virtual_network_id must also be set."]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enabled`.\nWhether to enable (the default) this origin within the pool. Disabled origins will not receive traffic and are excluded from health checks. The origin will only be disabled for the current pool."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `flatten_cname`.\nWhether to flatten CNAME records for this origin, resolving them to A/AAAA records before returning to the client. When true (the default), the director resolves CNAME addresses to their underlying A/AAAA records. When false, the origin address is returned as a raw CNAME record without resolution. This setting mirrors the DNS API record flatten_cname setting."]
    pub fn set_flatten_cname(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.flatten_cname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `header`.\nThe request header is used to pass additional information with an HTTP request. Currently supported header is 'Host'."]
    pub fn set_header(mut self, v: impl std::convert::Into<LoadBalancerPool_Origins_Header>) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nA human-identifiable name for the origin."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nThe port for upstream connections. A value of 0 means the default port for the protocol will be used."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `virtual_network_id`.\nThe virtual network subnet ID the origin belongs in. Virtual network must also belong to the account."]
    pub fn set_virtual_network_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.virtual_network_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `weight`.\nThe weight of this origin relative to other origins in the pool. Based on the configured weight the total traffic is distributed among origins within the pool.\n- `origin_steering.policy=\"least_outstanding_requests\"`: Use weight to scale the origin's outstanding requests.\n- `origin_steering.policy=\"least_connections\"`: Use weight to scale the origin's open connections."]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoadBalancerPool_Origins {
    type O = terrars::BlockAssignable<LoadBalancerPool_Origins>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerPool_Origins {}

impl BuildLoadBalancerPool_Origins {
    pub fn build(self) -> LoadBalancerPool_Origins {
        LoadBalancerPool_Origins {
            address: core::default::Default::default(),
            enabled: core::default::Default::default(),
            flatten_cname: core::default::Default::default(),
            header: core::default::Default::default(),
            name: core::default::Default::default(),
            port: core::default::Default::default(),
            virtual_network_id: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoadBalancerPool_OriginsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerPool_OriginsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoadBalancerPool_OriginsRef {
        LoadBalancerPool_OriginsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerPool_OriginsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nThe IP address (IPv4 or IPv6) of the origin, or its publicly addressable hostname. Hostnames entered here should resolve directly to the origin, and not be a hostname proxied by Cloudflare. To set an internal/reserved address, virtual_network_id must also be set."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled_at` after provisioning.\nThis field shows up only if the origin is disabled. This field is set with the time the origin was disabled."]
    pub fn disabled_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable (the default) this origin within the pool. Disabled origins will not receive traffic and are excluded from health checks. The origin will only be disabled for the current pool."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `flatten_cname` after provisioning.\nWhether to flatten CNAME records for this origin, resolving them to A/AAAA records before returning to the client. When true (the default), the director resolves CNAME addresses to their underlying A/AAAA records. When false, the origin address is returned as a raw CNAME record without resolution. This setting mirrors the DNS API record flatten_cname setting."]
    pub fn flatten_cname(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flatten_cname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header` after provisioning.\nThe request header is used to pass additional information with an HTTP request. Currently supported header is 'Host'."]
    pub fn header(&self) -> LoadBalancerPool_Origins_HeaderRef {
        LoadBalancerPool_Origins_HeaderRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA human-identifiable name for the origin."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port for upstream connections. A value of 0 means the default port for the protocol will be used."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\nThe virtual network subnet ID the origin belongs in. Virtual network must also belong to the account."]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nThe weight of this origin relative to other origins in the pool. Based on the configured weight the total traffic is distributed among origins within the pool.\n- `origin_steering.policy=\"least_outstanding_requests\"`: Use weight to scale the origin's outstanding requests.\n- `origin_steering.policy=\"least_connections\"`: Use weight to scale the origin's open connections."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
