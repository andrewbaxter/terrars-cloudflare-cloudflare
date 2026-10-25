#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustCasbWebhookData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    webhook_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustCasbWebhook_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustCasbWebhookData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustCasbWebhook(std::rc::Rc<DataZeroTrustCasbWebhook_>);

impl DataZeroTrustCasbWebhook {
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
        "Get a reference to the value of field `authentication_type` after provisioning.\nType of authentication used for the webhook.\nAvailable values: \"Basic Auth\", \"None\", \"Bearer Auth\", \"Static Headers\", \"HMAC-Signing\"."]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the webhook configuration was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destination_url` after provisioning.\nTarget URL for the webhook configuration. Where resulting data will be sent."]
    pub fn destination_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nList of header keys configured for this webhook. Values are not included for security reasons."]
    pub fn headers(&self) -> terrars::ListRef<DataZeroTrustCasbWebhook_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nAccount-specified display label for the webhook configuration."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nCurrent status of the webhook configuration. If disabled, data cannot be sent through this configuration.\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the webhook configuration was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion number of the configuration."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webhook_id` after provisioning.\n"]
    pub fn webhook_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustCasbWebhook {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustCasbWebhook { }

impl terrars::ToListMappable for DataZeroTrustCasbWebhook {
    type O = terrars::ListRef<DataZeroTrustCasbWebhookRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustCasbWebhook_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_casb_webhook".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustCasbWebhook {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub webhook_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustCasbWebhook {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustCasbWebhook {
        let out = DataZeroTrustCasbWebhook(std::rc::Rc::new(DataZeroTrustCasbWebhook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustCasbWebhookData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                webhook_id: self.webhook_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustCasbWebhookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbWebhookRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustCasbWebhookRef {
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
        "Get a reference to the value of field `authentication_type` after provisioning.\nType of authentication used for the webhook.\nAvailable values: \"Basic Auth\", \"None\", \"Bearer Auth\", \"Static Headers\", \"HMAC-Signing\"."]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the webhook configuration was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destination_url` after provisioning.\nTarget URL for the webhook configuration. Where resulting data will be sent."]
    pub fn destination_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nList of header keys configured for this webhook. Values are not included for security reasons."]
    pub fn headers(&self) -> terrars::ListRef<DataZeroTrustCasbWebhook_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nAccount-specified display label for the webhook configuration."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nCurrent status of the webhook configuration. If disabled, data cannot be sent through this configuration.\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the webhook configuration was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion number of the configuration."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webhook_id` after provisioning.\n"]
    pub fn webhook_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustCasbWebhook_Headers {}

impl DataZeroTrustCasbWebhook_Headers { }

impl terrars::ToListMappable for DataZeroTrustCasbWebhook_Headers {
    type O = terrars::BlockAssignable<DataZeroTrustCasbWebhook_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustCasbWebhook_Headers {}

impl BuildDataZeroTrustCasbWebhook_Headers {
    pub fn build(self) -> DataZeroTrustCasbWebhook_Headers {
        DataZeroTrustCasbWebhook_Headers {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustCasbWebhook_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustCasbWebhook_HeadersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustCasbWebhook_HeadersRef {
        DataZeroTrustCasbWebhook_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustCasbWebhook_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nHeader key name (lowercase)."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nHeader value. This field is never returned in API responses for security reasons."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
