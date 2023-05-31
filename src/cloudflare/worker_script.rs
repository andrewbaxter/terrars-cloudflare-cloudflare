use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WorkerScriptData {
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
    compatibility_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compatibility_flags: Option<SetField<PrimField<String>>>,
    content: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logpush: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    module: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_engine_binding: Option<Vec<WorkerScriptAnalyticsEngineBindingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kv_namespace_binding: Option<Vec<WorkerScriptKvNamespaceBindingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plain_text_binding: Option<Vec<WorkerScriptPlainTextBindingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_binding: Option<Vec<WorkerScriptQueueBindingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r2_bucket_binding: Option<Vec<WorkerScriptR2BucketBindingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_text_binding: Option<Vec<WorkerScriptSecretTextBindingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_binding: Option<Vec<WorkerScriptServiceBindingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webassembly_binding: Option<Vec<WorkerScriptWebassemblyBindingEl>>,
    dynamic: WorkerScriptDynamic,
}

struct WorkerScript_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkerScriptData>,
}

#[derive(Clone)]
pub struct WorkerScript(Rc<WorkerScript_>);

impl WorkerScript {
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

    #[doc= "Set the field `compatibility_date`.\nThe date to use for the compatibility flag."]
    pub fn set_compatibility_date(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().compatibility_date = Some(v.into());
        self
    }

    #[doc= "Set the field `compatibility_flags`.\nCompatibility flags used for Worker Scripts."]
    pub fn set_compatibility_flags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().compatibility_flags = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `logpush`.\nEnabling allows Worker events to be sent to a defined Logpush destination."]
    pub fn set_logpush(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().logpush = Some(v.into());
        self
    }

    #[doc= "Set the field `module`.\nWhether to upload Worker as a module."]
    pub fn set_module(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().module = Some(v.into());
        self
    }

