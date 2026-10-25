#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct LoadBalancerMonitorData_ {
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
    allow_insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consecutive_down: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consecutive_up: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_codes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    follow_redirects: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    probe_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct LoadBalancerMonitor_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LoadBalancerMonitorData_>,
}

#[derive(Clone)]
pub struct LoadBalancerMonitor(std::rc::Rc<LoadBalancerMonitor_>);

impl LoadBalancerMonitor {
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
        "Set the field `allow_insecure`.\nDo not validate the certificate when monitor use HTTPS. This parameter is currently only valid for HTTP and HTTPS monitors."]
    pub fn set_allow_insecure(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consecutive_down`.\nTo be marked unhealthy the monitored origin must fail this healthcheck N consecutive times."]
    pub fn set_consecutive_down(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().consecutive_down = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consecutive_up`.\nTo be marked healthy the monitored origin must pass this healthcheck N consecutive times."]
    pub fn set_consecutive_up(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().consecutive_up = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nObject description."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expected_body`.\nA case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn set_expected_body(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expected_body = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expected_codes`.\nThe expected HTTP response code or code range of the health check. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn set_expected_codes(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expected_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `follow_redirects`.\nFollow redirects if returned by the origin. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn set_follow_redirects(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().follow_redirects = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `header`.\nThe HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn set_header(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::ListField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.0.data.borrow_mut().header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interval`.\nThe interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations."]
    pub fn set_interval(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `method`.\nThe method to use for the health check. This defaults to 'GET' for HTTP/HTTPS based checks and 'connection_established' for TCP based health checks."]
    pub fn set_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nThe endpoint path you want to conduct a health check against. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn set_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nThe port number to connect to for the health check. Required for TCP, UDP, and SMTP checks. HTTP and HTTPS checks should only define the port when using a non-standard port (HTTP: default 80, HTTPS: default 443)."]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `probe_zone`.\nAssign this monitor to emulate the specified zone while probing. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn set_probe_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().probe_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retries`.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately."]
    pub fn set_retries(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nThe timeout (in seconds) before marking the health check as failed."]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe protocol to use for the health check. Currently supported protocols are 'HTTP','HTTPS', 'TCP', 'ICMP-PING', 'UDP-ICMP', and 'SMTP'.\nAvailable values: \"http\", \"https\", \"tcp\", \"udp_icmp\", \"icmp_ping\", \"smtp\"."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDo not validate the certificate when monitor use HTTPS. This parameter is currently only valid for HTTP and HTTPS monitors."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_down` after provisioning.\nTo be marked unhealthy the monitored origin must fail this healthcheck N consecutive times."]
    pub fn consecutive_down(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_down", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_up` after provisioning.\nTo be marked healthy the monitored origin must pass this healthcheck N consecutive times."]
    pub fn consecutive_up(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_up", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nObject description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expected_body` after provisioning.\nA case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn expected_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_body", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expected_codes` after provisioning.\nThe expected HTTP response code or code range of the health check. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn expected_codes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_codes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `follow_redirects` after provisioning.\nFollow redirects if returned by the origin. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn follow_redirects(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.follow_redirects", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `header` after provisioning.\nThe HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn header(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.header", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nThe interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations."]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nThe method to use for the health check. This defaults to 'GET' for HTTP/HTTPS based checks and 'connection_established' for TCP based health checks."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe endpoint path you want to conduct a health check against. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number to connect to for the health check. Required for TCP, UDP, and SMTP checks. HTTP and HTTPS checks should only define the port when using a non-standard port (HTTP: default 80, HTTPS: default 443)."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `probe_zone` after provisioning.\nAssign this monitor to emulate the specified zone while probing. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn probe_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.probe_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retries` after provisioning.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately."]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) before marking the health check as failed."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe protocol to use for the health check. Currently supported protocols are 'HTTP','HTTPS', 'TCP', 'ICMP-PING', 'UDP-ICMP', and 'SMTP'.\nAvailable values: \"http\", \"https\", \"tcp\", \"udp_icmp\", \"icmp_ping\", \"smtp\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for LoadBalancerMonitor {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LoadBalancerMonitor { }

impl terrars::ToListMappable for LoadBalancerMonitor {
    type O = terrars::ListRef<LoadBalancerMonitorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LoadBalancerMonitor_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_load_balancer_monitor".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoadBalancerMonitor {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildLoadBalancerMonitor {
    pub fn build(self, stack: &mut terrars::Stack) -> LoadBalancerMonitor {
        let out = LoadBalancerMonitor(std::rc::Rc::new(LoadBalancerMonitor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LoadBalancerMonitorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                allow_insecure: core::default::Default::default(),
                consecutive_down: core::default::Default::default(),
                consecutive_up: core::default::Default::default(),
                description: core::default::Default::default(),
                expected_body: core::default::Default::default(),
                expected_codes: core::default::Default::default(),
                follow_redirects: core::default::Default::default(),
                header: core::default::Default::default(),
                interval: core::default::Default::default(),
                method: core::default::Default::default(),
                path: core::default::Default::default(),
                port: core::default::Default::default(),
                probe_zone: core::default::Default::default(),
                retries: core::default::Default::default(),
                timeout: core::default::Default::default(),
                type_: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoadBalancerMonitorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadBalancerMonitorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoadBalancerMonitorRef {
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
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDo not validate the certificate when monitor use HTTPS. This parameter is currently only valid for HTTP and HTTPS monitors."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_down` after provisioning.\nTo be marked unhealthy the monitored origin must fail this healthcheck N consecutive times."]
    pub fn consecutive_down(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_down", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_up` after provisioning.\nTo be marked healthy the monitored origin must pass this healthcheck N consecutive times."]
    pub fn consecutive_up(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_up", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nObject description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expected_body` after provisioning.\nA case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn expected_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_body", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expected_codes` after provisioning.\nThe expected HTTP response code or code range of the health check. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn expected_codes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_codes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `follow_redirects` after provisioning.\nFollow redirects if returned by the origin. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn follow_redirects(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.follow_redirects", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `header` after provisioning.\nThe HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn header(&self) -> terrars::RecRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.header", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nThe interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations."]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nThe method to use for the health check. This defaults to 'GET' for HTTP/HTTPS based checks and 'connection_established' for TCP based health checks."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe endpoint path you want to conduct a health check against. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number to connect to for the health check. Required for TCP, UDP, and SMTP checks. HTTP and HTTPS checks should only define the port when using a non-standard port (HTTP: default 80, HTTPS: default 443)."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `probe_zone` after provisioning.\nAssign this monitor to emulate the specified zone while probing. This parameter is only valid for HTTP and HTTPS monitors."]
    pub fn probe_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.probe_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retries` after provisioning.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately."]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) before marking the health check as failed."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe protocol to use for the health check. Currently supported protocols are 'HTTP','HTTPS', 'TCP', 'ICMP-PING', 'UDP-ICMP', and 'SMTP'.\nAvailable values: \"http\", \"https\", \"tcp\", \"udp_icmp\", \"icmp_ping\", \"smtp\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}
