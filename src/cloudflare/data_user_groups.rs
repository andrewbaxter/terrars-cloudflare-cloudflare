#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataUserGroupsData_ {
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
    fuzzy_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataUserGroups_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataUserGroupsData_>,
}

#[derive(Clone)]
pub struct DataUserGroups(std::rc::Rc<DataUserGroups_>);

impl DataUserGroups {
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
        "Set the field `direction`.\nThe sort order of returned user groups by name (ascending or descending).\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fuzzy_name`.\nA string used for searching for user groups containing that substring."]
    pub fn set_fuzzy_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fuzzy_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nID of the user group to be fetched."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the user group to be fetched."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sort order of returned user groups by name (ascending or descending).\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fuzzy_name` after provisioning.\nA string used for searching for user groups containing that substring."]
    pub fn fuzzy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fuzzy_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the user group to be fetched."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the user group to be fetched."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataUserGroups_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataUserGroups {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataUserGroups { }

impl terrars::ToListMappable for DataUserGroups {
    type O = terrars::ListRef<DataUserGroupsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataUserGroups_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_user_groups".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUserGroups {
    pub tf_id: std::string::String,
    #[doc = "Account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataUserGroups {
    pub fn build(self, stack: &mut terrars::Stack) -> DataUserGroups {
        let out = DataUserGroups(std::rc::Rc::new(DataUserGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataUserGroupsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                direction: core::default::Default::default(),
                fuzzy_name: core::default::Default::default(),
                id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataUserGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUserGroupsRef {
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
        "Get a reference to the value of field `direction` after provisioning.\nThe sort order of returned user groups by name (ascending or descending).\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fuzzy_name` after provisioning.\nA string used for searching for user groups containing that substring."]
    pub fn fuzzy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fuzzy_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the user group to be fetched."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the user group to be fetched."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataUserGroups_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataUserGroups_Result_Policies_PermissionGroups_Meta {}

impl DataUserGroups_Result_Policies_PermissionGroups_Meta { }

