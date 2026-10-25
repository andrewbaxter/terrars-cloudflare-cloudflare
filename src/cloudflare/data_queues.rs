#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataQueuesData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
}

struct DataQueues_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataQueuesData_>,
}

#[derive(Clone)]
pub struct DataQueues(std::rc::Rc<DataQueues_>);

impl DataQueues {
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

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nA Resource identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataQueues_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataQueues {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataQueues { }

impl terrars::ToListMappable for DataQueues {
    type O = terrars::ListRef<DataQueuesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataQueues_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_queues".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataQueues {
    pub tf_id: std::string::String,
}

impl BuildDataQueues {
    pub fn build(self, stack: &mut terrars::Stack) -> DataQueues {
        let out = DataQueues(std::rc::Rc::new(DataQueues_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataQueuesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataQueuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueuesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataQueuesRef {
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

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataQueues_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataQueues_Result_Consumers_Settings {}

impl DataQueues_Result_Consumers_Settings { }

impl terrars::ToListMappable for DataQueues_Result_Consumers_Settings {
    type O = terrars::BlockAssignable<DataQueues_Result_Consumers_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQueues_Result_Consumers_Settings {}

impl BuildDataQueues_Result_Consumers_Settings {
    pub fn build(self) -> DataQueues_Result_Consumers_Settings {
        DataQueues_Result_Consumers_Settings {}
    }
}

#[allow(dead_code)]
pub struct DataQueues_Result_Consumers_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueues_Result_Consumers_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQueues_Result_Consumers_SettingsRef {
        DataQueues_Result_Consumers_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQueues_Result_Consumers_SettingsRef {
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
pub struct DataQueues_Result_Consumers {}

impl DataQueues_Result_Consumers { }

impl terrars::ToListMappable for DataQueues_Result_Consumers {
    type O = terrars::BlockAssignable<DataQueues_Result_Consumers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQueues_Result_Consumers {}

impl BuildDataQueues_Result_Consumers {
    pub fn build(self) -> DataQueues_Result_Consumers {
        DataQueues_Result_Consumers {}
    }
}

#[allow(dead_code)]
pub struct DataQueues_Result_ConsumersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueues_Result_ConsumersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQueues_Result_ConsumersRef {
        DataQueues_Result_ConsumersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQueues_Result_ConsumersRef {
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
    pub fn settings(&self) -> DataQueues_Result_Consumers_SettingsRef {
        DataQueues_Result_Consumers_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"worker\", \"http_pull\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQueues_Result_Producers {}

impl DataQueues_Result_Producers { }

impl terrars::ToListMappable for DataQueues_Result_Producers {
    type O = terrars::BlockAssignable<DataQueues_Result_Producers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQueues_Result_Producers {}

impl BuildDataQueues_Result_Producers {
    pub fn build(self) -> DataQueues_Result_Producers {
        DataQueues_Result_Producers {}
    }
}

#[allow(dead_code)]
pub struct DataQueues_Result_ProducersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueues_Result_ProducersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQueues_Result_ProducersRef {
        DataQueues_Result_ProducersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQueues_Result_ProducersRef {
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
pub struct DataQueues_Result_Settings {}

impl DataQueues_Result_Settings { }

impl terrars::ToListMappable for DataQueues_Result_Settings {
    type O = terrars::BlockAssignable<DataQueues_Result_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQueues_Result_Settings {}

impl BuildDataQueues_Result_Settings {
    pub fn build(self) -> DataQueues_Result_Settings {
        DataQueues_Result_Settings {}
    }
}

#[allow(dead_code)]
pub struct DataQueues_Result_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueues_Result_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQueues_Result_SettingsRef {
        DataQueues_Result_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQueues_Result_SettingsRef {
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

#[derive(Serialize)]
pub struct DataQueues_Result {}

impl DataQueues_Result { }

impl terrars::ToListMappable for DataQueues_Result {
    type O = terrars::BlockAssignable<DataQueues_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQueues_Result {}

impl BuildDataQueues_Result {
    pub fn build(self) -> DataQueues_Result {
        DataQueues_Result {}
    }
}

#[allow(dead_code)]
pub struct DataQueues_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQueues_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQueues_ResultRef {
        DataQueues_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQueues_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumers` after provisioning.\n"]
    pub fn consumers(&self) -> terrars::ListRef<DataQueues_Result_ConsumersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumers", self.base))
    }

    #[doc = "Get a reference to the value of field `consumers_total_count` after provisioning.\n"]
    pub fn consumers_total_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumers_total_count", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jurisdiction` after provisioning.\nAvailable values: \"eu\", \"us\", \"fedramp\"."]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc = "Get a reference to the value of field `producers` after provisioning.\n"]
    pub fn producers(&self) -> terrars::ListRef<DataQueues_Result_ProducersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.producers", self.base))
    }

    #[doc = "Get a reference to the value of field `producers_total_count` after provisioning.\n"]
    pub fn producers_total_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.producers_total_count", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\n"]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> DataQueues_Result_SettingsRef {
        DataQueues_Result_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
}
