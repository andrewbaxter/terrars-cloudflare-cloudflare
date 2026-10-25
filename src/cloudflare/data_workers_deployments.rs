#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataWorkersDeploymentsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    script_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    since: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    until: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataWorkersDeployments_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWorkersDeploymentsData_>,
}

#[derive(Clone)]
pub struct DataWorkersDeployments(std::rc::Rc<DataWorkersDeployments_>);

impl DataWorkersDeployments {
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

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `since`.\nStart of the deployment creation time range, inclusive."]
    pub fn set_since(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().since = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `until`.\nEnd of the deployment creation time range, inclusive."]
    pub fn set_until(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().until = std::option::Option::Some(v.into());
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
    pub fn result(&self) -> terrars::ListRef<DataWorkersDeployments_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_name` after provisioning.\nName of the script."]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `since` after provisioning.\nStart of the deployment creation time range, inclusive."]
    pub fn since(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.since", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `until` after provisioning.\nEnd of the deployment creation time range, inclusive."]
    pub fn until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.until", self.extract_ref()))
    }
}

impl terrars::Referable for DataWorkersDeployments {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWorkersDeployments { }

impl terrars::ToListMappable for DataWorkersDeployments {
    type O = terrars::ListRef<DataWorkersDeploymentsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWorkersDeployments_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_workers_deployments".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWorkersDeployments {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the script."]
    pub script_name: terrars::PrimField<std::string::String>,
}

impl BuildDataWorkersDeployments {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWorkersDeployments {
        let out = DataWorkersDeployments(std::rc::Rc::new(DataWorkersDeployments_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWorkersDeploymentsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                max_items: core::default::Default::default(),
                script_name: self.script_name,
                since: core::default::Default::default(),
                until: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWorkersDeploymentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersDeploymentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWorkersDeploymentsRef {
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
    pub fn result(&self) -> terrars::ListRef<DataWorkersDeployments_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_name` after provisioning.\nName of the script."]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `since` after provisioning.\nStart of the deployment creation time range, inclusive."]
    pub fn since(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.since", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `until` after provisioning.\nEnd of the deployment creation time range, inclusive."]
    pub fn until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.until", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWorkersDeployments_Result_Deployments_Annotations {}

impl DataWorkersDeployments_Result_Deployments_Annotations { }

impl terrars::ToListMappable for DataWorkersDeployments_Result_Deployments_Annotations {
    type O = terrars::BlockAssignable<DataWorkersDeployments_Result_Deployments_Annotations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersDeployments_Result_Deployments_Annotations {}

impl BuildDataWorkersDeployments_Result_Deployments_Annotations {
    pub fn build(self) -> DataWorkersDeployments_Result_Deployments_Annotations {
        DataWorkersDeployments_Result_Deployments_Annotations {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersDeployments_Result_Deployments_AnnotationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersDeployments_Result_Deployments_AnnotationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkersDeployments_Result_Deployments_AnnotationsRef {
        DataWorkersDeployments_Result_Deployments_AnnotationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersDeployments_Result_Deployments_AnnotationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `workers_message` after provisioning.\nHuman-readable message about the deployment. Truncated to 1000 bytes if longer."]
    pub fn workers_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workers_message", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workers_triggered_by` after provisioning.\nOperation that triggered the creation of the deployment."]
    pub fn workers_triggered_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workers_triggered_by", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkersDeployments_Result_Deployments_Versions {}

impl DataWorkersDeployments_Result_Deployments_Versions { }

impl terrars::ToListMappable for DataWorkersDeployments_Result_Deployments_Versions {
    type O = terrars::BlockAssignable<DataWorkersDeployments_Result_Deployments_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersDeployments_Result_Deployments_Versions {}

impl BuildDataWorkersDeployments_Result_Deployments_Versions {
    pub fn build(self) -> DataWorkersDeployments_Result_Deployments_Versions {
        DataWorkersDeployments_Result_Deployments_Versions {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersDeployments_Result_Deployments_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersDeployments_Result_Deployments_VersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWorkersDeployments_Result_Deployments_VersionsRef {
        DataWorkersDeployments_Result_Deployments_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersDeployments_Result_Deployments_VersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nPercentage of traffic served by this version."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\nIdentifier of the Worker Version."]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkersDeployments_Result_Deployments {}

impl DataWorkersDeployments_Result_Deployments { }

impl terrars::ToListMappable for DataWorkersDeployments_Result_Deployments {
    type O = terrars::BlockAssignable<DataWorkersDeployments_Result_Deployments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersDeployments_Result_Deployments {}

impl BuildDataWorkersDeployments_Result_Deployments {
    pub fn build(self) -> DataWorkersDeployments_Result_Deployments {
        DataWorkersDeployments_Result_Deployments {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersDeployments_Result_DeploymentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersDeployments_Result_DeploymentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersDeployments_Result_DeploymentsRef {
        DataWorkersDeployments_Result_DeploymentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersDeployments_Result_DeploymentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> DataWorkersDeployments_Result_Deployments_AnnotationsRef {
        DataWorkersDeployments_Result_Deployments_AnnotationsRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `author_email` after provisioning.\n"]
    pub fn author_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\nAvailable values: \"percentage\"."]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `versions` after provisioning.\nWorker versions included in this deployment. Each object must contain a `version_id` UUID and a `percentage`; percentages across all objects must total 100. In the `cf` CLI, pass the entire array as one JSON value to `--versions`, either inline, for example `--versions '[{\"version_id\":\"023e105f-2a42-4f8b-a1c1-73f6a2a30c0f\",\"percentage\":100}]'`, or from a JSON file with `--versions @versions.json`."]
    pub fn versions(&self) -> terrars::ListRef<DataWorkersDeployments_Result_Deployments_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkersDeployments_Result {}

impl DataWorkersDeployments_Result { }

impl terrars::ToListMappable for DataWorkersDeployments_Result {
    type O = terrars::BlockAssignable<DataWorkersDeployments_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersDeployments_Result {}

impl BuildDataWorkersDeployments_Result {
    pub fn build(self) -> DataWorkersDeployments_Result {
        DataWorkersDeployments_Result {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersDeployments_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersDeployments_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersDeployments_ResultRef {
        DataWorkersDeployments_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersDeployments_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployments` after provisioning.\n"]
    pub fn deployments(&self) -> terrars::ListRef<DataWorkersDeployments_Result_DeploymentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployments", self.base))
    }
}
