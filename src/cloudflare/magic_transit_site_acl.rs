#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct MagicTransitSiteAclData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_locally: std::option::Option<terrars::PrimField<bool>>,
    lan_1: MagicTransitSiteAcl_Lan1,
    lan_2: MagicTransitSiteAcl_Lan2,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    site_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unidirectional: std::option::Option<terrars::PrimField<bool>>,
}

struct MagicTransitSiteAcl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MagicTransitSiteAclData_>,
}

#[derive(Clone)]
pub struct MagicTransitSiteAcl(std::rc::Rc<MagicTransitSiteAcl_>);

impl MagicTransitSiteAcl {
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

    #[doc = "Set the field `description`.\nDescription for the ACL."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `forward_locally`.\nThe desired forwarding action for this ACL policy. If set to \"false\", the policy will forward traffic to Cloudflare. If set to \"true\", the policy will forward traffic locally on the Magic Connector. If not included in request, will default to false."]
    pub fn set_forward_locally(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().forward_locally = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocols`.\n"]
    pub fn set_protocols(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unidirectional`.\nThe desired traffic direction for this ACL policy. If set to \"false\", the policy will allow bidirectional traffic. If set to \"true\", the policy will only allow traffic in one direction. If not included in request, will default to false."]
    pub fn set_unidirectional(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().unidirectional = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription for the ACL."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `forward_locally` after provisioning.\nThe desired forwarding action for this ACL policy. If set to \"false\", the policy will forward traffic to Cloudflare. If set to \"true\", the policy will forward traffic locally on the Magic Connector. If not included in request, will default to false."]
    pub fn forward_locally(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forward_locally", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lan_1` after provisioning.\n"]
    pub fn lan_1(&self) -> MagicTransitSiteAcl_Lan1Ref {
        MagicTransitSiteAcl_Lan1Ref::new(self.shared().clone(), format!("{}.lan_1", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lan_2` after provisioning.\n"]
    pub fn lan_2(&self) -> MagicTransitSiteAcl_Lan2Ref {
        MagicTransitSiteAcl_Lan2Ref::new(self.shared().clone(), format!("{}.lan_2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the ACL."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unidirectional` after provisioning.\nThe desired traffic direction for this ACL policy. If set to \"false\", the policy will allow bidirectional traffic. If set to \"true\", the policy will only allow traffic in one direction. If not included in request, will default to false."]
    pub fn unidirectional(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unidirectional", self.extract_ref()))
    }
}

impl terrars::Referable for MagicTransitSiteAcl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MagicTransitSiteAcl { }

impl terrars::ToListMappable for MagicTransitSiteAcl {
    type O = terrars::ListRef<MagicTransitSiteAclRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MagicTransitSiteAcl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_site_acl".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMagicTransitSiteAcl {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lan_1: MagicTransitSiteAcl_Lan1,
    #[doc = ""]
    pub lan_2: MagicTransitSiteAcl_Lan2,
    #[doc = "The name of the ACL."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Identifier"]
    pub site_id: terrars::PrimField<std::string::String>,
}

impl BuildMagicTransitSiteAcl {
    pub fn build(self, stack: &mut terrars::Stack) -> MagicTransitSiteAcl {
        let out = MagicTransitSiteAcl(std::rc::Rc::new(MagicTransitSiteAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MagicTransitSiteAclData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                description: core::default::Default::default(),
                forward_locally: core::default::Default::default(),
                lan_1: self.lan_1,
                lan_2: self.lan_2,
                name: self.name,
                protocols: core::default::Default::default(),
                site_id: self.site_id,
                unidirectional: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MagicTransitSiteAclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteAclRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MagicTransitSiteAclRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription for the ACL."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `forward_locally` after provisioning.\nThe desired forwarding action for this ACL policy. If set to \"false\", the policy will forward traffic to Cloudflare. If set to \"true\", the policy will forward traffic locally on the Magic Connector. If not included in request, will default to false."]
    pub fn forward_locally(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forward_locally", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lan_1` after provisioning.\n"]
    pub fn lan_1(&self) -> MagicTransitSiteAcl_Lan1Ref {
        MagicTransitSiteAcl_Lan1Ref::new(self.shared().clone(), format!("{}.lan_1", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lan_2` after provisioning.\n"]
    pub fn lan_2(&self) -> MagicTransitSiteAcl_Lan2Ref {
        MagicTransitSiteAcl_Lan2Ref::new(self.shared().clone(), format!("{}.lan_2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the ACL."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unidirectional` after provisioning.\nThe desired traffic direction for this ACL policy. If set to \"false\", the policy will allow bidirectional traffic. If set to \"true\", the policy will only allow traffic in one direction. If not included in request, will default to false."]
    pub fn unidirectional(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unidirectional", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MagicTransitSiteAcl_Lan1 {
    lan_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lan_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl MagicTransitSiteAcl_Lan1 {
    #[doc = "Set the field `lan_name`.\nThe name of the LAN based on the provided lan_id."]
    pub fn set_lan_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lan_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_ranges`.\nArray of port ranges on the provided LAN that will be included in the ACL. If no ports or port rangess are provided, communication on any port on this LAN is allowed."]
    pub fn set_port_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.port_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ports`.\nArray of ports on the provided LAN that will be included in the ACL. If no ports or port ranges are provided, communication on any port on this LAN is allowed."]
    pub fn set_ports(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnets`.\nArray of subnet IPs within the LAN that will be included in the ACL. If no subnets are provided, communication on any subnets on this LAN are allowed."]
    pub fn set_subnets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitSiteAcl_Lan1 {
    type O = terrars::BlockAssignable<MagicTransitSiteAcl_Lan1>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitSiteAcl_Lan1 {
    #[doc = "The identifier for the LAN you want to create an ACL policy with."]
    pub lan_id: terrars::PrimField<std::string::String>,
}

impl BuildMagicTransitSiteAcl_Lan1 {
    pub fn build(self) -> MagicTransitSiteAcl_Lan1 {
        MagicTransitSiteAcl_Lan1 {
            lan_id: self.lan_id,
            lan_name: core::default::Default::default(),
            port_ranges: core::default::Default::default(),
            ports: core::default::Default::default(),
            subnets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitSiteAcl_Lan1Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteAcl_Lan1Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicTransitSiteAcl_Lan1Ref {
        MagicTransitSiteAcl_Lan1Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitSiteAcl_Lan1Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `lan_id` after provisioning.\nThe identifier for the LAN you want to create an ACL policy with."]
    pub fn lan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lan_name` after provisioning.\nThe name of the LAN based on the provided lan_id."]
    pub fn lan_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_ranges` after provisioning.\nArray of port ranges on the provided LAN that will be included in the ACL. If no ports or port rangess are provided, communication on any port on this LAN is allowed."]
    pub fn port_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ports` after provisioning.\nArray of ports on the provided LAN that will be included in the ACL. If no ports or port ranges are provided, communication on any port on this LAN is allowed."]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnets` after provisioning.\nArray of subnet IPs within the LAN that will be included in the ACL. If no subnets are provided, communication on any subnets on this LAN are allowed."]
    pub fn subnets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct MagicTransitSiteAcl_Lan2 {
    lan_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lan_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl MagicTransitSiteAcl_Lan2 {
    #[doc = "Set the field `lan_name`.\nThe name of the LAN based on the provided lan_id."]
    pub fn set_lan_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lan_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_ranges`.\nArray of port ranges on the provided LAN that will be included in the ACL. If no ports or port rangess are provided, communication on any port on this LAN is allowed."]
    pub fn set_port_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.port_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ports`.\nArray of ports on the provided LAN that will be included in the ACL. If no ports or port ranges are provided, communication on any port on this LAN is allowed."]
    pub fn set_ports(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnets`.\nArray of subnet IPs within the LAN that will be included in the ACL. If no subnets are provided, communication on any subnets on this LAN are allowed."]
    pub fn set_subnets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitSiteAcl_Lan2 {
    type O = terrars::BlockAssignable<MagicTransitSiteAcl_Lan2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitSiteAcl_Lan2 {
    #[doc = "The identifier for the LAN you want to create an ACL policy with."]
    pub lan_id: terrars::PrimField<std::string::String>,
}

impl BuildMagicTransitSiteAcl_Lan2 {
    pub fn build(self) -> MagicTransitSiteAcl_Lan2 {
        MagicTransitSiteAcl_Lan2 {
            lan_id: self.lan_id,
            lan_name: core::default::Default::default(),
            port_ranges: core::default::Default::default(),
            ports: core::default::Default::default(),
            subnets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitSiteAcl_Lan2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitSiteAcl_Lan2Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicTransitSiteAcl_Lan2Ref {
        MagicTransitSiteAcl_Lan2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitSiteAcl_Lan2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `lan_id` after provisioning.\nThe identifier for the LAN you want to create an ACL policy with."]
    pub fn lan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lan_name` after provisioning.\nThe name of the LAN based on the provided lan_id."]
    pub fn lan_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_ranges` after provisioning.\nArray of port ranges on the provided LAN that will be included in the ACL. If no ports or port rangess are provided, communication on any port on this LAN is allowed."]
    pub fn port_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ports` after provisioning.\nArray of ports on the provided LAN that will be included in the ACL. If no ports or port ranges are provided, communication on any port on this LAN is allowed."]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnets` after provisioning.\nArray of subnet IPs within the LAN that will be included in the ACL. If no subnets are provided, communication on any subnets on this LAN are allowed."]
    pub fn subnets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}
