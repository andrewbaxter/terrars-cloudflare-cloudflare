#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct QueueConsumerData_ {
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
    dead_letter_queue: std::option::Option<terrars::PrimField<std::string::String>>,
    queue_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<QueueConsumer_Settings>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

struct QueueConsumer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<QueueConsumerData_>,
}

#[derive(Clone)]
pub struct QueueConsumer(std::rc::Rc<QueueConsumer_>);

impl QueueConsumer {
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

    #[doc = "Set the field `dead_letter_queue`.\n"]
    pub fn set_dead_letter_queue(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dead_letter_queue = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `script_name`.\nName of a Worker"]
    pub fn set_script_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().script_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(self, v: impl std::convert::Into<QueueConsumer_Settings>) -> Self {
        self.0.data.borrow_mut().settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nA Resource identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumer_id` after provisioning.\nA Resource identifier."]
    pub fn consumer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_queue` after provisioning.\n"]
    pub fn dead_letter_queue(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dead_letter_queue", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\nA Resource identifier."]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_name` after provisioning.\nName of a Worker"]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> QueueConsumer_SettingsRef {
        QueueConsumer_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"worker\", \"http_pull\", \"notification\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for QueueConsumer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for QueueConsumer { }

impl terrars::ToListMappable for QueueConsumer {
    type O = terrars::ListRef<QueueConsumerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for QueueConsumer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_queue_consumer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildQueueConsumer {
    pub tf_id: std::string::String,
    #[doc = "A Resource identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "A Resource identifier."]
    pub queue_id: terrars::PrimField<std::string::String>,
    #[doc = "Available values: \"worker\", \"http_pull\", \"notification\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildQueueConsumer {
    pub fn build(self, stack: &mut terrars::Stack) -> QueueConsumer {
        let out = QueueConsumer(std::rc::Rc::new(QueueConsumer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(QueueConsumerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                dead_letter_queue: core::default::Default::default(),
                queue_id: self.queue_id,
                script_name: core::default::Default::default(),
                settings: core::default::Default::default(),
                type_: self.type_,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct QueueConsumerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QueueConsumerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl QueueConsumerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nA Resource identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumer_id` after provisioning.\nA Resource identifier."]
    pub fn consumer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_queue` after provisioning.\n"]
    pub fn dead_letter_queue(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dead_letter_queue", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\nA Resource identifier."]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_name` after provisioning.\nName of a Worker"]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> QueueConsumer_SettingsRef {
        QueueConsumer_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"worker\", \"http_pull\", \"notification\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct QueueConsumer_Settings_Email {
    id: terrars::PrimField<std::string::String>,
}

impl QueueConsumer_Settings_Email { }

impl terrars::ToListMappable for QueueConsumer_Settings_Email {
    type O = terrars::BlockAssignable<QueueConsumer_Settings_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQueueConsumer_Settings_Email {
    #[doc = "The email address."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildQueueConsumer_Settings_Email {
    pub fn build(self) -> QueueConsumer_Settings_Email {
        QueueConsumer_Settings_Email { id: self.id }
    }
}

#[allow(dead_code)]
pub struct QueueConsumer_Settings_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QueueConsumer_Settings_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QueueConsumer_Settings_EmailRef {
        QueueConsumer_Settings_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QueueConsumer_Settings_EmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe email address."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct QueueConsumer_Settings_Pagerduty {
    id: terrars::PrimField<std::string::String>,
}

impl QueueConsumer_Settings_Pagerduty { }

impl terrars::ToListMappable for QueueConsumer_Settings_Pagerduty {
    type O = terrars::BlockAssignable<QueueConsumer_Settings_Pagerduty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQueueConsumer_Settings_Pagerduty {
    #[doc = "UUID."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildQueueConsumer_Settings_Pagerduty {
    pub fn build(self) -> QueueConsumer_Settings_Pagerduty {
        QueueConsumer_Settings_Pagerduty { id: self.id }
    }
}

#[allow(dead_code)]
pub struct QueueConsumer_Settings_PagerdutyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QueueConsumer_Settings_PagerdutyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QueueConsumer_Settings_PagerdutyRef {
        QueueConsumer_Settings_PagerdutyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QueueConsumer_Settings_PagerdutyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct QueueConsumer_Settings_Webhooks {
    id: terrars::PrimField<std::string::String>,
}

impl QueueConsumer_Settings_Webhooks { }

impl terrars::ToListMappable for QueueConsumer_Settings_Webhooks {
    type O = terrars::BlockAssignable<QueueConsumer_Settings_Webhooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQueueConsumer_Settings_Webhooks {
    #[doc = "UUID."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildQueueConsumer_Settings_Webhooks {
    pub fn build(self) -> QueueConsumer_Settings_Webhooks {
        QueueConsumer_Settings_Webhooks { id: self.id }
    }
}

#[allow(dead_code)]
pub struct QueueConsumer_Settings_WebhooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QueueConsumer_Settings_WebhooksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QueueConsumer_Settings_WebhooksRef {
        QueueConsumer_Settings_WebhooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QueueConsumer_Settings_WebhooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct QueueConsumer_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<std::vec::Vec<QueueConsumer_Settings_Email>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrency: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_wait_time_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pagerduty: std::option::Option<std::vec::Vec<QueueConsumer_Settings_Pagerduty>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_delay: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility_timeout_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhooks: std::option::Option<std::vec::Vec<QueueConsumer_Settings_Webhooks>>,
}

impl QueueConsumer_Settings {
    #[doc = "Set the field `batch_size`.\nThe maximum number of messages to include in a batch."]
    pub fn set_batch_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<std::vec::Vec<QueueConsumer_Settings_Email>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_concurrency`.\nMaximum number of concurrent consumers that may consume from this Queue. Set to `null` to automatically opt in to the platform's maximum (recommended)."]
    pub fn set_max_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\nThe maximum number of retries"]
    pub fn set_max_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_wait_time_ms`.\nThe number of milliseconds to wait for a batch to fill up before attempting to deliver it"]
    pub fn set_max_wait_time_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_wait_time_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pagerduty`.\nPagerDuty notification destinations."]
    pub fn set_pagerduty(mut self, v: impl std::convert::Into<std::vec::Vec<QueueConsumer_Settings_Pagerduty>>) -> Self {
        self.pagerduty = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_delay`.\nThe number of seconds to delay before making the message available for another attempt."]
    pub fn set_retry_delay(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `visibility_timeout_ms`.\nThe number of milliseconds that a message is exclusively leased. After the timeout, the message becomes available for another attempt."]
    pub fn set_visibility_timeout_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.visibility_timeout_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `webhooks`.\nWebhook notification destinations."]
    pub fn set_webhooks(mut self, v: impl std::convert::Into<std::vec::Vec<QueueConsumer_Settings_Webhooks>>) -> Self {
        self.webhooks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QueueConsumer_Settings {
    type O = terrars::BlockAssignable<QueueConsumer_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQueueConsumer_Settings {}

impl BuildQueueConsumer_Settings {
    pub fn build(self) -> QueueConsumer_Settings {
        QueueConsumer_Settings {
            batch_size: core::default::Default::default(),
            email: core::default::Default::default(),
            max_concurrency: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            max_wait_time_ms: core::default::Default::default(),
            pagerduty: core::default::Default::default(),
            retry_delay: core::default::Default::default(),
            visibility_timeout_ms: core::default::Default::default(),
            webhooks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QueueConsumer_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QueueConsumer_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QueueConsumer_SettingsRef {
        QueueConsumer_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QueueConsumer_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `batch_size` after provisioning.\nThe maximum number of messages to include in a batch."]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<QueueConsumer_Settings_EmailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrency` after provisioning.\nMaximum number of concurrent consumers that may consume from this Queue. Set to `null` to automatically opt in to the platform's maximum (recommended)."]
    pub fn max_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\nThe maximum number of retries"]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_wait_time_ms` after provisioning.\nThe number of milliseconds to wait for a batch to fill up before attempting to deliver it"]
    pub fn max_wait_time_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_wait_time_ms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pagerduty` after provisioning.\nPagerDuty notification destinations."]
    pub fn pagerduty(&self) -> terrars::ListRef<QueueConsumer_Settings_PagerdutyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pagerduty", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_delay` after provisioning.\nThe number of seconds to delay before making the message available for another attempt."]
    pub fn retry_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_delay", self.base))
    }

    #[doc =
        "Get a reference to the value of field `visibility_timeout_ms` after provisioning.\nThe number of milliseconds that a message is exclusively leased. After the timeout, the message becomes available for another attempt."]
    pub fn visibility_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility_timeout_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `webhooks` after provisioning.\nWebhook notification destinations."]
    pub fn webhooks(&self) -> terrars::ListRef<QueueConsumer_Settings_WebhooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhooks", self.base))
    }
}
