#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustCasbIntegrationData_ {
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
    anthropic: std::option::Option<ZeroTrustCasbIntegration_Anthropic>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws: std::option::Option<ZeroTrustCasbIntegration_Aws>,
    #[serde(rename = "box", skip_serializing_if = "std::option::Option::is_none")]
    box_: std::option::Option<ZeroTrustCasbIntegration_Box>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dlp_profiles: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_platform: std::option::Option<ZeroTrustCasbIntegration_GoogleCloudPlatform>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_workspace: std::option::Option<ZeroTrustCasbIntegration_GoogleWorkspace>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    openai: std::option::Option<ZeroTrustCasbIntegration_Openai>,
    paused: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_cases: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

struct ZeroTrustCasbIntegration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustCasbIntegrationData_>,
}

#[derive(Clone)]
pub struct ZeroTrustCasbIntegration(std::rc::Rc<ZeroTrustCasbIntegration_>);

impl ZeroTrustCasbIntegration {
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

    #[doc = "Set the field `anthropic`.\nAnthropic integration configuration."]
    pub fn set_anthropic(self, v: impl std::convert::Into<ZeroTrustCasbIntegration_Anthropic>) -> Self {
        self.0.data.borrow_mut().anthropic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws`.\nAWS integration configuration."]
    pub fn set_aws(self, v: impl std::convert::Into<ZeroTrustCasbIntegration_Aws>) -> Self {
        self.0.data.borrow_mut().aws = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `box_`.\nBox integration configuration."]
    pub fn set_box(self, v: impl std::convert::Into<ZeroTrustCasbIntegration_Box>) -> Self {
        self.0.data.borrow_mut().box_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dlp_profiles`.\nDLP profile IDs to associate with the integration."]
    pub fn set_dlp_profiles(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().dlp_profiles = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_cloud_platform`.\nGoogle Cloud Platform integration configuration."]
    pub fn set_google_cloud_platform(
        self,
        v: impl std::convert::Into<ZeroTrustCasbIntegration_GoogleCloudPlatform>,
    ) -> Self {
        self.0.data.borrow_mut().google_cloud_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_workspace`.\nGoogle Workspace integration configuration."]
    pub fn set_google_workspace(self, v: impl std::convert::Into<ZeroTrustCasbIntegration_GoogleWorkspace>) -> Self {
        self.0.data.borrow_mut().google_workspace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `openai`.\nOpenAI integration configuration."]
    pub fn set_openai(self, v: impl std::convert::Into<ZeroTrustCasbIntegration_Openai>) -> Self {
        self.0.data.borrow_mut().openai = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permissions`.\nPermission scopes granted to the integration."]
    pub fn set_permissions(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_cases`.\nUse cases to enroll the integration in."]
    pub fn set_use_cases(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().use_cases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `anthropic` after provisioning.\nAnthropic integration configuration."]
    pub fn anthropic(&self) -> ZeroTrustCasbIntegration_AnthropicRef {
        ZeroTrustCasbIntegration_AnthropicRef::new(
            self.shared().clone(),
            format!("{}.anthropic", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `aws` after provisioning.\nAWS integration configuration."]
    pub fn aws(&self) -> ZeroTrustCasbIntegration_AwsRef {
        ZeroTrustCasbIntegration_AwsRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `box_` after provisioning.\nBox integration configuration."]
    pub fn box_(&self) -> ZeroTrustCasbIntegration_BoxRef {
        ZeroTrustCasbIntegration_BoxRef::new(self.shared().clone(), format!("{}.box", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dlp_profiles` after provisioning.\nDLP profile IDs to associate with the integration."]
    pub fn dlp_profiles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dlp_profiles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `google_cloud_platform` after provisioning.\nGoogle Cloud Platform integration configuration."]
    pub fn google_cloud_platform(&self) -> ZeroTrustCasbIntegration_GoogleCloudPlatformRef {
        ZeroTrustCasbIntegration_GoogleCloudPlatformRef::new(
            self.shared().clone(),
            format!("{}.google_cloud_platform", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `google_workspace` after provisioning.\nGoogle Workspace integration configuration."]
    pub fn google_workspace(&self) -> ZeroTrustCasbIntegration_GoogleWorkspaceRef {
        ZeroTrustCasbIntegration_GoogleWorkspaceRef::new(
            self.shared().clone(),
            format!("{}.google_workspace", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIntegration ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the integration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openai` after provisioning.\nOpenAI integration configuration."]
    pub fn openai(&self) -> ZeroTrustCasbIntegration_OpenaiRef {
        ZeroTrustCasbIntegration_OpenaiRef::new(self.shared().clone(), format!("{}.openai", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `paused` after provisioning.\nWhether the integration is paused."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nPermission scopes granted to the integration."]
    pub fn permissions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secrets_digest` after provisioning.\nSecure digest of the confidential arguments. Managed by the provider."]
    pub fn secrets_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_digest", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `use_cases` after provisioning.\nUse cases to enroll the integration in."]
    pub fn use_cases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.use_cases", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustCasbIntegration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustCasbIntegration { }

impl terrars::ToListMappable for ZeroTrustCasbIntegration {
    type O = terrars::ListRef<ZeroTrustCasbIntegrationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustCasbIntegration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_casb_integration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustCasbIntegration {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the integration."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Whether the integration is paused."]
    pub paused: terrars::PrimField<bool>,
}

impl BuildZeroTrustCasbIntegration {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustCasbIntegration {
        let out = ZeroTrustCasbIntegration(std::rc::Rc::new(ZeroTrustCasbIntegration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustCasbIntegrationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                anthropic: core::default::Default::default(),
                aws: core::default::Default::default(),
                box_: core::default::Default::default(),
                dlp_profiles: core::default::Default::default(),
                google_cloud_platform: core::default::Default::default(),
                google_workspace: core::default::Default::default(),
                name: self.name,
                openai: core::default::Default::default(),
                paused: self.paused,
                permissions: core::default::Default::default(),
                use_cases: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustCasbIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustCasbIntegrationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `anthropic` after provisioning.\nAnthropic integration configuration."]
    pub fn anthropic(&self) -> ZeroTrustCasbIntegration_AnthropicRef {
        ZeroTrustCasbIntegration_AnthropicRef::new(
            self.shared().clone(),
            format!("{}.anthropic", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `aws` after provisioning.\nAWS integration configuration."]
    pub fn aws(&self) -> ZeroTrustCasbIntegration_AwsRef {
        ZeroTrustCasbIntegration_AwsRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `box_` after provisioning.\nBox integration configuration."]
    pub fn box_(&self) -> ZeroTrustCasbIntegration_BoxRef {
        ZeroTrustCasbIntegration_BoxRef::new(self.shared().clone(), format!("{}.box", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dlp_profiles` after provisioning.\nDLP profile IDs to associate with the integration."]
    pub fn dlp_profiles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dlp_profiles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `google_cloud_platform` after provisioning.\nGoogle Cloud Platform integration configuration."]
    pub fn google_cloud_platform(&self) -> ZeroTrustCasbIntegration_GoogleCloudPlatformRef {
        ZeroTrustCasbIntegration_GoogleCloudPlatformRef::new(
            self.shared().clone(),
            format!("{}.google_cloud_platform", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `google_workspace` after provisioning.\nGoogle Workspace integration configuration."]
    pub fn google_workspace(&self) -> ZeroTrustCasbIntegration_GoogleWorkspaceRef {
        ZeroTrustCasbIntegration_GoogleWorkspaceRef::new(
            self.shared().clone(),
            format!("{}.google_workspace", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIntegration ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the integration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openai` after provisioning.\nOpenAI integration configuration."]
    pub fn openai(&self) -> ZeroTrustCasbIntegration_OpenaiRef {
        ZeroTrustCasbIntegration_OpenaiRef::new(self.shared().clone(), format!("{}.openai", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `paused` after provisioning.\nWhether the integration is paused."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nPermission scopes granted to the integration."]
    pub fn permissions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secrets_digest` after provisioning.\nSecure digest of the confidential arguments. Managed by the provider."]
    pub fn secrets_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_digest", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `use_cases` after provisioning.\nUse cases to enroll the integration in."]
    pub fn use_cases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.use_cases", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKey {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKey {
    #[doc = "Set the field `tenant_id`.\nOrganization ID. Auto-extracted from the key if not provided."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKey {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKey {
    #[doc = "Anthropic Admin API key. This value is write-only and is never persisted to Terraform state."]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKey {
    pub fn build(self) -> ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKey {
        ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKey {
            api_key: self.api_key,
            tenant_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKeyRef {
        ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key` after provisioning.\nAnthropic Admin API key. This value is write-only and is never persisted to Terraform state."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_id` after provisioning.\nOrganization ID. Auto-extracted from the key if not provided."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKey {
    compliance_api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKey {
    #[doc = "Set the field `tenant_id`.\nOrganization ID. Auto-extracted from the key if not provided."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKey {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKey {
    #[doc = "Anthropic Compliance API key. This value is write-only and is never persisted to Terraform state."]
    pub compliance_api_key: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKey {
    pub fn build(self) -> ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKey {
        ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKey {
            compliance_api_key: self.compliance_api_key,
            tenant_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKeyRef {
        ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compliance_api_key` after provisioning.\nAnthropic Compliance API key. This value is write-only and is never persisted to Terraform state."]
    pub fn compliance_api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_api_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_id` after provisioning.\nOrganization ID. Auto-extracted from the key if not provided."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKey {
    api_key: terrars::PrimField<std::string::String>,
    tenant_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKey { }

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKey {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKey {
    #[doc = "Anthropic Workspace API key. This value is write-only and is never persisted to Terraform state."]
    pub api_key: terrars::PrimField<std::string::String>,
    #[doc = "Workspace ID, found in the Anthropic Console URL after /workspaces/."]
    pub tenant_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKey {
    pub fn build(self) -> ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKey {
        ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKey {
            api_key: self.api_key,
            tenant_id: self.tenant_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKeyRef {
        ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key` after provisioning.\nAnthropic Workspace API key. This value is write-only and is never persisted to Terraform state."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_id` after provisioning.\nWorkspace ID, found in the Anthropic Console URL after /workspaces/."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Anthropic {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthropic_admin_api_key: std::option::Option<ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKey>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthropic_compliance_api_key: std::option::Option<ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKey>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthropic_workspace_api_key: std::option::Option<ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKey>,
}

impl ZeroTrustCasbIntegration_Anthropic {
    #[doc = "Set the field `anthropic_admin_api_key`.\nAuthenticate with an Anthropic Admin API key."]
    pub fn set_anthropic_admin_api_key(
        mut self,
        v: impl std::convert::Into<ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKey>,
    ) -> Self {
        self.anthropic_admin_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anthropic_compliance_api_key`.\nAuthenticate with an Anthropic Compliance API key."]
    pub fn set_anthropic_compliance_api_key(
        mut self,
        v: impl std::convert::Into<ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKey>,
    ) -> Self {
        self.anthropic_compliance_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anthropic_workspace_api_key`.\nAuthenticate with an Anthropic Workspace API key."]
    pub fn set_anthropic_workspace_api_key(
        mut self,
        v: impl std::convert::Into<ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKey>,
    ) -> Self {
        self.anthropic_workspace_api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Anthropic {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Anthropic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Anthropic {}

impl BuildZeroTrustCasbIntegration_Anthropic {
    pub fn build(self) -> ZeroTrustCasbIntegration_Anthropic {
        ZeroTrustCasbIntegration_Anthropic {
            anthropic_admin_api_key: core::default::Default::default(),
            anthropic_compliance_api_key: core::default::Default::default(),
            anthropic_workspace_api_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_AnthropicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_AnthropicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustCasbIntegration_AnthropicRef {
        ZeroTrustCasbIntegration_AnthropicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_AnthropicRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `anthropic_admin_api_key` after provisioning.\nAuthenticate with an Anthropic Admin API key."]
    pub fn anthropic_admin_api_key(&self) -> ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKeyRef {
        ZeroTrustCasbIntegration_Anthropic_AnthropicAdminApiKeyRef::new(
            self.shared().clone(),
            format!("{}.anthropic_admin_api_key", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `anthropic_compliance_api_key` after provisioning.\nAuthenticate with an Anthropic Compliance API key."]
    pub fn anthropic_compliance_api_key(&self) -> ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKeyRef {
        ZeroTrustCasbIntegration_Anthropic_AnthropicComplianceApiKeyRef::new(
            self.shared().clone(),
            format!("{}.anthropic_compliance_api_key", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `anthropic_workspace_api_key` after provisioning.\nAuthenticate with an Anthropic Workspace API key."]
    pub fn anthropic_workspace_api_key(&self) -> ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKeyRef {
        ZeroTrustCasbIntegration_Anthropic_AnthropicWorkspaceApiKeyRef::new(
            self.shared().clone(),
            format!("{}.anthropic_workspace_api_key", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Aws_AwsIamRole {
    external_id: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl ZeroTrustCasbIntegration_Aws_AwsIamRole { }

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Aws_AwsIamRole {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Aws_AwsIamRole>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Aws_AwsIamRole {
    #[doc = "External ID required when assuming the IAM role."]
    pub external_id: terrars::PrimField<std::string::String>,
    #[doc = "ARN of the cross-account IAM role Cloudflare will assume."]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbIntegration_Aws_AwsIamRole {
    pub fn build(self) -> ZeroTrustCasbIntegration_Aws_AwsIamRole {
        ZeroTrustCasbIntegration_Aws_AwsIamRole {
            external_id: self.external_id,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_Aws_AwsIamRoleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_Aws_AwsIamRoleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustCasbIntegration_Aws_AwsIamRoleRef {
        ZeroTrustCasbIntegration_Aws_AwsIamRoleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_Aws_AwsIamRoleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_id` after provisioning.\nExternal ID required when assuming the IAM role."]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `role_arn` after provisioning.\nARN of the cross-account IAM role Cloudflare will assume."]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Aws {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iam_role: std::option::Option<ZeroTrustCasbIntegration_Aws_AwsIamRole>,
}

impl ZeroTrustCasbIntegration_Aws {
    #[doc = "Set the field `aws_iam_role`.\nAuthenticate by delegating to a cross-account IAM role."]
    pub fn set_aws_iam_role(mut self, v: impl std::convert::Into<ZeroTrustCasbIntegration_Aws_AwsIamRole>) -> Self {
        self.aws_iam_role = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Aws {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Aws>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Aws {}

impl BuildZeroTrustCasbIntegration_Aws {
    pub fn build(self) -> ZeroTrustCasbIntegration_Aws {
        ZeroTrustCasbIntegration_Aws { aws_iam_role: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_AwsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_AwsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustCasbIntegration_AwsRef {
        ZeroTrustCasbIntegration_AwsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_AwsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aws_iam_role` after provisioning.\nAuthenticate by delegating to a cross-account IAM role."]
    pub fn aws_iam_role(&self) -> ZeroTrustCasbIntegration_Aws_AwsIamRoleRef {
        ZeroTrustCasbIntegration_Aws_AwsIamRoleRef::new(self.shared().clone(), format!("{}.aws_iam_role", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Box_BoxServerAuthentication {
    enterprise_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustCasbIntegration_Box_BoxServerAuthentication { }

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Box_BoxServerAuthentication {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Box_BoxServerAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Box_BoxServerAuthentication {
    #[doc = "Box Enterprise ID from Admin Console > Accounts & Billing."]
    pub enterprise_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbIntegration_Box_BoxServerAuthentication {
    pub fn build(self) -> ZeroTrustCasbIntegration_Box_BoxServerAuthentication {
        ZeroTrustCasbIntegration_Box_BoxServerAuthentication { enterprise_id: self.enterprise_id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_Box_BoxServerAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_Box_BoxServerAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustCasbIntegration_Box_BoxServerAuthenticationRef {
        ZeroTrustCasbIntegration_Box_BoxServerAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_Box_BoxServerAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enterprise_id` after provisioning.\nBox Enterprise ID from Admin Console > Accounts & Billing."]
    pub fn enterprise_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enterprise_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Box {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    box_server_authentication: std::option::Option<ZeroTrustCasbIntegration_Box_BoxServerAuthentication>,
}

impl ZeroTrustCasbIntegration_Box {
    #[doc =
        "Set the field `box_server_authentication`.\nAuthenticate with Box server authentication. Before creating the integration, add the Cloudflare CASB application in Box Admin Console > Integrations > Platform Apps Manager > Server Authentication Apps using client ID `puaghckpy0578r8p6f3g0rf860unup4r`."]
    pub fn set_box_server_authentication(
        mut self,
        v: impl std::convert::Into<ZeroTrustCasbIntegration_Box_BoxServerAuthentication>,
    ) -> Self {
        self.box_server_authentication = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Box {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Box>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Box {}

impl BuildZeroTrustCasbIntegration_Box {
    pub fn build(self) -> ZeroTrustCasbIntegration_Box {
        ZeroTrustCasbIntegration_Box { box_server_authentication: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_BoxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_BoxRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustCasbIntegration_BoxRef {
        ZeroTrustCasbIntegration_BoxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_BoxRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `box_server_authentication` after provisioning.\nAuthenticate with Box server authentication. Before creating the integration, add the Cloudflare CASB application in Box Admin Console > Integrations > Platform Apps Manager > Server Authentication Apps using client ID `puaghckpy0578r8p6f3g0rf860unup4r`."]
    pub fn box_server_authentication(&self) -> ZeroTrustCasbIntegration_Box_BoxServerAuthenticationRef {
        ZeroTrustCasbIntegration_Box_BoxServerAuthenticationRef::new(
            self.shared().clone(),
            format!("{}.box_server_authentication", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccount {
    service_account_key_json: terrars::PrimField<std::string::String>,
}

impl ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccount { }

impl terrars::ToListMappable for ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccount {
    type O =
        terrars::BlockAssignable<ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccount {
    #[doc =
        "Contents of a Google service account JSON key file. This value is write-only and is never persisted to Terraform state."]
    pub service_account_key_json: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccount {
    pub fn build(self) -> ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccount {
        ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccount {
            service_account_key_json: self.service_account_key_json,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccountRef {
        ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account_key_json` after provisioning.\nContents of a Google service account JSON key file. This value is write-only and is never persisted to Terraform state."]
    pub fn service_account_key_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_key_json", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_GoogleCloudPlatform {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_platform_service_account: std
    ::option
    ::Option<ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccount>,
}

impl ZeroTrustCasbIntegration_GoogleCloudPlatform {
    #[doc = "Set the field `google_cloud_platform_service_account`.\nAuthenticate with a service account key."]
    pub fn set_google_cloud_platform_service_account(
        mut self,
        v: impl std::convert::Into<ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccount>,
    ) -> Self {
        self.google_cloud_platform_service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbIntegration_GoogleCloudPlatform {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_GoogleCloudPlatform>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_GoogleCloudPlatform {}

impl BuildZeroTrustCasbIntegration_GoogleCloudPlatform {
    pub fn build(self) -> ZeroTrustCasbIntegration_GoogleCloudPlatform {
        ZeroTrustCasbIntegration_GoogleCloudPlatform {
            google_cloud_platform_service_account: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_GoogleCloudPlatformRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_GoogleCloudPlatformRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustCasbIntegration_GoogleCloudPlatformRef {
        ZeroTrustCasbIntegration_GoogleCloudPlatformRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_GoogleCloudPlatformRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `google_cloud_platform_service_account` after provisioning.\nAuthenticate with a service account key."]
    pub fn google_cloud_platform_service_account(
        &self,
    ) -> ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccountRef {
        ZeroTrustCasbIntegration_GoogleCloudPlatform_GoogleCloudPlatformServiceAccountRef::new(
            self.shared().clone(),
            format!("{}.google_cloud_platform_service_account", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccount {
    administrator_email: terrars::PrimField<std::string::String>,
    service_account_key_json: terrars::PrimField<std::string::String>,
}

impl ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccount { }

impl terrars::ToListMappable for ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccount {
    type O =
        terrars::BlockAssignable<ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccount {
    #[doc = "A Google Workspace super administrator email address."]
    pub administrator_email: terrars::PrimField<std::string::String>,
    #[doc =
        "Contents of a Google service account JSON key file. This value is write-only and is never persisted to Terraform state."]
    pub service_account_key_json: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccount {
    pub fn build(self) -> ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccount {
        ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccount {
            administrator_email: self.administrator_email,
            service_account_key_json: self.service_account_key_json,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccountRef {
        ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `administrator_email` after provisioning.\nA Google Workspace super administrator email address."]
    pub fn administrator_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.administrator_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_key_json` after provisioning.\nContents of a Google service account JSON key file. This value is write-only and is never persisted to Terraform state."]
    pub fn service_account_key_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_key_json", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_GoogleWorkspace {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_domain_wide_delegation_service_account: std
    ::option
    ::Option<ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccount>,
}

impl ZeroTrustCasbIntegration_GoogleWorkspace {
    #[doc =
        "Set the field `google_domain_wide_delegation_service_account`.\nAuthenticate with a service account granted domain-wide delegation."]
    pub fn set_google_domain_wide_delegation_service_account(
        mut self,
        v: impl std::convert::Into<ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccount>,
    ) -> Self {
        self.google_domain_wide_delegation_service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbIntegration_GoogleWorkspace {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_GoogleWorkspace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_GoogleWorkspace {}

impl BuildZeroTrustCasbIntegration_GoogleWorkspace {
    pub fn build(self) -> ZeroTrustCasbIntegration_GoogleWorkspace {
        ZeroTrustCasbIntegration_GoogleWorkspace {
            google_domain_wide_delegation_service_account: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_GoogleWorkspaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_GoogleWorkspaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustCasbIntegration_GoogleWorkspaceRef {
        ZeroTrustCasbIntegration_GoogleWorkspaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_GoogleWorkspaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `google_domain_wide_delegation_service_account` after provisioning.\nAuthenticate with a service account granted domain-wide delegation."]
    pub fn google_domain_wide_delegation_service_account(
        &self,
    ) -> ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccountRef {
        ZeroTrustCasbIntegration_GoogleWorkspace_GoogleDomainWideDelegationServiceAccountRef::new(
            self.shared().clone(),
            format!("{}.google_domain_wide_delegation_service_account", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKey {
    admin_api_key: terrars::PrimField<std::string::String>,
    compliance_api_key: terrars::PrimField<std::string::String>,
    organization_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    workspace_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKey {
    #[doc =
        "Set the field `project_api_key`.\nOpenAI Project API key, used for DLP. This value is write-only and is never persisted to Terraform state."]
    pub fn set_project_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\nOpenAI Project ID, used for DLP."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKey {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKey {
    #[doc =
        "OpenAI Admin API key with api.management.read access. This value is write-only and is never persisted to Terraform state."]
    pub admin_api_key: terrars::PrimField<std::string::String>,
    #[doc =
        "OpenAI Compliance API key for audit logs. This value is write-only and is never persisted to Terraform state."]
    pub compliance_api_key: terrars::PrimField<std::string::String>,
    #[doc = "OpenAI Organization ID."]
    pub organization_id: terrars::PrimField<std::string::String>,
    #[doc = "OpenAI Workspace ID for compliance data."]
    pub workspace_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKey {
    pub fn build(self) -> ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKey {
        ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKey {
            admin_api_key: self.admin_api_key,
            compliance_api_key: self.compliance_api_key,
            organization_id: self.organization_id,
            project_api_key: core::default::Default::default(),
            project_id: core::default::Default::default(),
            workspace_id: self.workspace_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKeyRef {
        ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_api_key` after provisioning.\nOpenAI Admin API key with api.management.read access. This value is write-only and is never persisted to Terraform state."]
    pub fn admin_api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_api_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compliance_api_key` after provisioning.\nOpenAI Compliance API key for audit logs. This value is write-only and is never persisted to Terraform state."]
    pub fn compliance_api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nOpenAI Organization ID."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_api_key` after provisioning.\nOpenAI Project API key, used for DLP. This value is write-only and is never persisted to Terraform state."]
    pub fn project_api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nOpenAI Project ID, used for DLP."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workspace_id` after provisioning.\nOpenAI Workspace ID for compliance data."]
    pub fn workspace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKey {
    admin_api_key: terrars::PrimField<std::string::String>,
    organization_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKey {
    #[doc =
        "Set the field `project_api_key`.\nOpenAI Project API key, used for DLP. This value is write-only and is never persisted to Terraform state."]
    pub fn set_project_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\nOpenAI Project ID, used for DLP."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKey {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Openai_ChatgptStandardApiKey {
    #[doc =
        "OpenAI Admin API key with api.management.read access. This value is write-only and is never persisted to Terraform state."]
    pub admin_api_key: terrars::PrimField<std::string::String>,
    #[doc = "OpenAI Organization ID."]
    pub organization_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustCasbIntegration_Openai_ChatgptStandardApiKey {
    pub fn build(self) -> ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKey {
        ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKey {
            admin_api_key: self.admin_api_key,
            organization_id: self.organization_id,
            project_api_key: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKeyRef {
        ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_api_key` after provisioning.\nOpenAI Admin API key with api.management.read access. This value is write-only and is never persisted to Terraform state."]
    pub fn admin_api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nOpenAI Organization ID."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_api_key` after provisioning.\nOpenAI Project API key, used for DLP. This value is write-only and is never persisted to Terraform state."]
    pub fn project_api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nOpenAI Project ID, used for DLP."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustCasbIntegration_Openai {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatgpt_compliance_api_key: std::option::Option<ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKey>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatgpt_standard_api_key: std::option::Option<ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKey>,
}

impl ZeroTrustCasbIntegration_Openai {
    #[doc =
        "Set the field `chatgpt_compliance_api_key`.\nAuthenticate with an OpenAI Compliance API key. Requires an Enterprise plan."]
    pub fn set_chatgpt_compliance_api_key(
        mut self,
        v: impl std::convert::Into<ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKey>,
    ) -> Self {
        self.chatgpt_compliance_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatgpt_standard_api_key`.\nAuthenticate with an OpenAI Admin API key."]
    pub fn set_chatgpt_standard_api_key(
        mut self,
        v: impl std::convert::Into<ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKey>,
    ) -> Self {
        self.chatgpt_standard_api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustCasbIntegration_Openai {
    type O = terrars::BlockAssignable<ZeroTrustCasbIntegration_Openai>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustCasbIntegration_Openai {}

impl BuildZeroTrustCasbIntegration_Openai {
    pub fn build(self) -> ZeroTrustCasbIntegration_Openai {
        ZeroTrustCasbIntegration_Openai {
            chatgpt_compliance_api_key: core::default::Default::default(),
            chatgpt_standard_api_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustCasbIntegration_OpenaiRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustCasbIntegration_OpenaiRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustCasbIntegration_OpenaiRef {
        ZeroTrustCasbIntegration_OpenaiRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustCasbIntegration_OpenaiRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `chatgpt_compliance_api_key` after provisioning.\nAuthenticate with an OpenAI Compliance API key. Requires an Enterprise plan."]
    pub fn chatgpt_compliance_api_key(&self) -> ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKeyRef {
        ZeroTrustCasbIntegration_Openai_ChatgptComplianceApiKeyRef::new(
            self.shared().clone(),
            format!("{}.chatgpt_compliance_api_key", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `chatgpt_standard_api_key` after provisioning.\nAuthenticate with an OpenAI Admin API key."]
    pub fn chatgpt_standard_api_key(&self) -> ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKeyRef {
        ZeroTrustCasbIntegration_Openai_ChatgptStandardApiKeyRef::new(
            self.shared().clone(),
            format!("{}.chatgpt_standard_api_key", self.base),
        )
    }
}
