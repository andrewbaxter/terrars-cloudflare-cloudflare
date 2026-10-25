#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataPagesProjectsData_ {
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
}

struct DataPagesProjects_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPagesProjectsData_>,
}

#[derive(Clone)]
pub struct DataPagesProjects(std::rc::Rc<DataPagesProjects_>);

impl DataPagesProjects {
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
    pub fn result(&self) -> terrars::ListRef<DataPagesProjects_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataPagesProjects {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPagesProjects { }

impl terrars::ToListMappable for DataPagesProjects {
    type O = terrars::ListRef<DataPagesProjectsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPagesProjects_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_pages_projects".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPagesProjects {
    pub tf_id: std::string::String,
}

impl BuildDataPagesProjects {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPagesProjects {
        let out = DataPagesProjects(std::rc::Rc::new(DataPagesProjects_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPagesProjectsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPagesProjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjectsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPagesProjectsRef {
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
    pub fn result(&self) -> terrars::ListRef<DataPagesProjects_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_BuildConfig {}

impl DataPagesProjects_Result_BuildConfig { }

impl terrars::ToListMappable for DataPagesProjects_Result_BuildConfig {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_BuildConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_BuildConfig {}

impl BuildDataPagesProjects_Result_BuildConfig {
    pub fn build(self) -> DataPagesProjects_Result_BuildConfig {
        DataPagesProjects_Result_BuildConfig {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_BuildConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_BuildConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPagesProjects_Result_BuildConfigRef {
        DataPagesProjects_Result_BuildConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_BuildConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `build_caching` after provisioning.\nEnable build caching for the project."]
    pub fn build_caching(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_caching", self.base))
    }

    #[doc = "Get a reference to the value of field `build_command` after provisioning.\nCommand used to build project."]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `destination_dir` after provisioning.\nAssets output directory of the build."]
    pub fn destination_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `root_dir` after provisioning.\nDirectory to run the command."]
    pub fn root_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `web_analytics_tag` after provisioning.\nThe classifying tag for analytics."]
    pub fn web_analytics_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_analytics_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `web_analytics_token` after provisioning.\nThe auth token for analytics."]
    pub fn web_analytics_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_analytics_token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_BuildImageAutoUpgrade {}

impl DataPagesProjects_Result_BuildImageAutoUpgrade { }

impl terrars::ToListMappable for DataPagesProjects_Result_BuildImageAutoUpgrade {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_BuildImageAutoUpgrade>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_BuildImageAutoUpgrade {}

impl BuildDataPagesProjects_Result_BuildImageAutoUpgrade {
    pub fn build(self) -> DataPagesProjects_Result_BuildImageAutoUpgrade {
        DataPagesProjects_Result_BuildImageAutoUpgrade {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_BuildImageAutoUpgradeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_BuildImageAutoUpgradeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_BuildImageAutoUpgradeRef {
        DataPagesProjects_Result_BuildImageAutoUpgradeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_BuildImageAutoUpgradeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `from_version` after provisioning.\nThe retired build image major version the project is set to."]
    pub fn from_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `to_version` after provisioning.\nThe build image major version the project's builds run on."]
    pub fn to_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_Metadata {}

impl DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_Metadata { }

impl terrars::ToListMappable for DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_Metadata {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_LatestDeployment_DeploymentTrigger_Metadata {}

impl BuildDataPagesProjects_Result_LatestDeployment_DeploymentTrigger_Metadata {
    pub fn build(self) -> DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_Metadata {
        DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_Metadata {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_MetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_MetadataRef {
        DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nWhere the trigger happened."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `commit_dirty` after provisioning.\nWhether the deployment trigger commit was dirty."]
    pub fn commit_dirty(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_dirty", self.base))
    }

    #[doc =
        "Get a reference to the value of field `commit_hash` after provisioning.\nHash of the deployment trigger commit."]
    pub fn commit_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_hash", self.base))
    }

    #[doc =
        "Get a reference to the value of field `commit_message` after provisioning.\nMessage of the deployment trigger commit."]
    pub fn commit_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_LatestDeployment_DeploymentTrigger {}

impl DataPagesProjects_Result_LatestDeployment_DeploymentTrigger { }

impl terrars::ToListMappable for DataPagesProjects_Result_LatestDeployment_DeploymentTrigger {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_LatestDeployment_DeploymentTrigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_LatestDeployment_DeploymentTrigger {}

impl BuildDataPagesProjects_Result_LatestDeployment_DeploymentTrigger {
    pub fn build(self) -> DataPagesProjects_Result_LatestDeployment_DeploymentTrigger {
        DataPagesProjects_Result_LatestDeployment_DeploymentTrigger {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_LatestDeployment_DeploymentTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_LatestDeployment_DeploymentTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_LatestDeployment_DeploymentTriggerRef {
        DataPagesProjects_Result_LatestDeployment_DeploymentTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_LatestDeployment_DeploymentTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nAdditional info about the trigger."]
    pub fn metadata(&self) -> DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_MetadataRef {
        DataPagesProjects_Result_LatestDeployment_DeploymentTrigger_MetadataRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nWhat caused the deployment.\nAvailable values: \"github:push\", \"ad_hoc\", \"deploy_hook\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_LatestDeployment_EnvVars {}

impl DataPagesProjects_Result_LatestDeployment_EnvVars { }

impl terrars::ToListMappable for DataPagesProjects_Result_LatestDeployment_EnvVars {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_LatestDeployment_EnvVars>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_LatestDeployment_EnvVars {}

impl BuildDataPagesProjects_Result_LatestDeployment_EnvVars {
    pub fn build(self) -> DataPagesProjects_Result_LatestDeployment_EnvVars {
        DataPagesProjects_Result_LatestDeployment_EnvVars {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_LatestDeployment_EnvVarsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_LatestDeployment_EnvVarsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_LatestDeployment_EnvVarsRef {
        DataPagesProjects_Result_LatestDeployment_EnvVarsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_LatestDeployment_EnvVarsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"plain_text\", \"secret_text\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nEnvironment variable value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_LatestDeployment_LatestStage {}

impl DataPagesProjects_Result_LatestDeployment_LatestStage { }

impl terrars::ToListMappable for DataPagesProjects_Result_LatestDeployment_LatestStage {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_LatestDeployment_LatestStage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_LatestDeployment_LatestStage {}

impl BuildDataPagesProjects_Result_LatestDeployment_LatestStage {
    pub fn build(self) -> DataPagesProjects_Result_LatestDeployment_LatestStage {
        DataPagesProjects_Result_LatestDeployment_LatestStage {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_LatestDeployment_LatestStageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_LatestDeployment_LatestStageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_LatestDeployment_LatestStageRef {
        DataPagesProjects_Result_LatestDeployment_LatestStageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_LatestDeployment_LatestStageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ended_on` after provisioning.\nWhen the stage ended."]
    pub fn ended_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ended_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe current build stage.\nAvailable values: \"queued\", \"initialize\", \"clone_repo\", \"build\", \"deploy\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `started_on` after provisioning.\nWhen the stage started."]
    pub fn started_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nState of the current stage.\nAvailable values: \"success\", \"idle\", \"active\", \"failure\", \"canceled\", \"skipped\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_Source_Config {}

impl DataPagesProjects_Result_Source_Config { }

impl terrars::ToListMappable for DataPagesProjects_Result_Source_Config {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_Source_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_Source_Config {}

impl BuildDataPagesProjects_Result_Source_Config {
    pub fn build(self) -> DataPagesProjects_Result_Source_Config {
        DataPagesProjects_Result_Source_Config {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_Source_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_Source_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPagesProjects_Result_Source_ConfigRef {
        DataPagesProjects_Result_Source_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_Source_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deployments_enabled` after provisioning.\nWhether to enable automatic deployments when pushing to the source repository.\nWhen disabled, no deployments (production or preview) will be triggered automatically."]
    pub fn deployments_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployments_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\nThe owner of the repository."]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\nThe owner ID of the repository."]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_excludes` after provisioning.\nA list of paths that should be excluded from triggering a preview deployment. Wildcard syntax (`*`) is supported."]
    pub fn path_excludes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_excludes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_includes` after provisioning.\nA list of paths that should be watched to trigger a preview deployment. Wildcard syntax (`*`) is supported."]
    pub fn path_includes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_includes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pr_comments_enabled` after provisioning.\nWhether to enable PR comments."]
    pub fn pr_comments_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pr_comments_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview_branch_excludes` after provisioning.\nA list of branches that should not trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`."]
    pub fn preview_branch_excludes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preview_branch_excludes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview_branch_includes` after provisioning.\nA list of branches that should trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`."]
    pub fn preview_branch_includes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preview_branch_includes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview_deployment_setting` after provisioning.\nControls whether commits to preview branches trigger a preview deployment.\nAvailable values: \"all\", \"none\", \"custom\"."]
    pub fn preview_deployment_setting(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview_deployment_setting", self.base))
    }

    #[doc =
        "Get a reference to the value of field `production_branch` after provisioning.\nThe production branch of the repository."]
    pub fn production_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `production_deployments_enabled` after provisioning.\nWhether to trigger a production deployment on commits to the production branch."]
    pub fn production_deployments_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_deployments_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_id` after provisioning.\nThe ID of the repository."]
    pub fn repo_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_id", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\nThe name of the repository."]
    pub fn repo_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_Source {}

impl DataPagesProjects_Result_Source { }

impl terrars::ToListMappable for DataPagesProjects_Result_Source {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_Source {}

impl BuildDataPagesProjects_Result_Source {
    pub fn build(self) -> DataPagesProjects_Result_Source {
        DataPagesProjects_Result_Source {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPagesProjects_Result_SourceRef {
        DataPagesProjects_Result_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> DataPagesProjects_Result_Source_ConfigRef {
        DataPagesProjects_Result_Source_ConfigRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe source control management provider.\nAvailable values: \"github\", \"gitlab\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_LatestDeployment_Stages {}

impl DataPagesProjects_Result_LatestDeployment_Stages { }

impl terrars::ToListMappable for DataPagesProjects_Result_LatestDeployment_Stages {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_LatestDeployment_Stages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_LatestDeployment_Stages {}

impl BuildDataPagesProjects_Result_LatestDeployment_Stages {
    pub fn build(self) -> DataPagesProjects_Result_LatestDeployment_Stages {
        DataPagesProjects_Result_LatestDeployment_Stages {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_LatestDeployment_StagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_LatestDeployment_StagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_LatestDeployment_StagesRef {
        DataPagesProjects_Result_LatestDeployment_StagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_LatestDeployment_StagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ended_on` after provisioning.\nWhen the stage ended."]
    pub fn ended_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ended_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe current build stage.\nAvailable values: \"queued\", \"initialize\", \"clone_repo\", \"build\", \"deploy\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `started_on` after provisioning.\nWhen the stage started."]
    pub fn started_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nState of the current stage.\nAvailable values: \"success\", \"idle\", \"active\", \"failure\", \"canceled\", \"skipped\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_CanonicalDeployment {}

impl DataPagesProjects_Result_CanonicalDeployment { }

impl terrars::ToListMappable for DataPagesProjects_Result_CanonicalDeployment {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_CanonicalDeployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_CanonicalDeployment {}

impl BuildDataPagesProjects_Result_CanonicalDeployment {
    pub fn build(self) -> DataPagesProjects_Result_CanonicalDeployment {
        DataPagesProjects_Result_CanonicalDeployment {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_CanonicalDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_CanonicalDeploymentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_CanonicalDeploymentRef {
        DataPagesProjects_Result_CanonicalDeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_CanonicalDeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aliases` after provisioning.\nA list of alias URLs pointing to this deployment."]
    pub fn aliases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aliases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `build_config` after provisioning.\nConfigs for the project build process."]
    pub fn build_config(&self) -> DataPagesProjects_Result_BuildConfigRef {
        DataPagesProjects_Result_BuildConfigRef::new(self.shared().clone(), format!("{}.build_config", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the deployment was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_trigger` after provisioning.\nInfo about what caused the deployment."]
    pub fn deployment_trigger(&self) -> DataPagesProjects_Result_LatestDeployment_DeploymentTriggerRef {
        DataPagesProjects_Result_LatestDeployment_DeploymentTriggerRef::new(
            self.shared().clone(),
            format!("{}.deployment_trigger", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `env_vars` after provisioning.\nEnvironment variables used for builds and Pages Functions."]
    pub fn env_vars(&self) -> terrars::RecRef<DataPagesProjects_Result_LatestDeployment_EnvVarsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_vars", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nType of deploy.\nAvailable values: \"preview\", \"production\"."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nId of the deployment."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_skipped` after provisioning.\nWhether the deployment was skipped."]
    pub fn is_skipped(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_skipped", self.base))
    }

    #[doc = "Get a reference to the value of field `latest_stage` after provisioning.\nThe status of the deployment."]
    pub fn latest_stage(&self) -> DataPagesProjects_Result_LatestDeployment_LatestStageRef {
        DataPagesProjects_Result_LatestDeployment_LatestStageRef::new(
            self.shared().clone(),
            format!("{}.latest_stage", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the deployment was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nId of the project."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_name` after provisioning.\nName of the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `short_id` after provisioning.\nShort Id (8 character) of the deployment."]
    pub fn short_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `skip_reason` after provisioning.\nWhy the deployment was skipped.\nAvailable values: \"commit_message\", \"preview_deployments_disabled\", \"production_deployments_disabled\", \"path_config\", \"branch_config\", \"pages_to_workers_conversion\", \"superseded_queued_build\"."]
    pub fn skip_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_reason", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nConfigs for the project source control."]
    pub fn source(&self) -> DataPagesProjects_Result_SourceRef {
        DataPagesProjects_Result_SourceRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `stages` after provisioning.\nList of past stages."]
    pub fn stages(&self) -> terrars::ListRef<DataPagesProjects_Result_LatestDeployment_StagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stages", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe live URL to view this deployment."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uses_functions` after provisioning.\nWhether the deployment uses functions."]
    pub fn uses_functions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uses_functions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindings {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindings { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindings {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_AiBindings {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_AiBindings {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindings {
        DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindings {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindingsRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasets {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasets { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasets {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasets {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasets {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasets {
        DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasets {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nName of the dataset."]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_Browsers {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_Browsers { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_Browsers {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_Browsers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_Browsers {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_Browsers {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_Browsers {
        DataPagesProjects_Result_DeploymentConfigs_Preview_Browsers {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_BrowsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_BrowsersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_BrowsersRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_BrowsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_BrowsersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_D1Databases {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_D1Databases { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_D1Databases {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_D1Databases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_D1Databases {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_D1Databases {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_D1Databases {
        DataPagesProjects_Result_DeploymentConfigs_Preview_D1Databases {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_D1DatabasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_D1DatabasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_D1DatabasesRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_D1DatabasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_D1DatabasesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the D1 database."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespaces {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespaces { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespaces {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespaces {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespaces {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespaces {
        DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespaces {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespacesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespacesRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `namespace_id` after provisioning.\nID of the Durable Object namespace."]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindings {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindings { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindings {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindings {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindings {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindings {
        DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindings {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindingsRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespaces {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespaces { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespaces {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespaces {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespaces {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespaces {
        DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespaces {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespacesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespacesRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\nID of the KV namespace."]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_Limits {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_Limits { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_Limits {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_Limits {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_Limits {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_Limits {
        DataPagesProjects_Result_DeploymentConfigs_Preview_Limits {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_LimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_LimitsRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_ms` after provisioning.\nCPU time limit in milliseconds."]
    pub fn cpu_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_ms", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificates {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificates { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificates {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificates {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificates {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificates {
        DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificates {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificatesRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_id` after provisioning.\n"]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_Placement {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_Placement { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_Placement {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_Placement {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_Placement {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_Placement {
        DataPagesProjects_Result_DeploymentConfigs_Preview_Placement {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_PlacementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_PlacementRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_PlacementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nPlacement mode."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducers {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducers { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducers {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducers {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducers {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducers {
        DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducers {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducersRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Queue."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_R2Buckets {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_R2Buckets { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_R2Buckets {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_R2Buckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_R2Buckets {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_R2Buckets {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_R2Buckets {
        DataPagesProjects_Result_DeploymentConfigs_Preview_R2Buckets {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_R2BucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_R2BucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_R2BucketsRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_R2BucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_R2BucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `jurisdiction` after provisioning.\nJurisdiction of the R2 bucket."]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the R2 bucket."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_Services {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_Services { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_Services {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_Services>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_Services {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_Services {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_Services {
        DataPagesProjects_Result_DeploymentConfigs_Preview_Services {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_ServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_ServicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_ServicesRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_ServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_ServicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\nThe entrypoint to bind to."]
    pub fn entrypoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entrypoint", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe Service environment."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe Service name."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindings {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindings { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindings {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindings {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindings {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindings {
        DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindings {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindingsRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Preview {}

impl DataPagesProjects_Result_DeploymentConfigs_Preview { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Preview {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Preview>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Preview {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Preview {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Preview {
        DataPagesProjects_Result_DeploymentConfigs_Preview {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_PreviewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_PreviewRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_PreviewRef {
        DataPagesProjects_Result_DeploymentConfigs_PreviewRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_PreviewRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ai_bindings` after provisioning.\nConstellation bindings used for Pages Functions."]
    pub fn ai_bindings(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindingsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ai_bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `always_use_latest_compatibility_date` after provisioning.\nWhether to always use the latest compatibility date for Pages Functions."]
    pub fn always_use_latest_compatibility_date(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.always_use_latest_compatibility_date", self.base))
    }

    #[doc =
        "Get a reference to the value of field `analytics_engine_datasets` after provisioning.\nAnalytics Engine bindings used for Pages Functions."]
    pub fn analytics_engine_datasets(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.analytics_engine_datasets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `browsers` after provisioning.\nBrowser bindings used for Pages Functions."]
    pub fn browsers(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_BrowsersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.browsers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `build_image_major_version` after provisioning.\nThe major version of the build image to use for Pages Functions. Version 1 is retired and its projects are moved to version 3. Once a project's builds have been moved, this is the version the builds run on and `build_image_auto_upgrade` is set on the project."]
    pub fn build_image_major_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_image_major_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_date` after provisioning.\nCompatibility date used for Pages Functions."]
    pub fn compatibility_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_flags` after provisioning.\nCompatibility flags used for Pages Functions."]
    pub fn compatibility_flags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `d1_databases` after provisioning.\nD1 databases used for Pages Functions."]
    pub fn d1_databases(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_D1DatabasesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.d1_databases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `durable_object_namespaces` after provisioning.\nDurable Object namespaces used for Pages Functions."]
    pub fn durable_object_namespaces(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespacesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.durable_object_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env_vars` after provisioning.\nEnvironment variables used for builds and Pages Functions."]
    pub fn env_vars(&self) -> terrars::RecRef<DataPagesProjects_Result_LatestDeployment_EnvVarsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_vars", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fail_open` after provisioning.\nWhether to fail open when the deployment config cannot be applied."]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hyperdrive_bindings` after provisioning.\nHyperdrive bindings used for Pages Functions."]
    pub fn hyperdrive_bindings(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindingsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.hyperdrive_bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kv_namespaces` after provisioning.\nKV namespaces used for Pages Functions."]
    pub fn kv_namespaces(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespacesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.kv_namespaces", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\nLimits for Pages Functions."]
    pub fn limits(&self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_LimitsRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_LimitsRef::new(
            self.shared().clone(),
            format!("{}.limits", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `mtls_certificates` after provisioning.\nmTLS bindings used for Pages Functions."]
    pub fn mtls_certificates(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificatesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.mtls_certificates", self.base))
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nPlacement setting used for Pages Functions."]
    pub fn placement(&self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_PlacementRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_PlacementRef::new(
            self.shared().clone(),
            format!("{}.placement", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `queue_producers` after provisioning.\nQueue Producer bindings used for Pages Functions."]
    pub fn queue_producers(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.queue_producers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `r2_buckets` after provisioning.\nR2 buckets used for Pages Functions."]
    pub fn r2_buckets(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_R2BucketsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.r2_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `services` after provisioning.\nServices used for Pages Functions."]
    pub fn services(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_ServicesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.services", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nThe usage model for Pages Functions.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vectorize_bindings` after provisioning.\nVectorize bindings used for Pages Functions."]
    pub fn vectorize_bindings(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindingsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.vectorize_bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wrangler_config_hash` after provisioning.\nHash of the Wrangler configuration used for the deployment."]
    pub fn wrangler_config_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wrangler_config_hash", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs_Production {}

impl DataPagesProjects_Result_DeploymentConfigs_Production { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs_Production {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs_Production>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs_Production {}

impl BuildDataPagesProjects_Result_DeploymentConfigs_Production {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs_Production {
        DataPagesProjects_Result_DeploymentConfigs_Production {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigs_ProductionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigs_ProductionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPagesProjects_Result_DeploymentConfigs_ProductionRef {
        DataPagesProjects_Result_DeploymentConfigs_ProductionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigs_ProductionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ai_bindings` after provisioning.\nConstellation bindings used for Pages Functions."]
    pub fn ai_bindings(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_AiBindingsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ai_bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `always_use_latest_compatibility_date` after provisioning.\nWhether to always use the latest compatibility date for Pages Functions."]
    pub fn always_use_latest_compatibility_date(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.always_use_latest_compatibility_date", self.base))
    }

    #[doc =
        "Get a reference to the value of field `analytics_engine_datasets` after provisioning.\nAnalytics Engine bindings used for Pages Functions."]
    pub fn analytics_engine_datasets(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.analytics_engine_datasets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `browsers` after provisioning.\nBrowser bindings used for Pages Functions."]
    pub fn browsers(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_BrowsersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.browsers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `build_image_major_version` after provisioning.\nThe major version of the build image to use for Pages Functions. Version 1 is retired and its projects are moved to version 3. Once a project's builds have been moved, this is the version the builds run on and `build_image_auto_upgrade` is set on the project."]
    pub fn build_image_major_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_image_major_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_date` after provisioning.\nCompatibility date used for Pages Functions."]
    pub fn compatibility_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_flags` after provisioning.\nCompatibility flags used for Pages Functions."]
    pub fn compatibility_flags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `d1_databases` after provisioning.\nD1 databases used for Pages Functions."]
    pub fn d1_databases(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_D1DatabasesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.d1_databases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `durable_object_namespaces` after provisioning.\nDurable Object namespaces used for Pages Functions."]
    pub fn durable_object_namespaces(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_DurableObjectNamespacesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.durable_object_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env_vars` after provisioning.\nEnvironment variables used for builds and Pages Functions."]
    pub fn env_vars(&self) -> terrars::RecRef<DataPagesProjects_Result_LatestDeployment_EnvVarsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_vars", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fail_open` after provisioning.\nWhether to fail open when the deployment config cannot be applied."]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hyperdrive_bindings` after provisioning.\nHyperdrive bindings used for Pages Functions."]
    pub fn hyperdrive_bindings(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_HyperdriveBindingsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.hyperdrive_bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kv_namespaces` after provisioning.\nKV namespaces used for Pages Functions."]
    pub fn kv_namespaces(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_KvNamespacesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.kv_namespaces", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\nLimits for Pages Functions."]
    pub fn limits(&self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_LimitsRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_LimitsRef::new(
            self.shared().clone(),
            format!("{}.limits", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `mtls_certificates` after provisioning.\nmTLS bindings used for Pages Functions."]
    pub fn mtls_certificates(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_MtlsCertificatesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.mtls_certificates", self.base))
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nPlacement setting used for Pages Functions."]
    pub fn placement(&self) -> DataPagesProjects_Result_DeploymentConfigs_Preview_PlacementRef {
        DataPagesProjects_Result_DeploymentConfigs_Preview_PlacementRef::new(
            self.shared().clone(),
            format!("{}.placement", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `queue_producers` after provisioning.\nQueue Producer bindings used for Pages Functions."]
    pub fn queue_producers(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_QueueProducersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.queue_producers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `r2_buckets` after provisioning.\nR2 buckets used for Pages Functions."]
    pub fn r2_buckets(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_R2BucketsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.r2_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `services` after provisioning.\nServices used for Pages Functions."]
    pub fn services(&self) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_ServicesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.services", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nThe usage model for Pages Functions.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vectorize_bindings` after provisioning.\nVectorize bindings used for Pages Functions."]
    pub fn vectorize_bindings(
        &self,
    ) -> terrars::RecRef<DataPagesProjects_Result_DeploymentConfigs_Preview_VectorizeBindingsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.vectorize_bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wrangler_config_hash` after provisioning.\nHash of the Wrangler configuration used for the deployment."]
    pub fn wrangler_config_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wrangler_config_hash", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_DeploymentConfigs {}

impl DataPagesProjects_Result_DeploymentConfigs { }

impl terrars::ToListMappable for DataPagesProjects_Result_DeploymentConfigs {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_DeploymentConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_DeploymentConfigs {}

impl BuildDataPagesProjects_Result_DeploymentConfigs {
    pub fn build(self) -> DataPagesProjects_Result_DeploymentConfigs {
        DataPagesProjects_Result_DeploymentConfigs {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_DeploymentConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_DeploymentConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPagesProjects_Result_DeploymentConfigsRef {
        DataPagesProjects_Result_DeploymentConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_DeploymentConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `preview` after provisioning.\nConfigs for preview deploys."]
    pub fn preview(&self) -> DataPagesProjects_Result_DeploymentConfigs_PreviewRef {
        DataPagesProjects_Result_DeploymentConfigs_PreviewRef::new(
            self.shared().clone(),
            format!("{}.preview", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `production` after provisioning.\nConfigs for production deploys."]
    pub fn production(&self) -> DataPagesProjects_Result_DeploymentConfigs_ProductionRef {
        DataPagesProjects_Result_DeploymentConfigs_ProductionRef::new(
            self.shared().clone(),
            format!("{}.production", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result_LatestDeployment {}

impl DataPagesProjects_Result_LatestDeployment { }

impl terrars::ToListMappable for DataPagesProjects_Result_LatestDeployment {
    type O = terrars::BlockAssignable<DataPagesProjects_Result_LatestDeployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result_LatestDeployment {}

impl BuildDataPagesProjects_Result_LatestDeployment {
    pub fn build(self) -> DataPagesProjects_Result_LatestDeployment {
        DataPagesProjects_Result_LatestDeployment {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_Result_LatestDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_Result_LatestDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPagesProjects_Result_LatestDeploymentRef {
        DataPagesProjects_Result_LatestDeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_Result_LatestDeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aliases` after provisioning.\nA list of alias URLs pointing to this deployment."]
    pub fn aliases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aliases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `build_config` after provisioning.\nConfigs for the project build process."]
    pub fn build_config(&self) -> DataPagesProjects_Result_BuildConfigRef {
        DataPagesProjects_Result_BuildConfigRef::new(self.shared().clone(), format!("{}.build_config", self.base))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the deployment was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_trigger` after provisioning.\nInfo about what caused the deployment."]
    pub fn deployment_trigger(&self) -> DataPagesProjects_Result_LatestDeployment_DeploymentTriggerRef {
        DataPagesProjects_Result_LatestDeployment_DeploymentTriggerRef::new(
            self.shared().clone(),
            format!("{}.deployment_trigger", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `env_vars` after provisioning.\nEnvironment variables used for builds and Pages Functions."]
    pub fn env_vars(&self) -> terrars::RecRef<DataPagesProjects_Result_LatestDeployment_EnvVarsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_vars", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nType of deploy.\nAvailable values: \"preview\", \"production\"."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nId of the deployment."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_skipped` after provisioning.\nWhether the deployment was skipped."]
    pub fn is_skipped(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_skipped", self.base))
    }

    #[doc = "Get a reference to the value of field `latest_stage` after provisioning.\nThe status of the deployment."]
    pub fn latest_stage(&self) -> DataPagesProjects_Result_LatestDeployment_LatestStageRef {
        DataPagesProjects_Result_LatestDeployment_LatestStageRef::new(
            self.shared().clone(),
            format!("{}.latest_stage", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the deployment was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nId of the project."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_name` after provisioning.\nName of the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `short_id` after provisioning.\nShort Id (8 character) of the deployment."]
    pub fn short_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `skip_reason` after provisioning.\nWhy the deployment was skipped.\nAvailable values: \"commit_message\", \"preview_deployments_disabled\", \"production_deployments_disabled\", \"path_config\", \"branch_config\", \"pages_to_workers_conversion\", \"superseded_queued_build\"."]
    pub fn skip_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_reason", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nConfigs for the project source control."]
    pub fn source(&self) -> DataPagesProjects_Result_SourceRef {
        DataPagesProjects_Result_SourceRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `stages` after provisioning.\nList of past stages."]
    pub fn stages(&self) -> terrars::ListRef<DataPagesProjects_Result_LatestDeployment_StagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stages", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe live URL to view this deployment."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uses_functions` after provisioning.\nWhether the deployment uses functions."]
    pub fn uses_functions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uses_functions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesProjects_Result {}

impl DataPagesProjects_Result { }

impl terrars::ToListMappable for DataPagesProjects_Result {
    type O = terrars::BlockAssignable<DataPagesProjects_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesProjects_Result {}

impl BuildDataPagesProjects_Result {
    pub fn build(self) -> DataPagesProjects_Result {
        DataPagesProjects_Result {}
    }
}

#[allow(dead_code)]
pub struct DataPagesProjects_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesProjects_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPagesProjects_ResultRef {
        DataPagesProjects_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesProjects_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `build_config` after provisioning.\nConfigs for the project build process."]
    pub fn build_config(&self) -> DataPagesProjects_Result_BuildConfigRef {
        DataPagesProjects_Result_BuildConfigRef::new(self.shared().clone(), format!("{}.build_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `build_image_auto_upgrade` after provisioning.\nPresent when the project is set to a retired build image major version and its builds have been moved to a newer one. `build_image_major_version` in `deployment_configs` then reports the version the builds run on."]
    pub fn build_image_auto_upgrade(&self) -> DataPagesProjects_Result_BuildImageAutoUpgradeRef {
        DataPagesProjects_Result_BuildImageAutoUpgradeRef::new(
            self.shared().clone(),
            format!("{}.build_image_auto_upgrade", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `canonical_deployment` after provisioning.\nMost recent production deployment of the project."]
    pub fn canonical_deployment(&self) -> DataPagesProjects_Result_CanonicalDeploymentRef {
        DataPagesProjects_Result_CanonicalDeploymentRef::new(
            self.shared().clone(),
            format!("{}.canonical_deployment", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the project was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_configs` after provisioning.\nConfigs for deployments in a project."]
    pub fn deployment_configs(&self) -> DataPagesProjects_Result_DeploymentConfigsRef {
        DataPagesProjects_Result_DeploymentConfigsRef::new(
            self.shared().clone(),
            format!("{}.deployment_configs", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nA list of associated custom domains for the project."]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\nFramework the project is using."]
    pub fn framework(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework", self.base))
    }

    #[doc =
        "Get a reference to the value of field `framework_version` after provisioning.\nVersion of the framework the project is using."]
    pub fn framework_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework_version", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the project."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `latest_deployment` after provisioning.\nMost recent deployment of the project."]
    pub fn latest_deployment(&self) -> DataPagesProjects_Result_LatestDeploymentRef {
        DataPagesProjects_Result_LatestDeploymentRef::new(
            self.shared().clone(),
            format!("{}.latest_deployment", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview_script_name` after provisioning.\nName of the preview script."]
    pub fn preview_script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview_script_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `production_branch` after provisioning.\nProduction branch of the project. Used to identify production deployments."]
    pub fn production_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `production_script_name` after provisioning.\nName of the production script."]
    pub fn production_script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_script_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nConfigs for the project source control."]
    pub fn source(&self) -> DataPagesProjects_Result_SourceRef {
        DataPagesProjects_Result_SourceRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subdomain` after provisioning.\nThe Cloudflare subdomain associated with the project."]
    pub fn subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uses_functions` after provisioning.\nWhether the project uses functions."]
    pub fn uses_functions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uses_functions", self.base))
    }
}
