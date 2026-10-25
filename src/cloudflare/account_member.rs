#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccountMemberData_ {
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
    email: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policies: std::option::Option<std::vec::Vec<AccountMember_Policies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct AccountMember_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccountMemberData_>,
}

#[derive(Clone)]
pub struct AccountMember(std::rc::Rc<AccountMember_>);

impl AccountMember {
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

    #[doc = "Set the field `policies`.\nArray of policies associated with this member."]
    pub fn set_policies(self, v: impl std::convert::Into<std::vec::Vec<AccountMember_Policies>>) -> Self {
        self.0.data.borrow_mut().policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `roles`.\nSet of roles associated with this member."]
    pub fn set_roles(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().roles = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nStatus of the member invitation. If not provided during creation, defaults to 'pending'.\nChanging from 'accepted' back to 'pending' will trigger a replacement of the member resource in Terraform.\nAvailable values: \"accepted\", \"pending\"."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe contact email address of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nMembership identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policies` after provisioning.\nArray of policies associated with this member."]
    pub fn policies(&self) -> terrars::SetRef<AccountMember_PoliciesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `roles` after provisioning.\nSet of roles associated with this member."]
    pub fn roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the member invitation. If not provided during creation, defaults to 'pending'.\nChanging from 'accepted' back to 'pending' will trigger a replacement of the member resource in Terraform.\nAvailable values: \"accepted\", \"pending\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nDetails of the user associated to the membership."]
    pub fn user(&self) -> AccountMember_UserRef {
        AccountMember_UserRef::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}

impl terrars::Referable for AccountMember {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccountMember { }

impl terrars::ToListMappable for AccountMember {
    type O = terrars::ListRef<AccountMemberRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccountMember_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_account_member".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccountMember {
    pub tf_id: std::string::String,
    #[doc = "Account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "The contact email address of the user."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildAccountMember {
    pub fn build(self, stack: &mut terrars::Stack) -> AccountMember {
        let out = AccountMember(std::rc::Rc::new(AccountMember_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccountMemberData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                email: self.email,
                policies: core::default::Default::default(),
                roles: core::default::Default::default(),
                status: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccountMemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountMemberRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccountMemberRef {
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

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe contact email address of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nMembership identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policies` after provisioning.\nArray of policies associated with this member."]
    pub fn policies(&self) -> terrars::SetRef<AccountMember_PoliciesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `roles` after provisioning.\nSet of roles associated with this member."]
    pub fn roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the member invitation. If not provided during creation, defaults to 'pending'.\nChanging from 'accepted' back to 'pending' will trigger a replacement of the member resource in Terraform.\nAvailable values: \"accepted\", \"pending\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nDetails of the user associated to the membership."]
    pub fn user(&self) -> AccountMember_UserRef {
        AccountMember_UserRef::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccountMember_Policies_PermissionGroups {
    id: terrars::PrimField<std::string::String>,
}

impl AccountMember_Policies_PermissionGroups { }

impl terrars::ToListMappable for AccountMember_Policies_PermissionGroups {
    type O = terrars::BlockAssignable<AccountMember_Policies_PermissionGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccountMember_Policies_PermissionGroups {
    #[doc = "Identifier of the group."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAccountMember_Policies_PermissionGroups {
    pub fn build(self) -> AccountMember_Policies_PermissionGroups {
        AccountMember_Policies_PermissionGroups { id: self.id }
    }
}

#[allow(dead_code)]
pub struct AccountMember_Policies_PermissionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountMember_Policies_PermissionGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccountMember_Policies_PermissionGroupsRef {
        AccountMember_Policies_PermissionGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccountMember_Policies_PermissionGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccountMember_Policies_ResourceGroups {
    id: terrars::PrimField<std::string::String>,
}

impl AccountMember_Policies_ResourceGroups { }

impl terrars::ToListMappable for AccountMember_Policies_ResourceGroups {
    type O = terrars::BlockAssignable<AccountMember_Policies_ResourceGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccountMember_Policies_ResourceGroups {
    #[doc = "Identifier of the group."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAccountMember_Policies_ResourceGroups {
    pub fn build(self) -> AccountMember_Policies_ResourceGroups {
        AccountMember_Policies_ResourceGroups { id: self.id }
    }
}

#[allow(dead_code)]
pub struct AccountMember_Policies_ResourceGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountMember_Policies_ResourceGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccountMember_Policies_ResourceGroupsRef {
        AccountMember_Policies_ResourceGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccountMember_Policies_ResourceGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccountMember_Policies {
    access: terrars::PrimField<std::string::String>,
    permission_groups: std::vec::Vec<AccountMember_Policies_PermissionGroups>,
    resource_groups: std::vec::Vec<AccountMember_Policies_ResourceGroups>,
}

impl AccountMember_Policies { }

impl terrars::ToListMappable for AccountMember_Policies {
    type O = terrars::BlockAssignable<AccountMember_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccountMember_Policies {
    #[doc = "Allow or deny operations against the resources.\nAvailable values: \"allow\", \"deny\"."]
    pub access: terrars::PrimField<std::string::String>,
    #[doc = "A set of permission groups that are specified to the policy."]
    pub permission_groups: std::vec::Vec<AccountMember_Policies_PermissionGroups>,
    #[doc = "A list of resource groups that the policy applies to."]
    pub resource_groups: std::vec::Vec<AccountMember_Policies_ResourceGroups>,
}

impl BuildAccountMember_Policies {
    pub fn build(self) -> AccountMember_Policies {
        AccountMember_Policies {
            access: self.access,
            permission_groups: self.permission_groups,
            resource_groups: self.resource_groups,
        }
    }
}

#[allow(dead_code)]
pub struct AccountMember_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountMember_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccountMember_PoliciesRef {
        AccountMember_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccountMember_PoliciesRef {
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
    pub fn permission_groups(&self) -> terrars::SetRef<AccountMember_Policies_PermissionGroupsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permission_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_groups` after provisioning.\nA list of resource groups that the policy applies to."]
    pub fn resource_groups(&self) -> terrars::SetRef<AccountMember_Policies_ResourceGroupsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_groups", self.base))
    }
}

#[derive(Serialize)]
pub struct AccountMember_User {}

impl AccountMember_User { }

impl terrars::ToListMappable for AccountMember_User {
    type O = terrars::BlockAssignable<AccountMember_User>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccountMember_User {}

impl BuildAccountMember_User {
    pub fn build(self) -> AccountMember_User {
        AccountMember_User {}
    }
}

#[allow(dead_code)]
pub struct AccountMember_UserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountMember_UserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccountMember_UserRef {
        AccountMember_UserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccountMember_UserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe contact email address of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\nUser's first name"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\nUser's last name"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `two_factor_authentication_enabled` after provisioning.\nIndicates whether two-factor authentication is enabled for the user account. Does not apply to API authentication."]
    pub fn two_factor_authentication_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.two_factor_authentication_enabled", self.base))
    }
}
