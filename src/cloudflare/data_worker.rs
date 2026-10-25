#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataWorkerData_ {
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
    filter: std::option::Option<DataWorker_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataWorker_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWorkerData_>,
}

#[derive(Clone)]
pub struct DataWorker(std::rc::Rc<DataWorker_>);

impl DataWorker {
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

    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataWorker_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_id`.\nIdentifier for the Worker, which can be ID or name."]
    pub fn set_worker_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().worker_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the Worker was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_on` after provisioning.\nWhen the Worker's most recent deployment was created. `null` if the Worker has never been deployed."]
    pub fn deployed_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataWorker_FilterRef {
        DataWorker_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIdentifier for the Worker, which can be ID or name."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logpush` after provisioning.\nWhether logpush is enabled for the Worker."]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Worker."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observability` after provisioning.\nObservability settings for the Worker."]
    pub fn observability(&self) -> DataWorker_ObservabilityRef {
        DataWorker_ObservabilityRef::new(self.shared().clone(), format!("{}.observability", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `previews_base_config` after provisioning.\nTemplate configuration used when creating new Previews for this Worker."]
    pub fn previews_base_config(&self) -> DataWorker_PreviewsBaseConfigRef {
        DataWorker_PreviewsBaseConfigRef::new(
            self.shared().clone(),
            format!("{}.previews_base_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `references` after provisioning.\nOther resources that reference the Worker and depend on it existing."]
    pub fn references(&self) -> DataWorker_ReferencesRef {
        DataWorker_ReferencesRef::new(self.shared().clone(), format!("{}.references", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\nSubdomain settings for the Worker."]
    pub fn subdomain(&self) -> DataWorker_SubdomainRef {
        DataWorker_SubdomainRef::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags associated with the Worker."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tail_consumers` after provisioning.\nOther Workers that should consume logs from the Worker."]
    pub fn tail_consumers(&self) -> terrars::SetRef<DataWorker_TailConsumersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tail_consumers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_on` after provisioning.\nWhen the Worker was most recently updated."]
    pub fn updated_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `worker_id` after provisioning.\nIdentifier for the Worker, which can be ID or name."]
    pub fn worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataWorker {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWorker { }

impl terrars::ToListMappable for DataWorker {
    type O = terrars::ListRef<DataWorkerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWorker_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_worker".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWorker {
    pub tf_id: std::string::String,
}

impl BuildDataWorker {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWorker {
        let out = DataWorker(std::rc::Rc::new(DataWorker_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWorkerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                worker_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWorkerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWorkerRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the Worker was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_on` after provisioning.\nWhen the Worker's most recent deployment was created. `null` if the Worker has never been deployed."]
    pub fn deployed_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataWorker_FilterRef {
        DataWorker_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIdentifier for the Worker, which can be ID or name."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logpush` after provisioning.\nWhether logpush is enabled for the Worker."]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Worker."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observability` after provisioning.\nObservability settings for the Worker."]
    pub fn observability(&self) -> DataWorker_ObservabilityRef {
        DataWorker_ObservabilityRef::new(self.shared().clone(), format!("{}.observability", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `previews_base_config` after provisioning.\nTemplate configuration used when creating new Previews for this Worker."]
    pub fn previews_base_config(&self) -> DataWorker_PreviewsBaseConfigRef {
        DataWorker_PreviewsBaseConfigRef::new(
            self.shared().clone(),
            format!("{}.previews_base_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `references` after provisioning.\nOther resources that reference the Worker and depend on it existing."]
    pub fn references(&self) -> DataWorker_ReferencesRef {
        DataWorker_ReferencesRef::new(self.shared().clone(), format!("{}.references", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\nSubdomain settings for the Worker."]
    pub fn subdomain(&self) -> DataWorker_SubdomainRef {
        DataWorker_SubdomainRef::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags associated with the Worker."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tail_consumers` after provisioning.\nOther Workers that should consume logs from the Worker."]
    pub fn tail_consumers(&self) -> terrars::SetRef<DataWorker_TailConsumersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tail_consumers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_on` after provisioning.\nWhen the Worker was most recently updated."]
    pub fn updated_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `worker_id` after provisioning.\nIdentifier for the Worker, which can be ID or name."]
    pub fn worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWorker_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order_by: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWorker_Filter {
    #[doc = "Set the field `order`.\nSort direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order_by`.\nProperty to sort results by.\nAvailable values: \"deployed_on\", \"updated_on\", \"created_on\", \"name\"."]
    pub fn set_order_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order_by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorker_Filter {
    type O = terrars::BlockAssignable<DataWorker_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_Filter {}

impl BuildDataWorker_Filter {
    pub fn build(self) -> DataWorker_Filter {
        DataWorker_Filter {
            order: core::default::Default::default(),
            order_by: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWorker_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_FilterRef {
        DataWorker_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nSort direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order_by` after provisioning.\nProperty to sort results by.\nAvailable values: \"deployed_on\", \"updated_on\", \"created_on\", \"name\"."]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_Observability_Issues {}

impl DataWorker_Observability_Issues { }

impl terrars::ToListMappable for DataWorker_Observability_Issues {
    type O = terrars::BlockAssignable<DataWorker_Observability_Issues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_Observability_Issues {}

impl BuildDataWorker_Observability_Issues {
    pub fn build(self) -> DataWorker_Observability_Issues {
        DataWorker_Observability_Issues {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_Observability_IssuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_Observability_IssuesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_Observability_IssuesRef {
        DataWorker_Observability_IssuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_Observability_IssuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether real-time Issues are enabled for the Worker."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_Observability_Logs {}

impl DataWorker_Observability_Logs { }

impl terrars::ToListMappable for DataWorker_Observability_Logs {
    type O = terrars::BlockAssignable<DataWorker_Observability_Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_Observability_Logs {}

impl BuildDataWorker_Observability_Logs {
    pub fn build(self) -> DataWorker_Observability_Logs {
        DataWorker_Observability_Logs {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_Observability_LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_Observability_LogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_Observability_LogsRef {
        DataWorker_Observability_LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_Observability_LogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `destinations` after provisioning.\nA list of destinations where logs will be exported to."]
    pub fn destinations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether logs are enabled for the Worker."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `head_sampling_rate` after provisioning.\nThe sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn head_sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.head_sampling_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invocation_logs` after provisioning.\nWhether [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs) are enabled for the Worker."]
    pub fn invocation_logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_logs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `persist` after provisioning.\nWhether log persistence is enabled for the Worker."]
    pub fn persist(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persist", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_Observability_Traces {}

impl DataWorker_Observability_Traces { }

impl terrars::ToListMappable for DataWorker_Observability_Traces {
    type O = terrars::BlockAssignable<DataWorker_Observability_Traces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_Observability_Traces {}

impl BuildDataWorker_Observability_Traces {
    pub fn build(self) -> DataWorker_Observability_Traces {
        DataWorker_Observability_Traces {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_Observability_TracesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_Observability_TracesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_Observability_TracesRef {
        DataWorker_Observability_TracesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_Observability_TracesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `destinations` after provisioning.\nA list of destinations where traces will be exported to."]
    pub fn destinations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether traces are enabled for the Worker."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `head_sampling_rate` after provisioning.\nThe sampling rate for traces. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn head_sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.head_sampling_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `persist` after provisioning.\nWhether trace persistence is enabled for the Worker."]
    pub fn persist(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persist", self.base))
    }

    #[doc =
        "Get a reference to the value of field `propagation_policy` after provisioning.\nControls how inbound trace context (traceparent/tracestate) headers on incoming requests are handled. \"authenticated\" honors inbound trace context only when accompanied by a valid trace auth token. \"accept\" unconditionally accepts inbound trace context. Requires the trace propagation feature to be enabled. Returns null when the trace propagation feature is not enabled for the account.\nAvailable values: \"authenticated\", \"accept\"."]
    pub fn propagation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagation_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_Observability {}

impl DataWorker_Observability { }

impl terrars::ToListMappable for DataWorker_Observability {
    type O = terrars::BlockAssignable<DataWorker_Observability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_Observability {}

impl BuildDataWorker_Observability {
    pub fn build(self) -> DataWorker_Observability {
        DataWorker_Observability {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_ObservabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_ObservabilityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_ObservabilityRef {
        DataWorker_ObservabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_ObservabilityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether observability is enabled for the Worker."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `head_sampling_rate` after provisioning.\nThe sampling rate for observability. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn head_sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.head_sampling_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issues` after provisioning.\nReal-time Issues settings for the Worker."]
    pub fn issues(&self) -> DataWorker_Observability_IssuesRef {
        DataWorker_Observability_IssuesRef::new(self.shared().clone(), format!("{}.issues", self.base))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\nLog settings for the Worker."]
    pub fn logs(&self) -> DataWorker_Observability_LogsRef {
        DataWorker_Observability_LogsRef::new(self.shared().clone(), format!("{}.logs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redact_query_string` after provisioning.\nWhether query strings are removed from request URLs in logs and traces."]
    pub fn redact_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redact_query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `traces` after provisioning.\nTrace settings for the Worker."]
    pub fn traces(&self) -> DataWorker_Observability_TracesRef {
        DataWorker_Observability_TracesRef::new(self.shared().clone(), format!("{}.traces", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig_CacheOptions {}

impl DataWorker_PreviewsBaseConfig_CacheOptions { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig_CacheOptions {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig_CacheOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig_CacheOptions {}

impl BuildDataWorker_PreviewsBaseConfig_CacheOptions {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig_CacheOptions {
        DataWorker_PreviewsBaseConfig_CacheOptions {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfig_CacheOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfig_CacheOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_PreviewsBaseConfig_CacheOptionsRef {
        DataWorker_PreviewsBaseConfig_CacheOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfig_CacheOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cross_version_cache` after provisioning.\nWhether cached responses are shared across Worker version\nuploads. This is independent of `enabled`. It can stay true\nwhile caching is off, so the preference survives turning\ncaching off and back on."]
    pub fn cross_version_cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_version_cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether caching is enabled for this Worker."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig_Env {}

impl DataWorker_PreviewsBaseConfig_Env { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig_Env {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig_Env {}

impl BuildDataWorker_PreviewsBaseConfig_Env {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig_Env {
        DataWorker_PreviewsBaseConfig_Env {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfig_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfig_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_PreviewsBaseConfig_EnvRef {
        DataWorker_PreviewsBaseConfig_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfig_EnvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe kind of resource that the binding provides."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig_Limits {}

impl DataWorker_PreviewsBaseConfig_Limits { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig_Limits {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig_Limits {}

impl BuildDataWorker_PreviewsBaseConfig_Limits {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig_Limits {
        DataWorker_PreviewsBaseConfig_Limits {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfig_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfig_LimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_PreviewsBaseConfig_LimitsRef {
        DataWorker_PreviewsBaseConfig_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfig_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu_ms` after provisioning.\nThe amount of CPU time this Worker can use in milliseconds."]
    pub fn cpu_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_ms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subrequests` after provisioning.\nThe number of subrequests this Worker can make per request."]
    pub fn subrequests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subrequests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig_Observability_Issues {}

impl DataWorker_PreviewsBaseConfig_Observability_Issues { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig_Observability_Issues {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig_Observability_Issues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig_Observability_Issues {}

impl BuildDataWorker_PreviewsBaseConfig_Observability_Issues {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig_Observability_Issues {
        DataWorker_PreviewsBaseConfig_Observability_Issues {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfig_Observability_IssuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfig_Observability_IssuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorker_PreviewsBaseConfig_Observability_IssuesRef {
        DataWorker_PreviewsBaseConfig_Observability_IssuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfig_Observability_IssuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether real-time Issues are enabled for the Worker."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig_Observability_Logs {}

impl DataWorker_PreviewsBaseConfig_Observability_Logs { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig_Observability_Logs {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig_Observability_Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig_Observability_Logs {}

impl BuildDataWorker_PreviewsBaseConfig_Observability_Logs {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig_Observability_Logs {
        DataWorker_PreviewsBaseConfig_Observability_Logs {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfig_Observability_LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfig_Observability_LogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorker_PreviewsBaseConfig_Observability_LogsRef {
        DataWorker_PreviewsBaseConfig_Observability_LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfig_Observability_LogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `destinations` after provisioning.\nA list of destinations where logs will be exported to."]
    pub fn destinations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether logs are enabled for the Worker."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `head_sampling_rate` after provisioning.\nThe sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn head_sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.head_sampling_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invocation_logs` after provisioning.\nWhether [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs) are enabled for the Worker."]
    pub fn invocation_logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_logs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `persist` after provisioning.\nWhether log persistence is enabled for the Worker."]
    pub fn persist(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persist", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig_Observability_Traces {}

impl DataWorker_PreviewsBaseConfig_Observability_Traces { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig_Observability_Traces {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig_Observability_Traces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig_Observability_Traces {}

impl BuildDataWorker_PreviewsBaseConfig_Observability_Traces {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig_Observability_Traces {
        DataWorker_PreviewsBaseConfig_Observability_Traces {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfig_Observability_TracesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfig_Observability_TracesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorker_PreviewsBaseConfig_Observability_TracesRef {
        DataWorker_PreviewsBaseConfig_Observability_TracesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfig_Observability_TracesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `destinations` after provisioning.\nA list of destinations where traces will be exported to."]
    pub fn destinations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether traces are enabled for the Worker."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `head_sampling_rate` after provisioning.\nThe sampling rate for traces. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn head_sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.head_sampling_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `persist` after provisioning.\nWhether trace persistence is enabled for the Worker."]
    pub fn persist(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persist", self.base))
    }

    #[doc =
        "Get a reference to the value of field `propagation_policy` after provisioning.\nControls how inbound trace context (traceparent/tracestate) headers on incoming requests are handled. \"authenticated\" honors inbound trace context only when accompanied by a valid trace auth token. \"accept\" unconditionally accepts inbound trace context. Requires the trace propagation feature to be enabled. Returns null when the trace propagation feature is not enabled for the account.\nAvailable values: \"authenticated\", \"accept\"."]
    pub fn propagation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagation_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig_Observability {}

impl DataWorker_PreviewsBaseConfig_Observability { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig_Observability {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig_Observability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig_Observability {}

impl BuildDataWorker_PreviewsBaseConfig_Observability {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig_Observability {
        DataWorker_PreviewsBaseConfig_Observability {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfig_ObservabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfig_ObservabilityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorker_PreviewsBaseConfig_ObservabilityRef {
        DataWorker_PreviewsBaseConfig_ObservabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfig_ObservabilityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether observability is enabled for the Worker."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `head_sampling_rate` after provisioning.\nThe sampling rate for observability. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn head_sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.head_sampling_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issues` after provisioning.\nReal-time Issues settings for the Worker."]
    pub fn issues(&self) -> DataWorker_PreviewsBaseConfig_Observability_IssuesRef {
        DataWorker_PreviewsBaseConfig_Observability_IssuesRef::new(
            self.shared().clone(),
            format!("{}.issues", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\nLog settings for the Worker."]
    pub fn logs(&self) -> DataWorker_PreviewsBaseConfig_Observability_LogsRef {
        DataWorker_PreviewsBaseConfig_Observability_LogsRef::new(
            self.shared().clone(),
            format!("{}.logs", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `redact_query_string` after provisioning.\nWhether query strings are removed from request URLs in logs and traces."]
    pub fn redact_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redact_query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `traces` after provisioning.\nTrace settings for the Worker."]
    pub fn traces(&self) -> DataWorker_PreviewsBaseConfig_Observability_TracesRef {
        DataWorker_PreviewsBaseConfig_Observability_TracesRef::new(
            self.shared().clone(),
            format!("{}.traces", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig_Placement_Target {}

impl DataWorker_PreviewsBaseConfig_Placement_Target { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig_Placement_Target {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig_Placement_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig_Placement_Target {}

impl BuildDataWorker_PreviewsBaseConfig_Placement_Target {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig_Placement_Target {
        DataWorker_PreviewsBaseConfig_Placement_Target {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfig_Placement_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfig_Placement_TargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorker_PreviewsBaseConfig_Placement_TargetRef {
        DataWorker_PreviewsBaseConfig_Placement_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfig_Placement_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nTCP host:port for targeted placement."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHTTP hostname for targeted placement."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nCloud region in format 'provider:region'."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig_Placement {}

impl DataWorker_PreviewsBaseConfig_Placement { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig_Placement {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig_Placement {}

impl BuildDataWorker_PreviewsBaseConfig_Placement {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig_Placement {
        DataWorker_PreviewsBaseConfig_Placement {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfig_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfig_PlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_PreviewsBaseConfig_PlacementRef {
        DataWorker_PreviewsBaseConfig_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfig_PlacementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nTCP host and port for targeted placement."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHTTP hostname for targeted placement."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nEnables [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).\nAvailable values: \"smart\", \"targeted\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nCloud region for targeted placement in format 'provider:region'."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nArray of placement targets (currently limited to single target)."]
    pub fn target(&self) -> terrars::ListRef<DataWorker_PreviewsBaseConfig_Placement_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig_TailConsumers {}

impl DataWorker_PreviewsBaseConfig_TailConsumers { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig_TailConsumers {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig_TailConsumers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig_TailConsumers {}

impl BuildDataWorker_PreviewsBaseConfig_TailConsumers {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig_TailConsumers {
        DataWorker_PreviewsBaseConfig_TailConsumers {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfig_TailConsumersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfig_TailConsumersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorker_PreviewsBaseConfig_TailConsumersRef {
        DataWorker_PreviewsBaseConfig_TailConsumersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfig_TailConsumersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the consumer Worker."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_PreviewsBaseConfig {}

impl DataWorker_PreviewsBaseConfig { }

impl terrars::ToListMappable for DataWorker_PreviewsBaseConfig {
    type O = terrars::BlockAssignable<DataWorker_PreviewsBaseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_PreviewsBaseConfig {}

impl BuildDataWorker_PreviewsBaseConfig {
    pub fn build(self) -> DataWorker_PreviewsBaseConfig {
        DataWorker_PreviewsBaseConfig {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_PreviewsBaseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_PreviewsBaseConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_PreviewsBaseConfigRef {
        DataWorker_PreviewsBaseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_PreviewsBaseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache_options` after provisioning.\nCache options used when creating new Previews."]
    pub fn cache_options(&self) -> DataWorker_PreviewsBaseConfig_CacheOptionsRef {
        DataWorker_PreviewsBaseConfig_CacheOptionsRef::new(
            self.shared().clone(),
            format!("{}.cache_options", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nBindings used when creating new Previews, keyed by binding name."]
    pub fn env(&self) -> terrars::RecRef<DataWorker_PreviewsBaseConfig_EnvRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `limits` after provisioning.\nResource limits enforced at runtime for newly created Previews."]
    pub fn limits(&self) -> DataWorker_PreviewsBaseConfig_LimitsRef {
        DataWorker_PreviewsBaseConfig_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logpush` after provisioning.\nWhether logpush is enabled when creating new Previews."]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.base))
    }

    #[doc =
        "Get a reference to the value of field `observability` after provisioning.\nObservability settings used when creating new Previews."]
    pub fn observability(&self) -> DataWorker_PreviewsBaseConfig_ObservabilityRef {
        DataWorker_PreviewsBaseConfig_ObservabilityRef::new(
            self.shared().clone(),
            format!("{}.observability", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nPlacement configuration used when creating new Previews."]
    pub fn placement(&self) -> DataWorker_PreviewsBaseConfig_PlacementRef {
        DataWorker_PreviewsBaseConfig_PlacementRef::new(self.shared().clone(), format!("{}.placement", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tail_consumers` after provisioning.\nOther Workers that should consume logs from newly created Previews."]
    pub fn tail_consumers(&self) -> terrars::SetRef<DataWorker_PreviewsBaseConfig_TailConsumersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tail_consumers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_References_DispatchNamespaceOutbounds {}

impl DataWorker_References_DispatchNamespaceOutbounds { }

impl terrars::ToListMappable for DataWorker_References_DispatchNamespaceOutbounds {
    type O = terrars::BlockAssignable<DataWorker_References_DispatchNamespaceOutbounds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_References_DispatchNamespaceOutbounds {}

impl BuildDataWorker_References_DispatchNamespaceOutbounds {
    pub fn build(self) -> DataWorker_References_DispatchNamespaceOutbounds {
        DataWorker_References_DispatchNamespaceOutbounds {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_References_DispatchNamespaceOutboundsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_References_DispatchNamespaceOutboundsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorker_References_DispatchNamespaceOutboundsRef {
        DataWorker_References_DispatchNamespaceOutboundsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_References_DispatchNamespaceOutboundsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\nID of the dispatch namespace."]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace_name` after provisioning.\nName of the dispatch namespace."]
    pub fn namespace_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_id` after provisioning.\nID of the Worker using the dispatch namespace."]
    pub fn worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_name` after provisioning.\nName of the Worker using the dispatch namespace."]
    pub fn worker_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_References_Domains {}

impl DataWorker_References_Domains { }

impl terrars::ToListMappable for DataWorker_References_Domains {
    type O = terrars::BlockAssignable<DataWorker_References_Domains>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_References_Domains {}

impl BuildDataWorker_References_Domains {
    pub fn build(self) -> DataWorker_References_Domains {
        DataWorker_References_Domains {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_References_DomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_References_DomainsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_References_DomainsRef {
        DataWorker_References_DomainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_References_DomainsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `certificate_id` after provisioning.\nID of the TLS certificate issued for the custom domain."]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nFull hostname of the custom domain, including the zone name."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the custom domain."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_name` after provisioning.\nName of the zone."]
    pub fn zone_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_References_DurableObjects {}

impl DataWorker_References_DurableObjects { }

impl terrars::ToListMappable for DataWorker_References_DurableObjects {
    type O = terrars::BlockAssignable<DataWorker_References_DurableObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_References_DurableObjects {}

impl BuildDataWorker_References_DurableObjects {
    pub fn build(self) -> DataWorker_References_DurableObjects {
        DataWorker_References_DurableObjects {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_References_DurableObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_References_DurableObjectsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_References_DurableObjectsRef {
        DataWorker_References_DurableObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_References_DurableObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `namespace_id` after provisioning.\nID of the Durable Object namespace being used."]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace_name` after provisioning.\nName of the Durable Object namespace being used."]
    pub fn namespace_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_id` after provisioning.\nID of the Worker using the Durable Object implementation."]
    pub fn worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_name` after provisioning.\nName of the Worker using the Durable Object implementation."]
    pub fn worker_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_References_Queues {}

impl DataWorker_References_Queues { }

impl terrars::ToListMappable for DataWorker_References_Queues {
    type O = terrars::BlockAssignable<DataWorker_References_Queues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_References_Queues {}

impl BuildDataWorker_References_Queues {
    pub fn build(self) -> DataWorker_References_Queues {
        DataWorker_References_Queues {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_References_QueuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_References_QueuesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_References_QueuesRef {
        DataWorker_References_QueuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_References_QueuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `queue_consumer_id` after provisioning.\nID of the queue consumer configuration."]
    pub fn queue_consumer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_consumer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\nID of the queue."]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\nName of the queue."]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_References_Workers {}

impl DataWorker_References_Workers { }

impl terrars::ToListMappable for DataWorker_References_Workers {
    type O = terrars::BlockAssignable<DataWorker_References_Workers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_References_Workers {}

impl BuildDataWorker_References_Workers {
    pub fn build(self) -> DataWorker_References_Workers {
        DataWorker_References_Workers {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_References_WorkersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_References_WorkersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_References_WorkersRef {
        DataWorker_References_WorkersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_References_WorkersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the referencing Worker."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the referencing Worker."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_References {}

impl DataWorker_References { }

impl terrars::ToListMappable for DataWorker_References {
    type O = terrars::BlockAssignable<DataWorker_References>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_References {}

impl BuildDataWorker_References {
    pub fn build(self) -> DataWorker_References {
        DataWorker_References {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_ReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_ReferencesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_ReferencesRef {
        DataWorker_ReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_ReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dispatch_namespace_outbounds` after provisioning.\nOther Workers that reference the Worker as an outbound for a dispatch namespace."]
    pub fn dispatch_namespace_outbounds(&self) -> terrars::ListRef<DataWorker_References_DispatchNamespaceOutboundsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dispatch_namespace_outbounds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nCustom domains connected to the Worker."]
    pub fn domains(&self) -> terrars::ListRef<DataWorker_References_DomainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `durable_objects` after provisioning.\nOther Workers that reference Durable Object classes implemented by the Worker."]
    pub fn durable_objects(&self) -> terrars::ListRef<DataWorker_References_DurableObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.durable_objects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `queues` after provisioning.\nQueues that send messages to the Worker."]
    pub fn queues(&self) -> terrars::ListRef<DataWorker_References_QueuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queues", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workers` after provisioning.\nOther Workers that reference the Worker using [service bindings](https://developers.cloudflare.com/workers/runtime-apis/bindings/service-bindings/)."]
    pub fn workers(&self) -> terrars::ListRef<DataWorker_References_WorkersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_Subdomain {}

impl DataWorker_Subdomain { }

impl terrars::ToListMappable for DataWorker_Subdomain {
    type O = terrars::BlockAssignable<DataWorker_Subdomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_Subdomain {}

impl BuildDataWorker_Subdomain {
    pub fn build(self) -> DataWorker_Subdomain {
        DataWorker_Subdomain {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_SubdomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_SubdomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_SubdomainRef {
        DataWorker_SubdomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_SubdomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the *.workers.dev subdomain is enabled for the Worker."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview_url_suffix` after provisioning.\nPrepend a version or preview prefix to this host suffix to form the *.workers.dev [preview URL](https://developers.cloudflare.com/workers/configuration/previews/) the Worker would serve on once previews are enabled, e.g. `https://<prefix>-my-worker.my-subdomain.workers.dev`. Present whenever the account owns a workers.dev subdomain, regardless of whether `previews_enabled` is true, so presence does not imply preview URLs are currently live. Absent only when the account owns no workers.dev subdomain."]
    pub fn preview_url_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview_url_suffix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `previews_enabled` after provisioning.\nWhether [preview URLs](https://developers.cloudflare.com/workers/configuration/previews/) are enabled for the Worker."]
    pub fn previews_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.previews_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe address the Worker would serve on once its *.workers.dev subdomain is enabled. Present whenever the account owns a workers.dev subdomain, regardless of whether `enabled` is true, so presence does not imply the Worker is currently live at this URL. Absent only when the account owns no workers.dev subdomain."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorker_TailConsumers {}

impl DataWorker_TailConsumers { }

impl terrars::ToListMappable for DataWorker_TailConsumers {
    type O = terrars::BlockAssignable<DataWorker_TailConsumers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorker_TailConsumers {}

impl BuildDataWorker_TailConsumers {
    pub fn build(self) -> DataWorker_TailConsumers {
        DataWorker_TailConsumers {}
    }
}

#[allow(dead_code)]
pub struct DataWorker_TailConsumersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorker_TailConsumersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorker_TailConsumersRef {
        DataWorker_TailConsumersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorker_TailConsumersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the consumer Worker."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
