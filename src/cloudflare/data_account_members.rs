#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAccountMembersData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAccountMembers_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountMembersData_>,
}

#[derive(Clone)]
pub struct DataAccountMembers(std::rc::Rc<DataAccountMembers_>);

impl DataAccountMembers {
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

    #[doc = "Set the field `account_id`.\nAccount identifier tag."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nField to order results by.\nAvailable values: \"user.first_name\", \"user.last_name\", \"user.email\", \"status\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nA member's status in the account.\nAvailable values: \"accepted\", \"pending\", \"rejected\"."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order results by.\nAvailable values: \"user.first_name\", \"user.last_name\", \"user.email\", \"status\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAccountMembers_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nA member's status in the account.\nAvailable values: \"accepted\", \"pending\", \"rejected\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountMembers {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountMembers { }

impl terrars::ToListMappable for DataAccountMembers {
    type O = terrars::ListRef<DataAccountMembersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountMembers_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_account_members".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountMembers {
    pub tf_id: std::string::String,
}

impl BuildDataAccountMembers {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountMembers {
        let out = DataAccountMembers(std::rc::Rc::new(DataAccountMembers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountMembersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                direction: core::default::Default::default(),
                max_items: core::default::Default::default(),
                order: core::default::Default::default(),
                status: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountMembersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountMembersRef {
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
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order results by.\nAvailable values: \"user.first_name\", \"user.last_name\", \"user.email\", \"status\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAccountMembers_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nA member's status in the account.\nAvailable values: \"accepted\", \"pending\", \"rejected\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Policies_PermissionGroups_Meta {}

impl DataAccountMembers_Result_Policies_PermissionGroups_Meta { }

impl terrars::ToListMappable for DataAccountMembers_Result_Policies_PermissionGroups_Meta {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Policies_PermissionGroups_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Policies_PermissionGroups_Meta {}

impl BuildDataAccountMembers_Result_Policies_PermissionGroups_Meta {
    pub fn build(self) -> DataAccountMembers_Result_Policies_PermissionGroups_Meta {
        DataAccountMembers_Result_Policies_PermissionGroups_Meta {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Policies_PermissionGroups_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Policies_PermissionGroups_MetaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Policies_PermissionGroups_MetaRef {
        DataAccountMembers_Result_Policies_PermissionGroups_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Policies_PermissionGroups_MetaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `category` after provisioning.\nA category used to group permission groups."]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deprecated` after provisioning.\nIndicates whether the permission group is deprecated."]
    pub fn deprecated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAdditional information about the permission group."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `editable` after provisioning.\nIndicates whether the permission group can be edited."]
    pub fn editable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.editable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `eol_at` after provisioning.\nThe planned end-of-life date and time, when provided."]
    pub fn eol_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eol_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nA label identifying the permission group."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nThe scope associated with the permission group."]
    pub fn scopes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `visibility` after provisioning.\nIndicates the permission group's availability or visibility."]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Policies_PermissionGroups {}

impl DataAccountMembers_Result_Policies_PermissionGroups { }

impl terrars::ToListMappable for DataAccountMembers_Result_Policies_PermissionGroups {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Policies_PermissionGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Policies_PermissionGroups {}

impl BuildDataAccountMembers_Result_Policies_PermissionGroups {
    pub fn build(self) -> DataAccountMembers_Result_Policies_PermissionGroups {
        DataAccountMembers_Result_Policies_PermissionGroups {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Policies_PermissionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Policies_PermissionGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Policies_PermissionGroupsRef {
        DataAccountMembers_Result_Policies_PermissionGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Policies_PermissionGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the permission group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nAttributes associated to the permission group."]
    pub fn meta(&self) -> DataAccountMembers_Result_Policies_PermissionGroups_MetaRef {
        DataAccountMembers_Result_Policies_PermissionGroups_MetaRef::new(
            self.shared().clone(),
            format!("{}.meta", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the permission group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Policies_ResourceGroups_Meta {}

impl DataAccountMembers_Result_Policies_ResourceGroups_Meta { }

impl terrars::ToListMappable for DataAccountMembers_Result_Policies_ResourceGroups_Meta {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Policies_ResourceGroups_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Policies_ResourceGroups_Meta {}

impl BuildDataAccountMembers_Result_Policies_ResourceGroups_Meta {
    pub fn build(self) -> DataAccountMembers_Result_Policies_ResourceGroups_Meta {
        DataAccountMembers_Result_Policies_ResourceGroups_Meta {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Policies_ResourceGroups_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Policies_ResourceGroups_MetaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Policies_ResourceGroups_MetaRef {
        DataAccountMembers_Result_Policies_ResourceGroups_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Policies_ResourceGroups_MetaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Policies_ResourceGroups_Scope_Objects {}

impl DataAccountMembers_Result_Policies_ResourceGroups_Scope_Objects { }

impl terrars::ToListMappable for DataAccountMembers_Result_Policies_ResourceGroups_Scope_Objects {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Policies_ResourceGroups_Scope_Objects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Policies_ResourceGroups_Scope_Objects {}

impl BuildDataAccountMembers_Result_Policies_ResourceGroups_Scope_Objects {
    pub fn build(self) -> DataAccountMembers_Result_Policies_ResourceGroups_Scope_Objects {
        DataAccountMembers_Result_Policies_ResourceGroups_Scope_Objects {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Policies_ResourceGroups_Scope_ObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Policies_ResourceGroups_Scope_ObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Policies_ResourceGroups_Scope_ObjectsRef {
        DataAccountMembers_Result_Policies_ResourceGroups_Scope_ObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Policies_ResourceGroups_Scope_ObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nThis is a combination of pre-defined resource name and identifier (like Zone ID etc.)"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Policies_ResourceGroups_Scope {}

impl DataAccountMembers_Result_Policies_ResourceGroups_Scope { }

impl terrars::ToListMappable for DataAccountMembers_Result_Policies_ResourceGroups_Scope {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Policies_ResourceGroups_Scope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Policies_ResourceGroups_Scope {}

impl BuildDataAccountMembers_Result_Policies_ResourceGroups_Scope {
    pub fn build(self) -> DataAccountMembers_Result_Policies_ResourceGroups_Scope {
        DataAccountMembers_Result_Policies_ResourceGroups_Scope {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Policies_ResourceGroups_ScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Policies_ResourceGroups_ScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Policies_ResourceGroups_ScopeRef {
        DataAccountMembers_Result_Policies_ResourceGroups_ScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Policies_ResourceGroups_ScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nThis is a combination of pre-defined resource name and identifier (like Account ID etc.)"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `objects` after provisioning.\nA list of scope objects for additional context."]
    pub fn objects(&self) -> terrars::ListRef<DataAccountMembers_Result_Policies_ResourceGroups_Scope_ObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Policies_ResourceGroups {}

impl DataAccountMembers_Result_Policies_ResourceGroups { }

impl terrars::ToListMappable for DataAccountMembers_Result_Policies_ResourceGroups {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Policies_ResourceGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Policies_ResourceGroups {}

impl BuildDataAccountMembers_Result_Policies_ResourceGroups {
    pub fn build(self) -> DataAccountMembers_Result_Policies_ResourceGroups {
        DataAccountMembers_Result_Policies_ResourceGroups {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Policies_ResourceGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Policies_ResourceGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Policies_ResourceGroupsRef {
        DataAccountMembers_Result_Policies_ResourceGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Policies_ResourceGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the resource group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nAttributes associated to the resource group."]
    pub fn meta(&self) -> DataAccountMembers_Result_Policies_ResourceGroups_MetaRef {
        DataAccountMembers_Result_Policies_ResourceGroups_MetaRef::new(
            self.shared().clone(),
            format!("{}.meta", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope associated to the resource group"]
    pub fn scope(&self) -> terrars::ListRef<DataAccountMembers_Result_Policies_ResourceGroups_ScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Policies {}

impl DataAccountMembers_Result_Policies { }

impl terrars::ToListMappable for DataAccountMembers_Result_Policies {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Policies {}

impl BuildDataAccountMembers_Result_Policies {
    pub fn build(self) -> DataAccountMembers_Result_Policies {
        DataAccountMembers_Result_Policies {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountMembers_Result_PoliciesRef {
        DataAccountMembers_Result_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access` after provisioning.\nAllow or deny operations against the resources.\nAvailable values: \"allow\", \"deny\"."]
    pub fn access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nPolicy identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission_groups` after provisioning.\nA set of permission groups that are specified to the policy."]
    pub fn permission_groups(&self) -> terrars::ListRef<DataAccountMembers_Result_Policies_PermissionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_groups` after provisioning.\nA list of resource groups that the policy applies to."]
    pub fn resource_groups(&self) -> terrars::ListRef<DataAccountMembers_Result_Policies_ResourceGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_groups", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_Analytics {}

impl DataAccountMembers_Result_Roles_Permissions_Analytics { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_Analytics {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_Analytics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_Analytics {}

impl BuildDataAccountMembers_Result_Roles_Permissions_Analytics {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_Analytics {
        DataAccountMembers_Result_Roles_Permissions_Analytics {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_AnalyticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_AnalyticsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_AnalyticsRef {
        DataAccountMembers_Result_Roles_Permissions_AnalyticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_AnalyticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_Billing {}

impl DataAccountMembers_Result_Roles_Permissions_Billing { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_Billing {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_Billing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_Billing {}

impl BuildDataAccountMembers_Result_Roles_Permissions_Billing {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_Billing {
        DataAccountMembers_Result_Roles_Permissions_Billing {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_BillingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_BillingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_BillingRef {
        DataAccountMembers_Result_Roles_Permissions_BillingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_BillingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_CachePurge {}

impl DataAccountMembers_Result_Roles_Permissions_CachePurge { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_CachePurge {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_CachePurge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_CachePurge {}

impl BuildDataAccountMembers_Result_Roles_Permissions_CachePurge {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_CachePurge {
        DataAccountMembers_Result_Roles_Permissions_CachePurge {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_CachePurgeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_CachePurgeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_CachePurgeRef {
        DataAccountMembers_Result_Roles_Permissions_CachePurgeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_CachePurgeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_Dns {}

impl DataAccountMembers_Result_Roles_Permissions_Dns { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_Dns {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_Dns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_Dns {}

impl BuildDataAccountMembers_Result_Roles_Permissions_Dns {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_Dns {
        DataAccountMembers_Result_Roles_Permissions_Dns {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_DnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_DnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_DnsRef {
        DataAccountMembers_Result_Roles_Permissions_DnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_DnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_DnsRecords {}

impl DataAccountMembers_Result_Roles_Permissions_DnsRecords { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_DnsRecords {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_DnsRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_DnsRecords {}

impl BuildDataAccountMembers_Result_Roles_Permissions_DnsRecords {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_DnsRecords {
        DataAccountMembers_Result_Roles_Permissions_DnsRecords {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_DnsRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_DnsRecordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_DnsRecordsRef {
        DataAccountMembers_Result_Roles_Permissions_DnsRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_DnsRecordsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_Lb {}

impl DataAccountMembers_Result_Roles_Permissions_Lb { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_Lb {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_Lb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_Lb {}

impl BuildDataAccountMembers_Result_Roles_Permissions_Lb {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_Lb {
        DataAccountMembers_Result_Roles_Permissions_Lb {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_LbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_LbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_LbRef {
        DataAccountMembers_Result_Roles_Permissions_LbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_LbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_Logs {}

impl DataAccountMembers_Result_Roles_Permissions_Logs { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_Logs {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_Logs {}

impl BuildDataAccountMembers_Result_Roles_Permissions_Logs {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_Logs {
        DataAccountMembers_Result_Roles_Permissions_Logs {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_LogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_LogsRef {
        DataAccountMembers_Result_Roles_Permissions_LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_LogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_Organization {}

impl DataAccountMembers_Result_Roles_Permissions_Organization { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_Organization {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_Organization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_Organization {}

impl BuildDataAccountMembers_Result_Roles_Permissions_Organization {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_Organization {
        DataAccountMembers_Result_Roles_Permissions_Organization {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_OrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_OrganizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_OrganizationRef {
        DataAccountMembers_Result_Roles_Permissions_OrganizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_OrganizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_Ssl {}

impl DataAccountMembers_Result_Roles_Permissions_Ssl { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_Ssl {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_Ssl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_Ssl {}

impl BuildDataAccountMembers_Result_Roles_Permissions_Ssl {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_Ssl {
        DataAccountMembers_Result_Roles_Permissions_Ssl {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_SslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_SslRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_SslRef {
        DataAccountMembers_Result_Roles_Permissions_SslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_SslRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_Waf {}

impl DataAccountMembers_Result_Roles_Permissions_Waf { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_Waf {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_Waf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_Waf {}

impl BuildDataAccountMembers_Result_Roles_Permissions_Waf {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_Waf {
        DataAccountMembers_Result_Roles_Permissions_Waf {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_WafRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_WafRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_WafRef {
        DataAccountMembers_Result_Roles_Permissions_WafRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_WafRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_ZoneSettings {}

impl DataAccountMembers_Result_Roles_Permissions_ZoneSettings { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_ZoneSettings {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_ZoneSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_ZoneSettings {}

impl BuildDataAccountMembers_Result_Roles_Permissions_ZoneSettings {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_ZoneSettings {
        DataAccountMembers_Result_Roles_Permissions_ZoneSettings {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_ZoneSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_ZoneSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_ZoneSettingsRef {
        DataAccountMembers_Result_Roles_Permissions_ZoneSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_ZoneSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions_Zones {}

impl DataAccountMembers_Result_Roles_Permissions_Zones { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions_Zones {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions_Zones>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions_Zones {}

impl BuildDataAccountMembers_Result_Roles_Permissions_Zones {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions_Zones {
        DataAccountMembers_Result_Roles_Permissions_Zones {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_Permissions_ZonesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_Permissions_ZonesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_Permissions_ZonesRef {
        DataAccountMembers_Result_Roles_Permissions_ZonesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_Permissions_ZonesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles_Permissions {}

impl DataAccountMembers_Result_Roles_Permissions { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles_Permissions {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles_Permissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles_Permissions {}

impl BuildDataAccountMembers_Result_Roles_Permissions {
    pub fn build(self) -> DataAccountMembers_Result_Roles_Permissions {
        DataAccountMembers_Result_Roles_Permissions {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_Roles_PermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_Roles_PermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountMembers_Result_Roles_PermissionsRef {
        DataAccountMembers_Result_Roles_PermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_Roles_PermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analytics` after provisioning.\n"]
    pub fn analytics(&self) -> DataAccountMembers_Result_Roles_Permissions_AnalyticsRef {
        DataAccountMembers_Result_Roles_Permissions_AnalyticsRef::new(
            self.shared().clone(),
            format!("{}.analytics", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `billing` after provisioning.\n"]
    pub fn billing(&self) -> DataAccountMembers_Result_Roles_Permissions_BillingRef {
        DataAccountMembers_Result_Roles_Permissions_BillingRef::new(
            self.shared().clone(),
            format!("{}.billing", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cache_purge` after provisioning.\n"]
    pub fn cache_purge(&self) -> DataAccountMembers_Result_Roles_Permissions_CachePurgeRef {
        DataAccountMembers_Result_Roles_Permissions_CachePurgeRef::new(
            self.shared().clone(),
            format!("{}.cache_purge", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> DataAccountMembers_Result_Roles_Permissions_DnsRef {
        DataAccountMembers_Result_Roles_Permissions_DnsRef::new(self.shared().clone(), format!("{}.dns", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_records` after provisioning.\n"]
    pub fn dns_records(&self) -> DataAccountMembers_Result_Roles_Permissions_DnsRecordsRef {
        DataAccountMembers_Result_Roles_Permissions_DnsRecordsRef::new(
            self.shared().clone(),
            format!("{}.dns_records", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lb` after provisioning.\n"]
    pub fn lb(&self) -> DataAccountMembers_Result_Roles_Permissions_LbRef {
        DataAccountMembers_Result_Roles_Permissions_LbRef::new(self.shared().clone(), format!("{}.lb", self.base))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> DataAccountMembers_Result_Roles_Permissions_LogsRef {
        DataAccountMembers_Result_Roles_Permissions_LogsRef::new(
            self.shared().clone(),
            format!("{}.logs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `organization` after provisioning.\n"]
    pub fn organization(&self) -> DataAccountMembers_Result_Roles_Permissions_OrganizationRef {
        DataAccountMembers_Result_Roles_Permissions_OrganizationRef::new(
            self.shared().clone(),
            format!("{}.organization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> DataAccountMembers_Result_Roles_Permissions_SslRef {
        DataAccountMembers_Result_Roles_Permissions_SslRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `waf` after provisioning.\n"]
    pub fn waf(&self) -> DataAccountMembers_Result_Roles_Permissions_WafRef {
        DataAccountMembers_Result_Roles_Permissions_WafRef::new(self.shared().clone(), format!("{}.waf", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_settings` after provisioning.\n"]
    pub fn zone_settings(&self) -> DataAccountMembers_Result_Roles_Permissions_ZoneSettingsRef {
        DataAccountMembers_Result_Roles_Permissions_ZoneSettingsRef::new(
            self.shared().clone(),
            format!("{}.zone_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(&self) -> DataAccountMembers_Result_Roles_Permissions_ZonesRef {
        DataAccountMembers_Result_Roles_Permissions_ZonesRef::new(
            self.shared().clone(),
            format!("{}.zones", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_Roles {}

impl DataAccountMembers_Result_Roles { }

impl terrars::ToListMappable for DataAccountMembers_Result_Roles {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_Roles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_Roles {}

impl BuildDataAccountMembers_Result_Roles {
    pub fn build(self) -> DataAccountMembers_Result_Roles {
        DataAccountMembers_Result_Roles {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_RolesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_RolesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountMembers_Result_RolesRef {
        DataAccountMembers_Result_RolesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_RolesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of role's permissions."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nRole identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nRole name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> DataAccountMembers_Result_Roles_PermissionsRef {
        DataAccountMembers_Result_Roles_PermissionsRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataAccountMembers_Result_User {}

impl DataAccountMembers_Result_User { }

impl terrars::ToListMappable for DataAccountMembers_Result_User {
    type O = terrars::BlockAssignable<DataAccountMembers_Result_User>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result_User {}

impl BuildDataAccountMembers_Result_User {
    pub fn build(self) -> DataAccountMembers_Result_User {
        DataAccountMembers_Result_User {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_Result_UserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_Result_UserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountMembers_Result_UserRef {
        DataAccountMembers_Result_UserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_Result_UserRef {
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

#[derive(Serialize)]
pub struct DataAccountMembers_Result {}

impl DataAccountMembers_Result { }

impl terrars::ToListMappable for DataAccountMembers_Result {
    type O = terrars::BlockAssignable<DataAccountMembers_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountMembers_Result {}

impl BuildDataAccountMembers_Result {
    pub fn build(self) -> DataAccountMembers_Result {
        DataAccountMembers_Result {}
    }
}

#[allow(dead_code)]
pub struct DataAccountMembers_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountMembers_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountMembers_ResultRef {
        DataAccountMembers_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountMembers_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe contact email address of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nMembership identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\nAccess policy for the membership"]
    pub fn policies(&self) -> terrars::ListRef<DataAccountMembers_Result_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.base))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\nRoles assigned to this Member."]
    pub fn roles(&self) -> terrars::ListRef<DataAccountMembers_Result_RolesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nA member's status in the account.\nAvailable values: \"accepted\", \"pending\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nDetails of the user associated to the membership."]
    pub fn user(&self) -> DataAccountMembers_Result_UserRef {
        DataAccountMembers_Result_UserRef::new(self.shared().clone(), format!("{}.user", self.base))
    }
}
