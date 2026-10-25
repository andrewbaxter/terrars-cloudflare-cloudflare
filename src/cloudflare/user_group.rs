#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct UserGroupData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policies: std::option::Option<std::vec::Vec<UserGroup_Policies>>,
}

struct UserGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<UserGroupData_>,
}

#[derive(Clone)]
pub struct UserGroup(std::rc::Rc<UserGroup_>);

impl UserGroup {
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

    #[doc = "Set the field `policies`.\nPolicies attached to the User group"]
    pub fn set_policies(self, v: impl std::convert::Into<std::vec::Vec<UserGroup_Policies>>) -> Self {
        self.0.data.borrow_mut().policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nTimestamp for the creation of the user group"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUser Group identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nLast time the user group was modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the User group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\nPolicies attached to the User group"]
    pub fn policies(&self) -> terrars::ListRef<UserGroup_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }
}

impl terrars::Referable for UserGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for UserGroup { }

impl terrars::ToListMappable for UserGroup {
    type O = terrars::ListRef<UserGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for UserGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_user_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildUserGroup {
    pub tf_id: std::string::String,
    #[doc = "Account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the User group."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildUserGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> UserGroup {
        let out = UserGroup(std::rc::Rc::new(UserGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(UserGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                name: self.name,
                policies: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct UserGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for UserGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl UserGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nTimestamp for the creation of the user group"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUser Group identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nLast time the user group was modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the User group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\nPolicies attached to the User group"]
    pub fn policies(&self) -> terrars::ListRef<UserGroup_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct UserGroup_Policies_PermissionGroups {
    id: terrars::PrimField<std::string::String>,
}

impl UserGroup_Policies_PermissionGroups { }

impl terrars::ToListMappable for UserGroup_Policies_PermissionGroups {
    type O = terrars::BlockAssignable<UserGroup_Policies_PermissionGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildUserGroup_Policies_PermissionGroups {
    #[doc = "Permission Group identifier tag."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildUserGroup_Policies_PermissionGroups {
    pub fn build(self) -> UserGroup_Policies_PermissionGroups {
        UserGroup_Policies_PermissionGroups { id: self.id }
    }
}

#[allow(dead_code)]
pub struct UserGroup_Policies_PermissionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for UserGroup_Policies_PermissionGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> UserGroup_Policies_PermissionGroupsRef {
        UserGroup_Policies_PermissionGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl UserGroup_Policies_PermissionGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nPermission Group identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct UserGroup_Policies_ResourceGroups {
    id: terrars::PrimField<std::string::String>,
}

impl UserGroup_Policies_ResourceGroups { }

impl terrars::ToListMappable for UserGroup_Policies_ResourceGroups {
    type O = terrars::BlockAssignable<UserGroup_Policies_ResourceGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildUserGroup_Policies_ResourceGroups {
    #[doc = "Resource Group identifier tag."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildUserGroup_Policies_ResourceGroups {
    pub fn build(self) -> UserGroup_Policies_ResourceGroups {
        UserGroup_Policies_ResourceGroups { id: self.id }
    }
}

#[allow(dead_code)]
pub struct UserGroup_Policies_ResourceGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for UserGroup_Policies_ResourceGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> UserGroup_Policies_ResourceGroupsRef {
        UserGroup_Policies_ResourceGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl UserGroup_Policies_ResourceGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nResource Group identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct UserGroup_Policies {
    access: terrars::PrimField<std::string::String>,
    permission_groups: std::vec::Vec<UserGroup_Policies_PermissionGroups>,
    resource_groups: std::vec::Vec<UserGroup_Policies_ResourceGroups>,
}

impl UserGroup_Policies { }

impl terrars::ToListMappable for UserGroup_Policies {
    type O = terrars::BlockAssignable<UserGroup_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildUserGroup_Policies {
    #[doc = "Allow or deny operations against the resources.\nAvailable values: \"allow\", \"deny\"."]
    pub access: terrars::PrimField<std::string::String>,
    #[doc = "A set of permission groups that are specified to the policy."]
    pub permission_groups: std::vec::Vec<UserGroup_Policies_PermissionGroups>,
    #[doc = "A set of resource groups that are specified to the policy."]
    pub resource_groups: std::vec::Vec<UserGroup_Policies_ResourceGroups>,
}

impl BuildUserGroup_Policies {
    pub fn build(self) -> UserGroup_Policies {
        UserGroup_Policies {
            access: self.access,
            permission_groups: self.permission_groups,
            resource_groups: self.resource_groups,
        }
    }
}

#[allow(dead_code)]
pub struct UserGroup_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for UserGroup_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> UserGroup_PoliciesRef {
        UserGroup_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl UserGroup_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access` after provisioning.\nAllow or deny operations against the resources.\nAvailable values: \"allow\", \"deny\"."]
    pub fn access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission_groups` after provisioning.\nA set of permission groups that are specified to the policy."]
    pub fn permission_groups(&self) -> terrars::ListRef<UserGroup_Policies_PermissionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_groups` after provisioning.\nA set of resource groups that are specified to the policy."]
    pub fn resource_groups(&self) -> terrars::ListRef<UserGroup_Policies_ResourceGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_groups", self.base))
    }
}
