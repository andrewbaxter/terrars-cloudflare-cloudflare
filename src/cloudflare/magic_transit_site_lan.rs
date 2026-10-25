#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct MagicTransitSiteLanData_ {
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
    bond_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ha_link: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_breakout: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_prioritized: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat: std::option::Option<MagicTransitSiteLan_Nat>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    physport: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routed_subnets: std::option::Option<std::vec::Vec<MagicTransitSiteLan_RoutedSubnets>>,
    site_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_addressing: std::option::Option<MagicTransitSiteLan_StaticAddressing>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vlan_tag: std::option::Option<terrars::PrimField<f64>>,
}

struct MagicTransitSiteLan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MagicTransitSiteLanData_>,
}

#[derive(Clone)]
pub struct MagicTransitSiteLan(std::rc::Rc<MagicTransitSiteLan_>);

impl MagicTransitSiteLan {
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

    #[doc = "Set the field `bond_id`.\n"]
    pub fn set_bond_id(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().bond_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ha_link`.\nmark true to use this LAN for HA probing. only works for site with HA turned on. only one LAN can be set as the ha_link."]
    pub fn set_ha_link(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ha_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_breakout`.\nmark true to use this LAN for source-based breakout traffic"]
    pub fn set_is_breakout(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_breakout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_prioritized`.\nmark true to use this LAN for source-based prioritized traffic"]
    pub fn set_is_prioritized(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_prioritized = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat`.\n"]
    pub fn set_nat(self, v: impl std::convert::Into<MagicTransitSiteLan_Nat>) -> Self {
        self.0.data.borrow_mut().nat = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `physport`.\n"]
    pub fn set_physport(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().physport = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routed_subnets`.\n"]
    pub fn set_routed_subnets(
        self,
        v: impl std::convert::Into<std::vec::Vec<MagicTransitSiteLan_RoutedSubnets>>,
    ) -> Self {
        self.0.data.borrow_mut().routed_subnets = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `static_addressing`.\nIf the site is not configured in high availability mode, this configuration is optional (if omitted, use DHCP). However, if in high availability mode, static_address is required along with secondary and virtual address."]
    pub fn set_static_addressing(self, v: impl std::convert::Into<MagicTransitSiteLan_StaticAddressing>) -> Self {
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

    #[doc = "Get a reference to the value of field `bond_id` after provisioning.\n"]
    pub fn bond_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bond_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ha_link` after provisioning.\nmark true to use this LAN for HA probing. only works for site with HA turned on. only one LAN can be set as the ha_link."]
    pub fn ha_link(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ha_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_breakout` after provisioning.\nmark true to use this LAN for source-based breakout traffic"]
    pub fn is_breakout(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_breakout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_prioritized` after provisioning.\nmark true to use this LAN for source-based prioritized traffic"]
    pub fn is_prioritized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_prioritized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nat` after provisioning.\n"]
    pub fn nat(&self) -> MagicTransitSiteLan_NatRef {
        MagicTransitSiteLan_NatRef::new(self.shared().clone(), format!("{}.nat", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `physport` after provisioning.\n"]
    pub fn physport(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physport", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routed_subnets` after provisioning.\n"]
    pub fn routed_subnets(&self) -> terrars::ListRef<MagicTransitSiteLan_RoutedSubnetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routed_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `static_addressing` after provisioning.\nIf the site is not configured in high availability mode, this configuration is optional (if omitted, use DHCP). However, if in high availability mode, static_address is required along with secondary and virtual address."]
    pub fn static_addressing(&self) -> MagicTransitSiteLan_StaticAddressingRef {
        MagicTransitSiteLan_StaticAddressingRef::new(
            self.shared().clone(),
            format!("{}.static_addressing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vlan_tag` after provisioning.\nVLAN ID. Use zero for untagged."]
    pub fn vlan_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag", self.extract_ref()))
    }
}

impl terrars::Referable for MagicTransitSiteLan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MagicTransitSiteLan { }

impl terrars::ToListMappable for MagicTransitSiteLan {
    type O = terrars::ListRef<MagicTransitSiteLanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MagicTransitSiteLan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_site_lan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMagicTransitSiteLan {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Identifier"]
    pub site_id: terrars::PrimField<std::string::String>,
}

impl BuildMagicTransitSiteLan {
    pub fn build(self, stack: &mut terrars::Stack) -> MagicTransitSiteLan {
        let out = MagicTransitSiteLan(std::rc::Rc::new(MagicTransitSiteLan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MagicTransitSiteLanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                bond_id: core::default::Default::default(),
                ha_link: core::default::Default::default(),
                is_breakout: core::default::Default::default(),
                is_prioritized: core::default::Default::default(),
                name: core::default::Default::default(),
                nat: core::default::Default::default(),
                physport: core::default::Default::default(),
                routed_subnets: core::default::Default::default(),
                site_id: self.site_id,
                static_addressing: core::default::Default::default(),
                vlan_tag: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MagicTransitSiteLanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteLanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MagicTransitSiteLanRef {
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

    #[doc = "Get a reference to the value of field `bond_id` after provisioning.\n"]
    pub fn bond_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bond_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ha_link` after provisioning.\nmark true to use this LAN for HA probing. only works for site with HA turned on. only one LAN can be set as the ha_link."]
    pub fn ha_link(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ha_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_breakout` after provisioning.\nmark true to use this LAN for source-based breakout traffic"]
    pub fn is_breakout(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_breakout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_prioritized` after provisioning.\nmark true to use this LAN for source-based prioritized traffic"]
    pub fn is_prioritized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_prioritized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nat` after provisioning.\n"]
    pub fn nat(&self) -> MagicTransitSiteLan_NatRef {
        MagicTransitSiteLan_NatRef::new(self.shared().clone(), format!("{}.nat", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `physport` after provisioning.\n"]
    pub fn physport(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physport", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routed_subnets` after provisioning.\n"]
    pub fn routed_subnets(&self) -> terrars::ListRef<MagicTransitSiteLan_RoutedSubnetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routed_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `static_addressing` after provisioning.\nIf the site is not configured in high availability mode, this configuration is optional (if omitted, use DHCP). However, if in high availability mode, static_address is required along with secondary and virtual address."]
    pub fn static_addressing(&self) -> MagicTransitSiteLan_StaticAddressingRef {
        MagicTransitSiteLan_StaticAddressingRef::new(
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
pub struct MagicTransitSiteLan_Nat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MagicTransitSiteLan_Nat {
    #[doc = "Set the field `static_prefix`.\nA valid CIDR notation representing an IP range."]
    pub fn set_static_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.static_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitSiteLan_Nat {
    type O = terrars::BlockAssignable<MagicTransitSiteLan_Nat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitSiteLan_Nat {}

impl BuildMagicTransitSiteLan_Nat {
    pub fn build(self) -> MagicTransitSiteLan_Nat {
        MagicTransitSiteLan_Nat { static_prefix: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MagicTransitSiteLan_NatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteLan_NatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicTransitSiteLan_NatRef {
        MagicTransitSiteLan_NatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitSiteLan_NatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `static_prefix` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn static_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicTransitSiteLan_RoutedSubnets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat: std::option::Option<MagicTransitSiteLan_Nat>,
    next_hop: terrars::PrimField<std::string::String>,
    prefix: terrars::PrimField<std::string::String>,
}

impl MagicTransitSiteLan_RoutedSubnets {
    #[doc = "Set the field `nat`.\n"]
    pub fn set_nat(mut self, v: impl std::convert::Into<MagicTransitSiteLan_Nat>) -> Self {
        self.nat = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitSiteLan_RoutedSubnets {
    type O = terrars::BlockAssignable<MagicTransitSiteLan_RoutedSubnets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitSiteLan_RoutedSubnets {
    #[doc = "A valid IPv4 address."]
    pub next_hop: terrars::PrimField<std::string::String>,
    #[doc = "A valid CIDR notation representing an IP range."]
    pub prefix: terrars::PrimField<std::string::String>,
}

impl BuildMagicTransitSiteLan_RoutedSubnets {
    pub fn build(self) -> MagicTransitSiteLan_RoutedSubnets {
        MagicTransitSiteLan_RoutedSubnets {
            nat: core::default::Default::default(),
            next_hop: self.next_hop,
            prefix: self.prefix,
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitSiteLan_RoutedSubnetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteLan_RoutedSubnetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicTransitSiteLan_RoutedSubnetsRef {
        MagicTransitSiteLan_RoutedSubnetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitSiteLan_RoutedSubnetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nat` after provisioning.\n"]
    pub fn nat(&self) -> MagicTransitSiteLan_NatRef {
        MagicTransitSiteLan_NatRef::new(self.shared().clone(), format!("{}.nat", self.base))
    }

    #[doc = "Get a reference to the value of field `next_hop` after provisioning.\nA valid IPv4 address."]
    pub fn next_hop(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_hop", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicTransitSiteLan_StaticAddressing_DhcpRelay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl MagicTransitSiteLan_StaticAddressing_DhcpRelay {
    #[doc = "Set the field `server_addresses`.\nList of DHCP server IPs."]
    pub fn set_server_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.server_addresses = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitSiteLan_StaticAddressing_DhcpRelay {
    type O = terrars::BlockAssignable<MagicTransitSiteLan_StaticAddressing_DhcpRelay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitSiteLan_StaticAddressing_DhcpRelay {}

impl BuildMagicTransitSiteLan_StaticAddressing_DhcpRelay {
    pub fn build(self) -> MagicTransitSiteLan_StaticAddressing_DhcpRelay {
        MagicTransitSiteLan_StaticAddressing_DhcpRelay { server_addresses: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MagicTransitSiteLan_StaticAddressing_DhcpRelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteLan_StaticAddressing_DhcpRelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MagicTransitSiteLan_StaticAddressing_DhcpRelayRef {
        MagicTransitSiteLan_StaticAddressing_DhcpRelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitSiteLan_StaticAddressing_DhcpRelayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `server_addresses` after provisioning.\nList of DHCP server IPs."]
    pub fn server_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptions {
    code: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptions { }

impl terrars::ToListMappable for MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptions {
    type O = terrars::BlockAssignable<MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptions {
    #[doc =
        "DHCP option number (1-254). Options 0 and 255 are reserved by RFC 2132. Options 3, 6, and 51 are not allowed because they conflict with connector-managed configuration."]
    pub code: terrars::PrimField<f64>,
    #[doc =
        "The type of the option value. text: a string (max 255 bytes). hex: colon-separated hex bytes (e.g. \"01:04:aa:bb:cc\", max 255 bytes). ip: an IPv4 address (e.g. \"10.20.30.40\"). byte: an unsigned integer 0-255 (1 byte). short: an unsigned integer 0-65535 (2 bytes). integer: an unsigned integer 0-4294967295 (4 bytes).\nAvailable values: \"text\", \"hex\", \"ip\", \"byte\", \"short\", \"integer\"."]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = "The option value, interpreted according to the type field."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildMagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptions {
    pub fn build(self) -> MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptions {
        MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptions {
            code: self.code,
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptionsRef {
        MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nDHCP option number (1-254). Options 0 and 255 are reserved by RFC 2132. Options 3, 6, and 51 are not allowed because they conflict with connector-managed configuration."]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the option value. text: a string (max 255 bytes). hex: colon-separated hex bytes (e.g. \"01:04:aa:bb:cc\", max 255 bytes). ip: an IPv4 address (e.g. \"10.20.30.40\"). byte: an unsigned integer 0-255 (1 byte). short: an unsigned integer 0-65535 (2 bytes). integer: an unsigned integer 0-4294967295 (4 bytes).\nAvailable values: \"text\", \"hex\", \"ip\", \"byte\", \"short\", \"integer\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe option value, interpreted according to the type field."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicTransitSiteLan_StaticAddressing_DhcpServer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dhcp_options: std::option::Option<std::vec::Vec<MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dhcp_pool_end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dhcp_pool_start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_server: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl MagicTransitSiteLan_StaticAddressing_DhcpServer {
    #[doc =
        "Set the field `dhcp_options`.\nOptional list of custom DHCP options to include in DHCP responses. Only valid when DHCP server is enabled."]
    pub fn set_dhcp_options(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptions>>,
    ) -> Self {
        self.dhcp_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dhcp_pool_end`.\nA valid IPv4 address."]
    pub fn set_dhcp_pool_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dhcp_pool_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dhcp_pool_start`.\nA valid IPv4 address."]
    pub fn set_dhcp_pool_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dhcp_pool_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_server`.\nA valid IPv4 address."]
    pub fn set_dns_server(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_server = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_servers`.\n"]
    pub fn set_dns_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reservations`.\nMapping of MAC addresses to IP addresses"]
    pub fn set_reservations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.reservations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitSiteLan_StaticAddressing_DhcpServer {
    type O = terrars::BlockAssignable<MagicTransitSiteLan_StaticAddressing_DhcpServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitSiteLan_StaticAddressing_DhcpServer {}

impl BuildMagicTransitSiteLan_StaticAddressing_DhcpServer {
    pub fn build(self) -> MagicTransitSiteLan_StaticAddressing_DhcpServer {
        MagicTransitSiteLan_StaticAddressing_DhcpServer {
            dhcp_options: core::default::Default::default(),
            dhcp_pool_end: core::default::Default::default(),
            dhcp_pool_start: core::default::Default::default(),
            dns_server: core::default::Default::default(),
            dns_servers: core::default::Default::default(),
            reservations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitSiteLan_StaticAddressing_DhcpServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteLan_StaticAddressing_DhcpServerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MagicTransitSiteLan_StaticAddressing_DhcpServerRef {
        MagicTransitSiteLan_StaticAddressing_DhcpServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitSiteLan_StaticAddressing_DhcpServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dhcp_options` after provisioning.\nOptional list of custom DHCP options to include in DHCP responses. Only valid when DHCP server is enabled."]
    pub fn dhcp_options(&self) -> terrars::ListRef<MagicTransitSiteLan_StaticAddressing_DhcpServer_DhcpOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dhcp_options", self.base))
    }

    #[doc = "Get a reference to the value of field `dhcp_pool_end` after provisioning.\nA valid IPv4 address."]
    pub fn dhcp_pool_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dhcp_pool_end", self.base))
    }

    #[doc = "Get a reference to the value of field `dhcp_pool_start` after provisioning.\nA valid IPv4 address."]
    pub fn dhcp_pool_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dhcp_pool_start", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_server` after provisioning.\nA valid IPv4 address."]
    pub fn dns_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_server", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reservations` after provisioning.\nMapping of MAC addresses to IP addresses"]
    pub fn reservations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.reservations", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicTransitSiteLan_StaticAddressing {
    address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dhcp_relay: std::option::Option<MagicTransitSiteLan_StaticAddressing_DhcpRelay>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dhcp_server: std::option::Option<MagicTransitSiteLan_StaticAddressing_DhcpServer>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MagicTransitSiteLan_StaticAddressing {
    #[doc = "Set the field `dhcp_relay`.\n"]
    pub fn set_dhcp_relay(mut self, v: impl std::convert::Into<MagicTransitSiteLan_StaticAddressing_DhcpRelay>) -> Self {
        self.dhcp_relay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dhcp_server`.\n"]
    pub fn set_dhcp_server(
        mut self,
        v: impl std::convert::Into<MagicTransitSiteLan_StaticAddressing_DhcpServer>,
    ) -> Self {
        self.dhcp_server = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_address`.\nA valid CIDR notation representing an IP range."]
    pub fn set_secondary_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secondary_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_address`.\nA valid CIDR notation representing an IP range."]
    pub fn set_virtual_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitSiteLan_StaticAddressing {
    type O = terrars::BlockAssignable<MagicTransitSiteLan_StaticAddressing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitSiteLan_StaticAddressing {
    #[doc = "A valid CIDR notation representing an IP range."]
    pub address: terrars::PrimField<std::string::String>,
}

impl BuildMagicTransitSiteLan_StaticAddressing {
    pub fn build(self) -> MagicTransitSiteLan_StaticAddressing {
        MagicTransitSiteLan_StaticAddressing {
            address: self.address,
            dhcp_relay: core::default::Default::default(),
            dhcp_server: core::default::Default::default(),
            secondary_address: core::default::Default::default(),
            virtual_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitSiteLan_StaticAddressingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteLan_StaticAddressingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicTransitSiteLan_StaticAddressingRef {
        MagicTransitSiteLan_StaticAddressingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitSiteLan_StaticAddressingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `dhcp_relay` after provisioning.\n"]
    pub fn dhcp_relay(&self) -> MagicTransitSiteLan_StaticAddressing_DhcpRelayRef {
        MagicTransitSiteLan_StaticAddressing_DhcpRelayRef::new(
            self.shared().clone(),
            format!("{}.dhcp_relay", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dhcp_server` after provisioning.\n"]
    pub fn dhcp_server(&self) -> MagicTransitSiteLan_StaticAddressing_DhcpServerRef {
        MagicTransitSiteLan_StaticAddressing_DhcpServerRef::new(
            self.shared().clone(),
            format!("{}.dhcp_server", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `secondary_address` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn secondary_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `virtual_address` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn virtual_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_address", self.base))
    }
}
