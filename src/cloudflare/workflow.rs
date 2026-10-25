#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WorkflowData_ {
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
    class_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    concurrency: std::option::Option<Workflow_Concurrency>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_retention: std::option::Option<Workflow_DefaultRetention>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<Workflow_Limits>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedules: std::option::Option<std::vec::Vec<Workflow_Schedules>>,
    script_name: terrars::PrimField<std::string::String>,
    workflow_name: terrars::PrimField<std::string::String>,
}

struct Workflow_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkflowData_>,
}

#[derive(Clone)]
pub struct Workflow(std::rc::Rc<Workflow_>);

impl Workflow {
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

    #[doc = "Set the field `concurrency`.\n"]
    pub fn set_concurrency(self, v: impl std::convert::Into<Workflow_Concurrency>) -> Self {
        self.0.data.borrow_mut().concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_retention`.\nDefault retention applied to instances of this version when they do not set their own retention."]
    pub fn set_default_retention(self, v: impl std::convert::Into<Workflow_DefaultRetention>) -> Self {
        self.0.data.borrow_mut().default_retention = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(self, v: impl std::convert::Into<Workflow_Limits>) -> Self {
        self.0.data.borrow_mut().limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedules`.\n"]
    pub fn set_schedules(self, v: impl std::convert::Into<std::vec::Vec<Workflow_Schedules>>) -> Self {
        self.0.data.borrow_mut().schedules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `class_name` after provisioning.\n"]
    pub fn class_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `concurrency` after provisioning.\n"]
    pub fn concurrency(&self) -> Workflow_ConcurrencyRef {
        Workflow_ConcurrencyRef::new(self.shared().clone(), format!("{}.concurrency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_retention` after provisioning.\nDefault retention applied to instances of this version when they do not set their own retention."]
    pub fn default_retention(&self) -> Workflow_DefaultRetentionRef {
        Workflow_DefaultRetentionRef::new(self.shared().clone(), format!("{}.default_retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_deleted` after provisioning.\n"]
    pub fn is_deleted(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> Workflow_LimitsRef {
        Workflow_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedules` after provisioning.\n"]
    pub fn schedules(&self) -> terrars::ListRef<Workflow_SchedulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `script_deleted` after provisioning.\nWhether the bound Worker was deleted, leaving this Workflow inactive."]
    pub fn script_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_deleted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_name` after provisioning.\n"]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `terminator_running` after provisioning.\n"]
    pub fn terminator_running(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminator_running", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `triggered_on` after provisioning.\n"]
    pub fn triggered_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.triggered_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workflow_name` after provisioning.\n"]
    pub fn workflow_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow_name", self.extract_ref()))
    }
}

impl terrars::Referable for Workflow {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Workflow { }

impl terrars::ToListMappable for Workflow {
    type O = terrars::ListRef<WorkflowRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Workflow_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_workflow".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkflow {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub class_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub script_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub workflow_name: terrars::PrimField<std::string::String>,
}

impl BuildWorkflow {
    pub fn build(self, stack: &mut terrars::Stack) -> Workflow {
        let out = Workflow(std::rc::Rc::new(Workflow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkflowData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                class_name: self.class_name,
                concurrency: core::default::Default::default(),
                default_retention: core::default::Default::default(),
                limits: core::default::Default::default(),
                schedules: core::default::Default::default(),
                script_name: self.script_name,
                workflow_name: self.workflow_name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkflowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkflowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkflowRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `class_name` after provisioning.\n"]
    pub fn class_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `concurrency` after provisioning.\n"]
    pub fn concurrency(&self) -> Workflow_ConcurrencyRef {
        Workflow_ConcurrencyRef::new(self.shared().clone(), format!("{}.concurrency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_retention` after provisioning.\nDefault retention applied to instances of this version when they do not set their own retention."]
    pub fn default_retention(&self) -> Workflow_DefaultRetentionRef {
        Workflow_DefaultRetentionRef::new(self.shared().clone(), format!("{}.default_retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_deleted` after provisioning.\n"]
    pub fn is_deleted(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> Workflow_LimitsRef {
        Workflow_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\n"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedules` after provisioning.\n"]
    pub fn schedules(&self) -> terrars::ListRef<Workflow_SchedulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `script_deleted` after provisioning.\nWhether the bound Worker was deleted, leaving this Workflow inactive."]
    pub fn script_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_deleted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_name` after provisioning.\n"]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `terminator_running` after provisioning.\n"]
    pub fn terminator_running(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminator_running", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `triggered_on` after provisioning.\n"]
    pub fn triggered_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.triggered_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workflow_name` after provisioning.\n"]
    pub fn workflow_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Workflow_Concurrency {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
}

impl Workflow_Concurrency {
    #[doc =
        "Set the field `limit`.\nMaximum number of instances of this workflow that can run concurrently. Additional instances are queued and started as running instances complete. Must not exceed the account concurrency limit."]
    pub fn set_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Workflow_Concurrency {
    type O = terrars::BlockAssignable<Workflow_Concurrency>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkflow_Concurrency {}

impl BuildWorkflow_Concurrency {
    pub fn build(self) -> Workflow_Concurrency {
        Workflow_Concurrency { limit: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Workflow_ConcurrencyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Workflow_ConcurrencyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Workflow_ConcurrencyRef {
        Workflow_ConcurrencyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Workflow_ConcurrencyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nMaximum number of instances of this workflow that can run concurrently. Additional instances are queued and started as running instances complete. Must not exceed the account concurrency limit."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }
}

#[derive(Serialize)]
pub struct Workflow_DefaultRetention {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_retention: std::option::Option<terrars::PrimField<serde_json::Value>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_retention: std::option::Option<terrars::PrimField<serde_json::Value>>,
}

impl Workflow_DefaultRetention {
    #[doc =
        "Set the field `error_retention`.\nSpecifies the duration in milliseconds or as a string like '5 minutes'."]
    pub fn set_error_retention(mut self, v: impl std::convert::Into<terrars::PrimField<serde_json::Value>>) -> Self {
        self.error_retention = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `success_retention`.\nSpecifies the duration in milliseconds or as a string like '5 minutes'."]
    pub fn set_success_retention(mut self, v: impl std::convert::Into<terrars::PrimField<serde_json::Value>>) -> Self {
        self.success_retention = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Workflow_DefaultRetention {
    type O = terrars::BlockAssignable<Workflow_DefaultRetention>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkflow_DefaultRetention {}

impl BuildWorkflow_DefaultRetention {
    pub fn build(self) -> Workflow_DefaultRetention {
        Workflow_DefaultRetention {
            error_retention: core::default::Default::default(),
            success_retention: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Workflow_DefaultRetentionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Workflow_DefaultRetentionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Workflow_DefaultRetentionRef {
        Workflow_DefaultRetentionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Workflow_DefaultRetentionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `error_retention` after provisioning.\nSpecifies the duration in milliseconds or as a string like '5 minutes'."]
    pub fn error_retention(&self) -> terrars::PrimExpr<serde_json::Value> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_retention", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_retention` after provisioning.\nSpecifies the duration in milliseconds or as a string like '5 minutes'."]
    pub fn success_retention(&self) -> terrars::PrimExpr<serde_json::Value> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_retention", self.base))
    }
}

#[derive(Serialize)]
pub struct Workflow_Limits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std::option::Option<terrars::PrimField<f64>>,
}

impl Workflow_Limits {
    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.steps = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Workflow_Limits {
    type O = terrars::BlockAssignable<Workflow_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkflow_Limits {}

impl BuildWorkflow_Limits {
    pub fn build(self) -> Workflow_Limits {
        Workflow_Limits { steps: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Workflow_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Workflow_LimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Workflow_LimitsRef {
        Workflow_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Workflow_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}

#[derive(Serialize)]
pub struct Workflow_Schedules {
    cron: terrars::PrimField<std::string::String>,
}

impl Workflow_Schedules { }

impl terrars::ToListMappable for Workflow_Schedules {
    type O = terrars::BlockAssignable<Workflow_Schedules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkflow_Schedules {
    #[doc = ""]
    pub cron: terrars::PrimField<std::string::String>,
}

impl BuildWorkflow_Schedules {
    pub fn build(self) -> Workflow_Schedules {
        Workflow_Schedules { cron: self.cron }
    }
}

#[allow(dead_code)]
pub struct Workflow_SchedulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Workflow_SchedulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Workflow_SchedulesRef {
        Workflow_SchedulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Workflow_SchedulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cron` after provisioning.\n"]
    pub fn cron(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron", self.base))
    }
}
