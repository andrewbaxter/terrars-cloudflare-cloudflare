#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataUserGroupMembersData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fuzzy_email: std::option::Option<terrars::PrimField<std::string::String>>,
    user_group_id: terrars::PrimField<std::string::String>,
}

struct DataUserGroupMembers_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataUserGroupMembersData_>,
}

#[derive(Clone)]
pub struct DataUserGroupMembers(std::rc::Rc<DataUserGroupMembers_>);

impl DataUserGroupMembers {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        "Set the field `direction`.\nThe sort order of returned user group members by email.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fuzzy_email`.\nA string used for filtering members by partial email match."]
    pub fn set_fuzzy_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fuzzy_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sort order of returned user group members by email.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fuzzy_email` after provisioning.\nA string used for filtering members by partial email match."]
    pub fn fuzzy_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fuzzy_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUser Group identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\nList of members in the user group."]
    pub fn members(&self) -> terrars::ListRef<DataUserGroupMembers_MembersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_id` after provisioning.\nUser Group identifier tag."]
    pub fn user_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataUserGroupMembers {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataUserGroupMembers { }

impl terrars::ToListMappable for DataUserGroupMembers {
    type O = terrars::ListRef<DataUserGroupMembersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataUserGroupMembers_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_user_group_members".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUserGroupMembers {
    pub tf_id: std::string::String,
    #[doc = "Account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "User Group identifier tag."]
    pub user_group_id: terrars::PrimField<std::string::String>,
}

impl BuildDataUserGroupMembers {
    pub fn build(self, stack: &mut terrars::Stack) -> DataUserGroupMembers {
        let out = DataUserGroupMembers(std::rc::Rc::new(DataUserGroupMembers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataUserGroupMembersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                direction: core::default::Default::default(),
                fuzzy_email: core::default::Default::default(),
                user_group_id: self.user_group_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataUserGroupMembersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroupMembersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUserGroupMembersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sort order of returned user group members by email.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fuzzy_email` after provisioning.\nA string used for filtering members by partial email match."]
    pub fn fuzzy_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fuzzy_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUser Group identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\nList of members in the user group."]
    pub fn members(&self) -> terrars::ListRef<DataUserGroupMembers_MembersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_id` after provisioning.\nUser Group identifier tag."]
    pub fn user_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataUserGroupMembers_Members {}

impl DataUserGroupMembers_Members { }

impl terrars::ToListMappable for DataUserGroupMembers_Members {
    type O = terrars::BlockAssignable<DataUserGroupMembers_Members>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserGroupMembers_Members {}

impl BuildDataUserGroupMembers_Members {
    pub fn build(self) -> DataUserGroupMembers_Members {
        DataUserGroupMembers_Members {}
    }
}

#[allow(dead_code)]
pub struct DataUserGroupMembers_MembersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroupMembers_MembersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataUserGroupMembers_MembersRef {
        DataUserGroupMembers_MembersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserGroupMembers_MembersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe contact email address of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAccount member identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe member's status in the account.\nAvailable values: \"accepted\", \"pending\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
