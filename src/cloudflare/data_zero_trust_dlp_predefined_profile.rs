#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDlpPredefinedProfileData_ {
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
    profile_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustDlpPredefinedProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDlpPredefinedProfileData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDlpPredefinedProfile(std::rc::Rc<DataZeroTrustDlpPredefinedProfile_>);

impl DataZeroTrustDlpPredefinedProfile {
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

    #[doc =
        "Get a reference to the value of field `enabled_entries` after provisioning.\nEntries to enable for this predefined profile. Any entries not provided will be disabled."]
    pub fn enabled_entries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_entries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entries` after provisioning.\nThis field has been deprecated for `enabled_entries`."]
    pub fn entries(&self) -> terrars::ListRef<DataZeroTrustDlpPredefinedProfile_EntriesRef> {
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

impl terrars::Referable for DataZeroTrustDlpPredefinedProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDlpPredefinedProfile { }

impl terrars::ToListMappable for DataZeroTrustDlpPredefinedProfile {
    type O = terrars::ListRef<DataZeroTrustDlpPredefinedProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDlpPredefinedProfile_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dlp_predefined_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDlpPredefinedProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub profile_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustDlpPredefinedProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDlpPredefinedProfile {
        let out = DataZeroTrustDlpPredefinedProfile(std::rc::Rc::new(DataZeroTrustDlpPredefinedProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDlpPredefinedProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                profile_id: self.profile_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDlpPredefinedProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpPredefinedProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDlpPredefinedProfileRef {
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

    #[doc =
        "Get a reference to the value of field `enabled_entries` after provisioning.\nEntries to enable for this predefined profile. Any entries not provided will be disabled."]
    pub fn enabled_entries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_entries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entries` after provisioning.\nThis field has been deprecated for `enabled_entries`."]
    pub fn entries(&self) -> terrars::ListRef<DataZeroTrustDlpPredefinedProfile_EntriesRef> {
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
pub struct DataZeroTrustDlpPredefinedProfile_Entries_Confidence {}

impl DataZeroTrustDlpPredefinedProfile_Entries_Confidence { }

impl terrars::ToListMappable for DataZeroTrustDlpPredefinedProfile_Entries_Confidence {
    type O = terrars::BlockAssignable<DataZeroTrustDlpPredefinedProfile_Entries_Confidence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpPredefinedProfile_Entries_Confidence {}

impl BuildDataZeroTrustDlpPredefinedProfile_Entries_Confidence {
    pub fn build(self) -> DataZeroTrustDlpPredefinedProfile_Entries_Confidence {
        DataZeroTrustDlpPredefinedProfile_Entries_Confidence {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpPredefinedProfile_Entries_ConfidenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpPredefinedProfile_Entries_ConfidenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDlpPredefinedProfile_Entries_ConfidenceRef {
        DataZeroTrustDlpPredefinedProfile_Entries_ConfidenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpPredefinedProfile_Entries_ConfidenceRef {
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
pub struct DataZeroTrustDlpPredefinedProfile_Entries_Pattern {}

impl DataZeroTrustDlpPredefinedProfile_Entries_Pattern { }

impl terrars::ToListMappable for DataZeroTrustDlpPredefinedProfile_Entries_Pattern {
    type O = terrars::BlockAssignable<DataZeroTrustDlpPredefinedProfile_Entries_Pattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpPredefinedProfile_Entries_Pattern {}

impl BuildDataZeroTrustDlpPredefinedProfile_Entries_Pattern {
    pub fn build(self) -> DataZeroTrustDlpPredefinedProfile_Entries_Pattern {
        DataZeroTrustDlpPredefinedProfile_Entries_Pattern {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpPredefinedProfile_Entries_PatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpPredefinedProfile_Entries_PatternRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDlpPredefinedProfile_Entries_PatternRef {
        DataZeroTrustDlpPredefinedProfile_Entries_PatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpPredefinedProfile_Entries_PatternRef {
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
pub struct DataZeroTrustDlpPredefinedProfile_Entries_Variant {}

impl DataZeroTrustDlpPredefinedProfile_Entries_Variant { }

impl terrars::ToListMappable for DataZeroTrustDlpPredefinedProfile_Entries_Variant {
    type O = terrars::BlockAssignable<DataZeroTrustDlpPredefinedProfile_Entries_Variant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpPredefinedProfile_Entries_Variant {}

impl BuildDataZeroTrustDlpPredefinedProfile_Entries_Variant {
    pub fn build(self) -> DataZeroTrustDlpPredefinedProfile_Entries_Variant {
        DataZeroTrustDlpPredefinedProfile_Entries_Variant {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpPredefinedProfile_Entries_VariantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpPredefinedProfile_Entries_VariantRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDlpPredefinedProfile_Entries_VariantRef {
        DataZeroTrustDlpPredefinedProfile_Entries_VariantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpPredefinedProfile_Entries_VariantRef {
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

#[derive(Serialize)]
pub struct DataZeroTrustDlpPredefinedProfile_Entries {}

impl DataZeroTrustDlpPredefinedProfile_Entries { }

impl terrars::ToListMappable for DataZeroTrustDlpPredefinedProfile_Entries {
    type O = terrars::BlockAssignable<DataZeroTrustDlpPredefinedProfile_Entries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpPredefinedProfile_Entries {}

impl BuildDataZeroTrustDlpPredefinedProfile_Entries {
    pub fn build(self) -> DataZeroTrustDlpPredefinedProfile_Entries {
        DataZeroTrustDlpPredefinedProfile_Entries {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpPredefinedProfile_EntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpPredefinedProfile_EntriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDlpPredefinedProfile_EntriesRef {
        DataZeroTrustDlpPredefinedProfile_EntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpPredefinedProfile_EntriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `case_sensitive` after provisioning.\nOnly applies to custom word lists.\nDetermines if the words should be matched in a case-sensitive manner\nCannot be set to false if secret is true"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.base))
    }

    #[doc = "Get a reference to the value of field `confidence` after provisioning.\n"]
    pub fn confidence(&self) -> DataZeroTrustDlpPredefinedProfile_Entries_ConfidenceRef {
        DataZeroTrustDlpPredefinedProfile_Entries_ConfidenceRef::new(
            self.shared().clone(),
            format!("{}.confidence", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deprecated` after provisioning.\nWhether this entry is deprecated for new use. This is computed from the static catalog and\nemitted only when true."]
    pub fn deprecated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecated", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> DataZeroTrustDlpPredefinedProfile_Entries_PatternRef {
        DataZeroTrustDlpPredefinedProfile_Entries_PatternRef::new(
            self.shared().clone(),
            format!("{}.pattern", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"custom\", \"custom_prompt_topic\", \"predefined\", \"integration\", \"exact_data\", \"document_fingerprint\", \"word_list\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `variant` after provisioning.\nA Predefined AI prompt classification topic entry."]
    pub fn variant(&self) -> DataZeroTrustDlpPredefinedProfile_Entries_VariantRef {
        DataZeroTrustDlpPredefinedProfile_Entries_VariantRef::new(
            self.shared().clone(),
            format!("{}.variant", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}
