#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WorkerData_ {
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
    force: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logpush: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    observability: std::option::Option<Worker_Observability>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    previews_base_config: std::option::Option<Worker_PreviewsBaseConfig>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subdomain: std::option::Option<Worker_Subdomain>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tail_consumers: std::option::Option<std::vec::Vec<Worker_TailConsumers>>,
}

struct Worker_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkerData_>,
}

#[derive(Clone)]
pub struct Worker(std::rc::Rc<Worker_>);

impl Worker {
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

    #[doc =
        "Set the field `force`.\nIf true, delete the Worker even when other Workers still reference it. Service bindings in those Workers may be left broken. Durable Object namespaces implemented by the deleted Worker are deleted even if other Workers reference them."]
    pub fn set_force(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logpush`.\nWhether logpush is enabled for the Worker."]
    pub fn set_logpush(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().logpush = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `observability`.\nObservability settings for the Worker."]
    pub fn set_observability(self, v: impl std::convert::Into<Worker_Observability>) -> Self {
        self.0.data.borrow_mut().observability = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `previews_base_config`.\nTemplate configuration used when creating new Previews for this Worker."]
    pub fn set_previews_base_config(self, v: impl std::convert::Into<Worker_PreviewsBaseConfig>) -> Self {
        self.0.data.borrow_mut().previews_base_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subdomain`.\nSubdomain settings for the Worker."]
    pub fn set_subdomain(self, v: impl std::convert::Into<Worker_Subdomain>) -> Self {
        self.0.data.borrow_mut().subdomain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nTags associated with the Worker."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tail_consumers`.\nOther Workers that should consume logs from the Worker."]
    pub fn set_tail_consumers(self, v: impl std::convert::Into<std::vec::Vec<Worker_TailConsumers>>) -> Self {
        self.0.data.borrow_mut().tail_consumers = std::option::Option::Some(v.into());
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

    #[doc =
        "Get a reference to the value of field `force` after provisioning.\nIf true, delete the Worker even when other Workers still reference it. Service bindings in those Workers may be left broken. Durable Object namespaces implemented by the deleted Worker are deleted even if other Workers reference them."]
    pub fn force(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nImmutable ID of the Worker."]
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
    pub fn observability(&self) -> Worker_ObservabilityRef {
        Worker_ObservabilityRef::new(self.shared().clone(), format!("{}.observability", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `previews_base_config` after provisioning.\nTemplate configuration used when creating new Previews for this Worker."]
    pub fn previews_base_config(&self) -> Worker_PreviewsBaseConfigRef {
        Worker_PreviewsBaseConfigRef::new(
            self.shared().clone(),
            format!("{}.previews_base_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `references` after provisioning.\nOther resources that reference the Worker and depend on it existing."]
    pub fn references(&self) -> Worker_ReferencesRef {
        Worker_ReferencesRef::new(self.shared().clone(), format!("{}.references", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\nSubdomain settings for the Worker."]
    pub fn subdomain(&self) -> Worker_SubdomainRef {
        Worker_SubdomainRef::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags associated with the Worker."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tail_consumers` after provisioning.\nOther Workers that should consume logs from the Worker."]
    pub fn tail_consumers(&self) -> terrars::SetRef<Worker_TailConsumersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tail_consumers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_on` after provisioning.\nWhen the Worker was most recently updated."]
    pub fn updated_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_on", self.extract_ref()))
    }
}

impl terrars::Referable for Worker {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Worker { }

impl terrars::ToListMappable for Worker {
    type O = terrars::ListRef<WorkerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Worker_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_worker".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorker {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Worker."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWorker {
    pub fn build(self, stack: &mut terrars::Stack) -> Worker {
        let out = Worker(std::rc::Rc::new(Worker_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                force: core::default::Default::default(),
                logpush: core::default::Default::default(),
                name: self.name,
                observability: core::default::Default::default(),
                previews_base_config: core::default::Default::default(),
                subdomain: core::default::Default::default(),
                tags: core::default::Default::default(),
                tail_consumers: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc =
        "Get a reference to the value of field `force` after provisioning.\nIf true, delete the Worker even when other Workers still reference it. Service bindings in those Workers may be left broken. Durable Object namespaces implemented by the deleted Worker are deleted even if other Workers reference them."]
    pub fn force(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nImmutable ID of the Worker."]
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
    pub fn observability(&self) -> Worker_ObservabilityRef {
        Worker_ObservabilityRef::new(self.shared().clone(), format!("{}.observability", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `previews_base_config` after provisioning.\nTemplate configuration used when creating new Previews for this Worker."]
    pub fn previews_base_config(&self) -> Worker_PreviewsBaseConfigRef {
        Worker_PreviewsBaseConfigRef::new(
            self.shared().clone(),
            format!("{}.previews_base_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `references` after provisioning.\nOther resources that reference the Worker and depend on it existing."]
    pub fn references(&self) -> Worker_ReferencesRef {
        Worker_ReferencesRef::new(self.shared().clone(), format!("{}.references", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\nSubdomain settings for the Worker."]
    pub fn subdomain(&self) -> Worker_SubdomainRef {
        Worker_SubdomainRef::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags associated with the Worker."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tail_consumers` after provisioning.\nOther Workers that should consume logs from the Worker."]
    pub fn tail_consumers(&self) -> terrars::SetRef<Worker_TailConsumersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tail_consumers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_on` after provisioning.\nWhen the Worker was most recently updated."]
    pub fn updated_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_on", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Worker_Observability_Issues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Worker_Observability_Issues {
    #[doc = "Set the field `enabled`.\nWhether real-time Issues are enabled for the Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_Observability_Issues {
    type O = terrars::BlockAssignable<Worker_Observability_Issues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_Observability_Issues {}

impl BuildWorker_Observability_Issues {
    pub fn build(self) -> Worker_Observability_Issues {
        Worker_Observability_Issues { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Worker_Observability_IssuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_Observability_IssuesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_Observability_IssuesRef {
        Worker_Observability_IssuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_Observability_IssuesRef {
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
pub struct Worker_Observability_Logs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    head_sampling_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_logs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persist: std::option::Option<terrars::PrimField<bool>>,
}

impl Worker_Observability_Logs {
    #[doc = "Set the field `destinations`.\nA list of destinations where logs will be exported to."]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destinations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether logs are enabled for the Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `head_sampling_rate`.\nThe sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn set_head_sampling_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.head_sampling_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invocation_logs`.\nWhether [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs) are enabled for the Worker."]
    pub fn set_invocation_logs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invocation_logs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `persist`.\nWhether log persistence is enabled for the Worker."]
    pub fn set_persist(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.persist = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_Observability_Logs {
    type O = terrars::BlockAssignable<Worker_Observability_Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_Observability_Logs {}

impl BuildWorker_Observability_Logs {
    pub fn build(self) -> Worker_Observability_Logs {
        Worker_Observability_Logs {
            destinations: core::default::Default::default(),
            enabled: core::default::Default::default(),
            head_sampling_rate: core::default::Default::default(),
            invocation_logs: core::default::Default::default(),
            persist: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_Observability_LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_Observability_LogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_Observability_LogsRef {
        Worker_Observability_LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_Observability_LogsRef {
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
pub struct Worker_Observability_Traces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    head_sampling_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persist: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagation_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Worker_Observability_Traces {
    #[doc = "Set the field `destinations`.\nA list of destinations where traces will be exported to."]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destinations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether traces are enabled for the Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `head_sampling_rate`.\nThe sampling rate for traces. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn set_head_sampling_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.head_sampling_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `persist`.\nWhether trace persistence is enabled for the Worker."]
    pub fn set_persist(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.persist = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `propagation_policy`.\nControls how inbound trace context (traceparent/tracestate) headers on incoming requests are handled. \"authenticated\" (default) honors inbound trace context only when accompanied by a valid trace auth token. \"accept\" unconditionally accepts inbound trace context. Requires the trace propagation feature to be enabled.\nAvailable values: \"authenticated\", \"accept\"."]
    pub fn set_propagation_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.propagation_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_Observability_Traces {
    type O = terrars::BlockAssignable<Worker_Observability_Traces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_Observability_Traces {}

impl BuildWorker_Observability_Traces {
    pub fn build(self) -> Worker_Observability_Traces {
        Worker_Observability_Traces {
            destinations: core::default::Default::default(),
            enabled: core::default::Default::default(),
            head_sampling_rate: core::default::Default::default(),
            persist: core::default::Default::default(),
            propagation_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_Observability_TracesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_Observability_TracesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_Observability_TracesRef {
        Worker_Observability_TracesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_Observability_TracesRef {
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
        "Get a reference to the value of field `propagation_policy` after provisioning.\nControls how inbound trace context (traceparent/tracestate) headers on incoming requests are handled. \"authenticated\" (default) honors inbound trace context only when accompanied by a valid trace auth token. \"accept\" unconditionally accepts inbound trace context. Requires the trace propagation feature to be enabled.\nAvailable values: \"authenticated\", \"accept\"."]
    pub fn propagation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagation_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct Worker_Observability {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    head_sampling_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issues: std::option::Option<Worker_Observability_Issues>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs: std::option::Option<Worker_Observability_Logs>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traces: std::option::Option<Worker_Observability_Traces>,
}

impl Worker_Observability {
    #[doc = "Set the field `enabled`.\nWhether observability is enabled for the Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `head_sampling_rate`.\nThe sampling rate for observability. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn set_head_sampling_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.head_sampling_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issues`.\nReal-time Issues settings for the Worker."]
    pub fn set_issues(mut self, v: impl std::convert::Into<Worker_Observability_Issues>) -> Self {
        self.issues = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logs`.\nLog settings for the Worker."]
    pub fn set_logs(mut self, v: impl std::convert::Into<Worker_Observability_Logs>) -> Self {
        self.logs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traces`.\nTrace settings for the Worker."]
    pub fn set_traces(mut self, v: impl std::convert::Into<Worker_Observability_Traces>) -> Self {
        self.traces = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_Observability {
    type O = terrars::BlockAssignable<Worker_Observability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_Observability {}

impl BuildWorker_Observability {
    pub fn build(self) -> Worker_Observability {
        Worker_Observability {
            enabled: core::default::Default::default(),
            head_sampling_rate: core::default::Default::default(),
            issues: core::default::Default::default(),
            logs: core::default::Default::default(),
            traces: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_ObservabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_ObservabilityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_ObservabilityRef {
        Worker_ObservabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_ObservabilityRef {
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
    pub fn issues(&self) -> Worker_Observability_IssuesRef {
        Worker_Observability_IssuesRef::new(self.shared().clone(), format!("{}.issues", self.base))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\nLog settings for the Worker."]
    pub fn logs(&self) -> Worker_Observability_LogsRef {
        Worker_Observability_LogsRef::new(self.shared().clone(), format!("{}.logs", self.base))
    }

    #[doc = "Get a reference to the value of field `traces` after provisioning.\nTrace settings for the Worker."]
    pub fn traces(&self) -> Worker_Observability_TracesRef {
        Worker_Observability_TracesRef::new(self.shared().clone(), format!("{}.traces", self.base))
    }
}

#[derive(Serialize)]
pub struct Worker_PreviewsBaseConfig_CacheOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_version_cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Worker_PreviewsBaseConfig_CacheOptions {
    #[doc =
        "Set the field `cross_version_cache`.\nWhether cached responses are shared across Worker version\nuploads. This is independent of `enabled`. It can stay true\nwhile caching is off, so the preference survives turning\ncaching off and back on."]
    pub fn set_cross_version_cache(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cross_version_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether caching is enabled for this Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_PreviewsBaseConfig_CacheOptions {
    type O = terrars::BlockAssignable<Worker_PreviewsBaseConfig_CacheOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_PreviewsBaseConfig_CacheOptions {}

impl BuildWorker_PreviewsBaseConfig_CacheOptions {
    pub fn build(self) -> Worker_PreviewsBaseConfig_CacheOptions {
        Worker_PreviewsBaseConfig_CacheOptions {
            cross_version_cache: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_PreviewsBaseConfig_CacheOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_PreviewsBaseConfig_CacheOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_PreviewsBaseConfig_CacheOptionsRef {
        Worker_PreviewsBaseConfig_CacheOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_PreviewsBaseConfig_CacheOptionsRef {
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
pub struct Worker_PreviewsBaseConfig_Env {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl Worker_PreviewsBaseConfig_Env { }

impl terrars::ToListMappable for Worker_PreviewsBaseConfig_Env {
    type O = terrars::BlockAssignable<Worker_PreviewsBaseConfig_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_PreviewsBaseConfig_Env {
    #[doc = "The kind of resource that the binding provides."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWorker_PreviewsBaseConfig_Env {
    pub fn build(self) -> Worker_PreviewsBaseConfig_Env {
        Worker_PreviewsBaseConfig_Env { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct Worker_PreviewsBaseConfig_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_PreviewsBaseConfig_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_PreviewsBaseConfig_EnvRef {
        Worker_PreviewsBaseConfig_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_PreviewsBaseConfig_EnvRef {
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
pub struct Worker_PreviewsBaseConfig_Limits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subrequests: std::option::Option<terrars::PrimField<f64>>,
}

impl Worker_PreviewsBaseConfig_Limits {
    #[doc = "Set the field `cpu_ms`.\nThe amount of CPU time this Worker can use in milliseconds."]
    pub fn set_cpu_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subrequests`.\nThe number of subrequests this Worker can make per request."]
    pub fn set_subrequests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.subrequests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_PreviewsBaseConfig_Limits {
    type O = terrars::BlockAssignable<Worker_PreviewsBaseConfig_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_PreviewsBaseConfig_Limits {}

impl BuildWorker_PreviewsBaseConfig_Limits {
    pub fn build(self) -> Worker_PreviewsBaseConfig_Limits {
        Worker_PreviewsBaseConfig_Limits {
            cpu_ms: core::default::Default::default(),
            subrequests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_PreviewsBaseConfig_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_PreviewsBaseConfig_LimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_PreviewsBaseConfig_LimitsRef {
        Worker_PreviewsBaseConfig_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_PreviewsBaseConfig_LimitsRef {
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
pub struct Worker_PreviewsBaseConfig_Observability_Traces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    head_sampling_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persist: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagation_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Worker_PreviewsBaseConfig_Observability_Traces {
    #[doc = "Set the field `destinations`.\nA list of destinations where traces will be exported to."]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destinations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether traces are enabled for the Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `head_sampling_rate`.\nThe sampling rate for traces. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn set_head_sampling_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.head_sampling_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `persist`.\nWhether trace persistence is enabled for the Worker."]
    pub fn set_persist(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.persist = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `propagation_policy`.\nControls how inbound trace context (traceparent/tracestate) headers on incoming requests are handled. \"authenticated\" honors inbound trace context only when accompanied by a valid trace auth token. \"accept\" unconditionally accepts inbound trace context. Requires the trace propagation feature to be enabled. Returns null when the trace propagation feature is not enabled for the account.\nAvailable values: \"authenticated\", \"accept\"."]
    pub fn set_propagation_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.propagation_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_PreviewsBaseConfig_Observability_Traces {
    type O = terrars::BlockAssignable<Worker_PreviewsBaseConfig_Observability_Traces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_PreviewsBaseConfig_Observability_Traces {}

impl BuildWorker_PreviewsBaseConfig_Observability_Traces {
    pub fn build(self) -> Worker_PreviewsBaseConfig_Observability_Traces {
        Worker_PreviewsBaseConfig_Observability_Traces {
            destinations: core::default::Default::default(),
            enabled: core::default::Default::default(),
            head_sampling_rate: core::default::Default::default(),
            persist: core::default::Default::default(),
            propagation_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_PreviewsBaseConfig_Observability_TracesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_PreviewsBaseConfig_Observability_TracesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Worker_PreviewsBaseConfig_Observability_TracesRef {
        Worker_PreviewsBaseConfig_Observability_TracesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_PreviewsBaseConfig_Observability_TracesRef {
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
pub struct Worker_PreviewsBaseConfig_Observability {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    head_sampling_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issues: std::option::Option<Worker_Observability_Issues>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs: std::option::Option<Worker_Observability_Logs>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redact_query_string: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traces: std::option::Option<Worker_PreviewsBaseConfig_Observability_Traces>,
}

impl Worker_PreviewsBaseConfig_Observability {
    #[doc = "Set the field `enabled`.\nWhether observability is enabled for the Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `head_sampling_rate`.\nThe sampling rate for observability. From 0 to 1 (1 = 100%, 0.1 = 10%)."]
    pub fn set_head_sampling_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.head_sampling_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issues`.\nReal-time Issues settings for the Worker."]
    pub fn set_issues(mut self, v: impl std::convert::Into<Worker_Observability_Issues>) -> Self {
        self.issues = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logs`.\nLog settings for the Worker."]
    pub fn set_logs(mut self, v: impl std::convert::Into<Worker_Observability_Logs>) -> Self {
        self.logs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redact_query_string`.\nWhether query strings are removed from request URLs in logs and traces."]
    pub fn set_redact_query_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.redact_query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traces`.\nTrace settings for the Worker."]
    pub fn set_traces(mut self, v: impl std::convert::Into<Worker_PreviewsBaseConfig_Observability_Traces>) -> Self {
        self.traces = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_PreviewsBaseConfig_Observability {
    type O = terrars::BlockAssignable<Worker_PreviewsBaseConfig_Observability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_PreviewsBaseConfig_Observability {}

impl BuildWorker_PreviewsBaseConfig_Observability {
    pub fn build(self) -> Worker_PreviewsBaseConfig_Observability {
        Worker_PreviewsBaseConfig_Observability {
            enabled: core::default::Default::default(),
            head_sampling_rate: core::default::Default::default(),
            issues: core::default::Default::default(),
            logs: core::default::Default::default(),
            redact_query_string: core::default::Default::default(),
            traces: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_PreviewsBaseConfig_ObservabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_PreviewsBaseConfig_ObservabilityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_PreviewsBaseConfig_ObservabilityRef {
        Worker_PreviewsBaseConfig_ObservabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_PreviewsBaseConfig_ObservabilityRef {
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
    pub fn issues(&self) -> Worker_Observability_IssuesRef {
        Worker_Observability_IssuesRef::new(self.shared().clone(), format!("{}.issues", self.base))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\nLog settings for the Worker."]
    pub fn logs(&self) -> Worker_Observability_LogsRef {
        Worker_Observability_LogsRef::new(self.shared().clone(), format!("{}.logs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redact_query_string` after provisioning.\nWhether query strings are removed from request URLs in logs and traces."]
    pub fn redact_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redact_query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `traces` after provisioning.\nTrace settings for the Worker."]
    pub fn traces(&self) -> Worker_PreviewsBaseConfig_Observability_TracesRef {
        Worker_PreviewsBaseConfig_Observability_TracesRef::new(
            self.shared().clone(),
            format!("{}.traces", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Worker_PreviewsBaseConfig_Placement_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Worker_PreviewsBaseConfig_Placement_Target {
    #[doc = "Set the field `host`.\nTCP host:port for targeted placement."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nHTTP hostname for targeted placement."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nCloud region in format 'provider:region'."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_PreviewsBaseConfig_Placement_Target {
    type O = terrars::BlockAssignable<Worker_PreviewsBaseConfig_Placement_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_PreviewsBaseConfig_Placement_Target {}

impl BuildWorker_PreviewsBaseConfig_Placement_Target {
    pub fn build(self) -> Worker_PreviewsBaseConfig_Placement_Target {
        Worker_PreviewsBaseConfig_Placement_Target {
            host: core::default::Default::default(),
            hostname: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_PreviewsBaseConfig_Placement_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_PreviewsBaseConfig_Placement_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_PreviewsBaseConfig_Placement_TargetRef {
        Worker_PreviewsBaseConfig_Placement_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_PreviewsBaseConfig_Placement_TargetRef {
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
pub struct Worker_PreviewsBaseConfig_Placement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<Worker_PreviewsBaseConfig_Placement_Target>>,
}

impl Worker_PreviewsBaseConfig_Placement {
    #[doc = "Set the field `host`.\nTCP host and port for targeted placement."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nHTTP hostname for targeted placement."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nEnables [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).\nAvailable values: \"smart\", \"targeted\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nCloud region for targeted placement in format 'provider:region'."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\nArray of placement targets (currently limited to single target)."]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<Worker_PreviewsBaseConfig_Placement_Target>>,
    ) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_PreviewsBaseConfig_Placement {
    type O = terrars::BlockAssignable<Worker_PreviewsBaseConfig_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_PreviewsBaseConfig_Placement {}

impl BuildWorker_PreviewsBaseConfig_Placement {
    pub fn build(self) -> Worker_PreviewsBaseConfig_Placement {
        Worker_PreviewsBaseConfig_Placement {
            host: core::default::Default::default(),
            hostname: core::default::Default::default(),
            mode: core::default::Default::default(),
            region: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_PreviewsBaseConfig_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_PreviewsBaseConfig_PlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_PreviewsBaseConfig_PlacementRef {
        Worker_PreviewsBaseConfig_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_PreviewsBaseConfig_PlacementRef {
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
    pub fn target(&self) -> terrars::ListRef<Worker_PreviewsBaseConfig_Placement_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct Worker_TailConsumers {
    name: terrars::PrimField<std::string::String>,
}

impl Worker_TailConsumers { }

impl terrars::ToListMappable for Worker_TailConsumers {
    type O = terrars::BlockAssignable<Worker_TailConsumers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_TailConsumers {
    #[doc = "Name of the consumer Worker."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWorker_TailConsumers {
    pub fn build(self) -> Worker_TailConsumers {
        Worker_TailConsumers { name: self.name }
    }
}

#[allow(dead_code)]
pub struct Worker_TailConsumersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_TailConsumersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_TailConsumersRef {
        Worker_TailConsumersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_TailConsumersRef {
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
pub struct Worker_PreviewsBaseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_options: std::option::Option<Worker_PreviewsBaseConfig_CacheOptions>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::collections::HashMap<std::string::String, Worker_PreviewsBaseConfig_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<Worker_PreviewsBaseConfig_Limits>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logpush: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    observability: std::option::Option<Worker_PreviewsBaseConfig_Observability>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<Worker_PreviewsBaseConfig_Placement>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tail_consumers: std::option::Option<std::vec::Vec<Worker_TailConsumers>>,
}

impl Worker_PreviewsBaseConfig {
    #[doc = "Set the field `cache_options`.\nCache options used when creating new Previews."]
    pub fn set_cache_options(mut self, v: impl std::convert::Into<Worker_PreviewsBaseConfig_CacheOptions>) -> Self {
        self.cache_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\nBindings used when creating new Previews, keyed by binding name."]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<std::collections::HashMap<std::string::String, Worker_PreviewsBaseConfig_Env>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limits`.\nResource limits enforced at runtime for newly created Previews."]
    pub fn set_limits(mut self, v: impl std::convert::Into<Worker_PreviewsBaseConfig_Limits>) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logpush`.\nWhether logpush is enabled when creating new Previews."]
    pub fn set_logpush(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.logpush = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `observability`.\nObservability settings used when creating new Previews."]
    pub fn set_observability(mut self, v: impl std::convert::Into<Worker_PreviewsBaseConfig_Observability>) -> Self {
        self.observability = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement`.\nPlacement configuration used when creating new Previews."]
    pub fn set_placement(mut self, v: impl std::convert::Into<Worker_PreviewsBaseConfig_Placement>) -> Self {
        self.placement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tail_consumers`.\nOther Workers that should consume logs from newly created Previews."]
    pub fn set_tail_consumers(mut self, v: impl std::convert::Into<std::vec::Vec<Worker_TailConsumers>>) -> Self {
        self.tail_consumers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_PreviewsBaseConfig {
    type O = terrars::BlockAssignable<Worker_PreviewsBaseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_PreviewsBaseConfig {}

impl BuildWorker_PreviewsBaseConfig {
    pub fn build(self) -> Worker_PreviewsBaseConfig {
        Worker_PreviewsBaseConfig {
            cache_options: core::default::Default::default(),
            env: core::default::Default::default(),
            limits: core::default::Default::default(),
            logpush: core::default::Default::default(),
            observability: core::default::Default::default(),
            placement: core::default::Default::default(),
            tail_consumers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_PreviewsBaseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_PreviewsBaseConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_PreviewsBaseConfigRef {
        Worker_PreviewsBaseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_PreviewsBaseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache_options` after provisioning.\nCache options used when creating new Previews."]
    pub fn cache_options(&self) -> Worker_PreviewsBaseConfig_CacheOptionsRef {
        Worker_PreviewsBaseConfig_CacheOptionsRef::new(self.shared().clone(), format!("{}.cache_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nBindings used when creating new Previews, keyed by binding name."]
    pub fn env(&self) -> terrars::RecRef<Worker_PreviewsBaseConfig_EnvRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `limits` after provisioning.\nResource limits enforced at runtime for newly created Previews."]
    pub fn limits(&self) -> Worker_PreviewsBaseConfig_LimitsRef {
        Worker_PreviewsBaseConfig_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logpush` after provisioning.\nWhether logpush is enabled when creating new Previews."]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.base))
    }

    #[doc =
        "Get a reference to the value of field `observability` after provisioning.\nObservability settings used when creating new Previews."]
    pub fn observability(&self) -> Worker_PreviewsBaseConfig_ObservabilityRef {
        Worker_PreviewsBaseConfig_ObservabilityRef::new(
            self.shared().clone(),
            format!("{}.observability", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nPlacement configuration used when creating new Previews."]
    pub fn placement(&self) -> Worker_PreviewsBaseConfig_PlacementRef {
        Worker_PreviewsBaseConfig_PlacementRef::new(self.shared().clone(), format!("{}.placement", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tail_consumers` after provisioning.\nOther Workers that should consume logs from newly created Previews."]
    pub fn tail_consumers(&self) -> terrars::SetRef<Worker_TailConsumersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tail_consumers", self.base))
    }
}

#[derive(Serialize)]
pub struct Worker_References_DispatchNamespaceOutbounds {}

impl Worker_References_DispatchNamespaceOutbounds { }

impl terrars::ToListMappable for Worker_References_DispatchNamespaceOutbounds {
    type O = terrars::BlockAssignable<Worker_References_DispatchNamespaceOutbounds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_References_DispatchNamespaceOutbounds {}

impl BuildWorker_References_DispatchNamespaceOutbounds {
    pub fn build(self) -> Worker_References_DispatchNamespaceOutbounds {
        Worker_References_DispatchNamespaceOutbounds {}
    }
}

#[allow(dead_code)]
pub struct Worker_References_DispatchNamespaceOutboundsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_References_DispatchNamespaceOutboundsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Worker_References_DispatchNamespaceOutboundsRef {
        Worker_References_DispatchNamespaceOutboundsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_References_DispatchNamespaceOutboundsRef {
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
pub struct Worker_References_Domains {}

impl Worker_References_Domains { }

impl terrars::ToListMappable for Worker_References_Domains {
    type O = terrars::BlockAssignable<Worker_References_Domains>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_References_Domains {}

impl BuildWorker_References_Domains {
    pub fn build(self) -> Worker_References_Domains {
        Worker_References_Domains {}
    }
}

#[allow(dead_code)]
pub struct Worker_References_DomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_References_DomainsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_References_DomainsRef {
        Worker_References_DomainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_References_DomainsRef {
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
pub struct Worker_References_DurableObjects {}

impl Worker_References_DurableObjects { }

impl terrars::ToListMappable for Worker_References_DurableObjects {
    type O = terrars::BlockAssignable<Worker_References_DurableObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_References_DurableObjects {}

impl BuildWorker_References_DurableObjects {
    pub fn build(self) -> Worker_References_DurableObjects {
        Worker_References_DurableObjects {}
    }
}

#[allow(dead_code)]
pub struct Worker_References_DurableObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_References_DurableObjectsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_References_DurableObjectsRef {
        Worker_References_DurableObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_References_DurableObjectsRef {
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
pub struct Worker_References_Queues {}

impl Worker_References_Queues { }

impl terrars::ToListMappable for Worker_References_Queues {
    type O = terrars::BlockAssignable<Worker_References_Queues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_References_Queues {}

impl BuildWorker_References_Queues {
    pub fn build(self) -> Worker_References_Queues {
        Worker_References_Queues {}
    }
}

#[allow(dead_code)]
pub struct Worker_References_QueuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_References_QueuesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_References_QueuesRef {
        Worker_References_QueuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_References_QueuesRef {
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
pub struct Worker_References_Workers {}

impl Worker_References_Workers { }

impl terrars::ToListMappable for Worker_References_Workers {
    type O = terrars::BlockAssignable<Worker_References_Workers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_References_Workers {}

impl BuildWorker_References_Workers {
    pub fn build(self) -> Worker_References_Workers {
        Worker_References_Workers {}
    }
}

#[allow(dead_code)]
pub struct Worker_References_WorkersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_References_WorkersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_References_WorkersRef {
        Worker_References_WorkersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_References_WorkersRef {
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
pub struct Worker_References {}

impl Worker_References { }

impl terrars::ToListMappable for Worker_References {
    type O = terrars::BlockAssignable<Worker_References>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_References {}

impl BuildWorker_References {
    pub fn build(self) -> Worker_References {
        Worker_References {}
    }
}

#[allow(dead_code)]
pub struct Worker_ReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_ReferencesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_ReferencesRef {
        Worker_ReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_ReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dispatch_namespace_outbounds` after provisioning.\nOther Workers that reference the Worker as an outbound for a dispatch namespace."]
    pub fn dispatch_namespace_outbounds(&self) -> terrars::ListRef<Worker_References_DispatchNamespaceOutboundsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dispatch_namespace_outbounds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nCustom domains connected to the Worker."]
    pub fn domains(&self) -> terrars::ListRef<Worker_References_DomainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `durable_objects` after provisioning.\nOther Workers that reference Durable Object classes implemented by the Worker."]
    pub fn durable_objects(&self) -> terrars::ListRef<Worker_References_DurableObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.durable_objects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `queues` after provisioning.\nQueues that send messages to the Worker."]
    pub fn queues(&self) -> terrars::ListRef<Worker_References_QueuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queues", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workers` after provisioning.\nOther Workers that reference the Worker using [service bindings](https://developers.cloudflare.com/workers/runtime-apis/bindings/service-bindings/)."]
    pub fn workers(&self) -> terrars::ListRef<Worker_References_WorkersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workers", self.base))
    }
}

#[derive(Serialize)]
pub struct Worker_Subdomain {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    previews_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Worker_Subdomain {
    #[doc = "Set the field `enabled`.\nWhether the *.workers.dev subdomain is enabled for the Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `previews_enabled`.\nWhether [preview URLs](https://developers.cloudflare.com/workers/configuration/previews/) are enabled for the Worker."]
    pub fn set_previews_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.previews_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Worker_Subdomain {
    type O = terrars::BlockAssignable<Worker_Subdomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorker_Subdomain {}

impl BuildWorker_Subdomain {
    pub fn build(self) -> Worker_Subdomain {
        Worker_Subdomain {
            enabled: core::default::Default::default(),
            previews_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Worker_SubdomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Worker_SubdomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Worker_SubdomainRef {
        Worker_SubdomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Worker_SubdomainRef {
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
