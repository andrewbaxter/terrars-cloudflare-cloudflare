#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDlpEntryData_ {
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
    entry_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustDlpEntry_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDlpEntryData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDlpEntry(std::rc::Rc<DataZeroTrustDlpEntry_>);

impl DataZeroTrustDlpEntry {
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

    #[doc =
        "Get a reference to the value of field `case_sensitive` after provisioning.\nOnly applies to custom word lists.\nDetermines if the words should be matched in a case-sensitive manner\nCannot be set to false if secret is true"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidence` after provisioning.\n"]
    pub fn confidence(&self) -> DataZeroTrustDlpEntry_ConfidenceRef {
        DataZeroTrustDlpEntry_ConfidenceRef::new(self.shared().clone(), format!("{}.confidence", self.extract_ref()))
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
    pub fn pattern(&self) -> DataZeroTrustDlpEntry_PatternRef {
        DataZeroTrustDlpEntry_PatternRef::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> terrars::ListRef<DataZeroTrustDlpEntry_ProfilesRef> {
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
    pub fn variant(&self) -> DataZeroTrustDlpEntry_VariantRef {
        DataZeroTrustDlpEntry_VariantRef::new(self.shared().clone(), format!("{}.variant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustDlpEntry {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDlpEntry { }

impl terrars::ToListMappable for DataZeroTrustDlpEntry {
    type O = terrars::ListRef<DataZeroTrustDlpEntryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDlpEntry_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dlp_entry".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDlpEntry {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub entry_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustDlpEntry {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDlpEntry {
        let out = DataZeroTrustDlpEntry(std::rc::Rc::new(DataZeroTrustDlpEntry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDlpEntryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                entry_id: self.entry_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDlpEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpEntryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDlpEntryRef {
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
        "Get a reference to the value of field `case_sensitive` after provisioning.\nOnly applies to custom word lists.\nDetermines if the words should be matched in a case-sensitive manner\nCannot be set to false if secret is true"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidence` after provisioning.\n"]
    pub fn confidence(&self) -> DataZeroTrustDlpEntry_ConfidenceRef {
        DataZeroTrustDlpEntry_ConfidenceRef::new(self.shared().clone(), format!("{}.confidence", self.extract_ref()))
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
    pub fn pattern(&self) -> DataZeroTrustDlpEntry_PatternRef {
        DataZeroTrustDlpEntry_PatternRef::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> terrars::ListRef<DataZeroTrustDlpEntry_ProfilesRef> {
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
    pub fn variant(&self) -> DataZeroTrustDlpEntry_VariantRef {
        DataZeroTrustDlpEntry_VariantRef::new(self.shared().clone(), format!("{}.variant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDlpEntry_Confidence {}

impl DataZeroTrustDlpEntry_Confidence { }

impl terrars::ToListMappable for DataZeroTrustDlpEntry_Confidence {
    type O = terrars::BlockAssignable<DataZeroTrustDlpEntry_Confidence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpEntry_Confidence {}

impl BuildDataZeroTrustDlpEntry_Confidence {
    pub fn build(self) -> DataZeroTrustDlpEntry_Confidence {
        DataZeroTrustDlpEntry_Confidence {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpEntry_ConfidenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpEntry_ConfidenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDlpEntry_ConfidenceRef {
        DataZeroTrustDlpEntry_ConfidenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpEntry_ConfidenceRef {
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
pub struct DataZeroTrustDlpEntry_Pattern {}

impl DataZeroTrustDlpEntry_Pattern { }

impl terrars::ToListMappable for DataZeroTrustDlpEntry_Pattern {
    type O = terrars::BlockAssignable<DataZeroTrustDlpEntry_Pattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpEntry_Pattern {}

impl BuildDataZeroTrustDlpEntry_Pattern {
    pub fn build(self) -> DataZeroTrustDlpEntry_Pattern {
        DataZeroTrustDlpEntry_Pattern {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpEntry_PatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpEntry_PatternRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDlpEntry_PatternRef {
        DataZeroTrustDlpEntry_PatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpEntry_PatternRef {
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
pub struct DataZeroTrustDlpEntry_Profiles {}

impl DataZeroTrustDlpEntry_Profiles { }

impl terrars::ToListMappable for DataZeroTrustDlpEntry_Profiles {
    type O = terrars::BlockAssignable<DataZeroTrustDlpEntry_Profiles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpEntry_Profiles {}

impl BuildDataZeroTrustDlpEntry_Profiles {
    pub fn build(self) -> DataZeroTrustDlpEntry_Profiles {
        DataZeroTrustDlpEntry_Profiles {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpEntry_ProfilesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpEntry_ProfilesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDlpEntry_ProfilesRef {
        DataZeroTrustDlpEntry_ProfilesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpEntry_ProfilesRef {
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
pub struct DataZeroTrustDlpEntry_Variant {}

impl DataZeroTrustDlpEntry_Variant { }

impl terrars::ToListMappable for DataZeroTrustDlpEntry_Variant {
    type O = terrars::BlockAssignable<DataZeroTrustDlpEntry_Variant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpEntry_Variant {}

impl BuildDataZeroTrustDlpEntry_Variant {
    pub fn build(self) -> DataZeroTrustDlpEntry_Variant {
        DataZeroTrustDlpEntry_Variant {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpEntry_VariantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpEntry_VariantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDlpEntry_VariantRef {
        DataZeroTrustDlpEntry_VariantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpEntry_VariantRef {
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
