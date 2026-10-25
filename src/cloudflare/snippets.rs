#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct SnippetsData_ {
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
    files: terrars::ListField<terrars::PrimField<std::string::String>>,
    metadata: Snippets_Metadata,
    snippet_name: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct Snippets_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SnippetsData_>,
}

#[derive(Clone)]
pub struct Snippets(std::rc::Rc<Snippets_>);

impl Snippets {
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

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe timestamp of when the snippet was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `files` after provisioning.\nThe list of files belonging to the snippet."]
    pub fn files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.files", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nMetadata about the snippet."]
    pub fn metadata(&self) -> Snippets_MetadataRef {
        Snippets_MetadataRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the snippet was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `snippet_name` after provisioning.\nThe identifying name of the snippet."]
    pub fn snippet_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snippet_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nThe unique ID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for Snippets {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Snippets { }

impl terrars::ToListMappable for Snippets {
    type O = terrars::ListRef<SnippetsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Snippets_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_snippets".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSnippets {
    pub tf_id: std::string::String,
    #[doc = "The list of files belonging to the snippet."]
    pub files: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Metadata about the snippet."]
    pub metadata: Snippets_Metadata,
    #[doc = "The identifying name of the snippet."]
    pub snippet_name: terrars::PrimField<std::string::String>,
    #[doc = "The unique ID of the zone."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildSnippets {
    pub fn build(self, stack: &mut terrars::Stack) -> Snippets {
        let out = Snippets(std::rc::Rc::new(Snippets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SnippetsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                files: self.files,
                metadata: self.metadata,
                snippet_name: self.snippet_name,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SnippetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SnippetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SnippetsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nThe timestamp of when the snippet was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `files` after provisioning.\nThe list of files belonging to the snippet."]
    pub fn files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.files", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nMetadata about the snippet."]
    pub fn metadata(&self) -> Snippets_MetadataRef {
        Snippets_MetadataRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nThe timestamp of when the snippet was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `snippet_name` after provisioning.\nThe identifying name of the snippet."]
    pub fn snippet_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snippet_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nThe unique ID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Snippets_Metadata {
    main_module: terrars::PrimField<std::string::String>,
}

impl Snippets_Metadata { }

impl terrars::ToListMappable for Snippets_Metadata {
    type O = terrars::BlockAssignable<Snippets_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSnippets_Metadata {
    #[doc = "Name of the file that contains the main module of the snippet."]
    pub main_module: terrars::PrimField<std::string::String>,
}

impl BuildSnippets_Metadata {
    pub fn build(self) -> Snippets_Metadata {
        Snippets_Metadata { main_module: self.main_module }
    }
}

#[allow(dead_code)]
pub struct Snippets_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Snippets_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Snippets_MetadataRef {
        Snippets_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Snippets_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `main_module` after provisioning.\nName of the file that contains the main module of the snippet."]
    pub fn main_module(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_module", self.base))
    }
}
