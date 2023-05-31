use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WorkerCronTriggerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    schedules: SetField<PrimField<String>>,
    script_name: PrimField<String>,
}

struct WorkerCronTrigger_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkerCronTriggerData>,
}

#[derive(Clone)]
pub struct WorkerCronTrigger(Rc<WorkerCronTrigger_>);

impl WorkerCronTrigger {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schedules` after provisioning.\nCron expressions to execute the Worker script."]
    pub fn schedules(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.schedules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `script_name` after provisioning.\nWorker script to target for the schedules."]
    pub fn script_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }
}

impl Referable for WorkerCronTrigger {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WorkerCronTrigger { }

impl ToListMappable for WorkerCronTrigger {
    type O = ListRef<WorkerCronTriggerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WorkerCronTrigger_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_worker_cron_trigger".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkerCronTrigger {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "Cron expressions to execute the Worker script."]
    pub schedules: SetField<PrimField<String>>,
    #[doc= "Worker script to target for the schedules."]
    pub script_name: PrimField<String>,
}

impl BuildWorkerCronTrigger {
    pub fn build(self, stack: &mut Stack) -> WorkerCronTrigger {
        let out = WorkerCronTrigger(Rc::new(WorkerCronTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorkerCronTriggerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
                schedules: self.schedules,
                script_name: self.script_name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkerCronTriggerRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkerCronTriggerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkerCronTriggerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schedules` after provisioning.\nCron expressions to execute the Worker script."]
    pub fn schedules(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.schedules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `script_name` after provisioning.\nWorker script to target for the schedules."]
    pub fn script_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }
}
