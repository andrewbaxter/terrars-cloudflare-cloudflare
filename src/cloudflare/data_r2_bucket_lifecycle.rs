#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataR2BucketLifecycleData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    bucket_name: terrars::PrimField<std::string::String>,
}

struct DataR2BucketLifecycle_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataR2BucketLifecycleData_>,
}

#[derive(Clone)]
pub struct DataR2BucketLifecycle(std::rc::Rc<DataR2BucketLifecycle_>);

impl DataR2BucketLifecycle {
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

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID that owns the R2 resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nName of the bucket."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataR2BucketLifecycle_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl terrars::Referable for DataR2BucketLifecycle {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataR2BucketLifecycle { }

impl terrars::ToListMappable for DataR2BucketLifecycle {
    type O = terrars::ListRef<DataR2BucketLifecycleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataR2BucketLifecycle_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_r2_bucket_lifecycle".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataR2BucketLifecycle {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID that owns the R2 resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildDataR2BucketLifecycle {
    pub fn build(self, stack: &mut terrars::Stack) -> DataR2BucketLifecycle {
        let out = DataR2BucketLifecycle(std::rc::Rc::new(DataR2BucketLifecycle_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataR2BucketLifecycleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                bucket_name: self.bucket_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataR2BucketLifecycleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketLifecycleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataR2BucketLifecycleRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID that owns the R2 resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nName of the bucket."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataR2BucketLifecycle_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {}

impl DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition { }

impl terrars::ToListMappable for DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {
    type O = terrars::BlockAssignable<DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {}

impl BuildDataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {
    pub fn build(self) -> DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {
        DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
        DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"Age\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition {}

impl DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition { }

impl terrars::ToListMappable for DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition {
    type O = terrars::BlockAssignable<DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition {}

impl BuildDataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition {
    pub fn build(self) -> DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition {
        DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
        DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nCondition for lifecycle transitions to apply after an object reaches an age in seconds."]
    pub fn condition(&self) -> DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
        DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef::new(
            self.shared().clone(),
            format!("{}.condition", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataR2BucketLifecycle_Rules_Conditions {}

impl DataR2BucketLifecycle_Rules_Conditions { }

impl terrars::ToListMappable for DataR2BucketLifecycle_Rules_Conditions {
    type O = terrars::BlockAssignable<DataR2BucketLifecycle_Rules_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketLifecycle_Rules_Conditions {}

impl BuildDataR2BucketLifecycle_Rules_Conditions {
    pub fn build(self) -> DataR2BucketLifecycle_Rules_Conditions {
        DataR2BucketLifecycle_Rules_Conditions {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketLifecycle_Rules_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketLifecycle_Rules_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataR2BucketLifecycle_Rules_ConditionsRef {
        DataR2BucketLifecycle_Rules_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketLifecycle_Rules_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nTransitions will only apply to objects/uploads in the bucket that start with the given prefix, an empty prefix can be provided to scope rule to all objects/uploads."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataR2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {}

impl DataR2BucketLifecycle_Rules_DeleteObjectsTransition_Condition { }

impl terrars::ToListMappable for DataR2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {
    type O = terrars::BlockAssignable<DataR2BucketLifecycle_Rules_DeleteObjectsTransition_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {}

impl BuildDataR2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {
    pub fn build(self) -> DataR2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {
        DataR2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataR2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
        DataR2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `date` after provisioning.\n"]
    pub fn date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"Age\", \"Date\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataR2BucketLifecycle_Rules_DeleteObjectsTransition {}

impl DataR2BucketLifecycle_Rules_DeleteObjectsTransition { }

impl terrars::ToListMappable for DataR2BucketLifecycle_Rules_DeleteObjectsTransition {
    type O = terrars::BlockAssignable<DataR2BucketLifecycle_Rules_DeleteObjectsTransition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketLifecycle_Rules_DeleteObjectsTransition {}

impl BuildDataR2BucketLifecycle_Rules_DeleteObjectsTransition {
    pub fn build(self) -> DataR2BucketLifecycle_Rules_DeleteObjectsTransition {
        DataR2BucketLifecycle_Rules_DeleteObjectsTransition {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataR2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
        DataR2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nCondition for lifecycle transitions to apply after an object reaches an age in seconds."]
    pub fn condition(&self) -> DataR2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
        DataR2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef::new(
            self.shared().clone(),
            format!("{}.condition", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataR2BucketLifecycle_Rules_StorageClassTransitions_Condition {}

impl DataR2BucketLifecycle_Rules_StorageClassTransitions_Condition { }

impl terrars::ToListMappable for DataR2BucketLifecycle_Rules_StorageClassTransitions_Condition {
    type O = terrars::BlockAssignable<DataR2BucketLifecycle_Rules_StorageClassTransitions_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketLifecycle_Rules_StorageClassTransitions_Condition {}

impl BuildDataR2BucketLifecycle_Rules_StorageClassTransitions_Condition {
    pub fn build(self) -> DataR2BucketLifecycle_Rules_StorageClassTransitions_Condition {
        DataR2BucketLifecycle_Rules_StorageClassTransitions_Condition {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataR2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
        DataR2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `date` after provisioning.\n"]
    pub fn date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"Age\", \"Date\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataR2BucketLifecycle_Rules_StorageClassTransitions {}

impl DataR2BucketLifecycle_Rules_StorageClassTransitions { }

impl terrars::ToListMappable for DataR2BucketLifecycle_Rules_StorageClassTransitions {
    type O = terrars::BlockAssignable<DataR2BucketLifecycle_Rules_StorageClassTransitions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketLifecycle_Rules_StorageClassTransitions {}

impl BuildDataR2BucketLifecycle_Rules_StorageClassTransitions {
    pub fn build(self) -> DataR2BucketLifecycle_Rules_StorageClassTransitions {
        DataR2BucketLifecycle_Rules_StorageClassTransitions {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketLifecycle_Rules_StorageClassTransitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketLifecycle_Rules_StorageClassTransitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataR2BucketLifecycle_Rules_StorageClassTransitionsRef {
        DataR2BucketLifecycle_Rules_StorageClassTransitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketLifecycle_Rules_StorageClassTransitionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nCondition for lifecycle transitions to apply after an object reaches an age in seconds."]
    pub fn condition(&self) -> DataR2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
        DataR2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef::new(
            self.shared().clone(),
            format!("{}.condition", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `storage_class` after provisioning.\nAvailable values: \"InfrequentAccess\"."]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize)]
pub struct DataR2BucketLifecycle_Rules {}

impl DataR2BucketLifecycle_Rules { }

impl terrars::ToListMappable for DataR2BucketLifecycle_Rules {
    type O = terrars::BlockAssignable<DataR2BucketLifecycle_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketLifecycle_Rules {}

impl BuildDataR2BucketLifecycle_Rules {
    pub fn build(self) -> DataR2BucketLifecycle_Rules {
        DataR2BucketLifecycle_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketLifecycle_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketLifecycle_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataR2BucketLifecycle_RulesRef {
        DataR2BucketLifecycle_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketLifecycle_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `abort_multipart_uploads_transition` after provisioning.\nTransition to abort ongoing multipart uploads."]
    pub fn abort_multipart_uploads_transition(&self) -> DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
        DataR2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef::new(
            self.shared().clone(),
            format!("{}.abort_multipart_uploads_transition", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `conditions` after provisioning.\nConditions that apply to all transitions of this rule."]
    pub fn conditions(&self) -> DataR2BucketLifecycle_Rules_ConditionsRef {
        DataR2BucketLifecycle_Rules_ConditionsRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete_objects_transition` after provisioning.\nTransition to delete objects."]
    pub fn delete_objects_transition(&self) -> DataR2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
        DataR2BucketLifecycle_Rules_DeleteObjectsTransitionRef::new(
            self.shared().clone(),
            format!("{}.delete_objects_transition", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not this rule is in effect."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for this rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_class_transitions` after provisioning.\nTransitions to change the storage class of objects."]
    pub fn storage_class_transitions(&self) -> terrars::ListRef<DataR2BucketLifecycle_Rules_StorageClassTransitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_class_transitions", self.base))
    }
}
