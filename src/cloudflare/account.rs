#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccountData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_by: std::option::Option<Account_ManagedBy>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<Account_Settings>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standalone: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<Account_Unit>,
}

struct Account_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccountData_>,
}

#[derive(Clone)]
pub struct Account(std::rc::Rc<Account_>);

impl Account {
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

    #[doc = "Set the field `managed_by`.\nParent container details"]
    pub fn set_managed_by(self, v: impl std::convert::Into<Account_ManagedBy>) -> Self {
        self.0.data.borrow_mut().managed_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\nAccount settings"]
    pub fn set_settings(self, v: impl std::convert::Into<Account_Settings>) -> Self {
        self.0.data.borrow_mut().settings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `standalone`.\nSet to `true` and omit `unit` to create a standalone Free Account. If provided, this field must be `true`."]
    pub fn set_standalone(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().standalone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nAvailable values: \"standard\", \"enterprise\"."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unit`.\nInformation related to the tenant unit. Provide its ID and omit `standalone` to create the Account within an Organization. See https://developers.cloudflare.com/tenant/how-to/manage-accounts/."]
    pub fn set_unit(self, v: impl std::convert::Into<Account_Unit>) -> Self {
        self.0.data.borrow_mut().unit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nTimestamp for the creation of the account"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\nParent container details"]
    pub fn managed_by(&self) -> Account_ManagedByRef {
        Account_ManagedByRef::new(self.shared().clone(), format!("{}.managed_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nAccount name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAccount settings"]
    pub fn settings(&self) -> Account_SettingsRef {
        Account_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standalone` after provisioning.\nSet to `true` and omit `unit` to create a standalone Free Account. If provided, this field must be `true`."]
    pub fn standalone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standalone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"standard\", \"enterprise\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit` after provisioning.\nInformation related to the tenant unit. Provide its ID and omit `standalone` to create the Account within an Organization. See https://developers.cloudflare.com/tenant/how-to/manage-accounts/."]
    pub fn unit(&self) -> Account_UnitRef {
        Account_UnitRef::new(self.shared().clone(), format!("{}.unit", self.extract_ref()))
    }
}

impl terrars::Referable for Account {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Account { }

impl terrars::ToListMappable for Account {
    type O = terrars::ListRef<AccountRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Account_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_account".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccount {
    pub tf_id: std::string::String,
    #[doc = "Account name"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAccount {
    pub fn build(self, stack: &mut terrars::Stack) -> Account {
        let out = Account(std::rc::Rc::new(Account_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccountData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                managed_by: core::default::Default::default(),
                name: self.name,
                settings: core::default::Default::default(),
                standalone: core::default::Default::default(),
                type_: core::default::Default::default(),
                unit: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccountRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nTimestamp for the creation of the account"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\nParent container details"]
    pub fn managed_by(&self) -> Account_ManagedByRef {
        Account_ManagedByRef::new(self.shared().clone(), format!("{}.managed_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nAccount name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\nAccount settings"]
    pub fn settings(&self) -> Account_SettingsRef {
        Account_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standalone` after provisioning.\nSet to `true` and omit `unit` to create a standalone Free Account. If provided, this field must be `true`."]
    pub fn standalone(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standalone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"standard\", \"enterprise\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit` after provisioning.\nInformation related to the tenant unit. Provide its ID and omit `standalone` to create the Account within an Organization. See https://developers.cloudflare.com/tenant/how-to/manage-accounts/."]
    pub fn unit(&self) -> Account_UnitRef {
        Account_UnitRef::new(self.shared().clone(), format!("{}.unit", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Account_ManagedBy {}

impl Account_ManagedBy { }

impl terrars::ToListMappable for Account_ManagedBy {
    type O = terrars::BlockAssignable<Account_ManagedBy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_ManagedBy {}

impl BuildAccount_ManagedBy {
    pub fn build(self) -> Account_ManagedBy {
        Account_ManagedBy {}
    }
}

#[allow(dead_code)]
pub struct Account_ManagedByRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_ManagedByRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Account_ManagedByRef {
        Account_ManagedByRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_ManagedByRef {
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
pub struct Account_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abuse_contact_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_twofactor: std::option::Option<terrars::PrimField<bool>>,
}

impl Account_Settings {
    #[doc = "Set the field `abuse_contact_email`.\nSets an abuse contact email to notify for abuse reports."]
    pub fn set_abuse_contact_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.abuse_contact_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforce_twofactor`.\nIndicates whether membership in this account requires that\nTwo-Factor Authentication is enabled"]
    pub fn set_enforce_twofactor(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforce_twofactor = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Account_Settings {
    type O = terrars::BlockAssignable<Account_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_Settings {}

impl BuildAccount_Settings {
    pub fn build(self) -> Account_Settings {
        Account_Settings {
            abuse_contact_email: core::default::Default::default(),
            enforce_twofactor: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Account_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Account_SettingsRef {
        Account_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_SettingsRef {
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

#[derive(Serialize)]
pub struct Account_Unit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Account_Unit {
    #[doc = "Set the field `id`.\nTenant unit ID"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Account_Unit {
    type O = terrars::BlockAssignable<Account_Unit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_Unit {}

impl BuildAccount_Unit {
    pub fn build(self) -> Account_Unit {
        Account_Unit { id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Account_UnitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_UnitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Account_UnitRef {
        Account_UnitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_UnitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nTenant unit ID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
