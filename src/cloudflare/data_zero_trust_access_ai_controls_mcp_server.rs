#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustAccessAiControlsMcpServerData_ {
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
    filter: std::option::Option<DataZeroTrustAccessAiControlsMcpServer_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustAccessAiControlsMcpServer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustAccessAiControlsMcpServerData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustAccessAiControlsMcpServer(std::rc::Rc<DataZeroTrustAccessAiControlsMcpServer_>);

impl DataZeroTrustAccessAiControlsMcpServer {
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
    pub fn set_filter(self, v: impl std::convert::Into<DataZeroTrustAccessAiControlsMcpServer_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nUnique identifier for the MCP server."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_config_summary` after provisioning.\nSafe subset of auth_credentials surfaced to the dashboard. Includes auth_mode (dcr|manual), has_client_secret, client_secret_version, and the OAuth endpoints + client_id for manual servers. Never includes the secret value."]
    pub fn auth_config_summary(&self) -> DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
        DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef::new(
            self.shared().clone(),
            format!("{}.auth_config_summary", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `auth_type` after provisioning.\nAuthentication method used to connect to the upstream MCP server.\nAvailable values: \"oauth\", \"bearer\", \"unauthenticated\"."]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authentication_status` after provisioning.\nWhether administrative authentication is required before capabilities can be synced. Manual OAuth is user-managed and has no administrative authentication flow.\nAvailable values: \"not_required\", \"required\", \"connected\", \"stale\", \"manual\"."]
    pub fn authentication_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional description of the MCP server."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(&self) -> DataZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
        DataZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef::new(
            self.shared().clone(),
            format!("{}.error_details", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustAccessAiControlsMcpServer_FilterRef {
        DataZeroTrustAccessAiControlsMcpServer_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nURL of the upstream MCP endpoint."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the MCP server."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_shared_oauth_callback_enabled` after provisioning.\nWhen true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. New public server creates default to true; existing servers default to false from migration until explicitly updated. Effective behavior is gated by the gateway worker's per-env rollout mode KV key."]
    pub fn is_shared_oauth_callback_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_shared_oauth_callback_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_successful_sync` after provisioning.\n"]
    pub fn last_successful_sync(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_successful_sync", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_synced` after provisioning.\n"]
    pub fn last_synced(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_synced", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDisplay name for the MCP server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prompts` after provisioning.\n"]
    pub fn prompts(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secure_web_gateway` after provisioning.\nRoute outbound traffic to this MCP server through Zero Trust Secure Web Gateway."]
    pub fn secure_web_gateway(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure_web_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nCurrent sync state of the server\nAvailable values: \"waiting\", \"ready\", \"stale\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tools` after provisioning.\n"]
    pub fn tools(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_prompts` after provisioning.\nServer-wide prompt capability overrides."]
    pub fn updated_prompts(&self) -> terrars::ListRef<DataZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_prompts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_tools` after provisioning.\nServer-wide tool capability overrides."]
    pub fn updated_tools(&self) -> terrars::ListRef<DataZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_tools", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustAccessAiControlsMcpServer {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustAccessAiControlsMcpServer { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServer {
    type O = terrars::ListRef<DataZeroTrustAccessAiControlsMcpServerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustAccessAiControlsMcpServer_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_ai_controls_mcp_server".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServer {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustAccessAiControlsMcpServer {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustAccessAiControlsMcpServer {
        let out = DataZeroTrustAccessAiControlsMcpServer(std::rc::Rc::new(DataZeroTrustAccessAiControlsMcpServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustAccessAiControlsMcpServerData_ {
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

pub struct DataZeroTrustAccessAiControlsMcpServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServerRef {
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
        "Get a reference to the value of field `auth_config_summary` after provisioning.\nSafe subset of auth_credentials surfaced to the dashboard. Includes auth_mode (dcr|manual), has_client_secret, client_secret_version, and the OAuth endpoints + client_id for manual servers. Never includes the secret value."]
    pub fn auth_config_summary(&self) -> DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
        DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef::new(
            self.shared().clone(),
            format!("{}.auth_config_summary", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `auth_type` after provisioning.\nAuthentication method used to connect to the upstream MCP server.\nAvailable values: \"oauth\", \"bearer\", \"unauthenticated\"."]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authentication_status` after provisioning.\nWhether administrative authentication is required before capabilities can be synced. Manual OAuth is user-managed and has no administrative authentication flow.\nAvailable values: \"not_required\", \"required\", \"connected\", \"stale\", \"manual\"."]
    pub fn authentication_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional description of the MCP server."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(&self) -> DataZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
        DataZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef::new(
            self.shared().clone(),
            format!("{}.error_details", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustAccessAiControlsMcpServer_FilterRef {
        DataZeroTrustAccessAiControlsMcpServer_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nURL of the upstream MCP endpoint."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the MCP server."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_shared_oauth_callback_enabled` after provisioning.\nWhen true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. New public server creates default to true; existing servers default to false from migration until explicitly updated. Effective behavior is gated by the gateway worker's per-env rollout mode KV key."]
    pub fn is_shared_oauth_callback_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_shared_oauth_callback_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_successful_sync` after provisioning.\n"]
    pub fn last_successful_sync(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_successful_sync", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_synced` after provisioning.\n"]
    pub fn last_synced(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_synced", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDisplay name for the MCP server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prompts` after provisioning.\n"]
    pub fn prompts(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secure_web_gateway` after provisioning.\nRoute outbound traffic to this MCP server through Zero Trust Secure Web Gateway."]
    pub fn secure_web_gateway(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure_web_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nCurrent sync state of the server\nAvailable values: \"waiting\", \"ready\", \"stale\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tools` after provisioning.\n"]
    pub fn tools(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_prompts` after provisioning.\nServer-wide prompt capability overrides."]
    pub fn updated_prompts(&self) -> terrars::ListRef<DataZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_prompts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_tools` after provisioning.\nServer-wide tool capability overrides."]
    pub fn updated_tools(&self) -> terrars::ListRef<DataZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_tools", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {}

impl DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {}

impl BuildDataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {
        DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
        DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `revocation_endpoint` after provisioning.\n"]
    pub fn revocation_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revocation_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {}

impl DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {}

impl BuildDataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {
        DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
        DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_uris` after provisioning.\n"]
    pub fn redirect_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_uris", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint_auth_method` after provisioning.\n"]
    pub fn token_endpoint_auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint_auth_method", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {}

impl DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {}

impl BuildDataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {
        DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
        DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_mode` after provisioning.\nAvailable values: \"dcr\", \"manual\"."]
    pub fn auth_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_version` after provisioning.\n"]
    pub fn client_secret_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
        DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `has_client_secret` after provisioning.\n"]
    pub fn has_client_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `registration_info` after provisioning.\n"]
    pub fn registration_info(&self) -> DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
        DataZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef::new(
            self.shared().clone(),
            format!("{}.registration_info", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServer_ErrorDetails {}

impl DataZeroTrustAccessAiControlsMcpServer_ErrorDetails { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServer_ErrorDetails {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServer_ErrorDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServer_ErrorDetails {}

impl BuildDataZeroTrustAccessAiControlsMcpServer_ErrorDetails {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServer_ErrorDetails {
        DataZeroTrustAccessAiControlsMcpServer_ErrorDetails {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
        DataZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cause` after provisioning.\nUnderlying error message"]
    pub fn cause(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cause", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_upstream` after provisioning.\nTrue = MCP server returned an error. False = couldn't reach the server"]
    pub fn is_upstream(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_upstream", self.base))
    }

    #[doc = "Get a reference to the value of field `mcp_code` after provisioning.\nMCP protocol error code"]
    pub fn mcp_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mcp_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retryable` after provisioning.\nWhether the error is transient and worth retrying"]
    pub fn retryable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retryable", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\nHTTP status code from the server"]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServer_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustAccessAiControlsMcpServer_Filter {
    #[doc = "Set the field `search`.\nSearch by id, name"]
    pub fn set_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServer_Filter {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServer_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServer_Filter {}

impl BuildDataZeroTrustAccessAiControlsMcpServer_Filter {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServer_Filter {
        DataZeroTrustAccessAiControlsMcpServer_Filter { search: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServer_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServer_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServer_FilterRef {
        DataZeroTrustAccessAiControlsMcpServer_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServer_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\nSearch by id, name"]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {}

impl DataZeroTrustAccessAiControlsMcpServer_UpdatedPrompts { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServer_UpdatedPrompts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {}

impl BuildDataZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {
        DataZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef {
        DataZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alias` after provisioning.\nCustom name exposed for the capability."]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nCustom description exposed for the capability."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the capability is available through the MCP server."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the tool or prompt capability to override."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServer_UpdatedTools {}

impl DataZeroTrustAccessAiControlsMcpServer_UpdatedTools { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServer_UpdatedTools {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServer_UpdatedTools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServer_UpdatedTools {}

impl BuildDataZeroTrustAccessAiControlsMcpServer_UpdatedTools {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServer_UpdatedTools {
        DataZeroTrustAccessAiControlsMcpServer_UpdatedTools {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef {
        DataZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alias` after provisioning.\nCustom name exposed for the capability."]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nCustom description exposed for the capability."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the capability is available through the MCP server."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the tool or prompt capability to override."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
