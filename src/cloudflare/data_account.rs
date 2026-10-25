#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAccountData_ {
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
    filter: std::option::Option<DataAccount_Filter>,
}

struct DataAccount_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountData_>,
}

#[derive(Clone)]
pub struct DataAccount(std::rc::Rc<DataAccount_>);

impl DataAccount {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataAccount_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nTimestamp for the creation of the account"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataAccount_FilterRef {
        DataAccount_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAccount identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\nParent container details"]
    pub fn managed_by(&self) -> DataAccount_ManagedByRef {
        DataAccount_ManagedByRef::new(self.shared().clone(), format!("{}.managed_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nAccount name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAccount settings"]
    pub fn settings(&self) -> DataAccount_SettingsRef {
        DataAccount_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"standard\", \"enterprise\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccount {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccount { }

impl terrars::ToListMappable for DataAccount {
    type O = terrars::ListRef<DataAccountRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccount_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_account".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccount {
    pub tf_id: std::string::String,
}

impl BuildDataAccount {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccount {
        let out = DataAccount(std::rc::Rc::new(DataAccount_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountRef {
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
        "Get a reference to the value of field `created_on` after provisioning.\nTimestamp for the creation of the account"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataAccount_FilterRef {
        DataAccount_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAccount identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\nParent container details"]
    pub fn managed_by(&self) -> DataAccount_ManagedByRef {
        DataAccount_ManagedByRef::new(self.shared().clone(), format!("{}.managed_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nAccount name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAccount settings"]
    pub fn settings(&self) -> DataAccount_SettingsRef {
        DataAccount_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"standard\", \"enterprise\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccount_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccount_Filter {
    #[doc = "Set the field `direction`.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the account."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccount_Filter {
    type O = terrars::BlockAssignable<DataAccount_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccount_Filter {}

impl BuildDataAccount_Filter {
    pub fn build(self) -> DataAccount_Filter {
        DataAccount_Filter {
            direction: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccount_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccount_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccount_FilterRef {
        DataAccount_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccount_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order results.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the account."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccount_ManagedBy {}

impl DataAccount_ManagedBy { }

impl terrars::ToListMappable for DataAccount_ManagedBy {
    type O = terrars::BlockAssignable<DataAccount_ManagedBy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccount_ManagedBy {}

impl BuildDataAccount_ManagedBy {
    pub fn build(self) -> DataAccount_ManagedBy {
        DataAccount_ManagedBy {}
    }
}

#[allow(dead_code)]
pub struct DataAccount_ManagedByRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccount_ManagedByRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccount_ManagedByRef {
        DataAccount_ManagedByRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccount_ManagedByRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `parent_org_id` after provisioning.\nID of the parent Organization, if one exists"]
    pub fn parent_org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_org_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parent_org_name` after provisioning.\nName of the parent Organization, if one exists"]
    pub fn parent_org_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_org_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccount_Settings {}

impl DataAccount_Settings { }

impl terrars::ToListMappable for DataAccount_Settings {
    type O = terrars::BlockAssignable<DataAccount_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccount_Settings {}

impl BuildDataAccount_Settings {
    pub fn build(self) -> DataAccount_Settings {
        DataAccount_Settings {}
    }
}

#[allow(dead_code)]
pub struct DataAccount_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccount_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccount_SettingsRef {
        DataAccount_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccount_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `abuse_contact_email` after provisioning.\nSets an abuse contact email to notify for abuse reports."]
    pub fn abuse_contact_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.abuse_contact_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce_twofactor` after provisioning.\nIndicates whether membership in this account requires that\nTwo-Factor Authentication is enabled"]
    pub fn enforce_twofactor(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_twofactor", self.base))
    }
}
