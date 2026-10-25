#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDlpIntegrationEntryData_ {
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
    enabled: terrars::PrimField<bool>,
    entry_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ZeroTrustDlpIntegrationEntry_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDlpIntegrationEntryData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDlpIntegrationEntry(std::rc::Rc<ZeroTrustDlpIntegrationEntry_>);

impl ZeroTrustDlpIntegrationEntry {
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
        "Set the field `profile_id`.\nThis field is not used as the owning profile.\nFor predefined entries it is already set to a predefined profile."]
    pub fn set_profile_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `case_sensitive` after provisioning.\nOnly applies to custom word lists.\nDetermines if the words should be matched in a case-sensitive manner\nCannot be set to false if secret is true"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidence` after provisioning.\n"]
    pub fn confidence(&self) -> ZeroTrustDlpIntegrationEntry_ConfidenceRef {
        ZeroTrustDlpIntegrationEntry_ConfidenceRef::new(
            self.shared().clone(),
            format!("{}.confidence", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deprecated` after provisioning.\nWhether this entry is deprecated for new use. This is computed from the static catalog and\nemitted only when true."]
    pub fn deprecated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\n"]
    pub fn entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> ZeroTrustDlpIntegrationEntry_PatternRef {
        ZeroTrustDlpIntegrationEntry_PatternRef::new(
            self.shared().clone(),
            format!("{}.pattern", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `profile_id` after provisioning.\nThis field is not used as the owning profile.\nFor predefined entries it is already set to a predefined profile."]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> terrars::ListRef<ZeroTrustDlpIntegrationEntry_ProfilesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"custom\", \"custom_prompt_topic\", \"predefined\", \"integration\", \"exact_data\", \"document_fingerprint\", \"word_list\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upload_status` after provisioning.\nAvailable values: \"empty\", \"uploading\", \"pending\", \"processing\", \"failed\", \"complete\"."]
    pub fn upload_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `variant` after provisioning.\nA Predefined AI prompt classification topic entry."]
    pub fn variant(&self) -> ZeroTrustDlpIntegrationEntry_VariantRef {
        ZeroTrustDlpIntegrationEntry_VariantRef::new(
            self.shared().clone(),
            format!("{}.variant", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustDlpIntegrationEntry {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDlpIntegrationEntry { }

impl terrars::ToListMappable for ZeroTrustDlpIntegrationEntry {
    type O = terrars::ListRef<ZeroTrustDlpIntegrationEntryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDlpIntegrationEntry_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dlp_integration_entry".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDlpIntegrationEntry {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub entry_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDlpIntegrationEntry {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDlpIntegrationEntry {
        let out = ZeroTrustDlpIntegrationEntry(std::rc::Rc::new(ZeroTrustDlpIntegrationEntry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDlpIntegrationEntryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                enabled: self.enabled,
                entry_id: self.entry_id,
                profile_id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDlpIntegrationEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpIntegrationEntryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDlpIntegrationEntryRef {
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
        "Get a reference to the value of field `case_sensitive` after provisioning.\nOnly applies to custom word lists.\nDetermines if the words should be matched in a case-sensitive manner\nCannot be set to false if secret is true"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidence` after provisioning.\n"]
    pub fn confidence(&self) -> ZeroTrustDlpIntegrationEntry_ConfidenceRef {
        ZeroTrustDlpIntegrationEntry_ConfidenceRef::new(
            self.shared().clone(),
            format!("{}.confidence", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deprecated` after provisioning.\nWhether this entry is deprecated for new use. This is computed from the static catalog and\nemitted only when true."]
    pub fn deprecated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\n"]
    pub fn entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> ZeroTrustDlpIntegrationEntry_PatternRef {
        ZeroTrustDlpIntegrationEntry_PatternRef::new(
            self.shared().clone(),
            format!("{}.pattern", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `profile_id` after provisioning.\nThis field is not used as the owning profile.\nFor predefined entries it is already set to a predefined profile."]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> terrars::ListRef<ZeroTrustDlpIntegrationEntry_ProfilesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"custom\", \"custom_prompt_topic\", \"predefined\", \"integration\", \"exact_data\", \"document_fingerprint\", \"word_list\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upload_status` after provisioning.\nAvailable values: \"empty\", \"uploading\", \"pending\", \"processing\", \"failed\", \"complete\"."]
    pub fn upload_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `variant` after provisioning.\nA Predefined AI prompt classification topic entry."]
    pub fn variant(&self) -> ZeroTrustDlpIntegrationEntry_VariantRef {
        ZeroTrustDlpIntegrationEntry_VariantRef::new(
            self.shared().clone(),
            format!("{}.variant", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpIntegrationEntry_Confidence {}

impl ZeroTrustDlpIntegrationEntry_Confidence { }

impl terrars::ToListMappable for ZeroTrustDlpIntegrationEntry_Confidence {
    type O = terrars::BlockAssignable<ZeroTrustDlpIntegrationEntry_Confidence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpIntegrationEntry_Confidence {}

impl BuildZeroTrustDlpIntegrationEntry_Confidence {
    pub fn build(self) -> ZeroTrustDlpIntegrationEntry_Confidence {
        ZeroTrustDlpIntegrationEntry_Confidence {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpIntegrationEntry_ConfidenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpIntegrationEntry_ConfidenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpIntegrationEntry_ConfidenceRef {
        ZeroTrustDlpIntegrationEntry_ConfidenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpIntegrationEntry_ConfidenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ai_context_available` after provisioning.\nIndicates whether this entry has AI remote service validation."]
    pub fn ai_context_available(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_context_available", self.base))
    }

    #[doc =
        "Get a reference to the value of field `available` after provisioning.\nIndicates whether this entry has any form of validation that is not an AI remote service."]
    pub fn available(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpIntegrationEntry_Pattern {}

impl ZeroTrustDlpIntegrationEntry_Pattern { }

impl terrars::ToListMappable for ZeroTrustDlpIntegrationEntry_Pattern {
    type O = terrars::BlockAssignable<ZeroTrustDlpIntegrationEntry_Pattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpIntegrationEntry_Pattern {}

impl BuildZeroTrustDlpIntegrationEntry_Pattern {
    pub fn build(self) -> ZeroTrustDlpIntegrationEntry_Pattern {
        ZeroTrustDlpIntegrationEntry_Pattern {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpIntegrationEntry_PatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpIntegrationEntry_PatternRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpIntegrationEntry_PatternRef {
        ZeroTrustDlpIntegrationEntry_PatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpIntegrationEntry_PatternRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\nAvailable values: \"luhn\"."]
    pub fn validation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpIntegrationEntry_Profiles {}

impl ZeroTrustDlpIntegrationEntry_Profiles { }

impl terrars::ToListMappable for ZeroTrustDlpIntegrationEntry_Profiles {
    type O = terrars::BlockAssignable<ZeroTrustDlpIntegrationEntry_Profiles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpIntegrationEntry_Profiles {}

impl BuildZeroTrustDlpIntegrationEntry_Profiles {
    pub fn build(self) -> ZeroTrustDlpIntegrationEntry_Profiles {
        ZeroTrustDlpIntegrationEntry_Profiles {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpIntegrationEntry_ProfilesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpIntegrationEntry_ProfilesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpIntegrationEntry_ProfilesRef {
        ZeroTrustDlpIntegrationEntry_ProfilesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpIntegrationEntry_ProfilesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpIntegrationEntry_Variant {}

impl ZeroTrustDlpIntegrationEntry_Variant { }

impl terrars::ToListMappable for ZeroTrustDlpIntegrationEntry_Variant {
    type O = terrars::BlockAssignable<ZeroTrustDlpIntegrationEntry_Variant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpIntegrationEntry_Variant {}

impl BuildZeroTrustDlpIntegrationEntry_Variant {
    pub fn build(self) -> ZeroTrustDlpIntegrationEntry_Variant {
        ZeroTrustDlpIntegrationEntry_Variant {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpIntegrationEntry_VariantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpIntegrationEntry_VariantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpIntegrationEntry_VariantRef {
        ZeroTrustDlpIntegrationEntry_VariantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpIntegrationEntry_VariantRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA customer-facing explanation of what this predefined AI prompt topic represents."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic_type` after provisioning.\nAvailable values: \"Intent\", \"Content\"."]
    pub fn topic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"PromptTopic\", \"General\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
