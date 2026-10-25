#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneDnsSettingsData_ {
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
    flatten_all_cnames: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    foundation_dns: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_dns: std::option::Option<ZoneDnsSettings_InternalDns>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_provider: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nameservers: std::option::Option<ZoneDnsSettings_Nameservers>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ns_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_overrides: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    soa: std::option::Option<ZoneDnsSettings_Soa>,
    zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ZoneDnsSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZoneDnsSettingsData_>,
}

#[derive(Clone)]
pub struct ZoneDnsSettings(std::rc::Rc<ZoneDnsSettings_>);

impl ZoneDnsSettings {
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
        "Set the field `flatten_all_cnames`.\nWhether to flatten all CNAME records in the zone. Note that, due to DNS limitations, a CNAME record at the zone apex will always be flattened."]
    pub fn set_flatten_all_cnames(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().flatten_all_cnames = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `foundation_dns`.\nDeprecated. Use nameservers.type to configure Advanced Nameservers."]
    pub fn set_foundation_dns(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().foundation_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal_dns`.\nSettings for this internal zone."]
    pub fn set_internal_dns(self, v: impl std::convert::Into<ZoneDnsSettings_InternalDns>) -> Self {
        self.0.data.borrow_mut().internal_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `multi_provider`.\nWhether to enable multi-provider DNS, which causes Cloudflare to activate the zone even when non-Cloudflare NS records exist, and to respect NS records at the zone apex during outbound zone transfers."]
    pub fn set_multi_provider(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nameservers`.\nControls the nameservers through which the zone is available."]
    pub fn set_nameservers(self, v: impl std::convert::Into<ZoneDnsSettings_Nameservers>) -> Self {
        self.0.data.borrow_mut().nameservers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ns_ttl`.\nThe time to live (TTL) of the zone's nameserver (NS) records."]
    pub fn set_ns_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ns_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secondary_overrides`.\nAllows a Secondary DNS zone to use (proxied) override records and CNAME flattening at the zone apex."]
    pub fn set_secondary_overrides(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().secondary_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `soa`.\nComponents of the zone's SOA record."]
    pub fn set_soa(self, v: impl std::convert::Into<ZoneDnsSettings_Soa>) -> Self {
        self.0.data.borrow_mut().soa = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone_mode`.\nWhether the zone mode is a regular or CDN/DNS only zone.\nAvailable values: \"standard\", \"cdn_only\", \"dns_only\"."]
    pub fn set_zone_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `flatten_all_cnames` after provisioning.\nWhether to flatten all CNAME records in the zone. Note that, due to DNS limitations, a CNAME record at the zone apex will always be flattened."]
    pub fn flatten_all_cnames(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flatten_all_cnames", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `foundation_dns` after provisioning.\nDeprecated. Use nameservers.type to configure Advanced Nameservers."]
    pub fn foundation_dns(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.foundation_dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `internal_dns` after provisioning.\nSettings for this internal zone."]
    pub fn internal_dns(&self) -> ZoneDnsSettings_InternalDnsRef {
        ZoneDnsSettings_InternalDnsRef::new(self.shared().clone(), format!("{}.internal_dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `multi_provider` after provisioning.\nWhether to enable multi-provider DNS, which causes Cloudflare to activate the zone even when non-Cloudflare NS records exist, and to respect NS records at the zone apex during outbound zone transfers."]
    pub fn multi_provider(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_provider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nameservers` after provisioning.\nControls the nameservers through which the zone is available."]
    pub fn nameservers(&self) -> ZoneDnsSettings_NameserversRef {
        ZoneDnsSettings_NameserversRef::new(self.shared().clone(), format!("{}.nameservers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ns_ttl` after provisioning.\nThe time to live (TTL) of the zone's nameserver (NS) records."]
    pub fn ns_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ns_ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secondary_overrides` after provisioning.\nAllows a Secondary DNS zone to use (proxied) override records and CNAME flattening at the zone apex."]
    pub fn secondary_overrides(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `soa` after provisioning.\nComponents of the zone's SOA record."]
    pub fn soa(&self) -> ZoneDnsSettings_SoaRef {
        ZoneDnsSettings_SoaRef::new(self.shared().clone(), format!("{}.soa", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_mode` after provisioning.\nWhether the zone mode is a regular or CDN/DNS only zone.\nAvailable values: \"standard\", \"cdn_only\", \"dns_only\"."]
    pub fn zone_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_mode", self.extract_ref()))
    }
}

impl terrars::Referable for ZoneDnsSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZoneDnsSettings { }

impl terrars::ToListMappable for ZoneDnsSettings {
    type O = terrars::ListRef<ZoneDnsSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZoneDnsSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zone_dns_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneDnsSettings {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildZoneDnsSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> ZoneDnsSettings {
        let out = ZoneDnsSettings(std::rc::Rc::new(ZoneDnsSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZoneDnsSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                flatten_all_cnames: core::default::Default::default(),
                foundation_dns: core::default::Default::default(),
                internal_dns: core::default::Default::default(),
                multi_provider: core::default::Default::default(),
                nameservers: core::default::Default::default(),
                ns_ttl: core::default::Default::default(),
                secondary_overrides: core::default::Default::default(),
                soa: core::default::Default::default(),
                zone_id: self.zone_id,
                zone_mode: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneDnsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneDnsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneDnsSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `flatten_all_cnames` after provisioning.\nWhether to flatten all CNAME records in the zone. Note that, due to DNS limitations, a CNAME record at the zone apex will always be flattened."]
    pub fn flatten_all_cnames(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flatten_all_cnames", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `foundation_dns` after provisioning.\nDeprecated. Use nameservers.type to configure Advanced Nameservers."]
    pub fn foundation_dns(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.foundation_dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `internal_dns` after provisioning.\nSettings for this internal zone."]
    pub fn internal_dns(&self) -> ZoneDnsSettings_InternalDnsRef {
        ZoneDnsSettings_InternalDnsRef::new(self.shared().clone(), format!("{}.internal_dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `multi_provider` after provisioning.\nWhether to enable multi-provider DNS, which causes Cloudflare to activate the zone even when non-Cloudflare NS records exist, and to respect NS records at the zone apex during outbound zone transfers."]
    pub fn multi_provider(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_provider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nameservers` after provisioning.\nControls the nameservers through which the zone is available."]
    pub fn nameservers(&self) -> ZoneDnsSettings_NameserversRef {
        ZoneDnsSettings_NameserversRef::new(self.shared().clone(), format!("{}.nameservers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ns_ttl` after provisioning.\nThe time to live (TTL) of the zone's nameserver (NS) records."]
    pub fn ns_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ns_ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secondary_overrides` after provisioning.\nAllows a Secondary DNS zone to use (proxied) override records and CNAME flattening at the zone apex."]
    pub fn secondary_overrides(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `soa` after provisioning.\nComponents of the zone's SOA record."]
    pub fn soa(&self) -> ZoneDnsSettings_SoaRef {
        ZoneDnsSettings_SoaRef::new(self.shared().clone(), format!("{}.soa", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_mode` after provisioning.\nWhether the zone mode is a regular or CDN/DNS only zone.\nAvailable values: \"standard\", \"cdn_only\", \"dns_only\"."]
    pub fn zone_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_mode", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZoneDnsSettings_InternalDns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZoneDnsSettings_InternalDns {
    #[doc = "Set the field `reference_zone_id`.\nThe ID of the zone to fallback to."]
    pub fn set_reference_zone_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reference_zone_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZoneDnsSettings_InternalDns {
    type O = terrars::BlockAssignable<ZoneDnsSettings_InternalDns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneDnsSettings_InternalDns {}

impl BuildZoneDnsSettings_InternalDns {
    pub fn build(self) -> ZoneDnsSettings_InternalDns {
        ZoneDnsSettings_InternalDns { reference_zone_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZoneDnsSettings_InternalDnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneDnsSettings_InternalDnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZoneDnsSettings_InternalDnsRef {
        ZoneDnsSettings_InternalDnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneDnsSettings_InternalDnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `reference_zone_id` after provisioning.\nThe ID of the zone to fallback to."]
    pub fn reference_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_zone_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneDnsSettings_Nameservers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nameserver_set_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ns_set: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ZoneDnsSettings_Nameservers {
    #[doc =
        "Set the field `nameserver_set_id`.\nIdentifier of the account-owned Custom Nameserver Set to use for this zone."]
    pub fn set_nameserver_set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nameserver_set_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ns_set`.\nConfigured nameserver set number to use for this zone."]
    pub fn set_ns_set(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ns_set = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZoneDnsSettings_Nameservers {
    type O = terrars::BlockAssignable<ZoneDnsSettings_Nameservers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneDnsSettings_Nameservers {
    #[doc =
        "Nameserver type.\nAvailable values: \"cloudflare.standard\", \"cloudflare.advanced\", \"custom.account\", \"custom.tenant\", \"custom.zone\", \"custom\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildZoneDnsSettings_Nameservers {
    pub fn build(self) -> ZoneDnsSettings_Nameservers {
        ZoneDnsSettings_Nameservers {
            nameserver_set_id: core::default::Default::default(),
            ns_set: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ZoneDnsSettings_NameserversRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneDnsSettings_NameserversRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZoneDnsSettings_NameserversRef {
        ZoneDnsSettings_NameserversRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneDnsSettings_NameserversRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nameserver_set_id` after provisioning.\nIdentifier of the account-owned Custom Nameserver Set to use for this zone."]
    pub fn nameserver_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nameserver_set_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ns_set` after provisioning.\nConfigured nameserver set number to use for this zone."]
    pub fn ns_set(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ns_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nNameserver type.\nAvailable values: \"cloudflare.standard\", \"cloudflare.advanced\", \"custom.account\", \"custom.tenant\", \"custom.zone\", \"custom\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneDnsSettings_Soa {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
}

impl ZoneDnsSettings_Soa {
    #[doc =
        "Set the field `expire`.\nTime in seconds of being unable to query the primary server after which secondary servers should stop serving the zone."]
    pub fn set_expire(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.expire = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_ttl`.\nThe time to live (TTL) for negative caching of records within the zone."]
    pub fn set_min_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mname`.\nThe primary nameserver, which may be used for outbound zone transfers. If null, a Cloudflare-assigned value will be used."]
    pub fn set_mname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `refresh`.\nTime in seconds after which secondary servers should re-check the SOA record to see if the zone has been updated."]
    pub fn set_refresh(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.refresh = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry`.\nTime in seconds after which secondary servers should retry queries after the primary server was unresponsive."]
    pub fn set_retry(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rname`.\nThe email address of the zone administrator, with the first label representing the local part of the email address."]
    pub fn set_rname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\nThe time to live (TTL) of the SOA record itself."]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZoneDnsSettings_Soa {
    type O = terrars::BlockAssignable<ZoneDnsSettings_Soa>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneDnsSettings_Soa {}

impl BuildZoneDnsSettings_Soa {
    pub fn build(self) -> ZoneDnsSettings_Soa {
        ZoneDnsSettings_Soa {
            expire: core::default::Default::default(),
            min_ttl: core::default::Default::default(),
            mname: core::default::Default::default(),
            refresh: core::default::Default::default(),
            retry: core::default::Default::default(),
            rname: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZoneDnsSettings_SoaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneDnsSettings_SoaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZoneDnsSettings_SoaRef {
        ZoneDnsSettings_SoaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneDnsSettings_SoaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expire` after provisioning.\nTime in seconds of being unable to query the primary server after which secondary servers should stop serving the zone."]
    pub fn expire(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_ttl` after provisioning.\nThe time to live (TTL) for negative caching of records within the zone."]
    pub fn min_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mname` after provisioning.\nThe primary nameserver, which may be used for outbound zone transfers. If null, a Cloudflare-assigned value will be used."]
    pub fn mname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `refresh` after provisioning.\nTime in seconds after which secondary servers should re-check the SOA record to see if the zone has been updated."]
    pub fn refresh(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry` after provisioning.\nTime in seconds after which secondary servers should retry queries after the primary server was unresponsive."]
    pub fn retry(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rname` after provisioning.\nThe email address of the zone administrator, with the first label representing the local part of the email address."]
    pub fn rname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe time to live (TTL) of the SOA record itself."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
