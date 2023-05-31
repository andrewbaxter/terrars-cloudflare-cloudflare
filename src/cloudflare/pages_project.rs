use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct PagesProjectData {
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
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    production_branch: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_config: Option<Vec<PagesProjectBuildConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_configs: Option<Vec<PagesProjectDeploymentConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<PagesProjectSourceEl>>,
    dynamic: PagesProjectDynamic,
}

struct PagesProject_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PagesProjectData>,
}

#[derive(Clone)]
pub struct PagesProject(Rc<PagesProject_>);

impl PagesProject {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `build_config`.\n"]
    pub fn set_build_config(self, v: impl Into<BlockAssignable<PagesProjectBuildConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `deployment_configs`.\n"]
    pub fn set_deployment_configs(self, v: impl Into<BlockAssignable<PagesProjectDeploymentConfigsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_configs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_configs = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<BlockAssignable<PagesProjectSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nWhen the project was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domains` after provisioning.\nA list of associated custom domains for the project."]
    pub fn domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the project. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `production_branch` after provisioning.\nThe name of the branch that is used for the production environment."]
    pub fn production_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.production_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subdomain` after provisioning.\nThe Cloudflare subdomain associated with the project."]
    pub fn subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `build_config` after provisioning.\n"]
    pub fn build_config(&self) -> ListRef<PagesProjectBuildConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.build_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_configs` after provisioning.\n"]
    pub fn deployment_configs(&self) -> ListRef<PagesProjectDeploymentConfigsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_configs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<PagesProjectSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl Referable for PagesProject {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for PagesProject { }

impl ToListMappable for PagesProject {
    type O = ListRef<PagesProjectRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PagesProject_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_pages_project".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPagesProject {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "Name of the project. **Modifying this attribute will force creation of a new resource.**"]
    pub name: PrimField<String>,
    #[doc= "The name of the branch that is used for the production environment."]
    pub production_branch: PrimField<String>,
}

impl BuildPagesProject {
    pub fn build(self, stack: &mut Stack) -> PagesProject {
        let out = PagesProject(Rc::new(PagesProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PagesProjectData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
                name: self.name,
                production_branch: self.production_branch,
                build_config: core::default::Default::default(),
                deployment_configs: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PagesProjectRef {
    shared: StackShared,
    base: String,
}

impl Ref for PagesProjectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PagesProjectRef {
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

    #[doc= "Get a reference to the value of field `created_on` after provisioning.\nWhen the project was created."]
    pub fn created_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domains` after provisioning.\nA list of associated custom domains for the project."]
    pub fn domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the project. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `production_branch` after provisioning.\nThe name of the branch that is used for the production environment."]
    pub fn production_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.production_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subdomain` after provisioning.\nThe Cloudflare subdomain associated with the project."]
    pub fn subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `build_config` after provisioning.\n"]
    pub fn build_config(&self) -> ListRef<PagesProjectBuildConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.build_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_configs` after provisioning.\n"]
    pub fn deployment_configs(&self) -> ListRef<PagesProjectDeploymentConfigsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_configs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<PagesProjectSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PagesProjectBuildConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_analytics_tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_analytics_token: Option<PrimField<String>>,
}

impl PagesProjectBuildConfigEl {
    #[doc= "Set the field `build_command`.\nCommand used to build project."]
    pub fn set_build_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_command = Some(v.into());
        self
    }

    #[doc= "Set the field `destination_dir`.\nOutput directory of the build."]
    pub fn set_destination_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `root_dir`.\nYour project's root directory, where Cloudflare runs the build command. If your site is not in a subdirectory, leave this path value empty."]
    pub fn set_root_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.root_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `web_analytics_tag`.\nThe classifying tag for analytics."]
    pub fn set_web_analytics_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_analytics_tag = Some(v.into());
        self
    }

    #[doc= "Set the field `web_analytics_token`.\nThe auth token for analytics."]
    pub fn set_web_analytics_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_analytics_token = Some(v.into());
        self
    }
}

