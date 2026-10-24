use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct PagesProjectData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    production_branch: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_config: std::option::Option<std::vec::Vec<PagesProjectBuildConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_configs: std::option::Option<std::vec::Vec<PagesProjectDeploymentConfigsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<PagesProjectSourceEl>>,
    dynamic: PagesProjectDynamic,
}

struct PagesProject_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PagesProjectData>,
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_config`.\n"]
    pub fn set_build_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PagesProjectBuildConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment_configs`.\n"]
    pub fn set_deployment_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PagesProjectDeploymentConfigsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl std::convert::Into<terrars::BlockAssignable<PagesProjectSourceEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the project was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nA list of associated custom domains for the project."]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the project. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `production_branch` after provisioning.\nThe name of the branch that is used for the production environment."]
    pub fn production_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_branch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subdomain` after provisioning.\nThe Cloudflare subdomain associated with the project."]
    pub fn subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_config` after provisioning.\n"]
    pub fn build_config(&self) -> terrars::ListRef<PagesProjectBuildConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_configs` after provisioning.\n"]
    pub fn deployment_configs(&self) -> terrars::ListRef<PagesProjectDeploymentConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<PagesProjectSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
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
    #[doc = "The account identifier to target for the resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the project. **Modifying this attribute will force creation of a new resource.**"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the branch that is used for the production environment."]
    pub production_branch: terrars::PrimField<std::string::String>,
}

impl BuildPagesProject {
    pub fn build(self, stack: &mut terrars::Stack) -> PagesProject {
        let out = PagesProject(std::rc::Rc::new(PagesProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PagesProjectData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                id: core::default::Default::default(),
                name: self.name,
                production_branch: self.production_branch,
                build_config: core::default::Default::default(),
                deployment_configs: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the project was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nA list of associated custom domains for the project."]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the project. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `production_branch` after provisioning.\nThe name of the branch that is used for the production environment."]
    pub fn production_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_branch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subdomain` after provisioning.\nThe Cloudflare subdomain associated with the project."]
    pub fn subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_config` after provisioning.\n"]
    pub fn build_config(&self) -> terrars::ListRef<PagesProjectBuildConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_configs` after provisioning.\n"]
    pub fn deployment_configs(&self) -> terrars::ListRef<PagesProjectDeploymentConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<PagesProjectSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PagesProjectBuildConfigEl {
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

impl PagesProjectBuildConfigEl {
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

    #[doc =
        "Set the field `root_dir`.\nYour project's root directory, where Cloudflare runs the build command. If your site is not in a subdirectory, leave this path value empty."]
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

impl terrars::ToListMappable for PagesProjectBuildConfigEl {
    type O = terrars::BlockAssignable<PagesProjectBuildConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectBuildConfigEl {}

impl BuildPagesProjectBuildConfigEl {
    pub fn build(self) -> PagesProjectBuildConfigEl {
        PagesProjectBuildConfigEl {
            build_command: core::default::Default::default(),
            destination_dir: core::default::Default::default(),
            root_dir: core::default::Default::default(),
            web_analytics_tag: core::default::Default::default(),
            web_analytics_token: core::default::Default::default(),
        }
    }
}

pub struct PagesProjectBuildConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProjectBuildConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProjectBuildConfigElRef {
        PagesProjectBuildConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectBuildConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc =
        "Get a reference to the value of field `root_dir` after provisioning.\nYour project's root directory, where Cloudflare runs the build command. If your site is not in a subdirectory, leave this path value empty."]
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
pub struct PagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
}

impl PagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
    #[doc = "Set the field `environment`.\nThe name of the Worker environment to bind to."]
    pub fn set_environment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
    type O = terrars::BlockAssignable<PagesProjectDeploymentConfigsElPreviewElServiceBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
    #[doc = "The global variable for the binding in your Worker code."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Worker to bind to."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildPagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
    pub fn build(self) -> PagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
        PagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
            environment: core::default::Default::default(),
            name: self.name,
            service: self.service,
        }
    }
}

