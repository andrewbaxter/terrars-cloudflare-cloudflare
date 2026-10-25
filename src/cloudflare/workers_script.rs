#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct WorkersScriptData_ {
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
    annotations: std::option::Option<WorkersScript_Annotations>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assets: std::option::Option<WorkersScript_Assets>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bindings: std::option::Option<std::vec::Vec<WorkersScript_Bindings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bindings_inherit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body_part: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_options: std::option::Option<WorkersScript_CacheOptions>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compatibility_flags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_sha256: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exports: std::option::Option<std::collections::HashMap<std::string::String, WorkersScript_Exports>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    files: std::option::Option<std::collections::HashMap<std::string::String, WorkersScript_Files>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_assets: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_bindings: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<WorkersScript_Limits>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logpush: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_module: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    migrations: std::option::Option<WorkersScript_Migrations>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    observability: std::option::Option<WorkersScript_Observability>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_dependencies: std::option::Option<std::vec::Vec<WorkersScript_PackageDependencies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<WorkersScript_Placement>,
    script_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tail_consumers: std::option::Option<std::vec::Vec<WorkersScript_TailConsumers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_model: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct WorkersScript_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkersScriptData_>,
}

#[derive(Clone)]
pub struct WorkersScript(std::rc::Rc<WorkersScript_>);

impl WorkersScript {
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