impl ToListMappable for PagesProjectBuildConfigEl {
    type O = BlockAssignable<PagesProjectBuildConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for PagesProjectBuildConfigElRef {
    fn new(shared: StackShared, base: String) -> PagesProjectBuildConfigElRef {
        PagesProjectBuildConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectBuildConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `build_command` after provisioning.\nCommand used to build project."]
    pub fn build_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc= "Get a reference to the value of field `destination_dir` after provisioning.\nOutput directory of the build."]
    pub fn destination_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `root_dir` after provisioning.\nYour project's root directory, where Cloudflare runs the build command. If your site is not in a subdirectory, leave this path value empty."]
    pub fn root_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `web_analytics_tag` after provisioning.\nThe classifying tag for analytics."]
    pub fn web_analytics_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_analytics_tag", self.base))
    }

    #[doc= "Get a reference to the value of field `web_analytics_token` after provisioning.\nThe auth token for analytics."]
    pub fn web_analytics_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_analytics_token", self.base))
    }
}

#[derive(Serialize)]
pub struct PagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<PrimField<String>>,
    name: PrimField<String>,
    service: PrimField<String>,
}

impl PagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
    #[doc= "Set the field `environment`.\nThe name of the Worker environment to bind to."]
    pub fn set_environment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment = Some(v.into());
        self
    }
}