    #[doc= "Set the field `analytics_engine_binding`.\n"]
    pub fn set_analytics_engine_binding(
        self,
        v: impl Into<BlockAssignable<WorkerScriptAnalyticsEngineBindingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().analytics_engine_binding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.analytics_engine_binding = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `kv_namespace_binding`.\n"]
    pub fn set_kv_namespace_binding(self, v: impl Into<BlockAssignable<WorkerScriptKvNamespaceBindingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kv_namespace_binding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kv_namespace_binding = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `plain_text_binding`.\n"]
    pub fn set_plain_text_binding(self, v: impl Into<BlockAssignable<WorkerScriptPlainTextBindingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().plain_text_binding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.plain_text_binding = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `queue_binding`.\n"]
    pub fn set_queue_binding(self, v: impl Into<BlockAssignable<WorkerScriptQueueBindingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().queue_binding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.queue_binding = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `r2_bucket_binding`.\n"]
    pub fn set_r2_bucket_binding(self, v: impl Into<BlockAssignable<WorkerScriptR2BucketBindingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().r2_bucket_binding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.r2_bucket_binding = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `secret_text_binding`.\n"]
    pub fn set_secret_text_binding(self, v: impl Into<BlockAssignable<WorkerScriptSecretTextBindingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secret_text_binding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secret_text_binding = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `service_binding`.\n"]
    pub fn set_service_binding(self, v: impl Into<BlockAssignable<WorkerScriptServiceBindingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_binding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_binding = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `webassembly_binding`.\n"]
    pub fn set_webassembly_binding(self, v: impl Into<BlockAssignable<WorkerScriptWebassemblyBindingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().webassembly_binding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.webassembly_binding = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `compatibility_date` after provisioning.\nThe date to use for the compatibility flag."]
    pub fn compatibility_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `compatibility_flags` after provisioning.\nCompatibility flags used for Worker Scripts."]
    pub fn compatibility_flags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nThe script content."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logpush` after provisioning.\nEnabling allows Worker events to be sent to a defined Logpush destination."]
    pub fn logpush(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `module` after provisioning.\nWhether to upload Worker as a module."]
    pub fn module(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.module", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name for the script. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl Referable for WorkerScript {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WorkerScript { }

impl ToListMappable for WorkerScript {
    type O = ListRef<WorkerScriptRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WorkerScript_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_worker_script".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkerScript {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "The script content."]
    pub content: PrimField<String>,
    #[doc= "The name for the script. **Modifying this attribute will force creation of a new resource.**"]
    pub name: PrimField<String>,
}

impl BuildWorkerScript {
    pub fn build(self, stack: &mut Stack) -> WorkerScript {
        let out = WorkerScript(Rc::new(WorkerScript_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorkerScriptData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                compatibility_date: core::default::Default::default(),
                compatibility_flags: core::default::Default::default(),
                content: self.content,
                id: core::default::Default::default(),
                logpush: core::default::Default::default(),
                module: core::default::Default::default(),
                name: self.name,
                analytics_engine_binding: core::default::Default::default(),
                kv_namespace_binding: core::default::Default::default(),
                plain_text_binding: core::default::Default::default(),
                queue_binding: core::default::Default::default(),
                r2_bucket_binding: core::default::Default::default(),
                secret_text_binding: core::default::Default::default(),
                service_binding: core::default::Default::default(),
                webassembly_binding: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkerScriptRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkerScriptRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkerScriptRef {
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

    #[doc= "Get a reference to the value of field `compatibility_date` after provisioning.\nThe date to use for the compatibility flag."]
    pub fn compatibility_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `compatibility_flags` after provisioning.\nCompatibility flags used for Worker Scripts."]
    pub fn compatibility_flags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nThe script content."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logpush` after provisioning.\nEnabling allows Worker events to be sent to a defined Logpush destination."]
    pub fn logpush(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `module` after provisioning.\nWhether to upload Worker as a module."]
    pub fn module(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.module", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name for the script. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptAnalyticsEngineBindingEl {
    dataset: PrimField<String>,
    name: PrimField<String>,
}

impl WorkerScriptAnalyticsEngineBindingEl { }

impl ToListMappable for WorkerScriptAnalyticsEngineBindingEl {
    type O = BlockAssignable<WorkerScriptAnalyticsEngineBindingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptAnalyticsEngineBindingEl {
    #[doc= "The name of the Analytics Engine dataset to write to."]
    pub dataset: PrimField<String>,
    #[doc= "The global variable for the binding in your Worker code."]
    pub name: PrimField<String>,
}

impl BuildWorkerScriptAnalyticsEngineBindingEl {
    pub fn build(self) -> WorkerScriptAnalyticsEngineBindingEl {
        WorkerScriptAnalyticsEngineBindingEl {
            dataset: self.dataset,
            name: self.name,
        }
    }
}

pub struct WorkerScriptAnalyticsEngineBindingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkerScriptAnalyticsEngineBindingElRef {
    fn new(shared: StackShared, base: String) -> WorkerScriptAnalyticsEngineBindingElRef {
        WorkerScriptAnalyticsEngineBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptAnalyticsEngineBindingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dataset` after provisioning.\nThe name of the Analytics Engine dataset to write to."]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptKvNamespaceBindingEl {
    name: PrimField<String>,
    namespace_id: PrimField<String>,
}

impl WorkerScriptKvNamespaceBindingEl { }

impl ToListMappable for WorkerScriptKvNamespaceBindingEl {
    type O = BlockAssignable<WorkerScriptKvNamespaceBindingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptKvNamespaceBindingEl {
    #[doc= "The global variable for the binding in your Worker code."]
    pub name: PrimField<String>,
    #[doc= "ID of the KV namespace you want to use."]
    pub namespace_id: PrimField<String>,
}

impl BuildWorkerScriptKvNamespaceBindingEl {
    pub fn build(self) -> WorkerScriptKvNamespaceBindingEl {
        WorkerScriptKvNamespaceBindingEl {
            name: self.name,
            namespace_id: self.namespace_id,
        }
    }
}

pub struct WorkerScriptKvNamespaceBindingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkerScriptKvNamespaceBindingElRef {
    fn new(shared: StackShared, base: String) -> WorkerScriptKvNamespaceBindingElRef {
        WorkerScriptKvNamespaceBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptKvNamespaceBindingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nID of the KV namespace you want to use."]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptPlainTextBindingEl {
    name: PrimField<String>,
    text: PrimField<String>,
}

impl WorkerScriptPlainTextBindingEl { }

impl ToListMappable for WorkerScriptPlainTextBindingEl {
    type O = BlockAssignable<WorkerScriptPlainTextBindingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptPlainTextBindingEl {
    #[doc= "The global variable for the binding in your Worker code."]
    pub name: PrimField<String>,
    #[doc= "The plain text you want to store."]
    pub text: PrimField<String>,
}

impl BuildWorkerScriptPlainTextBindingEl {
    pub fn build(self) -> WorkerScriptPlainTextBindingEl {
        WorkerScriptPlainTextBindingEl {
            name: self.name,
            text: self.text,
        }
    }
}

pub struct WorkerScriptPlainTextBindingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkerScriptPlainTextBindingElRef {
    fn new(shared: StackShared, base: String) -> WorkerScriptPlainTextBindingElRef {
        WorkerScriptPlainTextBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptPlainTextBindingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `text` after provisioning.\nThe plain text you want to store."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptQueueBindingEl {
    binding: PrimField<String>,
    queue: PrimField<String>,
}

impl WorkerScriptQueueBindingEl { }

impl ToListMappable for WorkerScriptQueueBindingEl {
    type O = BlockAssignable<WorkerScriptQueueBindingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptQueueBindingEl {
    #[doc= "The name of the global variable for the binding in your Worker code."]
    pub binding: PrimField<String>,
    #[doc= "Name of the queue you want to use."]
    pub queue: PrimField<String>,
}

impl BuildWorkerScriptQueueBindingEl {
    pub fn build(self) -> WorkerScriptQueueBindingEl {
        WorkerScriptQueueBindingEl {
            binding: self.binding,
            queue: self.queue,
        }
    }
}

pub struct WorkerScriptQueueBindingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkerScriptQueueBindingElRef {
    fn new(shared: StackShared, base: String) -> WorkerScriptQueueBindingElRef {
        WorkerScriptQueueBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptQueueBindingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `binding` after provisioning.\nThe name of the global variable for the binding in your Worker code."]
    pub fn binding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.binding", self.base))
    }

    #[doc= "Get a reference to the value of field `queue` after provisioning.\nName of the queue you want to use."]
    pub fn queue(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptR2BucketBindingEl {
    bucket_name: PrimField<String>,
    name: PrimField<String>,
}

impl WorkerScriptR2BucketBindingEl { }

impl ToListMappable for WorkerScriptR2BucketBindingEl {
    type O = BlockAssignable<WorkerScriptR2BucketBindingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptR2BucketBindingEl {
    #[doc= "The name of the Bucket to bind to."]
    pub bucket_name: PrimField<String>,
    #[doc= "The global variable for the binding in your Worker code."]
    pub name: PrimField<String>,
}

impl BuildWorkerScriptR2BucketBindingEl {
    pub fn build(self) -> WorkerScriptR2BucketBindingEl {
        WorkerScriptR2BucketBindingEl {
            bucket_name: self.bucket_name,
            name: self.name,
        }
    }
}

pub struct WorkerScriptR2BucketBindingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkerScriptR2BucketBindingElRef {
    fn new(shared: StackShared, base: String) -> WorkerScriptR2BucketBindingElRef {
        WorkerScriptR2BucketBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptR2BucketBindingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of the Bucket to bind to."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptSecretTextBindingEl {
    name: PrimField<String>,
    text: PrimField<String>,
}

impl WorkerScriptSecretTextBindingEl { }

impl ToListMappable for WorkerScriptSecretTextBindingEl {
    type O = BlockAssignable<WorkerScriptSecretTextBindingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptSecretTextBindingEl {
    #[doc= "The global variable for the binding in your Worker code."]
    pub name: PrimField<String>,
    #[doc= "The secret text you want to store."]
    pub text: PrimField<String>,
}

impl BuildWorkerScriptSecretTextBindingEl {
    pub fn build(self) -> WorkerScriptSecretTextBindingEl {
        WorkerScriptSecretTextBindingEl {
            name: self.name,
            text: self.text,
        }
    }
}

pub struct WorkerScriptSecretTextBindingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkerScriptSecretTextBindingElRef {
    fn new(shared: StackShared, base: String) -> WorkerScriptSecretTextBindingElRef {
        WorkerScriptSecretTextBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptSecretTextBindingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `text` after provisioning.\nThe secret text you want to store."]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptServiceBindingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<PrimField<String>>,
    name: PrimField<String>,
    service: PrimField<String>,
}

impl WorkerScriptServiceBindingEl {
    #[doc= "Set the field `environment`.\nThe name of the Worker environment to bind to."]
    pub fn set_environment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment = Some(v.into());
        self
    }
}

impl ToListMappable for WorkerScriptServiceBindingEl {
    type O = BlockAssignable<WorkerScriptServiceBindingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptServiceBindingEl {
    #[doc= "The global variable for the binding in your Worker code."]
    pub name: PrimField<String>,
    #[doc= "The name of the Worker to bind to."]
    pub service: PrimField<String>,
}

impl BuildWorkerScriptServiceBindingEl {
    pub fn build(self) -> WorkerScriptServiceBindingEl {
        WorkerScriptServiceBindingEl {
            environment: core::default::Default::default(),
            name: self.name,
            service: self.service,
        }
    }
}

pub struct WorkerScriptServiceBindingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkerScriptServiceBindingElRef {
    fn new(shared: StackShared, base: String) -> WorkerScriptServiceBindingElRef {
        WorkerScriptServiceBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptServiceBindingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nThe name of the Worker environment to bind to."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\nThe name of the Worker to bind to."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptWebassemblyBindingEl {
    module: PrimField<String>,
    name: PrimField<String>,
}

impl WorkerScriptWebassemblyBindingEl { }

impl ToListMappable for WorkerScriptWebassemblyBindingEl {
    type O = BlockAssignable<WorkerScriptWebassemblyBindingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptWebassemblyBindingEl {
    #[doc= "The base64 encoded wasm module you want to store."]
    pub module: PrimField<String>,
    #[doc= "The global variable for the binding in your Worker code."]
    pub name: PrimField<String>,
}

impl BuildWorkerScriptWebassemblyBindingEl {
    pub fn build(self) -> WorkerScriptWebassemblyBindingEl {
        WorkerScriptWebassemblyBindingEl {
            module: self.module,
            name: self.name,
        }
    }
}

pub struct WorkerScriptWebassemblyBindingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkerScriptWebassemblyBindingElRef {
    fn new(shared: StackShared, base: String) -> WorkerScriptWebassemblyBindingElRef {
        WorkerScriptWebassemblyBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptWebassemblyBindingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `module` after provisioning.\nThe base64 encoded wasm module you want to store."]
    pub fn module(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.module", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkerScriptDynamic {
    analytics_engine_binding: Option<DynamicBlock<WorkerScriptAnalyticsEngineBindingEl>>,
    kv_namespace_binding: Option<DynamicBlock<WorkerScriptKvNamespaceBindingEl>>,
    plain_text_binding: Option<DynamicBlock<WorkerScriptPlainTextBindingEl>>,
    queue_binding: Option<DynamicBlock<WorkerScriptQueueBindingEl>>,
    r2_bucket_binding: Option<DynamicBlock<WorkerScriptR2BucketBindingEl>>,
    secret_text_binding: Option<DynamicBlock<WorkerScriptSecretTextBindingEl>>,
    service_binding: Option<DynamicBlock<WorkerScriptServiceBindingEl>>,
    webassembly_binding: Option<DynamicBlock<WorkerScriptWebassemblyBindingEl>>,
}
