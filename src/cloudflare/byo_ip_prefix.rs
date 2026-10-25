#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ByoIpPrefixData_ {
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
    asn: terrars::PrimField<f64>,
    cidr: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delegate_loa_creation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    loa_document_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ByoIpPrefix_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ByoIpPrefixData_>,
}

#[derive(Clone)]
pub struct ByoIpPrefix(std::rc::Rc<ByoIpPrefix_>);

impl ByoIpPrefix {
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

    #[doc =
        "Set the field `delegate_loa_creation`.\nWhether Cloudflare is allowed to generate the LOA document on behalf of the prefix owner."]
    pub fn set_delegate_loa_creation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delegate_loa_creation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the prefix."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `loa_document_id`.\nIdentifier for the uploaded LOA document."]
    pub fn set_loa_document_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().loa_document_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nIdentifier of a Cloudflare account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advertised` after provisioning.\nPrefix advertisement status to the Internet. This field is only not 'null' if on demand is enabled."]
    pub fn advertised(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertised", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advertised_modified_at` after provisioning.\nLast time the advertisement status was changed. This field is only not 'null' if on demand is enabled."]
    pub fn advertised_modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertised_modified_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approved` after provisioning.\nApproval state of the prefix (P = pending, V = active)."]
    pub fn approved(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approved", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `asn` after provisioning.\nAutonomous System Number (ASN) the prefix will be advertised under."]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cidr` after provisioning.\nIP Prefix in Classless Inter-Domain Routing format."]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delegate_loa_creation` after provisioning.\nWhether Cloudflare is allowed to generate the LOA document on behalf of the prefix owner."]
    pub fn delegate_loa_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delegate_loa_creation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the prefix."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of an IP Prefix."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `irr_validation_state` after provisioning.\nState of one kind of validation for an IP prefix."]
    pub fn irr_validation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.irr_validation_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `loa_document_id` after provisioning.\nIdentifier for the uploaded LOA document."]
    pub fn loa_document_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.loa_document_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_demand_enabled` after provisioning.\nWhether advertisement of the prefix to the Internet may be dynamically enabled or disabled."]
    pub fn on_demand_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_demand_locked` after provisioning.\nWhether advertisement status of the prefix is locked, meaning it cannot be changed."]
    pub fn on_demand_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_locked", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_validation_state` after provisioning.\nState of one kind of validation for an IP prefix."]
    pub fn ownership_validation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_validation_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_validation_token` after provisioning.\nToken provided to demonstrate ownership of the prefix."]
    pub fn ownership_validation_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_validation_token", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rpki_validation_state` after provisioning.\nState of one kind of validation for an IP prefix."]
    pub fn rpki_validation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpki_validation_state", self.extract_ref()))
    }
}

impl terrars::Referable for ByoIpPrefix {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ByoIpPrefix { }

impl terrars::ToListMappable for ByoIpPrefix {
    type O = terrars::ListRef<ByoIpPrefixRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ByoIpPrefix_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_byo_ip_prefix".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildByoIpPrefix {
    pub tf_id: std::string::String,
    #[doc = "Identifier of a Cloudflare account."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Autonomous System Number (ASN) the prefix will be advertised under."]
    pub asn: terrars::PrimField<f64>,
    #[doc = "IP Prefix in Classless Inter-Domain Routing format."]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildByoIpPrefix {
    pub fn build(self, stack: &mut terrars::Stack) -> ByoIpPrefix {
        let out = ByoIpPrefix(std::rc::Rc::new(ByoIpPrefix_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ByoIpPrefixData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                asn: self.asn,
                cidr: self.cidr,
                delegate_loa_creation: core::default::Default::default(),
                description: core::default::Default::default(),
                loa_document_id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ByoIpPrefixRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ByoIpPrefixRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ByoIpPrefixRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nIdentifier of a Cloudflare account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advertised` after provisioning.\nPrefix advertisement status to the Internet. This field is only not 'null' if on demand is enabled."]
    pub fn advertised(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertised", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advertised_modified_at` after provisioning.\nLast time the advertisement status was changed. This field is only not 'null' if on demand is enabled."]
    pub fn advertised_modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertised_modified_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approved` after provisioning.\nApproval state of the prefix (P = pending, V = active)."]
    pub fn approved(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approved", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `asn` after provisioning.\nAutonomous System Number (ASN) the prefix will be advertised under."]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cidr` after provisioning.\nIP Prefix in Classless Inter-Domain Routing format."]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delegate_loa_creation` after provisioning.\nWhether Cloudflare is allowed to generate the LOA document on behalf of the prefix owner."]
    pub fn delegate_loa_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delegate_loa_creation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the prefix."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of an IP Prefix."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `irr_validation_state` after provisioning.\nState of one kind of validation for an IP prefix."]
    pub fn irr_validation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.irr_validation_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `loa_document_id` after provisioning.\nIdentifier for the uploaded LOA document."]
    pub fn loa_document_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.loa_document_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_demand_enabled` after provisioning.\nWhether advertisement of the prefix to the Internet may be dynamically enabled or disabled."]
    pub fn on_demand_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_demand_locked` after provisioning.\nWhether advertisement status of the prefix is locked, meaning it cannot be changed."]
    pub fn on_demand_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_demand_locked", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_validation_state` after provisioning.\nState of one kind of validation for an IP prefix."]
    pub fn ownership_validation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_validation_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_validation_token` after provisioning.\nToken provided to demonstrate ownership of the prefix."]
    pub fn ownership_validation_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_validation_token", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rpki_validation_state` after provisioning.\nState of one kind of validation for an IP prefix."]
    pub fn rpki_validation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpki_validation_state", self.extract_ref()))
    }
}
