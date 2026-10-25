#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAiSearchNamespaceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

struct DataAiSearchNamespace_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAiSearchNamespaceData_>,
}

#[derive(Clone)]
pub struct DataAiSearchNamespace(std::rc::Rc<DataAiSearchNamespace_>);

impl DataAiSearchNamespace {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional description for the namespace. Max 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_id` after provisioning.\n"]
    pub fn public_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_params` after provisioning.\n"]
    pub fn public_endpoint_params(&self) -> DataAiSearchNamespace_PublicEndpointParamsRef {
        DataAiSearchNamespace_PublicEndpointParamsRef::new(
            self.shared().clone(),
            format!("{}.public_endpoint_params", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataAiSearchNamespace {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAiSearchNamespace { }

impl terrars::ToListMappable for DataAiSearchNamespace {
    type O = terrars::ListRef<DataAiSearchNamespaceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAiSearchNamespace_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_ai_search_namespace".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAiSearchNamespace {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataAiSearchNamespace {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAiSearchNamespace {
        let out = DataAiSearchNamespace(std::rc::Rc::new(DataAiSearchNamespace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAiSearchNamespaceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAiSearchNamespaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchNamespaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAiSearchNamespaceRef {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional description for the namespace. Max 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_id` after provisioning.\n"]
    pub fn public_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_params` after provisioning.\n"]
    pub fn public_endpoint_params(&self) -> DataAiSearchNamespace_PublicEndpointParamsRef {
        DataAiSearchNamespace_PublicEndpointParamsRef::new(
            self.shared().clone(),
            format!("{}.public_endpoint_params", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {}

impl DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint { }

impl terrars::ToListMappable for DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {
    type O = terrars::BlockAssignable<DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {}

impl BuildDataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {
    pub fn build(self) -> DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {
        DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
        DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDisable chat completions endpoint for this public endpoint"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchNamespace_PublicEndpointParams_Mcp {}

impl DataAiSearchNamespace_PublicEndpointParams_Mcp { }

impl terrars::ToListMappable for DataAiSearchNamespace_PublicEndpointParams_Mcp {
    type O = terrars::BlockAssignable<DataAiSearchNamespace_PublicEndpointParams_Mcp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchNamespace_PublicEndpointParams_Mcp {}

impl BuildDataAiSearchNamespace_PublicEndpointParams_Mcp {
    pub fn build(self) -> DataAiSearchNamespace_PublicEndpointParams_Mcp {
        DataAiSearchNamespace_PublicEndpointParams_Mcp {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchNamespace_PublicEndpointParams_McpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchNamespace_PublicEndpointParams_McpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchNamespace_PublicEndpointParams_McpRef {
        DataAiSearchNamespace_PublicEndpointParams_McpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchNamespace_PublicEndpointParams_McpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDisable MCP endpoint for this public endpoint"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchNamespace_PublicEndpointParams_RateLimit {}

impl DataAiSearchNamespace_PublicEndpointParams_RateLimit { }

impl terrars::ToListMappable for DataAiSearchNamespace_PublicEndpointParams_RateLimit {
    type O = terrars::BlockAssignable<DataAiSearchNamespace_PublicEndpointParams_RateLimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchNamespace_PublicEndpointParams_RateLimit {}

impl BuildDataAiSearchNamespace_PublicEndpointParams_RateLimit {
    pub fn build(self) -> DataAiSearchNamespace_PublicEndpointParams_RateLimit {
        DataAiSearchNamespace_PublicEndpointParams_RateLimit {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchNamespace_PublicEndpointParams_RateLimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchNamespace_PublicEndpointParams_RateLimitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchNamespace_PublicEndpointParams_RateLimitRef {
        DataAiSearchNamespace_PublicEndpointParams_RateLimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchNamespace_PublicEndpointParams_RateLimitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `period_ms` after provisioning.\n"]
    pub fn period_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technique` after provisioning.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn technique(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technique", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchNamespace_PublicEndpointParams_SearchEndpoint {}

impl DataAiSearchNamespace_PublicEndpointParams_SearchEndpoint { }

impl terrars::ToListMappable for DataAiSearchNamespace_PublicEndpointParams_SearchEndpoint {
    type O = terrars::BlockAssignable<DataAiSearchNamespace_PublicEndpointParams_SearchEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchNamespace_PublicEndpointParams_SearchEndpoint {}

impl BuildDataAiSearchNamespace_PublicEndpointParams_SearchEndpoint {
    pub fn build(self) -> DataAiSearchNamespace_PublicEndpointParams_SearchEndpoint {
        DataAiSearchNamespace_PublicEndpointParams_SearchEndpoint {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
        DataAiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDisable search endpoint for this public endpoint"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiSearchNamespace_PublicEndpointParams {}

impl DataAiSearchNamespace_PublicEndpointParams { }

impl terrars::ToListMappable for DataAiSearchNamespace_PublicEndpointParams {
    type O = terrars::BlockAssignable<DataAiSearchNamespace_PublicEndpointParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiSearchNamespace_PublicEndpointParams {}

impl BuildDataAiSearchNamespace_PublicEndpointParams {
    pub fn build(self) -> DataAiSearchNamespace_PublicEndpointParams {
        DataAiSearchNamespace_PublicEndpointParams {}
    }
}

#[allow(dead_code)]
pub struct DataAiSearchNamespace_PublicEndpointParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiSearchNamespace_PublicEndpointParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiSearchNamespace_PublicEndpointParamsRef {
        DataAiSearchNamespace_PublicEndpointParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiSearchNamespace_PublicEndpointParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorized_hosts` after provisioning.\n"]
    pub fn authorized_hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `chat_completions_endpoint` after provisioning.\n"]
    pub fn chat_completions_endpoint(&self) -> DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
        DataAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef::new(
            self.shared().clone(),
            format!("{}.chat_completions_endpoint", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `custom_domains` after provisioning.\nCustom domain hostnames that alias this public endpoint. GET and create responses return the current set; on update (PUT) this field is only echoed back when supplied in the request body, otherwise it is null (omit it to leave domains unchanged)."]
    pub fn custom_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_domain_enabled` after provisioning.\nWhen false, the instance is reachable only via a registered custom domain and the default <public_endpoint_id>.search.ai.cloudflare.com host returns 404. Requires at least one custom domain. Defaults to true. public_endpoint_params is replaced wholesale on update, so resend default_domain_enabled on every update to keep the default host off — omitting it resets to true."]
    pub fn default_domain_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_domain_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instances_allowed` after provisioning.\nInstance IDs exposed through the namespace public endpoint. Empty means nothing is searchable. Every ID must be an existing instance in this namespace, and the list cannot exceed the account's multi-instance search limit."]
    pub fn instances_allowed(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances_allowed", self.base))
    }

    #[doc = "Get a reference to the value of field `mcp` after provisioning.\n"]
    pub fn mcp(&self) -> DataAiSearchNamespace_PublicEndpointParams_McpRef {
        DataAiSearchNamespace_PublicEndpointParams_McpRef::new(self.shared().clone(), format!("{}.mcp", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit` after provisioning.\n"]
    pub fn rate_limit(&self) -> DataAiSearchNamespace_PublicEndpointParams_RateLimitRef {
        DataAiSearchNamespace_PublicEndpointParams_RateLimitRef::new(
            self.shared().clone(),
            format!("{}.rate_limit", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `search_endpoint` after provisioning.\n"]
    pub fn search_endpoint(&self) -> DataAiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
        DataAiSearchNamespace_PublicEndpointParams_SearchEndpointRef::new(
            self.shared().clone(),
            format!("{}.search_endpoint", self.base),
        )
    }
}