pub struct PagesProjectDeploymentConfigsElPreviewElServiceBindingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProjectDeploymentConfigsElPreviewElServiceBindingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProjectDeploymentConfigsElPreviewElServiceBindingElRef {
        PagesProjectDeploymentConfigsElPreviewElServiceBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectDeploymentConfigsElPreviewElServiceBindingElRef {
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

#[derive(Serialize, Default)]
struct PagesProjectDeploymentConfigsElPreviewElDynamic {
    service_binding: std
    ::option
    ::Option<terrars::DynamicBlock<PagesProjectDeploymentConfigsElPreviewElServiceBindingEl>>,
}

#[derive(Serialize)]
pub struct PagesProjectDeploymentConfigsElPreviewEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    always_use_latest_compatibility_date: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_flags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    d1_databases: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    durable_object_namespaces: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kv_namespaces: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r2_buckets: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_binding: std::option::Option<std::vec::Vec<PagesProjectDeploymentConfigsElPreviewElServiceBindingEl>>,
    dynamic: PagesProjectDeploymentConfigsElPreviewElDynamic,
}

impl PagesProjectDeploymentConfigsElPreviewEl {
    #[doc =
        "Set the field `always_use_latest_compatibility_date`.\nUse latest compatibility date for Pages Functions. Defaults to `false`."]
    pub fn set_always_use_latest_compatibility_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.always_use_latest_compatibility_date = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `d1_databases`.\nD1 Databases used for Pages Functions."]
    pub fn set_d1_databases(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.d1_databases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `durable_object_namespaces`.\nDurable Object namespaces used for Pages Functions."]
    pub fn set_durable_object_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.durable_object_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\nEnvironment variables for Pages Functions."]
    pub fn set_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_open`.\nFail open used for Pages Functions. Defaults to `false`."]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kv_namespaces`.\nKV namespaces used for Pages Functions."]
    pub fn set_kv_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.kv_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `r2_buckets`.\nR2 Buckets used for Pages Functions."]
    pub fn set_r2_buckets(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.r2_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets`.\nEncrypted environment variables for Pages Functions."]
    pub fn set_secrets(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.secrets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usage_model`.\nUsage model used for Pages Functions. Defaults to `bundled`."]
    pub fn set_usage_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.usage_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_binding`.\n"]
    pub fn set_service_binding(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PagesProjectDeploymentConfigsElPreviewElServiceBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_binding = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PagesProjectDeploymentConfigsElPreviewEl {
    type O = terrars::BlockAssignable<PagesProjectDeploymentConfigsElPreviewEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectDeploymentConfigsElPreviewEl {}

impl BuildPagesProjectDeploymentConfigsElPreviewEl {
    pub fn build(self) -> PagesProjectDeploymentConfigsElPreviewEl {
        PagesProjectDeploymentConfigsElPreviewEl {
            always_use_latest_compatibility_date: core::default::Default::default(),
            compatibility_date: core::default::Default::default(),
            compatibility_flags: core::default::Default::default(),
            d1_databases: core::default::Default::default(),
            durable_object_namespaces: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            fail_open: core::default::Default::default(),
            kv_namespaces: core::default::Default::default(),
            r2_buckets: core::default::Default::default(),
            secrets: core::default::Default::default(),
            usage_model: core::default::Default::default(),
            service_binding: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PagesProjectDeploymentConfigsElPreviewElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProjectDeploymentConfigsElPreviewElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProjectDeploymentConfigsElPreviewElRef {
        PagesProjectDeploymentConfigsElPreviewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectDeploymentConfigsElPreviewElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `always_use_latest_compatibility_date` after provisioning.\nUse latest compatibility date for Pages Functions. Defaults to `false`."]
    pub fn always_use_latest_compatibility_date(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.always_use_latest_compatibility_date", self.base))
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
        "Get a reference to the value of field `d1_databases` after provisioning.\nD1 Databases used for Pages Functions."]
    pub fn d1_databases(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.d1_databases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `durable_object_namespaces` after provisioning.\nDurable Object namespaces used for Pages Functions."]
    pub fn durable_object_namespaces(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.durable_object_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_variables` after provisioning.\nEnvironment variables for Pages Functions."]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fail_open` after provisioning.\nFail open used for Pages Functions. Defaults to `false`."]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kv_namespaces` after provisioning.\nKV namespaces used for Pages Functions."]
    pub fn kv_namespaces(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.kv_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `r2_buckets` after provisioning.\nR2 Buckets used for Pages Functions."]
    pub fn r2_buckets(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.r2_buckets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secrets` after provisioning.\nEncrypted environment variables for Pages Functions."]
    pub fn secrets(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.secrets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nUsage model used for Pages Functions. Defaults to `bundled`."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.base))
    }
}

#[derive(Serialize)]
pub struct PagesProjectDeploymentConfigsElProductionElServiceBindingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
}

