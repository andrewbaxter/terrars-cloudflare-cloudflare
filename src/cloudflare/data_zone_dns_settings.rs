#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZoneDnsSettingsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZoneDnsSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZoneDnsSettingsData_>,
}

#[derive(Clone)]
pub struct DataZoneDnsSettings(std::rc::Rc<DataZoneDnsSettings_>);

impl DataZoneDnsSettings {
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

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
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
    pub fn internal_dns(&self) -> DataZoneDnsSettings_InternalDnsRef {
        DataZoneDnsSettings_InternalDnsRef::new(
            self.shared().clone(),
            format!("{}.internal_dns", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `multi_provider` after provisioning.\nWhether to enable multi-provider DNS, which causes Cloudflare to activate the zone even when non-Cloudflare NS records exist, and to respect NS records at the zone apex during outbound zone transfers."]
    pub fn multi_provider(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_provider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nameservers` after provisioning.\nControls the nameservers through which the zone is available."]
    pub fn nameservers(&self) -> DataZoneDnsSettings_NameserversRef {
        DataZoneDnsSettings_NameserversRef::new(self.shared().clone(), format!("{}.nameservers", self.extract_ref()))
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
    pub fn soa(&self) -> DataZoneDnsSettings_SoaRef {
        DataZoneDnsSettings_SoaRef::new(self.shared().clone(), format!("{}.soa", self.extract_ref()))
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

impl terrars::Referable for DataZoneDnsSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZoneDnsSettings { }

impl terrars::ToListMappable for DataZoneDnsSettings {
    type O = terrars::ListRef<DataZoneDnsSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZoneDnsSettings_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zone_dns_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZoneDnsSettings {
    pub tf_id: std::string::String,
}

impl BuildDataZoneDnsSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZoneDnsSettings {
        let out = DataZoneDnsSettings(std::rc::Rc::new(DataZoneDnsSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZoneDnsSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZoneDnsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneDnsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZoneDnsSettingsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
    pub fn internal_dns(&self) -> DataZoneDnsSettings_InternalDnsRef {
        DataZoneDnsSettings_InternalDnsRef::new(
            self.shared().clone(),
            format!("{}.internal_dns", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `multi_provider` after provisioning.\nWhether to enable multi-provider DNS, which causes Cloudflare to activate the zone even when non-Cloudflare NS records exist, and to respect NS records at the zone apex during outbound zone transfers."]
    pub fn multi_provider(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_provider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nameservers` after provisioning.\nControls the nameservers through which the zone is available."]
    pub fn nameservers(&self) -> DataZoneDnsSettings_NameserversRef {
        DataZoneDnsSettings_NameserversRef::new(self.shared().clone(), format!("{}.nameservers", self.extract_ref()))
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
    pub fn soa(&self) -> DataZoneDnsSettings_SoaRef {
        DataZoneDnsSettings_SoaRef::new(self.shared().clone(), format!("{}.soa", self.extract_ref()))
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
pub struct DataZoneDnsSettings_InternalDns {}

impl DataZoneDnsSettings_InternalDns { }

impl terrars::ToListMappable for DataZoneDnsSettings_InternalDns {
    type O = terrars::BlockAssignable<DataZoneDnsSettings_InternalDns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZoneDnsSettings_InternalDns {}

impl BuildDataZoneDnsSettings_InternalDns {
    pub fn build(self) -> DataZoneDnsSettings_InternalDns {
        DataZoneDnsSettings_InternalDns {}
    }
}

#[allow(dead_code)]
pub struct DataZoneDnsSettings_InternalDnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneDnsSettings_InternalDnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZoneDnsSettings_InternalDnsRef {
        DataZoneDnsSettings_InternalDnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZoneDnsSettings_InternalDnsRef {
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
pub struct DataZoneDnsSettings_Nameservers {}

impl DataZoneDnsSettings_Nameservers { }

impl terrars::ToListMappable for DataZoneDnsSettings_Nameservers {
    type O = terrars::BlockAssignable<DataZoneDnsSettings_Nameservers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZoneDnsSettings_Nameservers {}

impl BuildDataZoneDnsSettings_Nameservers {
    pub fn build(self) -> DataZoneDnsSettings_Nameservers {
        DataZoneDnsSettings_Nameservers {}
    }
}

#[allow(dead_code)]
pub struct DataZoneDnsSettings_NameserversRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneDnsSettings_NameserversRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZoneDnsSettings_NameserversRef {
        DataZoneDnsSettings_NameserversRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZoneDnsSettings_NameserversRef {
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
        "Get a reference to the value of field `type_` after provisioning.\nNameserver type.\nAvailable values: \"cloudflare.standard\", \"custom.account\", \"custom.tenant\", \"custom.zone\", \"custom\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZoneDnsSettings_Soa {}

impl DataZoneDnsSettings_Soa { }

impl terrars::ToListMappable for DataZoneDnsSettings_Soa {
    type O = terrars::BlockAssignable<DataZoneDnsSettings_Soa>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZoneDnsSettings_Soa {}

impl BuildDataZoneDnsSettings_Soa {
    pub fn build(self) -> DataZoneDnsSettings_Soa {
        DataZoneDnsSettings_Soa {}
    }
}

#[allow(dead_code)]
pub struct DataZoneDnsSettings_SoaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneDnsSettings_SoaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZoneDnsSettings_SoaRef {
        DataZoneDnsSettings_SoaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZoneDnsSettings_SoaRef {
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
