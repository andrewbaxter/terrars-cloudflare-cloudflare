#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct HealthcheckData_ {
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
    address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consecutive_fails: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consecutive_successes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_config: std::option::Option<Healthcheck_HttpConfig>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspended: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_config: std::option::Option<Healthcheck_TcpConfig>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct Healthcheck_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<HealthcheckData_>,
}

#[derive(Clone)]
pub struct Healthcheck(std::rc::Rc<Healthcheck_>);

impl Healthcheck {
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
        "Set the field `check_regions`.\nA list of regions from which to run health checks. Null means Cloudflare will pick a default region."]
    pub fn set_check_regions(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().check_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consecutive_fails`.\nThe number of consecutive fails required from a health check before changing the health to unhealthy."]
    pub fn set_consecutive_fails(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().consecutive_fails = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consecutive_successes`.\nThe number of consecutive successes required from a health check before changing the health to healthy."]
    pub fn set_consecutive_successes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().consecutive_successes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA human-readable description of the health check."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_config`.\nParameters specific to an HTTP or HTTPS health check."]
    pub fn set_http_config(self, v: impl std::convert::Into<Healthcheck_HttpConfig>) -> Self {
        self.0.data.borrow_mut().http_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interval`.\nThe interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase load on the origin as we check from multiple locations."]
    pub fn set_interval(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retries`.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately."]
    pub fn set_retries(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suspended`.\nIf suspended, no health checks are sent to the origin."]
    pub fn set_suspended(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_config`.\nParameters specific to TCP health check."]
    pub fn set_tcp_config(self, v: impl std::convert::Into<Healthcheck_TcpConfig>) -> Self {
        self.0.data.borrow_mut().tcp_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nThe timeout (in seconds) before marking the health check as failed."]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe protocol to use for the health check. Currently supported protocols are 'HTTP', 'HTTPS' and 'TCP'."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nThe hostname or IP address of the origin server to run health checks on."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `check_regions` after provisioning.\nA list of regions from which to run health checks. Null means Cloudflare will pick a default region."]
    pub fn check_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_regions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_fails` after provisioning.\nThe number of consecutive fails required from a health check before changing the health to unhealthy."]
    pub fn consecutive_fails(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_fails", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_successes` after provisioning.\nThe number of consecutive successes required from a health check before changing the health to healthy."]
    pub fn consecutive_successes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_successes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the health check."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_reason` after provisioning.\nThe current failure reason if status is unhealthy."]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `http_config` after provisioning.\nParameters specific to an HTTP or HTTPS health check."]
    pub fn http_config(&self) -> Healthcheck_HttpConfigRef {
        Healthcheck_HttpConfigRef::new(self.shared().clone(), format!("{}.http_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nThe interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase load on the origin as we check from multiple locations."]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA short name to identify the health check. Only alphanumeric characters, hyphens and underscores are allowed."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retries` after provisioning.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately."]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe current status of the origin server according to the health check.\nAvailable values: \"unknown\", \"healthy\", \"unhealthy\", \"suspended\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nIf suspended, no health checks are sent to the origin."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tcp_config` after provisioning.\nParameters specific to TCP health check."]
    pub fn tcp_config(&self) -> Healthcheck_TcpConfigRef {
        Healthcheck_TcpConfigRef::new(self.shared().clone(), format!("{}.tcp_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) before marking the health check as failed."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe protocol to use for the health check. Currently supported protocols are 'HTTP', 'HTTPS' and 'TCP'."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for Healthcheck {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Healthcheck { }

impl terrars::ToListMappable for Healthcheck {
    type O = terrars::ListRef<HealthcheckRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Healthcheck_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_healthcheck".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildHealthcheck {
    pub tf_id: std::string::String,
    #[doc = "The hostname or IP address of the origin server to run health checks on."]
    pub address: terrars::PrimField<std::string::String>,
    #[doc =
        "A short name to identify the health check. Only alphanumeric characters, hyphens and underscores are allowed."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Identifier"]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildHealthcheck {
    pub fn build(self, stack: &mut terrars::Stack) -> Healthcheck {
        let out = Healthcheck(std::rc::Rc::new(Healthcheck_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(HealthcheckData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                address: self.address,
                check_regions: core::default::Default::default(),
                consecutive_fails: core::default::Default::default(),
                consecutive_successes: core::default::Default::default(),
                description: core::default::Default::default(),
                http_config: core::default::Default::default(),
                interval: core::default::Default::default(),
                name: self.name,
                retries: core::default::Default::default(),
                suspended: core::default::Default::default(),
                tcp_config: core::default::Default::default(),
                timeout: core::default::Default::default(),
                type_: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct HealthcheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl HealthcheckRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nThe hostname or IP address of the origin server to run health checks on."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `check_regions` after provisioning.\nA list of regions from which to run health checks. Null means Cloudflare will pick a default region."]
    pub fn check_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_regions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_fails` after provisioning.\nThe number of consecutive fails required from a health check before changing the health to unhealthy."]
    pub fn consecutive_fails(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_fails", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_successes` after provisioning.\nThe number of consecutive successes required from a health check before changing the health to healthy."]
    pub fn consecutive_successes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_successes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the health check."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_reason` after provisioning.\nThe current failure reason if status is unhealthy."]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `http_config` after provisioning.\nParameters specific to an HTTP or HTTPS health check."]
    pub fn http_config(&self) -> Healthcheck_HttpConfigRef {
        Healthcheck_HttpConfigRef::new(self.shared().clone(), format!("{}.http_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nThe interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase load on the origin as we check from multiple locations."]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA short name to identify the health check. Only alphanumeric characters, hyphens and underscores are allowed."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retries` after provisioning.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately."]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe current status of the origin server according to the health check.\nAvailable values: \"unknown\", \"healthy\", \"unhealthy\", \"suspended\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nIf suspended, no health checks are sent to the origin."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tcp_config` after provisioning.\nParameters specific to TCP health check."]
    pub fn tcp_config(&self) -> Healthcheck_TcpConfigRef {
        Healthcheck_TcpConfigRef::new(self.shared().clone(), format!("{}.tcp_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) before marking the health check as failed."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe protocol to use for the health check. Currently supported protocols are 'HTTP', 'HTTPS' and 'TCP'."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Healthcheck_HttpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    follow_redirects: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl Healthcheck_HttpConfig {
    #[doc = "Set the field `allow_insecure`.\nDo not validate the certificate when the health check uses HTTPS."]
    pub fn set_allow_insecure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expected_body`.\nA case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy."]
    pub fn set_expected_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expected_body = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expected_codes`.\nThe expected HTTP response codes (e.g. \"200\") or code ranges (e.g. \"2xx\" for all codes starting with 2) of the health check."]
    pub fn set_expected_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expected_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `follow_redirects`.\nFollow redirects if the origin returns a 3xx status code."]
    pub fn set_follow_redirects(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.follow_redirects = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `header`.\nThe HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden."]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `method`.\nThe HTTP method to use for the health check.\nAvailable values: \"GET\", \"HEAD\"."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nThe endpoint path to health check against."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nPort number to connect to for the health check. Defaults to 80 if type is HTTP or 443 if type is HTTPS."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Healthcheck_HttpConfig {
    type O = terrars::BlockAssignable<Healthcheck_HttpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcheck_HttpConfig {}

impl BuildHealthcheck_HttpConfig {
    pub fn build(self) -> Healthcheck_HttpConfig {
        Healthcheck_HttpConfig {
            allow_insecure: core::default::Default::default(),
            expected_body: core::default::Default::default(),
            expected_codes: core::default::Default::default(),
            follow_redirects: core::default::Default::default(),
            header: core::default::Default::default(),
            method: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Healthcheck_HttpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Healthcheck_HttpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Healthcheck_HttpConfigRef {
        Healthcheck_HttpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Healthcheck_HttpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDo not validate the certificate when the health check uses HTTPS."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expected_body` after provisioning.\nA case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy."]
    pub fn expected_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expected_codes` after provisioning.\nThe expected HTTP response codes (e.g. \"200\") or code ranges (e.g. \"2xx\" for all codes starting with 2) of the health check."]
    pub fn expected_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expected_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `follow_redirects` after provisioning.\nFollow redirects if the origin returns a 3xx status code."]
    pub fn follow_redirects(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.follow_redirects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header` after provisioning.\nThe HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden."]
    pub fn header(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nThe HTTP method to use for the health check.\nAvailable values: \"GET\", \"HEAD\"."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe endpoint path to health check against."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to connect to for the health check. Defaults to 80 if type is HTTP or 443 if type is HTTPS."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct Healthcheck_TcpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl Healthcheck_TcpConfig {
    #[doc =
        "Set the field `method`.\nThe TCP connection method to use for the health check.\nAvailable values: \"connection_established\"."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nPort number to connect to for the health check. Defaults to 80."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Healthcheck_TcpConfig {
    type O = terrars::BlockAssignable<Healthcheck_TcpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcheck_TcpConfig {}

impl BuildHealthcheck_TcpConfig {
    pub fn build(self) -> Healthcheck_TcpConfig {
        Healthcheck_TcpConfig {
            method: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Healthcheck_TcpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Healthcheck_TcpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Healthcheck_TcpConfigRef {
        Healthcheck_TcpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Healthcheck_TcpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nThe TCP connection method to use for the health check.\nAvailable values: \"connection_established\"."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to connect to for the health check. Defaults to 80."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
