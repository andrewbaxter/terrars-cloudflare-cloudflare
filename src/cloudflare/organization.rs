#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct OrganizationData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<Organization_Parent>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile: std::option::Option<Organization_Profile>,
}

struct Organization_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OrganizationData_>,
}

#[derive(Clone)]
pub struct Organization(std::rc::Rc<Organization_>);

impl Organization {
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

    #[doc = "Set the field `parent`.\n"]
    pub fn set_parent(self, v: impl std::convert::Into<Organization_Parent>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile`.\n"]
    pub fn set_profile(self, v: impl std::convert::Into<Organization_Profile>) -> Self {
        self.0.data.borrow_mut().profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\n"]
    pub fn meta(&self) -> Organization_MetaRef {
        Organization_MetaRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> Organization_ParentRef {
        Organization_ParentRef::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> Organization_ProfileRef {
        Organization_ProfileRef::new(self.shared().clone(), format!("{}.profile", self.extract_ref()))
    }
}

impl terrars::Referable for Organization {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Organization { }

impl terrars::ToListMappable for Organization {
    type O = terrars::ListRef<OrganizationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Organization_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_organization".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOrganization {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildOrganization {
    pub fn build(self, stack: &mut terrars::Stack) -> Organization {
        let out = Organization(std::rc::Rc::new(Organization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OrganizationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                name: self.name,
                parent: core::default::Default::default(),
                profile: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrganizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OrganizationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\n"]
    pub fn meta(&self) -> Organization_MetaRef {
        Organization_MetaRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> Organization_ParentRef {
        Organization_ParentRef::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> Organization_ProfileRef {
        Organization_ProfileRef::new(self.shared().clone(), format!("{}.profile", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Organization_Meta_TenantFlags {}

impl Organization_Meta_TenantFlags { }

impl terrars::ToListMappable for Organization_Meta_TenantFlags {
    type O = terrars::BlockAssignable<Organization_Meta_TenantFlags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganization_Meta_TenantFlags {}

impl BuildOrganization_Meta_TenantFlags {
    pub fn build(self) -> Organization_Meta_TenantFlags {
        Organization_Meta_TenantFlags {}
    }
}

#[allow(dead_code)]
pub struct Organization_Meta_TenantFlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Organization_Meta_TenantFlagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Organization_Meta_TenantFlagsRef {
        Organization_Meta_TenantFlagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Organization_Meta_TenantFlagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_creation` after provisioning.\n"]
    pub fn account_creation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_creation", self.base))
    }

    #[doc = "Get a reference to the value of field `account_creation_applies_tenant_defaults` after provisioning.\n"]
    pub fn account_creation_applies_tenant_defaults(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_creation_applies_tenant_defaults", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `account_deletion` after provisioning.\n"]
    pub fn account_deletion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_deletion", self.base))
    }

    #[doc = "Get a reference to the value of field `account_migration` after provisioning.\n"]
    pub fn account_migration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_migration", self.base))
    }

    #[doc = "Get a reference to the value of field `account_mobility` after provisioning.\n"]
    pub fn account_mobility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_mobility", self.base))
    }

    #[doc = "Get a reference to the value of field `enterprise_capability` after provisioning.\n"]
    pub fn enterprise_capability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enterprise_capability", self.base))
    }

    #[doc = "Get a reference to the value of field `member_management` after provisioning.\n"]
    pub fn member_management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.member_management", self.base))
    }

    #[doc = "Get a reference to the value of field `sub_org_creation` after provisioning.\n"]
    pub fn sub_org_creation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sub_org_creation", self.base))
    }
}

#[derive(Serialize)]
pub struct Organization_Meta {}

impl Organization_Meta { }

impl terrars::ToListMappable for Organization_Meta {
    type O = terrars::BlockAssignable<Organization_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganization_Meta {}

impl BuildOrganization_Meta {
    pub fn build(self) -> Organization_Meta {
        Organization_Meta {}
    }
}

#[allow(dead_code)]
pub struct Organization_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Organization_MetaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Organization_MetaRef {
        Organization_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Organization_MetaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hierarchy_tags` after provisioning.\nOrdered chain of organization tags from the root organization down to\n(and including) this organization itself. Root organizations return a\nsingle-element array containing their own tag; sub-organizations return\n`[rootTag, ...intermediateTags, parentTag, selfTag]`. Useful for\nconstructing authorization scopes that need to cover every ancestor\nin the hierarchy."]
    pub fn hierarchy_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\n"]
    pub fn managed_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_flags` after provisioning.\nEnable features for Organizations."]
    pub fn tenant_flags(&self) -> Organization_Meta_TenantFlagsRef {
        Organization_Meta_TenantFlagsRef::new(self.shared().clone(), format!("{}.tenant_flags", self.base))
    }
}

#[derive(Serialize)]
pub struct Organization_Parent {
    id: terrars::PrimField<std::string::String>,
}

impl Organization_Parent { }

impl terrars::ToListMappable for Organization_Parent {
    type O = terrars::BlockAssignable<Organization_Parent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganization_Parent {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildOrganization_Parent {
    pub fn build(self) -> Organization_Parent {
        Organization_Parent { id: self.id }
    }
}

#[allow(dead_code)]
pub struct Organization_ParentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Organization_ParentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Organization_ParentRef {
        Organization_ParentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Organization_ParentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Organization_Profile {
    business_address: terrars::PrimField<std::string::String>,
    business_email: terrars::PrimField<std::string::String>,
    business_name: terrars::PrimField<std::string::String>,
    business_phone: terrars::PrimField<std::string::String>,
    external_metadata: terrars::PrimField<std::string::String>,
}

impl Organization_Profile { }

impl terrars::ToListMappable for Organization_Profile {
    type O = terrars::BlockAssignable<Organization_Profile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganization_Profile {
    #[doc = ""]
    pub business_address: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub business_email: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub business_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub business_phone: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub external_metadata: terrars::PrimField<std::string::String>,
}

impl BuildOrganization_Profile {
    pub fn build(self) -> Organization_Profile {
        Organization_Profile {
            business_address: self.business_address,
            business_email: self.business_email,
            business_name: self.business_name,
            business_phone: self.business_phone,
            external_metadata: self.external_metadata,
        }
    }
}

#[allow(dead_code)]
pub struct Organization_ProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Organization_ProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Organization_ProfileRef {
        Organization_ProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Organization_ProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `business_address` after provisioning.\n"]
    pub fn business_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_address", self.base))
    }

    #[doc = "Get a reference to the value of field `business_email` after provisioning.\n"]
    pub fn business_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_email", self.base))
    }

    #[doc = "Get a reference to the value of field `business_name` after provisioning.\n"]
    pub fn business_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_name", self.base))
    }

    #[doc = "Get a reference to the value of field `business_phone` after provisioning.\n"]
    pub fn business_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_phone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_metadata` after provisioning.\n"]
    pub fn external_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_metadata", self.base))
    }
}