impl PagesProjectDeploymentConfigsElProductionElServiceBindingEl {
    #[doc = "Set the field `environment`.\nThe name of the Worker environment to bind to."]
    pub fn set_environment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProjectDeploymentConfigsElProductionElServiceBindingEl {
    type O = terrars::BlockAssignable<PagesProjectDeploymentConfigsElProductionElServiceBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectDeploymentConfigsElProductionElServiceBindingEl {
    #[doc = "The global variable for the binding in your Worker code."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Worker to bind to."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildPagesProjectDeploymentConfigsElProductionElServiceBindingEl {
    pub fn build(self) -> PagesProjectDeploymentConfigsElProductionElServiceBindingEl {
        PagesProjectDeploymentConfigsElProductionElServiceBindingEl {
            environment: core::default::Default::default(),
            name: self.name,
            service: self.service,
        }
    }
}

pub struct PagesProjectDeploymentConfigsElProductionElServiceBindingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProjectDeploymentConfigsElProductionElServiceBindingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProjectDeploymentConfigsElProductionElServiceBindingElRef {
        PagesProjectDeploymentConfigsElProductionElServiceBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectDeploymentConfigsElProductionElServiceBindingElRef {
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

#[derive(Serialize, Default)]
struct PagesProjectDeploymentConfigsElProductionElDynamic {
    service_binding: std
    ::option
    ::Option<terrars::DynamicBlock<PagesProjectDeploymentConfigsElProductionElServiceBindingEl>>,
}

#[derive(Serialize)]
pub struct PagesProjectDeploymentConfigsElProductionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    always_use_latest_compatibility_date: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_flags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    d1_databases: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    durable_object_namespaces: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kv_namespaces: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r2_buckets: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_binding: std::option::Option<std::vec::Vec<PagesProjectDeploymentConfigsElProductionElServiceBindingEl>>,
    dynamic: PagesProjectDeploymentConfigsElProductionElDynamic,
}

impl PagesProjectDeploymentConfigsElProductionEl {
    #[doc =
        "Set the field `always_use_latest_compatibility_date`.\nUse latest compatibility date for Pages Functions. Defaults to `false`."]
    pub fn set_always_use_latest_compatibility_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.always_use_latest_compatibility_date = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `d1_databases`.\nD1 Databases used for Pages Functions."]
    pub fn set_d1_databases(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.d1_databases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `durable_object_namespaces`.\nDurable Object namespaces used for Pages Functions."]
    pub fn set_durable_object_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.durable_object_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\nEnvironment variables for Pages Functions."]
    pub fn set_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_open`.\nFail open used for Pages Functions. Defaults to `false`."]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kv_namespaces`.\nKV namespaces used for Pages Functions."]
    pub fn set_kv_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.kv_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `r2_buckets`.\nR2 Buckets used for Pages Functions."]
    pub fn set_r2_buckets(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.r2_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets`.\nEncrypted environment variables for Pages Functions."]
    pub fn set_secrets(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.secrets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usage_model`.\nUsage model used for Pages Functions. Defaults to `bundled`."]
    pub fn set_usage_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.usage_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_binding`.\n"]
    pub fn set_service_binding(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PagesProjectDeploymentConfigsElProductionElServiceBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_binding = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PagesProjectDeploymentConfigsElProductionEl {
    type O = terrars::BlockAssignable<PagesProjectDeploymentConfigsElProductionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectDeploymentConfigsElProductionEl {}

impl BuildPagesProjectDeploymentConfigsElProductionEl {
    pub fn build(self) -> PagesProjectDeploymentConfigsElProductionEl {
        PagesProjectDeploymentConfigsElProductionEl {
            always_use_latest_compatibility_date: core::default::Default::default(),
            compatibility_date: core::default::Default::default(),
            compatibility_flags: core::default::Default::default(),
            d1_databases: core::default::Default::default(),
            durable_object_namespaces: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            fail_open: core::default::Default::default(),
            kv_namespaces: core::default::Default::default(),
            r2_buckets: core::default::Default::default(),
            secrets: core::default::Default::default(),
            usage_model: core::default::Default::default(),
            service_binding: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PagesProjectDeploymentConfigsElProductionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProjectDeploymentConfigsElProductionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PagesProjectDeploymentConfigsElProductionElRef {
        PagesProjectDeploymentConfigsElProductionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectDeploymentConfigsElProductionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `always_use_latest_compatibility_date` after provisioning.\nUse latest compatibility date for Pages Functions. Defaults to `false`."]
    pub fn always_use_latest_compatibility_date(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.always_use_latest_compatibility_date", self.base))
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
        "Get a reference to the value of field `d1_databases` after provisioning.\nD1 Databases used for Pages Functions."]
    pub fn d1_databases(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.d1_databases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `durable_object_namespaces` after provisioning.\nDurable Object namespaces used for Pages Functions."]
    pub fn durable_object_namespaces(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.durable_object_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_variables` after provisioning.\nEnvironment variables for Pages Functions."]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fail_open` after provisioning.\nFail open used for Pages Functions. Defaults to `false`."]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kv_namespaces` after provisioning.\nKV namespaces used for Pages Functions."]
    pub fn kv_namespaces(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.kv_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `r2_buckets` after provisioning.\nR2 Buckets used for Pages Functions."]
    pub fn r2_buckets(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.r2_buckets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secrets` after provisioning.\nEncrypted environment variables for Pages Functions."]
    pub fn secrets(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.secrets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nUsage model used for Pages Functions. Defaults to `bundled`."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.base))
    }
}

#[derive(Serialize, Default)]
struct PagesProjectDeploymentConfigsElDynamic {
    preview: std::option::Option<terrars::DynamicBlock<PagesProjectDeploymentConfigsElPreviewEl>>,
    production: std::option::Option<terrars::DynamicBlock<PagesProjectDeploymentConfigsElProductionEl>>,
}

#[derive(Serialize)]
pub struct PagesProjectDeploymentConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview: std::option::Option<std::vec::Vec<PagesProjectDeploymentConfigsElPreviewEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    production: std::option::Option<std::vec::Vec<PagesProjectDeploymentConfigsElProductionEl>>,
    dynamic: PagesProjectDeploymentConfigsElDynamic,
}

impl PagesProjectDeploymentConfigsEl {
    #[doc = "Set the field `preview`.\n"]
    pub fn set_preview(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PagesProjectDeploymentConfigsElPreviewEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.preview = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.preview = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `production`.\n"]
    pub fn set_production(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PagesProjectDeploymentConfigsElProductionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.production = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.production = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PagesProjectDeploymentConfigsEl {
    type O = terrars::BlockAssignable<PagesProjectDeploymentConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectDeploymentConfigsEl {}

impl BuildPagesProjectDeploymentConfigsEl {
    pub fn build(self) -> PagesProjectDeploymentConfigsEl {
        PagesProjectDeploymentConfigsEl {
            preview: core::default::Default::default(),
            production: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PagesProjectDeploymentConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProjectDeploymentConfigsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProjectDeploymentConfigsElRef {
        PagesProjectDeploymentConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectDeploymentConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `preview` after provisioning.\n"]
    pub fn preview(&self) -> terrars::ListRef<PagesProjectDeploymentConfigsElPreviewElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preview", self.base))
    }

    #[doc = "Get a reference to the value of field `production` after provisioning.\n"]
    pub fn production(&self) -> terrars::ListRef<PagesProjectDeploymentConfigsElProductionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.production", self.base))
    }
}

#[derive(Serialize)]
pub struct PagesProjectSourceElConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployments_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pr_comments_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview_branch_excludes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview_branch_includes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview_deployment_setting: std::option::Option<terrars::PrimField<std::string::String>>,
    production_branch: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    production_deployment_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PagesProjectSourceElConfigEl {
    #[doc = "Set the field `deployments_enabled`.\nToggle deployments on this repo. Defaults to `true`."]
    pub fn set_deployments_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deployments_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner`.\nProject owner username."]
    pub fn set_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pr_comments_enabled`.\nEnable Pages to comment on Pull Requests. Defaults to `true`."]
    pub fn set_pr_comments_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pr_comments_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preview_branch_excludes`.\nBranches will be excluded from automatic deployment."]
    pub fn set_preview_branch_excludes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.preview_branch_excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preview_branch_includes`.\nBranches will be included for automatic deployment."]
    pub fn set_preview_branch_includes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.preview_branch_includes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preview_deployment_setting`.\nPreview Deployment Setting. Defaults to `all`."]
    pub fn set_preview_deployment_setting(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.preview_deployment_setting = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `production_deployment_enabled`.\nEnable production deployments. Defaults to `true`."]
    pub fn set_production_deployment_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.production_deployment_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_name`.\nProject repository name."]
    pub fn set_repo_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PagesProjectSourceElConfigEl {
    type O = terrars::BlockAssignable<PagesProjectSourceElConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectSourceElConfigEl {
    #[doc = "Project production branch name."]
    pub production_branch: terrars::PrimField<std::string::String>,
}

impl BuildPagesProjectSourceElConfigEl {
    pub fn build(self) -> PagesProjectSourceElConfigEl {
        PagesProjectSourceElConfigEl {
            deployments_enabled: core::default::Default::default(),
            owner: core::default::Default::default(),
            pr_comments_enabled: core::default::Default::default(),
            preview_branch_excludes: core::default::Default::default(),
            preview_branch_includes: core::default::Default::default(),
            preview_deployment_setting: core::default::Default::default(),
            production_branch: self.production_branch,
            production_deployment_enabled: core::default::Default::default(),
            repo_name: core::default::Default::default(),
        }
    }
}

pub struct PagesProjectSourceElConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProjectSourceElConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProjectSourceElConfigElRef {
        PagesProjectSourceElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectSourceElConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deployments_enabled` after provisioning.\nToggle deployments on this repo. Defaults to `true`."]
    pub fn deployments_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployments_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\nProject owner username."]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pr_comments_enabled` after provisioning.\nEnable Pages to comment on Pull Requests. Defaults to `true`."]
    pub fn pr_comments_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pr_comments_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview_branch_excludes` after provisioning.\nBranches will be excluded from automatic deployment."]
    pub fn preview_branch_excludes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preview_branch_excludes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview_branch_includes` after provisioning.\nBranches will be included for automatic deployment."]
    pub fn preview_branch_includes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preview_branch_includes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview_deployment_setting` after provisioning.\nPreview Deployment Setting. Defaults to `all`."]
    pub fn preview_deployment_setting(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview_deployment_setting", self.base))
    }

    #[doc =
        "Get a reference to the value of field `production_branch` after provisioning.\nProject production branch name."]
    pub fn production_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `production_deployment_enabled` after provisioning.\nEnable production deployments. Defaults to `true`."]
    pub fn production_deployment_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_deployment_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\nProject repository name."]
    pub fn repo_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct PagesProjectSourceElDynamic {
    config: std::option::Option<terrars::DynamicBlock<PagesProjectSourceElConfigEl>>,
}

#[derive(Serialize)]
pub struct PagesProjectSourceEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<PagesProjectSourceElConfigEl>>,
    dynamic: PagesProjectSourceElDynamic,
}

impl PagesProjectSourceEl {
    #[doc = "Set the field `type_`.\nProject host type."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PagesProjectSourceElConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PagesProjectSourceEl {
    type O = terrars::BlockAssignable<PagesProjectSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectSourceEl {}

impl BuildPagesProjectSourceEl {
    pub fn build(self) -> PagesProjectSourceEl {
        PagesProjectSourceEl {
            type_: core::default::Default::default(),
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PagesProjectSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesProjectSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesProjectSourceElRef {
        PagesProjectSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nProject host type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<PagesProjectSourceElConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize, Default)]
struct PagesProjectDynamic {
    build_config: std::option::Option<terrars::DynamicBlock<PagesProjectBuildConfigEl>>,
    deployment_configs: std::option::Option<terrars::DynamicBlock<PagesProjectDeploymentConfigsEl>>,
    source: std::option::Option<terrars::DynamicBlock<PagesProjectSourceEl>>,
}
