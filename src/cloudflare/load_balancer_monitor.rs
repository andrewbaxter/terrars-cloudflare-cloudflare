use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct LoadBalancerMonitorData {
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
    allow_insecure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_codes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    follow_redirects: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    probe_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<LoadBalancerMonitorHeaderEl>>,
    dynamic: LoadBalancerMonitorDynamic,
}

struct LoadBalancerMonitor_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LoadBalancerMonitorData>,
}

#[derive(Clone)]
pub struct LoadBalancerMonitor(Rc<LoadBalancerMonitor_>);

impl LoadBalancerMonitor {
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

    #[doc= "Set the field `allow_insecure`.\nDo not validate the certificate when monitor use HTTPS.  Only valid if `type` is \"http\" or \"https\"."]
    pub fn set_allow_insecure(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_insecure = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nFree text description."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `expected_body`.\nA case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. Only valid if `type` is \"http\" or \"https\"."]
    pub fn set_expected_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_body = Some(v.into());
        self
    }

    #[doc= "Set the field `expected_codes`.\nThe expected HTTP response code or code range of the health check. Eg `2xx`. Only valid and required if `type` is \"http\" or \"https\"."]
    pub fn set_expected_codes(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_codes = Some(v.into());
        self
    }

    #[doc= "Set the field `follow_redirects`.\nFollow redirects if returned by the origin. Only valid if `type` is \"http\" or \"https\"."]
    pub fn set_follow_redirects(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().follow_redirects = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `interval`.\nThe interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations. Defaults to `60`."]
    pub fn set_interval(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().interval = Some(v.into());
        self
    }

    #[doc= "Set the field `method`.\nThe method to use for the health check."]
    pub fn set_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().method = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\nThe endpoint path to health check against."]
    pub fn set_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().path = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\nThe port number to use for the healthcheck, required when creating a TCP monitor."]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }

    #[doc= "Set the field `probe_zone`.\nAssign this monitor to emulate the specified zone while probing. Only valid if `type` is \"http\" or \"https\"."]
    pub fn set_probe_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().probe_zone = Some(v.into());
        self
    }

    #[doc= "Set the field `retries`.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`."]
    pub fn set_retries(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retries = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout`.\nThe timeout (in seconds) before marking the health check as failed. Defaults to `5`."]
    pub fn set_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe protocol to use for the healthcheck. Available values: `http`, `https`, `tcp`, `udp_icmp`, `icmp_ping`, `smtp`. Defaults to `http`."]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `header`.\n"]
    pub fn set_header(self, v: impl Into<BlockAssignable<LoadBalancerMonitorHeaderEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().header = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.header = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_insecure` after provisioning.\nDo not validate the certificate when monitor use HTTPS.  Only valid if `type` is \"http\" or \"https\"."]
    pub fn allow_insecure(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer monitor was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nFree text description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expected_body` after provisioning.\nA case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. Only valid if `type` is \"http\" or \"https\"."]
    pub fn expected_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expected_codes` after provisioning.\nThe expected HTTP response code or code range of the health check. Eg `2xx`. Only valid and required if `type` is \"http\" or \"https\"."]
    pub fn expected_codes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_codes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `follow_redirects` after provisioning.\nFollow redirects if returned by the origin. Only valid if `type` is \"http\" or \"https\"."]
    pub fn follow_redirects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.follow_redirects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nThe interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations. Defaults to `60`."]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `method` after provisioning.\nThe method to use for the health check."]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer monitor was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe endpoint path to health check against."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nThe port number to use for the healthcheck, required when creating a TCP monitor."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `probe_zone` after provisioning.\nAssign this monitor to emulate the specified zone while probing. Only valid if `type` is \"http\" or \"https\"."]
    pub fn probe_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.probe_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retries` after provisioning.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`."]
    pub fn retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retries", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) before marking the health check as failed. Defaults to `5`."]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe protocol to use for the healthcheck. Available values: `http`, `https`, `tcp`, `udp_icmp`, `icmp_ping`, `smtp`. Defaults to `http`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl Referable for LoadBalancerMonitor {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LoadBalancerMonitor { }

impl ToListMappable for LoadBalancerMonitor {
    type O = ListRef<LoadBalancerMonitorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LoadBalancerMonitor_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_load_balancer_monitor".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoadBalancerMonitor {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
}

impl BuildLoadBalancerMonitor {
    pub fn build(self, stack: &mut Stack) -> LoadBalancerMonitor {
        let out = LoadBalancerMonitor(Rc::new(LoadBalancerMonitor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LoadBalancerMonitorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                allow_insecure: core::default::Default::default(),
                description: core::default::Default::default(),
                expected_body: core::default::Default::default(),
                expected_codes: core::default::Default::default(),
                follow_redirects: core::default::Default::default(),
                id: core::default::Default::default(),
                interval: core::default::Default::default(),
                method: core::default::Default::default(),
                path: core::default::Default::default(),
                port: core::default::Default::default(),
                probe_zone: core::default::Default::default(),
                retries: core::default::Default::default(),
                timeout: core::default::Default::default(),
                type_: core::default::Default::default(),
                header: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoadBalancerMonitorRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerMonitorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoadBalancerMonitorRef {
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

    #[doc= "Get a reference to the value of field `allow_insecure` after provisioning.\nDo not validate the certificate when monitor use HTTPS.  Only valid if `type` is \"http\" or \"https\"."]
    pub fn allow_insecure(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer monitor was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nFree text description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expected_body` after provisioning.\nA case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. Only valid if `type` is \"http\" or \"https\"."]
    pub fn expected_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expected_codes` after provisioning.\nThe expected HTTP response code or code range of the health check. Eg `2xx`. Only valid and required if `type` is \"http\" or \"https\"."]
    pub fn expected_codes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_codes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `follow_redirects` after provisioning.\nFollow redirects if returned by the origin. Only valid if `type` is \"http\" or \"https\"."]
    pub fn follow_redirects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.follow_redirects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nThe interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations. Defaults to `60`."]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `method` after provisioning.\nThe method to use for the health check."]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nThe RFC3339 timestamp of when the load balancer monitor was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe endpoint path to health check against."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nThe port number to use for the healthcheck, required when creating a TCP monitor."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `probe_zone` after provisioning.\nAssign this monitor to emulate the specified zone while probing. Only valid if `type` is \"http\" or \"https\"."]
    pub fn probe_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.probe_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retries` after provisioning.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`."]
    pub fn retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retries", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) before marking the health check as failed. Defaults to `5`."]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe protocol to use for the healthcheck. Available values: `http`, `https`, `tcp`, `udp_icmp`, `icmp_ping`, `smtp`. Defaults to `http`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LoadBalancerMonitorHeaderEl {
    header: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl LoadBalancerMonitorHeaderEl { }

impl ToListMappable for LoadBalancerMonitorHeaderEl {
    type O = BlockAssignable<LoadBalancerMonitorHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadBalancerMonitorHeaderEl {
    #[doc= "The header name."]
    pub header: PrimField<String>,
    #[doc= "A list of values for the header."]
    pub values: SetField<PrimField<String>>,
}

impl BuildLoadBalancerMonitorHeaderEl {
    pub fn build(self) -> LoadBalancerMonitorHeaderEl {
        LoadBalancerMonitorHeaderEl {
            header: self.header,
            values: self.values,
        }
    }
}

pub struct LoadBalancerMonitorHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadBalancerMonitorHeaderElRef {
    fn new(shared: StackShared, base: String) -> LoadBalancerMonitorHeaderElRef {
        LoadBalancerMonitorHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadBalancerMonitorHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header` after provisioning.\nThe header name."]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\nA list of values for the header."]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoadBalancerMonitorDynamic {
    header: Option<DynamicBlock<LoadBalancerMonitorHeaderEl>>,
}