impl ToListMappable for PagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
    type O = BlockAssignable<PagesProjectDeploymentConfigsElPreviewElServiceBindingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectDeploymentConfigsElPreviewElServiceBindingEl {
    #[doc= "The global variable for the binding in your Worker code."]
    pub name: PrimField<String>,
    #[doc= "The name of the Worker to bind to."]
    pub service: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for PagesProjectDeploymentConfigsElPreviewElServiceBindingElRef {
    fn new(shared: StackShared, base: String) -> PagesProjectDeploymentConfigsElPreviewElServiceBindingElRef {
        PagesProjectDeploymentConfigsElPreviewElServiceBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectDeploymentConfigsElPreviewElServiceBindingElRef {
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

#[derive(Serialize, Default)]
struct PagesProjectDeploymentConfigsElPreviewElDynamic {
    service_binding: Option<DynamicBlock<PagesProjectDeploymentConfigsElPreviewElServiceBindingEl>>,
}

#[derive(Serialize)]
pub struct PagesProjectDeploymentConfigsElPreviewEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    always_use_latest_compatibility_date: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compatibility_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compatibility_flags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    d1_databases: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    durable_object_namespaces: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kv_namespaces: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r2_buckets: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_binding: Option<Vec<PagesProjectDeploymentConfigsElPreviewElServiceBindingEl>>,
    dynamic: PagesProjectDeploymentConfigsElPreviewElDynamic,
}

impl PagesProjectDeploymentConfigsElPreviewEl {
    #[doc= "Set the field `always_use_latest_compatibility_date`.\nUse latest compatibility date for Pages Functions. Defaults to `false`."]
    pub fn set_always_use_latest_compatibility_date(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.always_use_latest_compatibility_date = Some(v.into());
        self
    }

    #[doc= "Set the field `compatibility_date`.\nCompatibility date used for Pages Functions."]
    pub fn set_compatibility_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compatibility_date = Some(v.into());
        self
    }

    #[doc= "Set the field `compatibility_flags`.\nCompatibility flags used for Pages Functions."]
    pub fn set_compatibility_flags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.compatibility_flags = Some(v.into());
        self
    }

    #[doc= "Set the field `d1_databases`.\nD1 Databases used for Pages Functions."]
    pub fn set_d1_databases(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.d1_databases = Some(v.into());
        self
    }

    #[doc= "Set the field `durable_object_namespaces`.\nDurable Object namespaces used for Pages Functions."]
    pub fn set_durable_object_namespaces(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.durable_object_namespaces = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_variables`.\nEnvironment variables for Pages Functions."]
    pub fn set_environment_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment_variables = Some(v.into());
        self
    }

    #[doc= "Set the field `fail_open`.\nFail open used for Pages Functions. Defaults to `false`."]
    pub fn set_fail_open(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_open = Some(v.into());
        self
    }

    #[doc= "Set the field `kv_namespaces`.\nKV namespaces used for Pages Functions."]
    pub fn set_kv_namespaces(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.kv_namespaces = Some(v.into());
        self
    }

    #[doc= "Set the field `r2_buckets`.\nR2 Buckets used for Pages Functions."]
    pub fn set_r2_buckets(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.r2_buckets = Some(v.into());
        self
    }

    #[doc= "Set the field `secrets`.\nEncrypted environment variables for Pages Functions."]
    pub fn set_secrets(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.secrets = Some(v.into());
        self
    }

    #[doc= "Set the field `usage_model`.\nUsage model used for Pages Functions. Defaults to `bundled`."]
    pub fn set_usage_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.usage_model = Some(v.into());
        self
    }

    #[doc= "Set the field `service_binding`.\n"]
    pub fn set_service_binding(
        mut self,
        v: impl Into<BlockAssignable<PagesProjectDeploymentConfigsElPreviewElServiceBindingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_binding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_binding = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for PagesProjectDeploymentConfigsElPreviewEl {
    type O = BlockAssignable<PagesProjectDeploymentConfigsElPreviewEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct PagesProjectDeploymentConfigsElPreviewElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PagesProjectDeploymentConfigsElPreviewElRef {
    fn new(shared: StackShared, base: String) -> PagesProjectDeploymentConfigsElPreviewElRef {
        PagesProjectDeploymentConfigsElPreviewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectDeploymentConfigsElPreviewElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `always_use_latest_compatibility_date` after provisioning.\nUse latest compatibility date for Pages Functions. Defaults to `false`."]
    pub fn always_use_latest_compatibility_date(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.always_use_latest_compatibility_date", self.base))
    }

    #[doc= "Get a reference to the value of field `compatibility_date` after provisioning.\nCompatibility date used for Pages Functions."]
    pub fn compatibility_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.base))
    }

    #[doc= "Get a reference to the value of field `compatibility_flags` after provisioning.\nCompatibility flags used for Pages Functions."]
    pub fn compatibility_flags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.base))
    }

    #[doc= "Get a reference to the value of field `d1_databases` after provisioning.\nD1 Databases used for Pages Functions."]
    pub fn d1_databases(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.d1_databases", self.base))
    }

    #[doc= "Get a reference to the value of field `durable_object_namespaces` after provisioning.\nDurable Object namespaces used for Pages Functions."]
    pub fn durable_object_namespaces(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.durable_object_namespaces", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_variables` after provisioning.\nEnvironment variables for Pages Functions."]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc= "Get a reference to the value of field `fail_open` after provisioning.\nFail open used for Pages Functions. Defaults to `false`."]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc= "Get a reference to the value of field `kv_namespaces` after provisioning.\nKV namespaces used for Pages Functions."]
    pub fn kv_namespaces(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.kv_namespaces", self.base))
    }

    #[doc= "Get a reference to the value of field `r2_buckets` after provisioning.\nR2 Buckets used for Pages Functions."]
    pub fn r2_buckets(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.r2_buckets", self.base))
    }

    #[doc= "Get a reference to the value of field `secrets` after provisioning.\nEncrypted environment variables for Pages Functions."]
    pub fn secrets(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.secrets", self.base))
    }

    #[doc= "Get a reference to the value of field `usage_model` after provisioning.\nUsage model used for Pages Functions. Defaults to `bundled`."]
    pub fn usage_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.base))
    }
}

#[derive(Serialize)]
pub struct PagesProjectDeploymentConfigsElProductionElServiceBindingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<PrimField<String>>,
    name: PrimField<String>,
    service: PrimField<String>,
}

impl PagesProjectDeploymentConfigsElProductionElServiceBindingEl {
    #[doc= "Set the field `environment`.\nThe name of the Worker environment to bind to."]
    pub fn set_environment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment = Some(v.into());
        self
    }
}

