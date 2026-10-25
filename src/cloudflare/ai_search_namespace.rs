#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AiSearchNamespaceData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_endpoint_params: std::option::Option<AiSearchNamespace_PublicEndpointParams>,
}

struct AiSearchNamespace_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AiSearchNamespaceData_>,
}

#[derive(Clone)]
pub struct AiSearchNamespace(std::rc::Rc<AiSearchNamespace_>);

impl AiSearchNamespace {
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

    #[doc = "Set the field `description`.\nOptional description for the namespace. Max 256 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_endpoint_params`.\n"]
    pub fn set_public_endpoint_params(self, v: impl std::convert::Into<AiSearchNamespace_PublicEndpointParams>) -> Self {
        self.0.data.borrow_mut().public_endpoint_params = std::option::Option::Some(v.into());
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
    pub fn public_endpoint_params(&self) -> AiSearchNamespace_PublicEndpointParamsRef {
        AiSearchNamespace_PublicEndpointParamsRef::new(
            self.shared().clone(),
            format!("{}.public_endpoint_params", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AiSearchNamespace {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AiSearchNamespace { }

impl terrars::ToListMappable for AiSearchNamespace {
    type O = terrars::ListRef<AiSearchNamespaceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AiSearchNamespace_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_ai_search_namespace".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAiSearchNamespace {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAiSearchNamespace {
    pub fn build(self, stack: &mut terrars::Stack) -> AiSearchNamespace {
        let out = AiSearchNamespace(std::rc::Rc::new(AiSearchNamespace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AiSearchNamespaceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                description: core::default::Default::default(),
                name: self.name,
                public_endpoint_params: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AiSearchNamespaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchNamespaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AiSearchNamespaceRef {
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
    pub fn public_endpoint_params(&self) -> AiSearchNamespace_PublicEndpointParamsRef {
        AiSearchNamespace_PublicEndpointParamsRef::new(
            self.shared().clone(),
            format!("{}.public_endpoint_params", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {
    #[doc = "Set the field `disabled`.\nDisable chat completions endpoint for this public endpoint"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {
    type O = terrars::BlockAssignable<AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {}

impl BuildAiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {
    pub fn build(self) -> AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {
        AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint {
            disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
        AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
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
pub struct AiSearchNamespace_PublicEndpointParams_Mcp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AiSearchNamespace_PublicEndpointParams_Mcp {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\nDisable MCP endpoint for this public endpoint"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchNamespace_PublicEndpointParams_Mcp {
    type O = terrars::BlockAssignable<AiSearchNamespace_PublicEndpointParams_Mcp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchNamespace_PublicEndpointParams_Mcp {}

impl BuildAiSearchNamespace_PublicEndpointParams_Mcp {
    pub fn build(self) -> AiSearchNamespace_PublicEndpointParams_Mcp {
        AiSearchNamespace_PublicEndpointParams_Mcp {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchNamespace_PublicEndpointParams_McpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchNamespace_PublicEndpointParams_McpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchNamespace_PublicEndpointParams_McpRef {
        AiSearchNamespace_PublicEndpointParams_McpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchNamespace_PublicEndpointParams_McpRef {
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
pub struct AiSearchNamespace_PublicEndpointParams_RateLimit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technique: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AiSearchNamespace_PublicEndpointParams_RateLimit {
    #[doc = "Set the field `period_ms`.\n"]
    pub fn set_period_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.requests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technique`.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn set_technique(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technique = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchNamespace_PublicEndpointParams_RateLimit {
    type O = terrars::BlockAssignable<AiSearchNamespace_PublicEndpointParams_RateLimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchNamespace_PublicEndpointParams_RateLimit {}

impl BuildAiSearchNamespace_PublicEndpointParams_RateLimit {
    pub fn build(self) -> AiSearchNamespace_PublicEndpointParams_RateLimit {
        AiSearchNamespace_PublicEndpointParams_RateLimit {
            period_ms: core::default::Default::default(),
            requests: core::default::Default::default(),
            technique: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchNamespace_PublicEndpointParams_RateLimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchNamespace_PublicEndpointParams_RateLimitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiSearchNamespace_PublicEndpointParams_RateLimitRef {
        AiSearchNamespace_PublicEndpointParams_RateLimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchNamespace_PublicEndpointParams_RateLimitRef {
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
pub struct AiSearchNamespace_PublicEndpointParams_SearchEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AiSearchNamespace_PublicEndpointParams_SearchEndpoint {
    #[doc = "Set the field `disabled`.\nDisable search endpoint for this public endpoint"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchNamespace_PublicEndpointParams_SearchEndpoint {
    type O = terrars::BlockAssignable<AiSearchNamespace_PublicEndpointParams_SearchEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchNamespace_PublicEndpointParams_SearchEndpoint {}

impl BuildAiSearchNamespace_PublicEndpointParams_SearchEndpoint {
    pub fn build(self) -> AiSearchNamespace_PublicEndpointParams_SearchEndpoint {
        AiSearchNamespace_PublicEndpointParams_SearchEndpoint { disabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
        AiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
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
pub struct AiSearchNamespace_PublicEndpointParams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_hosts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chat_completions_endpoint: std::option::Option<AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_domain_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances_allowed: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp: std::option::Option<AiSearchNamespace_PublicEndpointParams_Mcp>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit: std::option::Option<AiSearchNamespace_PublicEndpointParams_RateLimit>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search_endpoint: std::option::Option<AiSearchNamespace_PublicEndpointParams_SearchEndpoint>,
}

impl AiSearchNamespace_PublicEndpointParams {
    #[doc = "Set the field `authorized_hosts`.\n"]
    pub fn set_authorized_hosts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.authorized_hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chat_completions_endpoint`.\n"]
    pub fn set_chat_completions_endpoint(
        mut self,
        v: impl std::convert::Into<AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpoint>,
    ) -> Self {
        self.chat_completions_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_domains`.\nCustom domain hostnames that alias this public endpoint. GET and create responses return the current set; on update (PUT) this field is only echoed back when supplied in the request body, otherwise it is null (omit it to leave domains unchanged)."]
    pub fn set_custom_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_domain_enabled`.\nWhen false, the instance is reachable only via a registered custom domain and the default <public_endpoint_id>.search.ai.cloudflare.com host returns 404. Requires at least one custom domain. Defaults to true. public_endpoint_params is replaced wholesale on update, so resend default_domain_enabled on every update to keep the default host off — omitting it resets to true."]
    pub fn set_default_domain_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_domain_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instances_allowed`.\nInstance IDs exposed through the namespace public endpoint. Empty means nothing is searchable. Every ID must be an existing instance in this namespace, and the list cannot exceed the account's multi-instance search limit."]
    pub fn set_instances_allowed(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instances_allowed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mcp`.\n"]
    pub fn set_mcp(mut self, v: impl std::convert::Into<AiSearchNamespace_PublicEndpointParams_Mcp>) -> Self {
        self.mcp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_limit`.\n"]
    pub fn set_rate_limit(
        mut self,
        v: impl std::convert::Into<AiSearchNamespace_PublicEndpointParams_RateLimit>,
    ) -> Self {
        self.rate_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search_endpoint`.\n"]
    pub fn set_search_endpoint(
        mut self,
        v: impl std::convert::Into<AiSearchNamespace_PublicEndpointParams_SearchEndpoint>,
    ) -> Self {
        self.search_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiSearchNamespace_PublicEndpointParams {
    type O = terrars::BlockAssignable<AiSearchNamespace_PublicEndpointParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiSearchNamespace_PublicEndpointParams {}

impl BuildAiSearchNamespace_PublicEndpointParams {
    pub fn build(self) -> AiSearchNamespace_PublicEndpointParams {
        AiSearchNamespace_PublicEndpointParams {
            authorized_hosts: core::default::Default::default(),
            chat_completions_endpoint: core::default::Default::default(),
            custom_domains: core::default::Default::default(),
            default_domain_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            instances_allowed: core::default::Default::default(),
            mcp: core::default::Default::default(),
            rate_limit: core::default::Default::default(),
            search_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiSearchNamespace_PublicEndpointParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiSearchNamespace_PublicEndpointParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiSearchNamespace_PublicEndpointParamsRef {
        AiSearchNamespace_PublicEndpointParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiSearchNamespace_PublicEndpointParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorized_hosts` after provisioning.\n"]
    pub fn authorized_hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `chat_completions_endpoint` after provisioning.\n"]
    pub fn chat_completions_endpoint(&self) -> AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef {
        AiSearchNamespace_PublicEndpointParams_ChatCompletionsEndpointRef::new(
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
    pub fn mcp(&self) -> AiSearchNamespace_PublicEndpointParams_McpRef {
        AiSearchNamespace_PublicEndpointParams_McpRef::new(self.shared().clone(), format!("{}.mcp", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit` after provisioning.\n"]
    pub fn rate_limit(&self) -> AiSearchNamespace_PublicEndpointParams_RateLimitRef {
        AiSearchNamespace_PublicEndpointParams_RateLimitRef::new(
            self.shared().clone(),
            format!("{}.rate_limit", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `search_endpoint` after provisioning.\n"]
    pub fn search_endpoint(&self) -> AiSearchNamespace_PublicEndpointParams_SearchEndpointRef {
        AiSearchNamespace_PublicEndpointParams_SearchEndpointRef::new(
            self.shared().clone(),
            format!("{}.search_endpoint", self.base),
        )
    }
}
