#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WorkersDeploymentData_ {
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
    annotations: std::option::Option<WorkersDeployment_Annotations>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force: std::option::Option<terrars::PrimField<bool>>,
    script_name: terrars::PrimField<std::string::String>,
    strategy: terrars::PrimField<std::string::String>,
    versions: std::vec::Vec<WorkersDeployment_Versions>,
}

struct WorkersDeployment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkersDeploymentData_>,
}

#[derive(Clone)]
pub struct WorkersDeployment(std::rc::Rc<WorkersDeployment_>);

impl WorkersDeployment {
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

    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(self, v: impl std::convert::Into<WorkersDeployment_Annotations>) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force`.\nIf set to true, the deployment will be created even if normally blocked by something such rolling back to an older version when a secret has changed."]
    pub fn set_force(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> WorkersDeployment_AnnotationsRef {
        WorkersDeployment_AnnotationsRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `author_email` after provisioning.\n"]
    pub fn author_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force` after provisioning.\nIf set to true, the deployment will be created even if normally blocked by something such rolling back to an older version when a secret has changed."]
    pub fn force(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force", self.extract_ref()))
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
    pub fn versions(&self) -> terrars::ListRef<WorkersDeployment_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.extract_ref()))
    }
}

impl terrars::Referable for WorkersDeployment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkersDeployment { }

impl terrars::ToListMappable for WorkersDeployment {
    type O = terrars::ListRef<WorkersDeploymentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkersDeployment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_workers_deployment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkersDeployment {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the script."]
    pub script_name: terrars::PrimField<std::string::String>,
    #[doc = "Available values: \"percentage\"."]
    pub strategy: terrars::PrimField<std::string::String>,
    #[doc =
        "Worker versions included in this deployment. Each object must contain a `version_id` UUID and a `percentage`; percentages across all objects must total 100. In the `cf` CLI, pass the entire array as one JSON value to `--versions`, either inline, for example `--versions '[{\"version_id\":\"023e105f-2a42-4f8b-a1c1-73f6a2a30c0f\",\"percentage\":100}]'`, or from a JSON file with `--versions @versions.json`."]
    pub versions: std::vec::Vec<WorkersDeployment_Versions>,
}

impl BuildWorkersDeployment {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkersDeployment {
        let out = WorkersDeployment(std::rc::Rc::new(WorkersDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkersDeploymentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                annotations: core::default::Default::default(),
                force: core::default::Default::default(),
                script_name: self.script_name,
                strategy: self.strategy,
                versions: self.versions,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkersDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkersDeploymentRef {
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

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> WorkersDeployment_AnnotationsRef {
        WorkersDeployment_AnnotationsRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `author_email` after provisioning.\n"]
    pub fn author_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force` after provisioning.\nIf set to true, the deployment will be created even if normally blocked by something such rolling back to an older version when a secret has changed."]
    pub fn force(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force", self.extract_ref()))
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
    pub fn versions(&self) -> terrars::ListRef<WorkersDeployment_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WorkersDeployment_Annotations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workers_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkersDeployment_Annotations {
    #[doc =
        "Set the field `workers_message`.\nHuman-readable message about the deployment. Truncated to 1000 bytes if longer."]
    pub fn set_workers_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workers_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersDeployment_Annotations {
    type O = terrars::BlockAssignable<WorkersDeployment_Annotations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersDeployment_Annotations {}

impl BuildWorkersDeployment_Annotations {
    pub fn build(self) -> WorkersDeployment_Annotations {
        WorkersDeployment_Annotations { workers_message: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct WorkersDeployment_AnnotationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersDeployment_AnnotationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersDeployment_AnnotationsRef {
        WorkersDeployment_AnnotationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersDeployment_AnnotationsRef {
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
pub struct WorkersDeployment_Versions {
    percentage: terrars::PrimField<f64>,
    version_id: terrars::PrimField<std::string::String>,
}

impl WorkersDeployment_Versions { }

impl terrars::ToListMappable for WorkersDeployment_Versions {
    type O = terrars::BlockAssignable<WorkersDeployment_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersDeployment_Versions {
    #[doc = "Percentage of traffic served by this version."]
    pub percentage: terrars::PrimField<f64>,
    #[doc = "Identifier of the Worker Version."]
    pub version_id: terrars::PrimField<std::string::String>,
}

impl BuildWorkersDeployment_Versions {
    pub fn build(self) -> WorkersDeployment_Versions {
        WorkersDeployment_Versions {
            percentage: self.percentage,
            version_id: self.version_id,
        }
    }
}

#[allow(dead_code)]
pub struct WorkersDeployment_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersDeployment_VersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersDeployment_VersionsRef {
        WorkersDeployment_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersDeployment_VersionsRef {
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
