#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataLoadBalancerPoolsData_ {
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
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitor: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLoadBalancerPools_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLoadBalancerPoolsData_>,
}

#[derive(Clone)]
pub struct DataLoadBalancerPools(std::rc::Rc<DataLoadBalancerPools_>);

impl DataLoadBalancerPools {
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

    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `monitor`.\nThe ID of the Monitor to use for checking the health of origins within this pool."]
    pub fn set_monitor(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().monitor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitor` after provisioning.\nThe ID of the Monitor to use for checking the health of origins within this pool."]
    pub fn monitor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataLoadBalancerPools_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataLoadBalancerPools {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLoadBalancerPools { }

impl terrars::ToListMappable for DataLoadBalancerPools {
    type O = terrars::ListRef<DataLoadBalancerPoolsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLoadBalancerPools_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_load_balancer_pools".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLoadBalancerPools {
    pub tf_id: std::string::String,
}

impl BuildDataLoadBalancerPools {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLoadBalancerPools {
        let out = DataLoadBalancerPools(std::rc::Rc::new(DataLoadBalancerPools_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLoadBalancerPoolsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                monitor: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLoadBalancerPoolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPoolsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLoadBalancerPoolsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitor` after provisioning.\nThe ID of the Monitor to use for checking the health of origins within this pool."]
    pub fn monitor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataLoadBalancerPools_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPools_Result_LoadShedding {}

impl DataLoadBalancerPools_Result_LoadShedding { }

impl terrars::ToListMappable for DataLoadBalancerPools_Result_LoadShedding {
    type O = terrars::BlockAssignable<DataLoadBalancerPools_Result_LoadShedding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPools_Result_LoadShedding {}

impl BuildDataLoadBalancerPools_Result_LoadShedding {
    pub fn build(self) -> DataLoadBalancerPools_Result_LoadShedding {
        DataLoadBalancerPools_Result_LoadShedding {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerPools_Result_LoadSheddingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPools_Result_LoadSheddingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancerPools_Result_LoadSheddingRef {
        DataLoadBalancerPools_Result_LoadSheddingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPools_Result_LoadSheddingRef {
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
pub struct DataLoadBalancerPools_Result_NotificationFilter_Origin {}

impl DataLoadBalancerPools_Result_NotificationFilter_Origin { }

impl terrars::ToListMappable for DataLoadBalancerPools_Result_NotificationFilter_Origin {
    type O = terrars::BlockAssignable<DataLoadBalancerPools_Result_NotificationFilter_Origin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPools_Result_NotificationFilter_Origin {}

impl BuildDataLoadBalancerPools_Result_NotificationFilter_Origin {
    pub fn build(self) -> DataLoadBalancerPools_Result_NotificationFilter_Origin {
        DataLoadBalancerPools_Result_NotificationFilter_Origin {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerPools_Result_NotificationFilter_OriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPools_Result_NotificationFilter_OriginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancerPools_Result_NotificationFilter_OriginRef {
        DataLoadBalancerPools_Result_NotificationFilter_OriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPools_Result_NotificationFilter_OriginRef {
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
pub struct DataLoadBalancerPools_Result_NotificationFilter_Pool {}

impl DataLoadBalancerPools_Result_NotificationFilter_Pool { }

impl terrars::ToListMappable for DataLoadBalancerPools_Result_NotificationFilter_Pool {
    type O = terrars::BlockAssignable<DataLoadBalancerPools_Result_NotificationFilter_Pool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPools_Result_NotificationFilter_Pool {}

impl BuildDataLoadBalancerPools_Result_NotificationFilter_Pool {
    pub fn build(self) -> DataLoadBalancerPools_Result_NotificationFilter_Pool {
        DataLoadBalancerPools_Result_NotificationFilter_Pool {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerPools_Result_NotificationFilter_PoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPools_Result_NotificationFilter_PoolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancerPools_Result_NotificationFilter_PoolRef {
        DataLoadBalancerPools_Result_NotificationFilter_PoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPools_Result_NotificationFilter_PoolRef {
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
pub struct DataLoadBalancerPools_Result_NotificationFilter {}

impl DataLoadBalancerPools_Result_NotificationFilter { }

impl terrars::ToListMappable for DataLoadBalancerPools_Result_NotificationFilter {
    type O = terrars::BlockAssignable<DataLoadBalancerPools_Result_NotificationFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPools_Result_NotificationFilter {}

impl BuildDataLoadBalancerPools_Result_NotificationFilter {
    pub fn build(self) -> DataLoadBalancerPools_Result_NotificationFilter {
        DataLoadBalancerPools_Result_NotificationFilter {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerPools_Result_NotificationFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPools_Result_NotificationFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancerPools_Result_NotificationFilterRef {
        DataLoadBalancerPools_Result_NotificationFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPools_Result_NotificationFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nFilter options for a particular resource type (pool or origin). Use null to reset."]
    pub fn origin(&self) -> DataLoadBalancerPools_Result_NotificationFilter_OriginRef {
        DataLoadBalancerPools_Result_NotificationFilter_OriginRef::new(
            self.shared().clone(),
            format!("{}.origin", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `pool` after provisioning.\nFilter options for a particular resource type (pool or origin). Use null to reset."]
    pub fn pool(&self) -> DataLoadBalancerPools_Result_NotificationFilter_PoolRef {
        DataLoadBalancerPools_Result_NotificationFilter_PoolRef::new(
            self.shared().clone(),
            format!("{}.pool", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataLoadBalancerPools_Result_OriginSteering {}

impl DataLoadBalancerPools_Result_OriginSteering { }

impl terrars::ToListMappable for DataLoadBalancerPools_Result_OriginSteering {
    type O = terrars::BlockAssignable<DataLoadBalancerPools_Result_OriginSteering>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPools_Result_OriginSteering {}

impl BuildDataLoadBalancerPools_Result_OriginSteering {
    pub fn build(self) -> DataLoadBalancerPools_Result_OriginSteering {
        DataLoadBalancerPools_Result_OriginSteering {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerPools_Result_OriginSteeringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPools_Result_OriginSteeringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancerPools_Result_OriginSteeringRef {
        DataLoadBalancerPools_Result_OriginSteeringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPools_Result_OriginSteeringRef {
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
pub struct DataLoadBalancerPools_Result_Origins_Header {}

impl DataLoadBalancerPools_Result_Origins_Header { }

impl terrars::ToListMappable for DataLoadBalancerPools_Result_Origins_Header {
    type O = terrars::BlockAssignable<DataLoadBalancerPools_Result_Origins_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPools_Result_Origins_Header {}

impl BuildDataLoadBalancerPools_Result_Origins_Header {
    pub fn build(self) -> DataLoadBalancerPools_Result_Origins_Header {
        DataLoadBalancerPools_Result_Origins_Header {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerPools_Result_Origins_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPools_Result_Origins_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLoadBalancerPools_Result_Origins_HeaderRef {
        DataLoadBalancerPools_Result_Origins_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPools_Result_Origins_HeaderRef {
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
pub struct DataLoadBalancerPools_Result_Origins {}

impl DataLoadBalancerPools_Result_Origins { }

impl terrars::ToListMappable for DataLoadBalancerPools_Result_Origins {
    type O = terrars::BlockAssignable<DataLoadBalancerPools_Result_Origins>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPools_Result_Origins {}

impl BuildDataLoadBalancerPools_Result_Origins {
    pub fn build(self) -> DataLoadBalancerPools_Result_Origins {
        DataLoadBalancerPools_Result_Origins {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerPools_Result_OriginsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPools_Result_OriginsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancerPools_Result_OriginsRef {
        DataLoadBalancerPools_Result_OriginsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPools_Result_OriginsRef {
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
    pub fn header(&self) -> DataLoadBalancerPools_Result_Origins_HeaderRef {
        DataLoadBalancerPools_Result_Origins_HeaderRef::new(self.shared().clone(), format!("{}.header", self.base))
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

#[derive(Serialize)]
pub struct DataLoadBalancerPools_Result {}

impl DataLoadBalancerPools_Result { }

impl terrars::ToListMappable for DataLoadBalancerPools_Result {
    type O = terrars::BlockAssignable<DataLoadBalancerPools_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadBalancerPools_Result {}

impl BuildDataLoadBalancerPools_Result {
    pub fn build(self) -> DataLoadBalancerPools_Result {
        DataLoadBalancerPools_Result {}
    }
}

#[allow(dead_code)]
pub struct DataLoadBalancerPools_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadBalancerPools_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadBalancerPools_ResultRef {
        DataLoadBalancerPools_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadBalancerPools_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_regions` after provisioning.\nA list of regions from which to run health checks. Null means every Cloudflare data center."]
    pub fn check_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_regions", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the pool."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled_at` after provisioning.\nThis field shows up only if the pool is disabled. This field is set with the time the pool was disabled at."]
    pub fn disabled_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable (the default) or disable this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any)."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `health_sources` after provisioning.\nA list of health sources, ordered from highest to lowest priority, used to evaluate individual origin health and overall pool health. The load balancer uses the first source that has data and falls back to the next. Currently accepted values are null or the exact array [\"regional\", \"global\"]; any other combination is rejected. Null (the default) behaves like [\"local\", \"global\"]. [\"regional\", \"global\"] makes each region steer on its own health, falling back to the global decision when a region has no fresh data. Setting regional requires at least one region in check_regions."]
    pub fn health_sources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `latitude` after provisioning.\nThe latitude of the data center containing the origins used in this pool in decimal degrees. If this is set, longitude must also be set."]
    pub fn latitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.base))
    }

    #[doc =
        "Get a reference to the value of field `load_shedding` after provisioning.\nConfigures load shedding policies and percentages for the pool."]
    pub fn load_shedding(&self) -> DataLoadBalancerPools_Result_LoadSheddingRef {
        DataLoadBalancerPools_Result_LoadSheddingRef::new(
            self.shared().clone(),
            format!("{}.load_shedding", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `longitude` after provisioning.\nThe longitude of the data center containing the origins used in this pool in decimal degrees. If this is set, latitude must also be set."]
    pub fn longitude(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_origins` after provisioning.\nThe minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and will failover to the next available pool."]
    pub fn minimum_origins(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monitor` after provisioning.\nThe ID of the Monitor to use for checking the health of origins within this pool."]
    pub fn monitor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monitor_group` after provisioning.\nThe ID of the Monitor Group to use for checking the health of origins within this pool."]
    pub fn monitor_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor_group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA short name (tag) for the pool. Only alphanumeric characters, hyphens, and underscores are allowed."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nList of networks where Load Balancer or Pool is enabled."]
    pub fn networks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `notification_email` after provisioning.\nThis field is now deprecated. It has been moved to Cloudflare's Centralized Notification service https://developers.cloudflare.com/fundamentals/notifications/. The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list."]
    pub fn notification_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `notification_filter` after provisioning.\nFilter pool and origin health notifications by resource type or health status. Use null to reset."]
    pub fn notification_filter(&self) -> DataLoadBalancerPools_Result_NotificationFilterRef {
        DataLoadBalancerPools_Result_NotificationFilterRef::new(
            self.shared().clone(),
            format!("{}.notification_filter", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `origin_steering` after provisioning.\nConfigures origin steering for the pool. Controls how origins are selected for new sessions and traffic without session affinity."]
    pub fn origin_steering(&self) -> DataLoadBalancerPools_Result_OriginSteeringRef {
        DataLoadBalancerPools_Result_OriginSteeringRef::new(
            self.shared().clone(),
            format!("{}.origin_steering", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `origins` after provisioning.\nThe list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy."]
    pub fn origins(&self) -> terrars::SetRef<DataLoadBalancerPools_Result_OriginsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.origins", self.base))
    }
}