impl terrars::ToListMappable for DataUserGroups_Result_Policies_PermissionGroups_Meta {
    type O = terrars::BlockAssignable<DataUserGroups_Result_Policies_PermissionGroups_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserGroups_Result_Policies_PermissionGroups_Meta {}

impl BuildDataUserGroups_Result_Policies_PermissionGroups_Meta {
    pub fn build(self) -> DataUserGroups_Result_Policies_PermissionGroups_Meta {
        DataUserGroups_Result_Policies_PermissionGroups_Meta {}
    }
}

#[allow(dead_code)]
pub struct DataUserGroups_Result_Policies_PermissionGroups_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroups_Result_Policies_PermissionGroups_MetaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataUserGroups_Result_Policies_PermissionGroups_MetaRef {
        DataUserGroups_Result_Policies_PermissionGroups_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserGroups_Result_Policies_PermissionGroups_MetaRef {
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
pub struct DataUserGroups_Result_Policies_PermissionGroups {}

impl DataUserGroups_Result_Policies_PermissionGroups { }

impl terrars::ToListMappable for DataUserGroups_Result_Policies_PermissionGroups {
    type O = terrars::BlockAssignable<DataUserGroups_Result_Policies_PermissionGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserGroups_Result_Policies_PermissionGroups {}

impl BuildDataUserGroups_Result_Policies_PermissionGroups {
    pub fn build(self) -> DataUserGroups_Result_Policies_PermissionGroups {
        DataUserGroups_Result_Policies_PermissionGroups {}
    }
}

#[allow(dead_code)]
pub struct DataUserGroups_Result_Policies_PermissionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroups_Result_Policies_PermissionGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataUserGroups_Result_Policies_PermissionGroupsRef {
        DataUserGroups_Result_Policies_PermissionGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserGroups_Result_Policies_PermissionGroupsRef {
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
    pub fn meta(&self) -> DataUserGroups_Result_Policies_PermissionGroups_MetaRef {
        DataUserGroups_Result_Policies_PermissionGroups_MetaRef::new(
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
pub struct DataUserGroups_Result_Policies_ResourceGroups_Meta {}

impl DataUserGroups_Result_Policies_ResourceGroups_Meta { }

impl terrars::ToListMappable for DataUserGroups_Result_Policies_ResourceGroups_Meta {
    type O = terrars::BlockAssignable<DataUserGroups_Result_Policies_ResourceGroups_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserGroups_Result_Policies_ResourceGroups_Meta {}

impl BuildDataUserGroups_Result_Policies_ResourceGroups_Meta {
    pub fn build(self) -> DataUserGroups_Result_Policies_ResourceGroups_Meta {
        DataUserGroups_Result_Policies_ResourceGroups_Meta {}
    }
}

#[allow(dead_code)]
pub struct DataUserGroups_Result_Policies_ResourceGroups_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroups_Result_Policies_ResourceGroups_MetaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataUserGroups_Result_Policies_ResourceGroups_MetaRef {
        DataUserGroups_Result_Policies_ResourceGroups_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserGroups_Result_Policies_ResourceGroups_MetaRef {
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
pub struct DataUserGroups_Result_Policies_ResourceGroups_Scope_Objects {}

impl DataUserGroups_Result_Policies_ResourceGroups_Scope_Objects { }

impl terrars::ToListMappable for DataUserGroups_Result_Policies_ResourceGroups_Scope_Objects {
    type O = terrars::BlockAssignable<DataUserGroups_Result_Policies_ResourceGroups_Scope_Objects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserGroups_Result_Policies_ResourceGroups_Scope_Objects {}

impl BuildDataUserGroups_Result_Policies_ResourceGroups_Scope_Objects {
    pub fn build(self) -> DataUserGroups_Result_Policies_ResourceGroups_Scope_Objects {
        DataUserGroups_Result_Policies_ResourceGroups_Scope_Objects {}
    }
}

#[allow(dead_code)]
pub struct DataUserGroups_Result_Policies_ResourceGroups_Scope_ObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroups_Result_Policies_ResourceGroups_Scope_ObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataUserGroups_Result_Policies_ResourceGroups_Scope_ObjectsRef {
        DataUserGroups_Result_Policies_ResourceGroups_Scope_ObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserGroups_Result_Policies_ResourceGroups_Scope_ObjectsRef {
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
pub struct DataUserGroups_Result_Policies_ResourceGroups_Scope {}

impl DataUserGroups_Result_Policies_ResourceGroups_Scope { }

impl terrars::ToListMappable for DataUserGroups_Result_Policies_ResourceGroups_Scope {
    type O = terrars::BlockAssignable<DataUserGroups_Result_Policies_ResourceGroups_Scope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserGroups_Result_Policies_ResourceGroups_Scope {}

impl BuildDataUserGroups_Result_Policies_ResourceGroups_Scope {
    pub fn build(self) -> DataUserGroups_Result_Policies_ResourceGroups_Scope {
        DataUserGroups_Result_Policies_ResourceGroups_Scope {}
    }
}

#[allow(dead_code)]
pub struct DataUserGroups_Result_Policies_ResourceGroups_ScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroups_Result_Policies_ResourceGroups_ScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataUserGroups_Result_Policies_ResourceGroups_ScopeRef {
        DataUserGroups_Result_Policies_ResourceGroups_ScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserGroups_Result_Policies_ResourceGroups_ScopeRef {
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
    pub fn objects(&self) -> terrars::ListRef<DataUserGroups_Result_Policies_ResourceGroups_Scope_ObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DataUserGroups_Result_Policies_ResourceGroups {}

impl DataUserGroups_Result_Policies_ResourceGroups { }

impl terrars::ToListMappable for DataUserGroups_Result_Policies_ResourceGroups {
    type O = terrars::BlockAssignable<DataUserGroups_Result_Policies_ResourceGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserGroups_Result_Policies_ResourceGroups {}

impl BuildDataUserGroups_Result_Policies_ResourceGroups {
    pub fn build(self) -> DataUserGroups_Result_Policies_ResourceGroups {
        DataUserGroups_Result_Policies_ResourceGroups {}
    }
}

#[allow(dead_code)]
pub struct DataUserGroups_Result_Policies_ResourceGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroups_Result_Policies_ResourceGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataUserGroups_Result_Policies_ResourceGroupsRef {
        DataUserGroups_Result_Policies_ResourceGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserGroups_Result_Policies_ResourceGroupsRef {
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
    pub fn meta(&self) -> DataUserGroups_Result_Policies_ResourceGroups_MetaRef {
        DataUserGroups_Result_Policies_ResourceGroups_MetaRef::new(
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
    pub fn scope(&self) -> terrars::ListRef<DataUserGroups_Result_Policies_ResourceGroups_ScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DataUserGroups_Result_Policies {}

impl DataUserGroups_Result_Policies { }

impl terrars::ToListMappable for DataUserGroups_Result_Policies {
    type O = terrars::BlockAssignable<DataUserGroups_Result_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserGroups_Result_Policies {}

impl BuildDataUserGroups_Result_Policies {
    pub fn build(self) -> DataUserGroups_Result_Policies {
        DataUserGroups_Result_Policies {}
    }
}

#[allow(dead_code)]
pub struct DataUserGroups_Result_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroups_Result_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataUserGroups_Result_PoliciesRef {
        DataUserGroups_Result_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserGroups_Result_PoliciesRef {
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
    pub fn permission_groups(&self) -> terrars::ListRef<DataUserGroups_Result_Policies_PermissionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_groups` after provisioning.\nA list of resource groups that the policy applies to."]
    pub fn resource_groups(&self) -> terrars::ListRef<DataUserGroups_Result_Policies_ResourceGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_groups", self.base))
    }
}

#[derive(Serialize)]
pub struct DataUserGroups_Result {}

impl DataUserGroups_Result { }

impl terrars::ToListMappable for DataUserGroups_Result {
    type O = terrars::BlockAssignable<DataUserGroups_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserGroups_Result {}

impl BuildDataUserGroups_Result {
    pub fn build(self) -> DataUserGroups_Result {
        DataUserGroups_Result {}
    }
}

#[allow(dead_code)]
pub struct DataUserGroups_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserGroups_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataUserGroups_ResultRef {
        DataUserGroups_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserGroups_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nTimestamp for the creation of the user group"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUser Group identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nLast time the user group was modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the user group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\nPolicies attached to the User group"]
    pub fn policies(&self) -> terrars::ListRef<DataUserGroups_Result_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.base))
    }
}
