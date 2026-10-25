#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDlpSettingsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustDlpSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDlpSettingsData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDlpSettings(std::rc::Rc<DataZeroTrustDlpSettings_>);

impl DataZeroTrustDlpSettings {
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

    #[doc = "Get a reference to the value of field `payload_logging` after provisioning.\n"]
    pub fn payload_logging(&self) -> DataZeroTrustDlpSettings_PayloadLoggingRef {
        DataZeroTrustDlpSettings_PayloadLoggingRef::new(
            self.shared().clone(),
            format!("{}.payload_logging", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataZeroTrustDlpSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDlpSettings { }

impl terrars::ToListMappable for DataZeroTrustDlpSettings {
    type O = terrars::ListRef<DataZeroTrustDlpSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDlpSettings_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dlp_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDlpSettings {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustDlpSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDlpSettings {
        let out = DataZeroTrustDlpSettings(std::rc::Rc::new(DataZeroTrustDlpSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDlpSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDlpSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDlpSettingsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `payload_logging` after provisioning.\n"]
    pub fn payload_logging(&self) -> DataZeroTrustDlpSettings_PayloadLoggingRef {
        DataZeroTrustDlpSettings_PayloadLoggingRef::new(
            self.shared().clone(),
            format!("{}.payload_logging", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDlpSettings_PayloadLogging {}

impl DataZeroTrustDlpSettings_PayloadLogging { }

impl terrars::ToListMappable for DataZeroTrustDlpSettings_PayloadLogging {
    type O = terrars::BlockAssignable<DataZeroTrustDlpSettings_PayloadLogging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpSettings_PayloadLogging {}

impl BuildDataZeroTrustDlpSettings_PayloadLogging {
    pub fn build(self) -> DataZeroTrustDlpSettings_PayloadLogging {
        DataZeroTrustDlpSettings_PayloadLogging {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpSettings_PayloadLoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpSettings_PayloadLoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDlpSettings_PayloadLoggingRef {
        DataZeroTrustDlpSettings_PayloadLoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpSettings_PayloadLoggingRef {
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
        "Get a reference to the value of field `public_key` after provisioning.\nBase64-encoded public key for encrypting payload logs. Null when payload logging is disabled."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
