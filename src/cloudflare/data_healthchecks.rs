#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataHealthchecksData_ {
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

struct DataHealthchecks_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataHealthchecksData_>,
}

#[derive(Clone)]
pub struct DataHealthchecks(std::rc::Rc<DataHealthchecks_>);

impl DataHealthchecks {
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

    #[doc = "Set the field `zone_id`.\nIdentifier"]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataHealthchecks_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataHealthchecks {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataHealthchecks { }

impl terrars::ToListMappable for DataHealthchecks {
    type O = terrars::ListRef<DataHealthchecksRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataHealthchecks_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_healthchecks".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataHealthchecks {
    pub tf_id: std::string::String,
}

impl BuildDataHealthchecks {
    pub fn build(self, stack: &mut terrars::Stack) -> DataHealthchecks {
        let out = DataHealthchecks(std::rc::Rc::new(DataHealthchecks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataHealthchecksData_ {
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

pub struct DataHealthchecksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataHealthchecksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataHealthchecksRef {
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
    pub fn result(&self) -> terrars::ListRef<DataHealthchecks_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataHealthchecks_Result_HttpConfig {}

impl DataHealthchecks_Result_HttpConfig { }

impl terrars::ToListMappable for DataHealthchecks_Result_HttpConfig {
    type O = terrars::BlockAssignable<DataHealthchecks_Result_HttpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataHealthchecks_Result_HttpConfig {}

impl BuildDataHealthchecks_Result_HttpConfig {
    pub fn build(self) -> DataHealthchecks_Result_HttpConfig {
        DataHealthchecks_Result_HttpConfig {}
    }
}

#[allow(dead_code)]
pub struct DataHealthchecks_Result_HttpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataHealthchecks_Result_HttpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataHealthchecks_Result_HttpConfigRef {
        DataHealthchecks_Result_HttpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataHealthchecks_Result_HttpConfigRef {
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
pub struct DataHealthchecks_Result_TcpConfig {}

impl DataHealthchecks_Result_TcpConfig { }

impl terrars::ToListMappable for DataHealthchecks_Result_TcpConfig {
    type O = terrars::BlockAssignable<DataHealthchecks_Result_TcpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataHealthchecks_Result_TcpConfig {}

impl BuildDataHealthchecks_Result_TcpConfig {
    pub fn build(self) -> DataHealthchecks_Result_TcpConfig {
        DataHealthchecks_Result_TcpConfig {}
    }
}

#[allow(dead_code)]
pub struct DataHealthchecks_Result_TcpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataHealthchecks_Result_TcpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataHealthchecks_Result_TcpConfigRef {
        DataHealthchecks_Result_TcpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataHealthchecks_Result_TcpConfigRef {
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

#[derive(Serialize)]
pub struct DataHealthchecks_Result {}

impl DataHealthchecks_Result { }

impl terrars::ToListMappable for DataHealthchecks_Result {
    type O = terrars::BlockAssignable<DataHealthchecks_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataHealthchecks_Result {}

impl BuildDataHealthchecks_Result {
    pub fn build(self) -> DataHealthchecks_Result {
        DataHealthchecks_Result {}
    }
}

#[allow(dead_code)]
pub struct DataHealthchecks_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataHealthchecks_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataHealthchecks_ResultRef {
        DataHealthchecks_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataHealthchecks_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nThe hostname or IP address of the origin server to run health checks on."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `check_regions` after provisioning.\nA list of regions from which to run health checks. Null means Cloudflare will pick a default region."]
    pub fn check_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_regions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_fails` after provisioning.\nThe number of consecutive fails required from a health check before changing the health to unhealthy."]
    pub fn consecutive_fails(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_fails", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consecutive_successes` after provisioning.\nThe number of consecutive successes required from a health check before changing the health to healthy."]
    pub fn consecutive_successes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consecutive_successes", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the health check."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `failure_reason` after provisioning.\nThe current failure reason if status is unhealthy."]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_config` after provisioning.\nParameters specific to an HTTP or HTTPS health check."]
    pub fn http_config(&self) -> DataHealthchecks_Result_HttpConfigRef {
        DataHealthchecks_Result_HttpConfigRef::new(self.shared().clone(), format!("{}.http_config", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nThe interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase load on the origin as we check from multiple locations."]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA short name to identify the health check. Only alphanumeric characters, hyphens and underscores are allowed."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retries` after provisioning.\nThe number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately."]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe current status of the origin server according to the health check.\nAvailable values: \"unknown\", \"healthy\", \"unhealthy\", \"suspended\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nIf suspended, no health checks are sent to the origin."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tcp_config` after provisioning.\nParameters specific to TCP health check."]
    pub fn tcp_config(&self) -> DataHealthchecks_Result_TcpConfigRef {
        DataHealthchecks_Result_TcpConfigRef::new(self.shared().clone(), format!("{}.tcp_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) before marking the health check as failed."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe protocol to use for the health check. Currently supported protocols are 'HTTP', 'HTTPS' and 'TCP'."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
