#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataWorkersScriptsData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataWorkersScripts_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWorkersScriptsData_>,
}

#[derive(Clone)]
pub struct DataWorkersScripts(std::rc::Rc<DataWorkersScripts_>);

impl DataWorkersScripts {
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

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nFilter scripts by tags. Format: comma-separated list of tag:allowed pairs where allowed is 'yes' or 'no'."]
    pub fn set_tags(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataWorkersScripts_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nFilter scripts by tags. Format: comma-separated list of tag:allowed pairs where allowed is 'yes' or 'no'."]
    pub fn tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataWorkersScripts {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWorkersScripts { }

impl terrars::ToListMappable for DataWorkersScripts {
    type O = terrars::ListRef<DataWorkersScriptsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWorkersScripts_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_workers_scripts".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWorkersScripts {
    pub tf_id: std::string::String,
}

impl BuildDataWorkersScripts {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWorkersScripts {
        let out = DataWorkersScripts(std::rc::Rc::new(DataWorkersScripts_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWorkersScriptsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWorkersScriptsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScriptsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWorkersScriptsRef {
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

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataWorkersScripts_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nFilter scripts by tags. Format: comma-separated list of tag:allowed pairs where allowed is 'yes' or 'no'."]
    pub fn tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWorkersScripts_Result_CacheOptions {}

impl DataWorkersScripts_Result_CacheOptions { }

impl terrars::ToListMappable for DataWorkersScripts_Result_CacheOptions {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_CacheOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_CacheOptions {}

impl BuildDataWorkersScripts_Result_CacheOptions {
    pub fn build(self) -> DataWorkersScripts_Result_CacheOptions {
        DataWorkersScripts_Result_CacheOptions {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_CacheOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_CacheOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersScripts_Result_CacheOptionsRef {
        DataWorkersScripts_Result_CacheOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_CacheOptionsRef {
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
pub struct DataWorkersScripts_Result_Exports_Cache {}

impl DataWorkersScripts_Result_Exports_Cache { }

impl terrars::ToListMappable for DataWorkersScripts_Result_Exports_Cache {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_Exports_Cache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_Exports_Cache {}

impl BuildDataWorkersScripts_Result_Exports_Cache {
    pub fn build(self) -> DataWorkersScripts_Result_Exports_Cache {
        DataWorkersScripts_Result_Exports_Cache {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_Exports_CacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_Exports_CacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersScripts_Result_Exports_CacheRef {
        DataWorkersScripts_Result_Exports_CacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_Exports_CacheRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether caching is enabled for this entrypoint."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkersScripts_Result_Exports {}

impl DataWorkersScripts_Result_Exports { }

impl terrars::ToListMappable for DataWorkersScripts_Result_Exports {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_Exports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_Exports {}

impl BuildDataWorkersScripts_Result_Exports {
    pub fn build(self) -> DataWorkersScripts_Result_Exports {
        DataWorkersScripts_Result_Exports {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_ExportsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_ExportsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersScripts_Result_ExportsRef {
        DataWorkersScripts_Result_ExportsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_ExportsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache` after provisioning.\nCache override for this entrypoint. It applies only to\n`type: worker` entries and overrides the Worker's global\n`cache_options.enabled` for that entrypoint."]
    pub fn cache(&self) -> DataWorkersScripts_Result_Exports_CacheRef {
        DataWorkersScripts_Result_Exports_CacheRef::new(self.shared().clone(), format!("{}.cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `renamed_to` after provisioning.\nDestination class name for a `state: renamed` tombstone. The\ntarget must appear as a live (`created`) entry in the same\n`exports` map. Write-only: never present in GET responses."]
    pub fn renamed_to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.renamed_to", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nLifecycle state of the export entry. Defaults to `created`\n(a normal, live export) when omitted.\n\n`deleted`, `renamed`, and `transferred` are tombstones:\nwrite-only lifecycle operations that retire, rename, or hand\noff a provisioned Durable Object namespace. They are applied\nat upload and are filtered out of GET responses, so a read\nonly ever returns `created` or `expecting-transfer`.\n\n`expecting-transfer` is a live export whose data is being\nreceived from another script via the two-phase transfer flow;\nit carries `storage` and `transfer_from`.\nAvailable values: \"created\", \"deleted\", \"renamed\", \"transferred\", \"expecting-transfer\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage` after provisioning.\nStorage backend for a `type: durable-object` export. Required\nfor live Durable Object entries (`created` and\n`expecting-transfer`). `sqlite` selects SQLite-backed storage;\n`legacy-kv` selects the legacy key-value storage.\nAvailable values: \"sqlite\", \"legacy-kv\"."]
    pub fn storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transfer_from` after provisioning.\nSource script for a `state: expecting-transfer` entry. The\nnamespace on this script is materialised from the source\nscript's data via the pending-transfer flow. Present on reads\nfor `expecting-transfer` entries."]
    pub fn transfer_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_from", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transferred_to` after provisioning.\nDestination script for a `state: transferred` tombstone. Must\nreference a script in the same account; cross-dispatch-namespace\ntransfers are rejected. Write-only: never present in GET\nresponses."]
    pub fn transferred_to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transferred_to", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe kind of export.\nAvailable values: \"worker\", \"durable-object\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkersScripts_Result_NamedHandlers {}

impl DataWorkersScripts_Result_NamedHandlers { }

impl terrars::ToListMappable for DataWorkersScripts_Result_NamedHandlers {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_NamedHandlers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_NamedHandlers {}

impl BuildDataWorkersScripts_Result_NamedHandlers {
    pub fn build(self) -> DataWorkersScripts_Result_NamedHandlers {
        DataWorkersScripts_Result_NamedHandlers {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_NamedHandlersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_NamedHandlersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersScripts_Result_NamedHandlersRef {
        DataWorkersScripts_Result_NamedHandlersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_NamedHandlersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `handlers` after provisioning.\nThe names of handlers exported as part of the named export."]
    pub fn handlers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handlers", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the export."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkersScripts_Result_Observability_Issues {}

impl DataWorkersScripts_Result_Observability_Issues { }

impl terrars::ToListMappable for DataWorkersScripts_Result_Observability_Issues {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_Observability_Issues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_Observability_Issues {}

impl BuildDataWorkersScripts_Result_Observability_Issues {
    pub fn build(self) -> DataWorkersScripts_Result_Observability_Issues {
        DataWorkersScripts_Result_Observability_Issues {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_Observability_IssuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_Observability_IssuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkersScripts_Result_Observability_IssuesRef {
        DataWorkersScripts_Result_Observability_IssuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_Observability_IssuesRef {
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
pub struct DataWorkersScripts_Result_Observability_Logs {}

impl DataWorkersScripts_Result_Observability_Logs { }

impl terrars::ToListMappable for DataWorkersScripts_Result_Observability_Logs {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_Observability_Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_Observability_Logs {}

impl BuildDataWorkersScripts_Result_Observability_Logs {
    pub fn build(self) -> DataWorkersScripts_Result_Observability_Logs {
        DataWorkersScripts_Result_Observability_Logs {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_Observability_LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_Observability_LogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkersScripts_Result_Observability_LogsRef {
        DataWorkersScripts_Result_Observability_LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_Observability_LogsRef {
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
        "Get a reference to the value of field `head_sampling_rate` after provisioning.\nThe sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1."]
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
pub struct DataWorkersScripts_Result_Observability_Traces {}

impl DataWorkersScripts_Result_Observability_Traces { }

impl terrars::ToListMappable for DataWorkersScripts_Result_Observability_Traces {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_Observability_Traces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_Observability_Traces {}

impl BuildDataWorkersScripts_Result_Observability_Traces {
    pub fn build(self) -> DataWorkersScripts_Result_Observability_Traces {
        DataWorkersScripts_Result_Observability_Traces {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_Observability_TracesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_Observability_TracesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkersScripts_Result_Observability_TracesRef {
        DataWorkersScripts_Result_Observability_TracesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_Observability_TracesRef {
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
        "Get a reference to the value of field `head_sampling_rate` after provisioning.\nThe sampling rate for traces. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1."]
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
pub struct DataWorkersScripts_Result_Observability {}

impl DataWorkersScripts_Result_Observability { }

impl terrars::ToListMappable for DataWorkersScripts_Result_Observability {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_Observability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_Observability {}

impl BuildDataWorkersScripts_Result_Observability {
    pub fn build(self) -> DataWorkersScripts_Result_Observability {
        DataWorkersScripts_Result_Observability {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_ObservabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_ObservabilityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersScripts_Result_ObservabilityRef {
        DataWorkersScripts_Result_ObservabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_ObservabilityRef {
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
        "Get a reference to the value of field `head_sampling_rate` after provisioning.\nThe sampling rate for incoming requests. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1."]
    pub fn head_sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.head_sampling_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `issues` after provisioning.\nReal-time Issues settings for the Worker."]
    pub fn issues(&self) -> DataWorkersScripts_Result_Observability_IssuesRef {
        DataWorkersScripts_Result_Observability_IssuesRef::new(
            self.shared().clone(),
            format!("{}.issues", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\nLog settings for the Worker."]
    pub fn logs(&self) -> DataWorkersScripts_Result_Observability_LogsRef {
        DataWorkersScripts_Result_Observability_LogsRef::new(self.shared().clone(), format!("{}.logs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redact_query_string` after provisioning.\nWhether query strings are removed from request URLs in logs and traces."]
    pub fn redact_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redact_query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `traces` after provisioning.\nTrace settings for the Worker."]
    pub fn traces(&self) -> DataWorkersScripts_Result_Observability_TracesRef {
        DataWorkersScripts_Result_Observability_TracesRef::new(
            self.shared().clone(),
            format!("{}.traces", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataWorkersScripts_Result_Placement_Target {}

impl DataWorkersScripts_Result_Placement_Target { }

impl terrars::ToListMappable for DataWorkersScripts_Result_Placement_Target {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_Placement_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_Placement_Target {}

impl BuildDataWorkersScripts_Result_Placement_Target {
    pub fn build(self) -> DataWorkersScripts_Result_Placement_Target {
        DataWorkersScripts_Result_Placement_Target {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_Placement_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_Placement_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersScripts_Result_Placement_TargetRef {
        DataWorkersScripts_Result_Placement_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_Placement_TargetRef {
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
pub struct DataWorkersScripts_Result_Placement {}

impl DataWorkersScripts_Result_Placement { }

impl terrars::ToListMappable for DataWorkersScripts_Result_Placement {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_Placement {}

impl BuildDataWorkersScripts_Result_Placement {
    pub fn build(self) -> DataWorkersScripts_Result_Placement {
        DataWorkersScripts_Result_Placement {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_PlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersScripts_Result_PlacementRef {
        DataWorkersScripts_Result_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_PlacementRef {
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
        "Get a reference to the value of field `last_analyzed_at` after provisioning.\nThe last time the script was analyzed for [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement)."]
    pub fn last_analyzed_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_analyzed_at", self.base))
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
        "Get a reference to the value of field `status` after provisioning.\nStatus of [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).\nAvailable values: \"SUCCESS\", \"UNSUPPORTED_APPLICATION\", \"INSUFFICIENT_INVOCATIONS\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nArray of placement targets (currently limited to single target)."]
    pub fn target(&self) -> terrars::ListRef<DataWorkersScripts_Result_Placement_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkersScripts_Result_Routes {}

impl DataWorkersScripts_Result_Routes { }

impl terrars::ToListMappable for DataWorkersScripts_Result_Routes {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_Routes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_Routes {}

impl BuildDataWorkersScripts_Result_Routes {
    pub fn build(self) -> DataWorkersScripts_Result_Routes {
        DataWorkersScripts_Result_Routes {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_RoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_RoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersScripts_Result_RoutesRef {
        DataWorkersScripts_Result_RoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_RoutesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nPattern to match incoming requests against. [Learn more](https://developers.cloudflare.com/workers/configuration/routing/routes/#matching-behavior)."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script` after provisioning.\nName of the script to run if the route matches."]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkersScripts_Result_TailConsumers {}

impl DataWorkersScripts_Result_TailConsumers { }

impl terrars::ToListMappable for DataWorkersScripts_Result_TailConsumers {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result_TailConsumers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result_TailConsumers {}

impl BuildDataWorkersScripts_Result_TailConsumers {
    pub fn build(self) -> DataWorkersScripts_Result_TailConsumers {
        DataWorkersScripts_Result_TailConsumers {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_Result_TailConsumersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_Result_TailConsumersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersScripts_Result_TailConsumersRef {
        DataWorkersScripts_Result_TailConsumersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_Result_TailConsumersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nOptional environment if the Worker utilizes one."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nOptional dispatch namespace the script belongs to."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nName of Worker that is to be the consumer."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkersScripts_Result {}

impl DataWorkersScripts_Result { }

impl terrars::ToListMappable for DataWorkersScripts_Result {
    type O = terrars::BlockAssignable<DataWorkersScripts_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersScripts_Result {}

impl BuildDataWorkersScripts_Result {
    pub fn build(self) -> DataWorkersScripts_Result {
        DataWorkersScripts_Result {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersScripts_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersScripts_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersScripts_ResultRef {
        DataWorkersScripts_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersScripts_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache_options` after provisioning.\nGlobal CacheW configuration for the Worker. When caching is on,\nthe platform provisions a `cloudflare.app` zone for the Worker.\nA `type: worker` entry in the `exports` map can override this\nvalue for a single entrypoint."]
    pub fn cache_options(&self) -> DataWorkersScripts_Result_CacheOptionsRef {
        DataWorkersScripts_Result_CacheOptionsRef::new(self.shared().clone(), format!("{}.cache_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_date` after provisioning.\nDate indicating targeted support in the Workers runtime. Backwards incompatible fixes to the runtime following this date will not affect this Worker."]
    pub fn compatibility_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_flags` after provisioning.\nFlags that enable or disable certain features in the Workers runtime. Used to enable upcoming features or opt in or out of specific changes not included in a `compatibility_date`."]
    pub fn compatibility_flags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the script was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nHashed script content, can be used in a If-None-Match header when updating."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exports` after provisioning.\nDeclarative exports for the Worker's most recent version,\nincluding Durable Object classes (with their `storage`\nbackend) and named Worker entrypoints. Tombstoned lifecycle\nentries are omitted, so only live exports (`created` and\n`expecting-transfer`) are returned."]
    pub fn exports(&self) -> terrars::RecRef<DataWorkersScripts_Result_ExportsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.exports", self.base))
    }

    #[doc =
        "Get a reference to the value of field `handlers` after provisioning.\nThe names of handlers exported as part of the default export."]
    pub fn handlers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handlers", self.base))
    }

    #[doc = "Get a reference to the value of field `has_assets` after provisioning.\nWhether a Worker contains assets."]
    pub fn has_assets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_assets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `has_modules` after provisioning.\nWhether a Worker contains modules."]
    pub fn has_modules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_modules", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe name used to identify the script."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_deployed_from` after provisioning.\nThe client most recently used to deploy this Worker."]
    pub fn last_deployed_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_deployed_from", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logpush` after provisioning.\nWhether Logpush is turned on for the Worker."]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.base))
    }

    #[doc =
        "Get a reference to the value of field `migration_tag` after provisioning.\nThe tag of the Durable Object migration that was most recently applied for this Worker."]
    pub fn migration_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.migration_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the script was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `named_handlers` after provisioning.\nNamed exports, such as Durable Object class implementations and named entrypoints."]
    pub fn named_handlers(&self) -> terrars::ListRef<DataWorkersScripts_Result_NamedHandlersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.named_handlers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `observability` after provisioning.\nObservability settings for the Worker."]
    pub fn observability(&self) -> DataWorkersScripts_Result_ObservabilityRef {
        DataWorkersScripts_Result_ObservabilityRef::new(
            self.shared().clone(),
            format!("{}.observability", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nConfiguration for [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement). Specify mode='smart' for Smart Placement, or one of region/hostname/host."]
    pub fn placement(&self) -> DataWorkersScripts_Result_PlacementRef {
        DataWorkersScripts_Result_PlacementRef::new(self.shared().clone(), format!("{}.placement", self.base))
    }

    #[doc =
        "Get a reference to the value of field `placement_mode` after provisioning.\nAvailable values: \"smart\", \"targeted\"."]
    pub fn placement_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `placement_status` after provisioning.\nAvailable values: \"SUCCESS\", \"UNSUPPORTED_APPLICATION\", \"INSUFFICIENT_INVOCATIONS\"."]
    pub fn placement_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_status", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\nRoutes associated with the Worker."]
    pub fn routes(&self) -> terrars::ListRef<DataWorkersScripts_Result_RoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe immutable ID of the script."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags associated with the Worker."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tail_consumers` after provisioning.\nList of Workers that will consume logs from the attached Worker."]
    pub fn tail_consumers(&self) -> terrars::SetRef<DataWorkersScripts_Result_TailConsumersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tail_consumers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nUsage model for the Worker invocations.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.base))
    }
}