impl ToListMappable for PagesProjectDeploymentConfigsElProductionElServiceBindingEl {
    type O = BlockAssignable<PagesProjectDeploymentConfigsElProductionElServiceBindingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectDeploymentConfigsElProductionElServiceBindingEl {
    #[doc= "The global variable for the binding in your Worker code."]
    pub name: PrimField<String>,
    #[doc= "The name of the Worker to bind to."]
    pub service: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for PagesProjectDeploymentConfigsElProductionElServiceBindingElRef {
    fn new(shared: StackShared, base: String) -> PagesProjectDeploymentConfigsElProductionElServiceBindingElRef {
        PagesProjectDeploymentConfigsElProductionElServiceBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectDeploymentConfigsElProductionElServiceBindingElRef {
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

#[derive(Serialize, Default)]
struct PagesProjectDeploymentConfigsElProductionElDynamic {
    service_binding: Option<DynamicBlock<PagesProjectDeploymentConfigsElProductionElServiceBindingEl>>,
}

#[derive(Serialize)]
pub struct PagesProjectDeploymentConfigsElProductionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    always_use_latest_compatibility_date: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compatibility_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compatibility_flags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    d1_databases: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    durable_object_namespaces: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kv_namespaces: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r2_buckets: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_binding: Option<Vec<PagesProjectDeploymentConfigsElProductionElServiceBindingEl>>,
    dynamic: PagesProjectDeploymentConfigsElProductionElDynamic,
}

impl PagesProjectDeploymentConfigsElProductionEl {
    #[doc= "Set the field `always_use_latest_compatibility_date`.\nUse latest compatibility date for Pages Functions. Defaults to `false`."]
    pub fn set_always_use_latest_compatibility_date(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.always_use_latest_compatibility_date = Some(v.into());
        self
    }

    #[doc= "Set the field `compatibility_date`.\nCompatibility date used for Pages Functions."]
    pub fn set_compatibility_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compatibility_date = Some(v.into());
        self
    }

    #[doc= "Set the field `compatibility_flags`.\nCompatibility flags used for Pages Functions."]
    pub fn set_compatibility_flags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.compatibility_flags = Some(v.into());
        self
    }

    #[doc= "Set the field `d1_databases`.\nD1 Databases used for Pages Functions."]
    pub fn set_d1_databases(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.d1_databases = Some(v.into());
        self
    }

    #[doc= "Set the field `durable_object_namespaces`.\nDurable Object namespaces used for Pages Functions."]
    pub fn set_durable_object_namespaces(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.durable_object_namespaces = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_variables`.\nEnvironment variables for Pages Functions."]
    pub fn set_environment_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment_variables = Some(v.into());
        self
    }

    #[doc= "Set the field `fail_open`.\nFail open used for Pages Functions. Defaults to `false`."]
    pub fn set_fail_open(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_open = Some(v.into());
        self
    }

    #[doc= "Set the field `kv_namespaces`.\nKV namespaces used for Pages Functions."]
    pub fn set_kv_namespaces(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.kv_namespaces = Some(v.into());
        self
    }

    #[doc= "Set the field `r2_buckets`.\nR2 Buckets used for Pages Functions."]
    pub fn set_r2_buckets(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.r2_buckets = Some(v.into());
        self
    }

    #[doc= "Set the field `secrets`.\nEncrypted environment variables for Pages Functions."]
    pub fn set_secrets(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.secrets = Some(v.into());
        self
    }

    #[doc= "Set the field `usage_model`.\nUsage model used for Pages Functions. Defaults to `bundled`."]
    pub fn set_usage_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.usage_model = Some(v.into());
        self
    }

    #[doc= "Set the field `service_binding`.\n"]
    pub fn set_service_binding(
        mut self,
        v: impl Into<BlockAssignable<PagesProjectDeploymentConfigsElProductionElServiceBindingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_binding = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_binding = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for PagesProjectDeploymentConfigsElProductionEl {
    type O = BlockAssignable<PagesProjectDeploymentConfigsElProductionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct PagesProjectDeploymentConfigsElProductionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PagesProjectDeploymentConfigsElProductionElRef {
    fn new(shared: StackShared, base: String) -> PagesProjectDeploymentConfigsElProductionElRef {
        PagesProjectDeploymentConfigsElProductionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectDeploymentConfigsElProductionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `always_use_latest_compatibility_date` after provisioning.\nUse latest compatibility date for Pages Functions. Defaults to `false`."]
    pub fn always_use_latest_compatibility_date(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.always_use_latest_compatibility_date", self.base))
    }

    #[doc= "Get a reference to the value of field `compatibility_date` after provisioning.\nCompatibility date used for Pages Functions."]
    pub fn compatibility_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.base))
    }

    #[doc= "Get a reference to the value of field `compatibility_flags` after provisioning.\nCompatibility flags used for Pages Functions."]
    pub fn compatibility_flags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.base))
    }

    #[doc= "Get a reference to the value of field `d1_databases` after provisioning.\nD1 Databases used for Pages Functions."]
    pub fn d1_databases(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.d1_databases", self.base))
    }

    #[doc= "Get a reference to the value of field `durable_object_namespaces` after provisioning.\nDurable Object namespaces used for Pages Functions."]
    pub fn durable_object_namespaces(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.durable_object_namespaces", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_variables` after provisioning.\nEnvironment variables for Pages Functions."]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc= "Get a reference to the value of field `fail_open` after provisioning.\nFail open used for Pages Functions. Defaults to `false`."]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc= "Get a reference to the value of field `kv_namespaces` after provisioning.\nKV namespaces used for Pages Functions."]
    pub fn kv_namespaces(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.kv_namespaces", self.base))
    }

    #[doc= "Get a reference to the value of field `r2_buckets` after provisioning.\nR2 Buckets used for Pages Functions."]
    pub fn r2_buckets(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.r2_buckets", self.base))
    }

    #[doc= "Get a reference to the value of field `secrets` after provisioning.\nEncrypted environment variables for Pages Functions."]
    pub fn secrets(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.secrets", self.base))
    }

    #[doc= "Get a reference to the value of field `usage_model` after provisioning.\nUsage model used for Pages Functions. Defaults to `bundled`."]
    pub fn usage_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.base))
    }
}

