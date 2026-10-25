#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneData_ {
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
    account: Zone_Account,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paused: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vanity_name_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

struct Zone_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZoneData_>,
}

#[derive(Clone)]
pub struct Zone(std::rc::Rc<Zone_>);

impl Zone {
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
        "Set the field `paused`.\nIndicates whether the zone is only using Cloudflare DNS services. A\ntrue value means the zone will not receive security or performance\nbenefits."]
    pub fn set_paused(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().paused = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nA full zone implies that DNS is hosted with Cloudflare. A partial zone is\ntypically a partner-hosted zone or a CNAME setup.\nAvailable values: \"full\", \"partial\", \"secondary\", \"internal\"."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vanity_name_servers`.\nAn array of domains used for custom name servers. This is only\navailable for Business and Enterprise plans."]
    pub fn set_vanity_name_servers(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().vanity_name_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> Zone_AccountRef {
        Zone_AccountRef::new(self.shared().clone(), format!("{}.account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `activated_on` after provisioning.\nThe last time proof of ownership was detected and the zone was made\nactive."]
    pub fn activated_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activated_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cname_suffix` after provisioning.\nAllows the customer to use a custom apex.\n*Tenants Only Configuration*."]
    pub fn cname_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the zone was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `development_mode` after provisioning.\nThe interval (in seconds) from when development mode expires\n(positive integer) or last expired (negative integer) for the\ndomain. If development mode has never been enabled, this value is 0."]
    pub fn development_mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.development_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\nMetadata about the zone."]
    pub fn meta(&self) -> Zone_MetaRef {
        Zone_MetaRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\nWhen the zone was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment (\"label\") not exceeding 63 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name_servers` after provisioning.\nThe name servers Cloudflare assigns to a zone."]
    pub fn name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `original_dnshost` after provisioning.\nDNS host at the time of switching to Cloudflare."]
    pub fn original_dnshost(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.original_dnshost", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `original_name_servers` after provisioning.\nOriginal name servers before moving to Cloudflare."]
    pub fn original_name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.original_name_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `original_registrar` after provisioning.\nRegistrar for the domain at the time of switching to Cloudflare."]
    pub fn original_registrar(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.original_registrar", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\nThe owner of the zone."]
    pub fn owner(&self) -> Zone_OwnerRef {
        Zone_OwnerRef::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nIndicates whether the zone is only using Cloudflare DNS services. A\ntrue value means the zone will not receive security or performance\nbenefits."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nLegacy permissions based on legacy user membership information."]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plan` after provisioning.\nA Zones subscription information."]
    pub fn plan(&self) -> Zone_PlanRef {
        Zone_PlanRef::new(self.shared().clone(), format!("{}.plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe zone status on Cloudflare.\nAvailable values: \"initializing\", \"pending\", \"active\", \"moved\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant` after provisioning.\nThe root organizational unit that this zone belongs to (such as a tenant or organization)."]
    pub fn tenant(&self) -> Zone_TenantRef {
        Zone_TenantRef::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant_unit` after provisioning.\nThe immediate parent organizational unit that this zone belongs to (such as under a tenant or sub-organization)."]
    pub fn tenant_unit(&self) -> Zone_TenantUnitRef {
        Zone_TenantUnitRef::new(self.shared().clone(), format!("{}.tenant_unit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nA full zone implies that DNS is hosted with Cloudflare. A partial zone is\ntypically a partner-hosted zone or a CNAME setup.\nAvailable values: \"full\", \"partial\", \"secondary\", \"internal\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vanity_name_servers` after provisioning.\nAn array of domains used for custom name servers. This is only\navailable for Business and Enterprise plans."]
    pub fn vanity_name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vanity_name_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `verification_key` after provisioning.\nVerification key for partial zone setup."]
    pub fn verification_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_key", self.extract_ref()))
    }
}

impl terrars::Referable for Zone {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Zone { }

impl terrars::ToListMappable for Zone {
    type O = terrars::ListRef<ZoneRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Zone_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zone".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZone {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account: Zone_Account,
    #[doc =
        "The domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment (\"label\") not exceeding 63 characters."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZone {
    pub fn build(self, stack: &mut terrars::Stack) -> Zone {
        let out = Zone(std::rc::Rc::new(Zone_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZoneData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account: self.account,
                name: self.name,
                paused: core::default::Default::default(),
                type_: core::default::Default::default(),
                vanity_name_servers: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> Zone_AccountRef {
        Zone_AccountRef::new(self.shared().clone(), format!("{}.account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `activated_on` after provisioning.\nThe last time proof of ownership was detected and the zone was made\nactive."]
    pub fn activated_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activated_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cname_suffix` after provisioning.\nAllows the customer to use a custom apex.\n*Tenants Only Configuration*."]
    pub fn cname_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the zone was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `development_mode` after provisioning.\nThe interval (in seconds) from when development mode expires\n(positive integer) or last expired (negative integer) for the\ndomain. If development mode has never been enabled, this value is 0."]
    pub fn development_mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.development_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\nMetadata about the zone."]
    pub fn meta(&self) -> Zone_MetaRef {
        Zone_MetaRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\nWhen the zone was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment (\"label\") not exceeding 63 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name_servers` after provisioning.\nThe name servers Cloudflare assigns to a zone."]
    pub fn name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `original_dnshost` after provisioning.\nDNS host at the time of switching to Cloudflare."]
    pub fn original_dnshost(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.original_dnshost", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `original_name_servers` after provisioning.\nOriginal name servers before moving to Cloudflare."]
    pub fn original_name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.original_name_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `original_registrar` after provisioning.\nRegistrar for the domain at the time of switching to Cloudflare."]
    pub fn original_registrar(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.original_registrar", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\nThe owner of the zone."]
    pub fn owner(&self) -> Zone_OwnerRef {
        Zone_OwnerRef::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nIndicates whether the zone is only using Cloudflare DNS services. A\ntrue value means the zone will not receive security or performance\nbenefits."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nLegacy permissions based on legacy user membership information."]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plan` after provisioning.\nA Zones subscription information."]
    pub fn plan(&self) -> Zone_PlanRef {
        Zone_PlanRef::new(self.shared().clone(), format!("{}.plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe zone status on Cloudflare.\nAvailable values: \"initializing\", \"pending\", \"active\", \"moved\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant` after provisioning.\nThe root organizational unit that this zone belongs to (such as a tenant or organization)."]
    pub fn tenant(&self) -> Zone_TenantRef {
        Zone_TenantRef::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant_unit` after provisioning.\nThe immediate parent organizational unit that this zone belongs to (such as under a tenant or sub-organization)."]
    pub fn tenant_unit(&self) -> Zone_TenantUnitRef {
        Zone_TenantUnitRef::new(self.shared().clone(), format!("{}.tenant_unit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nA full zone implies that DNS is hosted with Cloudflare. A partial zone is\ntypically a partner-hosted zone or a CNAME setup.\nAvailable values: \"full\", \"partial\", \"secondary\", \"internal\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vanity_name_servers` after provisioning.\nAn array of domains used for custom name servers. This is only\navailable for Business and Enterprise plans."]
    pub fn vanity_name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vanity_name_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `verification_key` after provisioning.\nVerification key for partial zone setup."]
    pub fn verification_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_key", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Zone_Account {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Zone_Account {
    #[doc = "Set the field `id`.\nIdentifier"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Zone_Account {
    type O = terrars::BlockAssignable<Zone_Account>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZone_Account {}

impl BuildZone_Account {
    pub fn build(self) -> Zone_Account {
        Zone_Account { id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Zone_AccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Zone_AccountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Zone_AccountRef {
        Zone_AccountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Zone_AccountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct Zone_Meta {}

impl Zone_Meta { }

impl terrars::ToListMappable for Zone_Meta {
    type O = terrars::BlockAssignable<Zone_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZone_Meta {}

impl BuildZone_Meta {
    pub fn build(self) -> Zone_Meta {
        Zone_Meta {}
    }
}

#[allow(dead_code)]
pub struct Zone_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Zone_MetaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Zone_MetaRef {
        Zone_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Zone_MetaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cdn_only` after provisioning.\nThe zone is only configured for CDN."]
    pub fn cdn_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdn_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_certificate_quota` after provisioning.\nNumber of Custom Certificates the zone can have."]
    pub fn custom_certificate_quota(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_certificate_quota", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_only` after provisioning.\nThe zone is only configured for DNS."]
    pub fn dns_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `foundation_dns` after provisioning.\nThe zone is setup with Foundation DNS."]
    pub fn foundation_dns(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.foundation_dns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `page_rule_quota` after provisioning.\nNumber of Page Rules a zone can have."]
    pub fn page_rule_quota(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_rule_quota", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phishing_detected` after provisioning.\nThe zone has been flagged for phishing."]
    pub fn phishing_detected(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phishing_detected", self.base))
    }

    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.step", self.base))
    }
}

#[derive(Serialize)]
pub struct Zone_Owner {}

impl Zone_Owner { }

impl terrars::ToListMappable for Zone_Owner {
    type O = terrars::BlockAssignable<Zone_Owner>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZone_Owner {}

impl BuildZone_Owner {
    pub fn build(self) -> Zone_Owner {
        Zone_Owner {}
    }
}

#[allow(dead_code)]
pub struct Zone_OwnerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Zone_OwnerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Zone_OwnerRef {
        Zone_OwnerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Zone_OwnerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the owner."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of owner."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct Zone_Plan {}

impl Zone_Plan { }

impl terrars::ToListMappable for Zone_Plan {
    type O = terrars::BlockAssignable<Zone_Plan>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZone_Plan {}

impl BuildZone_Plan {
    pub fn build(self) -> Zone_Plan {
        Zone_Plan {}
    }
}

#[allow(dead_code)]
pub struct Zone_PlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Zone_PlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Zone_PlanRef {
        Zone_PlanRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Zone_PlanRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `can_subscribe` after provisioning.\nStates if the subscription can be activated."]
    pub fn can_subscribe(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_subscribe", self.base))
    }

    #[doc = "Get a reference to the value of field `currency` after provisioning.\nThe denomination of the customer."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `externally_managed` after provisioning.\nIf this Zone is managed by another company."]
    pub fn externally_managed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.externally_managed", self.base))
    }

    #[doc = "Get a reference to the value of field `frequency` after provisioning.\nHow often the customer is billed."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_subscribed` after provisioning.\nStates if the subscription active."]
    pub fn is_subscribed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_subscribed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `legacy_discount` after provisioning.\nIf the legacy discount applies to this Zone."]
    pub fn legacy_discount(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.legacy_discount", self.base))
    }

    #[doc = "Get a reference to the value of field `legacy_id` after provisioning.\nThe legacy name of the plan."]
    pub fn legacy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.legacy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the owner."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `price` after provisioning.\nHow much the customer is paying."]
    pub fn price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price", self.base))
    }
}

#[derive(Serialize)]
pub struct Zone_Tenant {}

impl Zone_Tenant { }

impl terrars::ToListMappable for Zone_Tenant {
    type O = terrars::BlockAssignable<Zone_Tenant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZone_Tenant {}

impl BuildZone_Tenant {
    pub fn build(self) -> Zone_Tenant {
        Zone_Tenant {}
    }
}

#[allow(dead_code)]
pub struct Zone_TenantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Zone_TenantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Zone_TenantRef {
        Zone_TenantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Zone_TenantRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Tenant account."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Zone_TenantUnit {}

impl Zone_TenantUnit { }

impl terrars::ToListMappable for Zone_TenantUnit {
    type O = terrars::BlockAssignable<Zone_TenantUnit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZone_TenantUnit {}

impl BuildZone_TenantUnit {
    pub fn build(self) -> Zone_TenantUnit {
        Zone_TenantUnit {}
    }
}

#[allow(dead_code)]
pub struct Zone_TenantUnitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Zone_TenantUnitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Zone_TenantUnitRef {
        Zone_TenantUnitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Zone_TenantUnitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
