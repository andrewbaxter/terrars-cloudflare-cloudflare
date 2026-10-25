#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDlpCustomProfileData_ {
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
    context_awareness: std::option::Option<ZeroTrustDlpCustomProfile_ContextAwareness>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entries: std::option::Option<std::vec::Vec<ZeroTrustDlpCustomProfile_Entries>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_levels: std::option::Option<std::vec::Vec<ZeroTrustDlpCustomProfile_SensitivityLevels>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_entries: std::option::Option<std::vec::Vec<ZeroTrustDlpCustomProfile_SharedEntries>>,
}

struct ZeroTrustDlpCustomProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDlpCustomProfileData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDlpCustomProfile(std::rc::Rc<ZeroTrustDlpCustomProfile_>);

impl ZeroTrustDlpCustomProfile {
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

    #[doc =
        "Set the field `allowed_match_count`.\nRelated DLP policies will trigger when the match count exceeds the number set."]
    pub fn set_allowed_match_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().allowed_match_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `confidence_threshold`.\n"]
    pub fn set_confidence_threshold(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().confidence_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `context_awareness`.\nScan the context of predefined entries to only return matches surrounded by keywords."]
    pub fn set_context_awareness(self, v: impl std::convert::Into<ZeroTrustDlpCustomProfile_ContextAwareness>) -> Self {
        self.0.data.borrow_mut().context_awareness = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_classes`.\nData class IDs to associate with the profile."]
    pub fn set_data_classes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().data_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_tags`.\nData tag IDs to associate with the profile."]
    pub fn set_data_tags(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().data_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the profile."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `entries`.\nCustom entries from this profile.\nIf this field is omitted, entries owned by this profile will not be changed."]
    pub fn set_entries(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustDlpCustomProfile_Entries>>) -> Self {
        self.0.data.borrow_mut().entries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocr_enabled`.\n"]
    pub fn set_ocr_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ocr_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_levels`.\nSensitivity levels to associate with the profile."]
    pub fn set_sensitivity_levels(
        self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustDlpCustomProfile_SensitivityLevels>>,
    ) -> Self {
        self.0.data.borrow_mut().sensitivity_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shared_entries`.\nEntries from other profiles (e.g. pre-defined Cloudflare profiles, or your Microsoft Information Protection profiles)."]
    pub fn set_shared_entries(
        self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustDlpCustomProfile_SharedEntries>>,
    ) -> Self {
        self.0.data.borrow_mut().shared_entries = std::option::Option::Some(v.into());
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

    #[doc =
        "Get a reference to the value of field `allowed_match_count` after provisioning.\nRelated DLP policies will trigger when the match count exceeds the number set."]
    pub fn allowed_match_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_match_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidence_threshold` after provisioning.\n"]
    pub fn confidence_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `context_awareness` after provisioning.\nScan the context of predefined entries to only return matches surrounded by keywords."]
    pub fn context_awareness(&self) -> ZeroTrustDlpCustomProfile_ContextAwarenessRef {
        ZeroTrustDlpCustomProfile_ContextAwarenessRef::new(
            self.shared().clone(),
            format!("{}.context_awareness", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nWhen the profile was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_classes` after provisioning.\nData class IDs to associate with the profile."]
    pub fn data_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_classes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_tags` after provisioning.\nData tag IDs to associate with the profile."]
    pub fn data_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the profile."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entries` after provisioning.\nCustom entries from this profile.\nIf this field is omitted, entries owned by this profile will not be changed."]
    pub fn entries(&self) -> terrars::SetRef<ZeroTrustDlpCustomProfile_EntriesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.entries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id of the profile (uuid)."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\n"]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
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

    #[doc =
        "Get a reference to the value of field `sensitivity_levels` after provisioning.\nSensitivity levels to associate with the profile."]
    pub fn sensitivity_levels(&self) -> terrars::ListRef<ZeroTrustDlpCustomProfile_SensitivityLevelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_levels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shared_entries` after provisioning.\nEntries from other profiles (e.g. pre-defined Cloudflare profiles, or your Microsoft Information Protection profiles)."]
    pub fn shared_entries(&self) -> terrars::SetRef<ZeroTrustDlpCustomProfile_SharedEntriesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.shared_entries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"custom\", \"predefined\", \"integration\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nWhen the profile was lasted updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustDlpCustomProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDlpCustomProfile { }

impl terrars::ToListMappable for ZeroTrustDlpCustomProfile {
    type O = terrars::ListRef<ZeroTrustDlpCustomProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDlpCustomProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dlp_custom_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDlpCustomProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDlpCustomProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDlpCustomProfile {
        let out = ZeroTrustDlpCustomProfile(std::rc::Rc::new(ZeroTrustDlpCustomProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDlpCustomProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                ai_context_enabled: core::default::Default::default(),
                allowed_match_count: core::default::Default::default(),
                confidence_threshold: core::default::Default::default(),
                context_awareness: core::default::Default::default(),
                data_classes: core::default::Default::default(),
                data_tags: core::default::Default::default(),
                description: core::default::Default::default(),
                entries: core::default::Default::default(),
                name: self.name,
                ocr_enabled: core::default::Default::default(),
                sensitivity_levels: core::default::Default::default(),
                shared_entries: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDlpCustomProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpCustomProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDlpCustomProfileRef {
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

    #[doc =
        "Get a reference to the value of field `allowed_match_count` after provisioning.\nRelated DLP policies will trigger when the match count exceeds the number set."]
    pub fn allowed_match_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_match_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidence_threshold` after provisioning.\n"]
    pub fn confidence_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `context_awareness` after provisioning.\nScan the context of predefined entries to only return matches surrounded by keywords."]
    pub fn context_awareness(&self) -> ZeroTrustDlpCustomProfile_ContextAwarenessRef {
        ZeroTrustDlpCustomProfile_ContextAwarenessRef::new(
            self.shared().clone(),
            format!("{}.context_awareness", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nWhen the profile was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_classes` after provisioning.\nData class IDs to associate with the profile."]
    pub fn data_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_classes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_tags` after provisioning.\nData tag IDs to associate with the profile."]
    pub fn data_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the profile."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entries` after provisioning.\nCustom entries from this profile.\nIf this field is omitted, entries owned by this profile will not be changed."]
    pub fn entries(&self) -> terrars::SetRef<ZeroTrustDlpCustomProfile_EntriesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.entries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id of the profile (uuid)."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\n"]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
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

    #[doc =
        "Get a reference to the value of field `sensitivity_levels` after provisioning.\nSensitivity levels to associate with the profile."]
    pub fn sensitivity_levels(&self) -> terrars::ListRef<ZeroTrustDlpCustomProfile_SensitivityLevelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_levels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shared_entries` after provisioning.\nEntries from other profiles (e.g. pre-defined Cloudflare profiles, or your Microsoft Information Protection profiles)."]
    pub fn shared_entries(&self) -> terrars::SetRef<ZeroTrustDlpCustomProfile_SharedEntriesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.shared_entries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"custom\", \"predefined\", \"integration\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nWhen the profile was lasted updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpCustomProfile_ContextAwareness_Skip {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    files: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustDlpCustomProfile_ContextAwareness_Skip {
    #[doc = "Set the field `files`.\nIf the content type is a file, skip context analysis and return all matches."]
    pub fn set_files(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.files = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDlpCustomProfile_ContextAwareness_Skip {
    type O = terrars::BlockAssignable<ZeroTrustDlpCustomProfile_ContextAwareness_Skip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpCustomProfile_ContextAwareness_Skip {}

impl BuildZeroTrustDlpCustomProfile_ContextAwareness_Skip {
    pub fn build(self) -> ZeroTrustDlpCustomProfile_ContextAwareness_Skip {
        ZeroTrustDlpCustomProfile_ContextAwareness_Skip { files: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
        ZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `files` after provisioning.\nIf the content type is a file, skip context analysis and return all matches."]
    pub fn files(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.files", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpCustomProfile_ContextAwareness {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip: std::option::Option<ZeroTrustDlpCustomProfile_ContextAwareness_Skip>,
}

impl ZeroTrustDlpCustomProfile_ContextAwareness {
    #[doc =
        "Set the field `enabled`.\nIf true, scan the context of predefined entries to only return matches surrounded by keywords."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip`.\nContent types to exclude from context analysis and return all matches."]
    pub fn set_skip(mut self, v: impl std::convert::Into<ZeroTrustDlpCustomProfile_ContextAwareness_Skip>) -> Self {
        self.skip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDlpCustomProfile_ContextAwareness {
    type O = terrars::BlockAssignable<ZeroTrustDlpCustomProfile_ContextAwareness>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpCustomProfile_ContextAwareness {}

impl BuildZeroTrustDlpCustomProfile_ContextAwareness {
    pub fn build(self) -> ZeroTrustDlpCustomProfile_ContextAwareness {
        ZeroTrustDlpCustomProfile_ContextAwareness {
            enabled: core::default::Default::default(),
            skip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpCustomProfile_ContextAwarenessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpCustomProfile_ContextAwarenessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpCustomProfile_ContextAwarenessRef {
        ZeroTrustDlpCustomProfile_ContextAwarenessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpCustomProfile_ContextAwarenessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIf true, scan the context of predefined entries to only return matches surrounded by keywords."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `skip` after provisioning.\nContent types to exclude from context analysis and return all matches."]
    pub fn skip(&self) -> ZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
        ZeroTrustDlpCustomProfile_ContextAwareness_SkipRef::new(self.shared().clone(), format!("{}.skip", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpCustomProfile_Entries_Pattern {
    regex: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustDlpCustomProfile_Entries_Pattern {
    #[doc = "Set the field `validation`.\nAvailable values: \"luhn\"."]
    pub fn set_validation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.validation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDlpCustomProfile_Entries_Pattern {
    type O = terrars::BlockAssignable<ZeroTrustDlpCustomProfile_Entries_Pattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpCustomProfile_Entries_Pattern {
    #[doc = ""]
    pub regex: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDlpCustomProfile_Entries_Pattern {
    pub fn build(self) -> ZeroTrustDlpCustomProfile_Entries_Pattern {
        ZeroTrustDlpCustomProfile_Entries_Pattern {
            regex: self.regex,
            validation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpCustomProfile_Entries_PatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpCustomProfile_Entries_PatternRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpCustomProfile_Entries_PatternRef {
        ZeroTrustDlpCustomProfile_Entries_PatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpCustomProfile_Entries_PatternRef {
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
pub struct ZeroTrustDlpCustomProfile_Entries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    pattern: ZeroTrustDlpCustomProfile_Entries_Pattern,
}

impl ZeroTrustDlpCustomProfile_Entries {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entry_id`.\n"]
    pub fn set_entry_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entry_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDlpCustomProfile_Entries {
    type O = terrars::BlockAssignable<ZeroTrustDlpCustomProfile_Entries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpCustomProfile_Entries {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub pattern: ZeroTrustDlpCustomProfile_Entries_Pattern,
}

impl BuildZeroTrustDlpCustomProfile_Entries {
    pub fn build(self) -> ZeroTrustDlpCustomProfile_Entries {
        ZeroTrustDlpCustomProfile_Entries {
            description: core::default::Default::default(),
            enabled: self.enabled,
            entry_id: core::default::Default::default(),
            name: self.name,
            pattern: self.pattern,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpCustomProfile_EntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpCustomProfile_EntriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpCustomProfile_EntriesRef {
        ZeroTrustDlpCustomProfile_EntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpCustomProfile_EntriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\n"]
    pub fn entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> ZeroTrustDlpCustomProfile_Entries_PatternRef {
        ZeroTrustDlpCustomProfile_Entries_PatternRef::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpCustomProfile_SensitivityLevels {
    group_id: terrars::PrimField<std::string::String>,
    level_id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDlpCustomProfile_SensitivityLevels { }

impl terrars::ToListMappable for ZeroTrustDlpCustomProfile_SensitivityLevels {
    type O = terrars::BlockAssignable<ZeroTrustDlpCustomProfile_SensitivityLevels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpCustomProfile_SensitivityLevels {
    #[doc = ""]
    pub group_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub level_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDlpCustomProfile_SensitivityLevels {
    pub fn build(self) -> ZeroTrustDlpCustomProfile_SensitivityLevels {
        ZeroTrustDlpCustomProfile_SensitivityLevels {
            group_id: self.group_id,
            level_id: self.level_id,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpCustomProfile_SensitivityLevelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpCustomProfile_SensitivityLevelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDlpCustomProfile_SensitivityLevelsRef {
        ZeroTrustDlpCustomProfile_SensitivityLevelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpCustomProfile_SensitivityLevelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `level_id` after provisioning.\n"]
    pub fn level_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpCustomProfile_SharedEntries {
    enabled: terrars::PrimField<bool>,
    entry_id: terrars::PrimField<std::string::String>,
    entry_type: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDlpCustomProfile_SharedEntries { }

impl terrars::ToListMappable for ZeroTrustDlpCustomProfile_SharedEntries {
    type O = terrars::BlockAssignable<ZeroTrustDlpCustomProfile_SharedEntries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpCustomProfile_SharedEntries {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub entry_id: terrars::PrimField<std::string::String>,
    #[doc = "Available values: \"custom\", \"predefined\", \"integration\", \"exact_data\", \"document_fingerprint\"."]
    pub entry_type: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDlpCustomProfile_SharedEntries {
    pub fn build(self) -> ZeroTrustDlpCustomProfile_SharedEntries {
        ZeroTrustDlpCustomProfile_SharedEntries {
            enabled: self.enabled,
            entry_id: self.entry_id,
            entry_type: self.entry_type,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpCustomProfile_SharedEntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpCustomProfile_SharedEntriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpCustomProfile_SharedEntriesRef {
        ZeroTrustDlpCustomProfile_SharedEntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpCustomProfile_SharedEntriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\n"]
    pub fn entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `entry_type` after provisioning.\nAvailable values: \"custom\", \"predefined\", \"integration\", \"exact_data\", \"document_fingerprint\"."]
    pub fn entry_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_type", self.base))
    }
}
