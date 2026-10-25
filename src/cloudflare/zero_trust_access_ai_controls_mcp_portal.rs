#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustAccessAiControlsMcpPortalData_ {
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
    allow_code_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    hostname: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secure_web_gateway: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    servers: std::option::Option<std::vec::Vec<ZeroTrustAccessAiControlsMcpPortal_Servers>>,
}

struct ZeroTrustAccessAiControlsMcpPortal_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustAccessAiControlsMcpPortalData_>,
}

#[derive(Clone)]
pub struct ZeroTrustAccessAiControlsMcpPortal(std::rc::Rc<ZeroTrustAccessAiControlsMcpPortal_>);

impl ZeroTrustAccessAiControlsMcpPortal {
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
        "Set the field `allow_code_mode`.\nDeprecated: use `code_mode` for new integrations. `true` maps to any non-off Code Mode policy; `false` maps to `code_mode: off`. If both fields are sent, they must be consistent or the request returns a 400."]
    pub fn set_allow_code_mode(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_code_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `code_mode`.\nCode Mode policy for this portal. `off`: Code Mode is unavailable; query parameters are ignored. `opt_in`: Code Mode is off by default; clients turn it on with `?codemode=search_and_execute`. `default_on`: Code Mode is on by default; clients can opt out with `?codemode=off`. `enforced`: Code Mode is always on; query parameters are ignored. Defaults to `opt_in` when omitted on create. If both `code_mode` and `allow_code_mode` are sent, they must be consistent or the request returns a 400.\nAvailable values: \"off\", \"opt_in\", \"default_on\", \"enforced\"."]
    pub fn set_code_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().code_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional description of the MCP portal."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secure_web_gateway`.\nRoute outbound MCP traffic through Zero Trust Secure Web Gateway."]
    pub fn set_secure_web_gateway(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().secure_web_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `servers`.\nMCP servers attached to the portal and their portal-specific settings."]
    pub fn set_servers(
        self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessAiControlsMcpPortal_Servers>>,
    ) -> Self {
        self.0.data.borrow_mut().servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_code_mode` after provisioning.\nDeprecated: use `code_mode` for new integrations. `true` maps to any non-off Code Mode policy; `false` maps to `code_mode: off`. If both fields are sent, they must be consistent or the request returns a 400."]
    pub fn allow_code_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_code_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `code_mode` after provisioning.\nCode Mode policy for this portal. `off`: Code Mode is unavailable; query parameters are ignored. `opt_in`: Code Mode is off by default; clients turn it on with `?codemode=search_and_execute`. `default_on`: Code Mode is on by default; clients can opt out with `?codemode=off`. `enforced`: Code Mode is always on; query parameters are ignored. Defaults to `opt_in` when omitted on create. If both `code_mode` and `allow_code_mode` are sent, they must be consistent or the request returns a 400.\nAvailable values: \"off\", \"opt_in\", \"default_on\", \"enforced\"."]
    pub fn code_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_mode", self.extract_ref()))
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
        "Get a reference to the value of field `description` after provisioning.\nOptional description of the MCP portal."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostname where the MCP portal is available."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the MCP portal."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDisplay name for the MCP portal."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secure_web_gateway` after provisioning.\nRoute outbound MCP traffic through Zero Trust Secure Web Gateway."]
    pub fn secure_web_gateway(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure_web_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `servers` after provisioning.\nMCP servers attached to the portal and their portal-specific settings."]
    pub fn servers(&self) -> terrars::SetRef<ZeroTrustAccessAiControlsMcpPortal_ServersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.servers", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustAccessAiControlsMcpPortal {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustAccessAiControlsMcpPortal { }

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpPortal {
    type O = terrars::ListRef<ZeroTrustAccessAiControlsMcpPortalRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustAccessAiControlsMcpPortal_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_ai_controls_mcp_portal".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpPortal {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Hostname where the MCP portal is available."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Unique identifier for the MCP portal."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "Display name for the MCP portal."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessAiControlsMcpPortal {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustAccessAiControlsMcpPortal {
        let out = ZeroTrustAccessAiControlsMcpPortal(std::rc::Rc::new(ZeroTrustAccessAiControlsMcpPortal_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustAccessAiControlsMcpPortalData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                allow_code_mode: core::default::Default::default(),
                code_mode: core::default::Default::default(),
                description: core::default::Default::default(),
                hostname: self.hostname,
                id: self.id,
                name: self.name,
                secure_web_gateway: core::default::Default::default(),
                servers: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustAccessAiControlsMcpPortalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpPortalRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustAccessAiControlsMcpPortalRef {
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
        "Get a reference to the value of field `allow_code_mode` after provisioning.\nDeprecated: use `code_mode` for new integrations. `true` maps to any non-off Code Mode policy; `false` maps to `code_mode: off`. If both fields are sent, they must be consistent or the request returns a 400."]
    pub fn allow_code_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_code_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `code_mode` after provisioning.\nCode Mode policy for this portal. `off`: Code Mode is unavailable; query parameters are ignored. `opt_in`: Code Mode is off by default; clients turn it on with `?codemode=search_and_execute`. `default_on`: Code Mode is on by default; clients can opt out with `?codemode=off`. `enforced`: Code Mode is always on; query parameters are ignored. Defaults to `opt_in` when omitted on create. If both `code_mode` and `allow_code_mode` are sent, they must be consistent or the request returns a 400.\nAvailable values: \"off\", \"opt_in\", \"default_on\", \"enforced\"."]
    pub fn code_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_mode", self.extract_ref()))
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
        "Get a reference to the value of field `description` after provisioning.\nOptional description of the MCP portal."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostname where the MCP portal is available."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the MCP portal."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDisplay name for the MCP portal."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secure_web_gateway` after provisioning.\nRoute outbound MCP traffic through Zero Trust Secure Web Gateway."]
    pub fn secure_web_gateway(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure_web_gateway", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `servers` after provisioning.\nMCP servers attached to the portal and their portal-specific settings."]
    pub fn servers(&self) -> terrars::SetRef<ZeroTrustAccessAiControlsMcpPortal_ServersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.servers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPrompts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPrompts {
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

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPrompts {
    type O = terrars::BlockAssignable<ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPrompts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPrompts {
    #[doc = "Name of the tool or prompt capability to override."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPrompts {
    pub fn build(self) -> ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPrompts {
        ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPrompts {
            alias: core::default::Default::default(),
            description: core::default::Default::default(),
            enabled: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPromptsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPromptsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPromptsRef {
        ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPromptsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPromptsRef {
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
pub struct ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedTools {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
}

impl ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedTools {
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

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedTools {
    type O = terrars::BlockAssignable<ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedTools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedTools {
    #[doc = "Name of the tool or prompt capability to override."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedTools {
    pub fn build(self) -> ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedTools {
        ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedTools {
            alias: core::default::Default::default(),
            description: core::default::Default::default(),
            enabled: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedToolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedToolsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedToolsRef {
        ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedToolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedToolsRef {
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
pub struct ZeroTrustAccessAiControlsMcpPortal_Servers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_behalf: std::option::Option<terrars::PrimField<bool>>,
    server_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_prompts: std::option::Option<std::vec::Vec<ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPrompts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_tools: std::option::Option<std::vec::Vec<ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedTools>>,
}

impl ZeroTrustAccessAiControlsMcpPortal_Servers {
    #[doc =
        "Set the field `default_disabled`.\nDisable this server by default for clients connecting through the portal."]
    pub fn set_default_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_behalf`.\nUse end-user OAuth credentials when connecting this server to the portal."]
    pub fn set_on_behalf(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.on_behalf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_prompts`.\nPortal-specific prompt overrides."]
    pub fn set_updated_prompts(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPrompts>>,
    ) -> Self {
        self.updated_prompts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_tools`.\nPortal-specific tool overrides."]
    pub fn set_updated_tools(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedTools>>,
    ) -> Self {
        self.updated_tools = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessAiControlsMcpPortal_Servers {
    type O = terrars::BlockAssignable<ZeroTrustAccessAiControlsMcpPortal_Servers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessAiControlsMcpPortal_Servers {
    #[doc = "Unique identifier for the MCP server."]
    pub server_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustAccessAiControlsMcpPortal_Servers {
    pub fn build(self) -> ZeroTrustAccessAiControlsMcpPortal_Servers {
        ZeroTrustAccessAiControlsMcpPortal_Servers {
            default_disabled: core::default::Default::default(),
            on_behalf: core::default::Default::default(),
            server_id: self.server_id,
            updated_prompts: core::default::Default::default(),
            updated_tools: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessAiControlsMcpPortal_ServersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessAiControlsMcpPortal_ServersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessAiControlsMcpPortal_ServersRef {
        ZeroTrustAccessAiControlsMcpPortal_ServersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessAiControlsMcpPortal_ServersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_disabled` after provisioning.\nDisable this server by default for clients connecting through the portal."]
    pub fn default_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `on_behalf` after provisioning.\nUse end-user OAuth credentials when connecting this server to the portal."]
    pub fn on_behalf(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_behalf", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_id` after provisioning.\nUnique identifier for the MCP server."]
    pub fn server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_prompts` after provisioning.\nPortal-specific prompt overrides."]
    pub fn updated_prompts(&self) -> terrars::ListRef<ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedPromptsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_prompts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_tools` after provisioning.\nPortal-specific tool overrides."]
    pub fn updated_tools(&self) -> terrars::ListRef<ZeroTrustAccessAiControlsMcpPortal_Servers_UpdatedToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_tools", self.base))
    }
}
