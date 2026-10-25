#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataWorkersDeploymentData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    deployment_id: terrars::PrimField<std::string::String>,
    script_name: terrars::PrimField<std::string::String>,
}

struct DataWorkersDeployment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWorkersDeploymentData_>,
}

#[derive(Clone)]
pub struct DataWorkersDeployment(std::rc::Rc<DataWorkersDeployment_>);

impl DataWorkersDeployment {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> DataWorkersDeployment_AnnotationsRef {
        DataWorkersDeployment_AnnotationsRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `author_email` after provisioning.\n"]
    pub fn author_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\n"]
    pub fn deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_name` after provisioning.\nName of the script."]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\nAvailable values: \"percentage\"."]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `versions` after provisioning.\nWorker versions included in this deployment. Each object must contain a `version_id` UUID and a `percentage`; percentages across all objects must total 100. In the `cf` CLI, pass the entire array as one JSON value to `--versions`, either inline, for example `--versions '[{\"version_id\":\"023e105f-2a42-4f8b-a1c1-73f6a2a30c0f\",\"percentage\":100}]'`, or from a JSON file with `--versions @versions.json`."]
    pub fn versions(&self) -> terrars::ListRef<DataWorkersDeployment_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.extract_ref()))
    }
}

impl terrars::Referable for DataWorkersDeployment {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWorkersDeployment { }

impl terrars::ToListMappable for DataWorkersDeployment {
    type O = terrars::ListRef<DataWorkersDeploymentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWorkersDeployment_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_workers_deployment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWorkersDeployment {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub deployment_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the script."]
    pub script_name: terrars::PrimField<std::string::String>,
}

impl BuildDataWorkersDeployment {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWorkersDeployment {
        let out = DataWorkersDeployment(std::rc::Rc::new(DataWorkersDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWorkersDeploymentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                deployment_id: self.deployment_id,
                script_name: self.script_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWorkersDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWorkersDeploymentRef {
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

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> DataWorkersDeployment_AnnotationsRef {
        DataWorkersDeployment_AnnotationsRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `author_email` after provisioning.\n"]
    pub fn author_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\n"]
    pub fn deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_name` after provisioning.\nName of the script."]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\nAvailable values: \"percentage\"."]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `versions` after provisioning.\nWorker versions included in this deployment. Each object must contain a `version_id` UUID and a `percentage`; percentages across all objects must total 100. In the `cf` CLI, pass the entire array as one JSON value to `--versions`, either inline, for example `--versions '[{\"version_id\":\"023e105f-2a42-4f8b-a1c1-73f6a2a30c0f\",\"percentage\":100}]'`, or from a JSON file with `--versions @versions.json`."]
    pub fn versions(&self) -> terrars::ListRef<DataWorkersDeployment_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWorkersDeployment_Annotations {}

impl DataWorkersDeployment_Annotations { }

impl terrars::ToListMappable for DataWorkersDeployment_Annotations {
    type O = terrars::BlockAssignable<DataWorkersDeployment_Annotations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersDeployment_Annotations {}

impl BuildDataWorkersDeployment_Annotations {
    pub fn build(self) -> DataWorkersDeployment_Annotations {
        DataWorkersDeployment_Annotations {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersDeployment_AnnotationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersDeployment_AnnotationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersDeployment_AnnotationsRef {
        DataWorkersDeployment_AnnotationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersDeployment_AnnotationsRef {
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
pub struct DataWorkersDeployment_Versions {}

impl DataWorkersDeployment_Versions { }

impl terrars::ToListMappable for DataWorkersDeployment_Versions {
    type O = terrars::BlockAssignable<DataWorkersDeployment_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkersDeployment_Versions {}

impl BuildDataWorkersDeployment_Versions {
    pub fn build(self) -> DataWorkersDeployment_Versions {
        DataWorkersDeployment_Versions {}
    }
}

#[allow(dead_code)]
pub struct DataWorkersDeployment_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkersDeployment_VersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkersDeployment_VersionsRef {
        DataWorkersDeployment_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkersDeployment_VersionsRef {
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
