#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct R2BucketEventNotificationData_ {
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
    queue_id: terrars::PrimField<std::string::String>,
    rules: std::vec::Vec<R2BucketEventNotification_Rules>,
}

struct R2BucketEventNotification_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<R2BucketEventNotificationData_>,
}

#[derive(Clone)]
pub struct R2BucketEventNotification(std::rc::Rc<R2BucketEventNotification_>);

impl R2BucketEventNotification {
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

    #[doc =
        "Get a reference to the value of field `queue_id` after provisioning.\nID of the Cloudflare Queue that receives notifications for matching R2 object events."]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\nName of the queue."]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nArray of rules to drive notifications."]
    pub fn rules(&self) -> terrars::ListRef<R2BucketEventNotification_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl terrars::Referable for R2BucketEventNotification {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for R2BucketEventNotification { }

impl terrars::ToListMappable for R2BucketEventNotification {
    type O = terrars::ListRef<R2BucketEventNotificationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for R2BucketEventNotification_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_r2_bucket_event_notification".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildR2BucketEventNotification {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID that owns the R2 resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = "ID of the Cloudflare Queue that receives notifications for matching R2 object events."]
    pub queue_id: terrars::PrimField<std::string::String>,
    #[doc = "Array of rules to drive notifications."]
    pub rules: std::vec::Vec<R2BucketEventNotification_Rules>,
}

impl BuildR2BucketEventNotification {
    pub fn build(self, stack: &mut terrars::Stack) -> R2BucketEventNotification {
        let out = R2BucketEventNotification(std::rc::Rc::new(R2BucketEventNotification_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(R2BucketEventNotificationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                bucket_name: self.bucket_name,
                jurisdiction: core::default::Default::default(),
                queue_id: self.queue_id,
                rules: self.rules,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct R2BucketEventNotificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketEventNotificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl R2BucketEventNotificationRef {
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

    #[doc =
        "Get a reference to the value of field `queue_id` after provisioning.\nID of the Cloudflare Queue that receives notifications for matching R2 object events."]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\nName of the queue."]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\nArray of rules to drive notifications."]
    pub fn rules(&self) -> terrars::ListRef<R2BucketEventNotification_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct R2BucketEventNotification_Rules {
    actions: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl R2BucketEventNotification_Rules {
    #[doc =
        "Set the field `description`.\nA description that can be used to identify the event notification rule after creation."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nNotifications will be sent only for objects with this prefix."]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suffix`.\nNotifications will be sent only for objects with this suffix."]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for R2BucketEventNotification_Rules {
    type O = terrars::BlockAssignable<R2BucketEventNotification_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketEventNotification_Rules {
    #[doc = "Array of R2 object actions that will trigger notifications."]
    pub actions: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildR2BucketEventNotification_Rules {
    pub fn build(self) -> R2BucketEventNotification_Rules {
        R2BucketEventNotification_Rules {
            actions: self.actions,
            description: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct R2BucketEventNotification_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketEventNotification_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> R2BucketEventNotification_RulesRef {
        R2BucketEventNotification_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketEventNotification_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nArray of R2 object actions that will trigger notifications."]
    pub fn actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description that can be used to identify the event notification rule after creation."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nNotifications will be sent only for objects with this prefix."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nNotifications will be sent only for objects with this suffix."]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
