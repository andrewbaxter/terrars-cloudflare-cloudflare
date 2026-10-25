#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAccountApiTokenPermissionGroupsData_ {
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
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAccountApiTokenPermissionGroups_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountApiTokenPermissionGroupsData_>,
}

#[derive(Clone)]
pub struct DataAccountApiTokenPermissionGroups(std::rc::Rc<DataAccountApiTokenPermissionGroups_>);

impl DataAccountApiTokenPermissionGroups {
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

    #[doc = "Set the field `name`.\nFilter by the name of the permission group.\nThe value must be URL-encoded."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nFilter by the scope of the permission group.\nThe value must be URL-encoded."]
    pub fn set_scope(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFilter by the name of the permission group.\nThe value must be URL-encoded."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_groups` after provisioning.\n"]
    pub fn permission_groups(&self) -> terrars::ListRef<DataAccountApiTokenPermissionGroups_PermissionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_groups", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nFilter by the scope of the permission group.\nThe value must be URL-encoded."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountApiTokenPermissionGroups {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountApiTokenPermissionGroups { }

impl terrars::ToListMappable for DataAccountApiTokenPermissionGroups {
    type O = terrars::ListRef<DataAccountApiTokenPermissionGroupsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountApiTokenPermissionGroups_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_account_api_token_permission_groups".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountApiTokenPermissionGroups {
    pub tf_id: std::string::String,
}

impl BuildDataAccountApiTokenPermissionGroups {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountApiTokenPermissionGroups {
        let out = DataAccountApiTokenPermissionGroups(std::rc::Rc::new(DataAccountApiTokenPermissionGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountApiTokenPermissionGroupsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                name: core::default::Default::default(),
                scope: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountApiTokenPermissionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountApiTokenPermissionGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountApiTokenPermissionGroupsRef {
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
        "Get a reference to the value of field `name` after provisioning.\nFilter by the name of the permission group.\nThe value must be URL-encoded."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_groups` after provisioning.\n"]
    pub fn permission_groups(&self) -> terrars::ListRef<DataAccountApiTokenPermissionGroups_PermissionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_groups", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nFilter by the scope of the permission group.\nThe value must be URL-encoded."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountApiTokenPermissionGroups_PermissionGroups {}

impl DataAccountApiTokenPermissionGroups_PermissionGroups { }

impl terrars::ToListMappable for DataAccountApiTokenPermissionGroups_PermissionGroups {
    type O = terrars::BlockAssignable<DataAccountApiTokenPermissionGroups_PermissionGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountApiTokenPermissionGroups_PermissionGroups {}

impl BuildDataAccountApiTokenPermissionGroups_PermissionGroups {
    pub fn build(self) -> DataAccountApiTokenPermissionGroups_PermissionGroups {
        DataAccountApiTokenPermissionGroups_PermissionGroups {}
    }
}

#[allow(dead_code)]
pub struct DataAccountApiTokenPermissionGroups_PermissionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountApiTokenPermissionGroups_PermissionGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountApiTokenPermissionGroups_PermissionGroupsRef {
        DataAccountApiTokenPermissionGroups_PermissionGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountApiTokenPermissionGroups_PermissionGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `category` after provisioning.\nProduct category that this permission group belongs to.\nAvailable values: \"developer_platform\", \"ai_and_machine_learning\", \"dns_and_zones\", \"app_security\", \"rules_and_configuration\", \"cloudflare_one_and_zero_trust\", \"analytics_and_logs\", \"network_services\", \"media\", \"email_and_messaging\", \"cache_and_performance\", \"account_and_billing\", \"other\"."]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nPublic ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_selectable` after provisioning.\nWhether the caller can select this permission group when creating a token."]
    pub fn is_selectable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_selectable", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nPermission Group Name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nResources to which the Permission Group is scoped"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}
