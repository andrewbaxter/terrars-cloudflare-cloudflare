#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AiGatewayData_ {
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
    authentication: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    byok_only: std::option::Option<terrars::PrimField<bool>>,
    cache_invalidate_on_update: terrars::PrimField<bool>,
    cache_ttl: terrars::PrimField<f64>,
    collect_logs: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dlp: std::option::Option<AiGateway_Dlp>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrails: std::option::Option<AiGateway_Guardrails>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_classification: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_management: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_management_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logpush: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logpush_public_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    otel: std::option::Option<std::vec::Vec<AiGateway_Otel>>,
    rate_limiting_interval: terrars::PrimField<f64>,
    rate_limiting_limit: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limiting_technique: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_delay: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_max_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spend_limits: std::option::Option<AiGateway_SpendLimits>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    store_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stripe: std::option::Option<AiGateway_Stripe>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workers_ai_billing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zdr: std::option::Option<terrars::PrimField<bool>>,
}

struct AiGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AiGatewayData_>,
}

#[derive(Clone)]
pub struct AiGateway(std::rc::Rc<AiGateway_>);

impl AiGateway {
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

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `byok_only`.\nRequires customer-provided provider credentials and prevents fallback to Unified Billing."]
    pub fn set_byok_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().byok_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dlp`.\n"]
    pub fn set_dlp(self, v: impl std::convert::Into<AiGateway_Dlp>) -> Self {
        self.0.data.borrow_mut().dlp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrails`.\n"]
    pub fn set_guardrails(self, v: impl std::convert::Into<AiGateway_Guardrails>) -> Self {
        self.0.data.borrow_mut().guardrails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_classification`.\n"]
    pub fn set_log_classification(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().log_classification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_management`.\n"]
    pub fn set_log_management(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().log_management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_management_strategy`.\nAvailable values: \"STOP_INSERTING\", \"DELETE_OLDEST\"."]
    pub fn set_log_management_strategy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().log_management_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logpush`.\n"]
    pub fn set_logpush(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().logpush = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logpush_public_key`.\n"]
    pub fn set_logpush_public_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().logpush_public_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `otel`.\n"]
    pub fn set_otel(self, v: impl std::convert::Into<std::vec::Vec<AiGateway_Otel>>) -> Self {
        self.0.data.borrow_mut().otel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_limiting_technique`.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn set_rate_limiting_technique(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().rate_limiting_technique = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_backoff`.\nBackoff strategy for retry delays\nAvailable values: \"constant\", \"linear\", \"exponential\"."]
    pub fn set_retry_backoff(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().retry_backoff = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_delay`.\nDelay between retry attempts in milliseconds (0-60000)"]
    pub fn set_retry_delay(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retry_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_max_attempts`.\nMaximum number of retry attempts for failed requests (1-5)"]
    pub fn set_retry_max_attempts(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retry_max_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spend_limits`.\n"]
    pub fn set_spend_limits(self, v: impl std::convert::Into<AiGateway_SpendLimits>) -> Self {
        self.0.data.borrow_mut().spend_limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `store_id`.\n"]
    pub fn set_store_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().store_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stripe`.\n"]
    pub fn set_stripe(self, v: impl std::convert::Into<AiGateway_Stripe>) -> Self {
        self.0.data.borrow_mut().stripe = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `workers_ai_billing_mode`.\nControls how Workers AI inference calls routed through this gateway are billed. 'postpaid' bills the account directly through Workers AI; 'unified' deducts credits via AI Gateway using neuron-based pricing and delegates billing to AI Gateway.\nAvailable values: \"postpaid\", \"unified\"."]
    pub fn set_workers_ai_billing_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().workers_ai_billing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zdr`.\n"]
    pub fn set_zdr(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().zdr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `byok_only` after provisioning.\nRequires customer-provided provider credentials and prevents fallback to Unified Billing."]
    pub fn byok_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.byok_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_invalidate_on_update` after provisioning.\n"]
    pub fn cache_invalidate_on_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_invalidate_on_update", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_ttl` after provisioning.\n"]
    pub fn cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collect_logs` after provisioning.\n"]
    pub fn collect_logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collect_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dlp` after provisioning.\n"]
    pub fn dlp(&self) -> AiGateway_DlpRef {
        AiGateway_DlpRef::new(self.shared().clone(), format!("{}.dlp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> AiGateway_GuardrailsRef {
        AiGateway_GuardrailsRef::new(self.shared().clone(), format!("{}.guardrails", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier of the AI Gateway within the account."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_default` after provisioning.\n"]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_classification` after provisioning.\n"]
    pub fn log_classification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_classification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_management` after provisioning.\n"]
    pub fn log_management(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_management", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_management_strategy` after provisioning.\nAvailable values: \"STOP_INSERTING\", \"DELETE_OLDEST\"."]
    pub fn log_management_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_management_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logpush` after provisioning.\n"]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logpush_public_key` after provisioning.\n"]
    pub fn logpush_public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush_public_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `otel` after provisioning.\n"]
    pub fn otel(&self) -> terrars::ListRef<AiGateway_OtelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.otel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rate_limiting_interval` after provisioning.\n"]
    pub fn rate_limiting_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rate_limiting_limit` after provisioning.\n"]
    pub fn rate_limiting_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rate_limiting_technique` after provisioning.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn rate_limiting_technique(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_technique", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retry_backoff` after provisioning.\nBackoff strategy for retry delays\nAvailable values: \"constant\", \"linear\", \"exponential\"."]
    pub fn retry_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_backoff", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retry_delay` after provisioning.\nDelay between retry attempts in milliseconds (0-60000)"]
    pub fn retry_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_delay", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retry_max_attempts` after provisioning.\nMaximum number of retry attempts for failed requests (1-5)"]
    pub fn retry_max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_max_attempts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spend_limits` after provisioning.\n"]
    pub fn spend_limits(&self) -> AiGateway_SpendLimitsRef {
        AiGateway_SpendLimitsRef::new(self.shared().clone(), format!("{}.spend_limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `store_id` after provisioning.\n"]
    pub fn store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stripe` after provisioning.\n"]
    pub fn stripe(&self) -> AiGateway_StripeRef {
        AiGateway_StripeRef::new(self.shared().clone(), format!("{}.stripe", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workers_ai_billing_mode` after provisioning.\nControls how Workers AI inference calls routed through this gateway are billed. 'postpaid' bills the account directly through Workers AI; 'unified' deducts credits via AI Gateway using neuron-based pricing and delegates billing to AI Gateway.\nAvailable values: \"postpaid\", \"unified\"."]
    pub fn workers_ai_billing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workers_ai_billing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zdr` after provisioning.\n"]
    pub fn zdr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zdr", self.extract_ref()))
    }
}

impl terrars::Referable for AiGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AiGateway { }

impl terrars::ToListMappable for AiGateway {
    type O = terrars::ListRef<AiGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AiGateway_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_ai_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAiGateway {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub cache_invalidate_on_update: terrars::PrimField<bool>,
    #[doc = ""]
    pub cache_ttl: terrars::PrimField<f64>,
    #[doc = ""]
    pub collect_logs: terrars::PrimField<bool>,
    #[doc = "Unique identifier of the AI Gateway within the account."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rate_limiting_interval: terrars::PrimField<f64>,
    #[doc = ""]
    pub rate_limiting_limit: terrars::PrimField<f64>,
}

impl BuildAiGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> AiGateway {
        let out = AiGateway(std::rc::Rc::new(AiGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AiGatewayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                authentication: core::default::Default::default(),
                byok_only: core::default::Default::default(),
                cache_invalidate_on_update: self.cache_invalidate_on_update,
                cache_ttl: self.cache_ttl,
                collect_logs: self.collect_logs,
                dlp: core::default::Default::default(),
                guardrails: core::default::Default::default(),
                id: self.id,
                log_classification: core::default::Default::default(),
                log_management: core::default::Default::default(),
                log_management_strategy: core::default::Default::default(),
                logpush: core::default::Default::default(),
                logpush_public_key: core::default::Default::default(),
                otel: core::default::Default::default(),
                rate_limiting_interval: self.rate_limiting_interval,
                rate_limiting_limit: self.rate_limiting_limit,
                rate_limiting_technique: core::default::Default::default(),
                retry_backoff: core::default::Default::default(),
                retry_delay: core::default::Default::default(),
                retry_max_attempts: core::default::Default::default(),
                spend_limits: core::default::Default::default(),
                store_id: core::default::Default::default(),
                stripe: core::default::Default::default(),
                workers_ai_billing_mode: core::default::Default::default(),
                zdr: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AiGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AiGatewayRef {
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

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `byok_only` after provisioning.\nRequires customer-provided provider credentials and prevents fallback to Unified Billing."]
    pub fn byok_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.byok_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_invalidate_on_update` after provisioning.\n"]
    pub fn cache_invalidate_on_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_invalidate_on_update", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_ttl` after provisioning.\n"]
    pub fn cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collect_logs` after provisioning.\n"]
    pub fn collect_logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collect_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dlp` after provisioning.\n"]
    pub fn dlp(&self) -> AiGateway_DlpRef {
        AiGateway_DlpRef::new(self.shared().clone(), format!("{}.dlp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> AiGateway_GuardrailsRef {
        AiGateway_GuardrailsRef::new(self.shared().clone(), format!("{}.guardrails", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier of the AI Gateway within the account."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_default` after provisioning.\n"]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_classification` after provisioning.\n"]
    pub fn log_classification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_classification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_management` after provisioning.\n"]
    pub fn log_management(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_management", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_management_strategy` after provisioning.\nAvailable values: \"STOP_INSERTING\", \"DELETE_OLDEST\"."]
    pub fn log_management_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_management_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logpush` after provisioning.\n"]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logpush_public_key` after provisioning.\n"]
    pub fn logpush_public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush_public_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `otel` after provisioning.\n"]
    pub fn otel(&self) -> terrars::ListRef<AiGateway_OtelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.otel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rate_limiting_interval` after provisioning.\n"]
    pub fn rate_limiting_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rate_limiting_limit` after provisioning.\n"]
    pub fn rate_limiting_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rate_limiting_technique` after provisioning.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn rate_limiting_technique(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_technique", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retry_backoff` after provisioning.\nBackoff strategy for retry delays\nAvailable values: \"constant\", \"linear\", \"exponential\"."]
    pub fn retry_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_backoff", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retry_delay` after provisioning.\nDelay between retry attempts in milliseconds (0-60000)"]
    pub fn retry_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_delay", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retry_max_attempts` after provisioning.\nMaximum number of retry attempts for failed requests (1-5)"]
    pub fn retry_max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_max_attempts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spend_limits` after provisioning.\n"]
    pub fn spend_limits(&self) -> AiGateway_SpendLimitsRef {
        AiGateway_SpendLimitsRef::new(self.shared().clone(), format!("{}.spend_limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `store_id` after provisioning.\n"]
    pub fn store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stripe` after provisioning.\n"]
    pub fn stripe(&self) -> AiGateway_StripeRef {
        AiGateway_StripeRef::new(self.shared().clone(), format!("{}.stripe", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workers_ai_billing_mode` after provisioning.\nControls how Workers AI inference calls routed through this gateway are billed. 'postpaid' bills the account directly through Workers AI; 'unified' deducts credits via AI Gateway using neuron-based pricing and delegates billing to AI Gateway.\nAvailable values: \"postpaid\", \"unified\"."]
    pub fn workers_ai_billing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workers_ai_billing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zdr` after provisioning.\n"]
    pub fn zdr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zdr", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AiGateway_Dlp_Policies {
    action: terrars::PrimField<std::string::String>,
    check: terrars::ListField<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    id: terrars::PrimField<std::string::String>,
    profiles: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl AiGateway_Dlp_Policies { }

impl terrars::ToListMappable for AiGateway_Dlp_Policies {
    type O = terrars::BlockAssignable<AiGateway_Dlp_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_Dlp_Policies {
    #[doc = "Available values: \"FLAG\", \"BLOCK\"."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub check: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub profiles: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildAiGateway_Dlp_Policies {
    pub fn build(self) -> AiGateway_Dlp_Policies {
        AiGateway_Dlp_Policies {
            action: self.action,
            check: self.check,
            enabled: self.enabled,
            id: self.id,
            profiles: self.profiles,
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_Dlp_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_Dlp_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_Dlp_PoliciesRef {
        AiGateway_Dlp_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_Dlp_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `check` after provisioning.\n"]
    pub fn check(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_Dlp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policies: std::option::Option<std::vec::Vec<AiGateway_Dlp_Policies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AiGateway_Dlp {
    #[doc = "Set the field `action`.\nAvailable values: \"BLOCK\", \"FLAG\"."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policies`.\n"]
    pub fn set_policies(mut self, v: impl std::convert::Into<std::vec::Vec<AiGateway_Dlp_Policies>>) -> Self {
        self.policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profiles`.\n"]
    pub fn set_profiles(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.profiles = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiGateway_Dlp {
    type O = terrars::BlockAssignable<AiGateway_Dlp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_Dlp {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildAiGateway_Dlp {
    pub fn build(self) -> AiGateway_Dlp {
        AiGateway_Dlp {
            action: core::default::Default::default(),
            enabled: self.enabled,
            policies: core::default::Default::default(),
            profiles: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_DlpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_DlpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_DlpRef {
        AiGateway_DlpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_DlpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAvailable values: \"BLOCK\", \"FLAG\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> terrars::ListRef<AiGateway_Dlp_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.base))
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_Guardrails_Prompt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    p1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s10: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s11: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s12: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s13: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s5: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s7: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s8: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s9: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AiGateway_Guardrails_Prompt {
    #[doc = "Set the field `p1`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_p1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.p1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s1`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s10`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s10(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s10 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s11`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s11(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s11 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s12`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s12(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s12 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s13`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s13(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s13 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s2`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s4`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s5`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s5(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s5 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s6`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s7`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s7(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s7 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s8`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s8(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s8 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s9`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s9(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s9 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiGateway_Guardrails_Prompt {
    type O = terrars::BlockAssignable<AiGateway_Guardrails_Prompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_Guardrails_Prompt {}

impl BuildAiGateway_Guardrails_Prompt {
    pub fn build(self) -> AiGateway_Guardrails_Prompt {
        AiGateway_Guardrails_Prompt {
            p1: core::default::Default::default(),
            s1: core::default::Default::default(),
            s10: core::default::Default::default(),
            s11: core::default::Default::default(),
            s12: core::default::Default::default(),
            s13: core::default::Default::default(),
            s2: core::default::Default::default(),
            s3: core::default::Default::default(),
            s4: core::default::Default::default(),
            s5: core::default::Default::default(),
            s6: core::default::Default::default(),
            s7: core::default::Default::default(),
            s8: core::default::Default::default(),
            s9: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_Guardrails_PromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_Guardrails_PromptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_Guardrails_PromptRef {
        AiGateway_Guardrails_PromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_Guardrails_PromptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `p1` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn p1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p1", self.base))
    }

    #[doc = "Get a reference to the value of field `s1` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s1", self.base))
    }

    #[doc = "Get a reference to the value of field `s10` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s10(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s10", self.base))
    }

    #[doc = "Get a reference to the value of field `s11` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s11(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s11", self.base))
    }

    #[doc = "Get a reference to the value of field `s12` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s12(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s12", self.base))
    }

    #[doc = "Get a reference to the value of field `s13` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s13(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s13", self.base))
    }

    #[doc = "Get a reference to the value of field `s2` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s2", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `s4` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s4", self.base))
    }

    #[doc = "Get a reference to the value of field `s5` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s5(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s5", self.base))
    }

    #[doc = "Get a reference to the value of field `s6` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s6", self.base))
    }

    #[doc = "Get a reference to the value of field `s7` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s7(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s7", self.base))
    }

    #[doc = "Get a reference to the value of field `s8` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s8(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s8", self.base))
    }

    #[doc = "Get a reference to the value of field `s9` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s9(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s9", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_Guardrails_Response {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    p1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s10: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s11: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s12: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s13: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s5: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s7: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s8: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s9: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AiGateway_Guardrails_Response {
    #[doc = "Set the field `p1`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_p1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.p1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s1`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s10`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s10(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s10 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s11`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s11(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s11 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s12`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s12(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s12 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s13`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s13(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s13 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s2`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s4`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s5`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s5(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s5 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s6`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s7`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s7(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s7 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s8`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s8(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s8 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s9`.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn set_s9(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s9 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiGateway_Guardrails_Response {
    type O = terrars::BlockAssignable<AiGateway_Guardrails_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_Guardrails_Response {}

impl BuildAiGateway_Guardrails_Response {
    pub fn build(self) -> AiGateway_Guardrails_Response {
        AiGateway_Guardrails_Response {
            p1: core::default::Default::default(),
            s1: core::default::Default::default(),
            s10: core::default::Default::default(),
            s11: core::default::Default::default(),
            s12: core::default::Default::default(),
            s13: core::default::Default::default(),
            s2: core::default::Default::default(),
            s3: core::default::Default::default(),
            s4: core::default::Default::default(),
            s5: core::default::Default::default(),
            s6: core::default::Default::default(),
            s7: core::default::Default::default(),
            s8: core::default::Default::default(),
            s9: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_Guardrails_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_Guardrails_ResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_Guardrails_ResponseRef {
        AiGateway_Guardrails_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_Guardrails_ResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `p1` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn p1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p1", self.base))
    }

    #[doc = "Get a reference to the value of field `s1` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s1", self.base))
    }

    #[doc = "Get a reference to the value of field `s10` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s10(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s10", self.base))
    }

    #[doc = "Get a reference to the value of field `s11` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s11(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s11", self.base))
    }

    #[doc = "Get a reference to the value of field `s12` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s12(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s12", self.base))
    }

    #[doc = "Get a reference to the value of field `s13` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s13(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s13", self.base))
    }

    #[doc = "Get a reference to the value of field `s2` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s2", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `s4` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s4", self.base))
    }

    #[doc = "Get a reference to the value of field `s5` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s5(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s5", self.base))
    }

    #[doc = "Get a reference to the value of field `s6` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s6", self.base))
    }

    #[doc = "Get a reference to the value of field `s7` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s7(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s7", self.base))
    }

    #[doc = "Get a reference to the value of field `s8` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s8(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s8", self.base))
    }

    #[doc = "Get a reference to the value of field `s9` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s9(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s9", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_Guardrails {
    prompt: AiGateway_Guardrails_Prompt,
    response: AiGateway_Guardrails_Response,
}

impl AiGateway_Guardrails { }

impl terrars::ToListMappable for AiGateway_Guardrails {
    type O = terrars::BlockAssignable<AiGateway_Guardrails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_Guardrails {
    #[doc = ""]
    pub prompt: AiGateway_Guardrails_Prompt,
    #[doc = ""]
    pub response: AiGateway_Guardrails_Response,
}

impl BuildAiGateway_Guardrails {
    pub fn build(self) -> AiGateway_Guardrails {
        AiGateway_Guardrails {
            prompt: self.prompt,
            response: self.response,
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_GuardrailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_GuardrailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_GuardrailsRef {
        AiGateway_GuardrailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_GuardrailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> AiGateway_Guardrails_PromptRef {
        AiGateway_Guardrails_PromptRef::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> AiGateway_Guardrails_ResponseRef {
        AiGateway_Guardrails_ResponseRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_Otel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    headers: terrars::RecField<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
}

impl AiGateway_Otel {
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.authorization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type`.\nAvailable values: \"json\", \"protobuf\"."]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiGateway_Otel {
    type O = terrars::BlockAssignable<AiGateway_Otel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_Otel {
    #[doc = ""]
    pub headers: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildAiGateway_Otel {
    pub fn build(self) -> AiGateway_Otel {
        AiGateway_Otel {
            authorization: core::default::Default::default(),
            content_type: core::default::Default::default(),
            headers: self.headers,
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_OtelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_OtelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_OtelRef {
        AiGateway_OtelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_OtelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nAvailable values: \"json\", \"protobuf\"."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_SpendLimits_Rules_AiGatewayProvider {
    mode: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl AiGateway_SpendLimits_Rules_AiGatewayProvider { }

impl terrars::ToListMappable for AiGateway_SpendLimits_Rules_AiGatewayProvider {
    type O = terrars::BlockAssignable<AiGateway_SpendLimits_Rules_AiGatewayProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_SpendLimits_Rules_AiGatewayProvider {
    #[doc = "Available values: \"filter\"."]
    pub mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildAiGateway_SpendLimits_Rules_AiGatewayProvider {
    pub fn build(self) -> AiGateway_SpendLimits_Rules_AiGatewayProvider {
        AiGateway_SpendLimits_Rules_AiGatewayProvider {
            mode: self.mode,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_SpendLimits_Rules_AiGatewayProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_SpendLimits_Rules_AiGatewayProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGateway_SpendLimits_Rules_AiGatewayProviderRef {
        AiGateway_SpendLimits_Rules_AiGatewayProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_SpendLimits_Rules_AiGatewayProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nAvailable values: \"filter\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_SpendLimits_Rules_Metadata {
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AiGateway_SpendLimits_Rules_Metadata {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiGateway_SpendLimits_Rules_Metadata {
    type O = terrars::BlockAssignable<AiGateway_SpendLimits_Rules_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_SpendLimits_Rules_Metadata {
    #[doc = "Available values: \"partition\", \"filter\"."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildAiGateway_SpendLimits_Rules_Metadata {
    pub fn build(self) -> AiGateway_SpendLimits_Rules_Metadata {
        AiGateway_SpendLimits_Rules_Metadata {
            mode: self.mode,
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_SpendLimits_Rules_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_SpendLimits_Rules_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_SpendLimits_Rules_MetadataRef {
        AiGateway_SpendLimits_Rules_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_SpendLimits_Rules_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nAvailable values: \"partition\", \"filter\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_SpendLimits_Rules_Model {
    mode: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl AiGateway_SpendLimits_Rules_Model { }

impl terrars::ToListMappable for AiGateway_SpendLimits_Rules_Model {
    type O = terrars::BlockAssignable<AiGateway_SpendLimits_Rules_Model>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_SpendLimits_Rules_Model {
    #[doc = "Available values: \"filter\"."]
    pub mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildAiGateway_SpendLimits_Rules_Model {
    pub fn build(self) -> AiGateway_SpendLimits_Rules_Model {
        AiGateway_SpendLimits_Rules_Model {
            mode: self.mode,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_SpendLimits_Rules_ModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_SpendLimits_Rules_ModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_SpendLimits_Rules_ModelRef {
        AiGateway_SpendLimits_Rules_ModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_SpendLimits_Rules_ModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nAvailable values: \"filter\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_SpendLimits_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_gateway_provider: std::option::Option<AiGateway_SpendLimits_Rules_AiGatewayProvider>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    limit: terrars::PrimField<f64>,
    limit_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::collections::HashMap<std::string::String, AiGateway_SpendLimits_Rules_Metadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<AiGateway_SpendLimits_Rules_Model>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technique: std::option::Option<terrars::PrimField<std::string::String>>,
    window: terrars::PrimField<f64>,
}

impl AiGateway_SpendLimits_Rules {
    #[doc = "Set the field `ai_gateway_provider`.\n"]
    pub fn set_ai_gateway_provider(
        mut self,
        v: impl std::convert::Into<AiGateway_SpendLimits_Rules_AiGatewayProvider>,
    ) -> Self {
        self.ai_gateway_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<std::collections::HashMap<std::string::String, AiGateway_SpendLimits_Rules_Metadata>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl std::convert::Into<AiGateway_SpendLimits_Rules_Model>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technique`.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn set_technique(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technique = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiGateway_SpendLimits_Rules {
    type O = terrars::BlockAssignable<AiGateway_SpendLimits_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_SpendLimits_Rules {
    #[doc = ""]
    pub limit: terrars::PrimField<f64>,
    #[doc = "Available values: \"cost\"."]
    pub limit_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub window: terrars::PrimField<f64>,
}

impl BuildAiGateway_SpendLimits_Rules {
    pub fn build(self) -> AiGateway_SpendLimits_Rules {
        AiGateway_SpendLimits_Rules {
            ai_gateway_provider: core::default::Default::default(),
            enabled: core::default::Default::default(),
            id: core::default::Default::default(),
            limit: self.limit,
            limit_type: self.limit_type,
            metadata: core::default::Default::default(),
            model: core::default::Default::default(),
            technique: core::default::Default::default(),
            window: self.window,
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_SpendLimits_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_SpendLimits_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_SpendLimits_RulesRef {
        AiGateway_SpendLimits_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_SpendLimits_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ai_gateway_provider` after provisioning.\n"]
    pub fn ai_gateway_provider(&self) -> AiGateway_SpendLimits_Rules_AiGatewayProviderRef {
        AiGateway_SpendLimits_Rules_AiGatewayProviderRef::new(
            self.shared().clone(),
            format!("{}.ai_gateway_provider", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `limit` after provisioning.\n"]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc = "Get a reference to the value of field `limit_type` after provisioning.\nAvailable values: \"cost\"."]
    pub fn limit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit_type", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<AiGateway_SpendLimits_Rules_MetadataRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> AiGateway_SpendLimits_Rules_ModelRef {
        AiGateway_SpendLimits_Rules_ModelRef::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technique` after provisioning.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn technique(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technique", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_SpendLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<AiGateway_SpendLimits_Rules>>,
}

impl AiGateway_SpendLimits {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(mut self, v: impl std::convert::Into<std::vec::Vec<AiGateway_SpendLimits_Rules>>) -> Self {
        self.rules = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiGateway_SpendLimits {
    type O = terrars::BlockAssignable<AiGateway_SpendLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_SpendLimits {}

impl BuildAiGateway_SpendLimits {
    pub fn build(self) -> AiGateway_SpendLimits {
        AiGateway_SpendLimits {
            enabled: core::default::Default::default(),
            rules: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_SpendLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_SpendLimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_SpendLimitsRef {
        AiGateway_SpendLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_SpendLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<AiGateway_SpendLimits_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_Stripe_UsageEvents {
    payload: terrars::PrimField<std::string::String>,
}

impl AiGateway_Stripe_UsageEvents { }

impl terrars::ToListMappable for AiGateway_Stripe_UsageEvents {
    type O = terrars::BlockAssignable<AiGateway_Stripe_UsageEvents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_Stripe_UsageEvents {
    #[doc = ""]
    pub payload: terrars::PrimField<std::string::String>,
}

impl BuildAiGateway_Stripe_UsageEvents {
    pub fn build(self) -> AiGateway_Stripe_UsageEvents {
        AiGateway_Stripe_UsageEvents { payload: self.payload }
    }
}

#[allow(dead_code)]
pub struct AiGateway_Stripe_UsageEventsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_Stripe_UsageEventsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_Stripe_UsageEventsRef {
        AiGateway_Stripe_UsageEventsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_Stripe_UsageEventsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGateway_Stripe {
    authorization: terrars::PrimField<std::string::String>,
    usage_events: std::vec::Vec<AiGateway_Stripe_UsageEvents>,
}

impl AiGateway_Stripe { }

impl terrars::ToListMappable for AiGateway_Stripe {
    type O = terrars::BlockAssignable<AiGateway_Stripe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGateway_Stripe {
    #[doc = ""]
    pub authorization: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub usage_events: std::vec::Vec<AiGateway_Stripe_UsageEvents>,
}

impl BuildAiGateway_Stripe {
    pub fn build(self) -> AiGateway_Stripe {
        AiGateway_Stripe {
            authorization: self.authorization,
            usage_events: self.usage_events,
        }
    }
}

#[allow(dead_code)]
pub struct AiGateway_StripeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGateway_StripeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGateway_StripeRef {
        AiGateway_StripeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGateway_StripeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization", self.base))
    }

    #[doc = "Get a reference to the value of field `usage_events` after provisioning.\n"]
    pub fn usage_events(&self) -> terrars::ListRef<AiGateway_Stripe_UsageEventsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usage_events", self.base))
    }
}
