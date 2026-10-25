#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDlpCustomProfileData_ {
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

struct DataZeroTrustDlpCustomProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDlpCustomProfileData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDlpCustomProfile(std::rc::Rc<DataZeroTrustDlpCustomProfile_>);

impl DataZeroTrustDlpCustomProfile {
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

    #[doc =
        "Get a reference to the value of field `allowed_match_count` after provisioning.\nRelated DLP policies will trigger when the match count exceeds the number set."]
    pub fn allowed_match_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_match_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `confidence_threshold` after provisioning.\nAvailable values: \"low\", \"medium\", \"high\", \"very_high\"."]
    pub fn confidence_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `context_awareness` after provisioning.\nScan the context of predefined entries to only return matches surrounded by keywords."]
    pub fn context_awareness(&self) -> DataZeroTrustDlpCustomProfile_ContextAwarenessRef {
        DataZeroTrustDlpCustomProfile_ContextAwarenessRef::new(
            self.shared().clone(),
            format!("{}.context_awareness", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nWhen the profile was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_classes` after provisioning.\nData classes associated with this profile."]
    pub fn data_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_classes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_tags` after provisioning.\nData tags associated with this profile."]
    pub fn data_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the profile."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> terrars::ListRef<DataZeroTrustDlpCustomProfile_EntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\n"]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the profile."]
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

    #[doc =
        "Get a reference to the value of field `sensitivity_levels` after provisioning.\nSensitivity levels associated with this profile."]
    pub fn sensitivity_levels(&self) -> terrars::ListRef<DataZeroTrustDlpCustomProfile_SensitivityLevelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_levels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shared_entries` after provisioning.\n"]
    pub fn shared_entries(&self) -> terrars::ListRef<DataZeroTrustDlpCustomProfile_SharedEntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shared_entries", self.extract_ref()))
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

impl terrars::Referable for DataZeroTrustDlpCustomProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDlpCustomProfile { }

impl terrars::ToListMappable for DataZeroTrustDlpCustomProfile {
    type O = terrars::ListRef<DataZeroTrustDlpCustomProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDlpCustomProfile_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dlp_custom_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDlpCustomProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub profile_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustDlpCustomProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDlpCustomProfile {
        let out = DataZeroTrustDlpCustomProfile(std::rc::Rc::new(DataZeroTrustDlpCustomProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDlpCustomProfileData_ {
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

pub struct DataZeroTrustDlpCustomProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpCustomProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDlpCustomProfileRef {
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

    #[doc =
        "Get a reference to the value of field `allowed_match_count` after provisioning.\nRelated DLP policies will trigger when the match count exceeds the number set."]
    pub fn allowed_match_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_match_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `confidence_threshold` after provisioning.\nAvailable values: \"low\", \"medium\", \"high\", \"very_high\"."]
    pub fn confidence_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `context_awareness` after provisioning.\nScan the context of predefined entries to only return matches surrounded by keywords."]
    pub fn context_awareness(&self) -> DataZeroTrustDlpCustomProfile_ContextAwarenessRef {
        DataZeroTrustDlpCustomProfile_ContextAwarenessRef::new(
            self.shared().clone(),
            format!("{}.context_awareness", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nWhen the profile was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_classes` after provisioning.\nData classes associated with this profile."]
    pub fn data_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_classes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_tags` after provisioning.\nData tags associated with this profile."]
    pub fn data_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the profile."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> terrars::ListRef<DataZeroTrustDlpCustomProfile_EntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\n"]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the profile."]
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

    #[doc =
        "Get a reference to the value of field `sensitivity_levels` after provisioning.\nSensitivity levels associated with this profile."]
    pub fn sensitivity_levels(&self) -> terrars::ListRef<DataZeroTrustDlpCustomProfile_SensitivityLevelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_levels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shared_entries` after provisioning.\n"]
    pub fn shared_entries(&self) -> terrars::ListRef<DataZeroTrustDlpCustomProfile_SharedEntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shared_entries", self.extract_ref()))
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
pub struct DataZeroTrustDlpCustomProfile_ContextAwareness_Skip {}

impl DataZeroTrustDlpCustomProfile_ContextAwareness_Skip { }

impl terrars::ToListMappable for DataZeroTrustDlpCustomProfile_ContextAwareness_Skip {
    type O = terrars::BlockAssignable<DataZeroTrustDlpCustomProfile_ContextAwareness_Skip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpCustomProfile_ContextAwareness_Skip {}

impl BuildDataZeroTrustDlpCustomProfile_ContextAwareness_Skip {
    pub fn build(self) -> DataZeroTrustDlpCustomProfile_ContextAwareness_Skip {
        DataZeroTrustDlpCustomProfile_ContextAwareness_Skip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
        DataZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
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
pub struct DataZeroTrustDlpCustomProfile_ContextAwareness {}

impl DataZeroTrustDlpCustomProfile_ContextAwareness { }

impl terrars::ToListMappable for DataZeroTrustDlpCustomProfile_ContextAwareness {
    type O = terrars::BlockAssignable<DataZeroTrustDlpCustomProfile_ContextAwareness>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpCustomProfile_ContextAwareness {}

impl BuildDataZeroTrustDlpCustomProfile_ContextAwareness {
    pub fn build(self) -> DataZeroTrustDlpCustomProfile_ContextAwareness {
        DataZeroTrustDlpCustomProfile_ContextAwareness {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpCustomProfile_ContextAwarenessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpCustomProfile_ContextAwarenessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDlpCustomProfile_ContextAwarenessRef {
        DataZeroTrustDlpCustomProfile_ContextAwarenessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpCustomProfile_ContextAwarenessRef {
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
    pub fn skip(&self) -> DataZeroTrustDlpCustomProfile_ContextAwareness_SkipRef {
        DataZeroTrustDlpCustomProfile_ContextAwareness_SkipRef::new(
            self.shared().clone(),
            format!("{}.skip", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDlpCustomProfile_Entries_Confidence {}

impl DataZeroTrustDlpCustomProfile_Entries_Confidence { }

impl terrars::ToListMappable for DataZeroTrustDlpCustomProfile_Entries_Confidence {
    type O = terrars::BlockAssignable<DataZeroTrustDlpCustomProfile_Entries_Confidence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpCustomProfile_Entries_Confidence {}

impl BuildDataZeroTrustDlpCustomProfile_Entries_Confidence {
    pub fn build(self) -> DataZeroTrustDlpCustomProfile_Entries_Confidence {
        DataZeroTrustDlpCustomProfile_Entries_Confidence {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpCustomProfile_Entries_ConfidenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpCustomProfile_Entries_ConfidenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDlpCustomProfile_Entries_ConfidenceRef {
        DataZeroTrustDlpCustomProfile_Entries_ConfidenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpCustomProfile_Entries_ConfidenceRef {
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
pub struct DataZeroTrustDlpCustomProfile_Entries_Pattern {}

impl DataZeroTrustDlpCustomProfile_Entries_Pattern { }

impl terrars::ToListMappable for DataZeroTrustDlpCustomProfile_Entries_Pattern {
    type O = terrars::BlockAssignable<DataZeroTrustDlpCustomProfile_Entries_Pattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpCustomProfile_Entries_Pattern {}

impl BuildDataZeroTrustDlpCustomProfile_Entries_Pattern {
    pub fn build(self) -> DataZeroTrustDlpCustomProfile_Entries_Pattern {
        DataZeroTrustDlpCustomProfile_Entries_Pattern {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpCustomProfile_Entries_PatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpCustomProfile_Entries_PatternRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDlpCustomProfile_Entries_PatternRef {
        DataZeroTrustDlpCustomProfile_Entries_PatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpCustomProfile_Entries_PatternRef {
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
pub struct DataZeroTrustDlpCustomProfile_Entries_Variant {}

impl DataZeroTrustDlpCustomProfile_Entries_Variant { }

impl terrars::ToListMappable for DataZeroTrustDlpCustomProfile_Entries_Variant {
    type O = terrars::BlockAssignable<DataZeroTrustDlpCustomProfile_Entries_Variant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpCustomProfile_Entries_Variant {}

impl BuildDataZeroTrustDlpCustomProfile_Entries_Variant {
    pub fn build(self) -> DataZeroTrustDlpCustomProfile_Entries_Variant {
        DataZeroTrustDlpCustomProfile_Entries_Variant {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpCustomProfile_Entries_VariantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpCustomProfile_Entries_VariantRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDlpCustomProfile_Entries_VariantRef {
        DataZeroTrustDlpCustomProfile_Entries_VariantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpCustomProfile_Entries_VariantRef {
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
pub struct DataZeroTrustDlpCustomProfile_Entries {}

impl DataZeroTrustDlpCustomProfile_Entries { }

impl terrars::ToListMappable for DataZeroTrustDlpCustomProfile_Entries {
    type O = terrars::BlockAssignable<DataZeroTrustDlpCustomProfile_Entries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpCustomProfile_Entries {}

impl BuildDataZeroTrustDlpCustomProfile_Entries {
    pub fn build(self) -> DataZeroTrustDlpCustomProfile_Entries {
        DataZeroTrustDlpCustomProfile_Entries {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpCustomProfile_EntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpCustomProfile_EntriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDlpCustomProfile_EntriesRef {
        DataZeroTrustDlpCustomProfile_EntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpCustomProfile_EntriesRef {
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
    pub fn confidence(&self) -> DataZeroTrustDlpCustomProfile_Entries_ConfidenceRef {
        DataZeroTrustDlpCustomProfile_Entries_ConfidenceRef::new(
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
    pub fn pattern(&self) -> DataZeroTrustDlpCustomProfile_Entries_PatternRef {
        DataZeroTrustDlpCustomProfile_Entries_PatternRef::new(
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
    pub fn variant(&self) -> DataZeroTrustDlpCustomProfile_Entries_VariantRef {
        DataZeroTrustDlpCustomProfile_Entries_VariantRef::new(
            self.shared().clone(),
            format!("{}.variant", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDlpCustomProfile_SensitivityLevels {}

impl DataZeroTrustDlpCustomProfile_SensitivityLevels { }

impl terrars::ToListMappable for DataZeroTrustDlpCustomProfile_SensitivityLevels {
    type O = terrars::BlockAssignable<DataZeroTrustDlpCustomProfile_SensitivityLevels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpCustomProfile_SensitivityLevels {}

impl BuildDataZeroTrustDlpCustomProfile_SensitivityLevels {
    pub fn build(self) -> DataZeroTrustDlpCustomProfile_SensitivityLevels {
        DataZeroTrustDlpCustomProfile_SensitivityLevels {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpCustomProfile_SensitivityLevelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpCustomProfile_SensitivityLevelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDlpCustomProfile_SensitivityLevelsRef {
        DataZeroTrustDlpCustomProfile_SensitivityLevelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpCustomProfile_SensitivityLevelsRef {
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
pub struct DataZeroTrustDlpCustomProfile_SharedEntries {}

impl DataZeroTrustDlpCustomProfile_SharedEntries { }

impl terrars::ToListMappable for DataZeroTrustDlpCustomProfile_SharedEntries {
    type O = terrars::BlockAssignable<DataZeroTrustDlpCustomProfile_SharedEntries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDlpCustomProfile_SharedEntries {}

impl BuildDataZeroTrustDlpCustomProfile_SharedEntries {
    pub fn build(self) -> DataZeroTrustDlpCustomProfile_SharedEntries {
        DataZeroTrustDlpCustomProfile_SharedEntries {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDlpCustomProfile_SharedEntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDlpCustomProfile_SharedEntriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDlpCustomProfile_SharedEntriesRef {
        DataZeroTrustDlpCustomProfile_SharedEntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDlpCustomProfile_SharedEntriesRef {
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
    pub fn confidence(&self) -> DataZeroTrustDlpCustomProfile_Entries_ConfidenceRef {
        DataZeroTrustDlpCustomProfile_Entries_ConfidenceRef::new(
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
    pub fn pattern(&self) -> DataZeroTrustDlpCustomProfile_Entries_PatternRef {
        DataZeroTrustDlpCustomProfile_Entries_PatternRef::new(
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
    pub fn variant(&self) -> DataZeroTrustDlpCustomProfile_Entries_VariantRef {
        DataZeroTrustDlpCustomProfile_Entries_VariantRef::new(
            self.shared().clone(),
            format!("{}.variant", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}
