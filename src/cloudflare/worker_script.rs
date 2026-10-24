use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WorkerScriptData {
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
    compatibility_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_flags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logpush: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    module: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analytics_engine_binding: std::option::Option<std::vec::Vec<WorkerScriptAnalyticsEngineBindingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kv_namespace_binding: std::option::Option<std::vec::Vec<WorkerScriptKvNamespaceBindingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plain_text_binding: std::option::Option<std::vec::Vec<WorkerScriptPlainTextBindingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_binding: std::option::Option<std::vec::Vec<WorkerScriptQueueBindingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r2_bucket_binding: std::option::Option<std::vec::Vec<WorkerScriptR2BucketBindingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_text_binding: std::option::Option<std::vec::Vec<WorkerScriptSecretTextBindingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_binding: std::option::Option<std::vec::Vec<WorkerScriptServiceBindingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webassembly_binding: std::option::Option<std::vec::Vec<WorkerScriptWebassemblyBindingEl>>,
    dynamic: WorkerScriptDynamic,
}

struct WorkerScript_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkerScriptData>,
}

#[derive(Clone)]
pub struct WorkerScript(std::rc::Rc<WorkerScript_>);

impl WorkerScript {
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

    #[doc = "Set the field `compatibility_date`.\nThe date to use for the compatibility flag."]
    pub fn set_compatibility_date(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().compatibility_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compatibility_flags`.\nCompatibility flags used for Worker Scripts."]
    pub fn set_compatibility_flags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().compatibility_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logpush`.\nEnabling allows Worker events to be sent to a defined Logpush destination."]
    pub fn set_logpush(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().logpush = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `module`.\nWhether to upload Worker as a module."]
    pub fn set_module(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().module = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analytics_engine_binding`.\n"]
    pub fn set_analytics_engine_binding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkerScriptAnalyticsEngineBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().analytics_engine_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.analytics_engine_binding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kv_namespace_binding`.\n"]
    pub fn set_kv_namespace_binding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkerScriptKvNamespaceBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kv_namespace_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kv_namespace_binding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `plain_text_binding`.\n"]
    pub fn set_plain_text_binding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkerScriptPlainTextBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().plain_text_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.plain_text_binding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `queue_binding`.\n"]
    pub fn set_queue_binding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkerScriptQueueBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().queue_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.queue_binding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `r2_bucket_binding`.\n"]
    pub fn set_r2_bucket_binding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkerScriptR2BucketBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().r2_bucket_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.r2_bucket_binding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_text_binding`.\n"]
    pub fn set_secret_text_binding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkerScriptSecretTextBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secret_text_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secret_text_binding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_binding`.\n"]
    pub fn set_service_binding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkerScriptServiceBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_binding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `webassembly_binding`.\n"]
    pub fn set_webassembly_binding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkerScriptWebassemblyBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().webassembly_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.webassembly_binding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_date` after provisioning.\nThe date to use for the compatibility flag."]
    pub fn compatibility_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_flags` after provisioning.\nCompatibility flags used for Worker Scripts."]
    pub fn compatibility_flags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe script content."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logpush` after provisioning.\nEnabling allows Worker events to be sent to a defined Logpush destination."]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `module` after provisioning.\nWhether to upload Worker as a module."]
    pub fn module(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.module", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name for the script. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl terrars::Referable for WorkerScript {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkerScript { }

impl terrars::ToListMappable for WorkerScript {
    type O = terrars::ListRef<WorkerScriptRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkerScript_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_worker_script".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkerScript {
    pub tf_id: std::string::String,
    #[doc = "The account identifier to target for the resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "The script content."]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = "The name for the script. **Modifying this attribute will force creation of a new resource.**"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWorkerScript {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkerScript {
        let out = WorkerScript(std::rc::Rc::new(WorkerScript_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkerScriptData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkerScriptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerScriptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkerScriptRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_date` after provisioning.\nThe date to use for the compatibility flag."]
    pub fn compatibility_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_flags` after provisioning.\nCompatibility flags used for Worker Scripts."]
    pub fn compatibility_flags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe script content."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logpush` after provisioning.\nEnabling allows Worker events to be sent to a defined Logpush destination."]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `module` after provisioning.\nWhether to upload Worker as a module."]
    pub fn module(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.module", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name for the script. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptAnalyticsEngineBindingEl {
    dataset: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl WorkerScriptAnalyticsEngineBindingEl { }

impl terrars::ToListMappable for WorkerScriptAnalyticsEngineBindingEl {
    type O = terrars::BlockAssignable<WorkerScriptAnalyticsEngineBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptAnalyticsEngineBindingEl {
    #[doc = "The name of the Analytics Engine dataset to write to."]
    pub dataset: terrars::PrimField<std::string::String>,
    #[doc = "The global variable for the binding in your Worker code."]
    pub name: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerScriptAnalyticsEngineBindingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerScriptAnalyticsEngineBindingElRef {
        WorkerScriptAnalyticsEngineBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptAnalyticsEngineBindingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nThe name of the Analytics Engine dataset to write to."]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptKvNamespaceBindingEl {
    name: terrars::PrimField<std::string::String>,
    namespace_id: terrars::PrimField<std::string::String>,
}

impl WorkerScriptKvNamespaceBindingEl { }

impl terrars::ToListMappable for WorkerScriptKvNamespaceBindingEl {
    type O = terrars::BlockAssignable<WorkerScriptKvNamespaceBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptKvNamespaceBindingEl {
    #[doc = "The global variable for the binding in your Worker code."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "ID of the KV namespace you want to use."]
    pub namespace_id: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerScriptKvNamespaceBindingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerScriptKvNamespaceBindingElRef {
        WorkerScriptKvNamespaceBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptKvNamespaceBindingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace_id` after provisioning.\nID of the KV namespace you want to use."]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptPlainTextBindingEl {
    name: terrars::PrimField<std::string::String>,
    text: terrars::PrimField<std::string::String>,
}

impl WorkerScriptPlainTextBindingEl { }

impl terrars::ToListMappable for WorkerScriptPlainTextBindingEl {
    type O = terrars::BlockAssignable<WorkerScriptPlainTextBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptPlainTextBindingEl {
    #[doc = "The global variable for the binding in your Worker code."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The plain text you want to store."]
    pub text: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerScriptPlainTextBindingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerScriptPlainTextBindingElRef {
        WorkerScriptPlainTextBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptPlainTextBindingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe plain text you want to store."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptQueueBindingEl {
    binding: terrars::PrimField<std::string::String>,
    queue: terrars::PrimField<std::string::String>,
}

impl WorkerScriptQueueBindingEl { }

impl terrars::ToListMappable for WorkerScriptQueueBindingEl {
    type O = terrars::BlockAssignable<WorkerScriptQueueBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptQueueBindingEl {
    #[doc = "The name of the global variable for the binding in your Worker code."]
    pub binding: terrars::PrimField<std::string::String>,
    #[doc = "Name of the queue you want to use."]
    pub queue: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerScriptQueueBindingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerScriptQueueBindingElRef {
        WorkerScriptQueueBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptQueueBindingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `binding` after provisioning.\nThe name of the global variable for the binding in your Worker code."]
    pub fn binding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.binding", self.base))
    }

    #[doc = "Get a reference to the value of field `queue` after provisioning.\nName of the queue you want to use."]
    pub fn queue(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptR2BucketBindingEl {
    bucket_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl WorkerScriptR2BucketBindingEl { }

impl terrars::ToListMappable for WorkerScriptR2BucketBindingEl {
    type O = terrars::BlockAssignable<WorkerScriptR2BucketBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptR2BucketBindingEl {
    #[doc = "The name of the Bucket to bind to."]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = "The global variable for the binding in your Worker code."]
    pub name: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerScriptR2BucketBindingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerScriptR2BucketBindingElRef {
        WorkerScriptR2BucketBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptR2BucketBindingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of the Bucket to bind to."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptSecretTextBindingEl {
    name: terrars::PrimField<std::string::String>,
    text: terrars::PrimField<std::string::String>,
}

impl WorkerScriptSecretTextBindingEl { }

impl terrars::ToListMappable for WorkerScriptSecretTextBindingEl {
    type O = terrars::BlockAssignable<WorkerScriptSecretTextBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptSecretTextBindingEl {
    #[doc = "The global variable for the binding in your Worker code."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The secret text you want to store."]
    pub text: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerScriptSecretTextBindingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerScriptSecretTextBindingElRef {
        WorkerScriptSecretTextBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptSecretTextBindingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe secret text you want to store."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptServiceBindingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
}

impl WorkerScriptServiceBindingEl {
    #[doc = "Set the field `environment`.\nThe name of the Worker environment to bind to."]
    pub fn set_environment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerScriptServiceBindingEl {
    type O = terrars::BlockAssignable<WorkerScriptServiceBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptServiceBindingEl {
    #[doc = "The global variable for the binding in your Worker code."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Worker to bind to."]
    pub service: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerScriptServiceBindingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerScriptServiceBindingElRef {
        WorkerScriptServiceBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptServiceBindingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nThe name of the Worker environment to bind to."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the Worker to bind to."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerScriptWebassemblyBindingEl {
    module: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl WorkerScriptWebassemblyBindingEl { }

impl terrars::ToListMappable for WorkerScriptWebassemblyBindingEl {
    type O = terrars::BlockAssignable<WorkerScriptWebassemblyBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerScriptWebassemblyBindingEl {
    #[doc = "The base64 encoded wasm module you want to store."]
    pub module: terrars::PrimField<std::string::String>,
    #[doc = "The global variable for the binding in your Worker code."]
    pub name: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerScriptWebassemblyBindingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerScriptWebassemblyBindingElRef {
        WorkerScriptWebassemblyBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerScriptWebassemblyBindingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `module` after provisioning.\nThe base64 encoded wasm module you want to store."]
    pub fn module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.module", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe global variable for the binding in your Worker code."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkerScriptDynamic {
    analytics_engine_binding: std::option::Option<terrars::DynamicBlock<WorkerScriptAnalyticsEngineBindingEl>>,
    kv_namespace_binding: std::option::Option<terrars::DynamicBlock<WorkerScriptKvNamespaceBindingEl>>,
    plain_text_binding: std::option::Option<terrars::DynamicBlock<WorkerScriptPlainTextBindingEl>>,
    queue_binding: std::option::Option<terrars::DynamicBlock<WorkerScriptQueueBindingEl>>,
    r2_bucket_binding: std::option::Option<terrars::DynamicBlock<WorkerScriptR2BucketBindingEl>>,
    secret_text_binding: std::option::Option<terrars::DynamicBlock<WorkerScriptSecretTextBindingEl>>,
    service_binding: std::option::Option<terrars::DynamicBlock<WorkerScriptServiceBindingEl>>,
    webassembly_binding: std::option::Option<terrars::DynamicBlock<WorkerScriptWebassemblyBindingEl>>,
}
