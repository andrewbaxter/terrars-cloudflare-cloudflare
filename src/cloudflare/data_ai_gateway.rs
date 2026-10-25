#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAiGatewayData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataAiGateway_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAiGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAiGatewayData_>,
}

#[derive(Clone)]
pub struct DataAiGateway(std::rc::Rc<DataAiGateway_>);

impl DataAiGateway {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataAiGateway_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nUnique identifier of the AI Gateway within the account."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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
    pub fn dlp(&self) -> DataAiGateway_DlpRef {
        DataAiGateway_DlpRef::new(self.shared().clone(), format!("{}.dlp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataAiGateway_FilterRef {
        DataAiGateway_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> DataAiGateway_GuardrailsRef {
        DataAiGateway_GuardrailsRef::new(self.shared().clone(), format!("{}.guardrails", self.extract_ref()))
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
    pub fn otel(&self) -> terrars::ListRef<DataAiGateway_OtelRef> {
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
    pub fn spend_limits(&self) -> DataAiGateway_SpendLimitsRef {
        DataAiGateway_SpendLimitsRef::new(self.shared().clone(), format!("{}.spend_limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `store_id` after provisioning.\n"]
    pub fn store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stripe` after provisioning.\n"]
    pub fn stripe(&self) -> DataAiGateway_StripeRef {
        DataAiGateway_StripeRef::new(self.shared().clone(), format!("{}.stripe", self.extract_ref()))
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

impl terrars::Referable for DataAiGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAiGateway { }

impl terrars::ToListMappable for DataAiGateway {
    type O = terrars::ListRef<DataAiGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAiGateway_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_ai_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAiGateway {
    pub tf_id: std::string::String,
}

impl BuildDataAiGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAiGateway {
        let out = DataAiGateway(std::rc::Rc::new(DataAiGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAiGatewayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAiGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAiGatewayRef {
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
    pub fn dlp(&self) -> DataAiGateway_DlpRef {
        DataAiGateway_DlpRef::new(self.shared().clone(), format!("{}.dlp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataAiGateway_FilterRef {
        DataAiGateway_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> DataAiGateway_GuardrailsRef {
        DataAiGateway_GuardrailsRef::new(self.shared().clone(), format!("{}.guardrails", self.extract_ref()))
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
    pub fn otel(&self) -> terrars::ListRef<DataAiGateway_OtelRef> {
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
    pub fn spend_limits(&self) -> DataAiGateway_SpendLimitsRef {
        DataAiGateway_SpendLimitsRef::new(self.shared().clone(), format!("{}.spend_limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `store_id` after provisioning.\n"]
    pub fn store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.store_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stripe` after provisioning.\n"]
    pub fn stripe(&self) -> DataAiGateway_StripeRef {
        DataAiGateway_StripeRef::new(self.shared().clone(), format!("{}.stripe", self.extract_ref()))
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
pub struct DataAiGateway_Dlp_Policies {}

impl DataAiGateway_Dlp_Policies { }

impl terrars::ToListMappable for DataAiGateway_Dlp_Policies {
    type O = terrars::BlockAssignable<DataAiGateway_Dlp_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_Dlp_Policies {}

impl BuildDataAiGateway_Dlp_Policies {
    pub fn build(self) -> DataAiGateway_Dlp_Policies {
        DataAiGateway_Dlp_Policies {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_Dlp_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_Dlp_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_Dlp_PoliciesRef {
        DataAiGateway_Dlp_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_Dlp_PoliciesRef {
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
pub struct DataAiGateway_Dlp {}

impl DataAiGateway_Dlp { }

impl terrars::ToListMappable for DataAiGateway_Dlp {
    type O = terrars::BlockAssignable<DataAiGateway_Dlp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_Dlp {}

impl BuildDataAiGateway_Dlp {
    pub fn build(self) -> DataAiGateway_Dlp {
        DataAiGateway_Dlp {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_DlpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_DlpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_DlpRef {
        DataAiGateway_DlpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_DlpRef {
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
    pub fn policies(&self) -> terrars::ListRef<DataAiGateway_Dlp_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.base))
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateway_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAiGateway_Filter {
    #[doc = "Set the field `search`.\nSearch by id"]
    pub fn set_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAiGateway_Filter {
    type O = terrars::BlockAssignable<DataAiGateway_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_Filter {}

impl BuildDataAiGateway_Filter {
    pub fn build(self) -> DataAiGateway_Filter {
        DataAiGateway_Filter { search: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_FilterRef {
        DataAiGateway_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\nSearch by id"]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateway_Guardrails_Prompt {}

impl DataAiGateway_Guardrails_Prompt { }

impl terrars::ToListMappable for DataAiGateway_Guardrails_Prompt {
    type O = terrars::BlockAssignable<DataAiGateway_Guardrails_Prompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_Guardrails_Prompt {}

impl BuildDataAiGateway_Guardrails_Prompt {
    pub fn build(self) -> DataAiGateway_Guardrails_Prompt {
        DataAiGateway_Guardrails_Prompt {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_Guardrails_PromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_Guardrails_PromptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_Guardrails_PromptRef {
        DataAiGateway_Guardrails_PromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_Guardrails_PromptRef {
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
pub struct DataAiGateway_Guardrails_Response {}

impl DataAiGateway_Guardrails_Response { }

impl terrars::ToListMappable for DataAiGateway_Guardrails_Response {
    type O = terrars::BlockAssignable<DataAiGateway_Guardrails_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_Guardrails_Response {}

impl BuildDataAiGateway_Guardrails_Response {
    pub fn build(self) -> DataAiGateway_Guardrails_Response {
        DataAiGateway_Guardrails_Response {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_Guardrails_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_Guardrails_ResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_Guardrails_ResponseRef {
        DataAiGateway_Guardrails_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_Guardrails_ResponseRef {
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
pub struct DataAiGateway_Guardrails {}

impl DataAiGateway_Guardrails { }

impl terrars::ToListMappable for DataAiGateway_Guardrails {
    type O = terrars::BlockAssignable<DataAiGateway_Guardrails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_Guardrails {}

impl BuildDataAiGateway_Guardrails {
    pub fn build(self) -> DataAiGateway_Guardrails {
        DataAiGateway_Guardrails {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_GuardrailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_GuardrailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_GuardrailsRef {
        DataAiGateway_GuardrailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_GuardrailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> DataAiGateway_Guardrails_PromptRef {
        DataAiGateway_Guardrails_PromptRef::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> DataAiGateway_Guardrails_ResponseRef {
        DataAiGateway_Guardrails_ResponseRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateway_Otel {}

impl DataAiGateway_Otel { }

impl terrars::ToListMappable for DataAiGateway_Otel {
    type O = terrars::BlockAssignable<DataAiGateway_Otel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_Otel {}

impl BuildDataAiGateway_Otel {
    pub fn build(self) -> DataAiGateway_Otel {
        DataAiGateway_Otel {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_OtelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_OtelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_OtelRef {
        DataAiGateway_OtelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_OtelRef {
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
pub struct DataAiGateway_SpendLimits_Rules_AiGatewayProvider {}

impl DataAiGateway_SpendLimits_Rules_AiGatewayProvider { }

impl terrars::ToListMappable for DataAiGateway_SpendLimits_Rules_AiGatewayProvider {
    type O = terrars::BlockAssignable<DataAiGateway_SpendLimits_Rules_AiGatewayProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_SpendLimits_Rules_AiGatewayProvider {}

impl BuildDataAiGateway_SpendLimits_Rules_AiGatewayProvider {
    pub fn build(self) -> DataAiGateway_SpendLimits_Rules_AiGatewayProvider {
        DataAiGateway_SpendLimits_Rules_AiGatewayProvider {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_SpendLimits_Rules_AiGatewayProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_SpendLimits_Rules_AiGatewayProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGateway_SpendLimits_Rules_AiGatewayProviderRef {
        DataAiGateway_SpendLimits_Rules_AiGatewayProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_SpendLimits_Rules_AiGatewayProviderRef {
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
pub struct DataAiGateway_SpendLimits_Rules_Metadata {}

impl DataAiGateway_SpendLimits_Rules_Metadata { }

impl terrars::ToListMappable for DataAiGateway_SpendLimits_Rules_Metadata {
    type O = terrars::BlockAssignable<DataAiGateway_SpendLimits_Rules_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_SpendLimits_Rules_Metadata {}

impl BuildDataAiGateway_SpendLimits_Rules_Metadata {
    pub fn build(self) -> DataAiGateway_SpendLimits_Rules_Metadata {
        DataAiGateway_SpendLimits_Rules_Metadata {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_SpendLimits_Rules_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_SpendLimits_Rules_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_SpendLimits_Rules_MetadataRef {
        DataAiGateway_SpendLimits_Rules_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_SpendLimits_Rules_MetadataRef {
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
pub struct DataAiGateway_SpendLimits_Rules_Model {}

impl DataAiGateway_SpendLimits_Rules_Model { }

impl terrars::ToListMappable for DataAiGateway_SpendLimits_Rules_Model {
    type O = terrars::BlockAssignable<DataAiGateway_SpendLimits_Rules_Model>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_SpendLimits_Rules_Model {}

impl BuildDataAiGateway_SpendLimits_Rules_Model {
    pub fn build(self) -> DataAiGateway_SpendLimits_Rules_Model {
        DataAiGateway_SpendLimits_Rules_Model {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_SpendLimits_Rules_ModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_SpendLimits_Rules_ModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_SpendLimits_Rules_ModelRef {
        DataAiGateway_SpendLimits_Rules_ModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_SpendLimits_Rules_ModelRef {
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
pub struct DataAiGateway_SpendLimits_Rules {}

impl DataAiGateway_SpendLimits_Rules { }

impl terrars::ToListMappable for DataAiGateway_SpendLimits_Rules {
    type O = terrars::BlockAssignable<DataAiGateway_SpendLimits_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_SpendLimits_Rules {}

impl BuildDataAiGateway_SpendLimits_Rules {
    pub fn build(self) -> DataAiGateway_SpendLimits_Rules {
        DataAiGateway_SpendLimits_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_SpendLimits_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_SpendLimits_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_SpendLimits_RulesRef {
        DataAiGateway_SpendLimits_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_SpendLimits_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ai_gateway_provider` after provisioning.\n"]
    pub fn ai_gateway_provider(&self) -> DataAiGateway_SpendLimits_Rules_AiGatewayProviderRef {
        DataAiGateway_SpendLimits_Rules_AiGatewayProviderRef::new(
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
    pub fn metadata(&self) -> terrars::RecRef<DataAiGateway_SpendLimits_Rules_MetadataRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> DataAiGateway_SpendLimits_Rules_ModelRef {
        DataAiGateway_SpendLimits_Rules_ModelRef::new(self.shared().clone(), format!("{}.model", self.base))
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
pub struct DataAiGateway_SpendLimits {}

impl DataAiGateway_SpendLimits { }

impl terrars::ToListMappable for DataAiGateway_SpendLimits {
    type O = terrars::BlockAssignable<DataAiGateway_SpendLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_SpendLimits {}

impl BuildDataAiGateway_SpendLimits {
    pub fn build(self) -> DataAiGateway_SpendLimits {
        DataAiGateway_SpendLimits {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_SpendLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_SpendLimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_SpendLimitsRef {
        DataAiGateway_SpendLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_SpendLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataAiGateway_SpendLimits_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateway_Stripe_UsageEvents {}

impl DataAiGateway_Stripe_UsageEvents { }

impl terrars::ToListMappable for DataAiGateway_Stripe_UsageEvents {
    type O = terrars::BlockAssignable<DataAiGateway_Stripe_UsageEvents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_Stripe_UsageEvents {}

impl BuildDataAiGateway_Stripe_UsageEvents {
    pub fn build(self) -> DataAiGateway_Stripe_UsageEvents {
        DataAiGateway_Stripe_UsageEvents {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_Stripe_UsageEventsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_Stripe_UsageEventsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_Stripe_UsageEventsRef {
        DataAiGateway_Stripe_UsageEventsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_Stripe_UsageEventsRef {
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
pub struct DataAiGateway_Stripe {}

impl DataAiGateway_Stripe { }

impl terrars::ToListMappable for DataAiGateway_Stripe {
    type O = terrars::BlockAssignable<DataAiGateway_Stripe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateway_Stripe {}

impl BuildDataAiGateway_Stripe {
    pub fn build(self) -> DataAiGateway_Stripe {
        DataAiGateway_Stripe {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateway_StripeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateway_StripeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateway_StripeRef {
        DataAiGateway_StripeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateway_StripeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization", self.base))
    }

    #[doc = "Get a reference to the value of field `usage_events` after provisioning.\n"]
    pub fn usage_events(&self) -> terrars::ListRef<DataAiGateway_Stripe_UsageEventsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usage_events", self.base))
    }
}
