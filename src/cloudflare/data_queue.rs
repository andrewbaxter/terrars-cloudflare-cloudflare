#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataQueueData_ {
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
    queue_id: terrars::PrimField<std::string::String>,
}

struct DataQueue_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataQueueData_>,
}

#[derive(Clone)]
pub struct DataQueue(std::rc::Rc<DataQueue_>);

impl DataQueue {
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

    #[doc = "Set the field `account_id`.\nA Resource identifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nA Resource identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumers` after provisioning.\n"]
    pub fn consumers(&self) -> terrars::ListRef<DataQueue_ConsumersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumers_total_count` after provisioning.\n"]
    pub fn consumers_total_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumers_total_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nA Resource identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jurisdiction` after provisioning.\nAvailable values: \"eu\", \"us\", \"fedramp\"."]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `producers` after provisioning.\n"]
    pub fn producers(&self) -> terrars::ListRef<DataQueue_ProducersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.producers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `producers_total_count` after provisioning.\n"]
    pub fn producers_total_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.producers_total_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\nA Resource identifier."]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> DataQueue_SettingsRef {
        DataQueue_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl terrars::Referable for DataQueue {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataQueue { }

impl terrars::ToListMappable for DataQueue {
    type O = terrars::ListRef<DataQueueRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataQueue_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_queue".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataQueue {
    pub tf_id: std::string::String,
    #[doc = "A Resource identifier."]
    pub queue_id: terrars::PrimField<std::string::String>,
}

impl BuildDataQueue {
    pub fn build(self, stack: &mut terrars::Stack) -> DataQueue {
        let out = DataQueue(std::rc::Rc::new(DataQueue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataQueueData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                queue_id: self.queue_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataQueueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueueRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataQueueRef {
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

    #[doc = "Get a reference to the value of field `consumers` after provisioning.\n"]
    pub fn consumers(&self) -> terrars::ListRef<DataQueue_ConsumersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumers_total_count` after provisioning.\n"]
    pub fn consumers_total_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumers_total_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nA Resource identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jurisdiction` after provisioning.\nAvailable values: \"eu\", \"us\", \"fedramp\"."]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `producers` after provisioning.\n"]
    pub fn producers(&self) -> terrars::ListRef<DataQueue_ProducersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.producers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `producers_total_count` after provisioning.\n"]
    pub fn producers_total_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.producers_total_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\nA Resource identifier."]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> DataQueue_SettingsRef {
        DataQueue_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataQueue_Consumers_Settings {}

impl DataQueue_Consumers_Settings { }

impl terrars::ToListMappable for DataQueue_Consumers_Settings {
    type O = terrars::BlockAssignable<DataQueue_Consumers_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQueue_Consumers_Settings {}

impl BuildDataQueue_Consumers_Settings {
    pub fn build(self) -> DataQueue_Consumers_Settings {
        DataQueue_Consumers_Settings {}
    }
}

#[allow(dead_code)]
pub struct DataQueue_Consumers_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueue_Consumers_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQueue_Consumers_SettingsRef {
        DataQueue_Consumers_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQueue_Consumers_SettingsRef {
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

#[derive(Serialize)]
pub struct DataQueue_Consumers {}

impl DataQueue_Consumers { }

impl terrars::ToListMappable for DataQueue_Consumers {
    type O = terrars::BlockAssignable<DataQueue_Consumers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQueue_Consumers {}

impl BuildDataQueue_Consumers {
    pub fn build(self) -> DataQueue_Consumers {
        DataQueue_Consumers {}
    }
}

#[allow(dead_code)]
pub struct DataQueue_ConsumersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueue_ConsumersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQueue_ConsumersRef {
        DataQueue_ConsumersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQueue_ConsumersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_id` after provisioning.\nA Resource identifier."]
    pub fn consumer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dead_letter_queue` after provisioning.\nName of the dead letter queue, or empty string if not configured"]
    pub fn dead_letter_queue(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dead_letter_queue", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.base))
    }

    #[doc = "Get a reference to the value of field `script_name` after provisioning.\nName of a Worker"]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> DataQueue_Consumers_SettingsRef {
        DataQueue_Consumers_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"worker\", \"http_pull\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQueue_Producers {}

impl DataQueue_Producers { }

impl terrars::ToListMappable for DataQueue_Producers {
    type O = terrars::BlockAssignable<DataQueue_Producers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQueue_Producers {}

impl BuildDataQueue_Producers {
    pub fn build(self) -> DataQueue_Producers {
        DataQueue_Producers {}
    }
}

#[allow(dead_code)]
pub struct DataQueue_ProducersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueue_ProducersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQueue_ProducersRef {
        DataQueue_ProducersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQueue_ProducersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `script` after provisioning.\n"]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"worker\", \"r2_bucket\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQueue_Settings {}

impl DataQueue_Settings { }

impl terrars::ToListMappable for DataQueue_Settings {
    type O = terrars::BlockAssignable<DataQueue_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQueue_Settings {}

impl BuildDataQueue_Settings {
    pub fn build(self) -> DataQueue_Settings {
        DataQueue_Settings {}
    }
}

#[allow(dead_code)]
pub struct DataQueue_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueue_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQueue_SettingsRef {
        DataQueue_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQueue_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delivery_delay` after provisioning.\nNumber of seconds to delay delivery of all messages to consumers."]
    pub fn delivery_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_delay", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delivery_paused` after provisioning.\nIndicates if message delivery to consumers is currently paused."]
    pub fn delivery_paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_paused", self.base))
    }

    #[doc =
        "Get a reference to the value of field `message_retention_period` after provisioning.\nNumber of seconds after which an unconsumed message will be delayed."]
    pub fn message_retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_retention_period", self.base))
    }
}
