#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataByoIpPrefixData_ {
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
    prefix_id: terrars::PrimField<std::string::String>,
}

struct DataByoIpPrefix_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataByoIpPrefixData_>,
}

#[derive(Clone)]
pub struct DataByoIpPrefix(std::rc::Rc<DataByoIpPrefix_>);

impl DataByoIpPrefix {
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

    #[doc = "Set the field `account_id`.\nIdentifier of a Cloudflare account."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `prefix_id` after provisioning.\nIdentifier of an IP Prefix."]
    pub fn prefix_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rpki_validation_state` after provisioning.\nState of one kind of validation for an IP prefix."]
    pub fn rpki_validation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpki_validation_state", self.extract_ref()))
    }
}

impl terrars::Referable for DataByoIpPrefix {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataByoIpPrefix { }

impl terrars::ToListMappable for DataByoIpPrefix {
    type O = terrars::ListRef<DataByoIpPrefixRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataByoIpPrefix_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_byo_ip_prefix".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataByoIpPrefix {
    pub tf_id: std::string::String,
    #[doc = "Identifier of an IP Prefix."]
    pub prefix_id: terrars::PrimField<std::string::String>,
}

impl BuildDataByoIpPrefix {
    pub fn build(self, stack: &mut terrars::Stack) -> DataByoIpPrefix {
        let out = DataByoIpPrefix(std::rc::Rc::new(DataByoIpPrefix_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataByoIpPrefixData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                prefix_id: self.prefix_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataByoIpPrefixRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataByoIpPrefixRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataByoIpPrefixRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `prefix_id` after provisioning.\nIdentifier of an IP Prefix."]
    pub fn prefix_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rpki_validation_state` after provisioning.\nState of one kind of validation for an IP prefix."]
    pub fn rpki_validation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpki_validation_state", self.extract_ref()))
    }
}
