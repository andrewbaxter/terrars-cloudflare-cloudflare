#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustAccessAiControlsMcpServerData_ {
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
    auth_credentials: std::option::Option<terrars::PrimField<std::string::String>>,
    auth_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    hostname: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_shared_oauth_callback_enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secure_web_gateway: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_prompts: std::option::Option<std::vec::Vec<ZeroTrustAccessAiControlsMcpServer_UpdatedPrompts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_tools: std::option::Option<std::vec::Vec<ZeroTrustAccessAiControlsMcpServer_UpdatedTools>>,
}

struct ZeroTrustAccessAiControlsMcpServer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustAccessAiControlsMcpServerData_>,
}

#[derive(Clone)]
pub struct ZeroTrustAccessAiControlsMcpServer(std::rc::Rc<ZeroTrustAccessAiControlsMcpServer_>);

impl ZeroTrustAccessAiControlsMcpServer {
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
        "Set the field `auth_credentials`.\nCredential configuration for the upstream MCP server. For auth_type \"bearer\", either a raw token string (e.g. \"sk-abc123\"), which is wrapped server-side as `Authorization: Bearer <token>`, or a JSON-encoded object of the form `{\"headers\":{\"Header-Name\":\"value\",...}}` for custom or multiple static headers (e.g. Cloudflare Access service tokens: `{\"headers\":{\"cf-access-client-id\":\"...\",\"cf-access-client-secret\":\"...\"}}`). For auth_type \"oauth\" with a pre-registered client, send a JSON-encoded object containing `auth_mode:\"manual\"`, `config.authorization_endpoint`, `config.token_endpoint`, and `registration_info.client_id`. Also provide `registration_info.redirect_uris` unless `is_shared_oauth_callback_enabled` is true. Optional fields include `config.issuer`, `config.revocation_endpoint`, `registration_info.scope`, and `registration_info.token_endpoint_auth_method`. Send the client secret in the separate `client_secret` field. Omit `auth_credentials` on update to preserve the existing configuration."]
    pub fn set_auth_credentials(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().auth_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_secret`.\nPre-registered OAuth client_secret. Write-only - accepted on create/update when auth_credentials.auth_mode is 'manual'. Required when creating a manual OAuth server or converting an existing server to manual mode. Omit it on update to preserve the existing secret; provide it to rotate the secret. Stored AES-GCM-encrypted in server_oauth_secrets; never returned by read endpoints."]
    pub fn set_client_secret(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional description of the MCP server."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_shared_oauth_callback_enabled`.\nWhen true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true."]
    pub fn set_is_shared_oauth_callback_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_shared_oauth_callback_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secure_web_gateway`.\nRoute outbound traffic to this MCP server through Zero Trust Secure Web Gateway."]
    pub fn set_secure_web_gateway(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().secure_web_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_prompts`.\nServer-wide prompt capability overrides."]
    pub fn set_updated_prompts(
        self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessAiControlsMcpServer_UpdatedPrompts>>,
    ) -> Self {
        self.0.data.borrow_mut().updated_prompts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_tools`.\nServer-wide tool capability overrides."]
    pub fn set_updated_tools(
        self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessAiControlsMcpServer_UpdatedTools>>,
    ) -> Self {
        self.0.data.borrow_mut().updated_tools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_config_summary` after provisioning.\nSafe subset of auth_credentials surfaced to the dashboard. Includes auth_mode (dcr|manual), has_client_secret, client_secret_version, and the OAuth endpoints + client_id for manual servers. Never includes the secret value."]
    pub fn auth_config_summary(&self) -> ZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
        ZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef::new(
            self.shared().clone(),
            format!("{}.auth_config_summary", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `auth_credentials` after provisioning.\nCredential configuration for the upstream MCP server. For auth_type \"bearer\", either a raw token string (e.g. \"sk-abc123\"), which is wrapped server-side as `Authorization: Bearer <token>`, or a JSON-encoded object of the form `{\"headers\":{\"Header-Name\":\"value\",...}}` for custom or multiple static headers (e.g. Cloudflare Access service tokens: `{\"headers\":{\"cf-access-client-id\":\"...\",\"cf-access-client-secret\":\"...\"}}`). For auth_type \"oauth\" with a pre-registered client, send a JSON-encoded object containing `auth_mode:\"manual\"`, `config.authorization_endpoint`, `config.token_endpoint`, and `registration_info.client_id`. Also provide `registration_info.redirect_uris` unless `is_shared_oauth_callback_enabled` is true. Optional fields include `config.issuer`, `config.revocation_endpoint`, `registration_info.scope`, and `registration_info.token_endpoint_auth_method`. Send the client secret in the separate `client_secret` field. Omit `auth_credentials` on update to preserve the existing configuration."]
    pub fn auth_credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_credentials", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nPre-registered OAuth client_secret. Write-only - accepted on create/update when auth_credentials.auth_mode is 'manual'. Required when creating a manual OAuth server or converting an existing server to manual mode. Omit it on update to preserve the existing secret; provide it to rotate the secret. Stored AES-GCM-encrypted in server_oauth_secrets; never returned by read endpoints."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.extract_ref()))
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
    pub fn error_details(&self) -> ZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
        ZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef::new(
            self.shared().clone(),
            format!("{}.error_details", self.extract_ref()),
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
        "Get a reference to the value of field `is_shared_oauth_callback_enabled` after provisioning.\nWhen true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true."]
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tools` after provisioning.\n"]
    pub fn tools(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_prompts` after provisioning.\nServer-wide prompt capability overrides."]
    pub fn updated_prompts(&self) -> terrars::ListRef<ZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_prompts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_tools` after provisioning.\nServer-wide tool capability overrides."]
    pub fn updated_tools(&self) -> terrars::ListRef<ZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_tools", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustAccessAiControlsMcpServer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustAccessAiControlsMcpServer { }

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpServer {
    type O = terrars::ListRef<ZeroTrustAccessAiControlsMcpServerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustAccessAiControlsMcpServer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_ai_controls_mcp_server".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpServer {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Authentication method used to connect to the upstream MCP server.\nAvailable values: \"oauth\", \"bearer\", \"unauthenticated\"."]
    pub auth_type: terrars::PrimField<std::string::String>,
    #[doc = "URL of the upstream MCP endpoint."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Unique identifier for the MCP server."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "Display name for the MCP server."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessAiControlsMcpServer {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustAccessAiControlsMcpServer {
        let out = ZeroTrustAccessAiControlsMcpServer(std::rc::Rc::new(ZeroTrustAccessAiControlsMcpServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustAccessAiControlsMcpServerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                auth_credentials: core::default::Default::default(),
                auth_type: self.auth_type,
                client_secret: core::default::Default::default(),
                description: core::default::Default::default(),
                hostname: self.hostname,
                id: self.id,
                is_shared_oauth_callback_enabled: core::default::Default::default(),
                name: self.name,
                secure_web_gateway: core::default::Default::default(),
                updated_prompts: core::default::Default::default(),
                updated_tools: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustAccessAiControlsMcpServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustAccessAiControlsMcpServerRef {
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
        "Get a reference to the value of field `auth_config_summary` after provisioning.\nSafe subset of auth_credentials surfaced to the dashboard. Includes auth_mode (dcr|manual), has_client_secret, client_secret_version, and the OAuth endpoints + client_id for manual servers. Never includes the secret value."]
    pub fn auth_config_summary(&self) -> ZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
        ZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef::new(
            self.shared().clone(),
            format!("{}.auth_config_summary", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `auth_credentials` after provisioning.\nCredential configuration for the upstream MCP server. For auth_type \"bearer\", either a raw token string (e.g. \"sk-abc123\"), which is wrapped server-side as `Authorization: Bearer <token>`, or a JSON-encoded object of the form `{\"headers\":{\"Header-Name\":\"value\",...}}` for custom or multiple static headers (e.g. Cloudflare Access service tokens: `{\"headers\":{\"cf-access-client-id\":\"...\",\"cf-access-client-secret\":\"...\"}}`). For auth_type \"oauth\" with a pre-registered client, send a JSON-encoded object containing `auth_mode:\"manual\"`, `config.authorization_endpoint`, `config.token_endpoint`, and `registration_info.client_id`. Also provide `registration_info.redirect_uris` unless `is_shared_oauth_callback_enabled` is true. Optional fields include `config.issuer`, `config.revocation_endpoint`, `registration_info.scope`, and `registration_info.token_endpoint_auth_method`. Send the client secret in the separate `client_secret` field. Omit `auth_credentials` on update to preserve the existing configuration."]
    pub fn auth_credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_credentials", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nPre-registered OAuth client_secret. Write-only - accepted on create/update when auth_credentials.auth_mode is 'manual'. Required when creating a manual OAuth server or converting an existing server to manual mode. Omit it on update to preserve the existing secret; provide it to rotate the secret. Stored AES-GCM-encrypted in server_oauth_secrets; never returned by read endpoints."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.extract_ref()))
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
    pub fn error_details(&self) -> ZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
        ZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef::new(
            self.shared().clone(),
            format!("{}.error_details", self.extract_ref()),
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
        "Get a reference to the value of field `is_shared_oauth_callback_enabled` after provisioning.\nWhen true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. Defaults to false (off); opt in per server by setting true."]
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tools` after provisioning.\n"]
    pub fn tools(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_prompts` after provisioning.\nServer-wide prompt capability overrides."]
    pub fn updated_prompts(&self) -> terrars::ListRef<ZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_prompts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_tools` after provisioning.\nServer-wide tool capability overrides."]
    pub fn updated_tools(&self) -> terrars::ListRef<ZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_tools", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {}

impl ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config { }

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {
    type O = terrars::BlockAssignable<ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {}

impl BuildZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {
    pub fn build(self) -> ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {
        ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_Config {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
        ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
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
pub struct ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {}

impl ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo { }

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {
    type O = terrars::BlockAssignable<ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {}

impl BuildZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {
    pub fn build(self) -> ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {
        ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfo {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
        ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
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
pub struct ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {}

impl ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary { }

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {
    type O = terrars::BlockAssignable<ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {}

impl BuildZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {
    pub fn build(self) -> ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {
        ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
        ZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessAiControlsMcpServer_AuthConfigSummaryRef {
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
    pub fn config(&self) -> ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef {
        ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `has_client_secret` after provisioning.\n"]
    pub fn has_client_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `registration_info` after provisioning.\n"]
    pub fn registration_info(&self) -> ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef {
        ZeroTrustAccessAiControlsMcpServer_AuthConfigSummary_RegistrationInfoRef::new(
            self.shared().clone(),
            format!("{}.registration_info", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessAiControlsMcpServer_ErrorDetails {}

impl ZeroTrustAccessAiControlsMcpServer_ErrorDetails { }

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpServer_ErrorDetails {
    type O = terrars::BlockAssignable<ZeroTrustAccessAiControlsMcpServer_ErrorDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpServer_ErrorDetails {}

impl BuildZeroTrustAccessAiControlsMcpServer_ErrorDetails {
    pub fn build(self) -> ZeroTrustAccessAiControlsMcpServer_ErrorDetails {
        ZeroTrustAccessAiControlsMcpServer_ErrorDetails {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
        ZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessAiControlsMcpServer_ErrorDetailsRef {
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
pub struct ZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {
    #[doc = "Set the field `alias`.\nCustom name exposed for the capability."]
    pub fn set_alias(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nCustom description exposed for the capability."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether the capability is available through the MCP server."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {
    type O = terrars::BlockAssignable<ZeroTrustAccessAiControlsMcpServer_UpdatedPrompts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {
    #[doc = "Name of the tool or prompt capability to override."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {
    pub fn build(self) -> ZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {
        ZeroTrustAccessAiControlsMcpServer_UpdatedPrompts {
            alias: core::default::Default::default(),
            description: core::default::Default::default(),
            enabled: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef {
        ZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessAiControlsMcpServer_UpdatedPromptsRef {
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
pub struct ZeroTrustAccessAiControlsMcpServer_UpdatedTools {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessAiControlsMcpServer_UpdatedTools {
    #[doc = "Set the field `alias`.\nCustom name exposed for the capability."]
    pub fn set_alias(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nCustom description exposed for the capability."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether the capability is available through the MCP server."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpServer_UpdatedTools {
    type O = terrars::BlockAssignable<ZeroTrustAccessAiControlsMcpServer_UpdatedTools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpServer_UpdatedTools {
    #[doc = "Name of the tool or prompt capability to override."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessAiControlsMcpServer_UpdatedTools {
    pub fn build(self) -> ZeroTrustAccessAiControlsMcpServer_UpdatedTools {
        ZeroTrustAccessAiControlsMcpServer_UpdatedTools {
            alias: core::default::Default::default(),
            description: core::default::Default::default(),
            enabled: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef {
        ZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessAiControlsMcpServer_UpdatedToolsRef {
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