#[derive(Serialize, Default)]
struct PagesProjectDeploymentConfigsElDynamic {
    preview: Option<DynamicBlock<PagesProjectDeploymentConfigsElPreviewEl>>,
    production: Option<DynamicBlock<PagesProjectDeploymentConfigsElProductionEl>>,
}

#[derive(Serialize)]
pub struct PagesProjectDeploymentConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<Vec<PagesProjectDeploymentConfigsElPreviewEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    production: Option<Vec<PagesProjectDeploymentConfigsElProductionEl>>,
    dynamic: PagesProjectDeploymentConfigsElDynamic,
}

impl PagesProjectDeploymentConfigsEl {
    #[doc= "Set the field `preview`.\n"]
    pub fn set_preview(mut self, v: impl Into<BlockAssignable<PagesProjectDeploymentConfigsElPreviewEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.preview = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.preview = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `production`.\n"]
    pub fn set_production(mut self, v: impl Into<BlockAssignable<PagesProjectDeploymentConfigsElProductionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.production = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.production = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for PagesProjectDeploymentConfigsEl {
    type O = BlockAssignable<PagesProjectDeploymentConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct PagesProjectDeploymentConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PagesProjectDeploymentConfigsElRef {
    fn new(shared: StackShared, base: String) -> PagesProjectDeploymentConfigsElRef {
        PagesProjectDeploymentConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectDeploymentConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `preview` after provisioning.\n"]
    pub fn preview(&self) -> ListRef<PagesProjectDeploymentConfigsElPreviewElRef> {
        ListRef::new(self.shared().clone(), format!("{}.preview", self.base))
    }

    #[doc= "Get a reference to the value of field `production` after provisioning.\n"]
    pub fn production(&self) -> ListRef<PagesProjectDeploymentConfigsElProductionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.production", self.base))
    }
}

#[derive(Serialize)]
pub struct PagesProjectSourceElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deployments_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pr_comments_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview_branch_excludes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview_branch_includes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview_deployment_setting: Option<PrimField<String>>,
    production_branch: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    production_deployment_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_name: Option<PrimField<String>>,
}

impl PagesProjectSourceElConfigEl {
    #[doc= "Set the field `deployments_enabled`.\nToggle deployments on this repo. Defaults to `true`."]
    pub fn set_deployments_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deployments_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `owner`.\nProject owner username."]
    pub fn set_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner = Some(v.into());
        self
    }

    #[doc= "Set the field `pr_comments_enabled`.\nEnable Pages to comment on Pull Requests. Defaults to `true`."]
    pub fn set_pr_comments_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.pr_comments_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `preview_branch_excludes`.\nBranches will be excluded from automatic deployment."]
    pub fn set_preview_branch_excludes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.preview_branch_excludes = Some(v.into());
        self
    }

    #[doc= "Set the field `preview_branch_includes`.\nBranches will be included for automatic deployment."]
    pub fn set_preview_branch_includes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.preview_branch_includes = Some(v.into());
        self
    }

