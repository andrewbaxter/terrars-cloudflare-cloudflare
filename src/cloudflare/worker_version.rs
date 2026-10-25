#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WorkerVersionData_ {
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
    annotations: std::option::Option<WorkerVersion_Annotations>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assets: std::option::Option<WorkerVersion_Assets>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bindings: std::option::Option<std::vec::Vec<WorkerVersion_Bindings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_options: std::option::Option<WorkerVersion_CacheOptions>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_flags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std::option::Option<std::vec::Vec<WorkerVersion_Containers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exports: std::option::Option<std::collections::HashMap<std::string::String, WorkerVersion_Exports>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<WorkerVersion_Limits>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_module: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    migrations: std::option::Option<WorkerVersion_Migrations>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modules: std::option::Option<std::vec::Vec<WorkerVersion_Modules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_dependencies: std::option::Option<std::vec::Vec<WorkerVersion_PackageDependencies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<WorkerVersion_Placement>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_model: std::option::Option<terrars::PrimField<std::string::String>>,
    worker_id: terrars::PrimField<std::string::String>,
}

struct WorkerVersion_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkerVersionData_>,
}

#[derive(Clone)]
pub struct WorkerVersion(std::rc::Rc<WorkerVersion_>);

impl WorkerVersion {
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

    #[doc = "Set the field `annotations`.\nMetadata about the version."]
    pub fn set_annotations(self, v: impl std::convert::Into<WorkerVersion_Annotations>) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `assets`.\nConfiguration for assets within a Worker.\n\n[`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers) and\n[`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/) files should be\nincluded as modules named `_headers` and `_redirects` with content type `text/plain`."]
    pub fn set_assets(self, v: impl std::convert::Into<WorkerVersion_Assets>) -> Self {
        self.0.data.borrow_mut().assets = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bindings`.\nList of bindings attached to a Worker. You can find more about bindings on our docs: https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings."]
    pub fn set_bindings(self, v: impl std::convert::Into<std::vec::Vec<WorkerVersion_Bindings>>) -> Self {
        self.0.data.borrow_mut().bindings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache_options`.\nGlobal CacheW configuration for the Worker. When caching is on,\nthe platform provisions a `cloudflare.app` zone for the Worker.\nA `type: worker` entry in the `exports` map can override this\nvalue for a single entrypoint."]
    pub fn set_cache_options(self, v: impl std::convert::Into<WorkerVersion_CacheOptions>) -> Self {
        self.0.data.borrow_mut().cache_options = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `compatibility_date`.\nDate indicating targeted support in the Workers runtime. Backwards incompatible fixes to the runtime following this date will not affect this Worker."]
    pub fn set_compatibility_date(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().compatibility_date = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `compatibility_flags`.\nFlags that enable or disable certain features in the Workers runtime. Used to enable upcoming features or opt in or out of specific changes not included in a `compatibility_date`."]
    pub fn set_compatibility_flags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().compatibility_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `containers`.\nList of containers attached to a Worker. Containers can only be attached to Durable Object classes of this Worker script."]
    pub fn set_containers(self, v: impl std::convert::Into<std::vec::Vec<WorkerVersion_Containers>>) -> Self {
        self.0.data.borrow_mut().containers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deploy`.\nIf true, a deployment will be created that sends 100% of traffic to the new version."]
    pub fn set_deploy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deploy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exports`.\nDeclarative exports for the version, including Durable Object\nclasses (with their `storage` backend) and named Worker\nentrypoints. On reads, tombstoned lifecycle entries are\nomitted, so only live exports (`created` and\n`expecting-transfer`) are returned. `exports` and `migrations`\nare mutually exclusive on upload."]
    pub fn set_exports(
        self,
        v: impl std::convert::Into<std::collections::HashMap<std::string::String, WorkerVersion_Exports>>,
    ) -> Self {
        self.0.data.borrow_mut().exports = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include`.\nWhether to include the `modules` property of the version in the response, which contains code and sourcemap content and may add several megabytes to the response size.\nAvailable values: \"modules\"."]
    pub fn set_include(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().include = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limits`.\nResource limits enforced at runtime."]
    pub fn set_limits(self, v: impl std::convert::Into<WorkerVersion_Limits>) -> Self {
        self.0.data.borrow_mut().limits = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_module`.\nThe name of the main module in the `modules` array (e.g. the name of the module that exports a `fetch` handler)."]
    pub fn set_main_module(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().main_module = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `migrations`.\nMigrations for Durable Objects associated with the version. Migrations are applied when the version is deployed."]
    pub fn set_migrations(self, v: impl std::convert::Into<WorkerVersion_Migrations>) -> Self {
        self.0.data.borrow_mut().migrations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `modules`.\nCode, sourcemaps, and other content used at runtime.\n\nThis includes [`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers) and\n[`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/) files used to configure\n[Static Assets](https://developers.cloudflare.com/workers/static-assets/). `_headers` and `_redirects` files should be\nincluded as modules named `_headers` and `_redirects` with content type `text/plain`."]
    pub fn set_modules(self, v: impl std::convert::Into<std::vec::Vec<WorkerVersion_Modules>>) -> Self {
        self.0.data.borrow_mut().modules = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `package_dependencies`.\nThe list of npm packages that were installed and used when this Worker\nversion was built."]
    pub fn set_package_dependencies(
        self,
        v: impl std::convert::Into<std::vec::Vec<WorkerVersion_PackageDependencies>>,
    ) -> Self {
        self.0.data.borrow_mut().package_dependencies = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `placement`.\nConfiguration for [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement). Specify mode='smart' for Smart Placement, or one of region/hostname/host."]
    pub fn set_placement(self, v: impl std::convert::Into<WorkerVersion_Placement>) -> Self {
        self.0.data.borrow_mut().placement = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `usage_model`.\nUsage model for the version.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn set_usage_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().usage_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nMetadata about the version."]
    pub fn annotations(&self) -> WorkerVersion_AnnotationsRef {
        WorkerVersion_AnnotationsRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `assets` after provisioning.\nConfiguration for assets within a Worker.\n\n[`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers) and\n[`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/) files should be\nincluded as modules named `_headers` and `_redirects` with content type `text/plain`."]
    pub fn assets(&self) -> WorkerVersion_AssetsRef {
        WorkerVersion_AssetsRef::new(self.shared().clone(), format!("{}.assets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `author_email` after provisioning.\nEmail of the user who created the version."]
    pub fn author_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `author_id` after provisioning.\nIdentifier of the user who created the version."]
    pub fn author_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bindings` after provisioning.\nList of bindings attached to a Worker. You can find more about bindings on our docs: https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings."]
    pub fn bindings(&self) -> terrars::ListRef<WorkerVersion_BindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bindings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cache_options` after provisioning.\nGlobal CacheW configuration for the Worker. When caching is on,\nthe platform provisions a `cloudflare.app` zone for the Worker.\nA `type: worker` entry in the `exports` map can override this\nvalue for a single entrypoint."]
    pub fn cache_options(&self) -> WorkerVersion_CacheOptionsRef {
        WorkerVersion_CacheOptionsRef::new(self.shared().clone(), format!("{}.cache_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_date` after provisioning.\nDate indicating targeted support in the Workers runtime. Backwards incompatible fixes to the runtime following this date will not affect this Worker."]
    pub fn compatibility_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_flags` after provisioning.\nFlags that enable or disable certain features in the Workers runtime. Used to enable upcoming features or opt in or out of specific changes not included in a `compatibility_date`."]
    pub fn compatibility_flags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `containers` after provisioning.\nList of containers attached to a Worker. Containers can only be attached to Durable Object classes of this Worker script."]
    pub fn containers(&self) -> terrars::SetRef<WorkerVersion_ContainersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.containers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the version was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deploy` after provisioning.\nIf true, a deployment will be created that sends 100% of traffic to the new version."]
    pub fn deploy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exports` after provisioning.\nDeclarative exports for the version, including Durable Object\nclasses (with their `storage` backend) and named Worker\nentrypoints. On reads, tombstoned lifecycle entries are\nomitted, so only live exports (`created` and\n`expecting-transfer`) are returned. `exports` and `migrations`\nare mutually exclusive on upload."]
    pub fn exports(&self) -> terrars::RecRef<WorkerVersion_ExportsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.exports", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exports_reconciliation` after provisioning.\nSummary of the declarative exports reconciliation that ran on this upload. Populated only when the uploaded metadata included an `exports` block. Durable Object entries drive reconciliation; `type: worker` entries do not contribute to this summary."]
    pub fn exports_reconciliation(&self) -> WorkerVersion_ExportsReconciliationRef {
        WorkerVersion_ExportsReconciliationRef::new(
            self.shared().clone(),
            format!("{}.exports_reconciliation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nVersion identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nWhether to include the `modules` property of the version in the response, which contains code and sourcemap content and may add several megabytes to the response size.\nAvailable values: \"modules\"."]
    pub fn include(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\nResource limits enforced at runtime."]
    pub fn limits(&self) -> WorkerVersion_LimitsRef {
        WorkerVersion_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `main_module` after provisioning.\nThe name of the main module in the `modules` array (e.g. the name of the module that exports a `fetch` handler)."]
    pub fn main_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_module", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `main_script_base64` after provisioning.\nThe base64-encoded main script content. This is only returned for service worker syntax workers (not ES modules). Used when importing existing workers that use the older service worker syntax."]
    pub fn main_script_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_script_base64", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migration_tag` after provisioning.\nDurable Object migration tag. Set when the version is deployed. Omitted if the version has not been deployed or the Worker does not use Durable Objects."]
    pub fn migration_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.migration_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migrations` after provisioning.\nMigrations for Durable Objects associated with the version. Migrations are applied when the version is deployed."]
    pub fn migrations(&self) -> WorkerVersion_MigrationsRef {
        WorkerVersion_MigrationsRef::new(self.shared().clone(), format!("{}.migrations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modules` after provisioning.\nCode, sourcemaps, and other content used at runtime.\n\nThis includes [`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers) and\n[`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/) files used to configure\n[Static Assets](https://developers.cloudflare.com/workers/static-assets/). `_headers` and `_redirects` files should be\nincluded as modules named `_headers` and `_redirects` with content type `text/plain`."]
    pub fn modules(&self) -> terrars::SetRef<WorkerVersion_ModulesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.modules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `number` after provisioning.\nThe integer version number, starting from one."]
    pub fn number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `package_dependencies` after provisioning.\nThe list of npm packages that were installed and used when this Worker\nversion was built."]
    pub fn package_dependencies(&self) -> terrars::ListRef<WorkerVersion_PackageDependenciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.package_dependencies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nConfiguration for [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement). Specify mode='smart' for Smart Placement, or one of region/hostname/host."]
    pub fn placement(&self) -> WorkerVersion_PlacementRef {
        WorkerVersion_PlacementRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nThe client used to create the version."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `startup_time_ms` after provisioning.\nTime in milliseconds spent on [Worker startup](https://developers.cloudflare.com/workers/platform/limits/#worker-startup-time)."]
    pub fn startup_time_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_time_ms", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urls` after provisioning.\nAll routable URLs that always point to this version. Does not include alias URLs, since aliases can be updated to point to a different version."]
    pub fn urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nUsage model for the version.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `worker_id` after provisioning.\nIdentifier for the Worker, which can be ID or name."]
    pub fn worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_id", self.extract_ref()))
    }
}

impl terrars::Referable for WorkerVersion {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkerVersion { }

impl terrars::ToListMappable for WorkerVersion {
    type O = terrars::ListRef<WorkerVersionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkerVersion_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_worker_version".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkerVersion {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Identifier for the Worker, which can be ID or name."]
    pub worker_id: terrars::PrimField<std::string::String>,
}

impl BuildWorkerVersion {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkerVersion {
        let out = WorkerVersion(std::rc::Rc::new(WorkerVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkerVersionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                annotations: core::default::Default::default(),
                assets: core::default::Default::default(),
                bindings: core::default::Default::default(),
                cache_options: core::default::Default::default(),
                compatibility_date: core::default::Default::default(),
                compatibility_flags: core::default::Default::default(),
                containers: core::default::Default::default(),
                deploy: core::default::Default::default(),
                exports: core::default::Default::default(),
                include: core::default::Default::default(),
                limits: core::default::Default::default(),
                main_module: core::default::Default::default(),
                migrations: core::default::Default::default(),
                modules: core::default::Default::default(),
                package_dependencies: core::default::Default::default(),
                placement: core::default::Default::default(),
                usage_model: core::default::Default::default(),
                worker_id: self.worker_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkerVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkerVersionRef {
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

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nMetadata about the version."]
    pub fn annotations(&self) -> WorkerVersion_AnnotationsRef {
        WorkerVersion_AnnotationsRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `assets` after provisioning.\nConfiguration for assets within a Worker.\n\n[`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers) and\n[`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/) files should be\nincluded as modules named `_headers` and `_redirects` with content type `text/plain`."]
    pub fn assets(&self) -> WorkerVersion_AssetsRef {
        WorkerVersion_AssetsRef::new(self.shared().clone(), format!("{}.assets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `author_email` after provisioning.\nEmail of the user who created the version."]
    pub fn author_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `author_id` after provisioning.\nIdentifier of the user who created the version."]
    pub fn author_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bindings` after provisioning.\nList of bindings attached to a Worker. You can find more about bindings on our docs: https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings."]
    pub fn bindings(&self) -> terrars::ListRef<WorkerVersion_BindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bindings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cache_options` after provisioning.\nGlobal CacheW configuration for the Worker. When caching is on,\nthe platform provisions a `cloudflare.app` zone for the Worker.\nA `type: worker` entry in the `exports` map can override this\nvalue for a single entrypoint."]
    pub fn cache_options(&self) -> WorkerVersion_CacheOptionsRef {
        WorkerVersion_CacheOptionsRef::new(self.shared().clone(), format!("{}.cache_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_date` after provisioning.\nDate indicating targeted support in the Workers runtime. Backwards incompatible fixes to the runtime following this date will not affect this Worker."]
    pub fn compatibility_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compatibility_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compatibility_flags` after provisioning.\nFlags that enable or disable certain features in the Workers runtime. Used to enable upcoming features or opt in or out of specific changes not included in a `compatibility_date`."]
    pub fn compatibility_flags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.compatibility_flags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `containers` after provisioning.\nList of containers attached to a Worker. Containers can only be attached to Durable Object classes of this Worker script."]
    pub fn containers(&self) -> terrars::SetRef<WorkerVersion_ContainersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.containers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the version was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deploy` after provisioning.\nIf true, a deployment will be created that sends 100% of traffic to the new version."]
    pub fn deploy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exports` after provisioning.\nDeclarative exports for the version, including Durable Object\nclasses (with their `storage` backend) and named Worker\nentrypoints. On reads, tombstoned lifecycle entries are\nomitted, so only live exports (`created` and\n`expecting-transfer`) are returned. `exports` and `migrations`\nare mutually exclusive on upload."]
    pub fn exports(&self) -> terrars::RecRef<WorkerVersion_ExportsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.exports", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exports_reconciliation` after provisioning.\nSummary of the declarative exports reconciliation that ran on this upload. Populated only when the uploaded metadata included an `exports` block. Durable Object entries drive reconciliation; `type: worker` entries do not contribute to this summary."]
    pub fn exports_reconciliation(&self) -> WorkerVersion_ExportsReconciliationRef {
        WorkerVersion_ExportsReconciliationRef::new(
            self.shared().clone(),
            format!("{}.exports_reconciliation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nVersion identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nWhether to include the `modules` property of the version in the response, which contains code and sourcemap content and may add several megabytes to the response size.\nAvailable values: \"modules\"."]
    pub fn include(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\nResource limits enforced at runtime."]
    pub fn limits(&self) -> WorkerVersion_LimitsRef {
        WorkerVersion_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `main_module` after provisioning.\nThe name of the main module in the `modules` array (e.g. the name of the module that exports a `fetch` handler)."]
    pub fn main_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_module", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `main_script_base64` after provisioning.\nThe base64-encoded main script content. This is only returned for service worker syntax workers (not ES modules). Used when importing existing workers that use the older service worker syntax."]
    pub fn main_script_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_script_base64", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migration_tag` after provisioning.\nDurable Object migration tag. Set when the version is deployed. Omitted if the version has not been deployed or the Worker does not use Durable Objects."]
    pub fn migration_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.migration_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migrations` after provisioning.\nMigrations for Durable Objects associated with the version. Migrations are applied when the version is deployed."]
    pub fn migrations(&self) -> WorkerVersion_MigrationsRef {
        WorkerVersion_MigrationsRef::new(self.shared().clone(), format!("{}.migrations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modules` after provisioning.\nCode, sourcemaps, and other content used at runtime.\n\nThis includes [`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers) and\n[`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/) files used to configure\n[Static Assets](https://developers.cloudflare.com/workers/static-assets/). `_headers` and `_redirects` files should be\nincluded as modules named `_headers` and `_redirects` with content type `text/plain`."]
    pub fn modules(&self) -> terrars::SetRef<WorkerVersion_ModulesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.modules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `number` after provisioning.\nThe integer version number, starting from one."]
    pub fn number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `package_dependencies` after provisioning.\nThe list of npm packages that were installed and used when this Worker\nversion was built."]
    pub fn package_dependencies(&self) -> terrars::ListRef<WorkerVersion_PackageDependenciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.package_dependencies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nConfiguration for [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement). Specify mode='smart' for Smart Placement, or one of region/hostname/host."]
    pub fn placement(&self) -> WorkerVersion_PlacementRef {
        WorkerVersion_PlacementRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nThe client used to create the version."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `startup_time_ms` after provisioning.\nTime in milliseconds spent on [Worker startup](https://developers.cloudflare.com/workers/platform/limits/#worker-startup-time)."]
    pub fn startup_time_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_time_ms", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urls` after provisioning.\nAll routable URLs that always point to this version. Does not include alias URLs, since aliases can be updated to point to a different version."]
    pub fn urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nUsage model for the version.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `worker_id` after provisioning.\nIdentifier for the Worker, which can be ID or name."]
    pub fn worker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_Annotations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workers_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workers_tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkerVersion_Annotations {
    #[doc =
        "Set the field `workers_message`.\nHuman-readable message about the version. Truncated to 1000 bytes if longer."]
    pub fn set_workers_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workers_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workers_tag`.\nUser-provided identifier for the version. Maximum 100 bytes."]
    pub fn set_workers_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workers_tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Annotations {
    type O = terrars::BlockAssignable<WorkerVersion_Annotations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Annotations {}

impl BuildWorkerVersion_Annotations {
    pub fn build(self) -> WorkerVersion_Annotations {
        WorkerVersion_Annotations {
            workers_message: core::default::Default::default(),
            workers_tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_AnnotationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_AnnotationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_AnnotationsRef {
        WorkerVersion_AnnotationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_AnnotationsRef {
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
pub struct WorkerVersion_Assets_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    html_handling: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_found_handling: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_worker_first: std::option::Option<terrars::PrimField<serde_json::Value>>,
}

impl WorkerVersion_Assets_Config {
    #[doc =
        "Set the field `base_path`.\nThe public URL path prefix under which assets are served. A null request value resets it to `/`; responses represent the root as `/`. All versions in a gradual deployment must use the same canonical value. To change it, first deploy the version containing the change at 100%."]
    pub fn set_base_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.base_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `html_handling`.\nDetermines the redirects and rewrites of requests for HTML content.\nAvailable values: \"auto-trailing-slash\", \"force-trailing-slash\", \"drop-trailing-slash\", \"none\"."]
    pub fn set_html_handling(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.html_handling = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `not_found_handling`.\nDetermines the response when a request does not match a static asset, and there is no Worker script.\nAvailable values: \"none\", \"404-page\", \"single-page-application\"."]
    pub fn set_not_found_handling(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.not_found_handling = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `run_worker_first`.\nWhen a boolean true, requests will always invoke the Worker script. Otherwise, attempt to serve an asset matching the request, falling back to the Worker script. When a list of strings, contains path rules to control routing to either the Worker or assets. Glob (*) and negative (!) rules are supported. Rules must start with either '/' or '!/'. At least one non-negative rule must be provided, and negative rules have higher precedence than non-negative rules."]
    pub fn set_run_worker_first(mut self, v: impl std::convert::Into<terrars::PrimField<serde_json::Value>>) -> Self {
        self.run_worker_first = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Assets_Config {
    type O = terrars::BlockAssignable<WorkerVersion_Assets_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Assets_Config {}

impl BuildWorkerVersion_Assets_Config {
    pub fn build(self) -> WorkerVersion_Assets_Config {
        WorkerVersion_Assets_Config {
            base_path: core::default::Default::default(),
            html_handling: core::default::Default::default(),
            not_found_handling: core::default::Default::default(),
            run_worker_first: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Assets_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Assets_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_Assets_ConfigRef {
        WorkerVersion_Assets_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Assets_ConfigRef {
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
        "Get a reference to the value of field `run_worker_first` after provisioning.\nWhen a boolean true, requests will always invoke the Worker script. Otherwise, attempt to serve an asset matching the request, falling back to the Worker script. When a list of strings, contains path rules to control routing to either the Worker or assets. Glob (*) and negative (!) rules are supported. Rules must start with either '/' or '!/'. At least one non-negative rule must be provided, and negative rules have higher precedence than non-negative rules."]
    pub fn run_worker_first(&self) -> terrars::PrimExpr<serde_json::Value> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_worker_first", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_Assets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<WorkerVersion_Assets_Config>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkerVersion_Assets {
    #[doc = "Set the field `config`.\nConfiguration for assets within a Worker."]
    pub fn set_config(mut self, v: impl std::convert::Into<WorkerVersion_Assets_Config>) -> Self {
        self.config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `directory`.\nPath to the directory containing asset files to upload."]
    pub fn set_directory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jwt`.\nToken provided upon successful upload of all files from a registered manifest."]
    pub fn set_jwt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Assets {
    type O = terrars::BlockAssignable<WorkerVersion_Assets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Assets {}

impl BuildWorkerVersion_Assets {
    pub fn build(self) -> WorkerVersion_Assets {
        WorkerVersion_Assets {
            config: core::default::Default::default(),
            directory: core::default::Default::default(),
            jwt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_AssetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_AssetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_AssetsRef {
        WorkerVersion_AssetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_AssetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asset_manifest_sha256` after provisioning.\nThe SHA-256 hash of the asset manifest of files to upload."]
    pub fn asset_manifest_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_manifest_sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nConfiguration for assets within a Worker."]
    pub fn config(&self) -> WorkerVersion_Assets_ConfigRef {
        WorkerVersion_Assets_ConfigRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `directory` after provisioning.\nPath to the directory containing asset files to upload."]
    pub fn directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jwt` after provisioning.\nToken provided upon successful upload of all files from a registered manifest."]
    pub fn jwt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwt", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_Bindings_Outbound_Params {
    name: terrars::PrimField<std::string::String>,
}

impl WorkerVersion_Bindings_Outbound_Params { }

impl terrars::ToListMappable for WorkerVersion_Bindings_Outbound_Params {
    type O = terrars::BlockAssignable<WorkerVersion_Bindings_Outbound_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Bindings_Outbound_Params {
    #[doc = "Name of the parameter."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWorkerVersion_Bindings_Outbound_Params {
    pub fn build(self) -> WorkerVersion_Bindings_Outbound_Params {
        WorkerVersion_Bindings_Outbound_Params { name: self.name }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Bindings_Outbound_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Bindings_Outbound_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_Bindings_Outbound_ParamsRef {
        WorkerVersion_Bindings_Outbound_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Bindings_Outbound_ParamsRef {
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
pub struct WorkerVersion_Bindings_Outbound_Worker {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkerVersion_Bindings_Outbound_Worker {
    #[doc = "Set the field `entrypoint`.\nEntrypoint to invoke on the outbound worker."]
    pub fn set_entrypoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entrypoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\nEnvironment of the outbound worker."]
    pub fn set_environment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\nName of the outbound worker."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Bindings_Outbound_Worker {
    type O = terrars::BlockAssignable<WorkerVersion_Bindings_Outbound_Worker>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Bindings_Outbound_Worker {}

impl BuildWorkerVersion_Bindings_Outbound_Worker {
    pub fn build(self) -> WorkerVersion_Bindings_Outbound_Worker {
        WorkerVersion_Bindings_Outbound_Worker {
            entrypoint: core::default::Default::default(),
            environment: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Bindings_Outbound_WorkerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Bindings_Outbound_WorkerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_Bindings_Outbound_WorkerRef {
        WorkerVersion_Bindings_Outbound_WorkerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Bindings_Outbound_WorkerRef {
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
pub struct WorkerVersion_Bindings_Outbound {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<WorkerVersion_Bindings_Outbound_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker: std::option::Option<WorkerVersion_Bindings_Outbound_Worker>,
}

impl WorkerVersion_Bindings_Outbound {
    #[doc =
        "Set the field `params`.\nPass information from the Dispatch Worker to the Outbound Worker through the parameters."]
    pub fn set_params(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<WorkerVersion_Bindings_Outbound_Params>>,
    ) -> Self {
        self.params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker`.\nOutbound worker."]
    pub fn set_worker(mut self, v: impl std::convert::Into<WorkerVersion_Bindings_Outbound_Worker>) -> Self {
        self.worker = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Bindings_Outbound {
    type O = terrars::BlockAssignable<WorkerVersion_Bindings_Outbound>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Bindings_Outbound {}

impl BuildWorkerVersion_Bindings_Outbound {
    pub fn build(self) -> WorkerVersion_Bindings_Outbound {
        WorkerVersion_Bindings_Outbound {
            params: core::default::Default::default(),
            worker: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Bindings_OutboundRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Bindings_OutboundRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_Bindings_OutboundRef {
        WorkerVersion_Bindings_OutboundRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Bindings_OutboundRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nPass information from the Dispatch Worker to the Outbound Worker through the parameters."]
    pub fn params(&self) -> terrars::ListRef<WorkerVersion_Bindings_Outbound_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }

    #[doc = "Get a reference to the value of field `worker` after provisioning.\nOutbound worker."]
    pub fn worker(&self) -> WorkerVersion_Bindings_Outbound_WorkerRef {
        WorkerVersion_Bindings_Outbound_WorkerRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_Bindings_Retry {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_ms: std::option::Option<terrars::PrimField<f64>>,
}

impl WorkerVersion_Bindings_Retry {
    #[doc =
        "Set the field `max_attempts`.\nMaximum number of retries after the initial request, not the total number of attempts. Defaults to 4. Zero disables retries."]
    pub fn set_max_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_ms`.\nRetry timeout in milliseconds, measured from the start of the call. No retry starts after it expires, and a retry still running when it expires is cancelled. This is not a request timeout; it does not limit the initial request, which is still subject to any timeouts set by your Worker. Defaults to 10000."]
    pub fn set_timeout_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_ms = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Bindings_Retry {
    type O = terrars::BlockAssignable<WorkerVersion_Bindings_Retry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Bindings_Retry {}

impl BuildWorkerVersion_Bindings_Retry {
    pub fn build(self) -> WorkerVersion_Bindings_Retry {
        WorkerVersion_Bindings_Retry {
            max_attempts: core::default::Default::default(),
            timeout_ms: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Bindings_RetryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Bindings_RetryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_Bindings_RetryRef {
        WorkerVersion_Bindings_RetryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Bindings_RetryRef {
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
pub struct WorkerVersion_Bindings_Simple {
    limit: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mitigation_timeout: std::option::Option<terrars::PrimField<f64>>,
    period: terrars::PrimField<f64>,
}

impl WorkerVersion_Bindings_Simple {
    #[doc =
        "Set the field `mitigation_timeout`.\nDuration in seconds to apply the mitigation action after the rate limit is exceeded. Valid values are 0 (disabled), 10, or multiples of 60 up to 86400. Must be greater than or equal to the period when non-zero."]
    pub fn set_mitigation_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mitigation_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Bindings_Simple {
    type O = terrars::BlockAssignable<WorkerVersion_Bindings_Simple>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Bindings_Simple {
    #[doc = "The limit (requests per period)."]
    pub limit: terrars::PrimField<f64>,
    #[doc = "The period in seconds."]
    pub period: terrars::PrimField<f64>,
}

impl BuildWorkerVersion_Bindings_Simple {
    pub fn build(self) -> WorkerVersion_Bindings_Simple {
        WorkerVersion_Bindings_Simple {
            limit: self.limit,
            mitigation_timeout: core::default::Default::default(),
            period: self.period,
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Bindings_SimpleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Bindings_SimpleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_Bindings_SimpleRef {
        WorkerVersion_Bindings_SimpleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Bindings_SimpleRef {
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
pub struct WorkerVersion_Bindings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_sender_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    class_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dispatch_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jurisdiction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_base64: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_jwk: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    old_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound: std::option::Option<WorkerVersion_Bindings_Outbound>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    part: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry: std::option::Option<WorkerVersion_Bindings_Retry>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    simple: std::option::Option<WorkerVersion_Bindings_Simple>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    store_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usages: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkerVersion_Bindings {
    #[doc =
        "Set the field `algorithm`.\nAlgorithm-specific key parameters. [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#algorithm)."]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_destination_addresses`.\nList of allowed destination addresses."]
    pub fn set_allowed_destination_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_destination_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_sender_addresses`.\nList of allowed sender addresses."]
    pub fn set_allowed_sender_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_sender_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_id`.\nID of the Flagship app to bind to for feature flag evaluation."]
    pub fn set_app_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.app_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_name`.\nR2 bucket to bind to."]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_id`.\nIdentifier of the certificate to bind to."]
    pub fn set_certificate_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `class_name`.\nThe exported class name of the Durable Object."]
    pub fn set_class_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.class_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_id`.\nIdentifier of the D1 database to bind to."]
    pub fn set_database_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dataset`.\nThe name of the dataset to bind to."]
    pub fn set_dataset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_address`.\nDestination address for the email."]
    pub fn set_destination_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dispatch_namespace`.\nThe dispatch namespace the Durable Object script belongs to."]
    pub fn set_dispatch_namespace(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.dispatch_namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entrypoint`.\nEntrypoint to invoke on the target Worker."]
    pub fn set_entrypoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entrypoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\nThe environment of the script_name to bind to."]
    pub fn set_environment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `format`.\nData format of the key. [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#format).\nAvailable values: \"raw\", \"pkcs8\", \"spki\", \"jwk\"."]
    pub fn set_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nIdentifier of the D1 database to bind to."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity`.\nEnables Gateway identity for the binding. Requires network_id to be \"cf1:network\" and cannot be combined with tunnel_id.\nAvailable values: \"runtime-email-alpha\"."]
    pub fn set_identity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_name`.\nName of the Vectorize index to bind to."]
    pub fn set_index_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instance_name`.\nThe user-chosen instance name. Must exist at deploy time. The worker can search, chat, update, and manage items/jobs on this instance."]
    pub fn set_instance_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `json`.\nJSON data to use."]
    pub fn set_json(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jurisdiction`.\nThe [jurisdiction](https://developers.cloudflare.com/r2/reference/data-location/#jurisdictional-restrictions) of the R2 bucket.\nAvailable values: \"eu\", \"fedramp\", \"fedramp-high\", \"us\"."]
    pub fn set_jurisdiction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jurisdiction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_base64`.\nBase64-encoded key data. Required if `format` is \"raw\", \"pkcs8\", or \"spki\"."]
    pub fn set_key_base64(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_base64 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_jwk`.\nKey data in [JSON Web Key](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#json_web_key) format. Required if `format` is \"jwk\"."]
    pub fn set_key_jwk(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_jwk = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `namespace`.\nThe namespace the instance belongs to. Defaults to \"default\" if omitted. Customers who don't use namespaces can simply omit this field."]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace_id`.\nNamespace identifier tag."]
    pub fn set_namespace_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_id`.\nIdentifier of the network to bind to. Only \"cf1:network\" is currently supported. Mutually exclusive with tunnel_id."]
    pub fn set_network_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `old_name`.\nThe old name of the inherited binding. If set, the binding will be renamed from `old_name` to `name` in the new version. If not set, the binding will keep the same name between versions."]
    pub fn set_old_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.old_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outbound`.\nOutbound worker."]
    pub fn set_outbound(mut self, v: impl std::convert::Into<WorkerVersion_Bindings_Outbound>) -> Self {
        self.outbound = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `part`.\nThe name of the file containing the data content. Only accepted for `service worker syntax` Workers."]
    pub fn set_part(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.part = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline`.\nName of the Pipeline to bind to."]
    pub fn set_pipeline(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pipeline = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_name`.\nName of the Queue to bind to."]
    pub fn set_queue_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.queue_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry`.\nRetry policy for calls made to the Durable Object through this binding. Omitted or null properties use the runtime defaults. These limits are upper bounds and do not enable retries for otherwise ineligible calls."]
    pub fn set_retry(mut self, v: impl std::convert::Into<WorkerVersion_Bindings_Retry>) -> Self {
        self.retry = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `script_name`.\nThe script where the Durable Object is defined, if it is external to this Worker."]
    pub fn set_script_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.script_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_name`.\nName of the secret in the store."]
    pub fn set_secret_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\nName of Worker to bind to."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_id`.\nIdentifier of the VPC service to bind to."]
    pub fn set_service_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `simple`.\nThe rate limit configuration."]
    pub fn set_simple(mut self, v: impl std::convert::Into<WorkerVersion_Bindings_Simple>) -> Self {
        self.simple = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `store_id`.\nID of the store containing the secret."]
    pub fn set_store_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.store_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream`.\nID of a K2 stream owned by the account deploying the Worker."]
    pub fn set_stream(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\nThe text value to use."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_id`.\nUUID of the Cloudflare Tunnel to bind to. Mutually exclusive with network_id."]
    pub fn set_tunnel_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tunnel_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `usages`.\nAllowed operations with the key. [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#keyUsages)."]
    pub fn set_usages(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.usages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version_id`.\nIdentifier for the version to inherit the binding from, which can be the version ID or the literal \"latest\" to inherit from the latest version. Defaults to inheriting the binding from the latest version."]
    pub fn set_version_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workflow_name`.\nName of the Workflow to bind to."]
    pub fn set_workflow_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workflow_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Bindings {
    type O = terrars::BlockAssignable<WorkerVersion_Bindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Bindings {
    #[doc = "A JavaScript variable name for the binding."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The kind of resource that the binding provides.\nAvailable values: \"ai\", \"ai_search\", \"ai_search_namespace\", \"messaging\", \"analytics_engine\", \"artifacts\", \"assets\", \"browser\", \"d1\", \"data_blob\", \"dispatch_namespace\", \"durable_object_namespace\", \"hyperdrive\", \"inherit\", \"images\", \"json\", \"kv_namespace\", \"media\", \"mtls_certificate\", \"plain_text\", \"pipelines\", \"k2\", \"queue\", \"ratelimit\", \"r2_bucket\", \"secret_text\", \"send_email\", \"service\", \"text_blob\", \"vectorize\", \"version_metadata\", \"secrets_store_secret\", \"flagship\", \"secret_key\", \"workflow\", \"wasm_module\", \"vpc_service\", \"vpc_network\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWorkerVersion_Bindings {
    pub fn build(self) -> WorkerVersion_Bindings {
        WorkerVersion_Bindings {
            algorithm: core::default::Default::default(),
            allowed_destination_addresses: core::default::Default::default(),
            allowed_sender_addresses: core::default::Default::default(),
            app_id: core::default::Default::default(),
            bucket_name: core::default::Default::default(),
            certificate_id: core::default::Default::default(),
            class_name: core::default::Default::default(),
            database_id: core::default::Default::default(),
            dataset: core::default::Default::default(),
            destination_address: core::default::Default::default(),
            dispatch_namespace: core::default::Default::default(),
            entrypoint: core::default::Default::default(),
            environment: core::default::Default::default(),
            format: core::default::Default::default(),
            id: core::default::Default::default(),
            identity: core::default::Default::default(),
            index_name: core::default::Default::default(),
            instance_name: core::default::Default::default(),
            json: core::default::Default::default(),
            jurisdiction: core::default::Default::default(),
            key_base64: core::default::Default::default(),
            key_jwk: core::default::Default::default(),
            name: self.name,
            namespace: core::default::Default::default(),
            namespace_id: core::default::Default::default(),
            network_id: core::default::Default::default(),
            old_name: core::default::Default::default(),
            outbound: core::default::Default::default(),
            part: core::default::Default::default(),
            pipeline: core::default::Default::default(),
            queue_name: core::default::Default::default(),
            retry: core::default::Default::default(),
            script_name: core::default::Default::default(),
            secret_name: core::default::Default::default(),
            service: core::default::Default::default(),
            service_id: core::default::Default::default(),
            simple: core::default::Default::default(),
            store_id: core::default::Default::default(),
            stream: core::default::Default::default(),
            text: core::default::Default::default(),
            tunnel_id: core::default::Default::default(),
            type_: self.type_,
            usages: core::default::Default::default(),
            version_id: core::default::Default::default(),
            workflow_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_BindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_BindingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_BindingsRef {
        WorkerVersion_BindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_BindingsRef {
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
    pub fn outbound(&self) -> WorkerVersion_Bindings_OutboundRef {
        WorkerVersion_Bindings_OutboundRef::new(self.shared().clone(), format!("{}.outbound", self.base))
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
    pub fn retry(&self) -> WorkerVersion_Bindings_RetryRef {
        WorkerVersion_Bindings_RetryRef::new(self.shared().clone(), format!("{}.retry", self.base))
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
    pub fn simple(&self) -> WorkerVersion_Bindings_SimpleRef {
        WorkerVersion_Bindings_SimpleRef::new(self.shared().clone(), format!("{}.simple", self.base))
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
pub struct WorkerVersion_CacheOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_version_cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl WorkerVersion_CacheOptions {
    #[doc =
        "Set the field `cross_version_cache`.\nWhether cached responses are shared across Worker version\nuploads. This is independent of `enabled`. It can stay true\nwhile caching is off, so the preference survives turning\ncaching off and back on."]
    pub fn set_cross_version_cache(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cross_version_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether caching is enabled for this Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_CacheOptions {
    type O = terrars::BlockAssignable<WorkerVersion_CacheOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_CacheOptions {}

impl BuildWorkerVersion_CacheOptions {
    pub fn build(self) -> WorkerVersion_CacheOptions {
        WorkerVersion_CacheOptions {
            cross_version_cache: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_CacheOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_CacheOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_CacheOptionsRef {
        WorkerVersion_CacheOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_CacheOptionsRef {
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
pub struct WorkerVersion_Containers {
    class_name: terrars::PrimField<std::string::String>,
}

impl WorkerVersion_Containers { }

impl terrars::ToListMappable for WorkerVersion_Containers {
    type O = terrars::BlockAssignable<WorkerVersion_Containers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Containers {
    #[doc = "Select which Durable Object class should get this container attached."]
    pub class_name: terrars::PrimField<std::string::String>,
}

impl BuildWorkerVersion_Containers {
    pub fn build(self) -> WorkerVersion_Containers {
        WorkerVersion_Containers { class_name: self.class_name }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_ContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_ContainersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_ContainersRef {
        WorkerVersion_ContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_ContainersRef {
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
pub struct WorkerVersion_Exports_Cache {
    enabled: terrars::PrimField<bool>,
}

impl WorkerVersion_Exports_Cache { }

impl terrars::ToListMappable for WorkerVersion_Exports_Cache {
    type O = terrars::BlockAssignable<WorkerVersion_Exports_Cache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Exports_Cache {
    #[doc = "Whether caching is enabled for this entrypoint."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildWorkerVersion_Exports_Cache {
    pub fn build(self) -> WorkerVersion_Exports_Cache {
        WorkerVersion_Exports_Cache { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Exports_CacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Exports_CacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_Exports_CacheRef {
        WorkerVersion_Exports_CacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Exports_CacheRef {
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
pub struct WorkerVersion_Exports {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache: std::option::Option<WorkerVersion_Exports_Cache>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    renamed_to: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transferred_to: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WorkerVersion_Exports {
    #[doc =
        "Set the field `cache`.\nCache override for this entrypoint. It applies only to\n`type: worker` entries and overrides the Worker's global\n`cache_options.enabled` for that entrypoint."]
    pub fn set_cache(mut self, v: impl std::convert::Into<WorkerVersion_Exports_Cache>) -> Self {
        self.cache = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `renamed_to`.\nDestination class name for a `state: renamed` tombstone. The\ntarget must appear as a live (`created`) entry in the same\n`exports` map. Write-only: never present in GET responses."]
    pub fn set_renamed_to(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.renamed_to = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `state`.\nLifecycle state of the export entry. Defaults to `created`\n(a normal, live export) when omitted.\n\n`deleted`, `renamed`, and `transferred` are tombstones:\nwrite-only lifecycle operations that retire, rename, or hand\noff a provisioned Durable Object namespace. They are applied\nat upload and are filtered out of GET responses, so a read\nonly ever returns `created` or `expecting-transfer`.\n\n`expecting-transfer` is a live export whose data is being\nreceived from another script via the two-phase transfer flow;\nit carries `storage` and `transfer_from`.\nAvailable values: \"created\", \"deleted\", \"renamed\", \"transferred\", \"expecting-transfer\"."]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage`.\nStorage backend for a `type: durable-object` export. Required\nfor live Durable Object entries (`created` and\n`expecting-transfer`). `sqlite` selects SQLite-backed storage;\n`legacy-kv` selects the legacy key-value storage.\nAvailable values: \"sqlite\", \"legacy-kv\"."]
    pub fn set_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transfer_from`.\nSource script for a `state: expecting-transfer` entry. The\nnamespace on this script is materialised from the source\nscript's data via the pending-transfer flow. Present on reads\nfor `expecting-transfer` entries."]
    pub fn set_transfer_from(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.transfer_from = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transferred_to`.\nDestination script for a `state: transferred` tombstone. Must\nreference a script in the same account; cross-dispatch-namespace\ntransfers are rejected. Write-only: never present in GET\nresponses."]
    pub fn set_transferred_to(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.transferred_to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Exports {
    type O = terrars::BlockAssignable<WorkerVersion_Exports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Exports {
    #[doc = "The kind of export.\nAvailable values: \"worker\", \"durable-object\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWorkerVersion_Exports {
    pub fn build(self) -> WorkerVersion_Exports {
        WorkerVersion_Exports {
            cache: core::default::Default::default(),
            renamed_to: core::default::Default::default(),
            state: core::default::Default::default(),
            storage: core::default::Default::default(),
            transfer_from: core::default::Default::default(),
            transferred_to: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_ExportsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_ExportsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_ExportsRef {
        WorkerVersion_ExportsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_ExportsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache` after provisioning.\nCache override for this entrypoint. It applies only to\n`type: worker` entries and overrides the Worker's global\n`cache_options.enabled` for that entrypoint."]
    pub fn cache(&self) -> WorkerVersion_Exports_CacheRef {
        WorkerVersion_Exports_CacheRef::new(self.shared().clone(), format!("{}.cache", self.base))
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
pub struct WorkerVersion_ExportsReconciliation_Info {}

impl WorkerVersion_ExportsReconciliation_Info { }

impl terrars::ToListMappable for WorkerVersion_ExportsReconciliation_Info {
    type O = terrars::BlockAssignable<WorkerVersion_ExportsReconciliation_Info>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_ExportsReconciliation_Info {}

impl BuildWorkerVersion_ExportsReconciliation_Info {
    pub fn build(self) -> WorkerVersion_ExportsReconciliation_Info {
        WorkerVersion_ExportsReconciliation_Info {}
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_ExportsReconciliation_InfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_ExportsReconciliation_InfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_ExportsReconciliation_InfoRef {
        WorkerVersion_ExportsReconciliation_InfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_ExportsReconciliation_InfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `class` after provisioning.\nThe class name the info entry is about."]
    pub fn class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\nHuman-readable explanation."]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace_id` after provisioning.\nThe provisioned namespace the entry relates to, when applicable."]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `referencing_scripts` after provisioning.\nOther Workers in the account that still bind to the affected class. Advisory: while non-empty the tombstone is not yet safe to remove — redeploy these Workers with bindings re-pointed first."]
    pub fn referencing_scripts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referencing_scripts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scenario` after provisioning.\nStable, machine-readable tag identifying which reconciliation scenario produced an error, warning, or info entry. Clients may branch on this value instead of parsing `message`."]
    pub fn scenario(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scenario", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_ExportsReconciliation_Renamed {}

impl WorkerVersion_ExportsReconciliation_Renamed { }

impl terrars::ToListMappable for WorkerVersion_ExportsReconciliation_Renamed {
    type O = terrars::BlockAssignable<WorkerVersion_ExportsReconciliation_Renamed>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_ExportsReconciliation_Renamed {}

impl BuildWorkerVersion_ExportsReconciliation_Renamed {
    pub fn build(self) -> WorkerVersion_ExportsReconciliation_Renamed {
        WorkerVersion_ExportsReconciliation_Renamed {}
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_ExportsReconciliation_RenamedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_ExportsReconciliation_RenamedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkerVersion_ExportsReconciliation_RenamedRef {
        WorkerVersion_ExportsReconciliation_RenamedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_ExportsReconciliation_RenamedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\nThe original (source) class name."]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\nThe new class name (`renamed_to`)."]
    pub fn to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_ExportsReconciliation_TransferPending {}

impl WorkerVersion_ExportsReconciliation_TransferPending { }

impl terrars::ToListMappable for WorkerVersion_ExportsReconciliation_TransferPending {
    type O = terrars::BlockAssignable<WorkerVersion_ExportsReconciliation_TransferPending>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_ExportsReconciliation_TransferPending {}

impl BuildWorkerVersion_ExportsReconciliation_TransferPending {
    pub fn build(self) -> WorkerVersion_ExportsReconciliation_TransferPending {
        WorkerVersion_ExportsReconciliation_TransferPending {}
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_ExportsReconciliation_TransferPendingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_ExportsReconciliation_TransferPendingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkerVersion_ExportsReconciliation_TransferPendingRef {
        WorkerVersion_ExportsReconciliation_TransferPendingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_ExportsReconciliation_TransferPendingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `class` after provisioning.\nThe target-side class name awaiting transfer."]
    pub fn class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `from` after provisioning.\nThe source script the namespace will be transferred from."]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_ExportsReconciliation_Transferred {}

impl WorkerVersion_ExportsReconciliation_Transferred { }

impl terrars::ToListMappable for WorkerVersion_ExportsReconciliation_Transferred {
    type O = terrars::BlockAssignable<WorkerVersion_ExportsReconciliation_Transferred>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_ExportsReconciliation_Transferred {}

impl BuildWorkerVersion_ExportsReconciliation_Transferred {
    pub fn build(self) -> WorkerVersion_ExportsReconciliation_Transferred {
        WorkerVersion_ExportsReconciliation_Transferred {}
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_ExportsReconciliation_TransferredRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_ExportsReconciliation_TransferredRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkerVersion_ExportsReconciliation_TransferredRef {
        WorkerVersion_ExportsReconciliation_TransferredRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_ExportsReconciliation_TransferredRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `class` after provisioning.\nThe source class name that was transferred."]
    pub fn class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phase` after provisioning.\nThe transfer phase. Currently always `committed`."]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `to` after provisioning.\nThe destination script that now owns the namespace."]
    pub fn to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_ExportsReconciliation_Warnings {}

impl WorkerVersion_ExportsReconciliation_Warnings { }

impl terrars::ToListMappable for WorkerVersion_ExportsReconciliation_Warnings {
    type O = terrars::BlockAssignable<WorkerVersion_ExportsReconciliation_Warnings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_ExportsReconciliation_Warnings {}

impl BuildWorkerVersion_ExportsReconciliation_Warnings {
    pub fn build(self) -> WorkerVersion_ExportsReconciliation_Warnings {
        WorkerVersion_ExportsReconciliation_Warnings {}
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_ExportsReconciliation_WarningsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_ExportsReconciliation_WarningsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkerVersion_ExportsReconciliation_WarningsRef {
        WorkerVersion_ExportsReconciliation_WarningsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_ExportsReconciliation_WarningsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `class` after provisioning.\nThe class name the warning is about."]
    pub fn class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `message` after provisioning.\nHuman-readable explanation of the warning."]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace_id` after provisioning.\nThe provisioned namespace the warning relates to, when applicable."]
    pub fn namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scenario` after provisioning.\nStable, machine-readable tag identifying which reconciliation scenario produced an error, warning, or info entry. Clients may branch on this value instead of parsing `message`."]
    pub fn scenario(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scenario", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_ExportsReconciliation {}

impl WorkerVersion_ExportsReconciliation { }

impl terrars::ToListMappable for WorkerVersion_ExportsReconciliation {
    type O = terrars::BlockAssignable<WorkerVersion_ExportsReconciliation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_ExportsReconciliation {}

impl BuildWorkerVersion_ExportsReconciliation {
    pub fn build(self) -> WorkerVersion_ExportsReconciliation {
        WorkerVersion_ExportsReconciliation {}
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_ExportsReconciliationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_ExportsReconciliationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_ExportsReconciliationRef {
        WorkerVersion_ExportsReconciliationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_ExportsReconciliationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nClass names for which a new namespace was provisioned."]
    pub fn created(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deleted` after provisioning.\nClass names whose namespace was deleted by a `deleted` tombstone."]
    pub fn deleted(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deleted", self.base))
    }

    #[doc =
        "Get a reference to the value of field `info` after provisioning.\nNon-blocking info entries (stale tombstones, tombstone applied with class still in code). See `exports_reconciliation_info`."]
    pub fn info(&self) -> terrars::ListRef<WorkerVersion_ExportsReconciliation_InfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info", self.base))
    }

    #[doc =
        "Get a reference to the value of field `removable_entries` after provisioning.\nSource class names whose tombstone entry is now stale and safe to delete from `exports` (no remaining referencing scripts)."]
    pub fn removable_entries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.removable_entries", self.base))
    }

    #[doc = "Get a reference to the value of field `renamed` after provisioning.\nApplied `renamed` tombstones."]
    pub fn renamed(&self) -> terrars::ListRef<WorkerVersion_ExportsReconciliation_RenamedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.renamed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transfer_pending` after provisioning.\nPhase-1 transfer hints recorded on the target side."]
    pub fn transfer_pending(&self) -> terrars::ListRef<WorkerVersion_ExportsReconciliation_TransferPendingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_pending", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transferred` after provisioning.\nCommitted `transferred` tombstones (phase-2)."]
    pub fn transferred(&self) -> terrars::ListRef<WorkerVersion_ExportsReconciliation_TransferredRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transferred", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nClass names whose provisioned namespace was mutated in place."]
    pub fn updated(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `warnings` after provisioning.\nNon-blocking warnings. See `exports_reconciliation_warning`."]
    pub fn warnings(&self) -> terrars::ListRef<WorkerVersion_ExportsReconciliation_WarningsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warnings", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_Limits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subrequests: std::option::Option<terrars::PrimField<f64>>,
}

impl WorkerVersion_Limits {
    #[doc = "Set the field `cpu_ms`.\nCPU time limit in milliseconds."]
    pub fn set_cpu_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subrequests`.\nSubrequest limit per request."]
    pub fn set_subrequests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.subrequests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Limits {
    type O = terrars::BlockAssignable<WorkerVersion_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Limits {}

impl BuildWorkerVersion_Limits {
    pub fn build(self) -> WorkerVersion_Limits {
        WorkerVersion_Limits {
            cpu_ms: core::default::Default::default(),
            subrequests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_LimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_LimitsRef {
        WorkerVersion_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_LimitsRef {
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
pub struct WorkerVersion_Migrations_RenamedClasses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkerVersion_Migrations_RenamedClasses {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Migrations_RenamedClasses {
    type O = terrars::BlockAssignable<WorkerVersion_Migrations_RenamedClasses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Migrations_RenamedClasses {}

impl BuildWorkerVersion_Migrations_RenamedClasses {
    pub fn build(self) -> WorkerVersion_Migrations_RenamedClasses {
        WorkerVersion_Migrations_RenamedClasses {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Migrations_RenamedClassesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Migrations_RenamedClassesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_Migrations_RenamedClassesRef {
        WorkerVersion_Migrations_RenamedClassesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Migrations_RenamedClassesRef {
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
pub struct WorkerVersion_Migrations_TransferredClasses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkerVersion_Migrations_TransferredClasses {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_script`.\n"]
    pub fn set_from_script(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.from_script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Migrations_TransferredClasses {
    type O = terrars::BlockAssignable<WorkerVersion_Migrations_TransferredClasses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Migrations_TransferredClasses {}

impl BuildWorkerVersion_Migrations_TransferredClasses {
    pub fn build(self) -> WorkerVersion_Migrations_TransferredClasses {
        WorkerVersion_Migrations_TransferredClasses {
            from: core::default::Default::default(),
            from_script: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Migrations_TransferredClassesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Migrations_TransferredClassesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkerVersion_Migrations_TransferredClassesRef {
        WorkerVersion_Migrations_TransferredClassesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Migrations_TransferredClassesRef {
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
pub struct WorkerVersion_Migrations_Steps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_sqlite_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    renamed_classes: std::option::Option<std::vec::Vec<WorkerVersion_Migrations_RenamedClasses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transferred_classes: std::option::Option<std::vec::Vec<WorkerVersion_Migrations_TransferredClasses>>,
}

impl WorkerVersion_Migrations_Steps {
    #[doc = "Set the field `deleted_classes`.\nA list of classes to delete Durable Object namespaces from."]
    pub fn set_deleted_classes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.deleted_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `new_classes`.\nA list of classes to create Durable Object namespaces from."]
    pub fn set_new_classes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.new_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `new_sqlite_classes`.\nA list of classes to create Durable Object namespaces with SQLite from."]
    pub fn set_new_sqlite_classes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.new_sqlite_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `renamed_classes`.\nA list of classes with Durable Object namespaces that were renamed."]
    pub fn set_renamed_classes(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<WorkerVersion_Migrations_RenamedClasses>>,
    ) -> Self {
        self.renamed_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transferred_classes`.\nA list of transfers for Durable Object namespaces from a different Worker and class to a class defined in this Worker."]
    pub fn set_transferred_classes(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<WorkerVersion_Migrations_TransferredClasses>>,
    ) -> Self {
        self.transferred_classes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Migrations_Steps {
    type O = terrars::BlockAssignable<WorkerVersion_Migrations_Steps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Migrations_Steps {}

impl BuildWorkerVersion_Migrations_Steps {
    pub fn build(self) -> WorkerVersion_Migrations_Steps {
        WorkerVersion_Migrations_Steps {
            deleted_classes: core::default::Default::default(),
            new_classes: core::default::Default::default(),
            new_sqlite_classes: core::default::Default::default(),
            renamed_classes: core::default::Default::default(),
            transferred_classes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Migrations_StepsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Migrations_StepsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_Migrations_StepsRef {
        WorkerVersion_Migrations_StepsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Migrations_StepsRef {
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
    pub fn renamed_classes(&self) -> terrars::ListRef<WorkerVersion_Migrations_RenamedClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.renamed_classes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transferred_classes` after provisioning.\nA list of transfers for Durable Object namespaces from a different Worker and class to a class defined in this Worker."]
    pub fn transferred_classes(&self) -> terrars::ListRef<WorkerVersion_Migrations_TransferredClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transferred_classes", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_Migrations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_sqlite_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    old_tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    renamed_classes: std::option::Option<std::vec::Vec<WorkerVersion_Migrations_RenamedClasses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std::option::Option<std::vec::Vec<WorkerVersion_Migrations_Steps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transferred_classes: std::option::Option<std::vec::Vec<WorkerVersion_Migrations_TransferredClasses>>,
}

impl WorkerVersion_Migrations {
    #[doc = "Set the field `deleted_classes`.\nA list of classes to delete Durable Object namespaces from."]
    pub fn set_deleted_classes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.deleted_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `new_classes`.\nA list of classes to create Durable Object namespaces from."]
    pub fn set_new_classes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.new_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `new_sqlite_classes`.\nA list of classes to create Durable Object namespaces with SQLite from."]
    pub fn set_new_sqlite_classes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.new_sqlite_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `new_tag`.\nTag to set as the latest migration tag."]
    pub fn set_new_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.new_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `old_tag`.\nTag used to verify against the latest migration tag for this Worker. If they don't match, the upload is rejected."]
    pub fn set_old_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.old_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `renamed_classes`.\nA list of classes with Durable Object namespaces that were renamed."]
    pub fn set_renamed_classes(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<WorkerVersion_Migrations_RenamedClasses>>,
    ) -> Self {
        self.renamed_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `steps`.\nMigrations to apply in order."]
    pub fn set_steps(mut self, v: impl std::convert::Into<std::vec::Vec<WorkerVersion_Migrations_Steps>>) -> Self {
        self.steps = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transferred_classes`.\nA list of transfers for Durable Object namespaces from a different Worker and class to a class defined in this Worker."]
    pub fn set_transferred_classes(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<WorkerVersion_Migrations_TransferredClasses>>,
    ) -> Self {
        self.transferred_classes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Migrations {
    type O = terrars::BlockAssignable<WorkerVersion_Migrations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Migrations {}

impl BuildWorkerVersion_Migrations {
    pub fn build(self) -> WorkerVersion_Migrations {
        WorkerVersion_Migrations {
            deleted_classes: core::default::Default::default(),
            new_classes: core::default::Default::default(),
            new_sqlite_classes: core::default::Default::default(),
            new_tag: core::default::Default::default(),
            old_tag: core::default::Default::default(),
            renamed_classes: core::default::Default::default(),
            steps: core::default::Default::default(),
            transferred_classes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_MigrationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_MigrationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_MigrationsRef {
        WorkerVersion_MigrationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_MigrationsRef {
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
    pub fn renamed_classes(&self) -> terrars::ListRef<WorkerVersion_Migrations_RenamedClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.renamed_classes", self.base))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\nMigrations to apply in order."]
    pub fn steps(&self) -> terrars::ListRef<WorkerVersion_Migrations_StepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transferred_classes` after provisioning.\nA list of transfers for Durable Object namespaces from a different Worker and class to a class defined in this Worker."]
    pub fn transferred_classes(&self) -> terrars::ListRef<WorkerVersion_Migrations_TransferredClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transferred_classes", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkerVersion_Modules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_base64: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_file: std::option::Option<terrars::PrimField<std::string::String>>,
    content_type: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl WorkerVersion_Modules {
    #[doc = "Set the field `content_base64`.\nThe base64-encoded module content."]
    pub fn set_content_base64(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_base64 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_file`.\nThe file path of the module content."]
    pub fn set_content_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_file = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Modules {
    type O = terrars::BlockAssignable<WorkerVersion_Modules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Modules {
    #[doc = "The content type of the module."]
    pub content_type: terrars::PrimField<std::string::String>,
    #[doc = "The name of the module."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWorkerVersion_Modules {
    pub fn build(self) -> WorkerVersion_Modules {
        WorkerVersion_Modules {
            content_base64: core::default::Default::default(),
            content_file: core::default::Default::default(),
            content_type: self.content_type,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_ModulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_ModulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_ModulesRef {
        WorkerVersion_ModulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_ModulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content_base64` after provisioning.\nThe base64-encoded module content."]
    pub fn content_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_file` after provisioning.\nThe file path of the module content."]
    pub fn content_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_sha256` after provisioning.\nThe SHA-256 hash of the module content."]
    pub fn content_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_sha256", self.base))
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
pub struct WorkerVersion_PackageDependencies {
    installed_version: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    package_json_version: terrars::PrimField<std::string::String>,
}

impl WorkerVersion_PackageDependencies { }

impl terrars::ToListMappable for WorkerVersion_PackageDependencies {
    type O = terrars::BlockAssignable<WorkerVersion_PackageDependencies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_PackageDependencies {
    #[doc = "The exact version that was resolved and installed by the package manager."]
    pub installed_version: terrars::PrimField<std::string::String>,
    #[doc = "The npm package name."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The version constraint as written in package.json."]
    pub package_json_version: terrars::PrimField<std::string::String>,
}

impl BuildWorkerVersion_PackageDependencies {
    pub fn build(self) -> WorkerVersion_PackageDependencies {
        WorkerVersion_PackageDependencies {
            installed_version: self.installed_version,
            name: self.name,
            package_json_version: self.package_json_version,
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_PackageDependenciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_PackageDependenciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_PackageDependenciesRef {
        WorkerVersion_PackageDependenciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_PackageDependenciesRef {
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
pub struct WorkerVersion_Placement_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkerVersion_Placement_Target {
    #[doc = "Set the field `host`.\nTCP host:port for targeted placement."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nHTTP hostname for targeted placement."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nCloud region in format 'provider:region'."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Placement_Target {
    type O = terrars::BlockAssignable<WorkerVersion_Placement_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Placement_Target {}

impl BuildWorkerVersion_Placement_Target {
    pub fn build(self) -> WorkerVersion_Placement_Target {
        WorkerVersion_Placement_Target {
            host: core::default::Default::default(),
            hostname: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_Placement_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_Placement_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_Placement_TargetRef {
        WorkerVersion_Placement_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_Placement_TargetRef {
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
pub struct WorkerVersion_Placement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<WorkerVersion_Placement_Target>>,
}

impl WorkerVersion_Placement {
    #[doc = "Set the field `host`.\nTCP host and port for targeted placement."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nHTTP hostname for targeted placement."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nEnables [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).\nAvailable values: \"smart\", \"targeted\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nCloud region for targeted placement in format 'provider:region'."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\nArray of placement targets (currently limited to single target)."]
    pub fn set_target(mut self, v: impl std::convert::Into<std::vec::Vec<WorkerVersion_Placement_Target>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkerVersion_Placement {
    type O = terrars::BlockAssignable<WorkerVersion_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkerVersion_Placement {}

impl BuildWorkerVersion_Placement {
    pub fn build(self) -> WorkerVersion_Placement {
        WorkerVersion_Placement {
            host: core::default::Default::default(),
            hostname: core::default::Default::default(),
            mode: core::default::Default::default(),
            region: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkerVersion_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkerVersion_PlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkerVersion_PlacementRef {
        WorkerVersion_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkerVersion_PlacementRef {
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
    pub fn target(&self) -> terrars::ListRef<WorkerVersion_Placement_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
