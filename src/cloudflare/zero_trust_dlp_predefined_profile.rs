#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDlpPredefinedProfileData_ {
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
    ai_context_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_match_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence_threshold: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_entries: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entries: std::option::Option<std::vec::Vec<ZeroTrustDlpPredefinedProfile_Entries>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr_enabled: std::option::Option<terrars::PrimField<bool>>,
    profile_id: terrars::PrimField<std::string::String>,
}

struct ZeroTrustDlpPredefinedProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDlpPredefinedProfileData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDlpPredefinedProfile(std::rc::Rc<ZeroTrustDlpPredefinedProfile_>);

impl ZeroTrustDlpPredefinedProfile {
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

    #[doc = "Set the field `ai_context_enabled`.\n"]
    pub fn set_ai_context_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ai_context_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_match_count`.\n"]
    pub fn set_allowed_match_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().allowed_match_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `confidence_threshold`.\n"]
    pub fn set_confidence_threshold(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().confidence_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_entries`.\n"]
    pub fn set_enabled_entries(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().enabled_entries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entries`.\n"]
    pub fn set_entries(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustDlpPredefinedProfile_Entries>>) -> Self {
        self.0.data.borrow_mut().entries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocr_enabled`.\n"]
    pub fn set_ocr_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ocr_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ai_context_enabled` after provisioning.\n"]
    pub fn ai_context_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_context_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_match_count` after provisioning.\n"]
    pub fn allowed_match_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_match_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidence_threshold` after provisioning.\n"]
    pub fn confidence_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_entries` after provisioning.\n"]
    pub fn enabled_entries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_entries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> terrars::ListRef<ZeroTrustDlpPredefinedProfile_EntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the predefined profile."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ocr_enabled` after provisioning.\n"]
    pub fn ocr_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocr_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `open_access` after provisioning.\nWhether this profile can be accessed by anyone."]
    pub fn open_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustDlpPredefinedProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDlpPredefinedProfile { }

impl terrars::ToListMappable for ZeroTrustDlpPredefinedProfile {
    type O = terrars::ListRef<ZeroTrustDlpPredefinedProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDlpPredefinedProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dlp_predefined_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDlpPredefinedProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub profile_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDlpPredefinedProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDlpPredefinedProfile {
        let out = ZeroTrustDlpPredefinedProfile(std::rc::Rc::new(ZeroTrustDlpPredefinedProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDlpPredefinedProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                ai_context_enabled: core::default::Default::default(),
                allowed_match_count: core::default::Default::default(),
                confidence_threshold: core::default::Default::default(),
                enabled_entries: core::default::Default::default(),
                entries: core::default::Default::default(),
                ocr_enabled: core::default::Default::default(),
                profile_id: self.profile_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDlpPredefinedProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpPredefinedProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDlpPredefinedProfileRef {
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

    #[doc = "Get a reference to the value of field `ai_context_enabled` after provisioning.\n"]
    pub fn ai_context_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_context_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_match_count` after provisioning.\n"]
    pub fn allowed_match_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_match_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidence_threshold` after provisioning.\n"]
    pub fn confidence_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_entries` after provisioning.\n"]
    pub fn enabled_entries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_entries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> terrars::ListRef<ZeroTrustDlpPredefinedProfile_EntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the predefined profile."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ocr_enabled` after provisioning.\n"]
    pub fn ocr_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocr_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `open_access` after provisioning.\nWhether this profile can be accessed by anyone."]
    pub fn open_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpPredefinedProfile_Entries {
    enabled: terrars::PrimField<bool>,
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDlpPredefinedProfile_Entries { }

impl terrars::ToListMappable for ZeroTrustDlpPredefinedProfile_Entries {
    type O = terrars::BlockAssignable<ZeroTrustDlpPredefinedProfile_Entries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpPredefinedProfile_Entries {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDlpPredefinedProfile_Entries {
    pub fn build(self) -> ZeroTrustDlpPredefinedProfile_Entries {
        ZeroTrustDlpPredefinedProfile_Entries {
            enabled: self.enabled,
            id: self.id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpPredefinedProfile_EntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpPredefinedProfile_EntriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpPredefinedProfile_EntriesRef {
        ZeroTrustDlpPredefinedProfile_EntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpPredefinedProfile_EntriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
