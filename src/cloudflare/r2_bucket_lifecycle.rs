#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct R2BucketLifecycleData_ {
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
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jurisdiction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<R2BucketLifecycle_Rules>>,
}

struct R2BucketLifecycle_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<R2BucketLifecycleData_>,
}

#[derive(Clone)]
pub struct R2BucketLifecycle(std::rc::Rc<R2BucketLifecycle_>);

impl R2BucketLifecycle {
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

    #[doc = "Set the field `jurisdiction`.\nJurisdiction of the bucket"]
    pub fn set_jurisdiction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().jurisdiction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl std::convert::Into<std::vec::Vec<R2BucketLifecycle_Rules>>) -> Self {
        self.0.data.borrow_mut().rules = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `jurisdiction` after provisioning.\nJurisdiction of the bucket"]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<R2BucketLifecycle_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl terrars::Referable for R2BucketLifecycle {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for R2BucketLifecycle { }

impl terrars::ToListMappable for R2BucketLifecycle {
    type O = terrars::ListRef<R2BucketLifecycleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for R2BucketLifecycle_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_r2_bucket_lifecycle".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildR2BucketLifecycle {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID that owns the R2 resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildR2BucketLifecycle {
    pub fn build(self, stack: &mut terrars::Stack) -> R2BucketLifecycle {
        let out = R2BucketLifecycle(std::rc::Rc::new(R2BucketLifecycle_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(R2BucketLifecycleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                bucket_name: self.bucket_name,
                jurisdiction: core::default::Default::default(),
                rules: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct R2BucketLifecycleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketLifecycleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl R2BucketLifecycleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `jurisdiction` after provisioning.\nJurisdiction of the bucket"]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<R2BucketLifecycle_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {
    max_age: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition { }

impl terrars::ToListMappable for R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {
    type O = terrars::BlockAssignable<R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {
    #[doc = ""]
    pub max_age: terrars::PrimField<f64>,
    #[doc = "Available values: \"Age\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildR2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {
    pub fn build(self) -> R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {
        R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition {
            max_age: self.max_age,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
        R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
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
pub struct R2BucketLifecycle_Rules_AbortMultipartUploadsTransition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition>,
}

impl R2BucketLifecycle_Rules_AbortMultipartUploadsTransition {
    #[doc =
        "Set the field `condition`.\nCondition for lifecycle transitions to apply after an object reaches an age in seconds."]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_Condition>,
    ) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for R2BucketLifecycle_Rules_AbortMultipartUploadsTransition {
    type O = terrars::BlockAssignable<R2BucketLifecycle_Rules_AbortMultipartUploadsTransition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketLifecycle_Rules_AbortMultipartUploadsTransition {}

impl BuildR2BucketLifecycle_Rules_AbortMultipartUploadsTransition {
    pub fn build(self) -> R2BucketLifecycle_Rules_AbortMultipartUploadsTransition {
        R2BucketLifecycle_Rules_AbortMultipartUploadsTransition { condition: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct R2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> R2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
        R2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nCondition for lifecycle transitions to apply after an object reaches an age in seconds."]
    pub fn condition(&self) -> R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef {
        R2BucketLifecycle_Rules_AbortMultipartUploadsTransition_ConditionRef::new(
            self.shared().clone(),
            format!("{}.condition", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct R2BucketLifecycle_Rules_Conditions {
    prefix: terrars::PrimField<std::string::String>,
}

impl R2BucketLifecycle_Rules_Conditions { }

impl terrars::ToListMappable for R2BucketLifecycle_Rules_Conditions {
    type O = terrars::BlockAssignable<R2BucketLifecycle_Rules_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketLifecycle_Rules_Conditions {
    #[doc =
        "Transitions will only apply to objects/uploads in the bucket that start with the given prefix, an empty prefix can be provided to scope rule to all objects/uploads."]
    pub prefix: terrars::PrimField<std::string::String>,
}

impl BuildR2BucketLifecycle_Rules_Conditions {
    pub fn build(self) -> R2BucketLifecycle_Rules_Conditions {
        R2BucketLifecycle_Rules_Conditions { prefix: self.prefix }
    }
}

#[allow(dead_code)]
pub struct R2BucketLifecycle_Rules_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketLifecycle_Rules_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> R2BucketLifecycle_Rules_ConditionsRef {
        R2BucketLifecycle_Rules_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketLifecycle_Rules_ConditionsRef {
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
pub struct R2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl R2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {
    #[doc = "Set the field `date`.\n"]
    pub fn set_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for R2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {
    type O = terrars::BlockAssignable<R2BucketLifecycle_Rules_DeleteObjectsTransition_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {
    #[doc = "Available values: \"Age\", \"Date\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildR2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {
    pub fn build(self) -> R2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {
        R2BucketLifecycle_Rules_DeleteObjectsTransition_Condition {
            date: core::default::Default::default(),
            max_age: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct R2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> R2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
        R2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
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
pub struct R2BucketLifecycle_Rules_DeleteObjectsTransition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<R2BucketLifecycle_Rules_DeleteObjectsTransition_Condition>,
}

impl R2BucketLifecycle_Rules_DeleteObjectsTransition {
    #[doc =
        "Set the field `condition`.\nCondition for lifecycle transitions to apply after an object reaches an age in seconds."]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<R2BucketLifecycle_Rules_DeleteObjectsTransition_Condition>,
    ) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for R2BucketLifecycle_Rules_DeleteObjectsTransition {
    type O = terrars::BlockAssignable<R2BucketLifecycle_Rules_DeleteObjectsTransition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketLifecycle_Rules_DeleteObjectsTransition {}

impl BuildR2BucketLifecycle_Rules_DeleteObjectsTransition {
    pub fn build(self) -> R2BucketLifecycle_Rules_DeleteObjectsTransition {
        R2BucketLifecycle_Rules_DeleteObjectsTransition { condition: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct R2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> R2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
        R2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nCondition for lifecycle transitions to apply after an object reaches an age in seconds."]
    pub fn condition(&self) -> R2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef {
        R2BucketLifecycle_Rules_DeleteObjectsTransition_ConditionRef::new(
            self.shared().clone(),
            format!("{}.condition", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct R2BucketLifecycle_Rules_StorageClassTransitions_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl R2BucketLifecycle_Rules_StorageClassTransitions_Condition {
    #[doc = "Set the field `date`.\n"]
    pub fn set_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for R2BucketLifecycle_Rules_StorageClassTransitions_Condition {
    type O = terrars::BlockAssignable<R2BucketLifecycle_Rules_StorageClassTransitions_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketLifecycle_Rules_StorageClassTransitions_Condition {
    #[doc = "Available values: \"Age\", \"Date\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildR2BucketLifecycle_Rules_StorageClassTransitions_Condition {
    pub fn build(self) -> R2BucketLifecycle_Rules_StorageClassTransitions_Condition {
        R2BucketLifecycle_Rules_StorageClassTransitions_Condition {
            date: core::default::Default::default(),
            max_age: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct R2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> R2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
        R2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
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
pub struct R2BucketLifecycle_Rules_StorageClassTransitions {
    condition: R2BucketLifecycle_Rules_StorageClassTransitions_Condition,
    storage_class: terrars::PrimField<std::string::String>,
}

impl R2BucketLifecycle_Rules_StorageClassTransitions { }

impl terrars::ToListMappable for R2BucketLifecycle_Rules_StorageClassTransitions {
    type O = terrars::BlockAssignable<R2BucketLifecycle_Rules_StorageClassTransitions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketLifecycle_Rules_StorageClassTransitions {
    #[doc = "Condition for lifecycle transitions to apply after an object reaches an age in seconds."]
    pub condition: R2BucketLifecycle_Rules_StorageClassTransitions_Condition,
    #[doc = "Available values: \"InfrequentAccess\"."]
    pub storage_class: terrars::PrimField<std::string::String>,
}

impl BuildR2BucketLifecycle_Rules_StorageClassTransitions {
    pub fn build(self) -> R2BucketLifecycle_Rules_StorageClassTransitions {
        R2BucketLifecycle_Rules_StorageClassTransitions {
            condition: self.condition,
            storage_class: self.storage_class,
        }
    }
}

#[allow(dead_code)]
pub struct R2BucketLifecycle_Rules_StorageClassTransitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketLifecycle_Rules_StorageClassTransitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> R2BucketLifecycle_Rules_StorageClassTransitionsRef {
        R2BucketLifecycle_Rules_StorageClassTransitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketLifecycle_Rules_StorageClassTransitionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nCondition for lifecycle transitions to apply after an object reaches an age in seconds."]
    pub fn condition(&self) -> R2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef {
        R2BucketLifecycle_Rules_StorageClassTransitions_ConditionRef::new(
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
pub struct R2BucketLifecycle_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort_multipart_uploads_transition: std::option::Option<R2BucketLifecycle_Rules_AbortMultipartUploadsTransition>,
    conditions: R2BucketLifecycle_Rules_Conditions,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_objects_transition: std::option::Option<R2BucketLifecycle_Rules_DeleteObjectsTransition>,
    enabled: terrars::PrimField<bool>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class_transitions: std::option::Option<std::vec::Vec<R2BucketLifecycle_Rules_StorageClassTransitions>>,
}

impl R2BucketLifecycle_Rules {
    #[doc = "Set the field `abort_multipart_uploads_transition`.\nTransition to abort ongoing multipart uploads."]
    pub fn set_abort_multipart_uploads_transition(
        mut self,
        v: impl std::convert::Into<R2BucketLifecycle_Rules_AbortMultipartUploadsTransition>,
    ) -> Self {
        self.abort_multipart_uploads_transition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_objects_transition`.\nTransition to delete objects."]
    pub fn set_delete_objects_transition(
        mut self,
        v: impl std::convert::Into<R2BucketLifecycle_Rules_DeleteObjectsTransition>,
    ) -> Self {
        self.delete_objects_transition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_class_transitions`.\nTransitions to change the storage class of objects."]
    pub fn set_storage_class_transitions(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<R2BucketLifecycle_Rules_StorageClassTransitions>>,
    ) -> Self {
        self.storage_class_transitions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for R2BucketLifecycle_Rules {
    type O = terrars::BlockAssignable<R2BucketLifecycle_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketLifecycle_Rules {
    #[doc = "Conditions that apply to all transitions of this rule."]
    pub conditions: R2BucketLifecycle_Rules_Conditions,
    #[doc = "Whether or not this rule is in effect."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "Unique identifier for this rule."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildR2BucketLifecycle_Rules {
    pub fn build(self) -> R2BucketLifecycle_Rules {
        R2BucketLifecycle_Rules {
            abort_multipart_uploads_transition: core::default::Default::default(),
            conditions: self.conditions,
            delete_objects_transition: core::default::Default::default(),
            enabled: self.enabled,
            id: self.id,
            storage_class_transitions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct R2BucketLifecycle_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketLifecycle_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> R2BucketLifecycle_RulesRef {
        R2BucketLifecycle_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketLifecycle_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `abort_multipart_uploads_transition` after provisioning.\nTransition to abort ongoing multipart uploads."]
    pub fn abort_multipart_uploads_transition(&self) -> R2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef {
        R2BucketLifecycle_Rules_AbortMultipartUploadsTransitionRef::new(
            self.shared().clone(),
            format!("{}.abort_multipart_uploads_transition", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `conditions` after provisioning.\nConditions that apply to all transitions of this rule."]
    pub fn conditions(&self) -> R2BucketLifecycle_Rules_ConditionsRef {
        R2BucketLifecycle_Rules_ConditionsRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete_objects_transition` after provisioning.\nTransition to delete objects."]
    pub fn delete_objects_transition(&self) -> R2BucketLifecycle_Rules_DeleteObjectsTransitionRef {
        R2BucketLifecycle_Rules_DeleteObjectsTransitionRef::new(
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
    pub fn storage_class_transitions(&self) -> terrars::ListRef<R2BucketLifecycle_Rules_StorageClassTransitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_class_transitions", self.base))
    }
}
