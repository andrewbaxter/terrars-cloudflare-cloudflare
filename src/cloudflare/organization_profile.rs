#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct OrganizationProfileData_ {
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
    business_address: terrars::PrimField<std::string::String>,
    business_email: terrars::PrimField<std::string::String>,
    business_name: terrars::PrimField<std::string::String>,
    business_phone: terrars::PrimField<std::string::String>,
    external_metadata: terrars::PrimField<std::string::String>,
    organization_id: terrars::PrimField<std::string::String>,
}

struct OrganizationProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OrganizationProfileData_>,
}

#[derive(Clone)]
pub struct OrganizationProfile(std::rc::Rc<OrganizationProfile_>);

impl OrganizationProfile {
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

    #[doc = "Get a reference to the value of field `business_address` after provisioning.\n"]
    pub fn business_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_email` after provisioning.\n"]
    pub fn business_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_name` after provisioning.\n"]
    pub fn business_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_phone` after provisioning.\n"]
    pub fn business_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_phone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_metadata` after provisioning.\n"]
    pub fn external_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\n"]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }
}

impl terrars::Referable for OrganizationProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OrganizationProfile { }

impl terrars::ToListMappable for OrganizationProfile {
    type O = terrars::ListRef<OrganizationProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OrganizationProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_organization_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOrganizationProfile {
    pub tf_id: std::string::String,
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
    #[doc = ""]
    pub organization_id: terrars::PrimField<std::string::String>,
}

impl BuildOrganizationProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> OrganizationProfile {
        let out = OrganizationProfile(std::rc::Rc::new(OrganizationProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OrganizationProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                business_address: self.business_address,
                business_email: self.business_email,
                business_name: self.business_name,
                business_phone: self.business_phone,
                external_metadata: self.external_metadata,
                organization_id: self.organization_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OrganizationProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrganizationProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OrganizationProfileRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `business_address` after provisioning.\n"]
    pub fn business_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_email` after provisioning.\n"]
    pub fn business_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_name` after provisioning.\n"]
    pub fn business_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_phone` after provisioning.\n"]
    pub fn business_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_phone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_metadata` after provisioning.\n"]
    pub fn external_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\n"]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }
}
