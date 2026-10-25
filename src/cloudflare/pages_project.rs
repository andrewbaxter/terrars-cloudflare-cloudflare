#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct PagesProjectData_ {
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
    build_config: std::option::Option<PagesProject_BuildConfig>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_configs: std::option::Option<PagesProject_DeploymentConfigs>,
    name: terrars::PrimField<std::string::String>,
    production_branch: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<PagesProject_Source>,
}

struct PagesProject_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PagesProjectData_>,
}

#[derive(Clone)]
pub struct PagesProject(std::rc::Rc<PagesProject_>);

impl PagesProject {
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

    #[doc = "Set the field `build_config`.\nConfigs for the project build process."]
    pub fn set_build_config(self, v: impl std::convert::Into<PagesProject_BuildConfig>) -> Self {
        self.0.data.borrow_mut().build_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_configs`.\nConfigs for deployments in a project."]
    pub fn set_deployment_configs(self, v: impl std::convert::Into<PagesProject_DeploymentConfigs>) -> Self {
        self.0.data.borrow_mut().deployment_configs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\nConfigs for the project source control."]
    pub fn set_source(self, v: impl std::convert::Into<PagesProject_Source>) -> Self {
        self.0.data.borrow_mut().source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_config` after provisioning.\nConfigs for the project build process."]
    pub fn build_config(&self) -> PagesProject_BuildConfigRef {
        PagesProject_BuildConfigRef::new(self.shared().clone(), format!("{}.build_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_image_auto_upgrade` after provisioning.\nPresent when the project is set to a retired build image major version and its builds have been moved to a newer one. `build_image_major_version` in `deployment_configs` then reports the version the builds run on."]
    pub fn build_image_auto_upgrade(&self) -> PagesProject_BuildImageAutoUpgradeRef {
        PagesProject_BuildImageAutoUpgradeRef::new(
            self.shared().clone(),
            format!("{}.build_image_auto_upgrade", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `canonical_deployment` after provisioning.\nMost recent production deployment of the project."]
    pub fn canonical_deployment(&self) -> PagesProject_CanonicalDeploymentRef {
        PagesProject_CanonicalDeploymentRef::new(
            self.shared().clone(),
            format!("{}.canonical_deployment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the project was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_configs` after provisioning.\nConfigs for deployments in a project."]
    pub fn deployment_configs(&self) -> PagesProject_DeploymentConfigsRef {
        PagesProject_DeploymentConfigsRef::new(
            self.shared().clone(),
            format!("{}.deployment_configs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nA list of associated custom domains for the project."]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\nFramework the project is using."]
    pub fn framework(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `framework_version` after provisioning.\nVersion of the framework the project is using."]
    pub fn framework_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nName for the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_deployment` after provisioning.\nMost recent deployment of the project."]
    pub fn latest_deployment(&self) -> PagesProject_LatestDeploymentRef {
        PagesProject_LatestDeploymentRef::new(
            self.shared().clone(),
            format!("{}.latest_deployment", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName for the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preview_script_name` after provisioning.\nName of the preview script."]
    pub fn preview_script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview_script_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `production_branch` after provisioning.\nProduction branch of the project. Used to identify production deployments."]
    pub fn production_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_branch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `production_script_name` after provisioning.\nName of the production script."]
    pub fn production_script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_script_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nConfigs for the project source control."]
    pub fn source(&self) -> PagesProject_SourceRef {
        PagesProject_SourceRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subdomain` after provisioning.\nThe Cloudflare subdomain associated with the project."]
    pub fn subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uses_functions` after provisioning.\nWhether the project uses functions."]
    pub fn uses_functions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uses_functions", self.extract_ref()))
    }
}

impl terrars::Referable for PagesProject {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PagesProject { }

impl terrars::ToListMappable for PagesProject {
    type O = terrars::ListRef<PagesProjectRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PagesProject_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_pages_project".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPagesProject {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Name for the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Production branch of the project. Used to identify production deployments."]
    pub production_branch: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject {
    pub fn build(self, stack: &mut terrars::Stack) -> PagesProject {
        let out = PagesProject(std::rc::Rc::new(PagesProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PagesProjectData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                build_config: core::default::Default::default(),
                deployment_configs: core::default::Default::default(),
                name: self.name,
                production_branch: self.production_branch,
                source: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PagesProjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PagesProjectRef {
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

    #[doc =
        "Get a reference to the value of field `build_config` after provisioning.\nConfigs for the project build process."]
    pub fn build_config(&self) -> PagesProject_BuildConfigRef {
        PagesProject_BuildConfigRef::new(self.shared().clone(), format!("{}.build_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_image_auto_upgrade` after provisioning.\nPresent when the project is set to a retired build image major version and its builds have been moved to a newer one. `build_image_major_version` in `deployment_configs` then reports the version the builds run on."]
    pub fn build_image_auto_upgrade(&self) -> PagesProject_BuildImageAutoUpgradeRef {
        PagesProject_BuildImageAutoUpgradeRef::new(
            self.shared().clone(),
            format!("{}.build_image_auto_upgrade", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `canonical_deployment` after provisioning.\nMost recent production deployment of the project."]
    pub fn canonical_deployment(&self) -> PagesProject_CanonicalDeploymentRef {
        PagesProject_CanonicalDeploymentRef::new(
            self.shared().clone(),
            format!("{}.canonical_deployment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the project was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_configs` after provisioning.\nConfigs for deployments in a project."]
    pub fn deployment_configs(&self) -> PagesProject_DeploymentConfigsRef {
        PagesProject_DeploymentConfigsRef::new(
            self.shared().clone(),
            format!("{}.deployment_configs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nA list of associated custom domains for the project."]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\nFramework the project is using."]
    pub fn framework(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `framework_version` after provisioning.\nVersion of the framework the project is using."]
    pub fn framework_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nName for the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_deployment` after provisioning.\nMost recent deployment of the project."]
    pub fn latest_deployment(&self) -> PagesProject_LatestDeploymentRef {
        PagesProject_LatestDeploymentRef::new(
            self.shared().clone(),
            format!("{}.latest_deployment", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName for the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preview_script_name` after provisioning.\nName of the preview script."]
    pub fn preview_script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview_script_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `production_branch` after provisioning.\nProduction branch of the project. Used to identify production deployments."]
    pub fn production_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_branch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `production_script_name` after provisioning.\nName of the production script."]
    pub fn production_script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_script_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nConfigs for the project source control."]
    pub fn source(&self) -> PagesProject_SourceRef {
        PagesProject_SourceRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subdomain` after provisioning.\nThe Cloudflare subdomain associated with the project."]
    pub fn subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uses_functions` after provisioning.\nWhether the project uses functions."]
    pub fn uses_functions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uses_functions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PagesProject_BuildConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_caching: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_analytics_tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_analytics_token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PagesProject_BuildConfig {
    #[doc = "Set the field `build_caching`.\nEnable build caching for the project."]
    pub fn set_build_caching(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.build_caching = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_command`.\nCommand used to build project."]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_dir`.\nOutput directory of the build."]
    pub fn set_destination_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_dir`.\nDirectory to run the command."]
    pub fn set_root_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_analytics_tag`.\nThe classifying tag for analytics."]
    pub fn set_web_analytics_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.web_analytics_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_analytics_token`.\nThe auth token for analytics."]
    pub fn set_web_analytics_token(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.web_analytics_token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProject_BuildConfig {
    type O = terrars::BlockAssignable<PagesProject_BuildConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_BuildConfig {}

impl BuildPagesProject_BuildConfig {
    pub fn build(self) -> PagesProject_BuildConfig {
        PagesProject_BuildConfig {
            build_caching: core::default::Default::default(),
            build_command: core::default::Default::default(),
            destination_dir: core::default::Default::default(),
            root_dir: core::default::Default::default(),
            web_analytics_tag: core::default::Default::default(),
            web_analytics_token: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PagesProject_BuildConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_BuildConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_BuildConfigRef {
        PagesProject_BuildConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_BuildConfigRef {
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
        "Get a reference to the value of field `destination_dir` after provisioning.\nOutput directory of the build."]
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
pub struct PagesProject_BuildImageAutoUpgrade {}

impl PagesProject_BuildImageAutoUpgrade { }

impl terrars::ToListMappable for PagesProject_BuildImageAutoUpgrade {
    type O = terrars::BlockAssignable<PagesProject_BuildImageAutoUpgrade>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_BuildImageAutoUpgrade {}

impl BuildPagesProject_BuildImageAutoUpgrade {
    pub fn build(self) -> PagesProject_BuildImageAutoUpgrade {
        PagesProject_BuildImageAutoUpgrade {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_BuildImageAutoUpgradeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_BuildImageAutoUpgradeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_BuildImageAutoUpgradeRef {
        PagesProject_BuildImageAutoUpgradeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_BuildImageAutoUpgradeRef {
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
pub struct PagesProject_LatestDeployment_BuildConfig {}

impl PagesProject_LatestDeployment_BuildConfig { }

impl terrars::ToListMappable for PagesProject_LatestDeployment_BuildConfig {
    type O = terrars::BlockAssignable<PagesProject_LatestDeployment_BuildConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_LatestDeployment_BuildConfig {}

impl BuildPagesProject_LatestDeployment_BuildConfig {
    pub fn build(self) -> PagesProject_LatestDeployment_BuildConfig {
        PagesProject_LatestDeployment_BuildConfig {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_LatestDeployment_BuildConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_LatestDeployment_BuildConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_LatestDeployment_BuildConfigRef {
        PagesProject_LatestDeployment_BuildConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_LatestDeployment_BuildConfigRef {
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
pub struct PagesProject_LatestDeployment_DeploymentTrigger_Metadata {}

impl PagesProject_LatestDeployment_DeploymentTrigger_Metadata { }

impl terrars::ToListMappable for PagesProject_LatestDeployment_DeploymentTrigger_Metadata {
    type O = terrars::BlockAssignable<PagesProject_LatestDeployment_DeploymentTrigger_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_LatestDeployment_DeploymentTrigger_Metadata {}

impl BuildPagesProject_LatestDeployment_DeploymentTrigger_Metadata {
    pub fn build(self) -> PagesProject_LatestDeployment_DeploymentTrigger_Metadata {
        PagesProject_LatestDeployment_DeploymentTrigger_Metadata {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_LatestDeployment_DeploymentTrigger_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_LatestDeployment_DeploymentTrigger_MetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_LatestDeployment_DeploymentTrigger_MetadataRef {
        PagesProject_LatestDeployment_DeploymentTrigger_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_LatestDeployment_DeploymentTrigger_MetadataRef {
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
pub struct PagesProject_LatestDeployment_DeploymentTrigger {}

impl PagesProject_LatestDeployment_DeploymentTrigger { }

impl terrars::ToListMappable for PagesProject_LatestDeployment_DeploymentTrigger {
    type O = terrars::BlockAssignable<PagesProject_LatestDeployment_DeploymentTrigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_LatestDeployment_DeploymentTrigger {}

impl BuildPagesProject_LatestDeployment_DeploymentTrigger {
    pub fn build(self) -> PagesProject_LatestDeployment_DeploymentTrigger {
        PagesProject_LatestDeployment_DeploymentTrigger {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_LatestDeployment_DeploymentTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_LatestDeployment_DeploymentTriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_LatestDeployment_DeploymentTriggerRef {
        PagesProject_LatestDeployment_DeploymentTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_LatestDeployment_DeploymentTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nAdditional info about the trigger."]
    pub fn metadata(&self) -> PagesProject_LatestDeployment_DeploymentTrigger_MetadataRef {
        PagesProject_LatestDeployment_DeploymentTrigger_MetadataRef::new(
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
pub struct PagesProject_LatestDeployment_EnvVars {}

impl PagesProject_LatestDeployment_EnvVars { }

impl terrars::ToListMappable for PagesProject_LatestDeployment_EnvVars {
    type O = terrars::BlockAssignable<PagesProject_LatestDeployment_EnvVars>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_LatestDeployment_EnvVars {}

impl BuildPagesProject_LatestDeployment_EnvVars {
    pub fn build(self) -> PagesProject_LatestDeployment_EnvVars {
        PagesProject_LatestDeployment_EnvVars {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_LatestDeployment_EnvVarsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_LatestDeployment_EnvVarsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_LatestDeployment_EnvVarsRef {
        PagesProject_LatestDeployment_EnvVarsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_LatestDeployment_EnvVarsRef {
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
pub struct PagesProject_LatestDeployment_LatestStage {}

impl PagesProject_LatestDeployment_LatestStage { }

impl terrars::ToListMappable for PagesProject_LatestDeployment_LatestStage {
    type O = terrars::BlockAssignable<PagesProject_LatestDeployment_LatestStage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_LatestDeployment_LatestStage {}

impl BuildPagesProject_LatestDeployment_LatestStage {
    pub fn build(self) -> PagesProject_LatestDeployment_LatestStage {
        PagesProject_LatestDeployment_LatestStage {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_LatestDeployment_LatestStageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_LatestDeployment_LatestStageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_LatestDeployment_LatestStageRef {
        PagesProject_LatestDeployment_LatestStageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_LatestDeployment_LatestStageRef {
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
pub struct PagesProject_LatestDeployment_Source_Config {}

impl PagesProject_LatestDeployment_Source_Config { }

impl terrars::ToListMappable for PagesProject_LatestDeployment_Source_Config {
    type O = terrars::BlockAssignable<PagesProject_LatestDeployment_Source_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_LatestDeployment_Source_Config {}

impl BuildPagesProject_LatestDeployment_Source_Config {
    pub fn build(self) -> PagesProject_LatestDeployment_Source_Config {
        PagesProject_LatestDeployment_Source_Config {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_LatestDeployment_Source_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_LatestDeployment_Source_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_LatestDeployment_Source_ConfigRef {
        PagesProject_LatestDeployment_Source_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_LatestDeployment_Source_ConfigRef {
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
pub struct PagesProject_LatestDeployment_Source {}

impl PagesProject_LatestDeployment_Source { }

impl terrars::ToListMappable for PagesProject_LatestDeployment_Source {
    type O = terrars::BlockAssignable<PagesProject_LatestDeployment_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_LatestDeployment_Source {}

impl BuildPagesProject_LatestDeployment_Source {
    pub fn build(self) -> PagesProject_LatestDeployment_Source {
        PagesProject_LatestDeployment_Source {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_LatestDeployment_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_LatestDeployment_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_LatestDeployment_SourceRef {
        PagesProject_LatestDeployment_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_LatestDeployment_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> PagesProject_LatestDeployment_Source_ConfigRef {
        PagesProject_LatestDeployment_Source_ConfigRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe source control management provider.\nAvailable values: \"github\", \"gitlab\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct PagesProject_LatestDeployment_Stages {}

impl PagesProject_LatestDeployment_Stages { }

impl terrars::ToListMappable for PagesProject_LatestDeployment_Stages {
    type O = terrars::BlockAssignable<PagesProject_LatestDeployment_Stages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_LatestDeployment_Stages {}

impl BuildPagesProject_LatestDeployment_Stages {
    pub fn build(self) -> PagesProject_LatestDeployment_Stages {
        PagesProject_LatestDeployment_Stages {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_LatestDeployment_StagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_LatestDeployment_StagesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_LatestDeployment_StagesRef {
        PagesProject_LatestDeployment_StagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_LatestDeployment_StagesRef {
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
pub struct PagesProject_CanonicalDeployment {}

impl PagesProject_CanonicalDeployment { }

impl terrars::ToListMappable for PagesProject_CanonicalDeployment {
    type O = terrars::BlockAssignable<PagesProject_CanonicalDeployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_CanonicalDeployment {}

impl BuildPagesProject_CanonicalDeployment {
    pub fn build(self) -> PagesProject_CanonicalDeployment {
        PagesProject_CanonicalDeployment {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_CanonicalDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_CanonicalDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_CanonicalDeploymentRef {
        PagesProject_CanonicalDeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_CanonicalDeploymentRef {
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
    pub fn build_config(&self) -> PagesProject_LatestDeployment_BuildConfigRef {
        PagesProject_LatestDeployment_BuildConfigRef::new(
            self.shared().clone(),
            format!("{}.build_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the deployment was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_trigger` after provisioning.\nInfo about what caused the deployment."]
    pub fn deployment_trigger(&self) -> PagesProject_LatestDeployment_DeploymentTriggerRef {
        PagesProject_LatestDeployment_DeploymentTriggerRef::new(
            self.shared().clone(),
            format!("{}.deployment_trigger", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `env_vars` after provisioning.\nEnvironment variables used for builds and Pages Functions."]
    pub fn env_vars(&self) -> terrars::RecRef<PagesProject_LatestDeployment_EnvVarsRef> {
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
    pub fn latest_stage(&self) -> PagesProject_LatestDeployment_LatestStageRef {
        PagesProject_LatestDeployment_LatestStageRef::new(
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
        "Get a reference to the value of field `source` after provisioning.\nConfigs for the project source control."]
    pub fn source(&self) -> PagesProject_LatestDeployment_SourceRef {
        PagesProject_LatestDeployment_SourceRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `stages` after provisioning.\nList of past stages."]
    pub fn stages(&self) -> terrars::ListRef<PagesProject_LatestDeployment_StagesRef> {
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
pub struct PagesProject_DeploymentConfigs_Preview_AiBindings {
    project_id: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_AiBindings { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_AiBindings {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_AiBindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_AiBindings {
    #[doc = ""]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_AiBindings {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_AiBindings {
        PagesProject_DeploymentConfigs_Preview_AiBindings { project_id: self.project_id }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_AiBindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_AiBindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_AiBindingsRef {
        PagesProject_DeploymentConfigs_Preview_AiBindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_AiBindingsRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets {
    dataset: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets {
    #[doc = "Name of the dataset."]
    pub dataset: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets {
        PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets { dataset: self.dataset }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef {
        PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_Browsers {}

impl PagesProject_DeploymentConfigs_Preview_Browsers { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_Browsers {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_Browsers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_Browsers {}

impl BuildPagesProject_DeploymentConfigs_Preview_Browsers {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_Browsers {
        PagesProject_DeploymentConfigs_Preview_Browsers {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_BrowsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_BrowsersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_BrowsersRef {
        PagesProject_DeploymentConfigs_Preview_BrowsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_BrowsersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct PagesProject_DeploymentConfigs_Preview_D1Databases {
    id: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_D1Databases { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_D1Databases {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_D1Databases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_D1Databases {
    #[doc = "UUID of the D1 database."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_D1Databases {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_D1Databases {
        PagesProject_DeploymentConfigs_Preview_D1Databases { id: self.id }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_D1DatabasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_D1DatabasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_D1DatabasesRef {
        PagesProject_DeploymentConfigs_Preview_D1DatabasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_D1DatabasesRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces {
    namespace_id: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces {
    #[doc = "ID of the Durable Object namespace."]
    pub namespace_id: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces {
        PagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces { namespace_id: self.namespace_id }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_DurableObjectNamespacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_DurableObjectNamespacesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_DurableObjectNamespacesRef {
        PagesProject_DeploymentConfigs_Preview_DurableObjectNamespacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_DurableObjectNamespacesRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_EnvVars {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_EnvVars { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_EnvVars {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_EnvVars>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_EnvVars {
    #[doc = "Available values: \"plain_text\", \"secret_text\"."]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = "Environment variable value."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_EnvVars {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_EnvVars {
        PagesProject_DeploymentConfigs_Preview_EnvVars {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_EnvVarsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_EnvVarsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_EnvVarsRef {
        PagesProject_DeploymentConfigs_Preview_EnvVarsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_EnvVarsRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_HyperdriveBindings {
    id: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_HyperdriveBindings { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_HyperdriveBindings {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_HyperdriveBindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_HyperdriveBindings {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_HyperdriveBindings {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_HyperdriveBindings {
        PagesProject_DeploymentConfigs_Preview_HyperdriveBindings { id: self.id }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_HyperdriveBindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_HyperdriveBindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_HyperdriveBindingsRef {
        PagesProject_DeploymentConfigs_Preview_HyperdriveBindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_HyperdriveBindingsRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_KvNamespaces {
    namespace_id: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_KvNamespaces { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_KvNamespaces {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_KvNamespaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_KvNamespaces {
    #[doc = "ID of the KV namespace."]
    pub namespace_id: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_KvNamespaces {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_KvNamespaces {
        PagesProject_DeploymentConfigs_Preview_KvNamespaces { namespace_id: self.namespace_id }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_KvNamespacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_KvNamespacesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_KvNamespacesRef {
        PagesProject_DeploymentConfigs_Preview_KvNamespacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_KvNamespacesRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_Limits {
    cpu_ms: terrars::PrimField<f64>,
}

impl PagesProject_DeploymentConfigs_Preview_Limits { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_Limits {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_Limits {
    #[doc = "CPU time limit in milliseconds."]
    pub cpu_ms: terrars::PrimField<f64>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_Limits {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_Limits {
        PagesProject_DeploymentConfigs_Preview_Limits { cpu_ms: self.cpu_ms }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_LimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_LimitsRef {
        PagesProject_DeploymentConfigs_Preview_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_LimitsRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_MtlsCertificates {
    certificate_id: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_MtlsCertificates { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_MtlsCertificates {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_MtlsCertificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_MtlsCertificates {
    #[doc = ""]
    pub certificate_id: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_MtlsCertificates {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_MtlsCertificates {
        PagesProject_DeploymentConfigs_Preview_MtlsCertificates { certificate_id: self.certificate_id }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_MtlsCertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_MtlsCertificatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_MtlsCertificatesRef {
        PagesProject_DeploymentConfigs_Preview_MtlsCertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_MtlsCertificatesRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_Placement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PagesProject_DeploymentConfigs_Preview_Placement {
    #[doc = "Set the field `mode`.\nPlacement mode."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_Placement {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_Placement {}

impl BuildPagesProject_DeploymentConfigs_Preview_Placement {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_Placement {
        PagesProject_DeploymentConfigs_Preview_Placement { mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_PlacementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_PlacementRef {
        PagesProject_DeploymentConfigs_Preview_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_PlacementRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_QueueProducers {
    name: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_QueueProducers { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_QueueProducers {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_QueueProducers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_QueueProducers {
    #[doc = "Name of the Queue."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_QueueProducers {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_QueueProducers {
        PagesProject_DeploymentConfigs_Preview_QueueProducers { name: self.name }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_QueueProducersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_QueueProducersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_QueueProducersRef {
        PagesProject_DeploymentConfigs_Preview_QueueProducersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_QueueProducersRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_R2Buckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jurisdiction: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_R2Buckets {
    #[doc = "Set the field `jurisdiction`.\nJurisdiction of the R2 bucket."]
    pub fn set_jurisdiction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jurisdiction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_R2Buckets {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_R2Buckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_R2Buckets {
    #[doc = "Name of the R2 bucket."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_R2Buckets {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_R2Buckets {
        PagesProject_DeploymentConfigs_Preview_R2Buckets {
            jurisdiction: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_R2BucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_R2BucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_R2BucketsRef {
        PagesProject_DeploymentConfigs_Preview_R2BucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_R2BucketsRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_Services {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_Services {
    #[doc = "Set the field `entrypoint`.\nThe entrypoint to bind to."]
    pub fn set_entrypoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entrypoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\nThe Service environment."]
    pub fn set_environment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_Services {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_Services>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_Services {
    #[doc = "The Service name."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_Services {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_Services {
        PagesProject_DeploymentConfigs_Preview_Services {
            entrypoint: core::default::Default::default(),
            environment: core::default::Default::default(),
            service: self.service,
        }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_ServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_ServicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_ServicesRef {
        PagesProject_DeploymentConfigs_Preview_ServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_ServicesRef {
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
pub struct PagesProject_DeploymentConfigs_Preview_VectorizeBindings {
    index_name: terrars::PrimField<std::string::String>,
}

impl PagesProject_DeploymentConfigs_Preview_VectorizeBindings { }

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview_VectorizeBindings {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview_VectorizeBindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview_VectorizeBindings {
    #[doc = ""]
    pub index_name: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_DeploymentConfigs_Preview_VectorizeBindings {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview_VectorizeBindings {
        PagesProject_DeploymentConfigs_Preview_VectorizeBindings { index_name: self.index_name }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_Preview_VectorizeBindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_Preview_VectorizeBindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProject_DeploymentConfigs_Preview_VectorizeBindingsRef {
        PagesProject_DeploymentConfigs_Preview_VectorizeBindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_Preview_VectorizeBindingsRef {
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
pub struct PagesProject_DeploymentConfigs_Preview {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_bindings: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_AiBindings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    always_use_latest_compatibility_date: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analytics_engine_datasets: std
    ::option
    ::Option<
        std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browsers: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_Browsers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_image_major_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_flags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    d1_databases: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_D1Databases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    durable_object_namespaces: std
    ::option
    ::Option<
        std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env_vars: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_EnvVars>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hyperdrive_bindings: std
    ::option
    ::Option<
        std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_HyperdriveBindings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kv_namespaces: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_KvNamespaces>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<PagesProject_DeploymentConfigs_Preview_Limits>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtls_certificates: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_MtlsCertificates>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<PagesProject_DeploymentConfigs_Preview_Placement>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_producers: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_QueueProducers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r2_buckets: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_R2Buckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_Services>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vectorize_bindings: std
    ::option
    ::Option<
        std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_VectorizeBindings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wrangler_config_hash: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PagesProject_DeploymentConfigs_Preview {
    #[doc = "Set the field `ai_bindings`.\nConstellation bindings used for Pages Functions."]
    pub fn set_ai_bindings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_AiBindings>,
                    >,
    ) -> Self {
        self.ai_bindings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `always_use_latest_compatibility_date`.\nWhether to always use the latest compatibility date for Pages Functions."]
    pub fn set_always_use_latest_compatibility_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.always_use_latest_compatibility_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analytics_engine_datasets`.\nAnalytics Engine bindings used for Pages Functions."]
    pub fn set_analytics_engine_datasets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets>,
                    >,
    ) -> Self {
        self.analytics_engine_datasets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `browsers`.\nBrowser bindings used for Pages Functions."]
    pub fn set_browsers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_Browsers>,
                    >,
    ) -> Self {
        self.browsers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `build_image_major_version`.\nThe major version of the build image to use for Pages Functions. Version 1 is retired and its projects are moved to version 3. Once a project's builds have been moved, a request that sets version 1 is rejected."]
    pub fn set_build_image_major_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.build_image_major_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compatibility_date`.\nCompatibility date used for Pages Functions."]
    pub fn set_compatibility_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.compatibility_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compatibility_flags`.\nCompatibility flags used for Pages Functions."]
    pub fn set_compatibility_flags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.compatibility_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `d1_databases`.\nD1 databases used for Pages Functions."]
    pub fn set_d1_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_D1Databases>,
                    >,
    ) -> Self {
        self.d1_databases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `durable_object_namespaces`.\nDurable Object namespaces used for Pages Functions."]
    pub fn set_durable_object_namespaces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces>,
                    >,
    ) -> Self {
        self.durable_object_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env_vars`.\nEnvironment variables used for builds and Pages Functions."]
    pub fn set_env_vars(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_EnvVars>,
                    >,
    ) -> Self {
        self.env_vars = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_open`.\nWhether to fail open when the deployment config cannot be applied."]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hyperdrive_bindings`.\nHyperdrive bindings used for Pages Functions."]
    pub fn set_hyperdrive_bindings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_HyperdriveBindings>,
                    >,
    ) -> Self {
        self.hyperdrive_bindings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kv_namespaces`.\nKV namespaces used for Pages Functions."]
    pub fn set_kv_namespaces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_KvNamespaces>,
                    >,
    ) -> Self {
        self.kv_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limits`.\nLimits for Pages Functions."]
    pub fn set_limits(mut self, v: impl std::convert::Into<PagesProject_DeploymentConfigs_Preview_Limits>) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mtls_certificates`.\nmTLS bindings used for Pages Functions."]
    pub fn set_mtls_certificates(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_MtlsCertificates>,
                    >,
    ) -> Self {
        self.mtls_certificates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement`.\nPlacement setting used for Pages Functions."]
    pub fn set_placement(mut self, v: impl std::convert::Into<PagesProject_DeploymentConfigs_Preview_Placement>) -> Self {
        self.placement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_producers`.\nQueue Producer bindings used for Pages Functions."]
    pub fn set_queue_producers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_QueueProducers>,
                    >,
    ) -> Self {
        self.queue_producers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `r2_buckets`.\nR2 buckets used for Pages Functions."]
    pub fn set_r2_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_R2Buckets>,
                    >,
    ) -> Self {
        self.r2_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `services`.\nServices used for Pages Functions."]
    pub fn set_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_Services>,
                    >,
    ) -> Self {
        self.services = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `usage_model`.\nThe usage model for Pages Functions.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn set_usage_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.usage_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vectorize_bindings`.\nVectorize bindings used for Pages Functions."]
    pub fn set_vectorize_bindings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_VectorizeBindings>,
                    >,
    ) -> Self {
        self.vectorize_bindings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wrangler_config_hash`.\nHash of the Wrangler configuration used for the deployment."]
    pub fn set_wrangler_config_hash(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.wrangler_config_hash = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Preview {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Preview>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Preview {}

impl BuildPagesProject_DeploymentConfigs_Preview {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Preview {
        PagesProject_DeploymentConfigs_Preview {
            ai_bindings: core::default::Default::default(),
            always_use_latest_compatibility_date: core::default::Default::default(),
            analytics_engine_datasets: core::default::Default::default(),
            browsers: core::default::Default::default(),
            build_image_major_version: core::default::Default::default(),
            compatibility_date: core::default::Default::default(),
            compatibility_flags: core::default::Default::default(),
            d1_databases: core::default::Default::default(),
            durable_object_namespaces: core::default::Default::default(),
            env_vars: core::default::Default::default(),
            fail_open: core::default::Default::default(),
            hyperdrive_bindings: core::default::Default::default(),
            kv_namespaces: core::default::Default::default(),
            limits: core::default::Default::default(),
            mtls_certificates: core::default::Default::default(),
            placement: core::default::Default::default(),
            queue_producers: core::default::Default::default(),
            r2_buckets: core::default::Default::default(),
            services: core::default::Default::default(),
            usage_model: core::default::Default::default(),
            vectorize_bindings: core::default::Default::default(),
            wrangler_config_hash: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_PreviewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_PreviewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_DeploymentConfigs_PreviewRef {
        PagesProject_DeploymentConfigs_PreviewRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_PreviewRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ai_bindings` after provisioning.\nConstellation bindings used for Pages Functions."]
    pub fn ai_bindings(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_AiBindingsRef> {
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
    ) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.analytics_engine_datasets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `browsers` after provisioning.\nBrowser bindings used for Pages Functions."]
    pub fn browsers(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_BrowsersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.browsers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `build_image_major_version` after provisioning.\nThe major version of the build image to use for Pages Functions. Version 1 is retired and its projects are moved to version 3. Once a project's builds have been moved, a request that sets version 1 is rejected."]
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
    pub fn d1_databases(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_D1DatabasesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.d1_databases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `durable_object_namespaces` after provisioning.\nDurable Object namespaces used for Pages Functions."]
    pub fn durable_object_namespaces(
        &self,
    ) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_DurableObjectNamespacesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.durable_object_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env_vars` after provisioning.\nEnvironment variables used for builds and Pages Functions."]
    pub fn env_vars(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_EnvVarsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_vars", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fail_open` after provisioning.\nWhether to fail open when the deployment config cannot be applied."]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hyperdrive_bindings` after provisioning.\nHyperdrive bindings used for Pages Functions."]
    pub fn hyperdrive_bindings(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_HyperdriveBindingsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.hyperdrive_bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kv_namespaces` after provisioning.\nKV namespaces used for Pages Functions."]
    pub fn kv_namespaces(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_KvNamespacesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.kv_namespaces", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\nLimits for Pages Functions."]
    pub fn limits(&self) -> PagesProject_DeploymentConfigs_Preview_LimitsRef {
        PagesProject_DeploymentConfigs_Preview_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mtls_certificates` after provisioning.\nmTLS bindings used for Pages Functions."]
    pub fn mtls_certificates(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_MtlsCertificatesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.mtls_certificates", self.base))
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nPlacement setting used for Pages Functions."]
    pub fn placement(&self) -> PagesProject_DeploymentConfigs_Preview_PlacementRef {
        PagesProject_DeploymentConfigs_Preview_PlacementRef::new(
            self.shared().clone(),
            format!("{}.placement", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `queue_producers` after provisioning.\nQueue Producer bindings used for Pages Functions."]
    pub fn queue_producers(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_QueueProducersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.queue_producers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `r2_buckets` after provisioning.\nR2 buckets used for Pages Functions."]
    pub fn r2_buckets(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_R2BucketsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.r2_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `services` after provisioning.\nServices used for Pages Functions."]
    pub fn services(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_ServicesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.services", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nThe usage model for Pages Functions.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vectorize_bindings` after provisioning.\nVectorize bindings used for Pages Functions."]
    pub fn vectorize_bindings(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_VectorizeBindingsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.vectorize_bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wrangler_config_hash` after provisioning.\nHash of the Wrangler configuration used for the deployment."]
    pub fn wrangler_config_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wrangler_config_hash", self.base))
    }
}

#[derive(Serialize)]
pub struct PagesProject_DeploymentConfigs_Production {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_bindings: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_AiBindings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    always_use_latest_compatibility_date: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analytics_engine_datasets: std
    ::option
    ::Option<
        std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browsers: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_Browsers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_image_major_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_flags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    d1_databases: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_D1Databases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    durable_object_namespaces: std
    ::option
    ::Option<
        std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env_vars: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_EnvVars>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hyperdrive_bindings: std
    ::option
    ::Option<
        std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_HyperdriveBindings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kv_namespaces: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_KvNamespaces>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<PagesProject_DeploymentConfigs_Preview_Limits>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtls_certificates: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_MtlsCertificates>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<PagesProject_DeploymentConfigs_Preview_Placement>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_producers: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_QueueProducers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r2_buckets: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_R2Buckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services: std
    ::option
    ::Option<std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_Services>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vectorize_bindings: std
    ::option
    ::Option<
        std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_VectorizeBindings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wrangler_config_hash: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PagesProject_DeploymentConfigs_Production {
    #[doc = "Set the field `ai_bindings`.\nConstellation bindings used for Pages Functions."]
    pub fn set_ai_bindings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_AiBindings>,
                    >,
    ) -> Self {
        self.ai_bindings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `always_use_latest_compatibility_date`.\nWhether to always use the latest compatibility date for Pages Functions."]
    pub fn set_always_use_latest_compatibility_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.always_use_latest_compatibility_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analytics_engine_datasets`.\nAnalytics Engine bindings used for Pages Functions."]
    pub fn set_analytics_engine_datasets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasets>,
                    >,
    ) -> Self {
        self.analytics_engine_datasets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `browsers`.\nBrowser bindings used for Pages Functions."]
    pub fn set_browsers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_Browsers>,
                    >,
    ) -> Self {
        self.browsers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `build_image_major_version`.\nThe major version of the build image to use for Pages Functions. Version 1 is retired and its projects are moved to version 3. Once a project's builds have been moved, a request that sets version 1 is rejected."]
    pub fn set_build_image_major_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.build_image_major_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compatibility_date`.\nCompatibility date used for Pages Functions."]
    pub fn set_compatibility_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.compatibility_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compatibility_flags`.\nCompatibility flags used for Pages Functions."]
    pub fn set_compatibility_flags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.compatibility_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `d1_databases`.\nD1 databases used for Pages Functions."]
    pub fn set_d1_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_D1Databases>,
                    >,
    ) -> Self {
        self.d1_databases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `durable_object_namespaces`.\nDurable Object namespaces used for Pages Functions."]
    pub fn set_durable_object_namespaces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_DurableObjectNamespaces>,
                    >,
    ) -> Self {
        self.durable_object_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env_vars`.\nEnvironment variables used for builds and Pages Functions."]
    pub fn set_env_vars(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std::collections::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_EnvVars>,
                    >,
    ) -> Self {
        self.env_vars = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_open`.\nWhether to fail open when the deployment config cannot be applied."]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hyperdrive_bindings`.\nHyperdrive bindings used for Pages Functions."]
    pub fn set_hyperdrive_bindings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_HyperdriveBindings>,
                    >,
    ) -> Self {
        self.hyperdrive_bindings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kv_namespaces`.\nKV namespaces used for Pages Functions."]
    pub fn set_kv_namespaces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_KvNamespaces>,
                    >,
    ) -> Self {
        self.kv_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limits`.\nLimits for Pages Functions."]
    pub fn set_limits(mut self, v: impl std::convert::Into<PagesProject_DeploymentConfigs_Preview_Limits>) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mtls_certificates`.\nmTLS bindings used for Pages Functions."]
    pub fn set_mtls_certificates(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_MtlsCertificates>,
                    >,
    ) -> Self {
        self.mtls_certificates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement`.\nPlacement setting used for Pages Functions."]
    pub fn set_placement(mut self, v: impl std::convert::Into<PagesProject_DeploymentConfigs_Preview_Placement>) -> Self {
        self.placement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_producers`.\nQueue Producer bindings used for Pages Functions."]
    pub fn set_queue_producers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_QueueProducers>,
                    >,
    ) -> Self {
        self.queue_producers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `r2_buckets`.\nR2 buckets used for Pages Functions."]
    pub fn set_r2_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_R2Buckets>,
                    >,
    ) -> Self {
        self.r2_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `services`.\nServices used for Pages Functions."]
    pub fn set_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_Services>,
                    >,
    ) -> Self {
        self.services = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `usage_model`.\nThe usage model for Pages Functions.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn set_usage_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.usage_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vectorize_bindings`.\nVectorize bindings used for Pages Functions."]
    pub fn set_vectorize_bindings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        std
                        ::collections
                        ::HashMap<std::string::String, PagesProject_DeploymentConfigs_Preview_VectorizeBindings>,
                    >,
    ) -> Self {
        self.vectorize_bindings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wrangler_config_hash`.\nHash of the Wrangler configuration used for the deployment."]
    pub fn set_wrangler_config_hash(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.wrangler_config_hash = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProject_DeploymentConfigs_Production {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs_Production>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs_Production {}

impl BuildPagesProject_DeploymentConfigs_Production {
    pub fn build(self) -> PagesProject_DeploymentConfigs_Production {
        PagesProject_DeploymentConfigs_Production {
            ai_bindings: core::default::Default::default(),
            always_use_latest_compatibility_date: core::default::Default::default(),
            analytics_engine_datasets: core::default::Default::default(),
            browsers: core::default::Default::default(),
            build_image_major_version: core::default::Default::default(),
            compatibility_date: core::default::Default::default(),
            compatibility_flags: core::default::Default::default(),
            d1_databases: core::default::Default::default(),
            durable_object_namespaces: core::default::Default::default(),
            env_vars: core::default::Default::default(),
            fail_open: core::default::Default::default(),
            hyperdrive_bindings: core::default::Default::default(),
            kv_namespaces: core::default::Default::default(),
            limits: core::default::Default::default(),
            mtls_certificates: core::default::Default::default(),
            placement: core::default::Default::default(),
            queue_producers: core::default::Default::default(),
            r2_buckets: core::default::Default::default(),
            services: core::default::Default::default(),
            usage_model: core::default::Default::default(),
            vectorize_bindings: core::default::Default::default(),
            wrangler_config_hash: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigs_ProductionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigs_ProductionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_DeploymentConfigs_ProductionRef {
        PagesProject_DeploymentConfigs_ProductionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigs_ProductionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ai_bindings` after provisioning.\nConstellation bindings used for Pages Functions."]
    pub fn ai_bindings(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_AiBindingsRef> {
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
    ) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_AnalyticsEngineDatasetsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.analytics_engine_datasets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `browsers` after provisioning.\nBrowser bindings used for Pages Functions."]
    pub fn browsers(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_BrowsersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.browsers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `build_image_major_version` after provisioning.\nThe major version of the build image to use for Pages Functions. Version 1 is retired and its projects are moved to version 3. Once a project's builds have been moved, a request that sets version 1 is rejected."]
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
    pub fn d1_databases(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_D1DatabasesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.d1_databases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `durable_object_namespaces` after provisioning.\nDurable Object namespaces used for Pages Functions."]
    pub fn durable_object_namespaces(
        &self,
    ) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_DurableObjectNamespacesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.durable_object_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env_vars` after provisioning.\nEnvironment variables used for builds and Pages Functions."]
    pub fn env_vars(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_EnvVarsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_vars", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fail_open` after provisioning.\nWhether to fail open when the deployment config cannot be applied."]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hyperdrive_bindings` after provisioning.\nHyperdrive bindings used for Pages Functions."]
    pub fn hyperdrive_bindings(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_HyperdriveBindingsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.hyperdrive_bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kv_namespaces` after provisioning.\nKV namespaces used for Pages Functions."]
    pub fn kv_namespaces(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_KvNamespacesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.kv_namespaces", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\nLimits for Pages Functions."]
    pub fn limits(&self) -> PagesProject_DeploymentConfigs_Preview_LimitsRef {
        PagesProject_DeploymentConfigs_Preview_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mtls_certificates` after provisioning.\nmTLS bindings used for Pages Functions."]
    pub fn mtls_certificates(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_MtlsCertificatesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.mtls_certificates", self.base))
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nPlacement setting used for Pages Functions."]
    pub fn placement(&self) -> PagesProject_DeploymentConfigs_Preview_PlacementRef {
        PagesProject_DeploymentConfigs_Preview_PlacementRef::new(
            self.shared().clone(),
            format!("{}.placement", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `queue_producers` after provisioning.\nQueue Producer bindings used for Pages Functions."]
    pub fn queue_producers(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_QueueProducersRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.queue_producers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `r2_buckets` after provisioning.\nR2 buckets used for Pages Functions."]
    pub fn r2_buckets(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_R2BucketsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.r2_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `services` after provisioning.\nServices used for Pages Functions."]
    pub fn services(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_ServicesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.services", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nThe usage model for Pages Functions.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vectorize_bindings` after provisioning.\nVectorize bindings used for Pages Functions."]
    pub fn vectorize_bindings(&self) -> terrars::RecRef<PagesProject_DeploymentConfigs_Preview_VectorizeBindingsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.vectorize_bindings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wrangler_config_hash` after provisioning.\nHash of the Wrangler configuration used for the deployment."]
    pub fn wrangler_config_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wrangler_config_hash", self.base))
    }
}

#[derive(Serialize)]
pub struct PagesProject_DeploymentConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview: std::option::Option<PagesProject_DeploymentConfigs_Preview>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    production: std::option::Option<PagesProject_DeploymentConfigs_Production>,
}

impl PagesProject_DeploymentConfigs {
    #[doc = "Set the field `preview`.\nConfigs for preview deploys."]
    pub fn set_preview(mut self, v: impl std::convert::Into<PagesProject_DeploymentConfigs_Preview>) -> Self {
        self.preview = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `production`.\nConfigs for production deploys."]
    pub fn set_production(mut self, v: impl std::convert::Into<PagesProject_DeploymentConfigs_Production>) -> Self {
        self.production = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProject_DeploymentConfigs {
    type O = terrars::BlockAssignable<PagesProject_DeploymentConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_DeploymentConfigs {}

impl BuildPagesProject_DeploymentConfigs {
    pub fn build(self) -> PagesProject_DeploymentConfigs {
        PagesProject_DeploymentConfigs {
            preview: core::default::Default::default(),
            production: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PagesProject_DeploymentConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_DeploymentConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_DeploymentConfigsRef {
        PagesProject_DeploymentConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_DeploymentConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `preview` after provisioning.\nConfigs for preview deploys."]
    pub fn preview(&self) -> PagesProject_DeploymentConfigs_PreviewRef {
        PagesProject_DeploymentConfigs_PreviewRef::new(self.shared().clone(), format!("{}.preview", self.base))
    }

    #[doc = "Get a reference to the value of field `production` after provisioning.\nConfigs for production deploys."]
    pub fn production(&self) -> PagesProject_DeploymentConfigs_ProductionRef {
        PagesProject_DeploymentConfigs_ProductionRef::new(self.shared().clone(), format!("{}.production", self.base))
    }
}

#[derive(Serialize)]
pub struct PagesProject_LatestDeployment {}

impl PagesProject_LatestDeployment { }

impl terrars::ToListMappable for PagesProject_LatestDeployment {
    type O = terrars::BlockAssignable<PagesProject_LatestDeployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_LatestDeployment {}

impl BuildPagesProject_LatestDeployment {
    pub fn build(self) -> PagesProject_LatestDeployment {
        PagesProject_LatestDeployment {}
    }
}

#[allow(dead_code)]
pub struct PagesProject_LatestDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_LatestDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_LatestDeploymentRef {
        PagesProject_LatestDeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_LatestDeploymentRef {
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
    pub fn build_config(&self) -> PagesProject_LatestDeployment_BuildConfigRef {
        PagesProject_LatestDeployment_BuildConfigRef::new(
            self.shared().clone(),
            format!("{}.build_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the deployment was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_trigger` after provisioning.\nInfo about what caused the deployment."]
    pub fn deployment_trigger(&self) -> PagesProject_LatestDeployment_DeploymentTriggerRef {
        PagesProject_LatestDeployment_DeploymentTriggerRef::new(
            self.shared().clone(),
            format!("{}.deployment_trigger", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `env_vars` after provisioning.\nEnvironment variables used for builds and Pages Functions."]
    pub fn env_vars(&self) -> terrars::RecRef<PagesProject_LatestDeployment_EnvVarsRef> {
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
    pub fn latest_stage(&self) -> PagesProject_LatestDeployment_LatestStageRef {
        PagesProject_LatestDeployment_LatestStageRef::new(
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
        "Get a reference to the value of field `source` after provisioning.\nConfigs for the project source control."]
    pub fn source(&self) -> PagesProject_LatestDeployment_SourceRef {
        PagesProject_LatestDeployment_SourceRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `stages` after provisioning.\nList of past stages."]
    pub fn stages(&self) -> terrars::ListRef<PagesProject_LatestDeployment_StagesRef> {
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
pub struct PagesProject_Source_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployments_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_excludes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_includes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pr_comments_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview_branch_excludes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview_branch_includes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview_deployment_setting: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    production_branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    production_deployments_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PagesProject_Source_Config {
    #[doc =
        "Set the field `deployments_enabled`.\nWhether to enable automatic deployments when pushing to the source repository.\nWhen disabled, no deployments (production or preview) will be triggered automatically."]
    pub fn set_deployments_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deployments_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner`.\nThe owner of the repository."]
    pub fn set_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner_id`.\nThe owner ID of the repository."]
    pub fn set_owner_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_excludes`.\nA list of paths that should be excluded from triggering a preview deployment. Wildcard syntax (`*`) is supported."]
    pub fn set_path_excludes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.path_excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_includes`.\nA list of paths that should be watched to trigger a preview deployment. Wildcard syntax (`*`) is supported."]
    pub fn set_path_includes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.path_includes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pr_comments_enabled`.\nWhether to enable PR comments."]
    pub fn set_pr_comments_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pr_comments_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preview_branch_excludes`.\nA list of branches that should not trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`."]
    pub fn set_preview_branch_excludes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.preview_branch_excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preview_branch_includes`.\nA list of branches that should trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`."]
    pub fn set_preview_branch_includes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.preview_branch_includes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preview_deployment_setting`.\nControls whether commits to preview branches trigger a preview deployment.\nAvailable values: \"all\", \"none\", \"custom\"."]
    pub fn set_preview_deployment_setting(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.preview_deployment_setting = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `production_branch`.\nThe production branch of the repository."]
    pub fn set_production_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.production_branch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `production_deployments_enabled`.\nWhether to trigger a production deployment on commits to the production branch."]
    pub fn set_production_deployments_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.production_deployments_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_id`.\nThe ID of the repository."]
    pub fn set_repo_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_name`.\nThe name of the repository."]
    pub fn set_repo_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProject_Source_Config {
    type O = terrars::BlockAssignable<PagesProject_Source_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_Source_Config {}

impl BuildPagesProject_Source_Config {
    pub fn build(self) -> PagesProject_Source_Config {
        PagesProject_Source_Config {
            deployments_enabled: core::default::Default::default(),
            owner: core::default::Default::default(),
            owner_id: core::default::Default::default(),
            path_excludes: core::default::Default::default(),
            path_includes: core::default::Default::default(),
            pr_comments_enabled: core::default::Default::default(),
            preview_branch_excludes: core::default::Default::default(),
            preview_branch_includes: core::default::Default::default(),
            preview_deployment_setting: core::default::Default::default(),
            production_branch: core::default::Default::default(),
            production_deployments_enabled: core::default::Default::default(),
            repo_id: core::default::Default::default(),
            repo_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PagesProject_Source_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_Source_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_Source_ConfigRef {
        PagesProject_Source_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_Source_ConfigRef {
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
pub struct PagesProject_Source {
    config: PagesProject_Source_Config,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl PagesProject_Source { }

impl terrars::ToListMappable for PagesProject_Source {
    type O = terrars::BlockAssignable<PagesProject_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProject_Source {
    #[doc = ""]
    pub config: PagesProject_Source_Config,
    #[doc = "The source control management provider.\nAvailable values: \"github\", \"gitlab\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject_Source {
    pub fn build(self) -> PagesProject_Source {
        PagesProject_Source {
            config: self.config,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct PagesProject_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProject_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProject_SourceRef {
        PagesProject_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProject_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> PagesProject_Source_ConfigRef {
        PagesProject_Source_ConfigRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe source control management provider.\nAvailable values: \"github\", \"gitlab\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
