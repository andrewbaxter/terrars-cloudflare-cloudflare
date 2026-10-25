#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustGatewayLoggingData_ {
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
    redact_pii: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_by_rule_type: std::option::Option<ZeroTrustGatewayLogging_SettingsByRuleType>,
}

struct ZeroTrustGatewayLogging_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustGatewayLoggingData_>,
}

#[derive(Clone)]
pub struct ZeroTrustGatewayLogging(std::rc::Rc<ZeroTrustGatewayLogging_>);

impl ZeroTrustGatewayLogging {
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
        "Set the field `redact_pii`.\nIndicate whether to redact personally identifiable information from activity logging (PII fields include source IP, user email, user ID, device ID, URL, referrer, and user agent)."]
    pub fn set_redact_pii(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().redact_pii = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `settings_by_rule_type`.\nConfigure logging settings for each rule type."]
    pub fn set_settings_by_rule_type(
        self,
        v: impl std::convert::Into<ZeroTrustGatewayLogging_SettingsByRuleType>,
    ) -> Self {
        self.0.data.borrow_mut().settings_by_rule_type = std::option::Option::Some(v.into());
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
    pub fn settings_by_rule_type(&self) -> ZeroTrustGatewayLogging_SettingsByRuleTypeRef {
        ZeroTrustGatewayLogging_SettingsByRuleTypeRef::new(
            self.shared().clone(),
            format!("{}.settings_by_rule_type", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ZeroTrustGatewayLogging {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustGatewayLogging { }

impl terrars::ToListMappable for ZeroTrustGatewayLogging {
    type O = terrars::ListRef<ZeroTrustGatewayLoggingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustGatewayLogging_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_gateway_logging".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustGatewayLogging {
    pub tf_id: std::string::String,
    #[doc = "Specify the Cloudflare account identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustGatewayLogging {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustGatewayLogging {
        let out = ZeroTrustGatewayLogging(std::rc::Rc::new(ZeroTrustGatewayLogging_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustGatewayLoggingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                redact_pii: core::default::Default::default(),
                settings_by_rule_type: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustGatewayLoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayLoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustGatewayLoggingRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn settings_by_rule_type(&self) -> ZeroTrustGatewayLogging_SettingsByRuleTypeRef {
        ZeroTrustGatewayLogging_SettingsByRuleTypeRef::new(
            self.shared().clone(),
            format!("{}.settings_by_rule_type", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewayLogging_SettingsByRuleType_Dns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_blocks: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewayLogging_SettingsByRuleType_Dns {
    #[doc = "Set the field `log_all`.\nSpecify whether to log all requests to this service."]
    pub fn set_log_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_blocks`.\nSpecify whether to log only blocking requests to this service."]
    pub fn set_log_blocks(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_blocks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayLogging_SettingsByRuleType_Dns {
    type O = terrars::BlockAssignable<ZeroTrustGatewayLogging_SettingsByRuleType_Dns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayLogging_SettingsByRuleType_Dns {}

impl BuildZeroTrustGatewayLogging_SettingsByRuleType_Dns {
    pub fn build(self) -> ZeroTrustGatewayLogging_SettingsByRuleType_Dns {
        ZeroTrustGatewayLogging_SettingsByRuleType_Dns {
            log_all: core::default::Default::default(),
            log_blocks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
        ZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
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
pub struct ZeroTrustGatewayLogging_SettingsByRuleType_Http {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_blocks: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewayLogging_SettingsByRuleType_Http {
    #[doc = "Set the field `log_all`.\nSpecify whether to log all requests to this service."]
    pub fn set_log_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_blocks`.\nSpecify whether to log only blocking requests to this service."]
    pub fn set_log_blocks(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_blocks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayLogging_SettingsByRuleType_Http {
    type O = terrars::BlockAssignable<ZeroTrustGatewayLogging_SettingsByRuleType_Http>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayLogging_SettingsByRuleType_Http {}

impl BuildZeroTrustGatewayLogging_SettingsByRuleType_Http {
    pub fn build(self) -> ZeroTrustGatewayLogging_SettingsByRuleType_Http {
        ZeroTrustGatewayLogging_SettingsByRuleType_Http {
            log_all: core::default::Default::default(),
            log_blocks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
        ZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
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
pub struct ZeroTrustGatewayLogging_SettingsByRuleType_L4 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_blocks: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewayLogging_SettingsByRuleType_L4 {
    #[doc = "Set the field `log_all`.\nSpecify whether to log all requests to this service."]
    pub fn set_log_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_blocks`.\nSpecify whether to log only blocking requests to this service."]
    pub fn set_log_blocks(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_blocks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayLogging_SettingsByRuleType_L4 {
    type O = terrars::BlockAssignable<ZeroTrustGatewayLogging_SettingsByRuleType_L4>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayLogging_SettingsByRuleType_L4 {}

impl BuildZeroTrustGatewayLogging_SettingsByRuleType_L4 {
    pub fn build(self) -> ZeroTrustGatewayLogging_SettingsByRuleType_L4 {
        ZeroTrustGatewayLogging_SettingsByRuleType_L4 {
            log_all: core::default::Default::default(),
            log_blocks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
        ZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
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
pub struct ZeroTrustGatewayLogging_SettingsByRuleType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns: std::option::Option<ZeroTrustGatewayLogging_SettingsByRuleType_Dns>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<ZeroTrustGatewayLogging_SettingsByRuleType_Http>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    l4: std::option::Option<ZeroTrustGatewayLogging_SettingsByRuleType_L4>,
}

impl ZeroTrustGatewayLogging_SettingsByRuleType {
    #[doc = "Set the field `dns`.\nConfigure logging settings for DNS firewall."]
    pub fn set_dns(mut self, v: impl std::convert::Into<ZeroTrustGatewayLogging_SettingsByRuleType_Dns>) -> Self {
        self.dns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http`.\nConfigure logging settings for HTTP/HTTPS firewall."]
    pub fn set_http(mut self, v: impl std::convert::Into<ZeroTrustGatewayLogging_SettingsByRuleType_Http>) -> Self {
        self.http = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `l4`.\nConfigure logging settings for Network firewall."]
    pub fn set_l4(mut self, v: impl std::convert::Into<ZeroTrustGatewayLogging_SettingsByRuleType_L4>) -> Self {
        self.l4 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewayLogging_SettingsByRuleType {
    type O = terrars::BlockAssignable<ZeroTrustGatewayLogging_SettingsByRuleType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewayLogging_SettingsByRuleType {}

impl BuildZeroTrustGatewayLogging_SettingsByRuleType {
    pub fn build(self) -> ZeroTrustGatewayLogging_SettingsByRuleType {
        ZeroTrustGatewayLogging_SettingsByRuleType {
            dns: core::default::Default::default(),
            http: core::default::Default::default(),
            l4: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewayLogging_SettingsByRuleTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewayLogging_SettingsByRuleTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustGatewayLogging_SettingsByRuleTypeRef {
        ZeroTrustGatewayLogging_SettingsByRuleTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewayLogging_SettingsByRuleTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dns` after provisioning.\nConfigure logging settings for DNS firewall."]
    pub fn dns(&self) -> ZeroTrustGatewayLogging_SettingsByRuleType_DnsRef {
        ZeroTrustGatewayLogging_SettingsByRuleType_DnsRef::new(self.shared().clone(), format!("{}.dns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http` after provisioning.\nConfigure logging settings for HTTP/HTTPS firewall."]
    pub fn http(&self) -> ZeroTrustGatewayLogging_SettingsByRuleType_HttpRef {
        ZeroTrustGatewayLogging_SettingsByRuleType_HttpRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc =
        "Get a reference to the value of field `l4` after provisioning.\nConfigure logging settings for Network firewall."]
    pub fn l4(&self) -> ZeroTrustGatewayLogging_SettingsByRuleType_L4Ref {
        ZeroTrustGatewayLogging_SettingsByRuleType_L4Ref::new(self.shared().clone(), format!("{}.l4", self.base))
    }
}
