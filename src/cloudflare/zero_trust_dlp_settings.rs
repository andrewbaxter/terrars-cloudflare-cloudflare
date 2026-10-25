#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDlpSettingsData_ {
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
    ai_context_analysis: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload_logging: std::option::Option<ZeroTrustDlpSettings_PayloadLogging>,
}

struct ZeroTrustDlpSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDlpSettingsData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDlpSettings(std::rc::Rc<ZeroTrustDlpSettings_>);

impl ZeroTrustDlpSettings {
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

    #[doc = "Set the field `ai_context_analysis`.\nWhether AI context analysis is enabled at the account level."]
    pub fn set_ai_context_analysis(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ai_context_analysis = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocr`.\nWhether OCR is enabled at the account level."]
    pub fn set_ocr(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ocr = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payload_logging`.\nRequest model for payload log settings within the DLP settings endpoint.\nUnlike the legacy endpoint, null and missing are treated identically here\n(both mean \"not provided\" for PATCH, \"reset to default\" for PUT)."]
    pub fn set_payload_logging(self, v: impl std::convert::Into<ZeroTrustDlpSettings_PayloadLogging>) -> Self {
        self.0.data.borrow_mut().payload_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ai_context_analysis` after provisioning.\nWhether AI context analysis is enabled at the account level."]
    pub fn ai_context_analysis(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_context_analysis", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocr` after provisioning.\nWhether OCR is enabled at the account level."]
    pub fn ocr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payload_logging` after provisioning.\nRequest model for payload log settings within the DLP settings endpoint.\nUnlike the legacy endpoint, null and missing are treated identically here\n(both mean \"not provided\" for PATCH, \"reset to default\" for PUT)."]
    pub fn payload_logging(&self) -> ZeroTrustDlpSettings_PayloadLoggingRef {
        ZeroTrustDlpSettings_PayloadLoggingRef::new(
            self.shared().clone(),
            format!("{}.payload_logging", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ZeroTrustDlpSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDlpSettings { }

impl terrars::ToListMappable for ZeroTrustDlpSettings {
    type O = terrars::ListRef<ZeroTrustDlpSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDlpSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dlp_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDlpSettings {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDlpSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDlpSettings {
        let out = ZeroTrustDlpSettings(std::rc::Rc::new(ZeroTrustDlpSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDlpSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                ai_context_analysis: core::default::Default::default(),
                ocr: core::default::Default::default(),
                payload_logging: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDlpSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDlpSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ai_context_analysis` after provisioning.\nWhether AI context analysis is enabled at the account level."]
    pub fn ai_context_analysis(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_context_analysis", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocr` after provisioning.\nWhether OCR is enabled at the account level."]
    pub fn ocr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payload_logging` after provisioning.\nRequest model for payload log settings within the DLP settings endpoint.\nUnlike the legacy endpoint, null and missing are treated identically here\n(both mean \"not provided\" for PATCH, \"reset to default\" for PUT)."]
    pub fn payload_logging(&self) -> ZeroTrustDlpSettings_PayloadLoggingRef {
        ZeroTrustDlpSettings_PayloadLoggingRef::new(
            self.shared().clone(),
            format!("{}.payload_logging", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpSettings_PayloadLogging {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    masking_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustDlpSettings_PayloadLogging {
    #[doc =
        "Set the field `masking_level`.\nMasking level for payload logs.\n\n- `full`: The entire payload is masked.\n- `partial`: Only partial payload content is masked.\n- `clear`: No masking is applied to the payload content.\n- `default`: DLP uses its default masking behavior.\nAvailable values: \"full\", \"partial\", \"clear\", \"default\"."]
    pub fn set_masking_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.masking_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `public_key`.\nBase64-encoded public key for encrypting payload logs.\n\n- Set to a non-empty base64 string to enable payload logging with the given key.\n- Set to an empty string to disable payload logging.\n- Omit or set to null to leave unchanged (PATCH) or reset to disabled (PUT)."]
    pub fn set_public_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDlpSettings_PayloadLogging {
    type O = terrars::BlockAssignable<ZeroTrustDlpSettings_PayloadLogging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpSettings_PayloadLogging {}

impl BuildZeroTrustDlpSettings_PayloadLogging {
    pub fn build(self) -> ZeroTrustDlpSettings_PayloadLogging {
        ZeroTrustDlpSettings_PayloadLogging {
            masking_level: core::default::Default::default(),
            public_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpSettings_PayloadLoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpSettings_PayloadLoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpSettings_PayloadLoggingRef {
        ZeroTrustDlpSettings_PayloadLoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpSettings_PayloadLoggingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `masking_level` after provisioning.\nMasking level for payload logs.\n\n- `full`: The entire payload is masked.\n- `partial`: Only partial payload content is masked.\n- `clear`: No masking is applied to the payload content.\n- `default`: DLP uses its default masking behavior.\nAvailable values: \"full\", \"partial\", \"clear\", \"default\"."]
    pub fn masking_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.masking_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_key` after provisioning.\nBase64-encoded public key for encrypting payload logs.\n\n- Set to a non-empty base64 string to enable payload logging with the given key.\n- Set to an empty string to disable payload logging.\n- Omit or set to null to leave unchanged (PATCH) or reset to disabled (PUT)."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}
