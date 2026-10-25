#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDnsLocationData_ {
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
    client_default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_destination_ips_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<ZeroTrustDnsLocation_Endpoints>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_ttl: std::option::Option<ZeroTrustDnsLocation_MaxTtl>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks: std::option::Option<std::vec::Vec<ZeroTrustDnsLocation_Networks>>,
}

struct ZeroTrustDnsLocation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDnsLocationData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDnsLocation(std::rc::Rc<ZeroTrustDnsLocation_>);

impl ZeroTrustDnsLocation {
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

    #[doc = "Set the field `client_default`.\nIndicate whether this location is the default location."]
    pub fn set_client_default(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().client_default = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dns_destination_ips_id`.\nSpecify the identifier of the pair of IPv4 addresses assigned to this location. When creating a location, if this field is absent or set to null, the pair of shared IPv4 addresses (0e4a32c6-6fb8-4858-9296-98f51631e8e6) is auto-assigned. When updating a location, if this field is absent or set to null, the pre-assigned pair remains unchanged."]
    pub fn set_dns_destination_ips_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().dns_destination_ips_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ecs_support`.\nIndicate whether the location must resolve EDNS queries."]
    pub fn set_ecs_support(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ecs_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoints`.\nConfigure the destination endpoints for this location."]
    pub fn set_endpoints(self, v: impl std::convert::Into<ZeroTrustDnsLocation_Endpoints>) -> Self {
        self.0.data.borrow_mut().endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_ttl`.\nControls how DNS response TTLs are capped for this location relative to the account `max_ttl_secs` setting. Omitting `max_ttl` on update resets it to `inherit`."]
    pub fn set_max_ttl(self, v: impl std::convert::Into<ZeroTrustDnsLocation_MaxTtl>) -> Self {
        self.0.data.borrow_mut().max_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `networks`.\nSpecify the list of network ranges from which requests at this location originate. The list takes effect only if it is non-empty and the IPv4 endpoint is enabled for this location."]
    pub fn set_networks(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustDnsLocation_Networks>>) -> Self {
        self.0.data.borrow_mut().networks = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_default` after provisioning.\nIndicate whether this location is the default location."]
    pub fn client_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_destination_ips_id` after provisioning.\nSpecify the identifier of the pair of IPv4 addresses assigned to this location. When creating a location, if this field is absent or set to null, the pair of shared IPv4 addresses (0e4a32c6-6fb8-4858-9296-98f51631e8e6) is auto-assigned. When updating a location, if this field is absent or set to null, the pre-assigned pair remains unchanged."]
    pub fn dns_destination_ips_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_destination_ips_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_destination_ipv6_block_id` after provisioning.\nSpecify the UUID of the IPv6 block brought to the gateway so that this location's IPv6 address is allocated from the Bring Your Own IPv6 (BYOIPv6) block rather than the standard Cloudflare IPv6 block."]
    pub fn dns_destination_ipv6_block_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_destination_ipv6_block_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `doh_subdomain` after provisioning.\nSpecify the DNS over HTTPS domain that receives DNS requests. Gateway automatically generates this value."]
    pub fn doh_subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.doh_subdomain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ecs_support` after provisioning.\nIndicate whether the location must resolve EDNS queries."]
    pub fn ecs_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_support", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoints` after provisioning.\nConfigure the destination endpoints for this location."]
    pub fn endpoints(&self) -> ZeroTrustDnsLocation_EndpointsRef {
        ZeroTrustDnsLocation_EndpointsRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nDefines the automatically generated IPv6 destination IP assigned to this location. Gateway counts all DNS requests sent to this IP as requests under this location."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_destination` after provisioning.\nShow the primary destination IPv4 address from the pair identified dns_destination_ips_id. This field read-only."]
    pub fn ipv4_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_destination", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_destination_backup` after provisioning.\nShow the backup destination IPv4 address from the pair identified dns_destination_ips_id. This field read-only."]
    pub fn ipv4_destination_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_destination_backup", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_ttl` after provisioning.\nControls how DNS response TTLs are capped for this location relative to the account `max_ttl_secs` setting. Omitting `max_ttl` on update resets it to `inherit`."]
    pub fn max_ttl(&self) -> ZeroTrustDnsLocation_MaxTtlRef {
        ZeroTrustDnsLocation_MaxTtlRef::new(self.shared().clone(), format!("{}.max_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecify the location name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nSpecify the list of network ranges from which requests at this location originate. The list takes effect only if it is non-empty and the IPv4 endpoint is enabled for this location."]
    pub fn networks(&self) -> terrars::ListRef<ZeroTrustDnsLocation_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustDnsLocation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDnsLocation { }

impl terrars::ToListMappable for ZeroTrustDnsLocation {
    type O = terrars::ListRef<ZeroTrustDnsLocationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDnsLocation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dns_location".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDnsLocation {
    pub tf_id: std::string::String,
    #[doc = "Specify the Cloudflare account identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Specify the location name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDnsLocation {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDnsLocation {
        let out = ZeroTrustDnsLocation(std::rc::Rc::new(ZeroTrustDnsLocation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDnsLocationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                client_default: core::default::Default::default(),
                dns_destination_ips_id: core::default::Default::default(),
                ecs_support: core::default::Default::default(),
                endpoints: core::default::Default::default(),
                max_ttl: core::default::Default::default(),
                name: self.name,
                networks: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDnsLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDnsLocationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_default` after provisioning.\nIndicate whether this location is the default location."]
    pub fn client_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_destination_ips_id` after provisioning.\nSpecify the identifier of the pair of IPv4 addresses assigned to this location. When creating a location, if this field is absent or set to null, the pair of shared IPv4 addresses (0e4a32c6-6fb8-4858-9296-98f51631e8e6) is auto-assigned. When updating a location, if this field is absent or set to null, the pre-assigned pair remains unchanged."]
    pub fn dns_destination_ips_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_destination_ips_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_destination_ipv6_block_id` after provisioning.\nSpecify the UUID of the IPv6 block brought to the gateway so that this location's IPv6 address is allocated from the Bring Your Own IPv6 (BYOIPv6) block rather than the standard Cloudflare IPv6 block."]
    pub fn dns_destination_ipv6_block_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_destination_ipv6_block_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `doh_subdomain` after provisioning.\nSpecify the DNS over HTTPS domain that receives DNS requests. Gateway automatically generates this value."]
    pub fn doh_subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.doh_subdomain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ecs_support` after provisioning.\nIndicate whether the location must resolve EDNS queries."]
    pub fn ecs_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_support", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoints` after provisioning.\nConfigure the destination endpoints for this location."]
    pub fn endpoints(&self) -> ZeroTrustDnsLocation_EndpointsRef {
        ZeroTrustDnsLocation_EndpointsRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nDefines the automatically generated IPv6 destination IP assigned to this location. Gateway counts all DNS requests sent to this IP as requests under this location."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_destination` after provisioning.\nShow the primary destination IPv4 address from the pair identified dns_destination_ips_id. This field read-only."]
    pub fn ipv4_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_destination", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_destination_backup` after provisioning.\nShow the backup destination IPv4 address from the pair identified dns_destination_ips_id. This field read-only."]
    pub fn ipv4_destination_backup(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_destination_backup", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_ttl` after provisioning.\nControls how DNS response TTLs are capped for this location relative to the account `max_ttl_secs` setting. Omitting `max_ttl` on update resets it to `inherit`."]
    pub fn max_ttl(&self) -> ZeroTrustDnsLocation_MaxTtlRef {
        ZeroTrustDnsLocation_MaxTtlRef::new(self.shared().clone(), format!("{}.max_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecify the location name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nSpecify the list of network ranges from which requests at this location originate. The list takes effect only if it is non-empty and the IPv4 endpoint is enabled for this location."]
    pub fn networks(&self) -> terrars::ListRef<ZeroTrustDnsLocation_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDnsLocation_Endpoints_Doh_Networks {
    network: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDnsLocation_Endpoints_Doh_Networks { }

impl terrars::ToListMappable for ZeroTrustDnsLocation_Endpoints_Doh_Networks {
    type O = terrars::BlockAssignable<ZeroTrustDnsLocation_Endpoints_Doh_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDnsLocation_Endpoints_Doh_Networks {
    #[doc = "Specify the IP address or IP CIDR."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDnsLocation_Endpoints_Doh_Networks {
    pub fn build(self) -> ZeroTrustDnsLocation_Endpoints_Doh_Networks {
        ZeroTrustDnsLocation_Endpoints_Doh_Networks { network: self.network }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDnsLocation_Endpoints_Doh_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocation_Endpoints_Doh_NetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDnsLocation_Endpoints_Doh_NetworksRef {
        ZeroTrustDnsLocation_Endpoints_Doh_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDnsLocation_Endpoints_Doh_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\nSpecify the IP address or IP CIDR."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDnsLocation_Endpoints_Doh {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks: std::option::Option<std::vec::Vec<ZeroTrustDnsLocation_Endpoints_Doh_Networks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_token: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustDnsLocation_Endpoints_Doh {
    #[doc = "Set the field `enabled`.\nIndicate whether the DOH endpoint is enabled for this location."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `networks`.\nSpecify the list of allowed source IP network ranges for this endpoint. When the list is empty, the endpoint allows all source IPs. The list takes effect only if the endpoint is enabled for this location."]
    pub fn set_networks(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustDnsLocation_Endpoints_Doh_Networks>>,
    ) -> Self {
        self.networks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_token`.\nSpecify whether the DOH endpoint requires user identity authentication."]
    pub fn set_require_token(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDnsLocation_Endpoints_Doh {
    type O = terrars::BlockAssignable<ZeroTrustDnsLocation_Endpoints_Doh>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDnsLocation_Endpoints_Doh {}

impl BuildZeroTrustDnsLocation_Endpoints_Doh {
    pub fn build(self) -> ZeroTrustDnsLocation_Endpoints_Doh {
        ZeroTrustDnsLocation_Endpoints_Doh {
            enabled: core::default::Default::default(),
            networks: core::default::Default::default(),
            require_token: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDnsLocation_Endpoints_DohRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocation_Endpoints_DohRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDnsLocation_Endpoints_DohRef {
        ZeroTrustDnsLocation_Endpoints_DohRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDnsLocation_Endpoints_DohRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicate whether the DOH endpoint is enabled for this location."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nSpecify the list of allowed source IP network ranges for this endpoint. When the list is empty, the endpoint allows all source IPs. The list takes effect only if the endpoint is enabled for this location."]
    pub fn networks(&self) -> terrars::ListRef<ZeroTrustDnsLocation_Endpoints_Doh_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_token` after provisioning.\nSpecify whether the DOH endpoint requires user identity authentication."]
    pub fn require_token(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_token", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDnsLocation_Endpoints_Dot_Networks {
    network: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDnsLocation_Endpoints_Dot_Networks { }

impl terrars::ToListMappable for ZeroTrustDnsLocation_Endpoints_Dot_Networks {
    type O = terrars::BlockAssignable<ZeroTrustDnsLocation_Endpoints_Dot_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDnsLocation_Endpoints_Dot_Networks {
    #[doc = "Specify the IP address or IP CIDR."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDnsLocation_Endpoints_Dot_Networks {
    pub fn build(self) -> ZeroTrustDnsLocation_Endpoints_Dot_Networks {
        ZeroTrustDnsLocation_Endpoints_Dot_Networks { network: self.network }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDnsLocation_Endpoints_Dot_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocation_Endpoints_Dot_NetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDnsLocation_Endpoints_Dot_NetworksRef {
        ZeroTrustDnsLocation_Endpoints_Dot_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDnsLocation_Endpoints_Dot_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\nSpecify the IP address or IP CIDR."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDnsLocation_Endpoints_Dot {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks: std::option::Option<std::vec::Vec<ZeroTrustDnsLocation_Endpoints_Dot_Networks>>,
}

impl ZeroTrustDnsLocation_Endpoints_Dot {
    #[doc = "Set the field `enabled`.\nIndicate whether the DOT endpoint is enabled for this location."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `networks`.\nSpecify the list of allowed source IP network ranges for this endpoint. When the list is empty, the endpoint allows all source IPs. The list takes effect only if the endpoint is enabled for this location."]
    pub fn set_networks(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustDnsLocation_Endpoints_Dot_Networks>>,
    ) -> Self {
        self.networks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDnsLocation_Endpoints_Dot {
    type O = terrars::BlockAssignable<ZeroTrustDnsLocation_Endpoints_Dot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDnsLocation_Endpoints_Dot {}

impl BuildZeroTrustDnsLocation_Endpoints_Dot {
    pub fn build(self) -> ZeroTrustDnsLocation_Endpoints_Dot {
        ZeroTrustDnsLocation_Endpoints_Dot {
            enabled: core::default::Default::default(),
            networks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDnsLocation_Endpoints_DotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocation_Endpoints_DotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDnsLocation_Endpoints_DotRef {
        ZeroTrustDnsLocation_Endpoints_DotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDnsLocation_Endpoints_DotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicate whether the DOT endpoint is enabled for this location."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nSpecify the list of allowed source IP network ranges for this endpoint. When the list is empty, the endpoint allows all source IPs. The list takes effect only if the endpoint is enabled for this location."]
    pub fn networks(&self) -> terrars::ListRef<ZeroTrustDnsLocation_Endpoints_Dot_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDnsLocation_Endpoints_Ipv4 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustDnsLocation_Endpoints_Ipv4 {
    #[doc = "Set the field `enabled`.\nIndicate whether the IPv4 endpoint is enabled for this location."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDnsLocation_Endpoints_Ipv4 {
    type O = terrars::BlockAssignable<ZeroTrustDnsLocation_Endpoints_Ipv4>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDnsLocation_Endpoints_Ipv4 {}

impl BuildZeroTrustDnsLocation_Endpoints_Ipv4 {
    pub fn build(self) -> ZeroTrustDnsLocation_Endpoints_Ipv4 {
        ZeroTrustDnsLocation_Endpoints_Ipv4 { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDnsLocation_Endpoints_Ipv4Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocation_Endpoints_Ipv4Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDnsLocation_Endpoints_Ipv4Ref {
        ZeroTrustDnsLocation_Endpoints_Ipv4Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDnsLocation_Endpoints_Ipv4Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicate whether the IPv4 endpoint is enabled for this location."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDnsLocation_Endpoints_Ipv6_Networks {
    network: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDnsLocation_Endpoints_Ipv6_Networks { }

impl terrars::ToListMappable for ZeroTrustDnsLocation_Endpoints_Ipv6_Networks {
    type O = terrars::BlockAssignable<ZeroTrustDnsLocation_Endpoints_Ipv6_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDnsLocation_Endpoints_Ipv6_Networks {
    #[doc = "Specify the IPv6 address or IPv6 CIDR."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDnsLocation_Endpoints_Ipv6_Networks {
    pub fn build(self) -> ZeroTrustDnsLocation_Endpoints_Ipv6_Networks {
        ZeroTrustDnsLocation_Endpoints_Ipv6_Networks { network: self.network }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDnsLocation_Endpoints_Ipv6_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocation_Endpoints_Ipv6_NetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDnsLocation_Endpoints_Ipv6_NetworksRef {
        ZeroTrustDnsLocation_Endpoints_Ipv6_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDnsLocation_Endpoints_Ipv6_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nSpecify the IPv6 address or IPv6 CIDR."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDnsLocation_Endpoints_Ipv6 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks: std::option::Option<std::vec::Vec<ZeroTrustDnsLocation_Endpoints_Ipv6_Networks>>,
}

impl ZeroTrustDnsLocation_Endpoints_Ipv6 {
    #[doc = "Set the field `enabled`.\nIndicate whether the IPV6 endpoint is enabled for this location."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `networks`.\nSpecify the list of allowed source IPv6 network ranges for this endpoint. When the list is empty, the endpoint allows all source IPs. The list takes effect only if the endpoint is enabled for this location."]
    pub fn set_networks(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustDnsLocation_Endpoints_Ipv6_Networks>>,
    ) -> Self {
        self.networks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDnsLocation_Endpoints_Ipv6 {
    type O = terrars::BlockAssignable<ZeroTrustDnsLocation_Endpoints_Ipv6>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDnsLocation_Endpoints_Ipv6 {}

impl BuildZeroTrustDnsLocation_Endpoints_Ipv6 {
    pub fn build(self) -> ZeroTrustDnsLocation_Endpoints_Ipv6 {
        ZeroTrustDnsLocation_Endpoints_Ipv6 {
            enabled: core::default::Default::default(),
            networks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDnsLocation_Endpoints_Ipv6Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocation_Endpoints_Ipv6Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDnsLocation_Endpoints_Ipv6Ref {
        ZeroTrustDnsLocation_Endpoints_Ipv6Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDnsLocation_Endpoints_Ipv6Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicate whether the IPV6 endpoint is enabled for this location."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nSpecify the list of allowed source IPv6 network ranges for this endpoint. When the list is empty, the endpoint allows all source IPs. The list takes effect only if the endpoint is enabled for this location."]
    pub fn networks(&self) -> terrars::ListRef<ZeroTrustDnsLocation_Endpoints_Ipv6_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDnsLocation_Endpoints {
    doh: ZeroTrustDnsLocation_Endpoints_Doh,
    dot: ZeroTrustDnsLocation_Endpoints_Dot,
    ipv4: ZeroTrustDnsLocation_Endpoints_Ipv4,
    ipv6: ZeroTrustDnsLocation_Endpoints_Ipv6,
}

impl ZeroTrustDnsLocation_Endpoints { }

impl terrars::ToListMappable for ZeroTrustDnsLocation_Endpoints {
    type O = terrars::BlockAssignable<ZeroTrustDnsLocation_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDnsLocation_Endpoints {
    #[doc = ""]
    pub doh: ZeroTrustDnsLocation_Endpoints_Doh,
    #[doc = ""]
    pub dot: ZeroTrustDnsLocation_Endpoints_Dot,
    #[doc = ""]
    pub ipv4: ZeroTrustDnsLocation_Endpoints_Ipv4,
    #[doc = ""]
    pub ipv6: ZeroTrustDnsLocation_Endpoints_Ipv6,
}

impl BuildZeroTrustDnsLocation_Endpoints {
    pub fn build(self) -> ZeroTrustDnsLocation_Endpoints {
        ZeroTrustDnsLocation_Endpoints {
            doh: self.doh,
            dot: self.dot,
            ipv4: self.ipv4,
            ipv6: self.ipv6,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDnsLocation_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocation_EndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDnsLocation_EndpointsRef {
        ZeroTrustDnsLocation_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDnsLocation_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `doh` after provisioning.\n"]
    pub fn doh(&self) -> ZeroTrustDnsLocation_Endpoints_DohRef {
        ZeroTrustDnsLocation_Endpoints_DohRef::new(self.shared().clone(), format!("{}.doh", self.base))
    }

    #[doc = "Get a reference to the value of field `dot` after provisioning.\n"]
    pub fn dot(&self) -> ZeroTrustDnsLocation_Endpoints_DotRef {
        ZeroTrustDnsLocation_Endpoints_DotRef::new(self.shared().clone(), format!("{}.dot", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4` after provisioning.\n"]
    pub fn ipv4(&self) -> ZeroTrustDnsLocation_Endpoints_Ipv4Ref {
        ZeroTrustDnsLocation_Endpoints_Ipv4Ref::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> ZeroTrustDnsLocation_Endpoints_Ipv6Ref {
        ZeroTrustDnsLocation_Endpoints_Ipv6Ref::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDnsLocation_MaxTtl {
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl_secs: std::option::Option<terrars::PrimField<f64>>,
}

impl ZeroTrustDnsLocation_MaxTtl {
    #[doc =
        "Set the field `ttl_secs`.\nLocation-specific cap on DNS response TTLs, in seconds. Required when `mode` is `override`. Must be omitted when `mode` is `inherit` or `disabled`."]
    pub fn set_ttl_secs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ttl_secs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDnsLocation_MaxTtl {
    type O = terrars::BlockAssignable<ZeroTrustDnsLocation_MaxTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDnsLocation_MaxTtl {
    #[doc =
        "`inherit` uses the account `max_ttl_secs`. `override` uses this location's `ttl_secs`. `disabled` leaves returned TTLs unchanged.\nAvailable values: \"inherit\", \"override\", \"disabled\"."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDnsLocation_MaxTtl {
    pub fn build(self) -> ZeroTrustDnsLocation_MaxTtl {
        ZeroTrustDnsLocation_MaxTtl {
            mode: self.mode,
            ttl_secs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDnsLocation_MaxTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocation_MaxTtlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDnsLocation_MaxTtlRef {
        ZeroTrustDnsLocation_MaxTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDnsLocation_MaxTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\n`inherit` uses the account `max_ttl_secs`. `override` uses this location's `ttl_secs`. `disabled` leaves returned TTLs unchanged.\nAvailable values: \"inherit\", \"override\", \"disabled\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl_secs` after provisioning.\nLocation-specific cap on DNS response TTLs, in seconds. Required when `mode` is `override`. Must be omitted when `mode` is `inherit` or `disabled`."]
    pub fn ttl_secs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl_secs", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDnsLocation_Networks {
    network: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDnsLocation_Networks { }

impl terrars::ToListMappable for ZeroTrustDnsLocation_Networks {
    type O = terrars::BlockAssignable<ZeroTrustDnsLocation_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDnsLocation_Networks {
    #[doc = "Specify the IPv4 address or IPv4 CIDR. Limit IPv4 CIDRs to a maximum of /24."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDnsLocation_Networks {
    pub fn build(self) -> ZeroTrustDnsLocation_Networks {
        ZeroTrustDnsLocation_Networks { network: self.network }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDnsLocation_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDnsLocation_NetworksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDnsLocation_NetworksRef {
        ZeroTrustDnsLocation_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDnsLocation_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nSpecify the IPv4 address or IPv4 CIDR. Limit IPv4 CIDRs to a maximum of /24."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}