    #[doc = "Set the field `annotations`.\nAnnotations for the version created by this upload."]
    pub fn set_annotations(self, v: impl std::convert::Into<WorkersScript_Annotations>) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `assets`.\nConfiguration for assets within a Worker."]
    pub fn set_assets(self, v: impl std::convert::Into<WorkersScript_Assets>) -> Self {
        self.0.data.borrow_mut().assets = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bindings`.\nList of bindings attached to a Worker. You can find more about bindings on our docs: https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings."]
    pub fn set_bindings(self, v: impl std::convert::Into<std::vec::Vec<WorkersScript_Bindings>>) -> Self {
        self.0.data.borrow_mut().bindings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bindings_inherit`.\nWhen set to \"strict\", the upload will fail if any `inherit` type bindings cannot be resolved against the previous version of the Worker. Without this, unresolvable inherit bindings are silently dropped.\nAvailable values: \"strict\"."]
    pub fn set_bindings_inherit(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bindings_inherit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `body_part`.\nName of the uploaded file that contains the script (e.g. the file adding a listener to the `fetch` event). Indicates a `service worker syntax` Worker."]
    pub fn set_body_part(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().body_part = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache_options`.\nGlobal CacheW configuration for the Worker. When caching is on,\nthe platform provisions a `cloudflare.app` zone for the Worker.\nA `type: worker` entry in the `exports` map can override this\nvalue for a single entrypoint."]
    pub fn set_cache_options(self, v: impl std::convert::Into<WorkersScript_CacheOptions>) -> Self {
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

    #[doc = "Set the field `content`.\nModule or Service Worker contents of the Worker. Conflicts with `content_file`."]
    pub fn set_content(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_file`.\nPath to a file containing the Module or Service Worker contents of the Worker. Conflicts with `content`. Must be paired with `content_sha256`."]
    pub fn set_content_file(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_sha256`.\nSHA-256 hash of the Worker contents. Used to trigger updates when source code changes. Must be provided when `content_file` is specified."]
    pub fn set_content_sha256(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_sha256 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_type`.\nContent-Type of the Worker. Required if uploading a non-JavaScript Worker (e.g. \"text/x-python\")."]
    pub fn set_content_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exports`.\nPer-entrypoint export configuration. Keys are the export names; values describe the entrypoint's kind and per-entrypoint cache behavior."]
    pub fn set_exports(
        self,
        v: impl std::convert::Into<std::collections::HashMap<std::string::String, WorkersScript_Exports>>,
    ) -> Self {
        self.0.data.borrow_mut().exports = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `files`.\nAdditional modules and data files to include in the multipart Worker upload. Map keys are multipart part names referenced by binding `part` values and module imports."]
    pub fn set_files(
        self,
        v: impl std::convert::Into<std::collections::HashMap<std::string::String, WorkersScript_Files>>,
    ) -> Self {
        self.0.data.borrow_mut().files = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force`.\nIf true, delete the Worker even when other Workers still reference it. Service bindings in those Workers may be left broken. Durable Object namespaces implemented by the deleted Worker are deleted even if other Workers reference them."]
    pub fn set_force(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keep_assets`.\nRetain assets which exist for a previously uploaded Worker version; used in lieu of providing a completion token. An explicit `assets` upload takes precedence over `keep_assets`."]
    pub fn set_keep_assets(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().keep_assets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keep_bindings`.\nList of binding types to keep from previous_upload."]
    pub fn set_keep_bindings(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().keep_bindings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limits`.\nLimits to apply for this Worker."]
    pub fn set_limits(self, v: impl std::convert::Into<WorkersScript_Limits>) -> Self {
        self.0.data.borrow_mut().limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logpush`.\nWhether Logpush is turned on for the Worker."]
    pub fn set_logpush(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().logpush = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_module`.\nName of the uploaded file that contains the main module (e.g. the file exporting a `fetch` handler). Indicates a `module syntax` Worker."]
    pub fn set_main_module(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().main_module = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `migrations`.\nMigrations to apply for Durable Objects associated with this Worker."]
    pub fn set_migrations(self, v: impl std::convert::Into<WorkersScript_Migrations>) -> Self {
        self.0.data.borrow_mut().migrations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `observability`.\nObservability settings for the Worker."]
    pub fn set_observability(self, v: impl std::convert::Into<WorkersScript_Observability>) -> Self {
        self.0.data.borrow_mut().observability = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `package_dependencies`.\nThe list of npm packages that were installed and used when this Worker was built."]
    pub fn set_package_dependencies(
        self,
        v: impl std::convert::Into<std::vec::Vec<WorkersScript_PackageDependencies>>,
    ) -> Self {
        self.0.data.borrow_mut().package_dependencies = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `placement`.\nConfiguration for [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement). Specify mode='smart' for Smart Placement, or one of region/hostname/host."]
    pub fn set_placement(self, v: impl std::convert::Into<WorkersScript_Placement>) -> Self {
        self.0.data.borrow_mut().placement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tail_consumers`.\nList of Workers that will consume logs from the attached Worker."]
    pub fn set_tail_consumers(self, v: impl std::convert::Into<std::vec::Vec<WorkersScript_TailConsumers>>) -> Self {
        self.0.data.borrow_mut().tail_consumers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `usage_model`.\nUsage model for the Worker invocations.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn set_usage_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().usage_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations for the version created by this upload."]
    pub fn annotations(&self) -> WorkersScript_AnnotationsRef {
        WorkersScript_AnnotationsRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `assets` after provisioning.\nConfiguration for assets within a Worker."]
    pub fn assets(&self) -> WorkersScript_AssetsRef {
        WorkersScript_AssetsRef::new(self.shared().clone(), format!("{}.assets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bindings` after provisioning.\nList of bindings attached to a Worker. You can find more about bindings on our docs: https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings."]
    pub fn bindings(&self) -> terrars::ListRef<WorkersScript_BindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bindings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bindings_inherit` after provisioning.\nWhen set to \"strict\", the upload will fail if any `inherit` type bindings cannot be resolved against the previous version of the Worker. Without this, unresolvable inherit bindings are silently dropped.\nAvailable values: \"strict\"."]
    pub fn bindings_inherit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bindings_inherit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `body_part` after provisioning.\nName of the uploaded file that contains the script (e.g. the file adding a listener to the `fetch` event). Indicates a `service worker syntax` Worker."]
    pub fn body_part(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body_part", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cache_options` after provisioning.\nGlobal CacheW configuration for the Worker. When caching is on,\nthe platform provisions a `cloudflare.app` zone for the Worker.\nA `type: worker` entry in the `exports` map can override this\nvalue for a single entrypoint."]
    pub fn cache_options(&self) -> WorkersScript_CacheOptionsRef {
        WorkersScript_CacheOptionsRef::new(self.shared().clone(), format!("{}.cache_options", self.extract_ref()))
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
        "Get a reference to the value of field `content` after provisioning.\nModule or Service Worker contents of the Worker. Conflicts with `content_file`."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_file` after provisioning.\nPath to a file containing the Module or Service Worker contents of the Worker. Conflicts with `content`. Must be paired with `content_sha256`."]
    pub fn content_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_file", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_sha256` after provisioning.\nSHA-256 hash of the Worker contents. Used to trigger updates when source code changes. Must be provided when `content_file` is specified."]
    pub fn content_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_sha256", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the Worker. Required if uploading a non-JavaScript Worker (e.g. \"text/x-python\")."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the script was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nHashed script content, can be used in a If-None-Match header when updating."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exports` after provisioning.\nPer-entrypoint export configuration. Keys are the export names; values describe the entrypoint's kind and per-entrypoint cache behavior."]
    pub fn exports(&self) -> terrars::RecRef<WorkersScript_ExportsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.exports", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `files` after provisioning.\nAdditional modules and data files to include in the multipart Worker upload. Map keys are multipart part names referenced by binding `part` values and module imports."]
    pub fn files(&self) -> terrars::RecRef<WorkersScript_FilesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.files", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force` after provisioning.\nIf true, delete the Worker even when other Workers still reference it. Service bindings in those Workers may be left broken. Durable Object namespaces implemented by the deleted Worker are deleted even if other Workers reference them."]
    pub fn force(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `handlers` after provisioning.\nThe names of handlers exported as part of the default export."]
    pub fn handlers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `has_assets` after provisioning.\nWhether a Worker contains assets."]
    pub fn has_assets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_assets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_modules` after provisioning.\nWhether a Worker contains modules."]
    pub fn has_modules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_modules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nName of the script, used in URLs and route configuration."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keep_assets` after provisioning.\nRetain assets which exist for a previously uploaded Worker version; used in lieu of providing a completion token. An explicit `assets` upload takes precedence over `keep_assets`."]
    pub fn keep_assets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_assets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keep_bindings` after provisioning.\nList of binding types to keep from previous_upload."]
    pub fn keep_bindings(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.keep_bindings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_deployed_from` after provisioning.\nThe client most recently used to deploy this Worker."]
    pub fn last_deployed_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_deployed_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\nLimits to apply for this Worker."]
    pub fn limits(&self) -> WorkersScript_LimitsRef {
        WorkersScript_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logpush` after provisioning.\nWhether Logpush is turned on for the Worker."]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `main_module` after provisioning.\nName of the uploaded file that contains the main module (e.g. the file exporting a `fetch` handler). Indicates a `module syntax` Worker."]
    pub fn main_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_module", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migration_tag` after provisioning.\nThe tag of the Durable Object migration that was most recently applied for this Worker."]
    pub fn migration_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.migration_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migrations` after provisioning.\nMigrations to apply for Durable Objects associated with this Worker."]
    pub fn migrations(&self) -> WorkersScript_MigrationsRef {
        WorkersScript_MigrationsRef::new(self.shared().clone(), format!("{}.migrations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the script was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `named_handlers` after provisioning.\nNamed exports, such as Durable Object class implementations and named entrypoints."]
    pub fn named_handlers(&self) -> terrars::ListRef<WorkersScript_NamedHandlersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.named_handlers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observability` after provisioning.\nObservability settings for the Worker."]
    pub fn observability(&self) -> WorkersScript_ObservabilityRef {
        WorkersScript_ObservabilityRef::new(self.shared().clone(), format!("{}.observability", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `package_dependencies` after provisioning.\nThe list of npm packages that were installed and used when this Worker was built."]
    pub fn package_dependencies(&self) -> terrars::ListRef<WorkersScript_PackageDependenciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.package_dependencies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nConfiguration for [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement). Specify mode='smart' for Smart Placement, or one of region/hostname/host."]
    pub fn placement(&self) -> WorkersScript_PlacementRef {
        WorkersScript_PlacementRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement_mode` after provisioning.\nAvailable values: \"smart\", \"targeted\"."]
    pub fn placement_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement_status` after provisioning.\nAvailable values: \"SUCCESS\", \"UNSUPPORTED_APPLICATION\", \"INSUFFICIENT_INVOCATIONS\"."]
    pub fn placement_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `script_name` after provisioning.\nName of the script, used in URLs and route configuration."]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `startup_time_ms` after provisioning.\n"]
    pub fn startup_time_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_time_ms", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tail_consumers` after provisioning.\nList of Workers that will consume logs from the attached Worker."]
    pub fn tail_consumers(&self) -> terrars::SetRef<WorkersScript_TailConsumersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tail_consumers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nUsage model for the Worker invocations.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.extract_ref()))
    }
}

impl terrars::Referable for WorkersScript {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkersScript { }

impl terrars::ToListMappable for WorkersScript {
    type O = terrars::ListRef<WorkersScriptRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkersScript_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_workers_script".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkersScript {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the script, used in URLs and route configuration."]
    pub script_name: terrars::PrimField<std::string::String>,
}

impl BuildWorkersScript {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkersScript {
        let out = WorkersScript(std::rc::Rc::new(WorkersScript_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkersScriptData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                annotations: core::default::Default::default(),
                assets: core::default::Default::default(),
                bindings: core::default::Default::default(),
                bindings_inherit: core::default::Default::default(),
                body_part: core::default::Default::default(),
                cache_options: core::default::Default::default(),
                compatibility_date: core::default::Default::default(),
                compatibility_flags: core::default::Default::default(),
                content: core::default::Default::default(),
                content_file: core::default::Default::default(),
                content_sha256: core::default::Default::default(),
                content_type: core::default::Default::default(),
                exports: core::default::Default::default(),
                files: core::default::Default::default(),
                force: core::default::Default::default(),
                keep_assets: core::default::Default::default(),
                keep_bindings: core::default::Default::default(),
                limits: core::default::Default::default(),
                logpush: core::default::Default::default(),
                main_module: core::default::Default::default(),
                migrations: core::default::Default::default(),
                observability: core::default::Default::default(),
                package_dependencies: core::default::Default::default(),
                placement: core::default::Default::default(),
                script_name: self.script_name,
                tail_consumers: core::default::Default::default(),
                usage_model: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkersScriptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScriptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkersScriptRef {
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
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations for the version created by this upload."]
    pub fn annotations(&self) -> WorkersScript_AnnotationsRef {
        WorkersScript_AnnotationsRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `assets` after provisioning.\nConfiguration for assets within a Worker."]
    pub fn assets(&self) -> WorkersScript_AssetsRef {
        WorkersScript_AssetsRef::new(self.shared().clone(), format!("{}.assets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bindings` after provisioning.\nList of bindings attached to a Worker. You can find more about bindings on our docs: https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings."]
    pub fn bindings(&self) -> terrars::ListRef<WorkersScript_BindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bindings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bindings_inherit` after provisioning.\nWhen set to \"strict\", the upload will fail if any `inherit` type bindings cannot be resolved against the previous version of the Worker. Without this, unresolvable inherit bindings are silently dropped.\nAvailable values: \"strict\"."]
    pub fn bindings_inherit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bindings_inherit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `body_part` after provisioning.\nName of the uploaded file that contains the script (e.g. the file adding a listener to the `fetch` event). Indicates a `service worker syntax` Worker."]
    pub fn body_part(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body_part", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cache_options` after provisioning.\nGlobal CacheW configuration for the Worker. When caching is on,\nthe platform provisions a `cloudflare.app` zone for the Worker.\nA `type: worker` entry in the `exports` map can override this\nvalue for a single entrypoint."]
    pub fn cache_options(&self) -> WorkersScript_CacheOptionsRef {
        WorkersScript_CacheOptionsRef::new(self.shared().clone(), format!("{}.cache_options", self.extract_ref()))
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
        "Get a reference to the value of field `content` after provisioning.\nModule or Service Worker contents of the Worker. Conflicts with `content_file`."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_file` after provisioning.\nPath to a file containing the Module or Service Worker contents of the Worker. Conflicts with `content`. Must be paired with `content_sha256`."]
    pub fn content_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_file", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_sha256` after provisioning.\nSHA-256 hash of the Worker contents. Used to trigger updates when source code changes. Must be provided when `content_file` is specified."]
    pub fn content_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_sha256", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the Worker. Required if uploading a non-JavaScript Worker (e.g. \"text/x-python\")."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the script was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nHashed script content, can be used in a If-None-Match header when updating."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exports` after provisioning.\nPer-entrypoint export configuration. Keys are the export names; values describe the entrypoint's kind and per-entrypoint cache behavior."]
    pub fn exports(&self) -> terrars::RecRef<WorkersScript_ExportsRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.exports", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `files` after provisioning.\nAdditional modules and data files to include in the multipart Worker upload. Map keys are multipart part names referenced by binding `part` values and module imports."]
    pub fn files(&self) -> terrars::RecRef<WorkersScript_FilesRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.files", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force` after provisioning.\nIf true, delete the Worker even when other Workers still reference it. Service bindings in those Workers may be left broken. Durable Object namespaces implemented by the deleted Worker are deleted even if other Workers reference them."]
    pub fn force(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `handlers` after provisioning.\nThe names of handlers exported as part of the default export."]
    pub fn handlers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `has_assets` after provisioning.\nWhether a Worker contains assets."]
    pub fn has_assets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_assets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_modules` after provisioning.\nWhether a Worker contains modules."]
    pub fn has_modules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_modules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nName of the script, used in URLs and route configuration."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keep_assets` after provisioning.\nRetain assets which exist for a previously uploaded Worker version; used in lieu of providing a completion token. An explicit `assets` upload takes precedence over `keep_assets`."]
    pub fn keep_assets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_assets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keep_bindings` after provisioning.\nList of binding types to keep from previous_upload."]
    pub fn keep_bindings(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.keep_bindings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_deployed_from` after provisioning.\nThe client most recently used to deploy this Worker."]
    pub fn last_deployed_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_deployed_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\nLimits to apply for this Worker."]
    pub fn limits(&self) -> WorkersScript_LimitsRef {
        WorkersScript_LimitsRef::new(self.shared().clone(), format!("{}.limits", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logpush` after provisioning.\nWhether Logpush is turned on for the Worker."]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `main_module` after provisioning.\nName of the uploaded file that contains the main module (e.g. the file exporting a `fetch` handler). Indicates a `module syntax` Worker."]
    pub fn main_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_module", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migration_tag` after provisioning.\nThe tag of the Durable Object migration that was most recently applied for this Worker."]
    pub fn migration_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.migration_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migrations` after provisioning.\nMigrations to apply for Durable Objects associated with this Worker."]
    pub fn migrations(&self) -> WorkersScript_MigrationsRef {
        WorkersScript_MigrationsRef::new(self.shared().clone(), format!("{}.migrations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nWhen the script was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `named_handlers` after provisioning.\nNamed exports, such as Durable Object class implementations and named entrypoints."]
    pub fn named_handlers(&self) -> terrars::ListRef<WorkersScript_NamedHandlersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.named_handlers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observability` after provisioning.\nObservability settings for the Worker."]
    pub fn observability(&self) -> WorkersScript_ObservabilityRef {
        WorkersScript_ObservabilityRef::new(self.shared().clone(), format!("{}.observability", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `package_dependencies` after provisioning.\nThe list of npm packages that were installed and used when this Worker was built."]
    pub fn package_dependencies(&self) -> terrars::ListRef<WorkersScript_PackageDependenciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.package_dependencies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement` after provisioning.\nConfiguration for [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement). Specify mode='smart' for Smart Placement, or one of region/hostname/host."]
    pub fn placement(&self) -> WorkersScript_PlacementRef {
        WorkersScript_PlacementRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement_mode` after provisioning.\nAvailable values: \"smart\", \"targeted\"."]
    pub fn placement_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement_status` after provisioning.\nAvailable values: \"SUCCESS\", \"UNSUPPORTED_APPLICATION\", \"INSUFFICIENT_INVOCATIONS\"."]
    pub fn placement_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `script_name` after provisioning.\nName of the script, used in URLs and route configuration."]
    pub fn script_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `startup_time_ms` after provisioning.\n"]
    pub fn startup_time_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_time_ms", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tail_consumers` after provisioning.\nList of Workers that will consume logs from the attached Worker."]
    pub fn tail_consumers(&self) -> terrars::SetRef<WorkersScript_TailConsumersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tail_consumers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `usage_model` after provisioning.\nUsage model for the Worker invocations.\nAvailable values: \"standard\", \"bundled\", \"unbound\"."]
    pub fn usage_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_model", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_Annotations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workers_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workers_tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkersScript_Annotations {
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

impl terrars::ToListMappable for WorkersScript_Annotations {
    type O = terrars::BlockAssignable<WorkersScript_Annotations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Annotations {}

impl BuildWorkersScript_Annotations {
    pub fn build(self) -> WorkersScript_Annotations {
        WorkersScript_Annotations {
            workers_message: core::default::Default::default(),
            workers_tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_AnnotationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_AnnotationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_AnnotationsRef {
        WorkersScript_AnnotationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_AnnotationsRef {
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
        "Get a reference to the value of field `workers_triggered_by` after provisioning.\nIndicates the trigger that created this version. Server-set value."]
    pub fn workers_triggered_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workers_triggered_by", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_Assets_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    html_handling: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_found_handling: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirects: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_worker_first: std::option::Option<terrars::PrimField<serde_json::Value>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serve_directly: std::option::Option<terrars::PrimField<bool>>,
}

impl WorkersScript_Assets_Config {
    #[doc =
        "Set the field `base_path`.\nThe public URL path prefix under which assets are served. A null request value resets it to `/`; responses represent the root as `/`. All versions in a gradual deployment must use the same canonical value. To change it, first deploy the version containing the change at 100%."]
    pub fn set_base_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.base_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `headers`.\nThe contents of a _headers file (used to attach custom headers on asset responses)."]
    pub fn set_headers(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.headers = std::option::Option::Some(v.into());
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
        "Set the field `redirects`.\nThe contents of a _redirects file (used to apply redirects or proxy paths ahead of asset serving)."]
    pub fn set_redirects(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.redirects = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `run_worker_first`.\nWhen a boolean true, requests will always invoke the Worker script. Otherwise, attempt to serve an asset matching the request, falling back to the Worker script. When a list of strings, contains path rules to control routing to either the Worker or assets. Glob (*) and negative (!) rules are supported. Rules must start with either '/' or '!/'. At least one non-negative rule must be provided, and negative rules have higher precedence than non-negative rules."]
    pub fn set_run_worker_first(mut self, v: impl std::convert::Into<terrars::PrimField<serde_json::Value>>) -> Self {
        self.run_worker_first = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `serve_directly`.\nWhen true and the incoming request matches an asset, that will be served instead of invoking the Worker script. When false, requests will always invoke the Worker script."]
    pub fn set_serve_directly(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.serve_directly = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Assets_Config {
    type O = terrars::BlockAssignable<WorkersScript_Assets_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Assets_Config {}

impl BuildWorkersScript_Assets_Config {
    pub fn build(self) -> WorkersScript_Assets_Config {
        WorkersScript_Assets_Config {
            base_path: core::default::Default::default(),
            headers: core::default::Default::default(),
            html_handling: core::default::Default::default(),
            not_found_handling: core::default::Default::default(),
            redirects: core::default::Default::default(),
            run_worker_first: core::default::Default::default(),
            serve_directly: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Assets_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Assets_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Assets_ConfigRef {
        WorkersScript_Assets_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Assets_ConfigRef {
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
        "Get a reference to the value of field `headers` after provisioning.\nThe contents of a _headers file (used to attach custom headers on asset responses)."]
    pub fn headers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.headers", self.base))
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
        "Get a reference to the value of field `redirects` after provisioning.\nThe contents of a _redirects file (used to apply redirects or proxy paths ahead of asset serving)."]
    pub fn redirects(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_worker_first` after provisioning.\nWhen a boolean true, requests will always invoke the Worker script. Otherwise, attempt to serve an asset matching the request, falling back to the Worker script. When a list of strings, contains path rules to control routing to either the Worker or assets. Glob (*) and negative (!) rules are supported. Rules must start with either '/' or '!/'. At least one non-negative rule must be provided, and negative rules have higher precedence than non-negative rules."]
    pub fn run_worker_first(&self) -> terrars::PrimExpr<serde_json::Value> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_worker_first", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serve_directly` after provisioning.\nWhen true and the incoming request matches an asset, that will be served instead of invoking the Worker script. When false, requests will always invoke the Worker script."]
    pub fn serve_directly(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serve_directly", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_Assets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<WorkersScript_Assets_Config>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkersScript_Assets {
    #[doc = "Set the field `config`.\nConfiguration for assets within a Worker."]
    pub fn set_config(mut self, v: impl std::convert::Into<WorkersScript_Assets_Config>) -> Self {
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

impl terrars::ToListMappable for WorkersScript_Assets {
    type O = terrars::BlockAssignable<WorkersScript_Assets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Assets {}

impl BuildWorkersScript_Assets {
    pub fn build(self) -> WorkersScript_Assets {
        WorkersScript_Assets {
            config: core::default::Default::default(),
            directory: core::default::Default::default(),
            jwt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_AssetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_AssetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_AssetsRef {
        WorkersScript_AssetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_AssetsRef {
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
    pub fn config(&self) -> WorkersScript_Assets_ConfigRef {
        WorkersScript_Assets_ConfigRef::new(self.shared().clone(), format!("{}.config", self.base))
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
pub struct WorkersScript_Bindings_Outbound_Worker {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkersScript_Bindings_Outbound_Worker {
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

impl terrars::ToListMappable for WorkersScript_Bindings_Outbound_Worker {
    type O = terrars::BlockAssignable<WorkersScript_Bindings_Outbound_Worker>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Bindings_Outbound_Worker {}

impl BuildWorkersScript_Bindings_Outbound_Worker {
    pub fn build(self) -> WorkersScript_Bindings_Outbound_Worker {
        WorkersScript_Bindings_Outbound_Worker {
            environment: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Bindings_Outbound_WorkerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Bindings_Outbound_WorkerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Bindings_Outbound_WorkerRef {
        WorkersScript_Bindings_Outbound_WorkerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Bindings_Outbound_WorkerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct WorkersScript_Bindings_Outbound {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker: std::option::Option<WorkersScript_Bindings_Outbound_Worker>,
}

impl WorkersScript_Bindings_Outbound {
    #[doc =
        "Set the field `params`.\nPass information from the Dispatch Worker to the Outbound Worker through the parameters."]
    pub fn set_params(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker`.\nOutbound worker."]
    pub fn set_worker(mut self, v: impl std::convert::Into<WorkersScript_Bindings_Outbound_Worker>) -> Self {
        self.worker = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Bindings_Outbound {
    type O = terrars::BlockAssignable<WorkersScript_Bindings_Outbound>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Bindings_Outbound {}

impl BuildWorkersScript_Bindings_Outbound {
    pub fn build(self) -> WorkersScript_Bindings_Outbound {
        WorkersScript_Bindings_Outbound {
            params: core::default::Default::default(),
            worker: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Bindings_OutboundRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Bindings_OutboundRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Bindings_OutboundRef {
        WorkersScript_Bindings_OutboundRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Bindings_OutboundRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nPass information from the Dispatch Worker to the Outbound Worker through the parameters."]
    pub fn params(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }

    #[doc = "Get a reference to the value of field `worker` after provisioning.\nOutbound worker."]
    pub fn worker(&self) -> WorkersScript_Bindings_Outbound_WorkerRef {
        WorkersScript_Bindings_Outbound_WorkerRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_Bindings_Retry {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_ms: std::option::Option<terrars::PrimField<f64>>,
}

impl WorkersScript_Bindings_Retry {
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

impl terrars::ToListMappable for WorkersScript_Bindings_Retry {
    type O = terrars::BlockAssignable<WorkersScript_Bindings_Retry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Bindings_Retry {}

impl BuildWorkersScript_Bindings_Retry {
    pub fn build(self) -> WorkersScript_Bindings_Retry {
        WorkersScript_Bindings_Retry {
            max_attempts: core::default::Default::default(),
            timeout_ms: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Bindings_RetryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Bindings_RetryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Bindings_RetryRef {
        WorkersScript_Bindings_RetryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Bindings_RetryRef {
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
pub struct WorkersScript_Bindings_Simple {
    limit: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mitigation_timeout: std::option::Option<terrars::PrimField<f64>>,
    period: terrars::PrimField<f64>,
}

impl WorkersScript_Bindings_Simple {
    #[doc =
        "Set the field `mitigation_timeout`.\nDuration in seconds to apply the mitigation action after the rate limit is exceeded. Valid values are 0 (disabled), 10, or multiples of 60 up to 86400. Must be greater than or equal to the period when non-zero."]
    pub fn set_mitigation_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mitigation_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Bindings_Simple {
    type O = terrars::BlockAssignable<WorkersScript_Bindings_Simple>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Bindings_Simple {
    #[doc = "The rate limit value."]
    pub limit: terrars::PrimField<f64>,
    #[doc = "The rate limit period in seconds."]
    pub period: terrars::PrimField<f64>,
}

impl BuildWorkersScript_Bindings_Simple {
    pub fn build(self) -> WorkersScript_Bindings_Simple {
        WorkersScript_Bindings_Simple {
            limit: self.limit,
            mitigation_timeout: core::default::Default::default(),
            period: self.period,
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Bindings_SimpleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Bindings_SimpleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Bindings_SimpleRef {
        WorkersScript_Bindings_SimpleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Bindings_SimpleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limit` after provisioning.\nThe rate limit value."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mitigation_timeout` after provisioning.\nDuration in seconds to apply the mitigation action after the rate limit is exceeded. Valid values are 0 (disabled), 10, or multiples of 60 up to 86400. Must be greater than or equal to the period when non-zero."]
    pub fn mitigation_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mitigation_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `period` after provisioning.\nThe rate limit period in seconds."]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_Bindings {
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
    outbound: std::option::Option<WorkersScript_Bindings_Outbound>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    part: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry: std::option::Option<WorkersScript_Bindings_Retry>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    simple: std::option::Option<WorkersScript_Bindings_Simple>,
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

impl WorkersScript_Bindings {
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
        "Set the field `jurisdiction`.\nThe [jurisdiction](https://developers.cloudflare.com/r2/reference/data-location/#jurisdictional-restrictions) of the R2 bucket.\nAvailable values: \"eu\", \"fedramp\", \"fedramp-high\"."]
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
    pub fn set_outbound(mut self, v: impl std::convert::Into<WorkersScript_Bindings_Outbound>) -> Self {
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
    pub fn set_retry(mut self, v: impl std::convert::Into<WorkersScript_Bindings_Retry>) -> Self {
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

    #[doc = "Set the field `simple`.\nA simple rate limit."]
    pub fn set_simple(mut self, v: impl std::convert::Into<WorkersScript_Bindings_Simple>) -> Self {
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

impl terrars::ToListMappable for WorkersScript_Bindings {
    type O = terrars::BlockAssignable<WorkersScript_Bindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Bindings {
    #[doc = "A JavaScript variable name for the binding."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The kind of resource that the binding provides.\nAvailable values: \"ai\", \"ai_search\", \"ai_search_namespace\", \"messaging\", \"analytics_engine\", \"artifacts\", \"assets\", \"browser\", \"d1\", \"data_blob\", \"dispatch_namespace\", \"durable_object_namespace\", \"hyperdrive\", \"inherit\", \"images\", \"json\", \"kv_namespace\", \"media\", \"mtls_certificate\", \"plain_text\", \"pipelines\", \"k2\", \"queue\", \"ratelimit\", \"r2_bucket\", \"secret_text\", \"send_email\", \"service\", \"text_blob\", \"vectorize\", \"version_metadata\", \"secrets_store_secret\", \"flagship\", \"secret_key\", \"workflow\", \"wasm_module\", \"vpc_service\", \"vpc_network\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWorkersScript_Bindings {
    pub fn build(self) -> WorkersScript_Bindings {
        WorkersScript_Bindings {
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
pub struct WorkersScript_BindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_BindingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_BindingsRef {
        WorkersScript_BindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_BindingsRef {
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
        "Get a reference to the value of field `jurisdiction` after provisioning.\nThe [jurisdiction](https://developers.cloudflare.com/r2/reference/data-location/#jurisdictional-restrictions) of the R2 bucket.\nAvailable values: \"eu\", \"fedramp\", \"fedramp-high\"."]
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
    pub fn outbound(&self) -> WorkersScript_Bindings_OutboundRef {
        WorkersScript_Bindings_OutboundRef::new(self.shared().clone(), format!("{}.outbound", self.base))
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
    pub fn retry(&self) -> WorkersScript_Bindings_RetryRef {
        WorkersScript_Bindings_RetryRef::new(self.shared().clone(), format!("{}.retry", self.base))
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

    #[doc = "Get a reference to the value of field `simple` after provisioning.\nA simple rate limit."]
    pub fn simple(&self) -> WorkersScript_Bindings_SimpleRef {
        WorkersScript_Bindings_SimpleRef::new(self.shared().clone(), format!("{}.simple", self.base))
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
pub struct WorkersScript_CacheOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_version_cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl WorkersScript_CacheOptions {
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

impl terrars::ToListMappable for WorkersScript_CacheOptions {
    type O = terrars::BlockAssignable<WorkersScript_CacheOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_CacheOptions {}

impl BuildWorkersScript_CacheOptions {
    pub fn build(self) -> WorkersScript_CacheOptions {
        WorkersScript_CacheOptions {
            cross_version_cache: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_CacheOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_CacheOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_CacheOptionsRef {
        WorkersScript_CacheOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_CacheOptionsRef {
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
pub struct WorkersScript_Exports_Cache {
    enabled: terrars::PrimField<bool>,
}

impl WorkersScript_Exports_Cache { }

impl terrars::ToListMappable for WorkersScript_Exports_Cache {
    type O = terrars::BlockAssignable<WorkersScript_Exports_Cache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Exports_Cache {
    #[doc = "Whether caching is enabled for this entrypoint."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildWorkersScript_Exports_Cache {
    pub fn build(self) -> WorkersScript_Exports_Cache {
        WorkersScript_Exports_Cache { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Exports_CacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Exports_CacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Exports_CacheRef {
        WorkersScript_Exports_CacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Exports_CacheRef {
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
pub struct WorkersScript_Exports {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache: std::option::Option<WorkersScript_Exports_Cache>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WorkersScript_Exports {
    #[doc =
        "Set the field `cache`.\nPer-entrypoint cache override. When present, this overrides the top-level `cache_options` for this specific entrypoint."]
    pub fn set_cache(mut self, v: impl std::convert::Into<WorkersScript_Exports_Cache>) -> Self {
        self.cache = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Exports {
    type O = terrars::BlockAssignable<WorkersScript_Exports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Exports {
    #[doc =
        "The kind of entrypoint. A `type: worker` entry overrides the top-level `cache_options` for this specific entrypoint."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWorkersScript_Exports {
    pub fn build(self) -> WorkersScript_Exports {
        WorkersScript_Exports {
            cache: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_ExportsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_ExportsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_ExportsRef {
        WorkersScript_ExportsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_ExportsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache` after provisioning.\nPer-entrypoint cache override. When present, this overrides the top-level `cache_options` for this specific entrypoint."]
    pub fn cache(&self) -> WorkersScript_Exports_CacheRef {
        WorkersScript_Exports_CacheRef::new(self.shared().clone(), format!("{}.cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe kind of entrypoint. A `type: worker` entry overrides the top-level `cache_options` for this specific entrypoint."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_Files {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_base64: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_file: std::option::Option<terrars::PrimField<std::string::String>>,
    content_type: terrars::PrimField<std::string::String>,
}

impl WorkersScript_Files {
    #[doc = "Set the field `content_base64`.\nBase64-encoded file content."]
    pub fn set_content_base64(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_base64 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_file`.\nPath to the file content."]
    pub fn set_content_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_file = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Files {
    type O = terrars::BlockAssignable<WorkersScript_Files>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Files {
    #[doc = "Content type of the file, such as `application/wasm`, `text/plain`, or `application/octet-stream`."]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildWorkersScript_Files {
    pub fn build(self) -> WorkersScript_Files {
        WorkersScript_Files {
            content_base64: core::default::Default::default(),
            content_file: core::default::Default::default(),
            content_type: self.content_type,
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_FilesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_FilesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_FilesRef {
        WorkersScript_FilesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_FilesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_base64` after provisioning.\nBase64-encoded file content."]
    pub fn content_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.base))
    }

    #[doc = "Get a reference to the value of field `content_file` after provisioning.\nPath to the file content."]
    pub fn content_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_sha256` after provisioning.\nSHA-256 hash of the file content, used to detect changes and remote drift."]
    pub fn content_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nContent type of the file, such as `application/wasm`, `text/plain`, or `application/octet-stream`."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_Limits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subrequests: std::option::Option<terrars::PrimField<f64>>,
}

impl WorkersScript_Limits {
    #[doc = "Set the field `cpu_ms`.\nThe amount of CPU time this Worker can use in milliseconds."]
    pub fn set_cpu_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subrequests`.\nThe number of subrequests this Worker can make per request."]
    pub fn set_subrequests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.subrequests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Limits {
    type O = terrars::BlockAssignable<WorkersScript_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Limits {}

impl BuildWorkersScript_Limits {
    pub fn build(self) -> WorkersScript_Limits {
        WorkersScript_Limits {
            cpu_ms: core::default::Default::default(),
            subrequests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_LimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_LimitsRef {
        WorkersScript_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu_ms` after provisioning.\nThe amount of CPU time this Worker can use in milliseconds."]
    pub fn cpu_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_ms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subrequests` after provisioning.\nThe number of subrequests this Worker can make per request."]
    pub fn subrequests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subrequests", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_Migrations_RenamedClasses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkersScript_Migrations_RenamedClasses {
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

impl terrars::ToListMappable for WorkersScript_Migrations_RenamedClasses {
    type O = terrars::BlockAssignable<WorkersScript_Migrations_RenamedClasses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Migrations_RenamedClasses {}

impl BuildWorkersScript_Migrations_RenamedClasses {
    pub fn build(self) -> WorkersScript_Migrations_RenamedClasses {
        WorkersScript_Migrations_RenamedClasses {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Migrations_RenamedClassesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Migrations_RenamedClassesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Migrations_RenamedClassesRef {
        WorkersScript_Migrations_RenamedClassesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Migrations_RenamedClassesRef {
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
pub struct WorkersScript_Migrations_Steps_RenamedClasses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkersScript_Migrations_Steps_RenamedClasses {
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

impl terrars::ToListMappable for WorkersScript_Migrations_Steps_RenamedClasses {
    type O = terrars::BlockAssignable<WorkersScript_Migrations_Steps_RenamedClasses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Migrations_Steps_RenamedClasses {}

impl BuildWorkersScript_Migrations_Steps_RenamedClasses {
    pub fn build(self) -> WorkersScript_Migrations_Steps_RenamedClasses {
        WorkersScript_Migrations_Steps_RenamedClasses {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Migrations_Steps_RenamedClassesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Migrations_Steps_RenamedClassesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkersScript_Migrations_Steps_RenamedClassesRef {
        WorkersScript_Migrations_Steps_RenamedClassesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Migrations_Steps_RenamedClassesRef {
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
pub struct WorkersScript_Migrations_Steps_TransferredClasses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkersScript_Migrations_Steps_TransferredClasses {
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

impl terrars::ToListMappable for WorkersScript_Migrations_Steps_TransferredClasses {
    type O = terrars::BlockAssignable<WorkersScript_Migrations_Steps_TransferredClasses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Migrations_Steps_TransferredClasses {}

impl BuildWorkersScript_Migrations_Steps_TransferredClasses {
    pub fn build(self) -> WorkersScript_Migrations_Steps_TransferredClasses {
        WorkersScript_Migrations_Steps_TransferredClasses {
            from: core::default::Default::default(),
            from_script: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Migrations_Steps_TransferredClassesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Migrations_Steps_TransferredClassesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkersScript_Migrations_Steps_TransferredClassesRef {
        WorkersScript_Migrations_Steps_TransferredClassesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Migrations_Steps_TransferredClassesRef {
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
pub struct WorkersScript_Migrations_Steps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_sqlite_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    renamed_classes: std::option::Option<std::vec::Vec<WorkersScript_Migrations_Steps_RenamedClasses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transferred_classes: std::option::Option<std::vec::Vec<WorkersScript_Migrations_Steps_TransferredClasses>>,
}

impl WorkersScript_Migrations_Steps {
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
        v: impl std::convert::Into<std::vec::Vec<WorkersScript_Migrations_Steps_RenamedClasses>>,
    ) -> Self {
        self.renamed_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transferred_classes`.\nA list of transfers for Durable Object namespaces from a different Worker and class to a class defined in this Worker."]
    pub fn set_transferred_classes(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<WorkersScript_Migrations_Steps_TransferredClasses>>,
    ) -> Self {
        self.transferred_classes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Migrations_Steps {
    type O = terrars::BlockAssignable<WorkersScript_Migrations_Steps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Migrations_Steps {}

impl BuildWorkersScript_Migrations_Steps {
    pub fn build(self) -> WorkersScript_Migrations_Steps {
        WorkersScript_Migrations_Steps {
            deleted_classes: core::default::Default::default(),
            new_classes: core::default::Default::default(),
            new_sqlite_classes: core::default::Default::default(),
            renamed_classes: core::default::Default::default(),
            transferred_classes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Migrations_StepsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Migrations_StepsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Migrations_StepsRef {
        WorkersScript_Migrations_StepsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Migrations_StepsRef {
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
    pub fn renamed_classes(&self) -> terrars::ListRef<WorkersScript_Migrations_Steps_RenamedClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.renamed_classes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transferred_classes` after provisioning.\nA list of transfers for Durable Object namespaces from a different Worker and class to a class defined in this Worker."]
    pub fn transferred_classes(&self) -> terrars::ListRef<WorkersScript_Migrations_Steps_TransferredClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transferred_classes", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_Migrations_TransferredClasses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkersScript_Migrations_TransferredClasses {
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

impl terrars::ToListMappable for WorkersScript_Migrations_TransferredClasses {
    type O = terrars::BlockAssignable<WorkersScript_Migrations_TransferredClasses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Migrations_TransferredClasses {}

impl BuildWorkersScript_Migrations_TransferredClasses {
    pub fn build(self) -> WorkersScript_Migrations_TransferredClasses {
        WorkersScript_Migrations_TransferredClasses {
            from: core::default::Default::default(),
            from_script: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Migrations_TransferredClassesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Migrations_TransferredClassesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkersScript_Migrations_TransferredClassesRef {
        WorkersScript_Migrations_TransferredClassesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Migrations_TransferredClassesRef {
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
pub struct WorkersScript_Migrations {
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
    renamed_classes: std::option::Option<std::vec::Vec<WorkersScript_Migrations_RenamedClasses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std::option::Option<std::vec::Vec<WorkersScript_Migrations_Steps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transferred_classes: std::option::Option<std::vec::Vec<WorkersScript_Migrations_TransferredClasses>>,
}

impl WorkersScript_Migrations {
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
        v: impl std::convert::Into<std::vec::Vec<WorkersScript_Migrations_RenamedClasses>>,
    ) -> Self {
        self.renamed_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `steps`.\nMigrations to apply in order."]
    pub fn set_steps(mut self, v: impl std::convert::Into<std::vec::Vec<WorkersScript_Migrations_Steps>>) -> Self {
        self.steps = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transferred_classes`.\nA list of transfers for Durable Object namespaces from a different Worker and class to a class defined in this Worker."]
    pub fn set_transferred_classes(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<WorkersScript_Migrations_TransferredClasses>>,
    ) -> Self {
        self.transferred_classes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Migrations {
    type O = terrars::BlockAssignable<WorkersScript_Migrations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Migrations {}

impl BuildWorkersScript_Migrations {
    pub fn build(self) -> WorkersScript_Migrations {
        WorkersScript_Migrations {
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
pub struct WorkersScript_MigrationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_MigrationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_MigrationsRef {
        WorkersScript_MigrationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_MigrationsRef {
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
    pub fn renamed_classes(&self) -> terrars::ListRef<WorkersScript_Migrations_RenamedClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.renamed_classes", self.base))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\nMigrations to apply in order."]
    pub fn steps(&self) -> terrars::ListRef<WorkersScript_Migrations_StepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transferred_classes` after provisioning.\nA list of transfers for Durable Object namespaces from a different Worker and class to a class defined in this Worker."]
    pub fn transferred_classes(&self) -> terrars::ListRef<WorkersScript_Migrations_TransferredClassesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transferred_classes", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_NamedHandlers {}

impl WorkersScript_NamedHandlers { }

impl terrars::ToListMappable for WorkersScript_NamedHandlers {
    type O = terrars::BlockAssignable<WorkersScript_NamedHandlers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_NamedHandlers {}

impl BuildWorkersScript_NamedHandlers {
    pub fn build(self) -> WorkersScript_NamedHandlers {
        WorkersScript_NamedHandlers {}
    }
}

#[allow(dead_code)]
pub struct WorkersScript_NamedHandlersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_NamedHandlersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_NamedHandlersRef {
        WorkersScript_NamedHandlersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_NamedHandlersRef {
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
pub struct WorkersScript_Observability_Issues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl WorkersScript_Observability_Issues {
    #[doc = "Set the field `enabled`.\nWhether real-time Issues are enabled for the Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Observability_Issues {
    type O = terrars::BlockAssignable<WorkersScript_Observability_Issues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Observability_Issues {}

impl BuildWorkersScript_Observability_Issues {
    pub fn build(self) -> WorkersScript_Observability_Issues {
        WorkersScript_Observability_Issues { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Observability_IssuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Observability_IssuesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Observability_IssuesRef {
        WorkersScript_Observability_IssuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Observability_IssuesRef {
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
pub struct WorkersScript_Observability_Logs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    head_sampling_rate: std::option::Option<terrars::PrimField<f64>>,
    invocation_logs: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persist: std::option::Option<terrars::PrimField<bool>>,
}

impl WorkersScript_Observability_Logs {
    #[doc = "Set the field `destinations`.\nA list of destinations where logs will be exported to."]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destinations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `head_sampling_rate`.\nThe sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1."]
    pub fn set_head_sampling_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.head_sampling_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `persist`.\nWhether log persistence is enabled for the Worker."]
    pub fn set_persist(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.persist = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Observability_Logs {
    type O = terrars::BlockAssignable<WorkersScript_Observability_Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Observability_Logs {
    #[doc = "Whether logs are enabled for the Worker."]
    pub enabled: terrars::PrimField<bool>,
    #[doc =
        "Whether [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs) are enabled for the Worker."]
    pub invocation_logs: terrars::PrimField<bool>,
}

impl BuildWorkersScript_Observability_Logs {
    pub fn build(self) -> WorkersScript_Observability_Logs {
        WorkersScript_Observability_Logs {
            destinations: core::default::Default::default(),
            enabled: self.enabled,
            head_sampling_rate: core::default::Default::default(),
            invocation_logs: self.invocation_logs,
            persist: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Observability_LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Observability_LogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Observability_LogsRef {
        WorkersScript_Observability_LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Observability_LogsRef {
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
pub struct WorkersScript_Observability_Traces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    head_sampling_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persist: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagation_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkersScript_Observability_Traces {
    #[doc = "Set the field `destinations`.\nA list of destinations where traces will be exported to."]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destinations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether traces are enabled for the Worker."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `head_sampling_rate`.\nThe sampling rate for traces. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1."]
    pub fn set_head_sampling_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.head_sampling_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `persist`.\nWhether trace persistence is enabled for the Worker."]
    pub fn set_persist(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.persist = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `propagation_policy`.\nControls how inbound trace context (traceparent/tracestate) headers on incoming requests are handled. \"authenticated\" (default) honors inbound trace context only when accompanied by a valid trace auth token. \"accept\" unconditionally accepts inbound trace context. Requires the trace propagation feature to be enabled.\nAvailable values: \"authenticated\", \"accept\"."]
    pub fn set_propagation_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.propagation_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Observability_Traces {
    type O = terrars::BlockAssignable<WorkersScript_Observability_Traces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Observability_Traces {}

impl BuildWorkersScript_Observability_Traces {
    pub fn build(self) -> WorkersScript_Observability_Traces {
        WorkersScript_Observability_Traces {
            destinations: core::default::Default::default(),
            enabled: core::default::Default::default(),
            head_sampling_rate: core::default::Default::default(),
            persist: core::default::Default::default(),
            propagation_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Observability_TracesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Observability_TracesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Observability_TracesRef {
        WorkersScript_Observability_TracesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Observability_TracesRef {
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
        "Get a reference to the value of field `propagation_policy` after provisioning.\nControls how inbound trace context (traceparent/tracestate) headers on incoming requests are handled. \"authenticated\" (default) honors inbound trace context only when accompanied by a valid trace auth token. \"accept\" unconditionally accepts inbound trace context. Requires the trace propagation feature to be enabled.\nAvailable values: \"authenticated\", \"accept\"."]
    pub fn propagation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagation_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_Observability {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    head_sampling_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issues: std::option::Option<WorkersScript_Observability_Issues>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs: std::option::Option<WorkersScript_Observability_Logs>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traces: std::option::Option<WorkersScript_Observability_Traces>,
}

impl WorkersScript_Observability {
    #[doc =
        "Set the field `head_sampling_rate`.\nThe sampling rate for incoming requests. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1."]
    pub fn set_head_sampling_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.head_sampling_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issues`.\nReal-time Issues settings for the Worker."]
    pub fn set_issues(mut self, v: impl std::convert::Into<WorkersScript_Observability_Issues>) -> Self {
        self.issues = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logs`.\nLog settings for the Worker."]
    pub fn set_logs(mut self, v: impl std::convert::Into<WorkersScript_Observability_Logs>) -> Self {
        self.logs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traces`.\nTrace settings for the Worker."]
    pub fn set_traces(mut self, v: impl std::convert::Into<WorkersScript_Observability_Traces>) -> Self {
        self.traces = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Observability {
    type O = terrars::BlockAssignable<WorkersScript_Observability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Observability {
    #[doc = "Whether observability is enabled for the Worker."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildWorkersScript_Observability {
    pub fn build(self) -> WorkersScript_Observability {
        WorkersScript_Observability {
            enabled: self.enabled,
            head_sampling_rate: core::default::Default::default(),
            issues: core::default::Default::default(),
            logs: core::default::Default::default(),
            traces: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_ObservabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_ObservabilityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_ObservabilityRef {
        WorkersScript_ObservabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_ObservabilityRef {
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
    pub fn issues(&self) -> WorkersScript_Observability_IssuesRef {
        WorkersScript_Observability_IssuesRef::new(self.shared().clone(), format!("{}.issues", self.base))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\nLog settings for the Worker."]
    pub fn logs(&self) -> WorkersScript_Observability_LogsRef {
        WorkersScript_Observability_LogsRef::new(self.shared().clone(), format!("{}.logs", self.base))
    }

    #[doc = "Get a reference to the value of field `traces` after provisioning.\nTrace settings for the Worker."]
    pub fn traces(&self) -> WorkersScript_Observability_TracesRef {
        WorkersScript_Observability_TracesRef::new(self.shared().clone(), format!("{}.traces", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_PackageDependencies {
    installed_version: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    package_json_version: terrars::PrimField<std::string::String>,
}

impl WorkersScript_PackageDependencies { }

impl terrars::ToListMappable for WorkersScript_PackageDependencies {
    type O = terrars::BlockAssignable<WorkersScript_PackageDependencies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_PackageDependencies {
    #[doc = "The exact version that was resolved and installed by the package manager."]
    pub installed_version: terrars::PrimField<std::string::String>,
    #[doc = "The npm package name."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The version constraint as written in package.json."]
    pub package_json_version: terrars::PrimField<std::string::String>,
}

impl BuildWorkersScript_PackageDependencies {
    pub fn build(self) -> WorkersScript_PackageDependencies {
        WorkersScript_PackageDependencies {
            installed_version: self.installed_version,
            name: self.name,
            package_json_version: self.package_json_version,
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_PackageDependenciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_PackageDependenciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_PackageDependenciesRef {
        WorkersScript_PackageDependenciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_PackageDependenciesRef {
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
pub struct WorkersScript_Placement_Target {}

impl WorkersScript_Placement_Target { }

impl terrars::ToListMappable for WorkersScript_Placement_Target {
    type O = terrars::BlockAssignable<WorkersScript_Placement_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Placement_Target {}

impl BuildWorkersScript_Placement_Target {
    pub fn build(self) -> WorkersScript_Placement_Target {
        WorkersScript_Placement_Target {}
    }
}

#[allow(dead_code)]
pub struct WorkersScript_Placement_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_Placement_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_Placement_TargetRef {
        WorkersScript_Placement_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_Placement_TargetRef {
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
pub struct WorkersScript_Placement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkersScript_Placement {
    #[doc =
        "Set the field `mode`.\nEnables [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).\nAvailable values: \"smart\", \"targeted\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_Placement {
    type O = terrars::BlockAssignable<WorkersScript_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_Placement {}

impl BuildWorkersScript_Placement {
    pub fn build(self) -> WorkersScript_Placement {
        WorkersScript_Placement { mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_PlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_PlacementRef {
        WorkersScript_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_PlacementRef {
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
    pub fn target(&self) -> terrars::ListRef<WorkersScript_Placement_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkersScript_TailConsumers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
}

impl WorkersScript_TailConsumers {
    #[doc = "Set the field `environment`.\nOptional environment if the Worker utilizes one."]
    pub fn set_environment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\nOptional dispatch namespace the script belongs to."]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkersScript_TailConsumers {
    type O = terrars::BlockAssignable<WorkersScript_TailConsumers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkersScript_TailConsumers {
    #[doc = "Name of Worker that is to be the consumer."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildWorkersScript_TailConsumers {
    pub fn build(self) -> WorkersScript_TailConsumers {
        WorkersScript_TailConsumers {
            environment: core::default::Default::default(),
            namespace: core::default::Default::default(),
            service: self.service,
        }
    }
}

#[allow(dead_code)]
pub struct WorkersScript_TailConsumersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkersScript_TailConsumersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkersScript_TailConsumersRef {
        WorkersScript_TailConsumersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkersScript_TailConsumersRef {
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
