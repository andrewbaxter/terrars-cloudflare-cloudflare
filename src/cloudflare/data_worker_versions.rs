#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataWorkerVersionsData_ {
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
    worker_id: terrars::PrimField<std::string::String>,
}

struct DataWorkerVersions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWorkerVersionsData_>,
}

#[derive(Clone)]
pub struct DataWorkerVersions(std::rc::Rc<DataWorkerVersions_>);

impl DataWorkerVersions {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataWorkerVersions_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `worker_id` after provisioning.\nIdentifier for the Worker, which can be ID or name."]
    pub fn worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataWorkerVersions {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWorkerVersions { }

impl terrars::ToListMappable for DataWorkerVersions {
    type O = terrars::ListRef<DataWorkerVersionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWorkerVersions_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_worker_versions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWorkerVersions {
    pub tf_id: std::string::String,
    #[doc = "Identifier for the Worker, which can be ID or name."]
    pub worker_id: terrars::PrimField<std::string::String>,
}

impl BuildDataWorkerVersions {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWorkerVersions {
        let out = DataWorkerVersions(std::rc::Rc::new(DataWorkerVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWorkerVersionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                worker_id: self.worker_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWorkerVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWorkerVersionsRef {
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
    pub fn result(&self) -> terrars::ListRef<DataWorkerVersions_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `worker_id` after provisioning.\nIdentifier for the Worker, which can be ID or name."]
    pub fn worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Annotations {}

impl DataWorkerVersions_Result_Annotations { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Annotations {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Annotations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Annotations {}

impl BuildDataWorkerVersions_Result_Annotations {
    pub fn build(self) -> DataWorkerVersions_Result_Annotations {
        DataWorkerVersions_Result_Annotations {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_AnnotationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_AnnotationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_AnnotationsRef {
        DataWorkerVersions_Result_AnnotationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_AnnotationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `workers_message` after provisioning.\nHuman-readable message about the version. Truncated to 1000 bytes if longer."]
    pub fn workers_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workers_message", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workers_tag` after provisioning.\nUser-provided identifier for the version. Maximum 100 bytes."]
    pub fn workers_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workers_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workers_triggered_by` after provisioning.\nOperation that triggered the creation of the version."]
    pub fn workers_triggered_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workers_triggered_by", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Assets_Config {}

impl DataWorkerVersions_Result_Assets_Config { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Assets_Config {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Assets_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Assets_Config {}

impl BuildDataWorkerVersions_Result_Assets_Config {
    pub fn build(self) -> DataWorkerVersions_Result_Assets_Config {
        DataWorkerVersions_Result_Assets_Config {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Assets_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Assets_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_Assets_ConfigRef {
        DataWorkerVersions_Result_Assets_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Assets_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `base_path` after provisioning.\nThe public URL path prefix under which assets are served. A null request value resets it to `/`; responses represent the root as `/`. All versions in a gradual deployment must use the same canonical value. To change it, first deploy the version containing the change at 100%."]
    pub fn base_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `html_handling` after provisioning.\nDetermines the redirects and rewrites of requests for HTML content.\nAvailable values: \"auto-trailing-slash\", \"force-trailing-slash\", \"drop-trailing-slash\", \"none\"."]
    pub fn html_handling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html_handling", self.base))
    }

    #[doc =
        "Get a reference to the value of field `not_found_handling` after provisioning.\nDetermines the response when a request does not match a static asset, and there is no Worker script.\nAvailable values: \"none\", \"404-page\", \"single-page-application\"."]
    pub fn not_found_handling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_found_handling", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_worker_first` after provisioning.\nContains a list path rules to control routing to either the Worker or assets. Glob (*) and negative (!) rules are supported. Rules must start with either '/' or '!/'. At least one non-negative rule must be provided, and negative rules have higher precedence than non-negative rules."]
    pub fn run_worker_first(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run_worker_first", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Assets {}

impl DataWorkerVersions_Result_Assets { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Assets {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Assets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Assets {}

impl BuildDataWorkerVersions_Result_Assets {
    pub fn build(self) -> DataWorkerVersions_Result_Assets {
        DataWorkerVersions_Result_Assets {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_AssetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_AssetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_AssetsRef {
        DataWorkerVersions_Result_AssetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_AssetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nConfiguration for assets within a Worker."]
    pub fn config(&self) -> DataWorkerVersions_Result_Assets_ConfigRef {
        DataWorkerVersions_Result_Assets_ConfigRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jwt` after provisioning.\nToken provided upon successful upload of all files from a registered manifest."]
    pub fn jwt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwt", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Bindings_Outbound_Params {}

impl DataWorkerVersions_Result_Bindings_Outbound_Params { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Bindings_Outbound_Params {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Bindings_Outbound_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Bindings_Outbound_Params {}

impl BuildDataWorkerVersions_Result_Bindings_Outbound_Params {
    pub fn build(self) -> DataWorkerVersions_Result_Bindings_Outbound_Params {
        DataWorkerVersions_Result_Bindings_Outbound_Params {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Bindings_Outbound_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Bindings_Outbound_ParamsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkerVersions_Result_Bindings_Outbound_ParamsRef {
        DataWorkerVersions_Result_Bindings_Outbound_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Bindings_Outbound_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the parameter."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Bindings_Outbound_Worker {}

impl DataWorkerVersions_Result_Bindings_Outbound_Worker { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Bindings_Outbound_Worker {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Bindings_Outbound_Worker>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Bindings_Outbound_Worker {}

impl BuildDataWorkerVersions_Result_Bindings_Outbound_Worker {
    pub fn build(self) -> DataWorkerVersions_Result_Bindings_Outbound_Worker {
        DataWorkerVersions_Result_Bindings_Outbound_Worker {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Bindings_Outbound_WorkerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Bindings_Outbound_WorkerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkerVersions_Result_Bindings_Outbound_WorkerRef {
        DataWorkerVersions_Result_Bindings_Outbound_WorkerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Bindings_Outbound_WorkerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `entrypoint` after provisioning.\nEntrypoint to invoke on the outbound worker."]
    pub fn entrypoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entrypoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nEnvironment of the outbound worker."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\nName of the outbound worker."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Bindings_Outbound {}

impl DataWorkerVersions_Result_Bindings_Outbound { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Bindings_Outbound {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Bindings_Outbound>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Bindings_Outbound {}

impl BuildDataWorkerVersions_Result_Bindings_Outbound {
    pub fn build(self) -> DataWorkerVersions_Result_Bindings_Outbound {
        DataWorkerVersions_Result_Bindings_Outbound {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Bindings_OutboundRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Bindings_OutboundRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkerVersions_Result_Bindings_OutboundRef {
        DataWorkerVersions_Result_Bindings_OutboundRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Bindings_OutboundRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nPass information from the Dispatch Worker to the Outbound Worker through the parameters."]
    pub fn params(&self) -> terrars::ListRef<DataWorkerVersions_Result_Bindings_Outbound_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }

    #[doc = "Get a reference to the value of field `worker` after provisioning.\nOutbound worker."]
    pub fn worker(&self) -> DataWorkerVersions_Result_Bindings_Outbound_WorkerRef {
        DataWorkerVersions_Result_Bindings_Outbound_WorkerRef::new(
            self.shared().clone(),
            format!("{}.worker", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Bindings_Retry {}

impl DataWorkerVersions_Result_Bindings_Retry { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Bindings_Retry {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Bindings_Retry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Bindings_Retry {}

impl BuildDataWorkerVersions_Result_Bindings_Retry {
    pub fn build(self) -> DataWorkerVersions_Result_Bindings_Retry {
        DataWorkerVersions_Result_Bindings_Retry {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Bindings_RetryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Bindings_RetryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_Bindings_RetryRef {
        DataWorkerVersions_Result_Bindings_RetryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Bindings_RetryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_attempts` after provisioning.\nMaximum number of retries after the initial request, not the total number of attempts. Defaults to 4. Zero disables retries."]
    pub fn max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_ms` after provisioning.\nRetry timeout in milliseconds, measured from the start of the call. No retry starts after it expires, and a retry still running when it expires is cancelled. This is not a request timeout; it does not limit the initial request, which is still subject to any timeouts set by your Worker. Defaults to 10000."]
    pub fn timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_ms", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Bindings_Simple {}

impl DataWorkerVersions_Result_Bindings_Simple { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Bindings_Simple {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Bindings_Simple>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Bindings_Simple {}

impl BuildDataWorkerVersions_Result_Bindings_Simple {
    pub fn build(self) -> DataWorkerVersions_Result_Bindings_Simple {
        DataWorkerVersions_Result_Bindings_Simple {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Bindings_SimpleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Bindings_SimpleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_Bindings_SimpleRef {
        DataWorkerVersions_Result_Bindings_SimpleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Bindings_SimpleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limit` after provisioning.\nThe limit (requests per period)."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mitigation_timeout` after provisioning.\nDuration in seconds to apply the mitigation action after the rate limit is exceeded. Valid values are 0 (disabled), 10, or multiples of 60 up to 86400. Must be greater than or equal to the period when non-zero."]
    pub fn mitigation_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mitigation_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `period` after provisioning.\nThe period in seconds."]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Bindings {}

impl DataWorkerVersions_Result_Bindings { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Bindings {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Bindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Bindings {}

impl BuildDataWorkerVersions_Result_Bindings {
    pub fn build(self) -> DataWorkerVersions_Result_Bindings {
        DataWorkerVersions_Result_Bindings {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_BindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_BindingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_BindingsRef {
        DataWorkerVersions_Result_BindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_BindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `algorithm` after provisioning.\nAlgorithm-specific key parameters. [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#algorithm)."]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_destination_addresses` after provisioning.\nList of allowed destination addresses."]
    pub fn allowed_destination_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_destination_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_sender_addresses` after provisioning.\nList of allowed sender addresses."]
    pub fn allowed_sender_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_sender_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nID of the Flagship app to bind to for feature flag evaluation."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nR2 bucket to bind to."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `certificate_id` after provisioning.\nIdentifier of the certificate to bind to."]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `class_name` after provisioning.\nThe exported class name of the Durable Object."]
    pub fn class_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_id` after provisioning.\nIdentifier of the D1 database to bind to."]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nThe name of the dataset to bind to."]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `destination_address` after provisioning.\nDestination address for the email."]
    pub fn destination_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dispatch_namespace` after provisioning.\nThe dispatch namespace the Durable Object script belongs to."]
    pub fn dispatch_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dispatch_namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `entrypoint` after provisioning.\nEntrypoint to invoke on the target Worker."]
    pub fn entrypoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entrypoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nThe environment of the script_name to bind to."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nData format of the key. [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#format).\nAvailable values: \"raw\", \"pkcs8\", \"spki\", \"jwk\"."]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the D1 database to bind to."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity` after provisioning.\nEnables Gateway identity for the binding. Requires network_id to be \"cf1:network\" and cannot be combined with tunnel_id.\nAvailable values: \"runtime-email-alpha\"."]
    pub fn identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `index_name` after provisioning.\nName of the Vectorize index to bind to."]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_name` after provisioning.\nThe user-chosen instance name. Must exist at deploy time. The worker can search, chat, update, and manage items/jobs on this instance."]
    pub fn instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\nJSON data to use."]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jurisdiction` after provisioning.\nThe [jurisdiction](https://developers.cloudflare.com/r2/reference/data-location/#jurisdictional-restrictions) of the R2 bucket.\nAvailable values: \"eu\", \"fedramp\", \"fedramp-high\", \"us\"."]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_base64` after provisioning.\nBase64-encoded key data. Required if `format` is \"raw\", \"pkcs8\", or \"spki\"."]
    pub fn key_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_base64", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_jwk` after provisioning.\nKey data in [JSON Web Key](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#json_web_key) format. Required if `format` is \"jwk\"."]
    pub fn key_jwk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_jwk", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA JavaScript variable name for the binding."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nThe namespace the instance belongs to. Defaults to \"default\" if omitted. Customers who don't use namespaces can simply omit this field."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\nNamespace identifier tag."]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_id` after provisioning.\nIdentifier of the network to bind to. Only \"cf1:network\" is currently supported. Mutually exclusive with tunnel_id."]
    pub fn network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `old_name` after provisioning.\nThe old name of the inherited binding. If set, the binding will be renamed from `old_name` to `name` in the new version. If not set, the binding will keep the same name between versions."]
    pub fn old_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.old_name", self.base))
    }

    #[doc = "Get a reference to the value of field `outbound` after provisioning.\nOutbound worker."]
    pub fn outbound(&self) -> DataWorkerVersions_Result_Bindings_OutboundRef {
        DataWorkerVersions_Result_Bindings_OutboundRef::new(self.shared().clone(), format!("{}.outbound", self.base))
    }

    #[doc =
        "Get a reference to the value of field `part` after provisioning.\nThe name of the file containing the data content. Only accepted for `service worker syntax` Workers."]
    pub fn part(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.part", self.base))
    }

    #[doc = "Get a reference to the value of field `pipeline` after provisioning.\nName of the Pipeline to bind to."]
    pub fn pipeline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\nName of the Queue to bind to."]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry` after provisioning.\nRetry policy for calls made to the Durable Object through this binding. Omitted or null properties use the runtime defaults. These limits are upper bounds and do not enable retries for otherwise ineligible calls."]
    pub fn retry(&self) -> DataWorkerVersions_Result_Bindings_RetryRef {
        DataWorkerVersions_Result_Bindings_RetryRef::new(self.shared().clone(), format!("{}.retry", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script_name` after provisioning.\nThe script where the Durable Object is defined, if it is external to this Worker."]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\nName of the secret in the store."]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\nName of Worker to bind to."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_id` after provisioning.\nIdentifier of the VPC service to bind to."]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.base))
    }

    #[doc = "Get a reference to the value of field `simple` after provisioning.\nThe rate limit configuration."]
    pub fn simple(&self) -> DataWorkerVersions_Result_Bindings_SimpleRef {
        DataWorkerVersions_Result_Bindings_SimpleRef::new(self.shared().clone(), format!("{}.simple", self.base))
    }

    #[doc =
        "Get a reference to the value of field `store_id` after provisioning.\nID of the store containing the secret."]
    pub fn store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.store_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stream` after provisioning.\nID of a K2 stream owned by the account deploying the Worker."]
    pub fn stream(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe text value to use."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the Cloudflare Tunnel to bind to. Mutually exclusive with network_id."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe kind of resource that the binding provides.\nAvailable values: \"ai\", \"ai_search\", \"ai_search_namespace\", \"messaging\", \"analytics_engine\", \"artifacts\", \"assets\", \"browser\", \"d1\", \"data_blob\", \"dispatch_namespace\", \"durable_object_namespace\", \"hyperdrive\", \"inherit\", \"images\", \"json\", \"kv_namespace\", \"media\", \"mtls_certificate\", \"plain_text\", \"pipelines\", \"k2\", \"queue\", \"ratelimit\", \"r2_bucket\", \"secret_text\", \"send_email\", \"service\", \"text_blob\", \"vectorize\", \"version_metadata\", \"secrets_store_secret\", \"flagship\", \"secret_key\", \"workflow\", \"wasm_module\", \"vpc_service\", \"vpc_network\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usages` after provisioning.\nAllowed operations with the key. [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#keyUsages)."]
    pub fn usages(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.usages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version_id` after provisioning.\nIdentifier for the version to inherit the binding from, which can be the version ID or the literal \"latest\" to inherit from the latest version. Defaults to inheriting the binding from the latest version."]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workflow_name` after provisioning.\nName of the Workflow to bind to."]
    pub fn workflow_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_CacheOptions {}

impl DataWorkerVersions_Result_CacheOptions { }

impl terrars::ToListMappable for DataWorkerVersions_Result_CacheOptions {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_CacheOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_CacheOptions {}

impl BuildDataWorkerVersions_Result_CacheOptions {
    pub fn build(self) -> DataWorkerVersions_Result_CacheOptions {
        DataWorkerVersions_Result_CacheOptions {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_CacheOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_CacheOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_CacheOptionsRef {
        DataWorkerVersions_Result_CacheOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_CacheOptionsRef {
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
pub struct DataWorkerVersions_Result_Containers {}

impl DataWorkerVersions_Result_Containers { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Containers {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Containers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Containers {}

impl BuildDataWorkerVersions_Result_Containers {
    pub fn build(self) -> DataWorkerVersions_Result_Containers {
        DataWorkerVersions_Result_Containers {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_ContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_ContainersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_ContainersRef {
        DataWorkerVersions_Result_ContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_ContainersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `class_name` after provisioning.\nSelect which Durable Object class should get this container attached."]
    pub fn class_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Exports_Cache {}

impl DataWorkerVersions_Result_Exports_Cache { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Exports_Cache {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Exports_Cache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Exports_Cache {}

impl BuildDataWorkerVersions_Result_Exports_Cache {
    pub fn build(self) -> DataWorkerVersions_Result_Exports_Cache {
        DataWorkerVersions_Result_Exports_Cache {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Exports_CacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Exports_CacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_Exports_CacheRef {
        DataWorkerVersions_Result_Exports_CacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Exports_CacheRef {
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
pub struct DataWorkerVersions_Result_Exports {}

impl DataWorkerVersions_Result_Exports { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Exports {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Exports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Exports {}

impl BuildDataWorkerVersions_Result_Exports {
    pub fn build(self) -> DataWorkerVersions_Result_Exports {
        DataWorkerVersions_Result_Exports {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_ExportsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_ExportsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_ExportsRef {
        DataWorkerVersions_Result_ExportsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_ExportsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache` after provisioning.\nCache override for this entrypoint. It applies only to\n`type: worker` entries and overrides the Worker's global\n`cache_options.enabled` for that entrypoint."]
    pub fn cache(&self) -> DataWorkerVersions_Result_Exports_CacheRef {
        DataWorkerVersions_Result_Exports_CacheRef::new(self.shared().clone(), format!("{}.cache", self.base))
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
pub struct DataWorkerVersions_Result_Limits {}

impl DataWorkerVersions_Result_Limits { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Limits {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Limits {}

impl BuildDataWorkerVersions_Result_Limits {
    pub fn build(self) -> DataWorkerVersions_Result_Limits {
        DataWorkerVersions_Result_Limits {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_LimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_LimitsRef {
        DataWorkerVersions_Result_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_ms` after provisioning.\nCPU time limit in milliseconds."]
    pub fn cpu_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `subrequests` after provisioning.\nSubrequest limit per request."]
    pub fn subrequests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subrequests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Migrations_RenamedClasses {}

impl DataWorkerVersions_Result_Migrations_RenamedClasses { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Migrations_RenamedClasses {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Migrations_RenamedClasses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Migrations_RenamedClasses {}

impl BuildDataWorkerVersions_Result_Migrations_RenamedClasses {
    pub fn build(self) -> DataWorkerVersions_Result_Migrations_RenamedClasses {
        DataWorkerVersions_Result_Migrations_RenamedClasses {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Migrations_RenamedClassesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Migrations_RenamedClassesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkerVersions_Result_Migrations_RenamedClassesRef {
        DataWorkerVersions_Result_Migrations_RenamedClassesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Migrations_RenamedClassesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Migrations_Steps_RenamedClasses {}

impl DataWorkerVersions_Result_Migrations_Steps_RenamedClasses { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Migrations_Steps_RenamedClasses {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Migrations_Steps_RenamedClasses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Migrations_Steps_RenamedClasses {}

impl BuildDataWorkerVersions_Result_Migrations_Steps_RenamedClasses {
    pub fn build(self) -> DataWorkerVersions_Result_Migrations_Steps_RenamedClasses {
        DataWorkerVersions_Result_Migrations_Steps_RenamedClasses {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Migrations_Steps_RenamedClassesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Migrations_Steps_RenamedClassesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkerVersions_Result_Migrations_Steps_RenamedClassesRef {
        DataWorkerVersions_Result_Migrations_Steps_RenamedClassesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Migrations_Steps_RenamedClassesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Migrations_Steps_TransferredClasses {}

impl DataWorkerVersions_Result_Migrations_Steps_TransferredClasses { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Migrations_Steps_TransferredClasses {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Migrations_Steps_TransferredClasses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Migrations_Steps_TransferredClasses {}

impl BuildDataWorkerVersions_Result_Migrations_Steps_TransferredClasses {
    pub fn build(self) -> DataWorkerVersions_Result_Migrations_Steps_TransferredClasses {
        DataWorkerVersions_Result_Migrations_Steps_TransferredClasses {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Migrations_Steps_TransferredClassesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Migrations_Steps_TransferredClassesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkerVersions_Result_Migrations_Steps_TransferredClassesRef {
        DataWorkerVersions_Result_Migrations_Steps_TransferredClassesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Migrations_Steps_TransferredClassesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `from_script` after provisioning.\n"]
    pub fn from_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_script", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Migrations_Steps {}

impl DataWorkerVersions_Result_Migrations_Steps { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Migrations_Steps {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Migrations_Steps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Migrations_Steps {}

impl BuildDataWorkerVersions_Result_Migrations_Steps {
    pub fn build(self) -> DataWorkerVersions_Result_Migrations_Steps {
        DataWorkerVersions_Result_Migrations_Steps {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Migrations_StepsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Migrations_StepsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_Migrations_StepsRef {
        DataWorkerVersions_Result_Migrations_StepsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Migrations_StepsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deleted_classes` after provisioning.\nA list of classes to delete Durable Object namespaces from."]
    pub fn deleted_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deleted_classes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `new_classes` after provisioning.\nA list of classes to create Durable Object namespaces from."]
    pub fn new_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_classes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `new_sqlite_classes` after provisioning.\nA list of classes to create Durable Object namespaces with SQLite from."]
    pub fn new_sqlite_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_sqlite_classes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `renamed_classes` after provisioning.\nA list of classes with Durable Object namespaces that were renamed."]
    pub fn renamed_classes(&self) -> terrars::ListRef<DataWorkerVersions_Result_Migrations_Steps_RenamedClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.renamed_classes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transferred_classes` after provisioning.\nA list of transfers for Durable Object namespaces from a different Worker and class to a class defined in this Worker."]
    pub fn transferred_classes(
        &self,
    ) -> terrars::ListRef<DataWorkerVersions_Result_Migrations_Steps_TransferredClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transferred_classes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Migrations_TransferredClasses {}

impl DataWorkerVersions_Result_Migrations_TransferredClasses { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Migrations_TransferredClasses {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Migrations_TransferredClasses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Migrations_TransferredClasses {}

impl BuildDataWorkerVersions_Result_Migrations_TransferredClasses {
    pub fn build(self) -> DataWorkerVersions_Result_Migrations_TransferredClasses {
        DataWorkerVersions_Result_Migrations_TransferredClasses {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Migrations_TransferredClassesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Migrations_TransferredClassesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkerVersions_Result_Migrations_TransferredClassesRef {
        DataWorkerVersions_Result_Migrations_TransferredClassesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Migrations_TransferredClassesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `from_script` after provisioning.\n"]
    pub fn from_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_script", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Migrations {}

impl DataWorkerVersions_Result_Migrations { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Migrations {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Migrations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Migrations {}

impl BuildDataWorkerVersions_Result_Migrations {
    pub fn build(self) -> DataWorkerVersions_Result_Migrations {
        DataWorkerVersions_Result_Migrations {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_MigrationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_MigrationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_MigrationsRef {
        DataWorkerVersions_Result_MigrationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_MigrationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deleted_classes` after provisioning.\nA list of classes to delete Durable Object namespaces from."]
    pub fn deleted_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deleted_classes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `new_classes` after provisioning.\nA list of classes to create Durable Object namespaces from."]
    pub fn new_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_classes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `new_sqlite_classes` after provisioning.\nA list of classes to create Durable Object namespaces with SQLite from."]
    pub fn new_sqlite_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_sqlite_classes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `new_tag` after provisioning.\nTag to set as the latest migration tag."]
    pub fn new_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.new_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `old_tag` after provisioning.\nTag used to verify against the latest migration tag for this Worker. If they don't match, the upload is rejected."]
    pub fn old_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.old_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `renamed_classes` after provisioning.\nA list of classes with Durable Object namespaces that were renamed."]
    pub fn renamed_classes(&self) -> terrars::ListRef<DataWorkerVersions_Result_Migrations_RenamedClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.renamed_classes", self.base))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\nMigrations to apply in order."]
    pub fn steps(&self) -> terrars::ListRef<DataWorkerVersions_Result_Migrations_StepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transferred_classes` after provisioning.\nA list of transfers for Durable Object namespaces from a different Worker and class to a class defined in this Worker."]
    pub fn transferred_classes(&self) -> terrars::ListRef<DataWorkerVersions_Result_Migrations_TransferredClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transferred_classes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Modules {}

impl DataWorkerVersions_Result_Modules { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Modules {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Modules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Modules {}

impl BuildDataWorkerVersions_Result_Modules {
    pub fn build(self) -> DataWorkerVersions_Result_Modules {
        DataWorkerVersions_Result_Modules {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_ModulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_ModulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_ModulesRef {
        DataWorkerVersions_Result_ModulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_ModulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content_base64` after provisioning.\nThe base64-encoded module content."]
    pub fn content_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\nThe content type of the module."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the module."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_PackageDependencies {}

impl DataWorkerVersions_Result_PackageDependencies { }

impl terrars::ToListMappable for DataWorkerVersions_Result_PackageDependencies {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_PackageDependencies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_PackageDependencies {}

impl BuildDataWorkerVersions_Result_PackageDependencies {
    pub fn build(self) -> DataWorkerVersions_Result_PackageDependencies {
        DataWorkerVersions_Result_PackageDependencies {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_PackageDependenciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_PackageDependenciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkerVersions_Result_PackageDependenciesRef {
        DataWorkerVersions_Result_PackageDependenciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_PackageDependenciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `installed_version` after provisioning.\nThe exact version that was resolved and installed by the package manager."]
    pub fn installed_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.installed_version", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe npm package name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `package_json_version` after provisioning.\nThe version constraint as written in package.json."]
    pub fn package_json_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_json_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result_Placement_Target {}

impl DataWorkerVersions_Result_Placement_Target { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Placement_Target {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Placement_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Placement_Target {}

impl BuildDataWorkerVersions_Result_Placement_Target {
    pub fn build(self) -> DataWorkerVersions_Result_Placement_Target {
        DataWorkerVersions_Result_Placement_Target {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_Placement_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_Placement_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_Placement_TargetRef {
        DataWorkerVersions_Result_Placement_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_Placement_TargetRef {
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
pub struct DataWorkerVersions_Result_Placement {}

impl DataWorkerVersions_Result_Placement { }

impl terrars::ToListMappable for DataWorkerVersions_Result_Placement {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result_Placement {}

impl BuildDataWorkerVersions_Result_Placement {
    pub fn build(self) -> DataWorkerVersions_Result_Placement {
        DataWorkerVersions_Result_Placement {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_Result_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_Result_PlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_Result_PlacementRef {
        DataWorkerVersions_Result_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_Result_PlacementRef {
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
    pub fn target(&self) -> terrars::ListRef<DataWorkerVersions_Result_Placement_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkerVersions_Result {}

impl DataWorkerVersions_Result { }

impl terrars::ToListMappable for DataWorkerVersions_Result {
    type O = terrars::BlockAssignable<DataWorkerVersions_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkerVersions_Result {}

impl BuildDataWorkerVersions_Result {
    pub fn build(self) -> DataWorkerVersions_Result {
        DataWorkerVersions_Result {}
    }
}

#[allow(dead_code)]
pub struct DataWorkerVersions_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkerVersions_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkerVersions_ResultRef {
        DataWorkerVersions_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkerVersions_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nMetadata about the version."]
    pub fn annotations(&self) -> DataWorkerVersions_Result_AnnotationsRef {
        DataWorkerVersions_Result_AnnotationsRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `assets` after provisioning.\nConfiguration for assets within a Worker.\n\n[`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers) and\n[`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/) files should be\nincluded as modules named `_headers` and `_redirects` with content type `text/plain`."]
    pub fn assets(&self) -> DataWorkerVersions_Result_AssetsRef {
        DataWorkerVersions_Result_AssetsRef::new(self.shared().clone(), format!("{}.assets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `author_email` after provisioning.\nEmail of the user who created the version."]
    pub fn author_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `author_id` after provisioning.\nIdentifier of the user who created the version."]
    pub fn author_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bindings` after provisioning.\nList of bindings attached to a Worker. You can find more about bindings on our docs: https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings."]
    pub fn bindings(&self) -> terrars::ListRef<DataWorkerVersions_Result_BindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_options` after provisioning.\nGlobal CacheW configuration for the Worker. When caching is on,\nthe platform provisions a `cloudflare.app` zone for the Worker.\nA `type: worker` entry in the `exports` map can override this\nvalue for a single entrypoint."]
    pub fn cache_options(&self) -> DataWorkerVersions_Result_CacheOptionsRef {
        DataWorkerVersions_Result_CacheOptionsRef::new(self.shared().clone(), format!("{}.cache_options", self.base))
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

    #[doc =
        "Get a reference to the value of field `containers` after provisioning.\nList of containers attached to a Worker. Containers can only be attached to Durable Object classes of this Worker script."]
    pub fn containers(&self) -> terrars::SetRef<DataWorkerVersions_Result_ContainersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the version was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exports` after provisioning.\nDeclarative exports for the version, including Durable Object\nclasses (with their `storage` backend) and named Worker\nentrypoints. On reads, tombstoned lifecycle entries are\nomitted, so only live exports (`created` and\n`expecting-transfer`) are returned. `exports` and `migrations`\nare mutually exclusive on upload."]
    pub fn exports(&self) -> terrars::RecRef<DataWorkerVersions_Result_ExportsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.exports", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nVersion identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\nResource limits enforced at runtime."]
    pub fn limits(&self) -> DataWorkerVersions_Result_LimitsRef {
        DataWorkerVersions_Result_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_module` after provisioning.\nThe name of the main module in the `modules` array (e.g. the name of the module that exports a `fetch` handler)."]
    pub fn main_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_module", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_script_base64` after provisioning.\nThe base64-encoded main script content. This is only returned for service worker syntax workers (not ES modules)."]
    pub fn main_script_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_script_base64", self.base))
    }

    #[doc =
        "Get a reference to the value of field `migration_tag` after provisioning.\nDurable Object migration tag. Set when the version is deployed. Omitted if the version has not been deployed or the Worker does not use Durable Objects."]
    pub fn migration_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.migration_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `migrations` after provisioning.\nMigrations for Durable Objects associated with the version. Migrations are applied when the version is deployed."]
    pub fn migrations(&self) -> DataWorkerVersions_Result_MigrationsRef {
        DataWorkerVersions_Result_MigrationsRef::new(self.shared().clone(), format!("{}.migrations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modules` after provisioning.\nCode, sourcemaps, and other content used at runtime.\n\nThis includes [`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers) and\n[`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/) files used to configure\n[Static Assets](https://developers.cloudflare.com/workers/static-assets/). `_headers` and `_redirects` files should be\nincluded as modules named `_headers` and `_redirects` with content type `text/plain`."]
    pub fn modules(&self) -> terrars::SetRef<DataWorkerVersions_Result_ModulesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.modules", self.base))
    }

    #[doc =
        "Get a reference to the value of field `number` after provisioning.\nThe integer version number, starting from one."]
    pub fn number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number", self.base))
    }

    #[doc =
        "Get a reference to the value of field `package_dependencies` after provisioning.\nThe list of npm packages that were installed and used when this Worker\nversion was built."]
    pub fn package_dependencies(&self) -> terrars::ListRef<DataWorkerVersions_Result_PackageDependenciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.package_dependencies", self.base))
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nConfiguration for [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement). Specify mode='smart' for Smart Placement, or one of region/hostname/host."]
    pub fn placement(&self) -> DataWorkerVersions_Result_PlacementRef {
        DataWorkerVersions_Result_PlacementRef::new(self.shared().clone(), format!("{}.placement", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nThe client used to create the version."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc =
        "Get a reference to the value of field `startup_time_ms` after provisioning.\nTime in milliseconds spent on [Worker startup](https://developers.cloudflare.com/workers/platform/limits/#worker-startup-time)."]
    pub fn startup_time_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_time_ms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `urls` after provisioning.\nAll routable URLs that always point to this version. Does not include alias URLs, since aliases can be updated to point to a different version."]
    pub fn urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.urls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nUsage model for the version.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.base))
    }
}
