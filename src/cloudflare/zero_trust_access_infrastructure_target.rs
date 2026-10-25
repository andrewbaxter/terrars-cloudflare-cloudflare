#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustAccessInfrastructureTargetData_ {
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
    hostname: terrars::PrimField<std::string::String>,
    ip: ZeroTrustAccessInfrastructureTarget_Ip,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct ZeroTrustAccessInfrastructureTarget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustAccessInfrastructureTargetData_>,
}

#[derive(Clone)]
pub struct ZeroTrustAccessInfrastructureTarget(std::rc::Rc<ZeroTrustAccessInfrastructureTarget_>);

impl ZeroTrustAccessInfrastructureTarget {
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
        "Set the field `tags`.\nOptional tags to associate with the target. Keys and values are\nuser-defined strings."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nDate and time at which the target was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nA non-unique field that refers to a target. Case insensitive, maximum\nlength of 255 characters, supports the use of special characters dash\nand period, does not support spaces, and must start and end with an\nalphanumeric character."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nTarget identifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nThe IPv4/IPv6 address that identifies where to reach a target"]
    pub fn ip(&self) -> ZeroTrustAccessInfrastructureTarget_IpRef {
        ZeroTrustAccessInfrastructureTarget_IpRef::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_at` after provisioning.\nDate and time at which the target was modified"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nOptional tags to associate with the target. Keys and values are\nuser-defined strings."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustAccessInfrastructureTarget {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustAccessInfrastructureTarget { }

impl terrars::ToListMappable for ZeroTrustAccessInfrastructureTarget {
    type O = terrars::ListRef<ZeroTrustAccessInfrastructureTargetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustAccessInfrastructureTarget_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_infrastructure_target".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustAccessInfrastructureTarget {
    pub tf_id: std::string::String,
    #[doc = "Account identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "A non-unique field that refers to a target. Case insensitive, maximum\nlength of 255 characters, supports the use of special characters dash\nand period, does not support spaces, and must start and end with an\nalphanumeric character."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "The IPv4/IPv6 address that identifies where to reach a target"]
    pub ip: ZeroTrustAccessInfrastructureTarget_Ip,
}

impl BuildZeroTrustAccessInfrastructureTarget {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustAccessInfrastructureTarget {
        let out = ZeroTrustAccessInfrastructureTarget(std::rc::Rc::new(ZeroTrustAccessInfrastructureTarget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustAccessInfrastructureTargetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                hostname: self.hostname,
                ip: self.ip,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustAccessInfrastructureTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessInfrastructureTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustAccessInfrastructureTargetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nDate and time at which the target was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nA non-unique field that refers to a target. Case insensitive, maximum\nlength of 255 characters, supports the use of special characters dash\nand period, does not support spaces, and must start and end with an\nalphanumeric character."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nTarget identifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nThe IPv4/IPv6 address that identifies where to reach a target"]
    pub fn ip(&self) -> ZeroTrustAccessInfrastructureTarget_IpRef {
        ZeroTrustAccessInfrastructureTarget_IpRef::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_at` after provisioning.\nDate and time at which the target was modified"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nOptional tags to associate with the target. Keys and values are\nuser-defined strings."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessInfrastructureTarget_Ip_Ipv4 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessInfrastructureTarget_Ip_Ipv4 {
    #[doc = "Set the field `ip_addr`.\nIP address of the target"]
    pub fn set_ip_addr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_addr = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `virtual_network_id`.\n(optional) Private virtual network identifier for the target. If omitted, the default virtual network ID will be used."]
    pub fn set_virtual_network_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.virtual_network_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessInfrastructureTarget_Ip_Ipv4 {
    type O = terrars::BlockAssignable<ZeroTrustAccessInfrastructureTarget_Ip_Ipv4>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessInfrastructureTarget_Ip_Ipv4 {}

impl BuildZeroTrustAccessInfrastructureTarget_Ip_Ipv4 {
    pub fn build(self) -> ZeroTrustAccessInfrastructureTarget_Ip_Ipv4 {
        ZeroTrustAccessInfrastructureTarget_Ip_Ipv4 {
            ip_addr: core::default::Default::default(),
            virtual_network_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessInfrastructureTarget_Ip_Ipv4Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessInfrastructureTarget_Ip_Ipv4Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessInfrastructureTarget_Ip_Ipv4Ref {
        ZeroTrustAccessInfrastructureTarget_Ip_Ipv4Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessInfrastructureTarget_Ip_Ipv4Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_addr` after provisioning.\nIP address of the target"]
    pub fn ip_addr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_addr", self.base))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\n(optional) Private virtual network identifier for the target. If omitted, the default virtual network ID will be used."]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessInfrastructureTarget_Ip_Ipv6 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustAccessInfrastructureTarget_Ip_Ipv6 {
    #[doc = "Set the field `ip_addr`.\nIP address of the target"]
    pub fn set_ip_addr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_addr = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `virtual_network_id`.\n(optional) Private virtual network identifier for the target. If omitted, the default virtual network ID will be used."]
    pub fn set_virtual_network_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.virtual_network_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessInfrastructureTarget_Ip_Ipv6 {
    type O = terrars::BlockAssignable<ZeroTrustAccessInfrastructureTarget_Ip_Ipv6>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessInfrastructureTarget_Ip_Ipv6 {}

impl BuildZeroTrustAccessInfrastructureTarget_Ip_Ipv6 {
    pub fn build(self) -> ZeroTrustAccessInfrastructureTarget_Ip_Ipv6 {
        ZeroTrustAccessInfrastructureTarget_Ip_Ipv6 {
            ip_addr: core::default::Default::default(),
            virtual_network_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessInfrastructureTarget_Ip_Ipv6Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessInfrastructureTarget_Ip_Ipv6Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustAccessInfrastructureTarget_Ip_Ipv6Ref {
        ZeroTrustAccessInfrastructureTarget_Ip_Ipv6Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessInfrastructureTarget_Ip_Ipv6Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_addr` after provisioning.\nIP address of the target"]
    pub fn ip_addr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_addr", self.base))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\n(optional) Private virtual network identifier for the target. If omitted, the default virtual network ID will be used."]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustAccessInfrastructureTarget_Ip {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4: std::option::Option<ZeroTrustAccessInfrastructureTarget_Ip_Ipv4>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6: std::option::Option<ZeroTrustAccessInfrastructureTarget_Ip_Ipv6>,
}

impl ZeroTrustAccessInfrastructureTarget_Ip {
    #[doc = "Set the field `ipv4`.\nThe target's IPv4 address"]
    pub fn set_ipv4(mut self, v: impl std::convert::Into<ZeroTrustAccessInfrastructureTarget_Ip_Ipv4>) -> Self {
        self.ipv4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6`.\nThe target's IPv6 address"]
    pub fn set_ipv6(mut self, v: impl std::convert::Into<ZeroTrustAccessInfrastructureTarget_Ip_Ipv6>) -> Self {
        self.ipv6 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustAccessInfrastructureTarget_Ip {
    type O = terrars::BlockAssignable<ZeroTrustAccessInfrastructureTarget_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustAccessInfrastructureTarget_Ip {}

impl BuildZeroTrustAccessInfrastructureTarget_Ip {
    pub fn build(self) -> ZeroTrustAccessInfrastructureTarget_Ip {
        ZeroTrustAccessInfrastructureTarget_Ip {
            ipv4: core::default::Default::default(),
            ipv6: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustAccessInfrastructureTarget_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustAccessInfrastructureTarget_IpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustAccessInfrastructureTarget_IpRef {
        ZeroTrustAccessInfrastructureTarget_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustAccessInfrastructureTarget_IpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4` after provisioning.\nThe target's IPv4 address"]
    pub fn ipv4(&self) -> ZeroTrustAccessInfrastructureTarget_Ip_Ipv4Ref {
        ZeroTrustAccessInfrastructureTarget_Ip_Ipv4Ref::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\nThe target's IPv6 address"]
    pub fn ipv6(&self) -> ZeroTrustAccessInfrastructureTarget_Ip_Ipv6Ref {
        ZeroTrustAccessInfrastructureTarget_Ip_Ipv6Ref::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }
}
