use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct HealthcheckData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_insecure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_regions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consecutive_fails: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consecutive_successes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    follow_redirects: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspended: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<HealthcheckHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<HealthcheckTimeoutsEl>,
    dynamic: HealthcheckDynamic,
}

struct Healthcheck_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<HealthcheckData>,
}

#[derive(Clone)]
pub struct Healthcheck(Rc<Healthcheck_>);

impl Healthcheck {
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

    #[doc= "Set the field `allow_insecure`.\nDo not validate the certificate when the health check uses HTTPS. Defaults to `false`."]
    pub fn set_allow_insecure(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_insecure = Some(v.into());
        self
    }

    #[doc= "Set the field `check_regions`.\nA list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`."]
    pub fn set_check_regions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().check_regions = Some(v.into());
        self
    }

    #[doc= "Set the field `consecutive_fails`.\nThe number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`."]
    pub fn set_consecutive_fails(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().consecutive_fails = Some(v.into());
        self
    }

    #[doc= "Set the field `consecutive_successes`.\nThe number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`."]
    pub fn set_consecutive_successes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().consecutive_successes = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA human-readable description of the health check."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `expected_body`.\nA case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy."]
    pub fn set_expected_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_body = Some(v.into());
        self
    }

    #[doc= "Set the field `expected_codes`.\nThe expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check."]
    pub fn set_expected_codes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().expected_codes = Some(v.into());
        self
    }

    #[doc= "Set the field `follow_redirects`.\nFollow redirects if the origin returns a 3xx status code. Defaults to `false`."]
    pub fn set_follow_redirects(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().follow_redirects = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `interval`.\nThe interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`."]
    pub fn set_interval(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().interval = Some(v.into());
        self
    }

    #[doc= "Set the field `method`.\nThe HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`."]
    pub fn set_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().method = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\nThe endpoint path to health check against. Defaults to `/`."]
    pub fn set_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().path = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\nPort number to connect to for the health check. Defaults to `80`."]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }

    #[doc= "Set the field `retries`.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`."]
    pub fn set_retries(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retries = Some(v.into());
        self
    }

    #[doc= "Set the field `suspended`.\nIf suspended, no health checks are sent to the origin. Defaults to `false`."]
    pub fn set_suspended(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout`.\nThe timeout (in seconds) before marking the health check as failed. Defaults to `5`."]
    pub fn set_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `header`.\n"]
    pub fn set_header(self, v: impl Into<BlockAssignable<HealthcheckHeaderEl>>) -> Self {
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

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<HealthcheckTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\nThe hostname or IP address of the origin server to run health checks on."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_insecure` after provisioning.\nDo not validate the certificate when the health check uses HTTPS. Defaults to `false`."]
    pub fn allow_insecure(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `check_regions` after provisioning.\nA list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`."]
    pub fn check_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.check_regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `consecutive_fails` after provisioning.\nThe number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`."]
    pub fn consecutive_fails(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.consecutive_fails", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `consecutive_successes` after provisioning.\nThe number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`."]
    pub fn consecutive_successes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.consecutive_successes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nCreation time."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the health check."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expected_body` after provisioning.\nA case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy."]
    pub fn expected_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expected_codes` after provisioning.\nThe expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check."]
    pub fn expected_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.expected_codes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `follow_redirects` after provisioning.\nFollow redirects if the origin returns a 3xx status code. Defaults to `false`."]
    pub fn follow_redirects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.follow_redirects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nThe interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`."]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `method` after provisioning.\nThe HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`."]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nLast modified time."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe endpoint path to health check against. Defaults to `/`."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nPort number to connect to for the health check. Defaults to `80`."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retries` after provisioning.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`."]
    pub fn retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retries", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suspended` after provisioning.\nIf suspended, no health checks are sent to the origin. Defaults to `false`."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) before marking the health check as failed. Defaults to `5`."]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcheckTimeoutsElRef {
        HealthcheckTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for Healthcheck {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Healthcheck { }

impl ToListMappable for Healthcheck {
    type O = ListRef<HealthcheckRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Healthcheck_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_healthcheck".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildHealthcheck {
    pub tf_id: String,
    #[doc= "The hostname or IP address of the origin server to run health checks on."]
    pub address: PrimField<String>,
    #[doc= "A short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed."]
    pub name: PrimField<String>,
    #[doc= "The protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`."]
    pub type_: PrimField<String>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildHealthcheck {
    pub fn build(self, stack: &mut Stack) -> Healthcheck {
        let out = Healthcheck(Rc::new(Healthcheck_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(HealthcheckData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                address: self.address,
                allow_insecure: core::default::Default::default(),
                check_regions: core::default::Default::default(),
                consecutive_fails: core::default::Default::default(),
                consecutive_successes: core::default::Default::default(),
                description: core::default::Default::default(),
                expected_body: core::default::Default::default(),
                expected_codes: core::default::Default::default(),
                follow_redirects: core::default::Default::default(),
                id: core::default::Default::default(),
                interval: core::default::Default::default(),
                method: core::default::Default::default(),
                name: self.name,
                path: core::default::Default::default(),
                port: core::default::Default::default(),
                retries: core::default::Default::default(),
                suspended: core::default::Default::default(),
                timeout: core::default::Default::default(),
                type_: self.type_,
                zone_id: self.zone_id,
                header: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct HealthcheckRef {
    shared: StackShared,
    base: String,
}

impl Ref for HealthcheckRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl HealthcheckRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\nThe hostname or IP address of the origin server to run health checks on."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_insecure` after provisioning.\nDo not validate the certificate when the health check uses HTTPS. Defaults to `false`."]
    pub fn allow_insecure(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `check_regions` after provisioning.\nA list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`."]
    pub fn check_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.check_regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `consecutive_fails` after provisioning.\nThe number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`."]
    pub fn consecutive_fails(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.consecutive_fails", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `consecutive_successes` after provisioning.\nThe number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`."]
    pub fn consecutive_successes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.consecutive_successes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nCreation time."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the health check."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expected_body` after provisioning.\nA case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy."]
    pub fn expected_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expected_codes` after provisioning.\nThe expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check."]
    pub fn expected_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.expected_codes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `follow_redirects` after provisioning.\nFollow redirects if the origin returns a 3xx status code. Defaults to `false`."]
    pub fn follow_redirects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.follow_redirects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nThe interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`."]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `method` after provisioning.\nThe HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`."]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nLast modified time."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe endpoint path to health check against. Defaults to `/`."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nPort number to connect to for the health check. Defaults to `80`."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retries` after provisioning.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`."]
    pub fn retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retries", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suspended` after provisioning.\nIf suspended, no health checks are sent to the origin. Defaults to `false`."]
    pub fn suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) before marking the health check as failed. Defaults to `5`."]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcheckTimeoutsElRef {
        HealthcheckTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct HealthcheckHeaderEl {
    header: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl HealthcheckHeaderEl { }

impl ToListMappable for HealthcheckHeaderEl {
    type O = BlockAssignable<HealthcheckHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcheckHeaderEl {
    #[doc= "The header name."]
    pub header: PrimField<String>,
    #[doc= "A list of string values for the header."]
    pub values: SetField<PrimField<String>>,
}

impl BuildHealthcheckHeaderEl {
    pub fn build(self) -> HealthcheckHeaderEl {
        HealthcheckHeaderEl {
            header: self.header,
            values: self.values,
        }
    }
}

pub struct HealthcheckHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for HealthcheckHeaderElRef {
    fn new(shared: StackShared, base: String) -> HealthcheckHeaderElRef {
        HealthcheckHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcheckHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header` after provisioning.\nThe header name."]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\nA list of string values for the header."]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct HealthcheckTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl HealthcheckTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for HealthcheckTimeoutsEl {
    type O = BlockAssignable<HealthcheckTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcheckTimeoutsEl {}

impl BuildHealthcheckTimeoutsEl {
    pub fn build(self) -> HealthcheckTimeoutsEl {
        HealthcheckTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct HealthcheckTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for HealthcheckTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> HealthcheckTimeoutsElRef {
        HealthcheckTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcheckTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct HealthcheckDynamic {
    header: Option<DynamicBlock<HealthcheckHeaderEl>>,
}
