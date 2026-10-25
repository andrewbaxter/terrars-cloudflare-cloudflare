#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataQueueConsumerData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    queue_id: terrars::PrimField<std::string::String>,
}

struct DataQueueConsumer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataQueueConsumerData_>,
}

#[derive(Clone)]
pub struct DataQueueConsumer(std::rc::Rc<DataQueueConsumer_>);

impl DataQueueConsumer {
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

    #[doc =
        "Get a reference to the value of field `dead_letter_queue` after provisioning.\nName of the dead letter queue, or empty string if not configured"]
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
    pub fn settings(&self) -> DataQueueConsumer_SettingsRef {
        DataQueueConsumer_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"worker\", \"http_pull\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataQueueConsumer {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataQueueConsumer { }

impl terrars::ToListMappable for DataQueueConsumer {
    type O = terrars::ListRef<DataQueueConsumerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataQueueConsumer_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_queue_consumer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataQueueConsumer {
    pub tf_id: std::string::String,
    #[doc = "A Resource identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "A Resource identifier."]
    pub queue_id: terrars::PrimField<std::string::String>,
}

impl BuildDataQueueConsumer {
    pub fn build(self, stack: &mut terrars::Stack) -> DataQueueConsumer {
        let out = DataQueueConsumer(std::rc::Rc::new(DataQueueConsumer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataQueueConsumerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                queue_id: self.queue_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataQueueConsumerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueueConsumerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataQueueConsumerRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc =
        "Get a reference to the value of field `dead_letter_queue` after provisioning.\nName of the dead letter queue, or empty string if not configured"]
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
    pub fn settings(&self) -> DataQueueConsumer_SettingsRef {
        DataQueueConsumer_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"worker\", \"http_pull\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataQueueConsumer_Settings {}

impl DataQueueConsumer_Settings { }

impl terrars::ToListMappable for DataQueueConsumer_Settings {
    type O = terrars::BlockAssignable<DataQueueConsumer_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQueueConsumer_Settings {}

impl BuildDataQueueConsumer_Settings {
    pub fn build(self) -> DataQueueConsumer_Settings {
        DataQueueConsumer_Settings {}
    }
}

#[allow(dead_code)]
pub struct DataQueueConsumer_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueueConsumer_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQueueConsumer_SettingsRef {
        DataQueueConsumer_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQueueConsumer_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `batch_size` after provisioning.\nThe maximum number of messages to include in a batch."]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
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
        "Get a reference to the value of field `retry_delay` after provisioning.\nThe number of seconds to delay before making the message available for another attempt."]
    pub fn retry_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_delay", self.base))
    }

    #[doc =
        "Get a reference to the value of field `visibility_timeout_ms` after provisioning.\nThe number of milliseconds that a message is exclusively leased. After the timeout, the message becomes available for another attempt."]
    pub fn visibility_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility_timeout_ms", self.base))
    }
}