    #[doc= "Set the field `preview_deployment_setting`.\nPreview Deployment Setting. Defaults to `all`."]
    pub fn set_preview_deployment_setting(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preview_deployment_setting = Some(v.into());
        self
    }

    #[doc= "Set the field `production_deployment_enabled`.\nEnable production deployments. Defaults to `true`."]
    pub fn set_production_deployment_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.production_deployment_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_name`.\nProject repository name."]
    pub fn set_repo_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_name = Some(v.into());
        self
    }
}

impl ToListMappable for PagesProjectSourceElConfigEl {
    type O = BlockAssignable<PagesProjectSourceElConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesProjectSourceElConfigEl {
    #[doc= "Project production branch name."]
    pub production_branch: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for PagesProjectSourceElConfigElRef {
    fn new(shared: StackShared, base: String) -> PagesProjectSourceElConfigElRef {
        PagesProjectSourceElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectSourceElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `deployments_enabled` after provisioning.\nToggle deployments on this repo. Defaults to `true`."]
    pub fn deployments_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployments_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\nProject owner username."]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc= "Get a reference to the value of field `pr_comments_enabled` after provisioning.\nEnable Pages to comment on Pull Requests. Defaults to `true`."]
    pub fn pr_comments_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pr_comments_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `preview_branch_excludes` after provisioning.\nBranches will be excluded from automatic deployment."]
    pub fn preview_branch_excludes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.preview_branch_excludes", self.base))
    }

    #[doc= "Get a reference to the value of field `preview_branch_includes` after provisioning.\nBranches will be included for automatic deployment."]
    pub fn preview_branch_includes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.preview_branch_includes", self.base))
    }

    #[doc= "Get a reference to the value of field `preview_deployment_setting` after provisioning.\nPreview Deployment Setting. Defaults to `all`."]
    pub fn preview_deployment_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preview_deployment_setting", self.base))
    }

    #[doc= "Get a reference to the value of field `production_branch` after provisioning.\nProject production branch name."]
    pub fn production_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.production_branch", self.base))
    }

    #[doc= "Get a reference to the value of field `production_deployment_enabled` after provisioning.\nEnable production deployments. Defaults to `true`."]
    pub fn production_deployment_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.production_deployment_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_name` after provisioning.\nProject repository name."]
    pub fn repo_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct PagesProjectSourceElDynamic {
    config: Option<DynamicBlock<PagesProjectSourceElConfigEl>>,
}

#[derive(Serialize)]
pub struct PagesProjectSourceEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<PagesProjectSourceElConfigEl>>,
    dynamic: PagesProjectSourceElDynamic,
}

impl PagesProjectSourceEl {
    #[doc= "Set the field `type_`.\nProject host type."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `config`.\n"]
    pub fn set_config(mut self, v: impl Into<BlockAssignable<PagesProjectSourceElConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for PagesProjectSourceEl {
    type O = BlockAssignable<PagesProjectSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct PagesProjectSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PagesProjectSourceElRef {
    fn new(shared: StackShared, base: String) -> PagesProjectSourceElRef {
        PagesProjectSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesProjectSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nProject host type."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<PagesProjectSourceElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize, Default)]
struct PagesProjectDynamic {
    build_config: Option<DynamicBlock<PagesProjectBuildConfigEl>>,
    deployment_configs: Option<DynamicBlock<PagesProjectDeploymentConfigsEl>>,
    source: Option<DynamicBlock<PagesProjectSourceEl>>,
}
