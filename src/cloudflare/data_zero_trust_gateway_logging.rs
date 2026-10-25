#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustGatewayLoggingData_ {
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
}

struct DataZeroTrustGatewayLogging_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustGatewayLoggingData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustGatewayLogging(std::rc::Rc<DataZeroTrustGatewayLogging_>);

impl DataZeroTrustGatewayLogging {
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

    #[doc = "Set the field `account_id`.\nSpecify the Cloudflare account identifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redact_pii` after provisioning.\nIndicate whether to redact personally identifiable information from activity logging (PII fields include source IP, user email, user ID, device ID, URL, referrer, and user agent)."]
    pub fn redact_pii(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redact_pii", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `settings_by_rule_type` after provisioning.\nConfigure logging settings for each rule type."]
    pub fn settings_by_rule_type(&self) -> DataZeroTrustGatewayLogging_SettingsByRuleTypeRef {
        DataZeroTrustGatewayLogging_SettingsByRuleTypeRef::new(
            self.shared().clone(),
            format!("{}.settings_by_rule_type", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataZeroTrustGatewayLogging {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustGatewayLogging { }

impl terrars::ToListMappable for DataZeroTrustGatewayLogging {
    type O = terrars::ListRef<DataZeroTrustGatewayLoggingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustGatewayLogging_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_gateway_logging".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustGatewayLogging {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustGatewayLogging {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustGatewayLogging {
        let out = DataZeroTrustGatewayLogging(std::rc::Rc::new(DataZeroTrustGatewayLogging_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustGatewayLoggingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustGatewayLoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayLoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustGatewayLoggingRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redact_pii` after provisioning.\nIndicate whether to redact personally identifiable information from activity logging (PII fields include source IP, user email, user ID, device ID, URL, referrer, and user agent)."]
    pub fn redact_pii(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redact_pii", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `settings_by_rule_type` after provisioning.\nConfigure logging settings for each rule type."]
    pub fn settings_by_rule_type(&self) -> DataZeroTrustGatewayLogging_SettingsByRuleTypeRef {
        DataZeroTrustGatewayLogging_SettingsByRuleTypeRef::new(
            self.shared().clone(),
            format!("{}.settings_by_rule_type", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewayLogging_SettingsByRuleType_Dns {}

impl DataZeroTrustGatewayLogging_SettingsByRuleType_Dns { }

impl terrars::ToListMappable for DataZeroTrustGatewayLogging_SettingsByRuleType_Dns {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayLogging_SettingsByRuleType_Dns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayLogging_SettingsByRuleType_Dns {}

impl BuildDataZeroTrustGatewayLogging_SettingsByRuleType_Dns {
    pub fn build(self) -> DataZeroTrustGatewayLogging_SettingsByRuleType_Dns {
        DataZeroTrustGatewayLogging_SettingsByRuleType_Dns {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
        DataZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `log_all` after provisioning.\nSpecify whether to log all requests to this service."]
    pub fn log_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_blocks` after provisioning.\nSpecify whether to log only blocking requests to this service."]
    pub fn log_blocks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_blocks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewayLogging_SettingsByRuleType_Http {}

impl DataZeroTrustGatewayLogging_SettingsByRuleType_Http { }

impl terrars::ToListMappable for DataZeroTrustGatewayLogging_SettingsByRuleType_Http {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayLogging_SettingsByRuleType_Http>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayLogging_SettingsByRuleType_Http {}

impl BuildDataZeroTrustGatewayLogging_SettingsByRuleType_Http {
    pub fn build(self) -> DataZeroTrustGatewayLogging_SettingsByRuleType_Http {
        DataZeroTrustGatewayLogging_SettingsByRuleType_Http {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
        DataZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `log_all` after provisioning.\nSpecify whether to log all requests to this service."]
    pub fn log_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_blocks` after provisioning.\nSpecify whether to log only blocking requests to this service."]
    pub fn log_blocks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_blocks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewayLogging_SettingsByRuleType_L4 {}

impl DataZeroTrustGatewayLogging_SettingsByRuleType_L4 { }

impl terrars::ToListMappable for DataZeroTrustGatewayLogging_SettingsByRuleType_L4 {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayLogging_SettingsByRuleType_L4>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayLogging_SettingsByRuleType_L4 {}

impl BuildDataZeroTrustGatewayLogging_SettingsByRuleType_L4 {
    pub fn build(self) -> DataZeroTrustGatewayLogging_SettingsByRuleType_L4 {
        DataZeroTrustGatewayLogging_SettingsByRuleType_L4 {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
        DataZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `log_all` after provisioning.\nSpecify whether to log all requests to this service."]
    pub fn log_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_blocks` after provisioning.\nSpecify whether to log only blocking requests to this service."]
    pub fn log_blocks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_blocks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewayLogging_SettingsByRuleType {}

impl DataZeroTrustGatewayLogging_SettingsByRuleType { }

impl terrars::ToListMappable for DataZeroTrustGatewayLogging_SettingsByRuleType {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayLogging_SettingsByRuleType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayLogging_SettingsByRuleType {}

impl BuildDataZeroTrustGatewayLogging_SettingsByRuleType {
    pub fn build(self) -> DataZeroTrustGatewayLogging_SettingsByRuleType {
        DataZeroTrustGatewayLogging_SettingsByRuleType {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayLogging_SettingsByRuleTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayLogging_SettingsByRuleTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayLogging_SettingsByRuleTypeRef {
        DataZeroTrustGatewayLogging_SettingsByRuleTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayLogging_SettingsByRuleTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dns` after provisioning.\nConfigure logging settings for DNS firewall."]
    pub fn dns(&self) -> DataZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
        DataZeroTrustGatewayLogging_SettingsByRuleType_DnsRef::new(
            self.shared().clone(),
            format!("{}.dns", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `http` after provisioning.\nConfigure logging settings for HTTP/HTTPS firewall."]
    pub fn http(&self) -> DataZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
        DataZeroTrustGatewayLogging_SettingsByRuleType_HttpRef::new(
            self.shared().clone(),
            format!("{}.http", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `l4` after provisioning.\nConfigure logging settings for Network firewall."]
    pub fn l4(&self) -> DataZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
        DataZeroTrustGatewayLogging_SettingsByRuleType_L4Ref::new(self.shared().clone(), format!("{}.l4", self.base))
    }
}
