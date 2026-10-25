#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct MagicTransitSiteWanData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_rate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balance_inner_flows: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    physport: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    site_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_addressing: std::option::Option<MagicTransitSiteWan_StaticAddressing>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vlan_tag: std::option::Option<terrars::PrimField<f64>>,
}

struct MagicTransitSiteWan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MagicTransitSiteWanData_>,
}

#[derive(Clone)]
pub struct MagicTransitSiteWan(std::rc::Rc<MagicTransitSiteWan_>);

impl MagicTransitSiteWan {
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
        "Set the field `health_check_rate`.\nMagic WAN health check rate for tunnels created on this link. The default value is `mid`.\nAvailable values: \"low\", \"mid\", \"high\"."]
    pub fn set_health_check_rate(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().health_check_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balance_inner_flows`.\n"]
    pub fn set_load_balance_inner_flows(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().load_balance_inner_flows = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `static_addressing`.\n(optional) if omitted, use DHCP. Submit secondary_address when site is in high availability mode."]
    pub fn set_static_addressing(self, v: impl std::convert::Into<MagicTransitSiteWan_StaticAddressing>) -> Self {
        self.0.data.borrow_mut().static_addressing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vlan_tag`.\nVLAN ID. Use zero for untagged."]
    pub fn set_vlan_tag(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().vlan_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_check_rate` after provisioning.\nMagic WAN health check rate for tunnels created on this link. The default value is `mid`.\nAvailable values: \"low\", \"mid\", \"high\"."]
    pub fn health_check_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_rate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balance_inner_flows` after provisioning.\n"]
    pub fn load_balance_inner_flows(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balance_inner_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `physport` after provisioning.\n"]
    pub fn physport(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physport", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `static_addressing` after provisioning.\n(optional) if omitted, use DHCP. Submit secondary_address when site is in high availability mode."]
    pub fn static_addressing(&self) -> MagicTransitSiteWan_StaticAddressingRef {
        MagicTransitSiteWan_StaticAddressingRef::new(
            self.shared().clone(),
            format!("{}.static_addressing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vlan_tag` after provisioning.\nVLAN ID. Use zero for untagged."]
    pub fn vlan_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag", self.extract_ref()))
    }
}

impl terrars::Referable for MagicTransitSiteWan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MagicTransitSiteWan { }

impl terrars::ToListMappable for MagicTransitSiteWan {
    type O = terrars::ListRef<MagicTransitSiteWanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MagicTransitSiteWan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_site_wan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMagicTransitSiteWan {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub physport: terrars::PrimField<f64>,
    #[doc = "Identifier"]
    pub site_id: terrars::PrimField<std::string::String>,
}

impl BuildMagicTransitSiteWan {
    pub fn build(self, stack: &mut terrars::Stack) -> MagicTransitSiteWan {
        let out = MagicTransitSiteWan(std::rc::Rc::new(MagicTransitSiteWan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MagicTransitSiteWanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                health_check_rate: core::default::Default::default(),
                load_balance_inner_flows: core::default::Default::default(),
                name: core::default::Default::default(),
                physport: self.physport,
                priority: core::default::Default::default(),
                site_id: self.site_id,
                static_addressing: core::default::Default::default(),
                vlan_tag: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MagicTransitSiteWanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteWanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MagicTransitSiteWanRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_check_rate` after provisioning.\nMagic WAN health check rate for tunnels created on this link. The default value is `mid`.\nAvailable values: \"low\", \"mid\", \"high\"."]
    pub fn health_check_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_rate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balance_inner_flows` after provisioning.\n"]
    pub fn load_balance_inner_flows(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balance_inner_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `physport` after provisioning.\n"]
    pub fn physport(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physport", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `static_addressing` after provisioning.\n(optional) if omitted, use DHCP. Submit secondary_address when site is in high availability mode."]
    pub fn static_addressing(&self) -> MagicTransitSiteWan_StaticAddressingRef {
        MagicTransitSiteWan_StaticAddressingRef::new(
            self.shared().clone(),
            format!("{}.static_addressing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vlan_tag` after provisioning.\nVLAN ID. Use zero for untagged."]
    pub fn vlan_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MagicTransitSiteWan_StaticAddressing {
    address: terrars::PrimField<std::string::String>,
    gateway_address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MagicTransitSiteWan_StaticAddressing {
    #[doc = "Set the field `secondary_address`.\nA valid CIDR notation representing an IP range."]
    pub fn set_secondary_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secondary_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitSiteWan_StaticAddressing {
    type O = terrars::BlockAssignable<MagicTransitSiteWan_StaticAddressing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitSiteWan_StaticAddressing {
    #[doc = "A valid CIDR notation representing an IP range."]
    pub address: terrars::PrimField<std::string::String>,
    #[doc = "A valid IPv4 address."]
    pub gateway_address: terrars::PrimField<std::string::String>,
}

impl BuildMagicTransitSiteWan_StaticAddressing {
    pub fn build(self) -> MagicTransitSiteWan_StaticAddressing {
        MagicTransitSiteWan_StaticAddressing {
            address: self.address,
            gateway_address: self.gateway_address,
            secondary_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitSiteWan_StaticAddressingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteWan_StaticAddressingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicTransitSiteWan_StaticAddressingRef {
        MagicTransitSiteWan_StaticAddressingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitSiteWan_StaticAddressingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_address` after provisioning.\nA valid IPv4 address."]
    pub fn gateway_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secondary_address` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn secondary_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_address", self.base))
    }
}
