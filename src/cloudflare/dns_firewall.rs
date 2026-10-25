#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DnsFirewallData_ {
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
    attack_mitigation: std::option::Option<DnsFirewall_AttackMitigation>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deprecate_any_requests: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_firewall_ip_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_fallback: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_cache_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_cache_ttl: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negative_cache_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ratelimit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retries: std::option::Option<terrars::PrimField<f64>>,
    upstream_ips: terrars::SetField<terrars::PrimField<std::string::String>>,
}

struct DnsFirewall_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DnsFirewallData_>,
}

#[derive(Clone)]
pub struct DnsFirewall(std::rc::Rc<DnsFirewall_>);

impl DnsFirewall {
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

    #[doc = "Set the field `attack_mitigation`.\nAttack mitigation settings"]
    pub fn set_attack_mitigation(self, v: impl std::convert::Into<DnsFirewall_AttackMitigation>) -> Self {
        self.0.data.borrow_mut().attack_mitigation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deprecate_any_requests`.\nWhether to refuse to answer queries for the ANY type"]
    pub fn set_deprecate_any_requests(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deprecate_any_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dns_firewall_ip_count`.\nNumber of IPv4 addresses to assign to the DNS Firewall cluster. Only used during cluster creation and cannot be changed later."]
    pub fn set_dns_firewall_ip_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().dns_firewall_ip_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ecs_fallback`.\nWhether to forward client IP (resolver) subnet if no EDNS Client Subnet is sent"]
    pub fn set_ecs_fallback(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ecs_fallback = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `maximum_cache_ttl`.\nBy default, Cloudflare attempts to cache responses for as long as\nindicated by the TTL received from upstream nameservers. This setting\nsets an upper bound on this duration. For caching purposes, higher TTLs\nwill be decreased to the maximum value defined by this setting.\n\nThis setting does not affect the TTL value in the DNS response\nCloudflare returns to clients. Cloudflare will always forward the TTL\nvalue received from upstream nameservers."]
    pub fn set_maximum_cache_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_cache_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minimum_cache_ttl`.\nBy default, Cloudflare attempts to cache responses for as long as\nindicated by the TTL received from upstream nameservers. This setting\nsets a lower bound on this duration. For caching purposes, lower TTLs\nwill be increased to the minimum value defined by this setting.\n\nThis setting does not affect the TTL value in the DNS response\nCloudflare returns to clients. Cloudflare will always forward the TTL\nvalue received from upstream nameservers.\n\nNote that, even with this setting, there is no guarantee that a\nresponse will be cached for at least the specified duration. Cached\nresponses may be removed earlier for capacity or other operational\nreasons."]
    pub fn set_minimum_cache_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().minimum_cache_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `negative_cache_ttl`.\nThis setting controls how long DNS Firewall should cache negative\nresponses (e.g., NXDOMAIN) from the upstream servers.\n\nThis setting does not affect the TTL value in the DNS response\nCloudflare returns to clients. Cloudflare will always forward the TTL\nvalue received from upstream nameservers."]
    pub fn set_negative_cache_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().negative_cache_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ratelimit`.\nMaximum number of DNS queries per second that will be forwarded to your upstream nameservers. The limit is enforced per server, where each server receives a fraction of the configured value. The actual aggregate rate for a data center may vary depending on how many servers are present. Responses served from cache do not count toward this limit. Set to null to disable rate limiting."]
    pub fn set_ratelimit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ratelimit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retries`.\nNumber of retries for fetching DNS responses from upstream nameservers (not counting the initial attempt)"]
    pub fn set_retries(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attack_mitigation` after provisioning.\nAttack mitigation settings"]
    pub fn attack_mitigation(&self) -> DnsFirewall_AttackMitigationRef {
        DnsFirewall_AttackMitigationRef::new(
            self.shared().clone(),
            format!("{}.attack_mitigation", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deprecate_any_requests` after provisioning.\nWhether to refuse to answer queries for the ANY type"]
    pub fn deprecate_any_requests(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecate_any_requests", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_firewall_ip_count` after provisioning.\nNumber of IPv4 addresses to assign to the DNS Firewall cluster. Only used during cluster creation and cannot be changed later."]
    pub fn dns_firewall_ip_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_firewall_ip_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_firewall_ips` after provisioning.\n"]
    pub fn dns_firewall_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_firewall_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ecs_fallback` after provisioning.\nWhether to forward client IP (resolver) subnet if no EDNS Client Subnet is sent"]
    pub fn ecs_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_fallback", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maximum_cache_ttl` after provisioning.\nBy default, Cloudflare attempts to cache responses for as long as\nindicated by the TTL received from upstream nameservers. This setting\nsets an upper bound on this duration. For caching purposes, higher TTLs\nwill be decreased to the maximum value defined by this setting.\n\nThis setting does not affect the TTL value in the DNS response\nCloudflare returns to clients. Cloudflare will always forward the TTL\nvalue received from upstream nameservers."]
    pub fn maximum_cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_cache_ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `minimum_cache_ttl` after provisioning.\nBy default, Cloudflare attempts to cache responses for as long as\nindicated by the TTL received from upstream nameservers. This setting\nsets a lower bound on this duration. For caching purposes, lower TTLs\nwill be increased to the minimum value defined by this setting.\n\nThis setting does not affect the TTL value in the DNS response\nCloudflare returns to clients. Cloudflare will always forward the TTL\nvalue received from upstream nameservers.\n\nNote that, even with this setting, there is no guarantee that a\nresponse will be cached for at least the specified duration. Cached\nresponses may be removed earlier for capacity or other operational\nreasons."]
    pub fn minimum_cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_cache_ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nLast modification of DNS Firewall cluster"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDNS Firewall cluster name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `negative_cache_ttl` after provisioning.\nThis setting controls how long DNS Firewall should cache negative\nresponses (e.g., NXDOMAIN) from the upstream servers.\n\nThis setting does not affect the TTL value in the DNS response\nCloudflare returns to clients. Cloudflare will always forward the TTL\nvalue received from upstream nameservers."]
    pub fn negative_cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.negative_cache_ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ratelimit` after provisioning.\nMaximum number of DNS queries per second that will be forwarded to your upstream nameservers. The limit is enforced per server, where each server receives a fraction of the configured value. The actual aggregate rate for a data center may vary depending on how many servers are present. Responses served from cache do not count toward this limit. Set to null to disable rate limiting."]
    pub fn ratelimit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ratelimit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retries` after provisioning.\nNumber of retries for fetching DNS responses from upstream nameservers (not counting the initial attempt)"]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upstream_ips` after provisioning.\n"]
    pub fn upstream_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.upstream_ips", self.extract_ref()))
    }
}

impl terrars::Referable for DnsFirewall {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DnsFirewall { }

impl terrars::ToListMappable for DnsFirewall {
    type O = terrars::ListRef<DnsFirewallRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DnsFirewall_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_dns_firewall".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDnsFirewall {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "DNS Firewall cluster name"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub upstream_ips: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDnsFirewall {
    pub fn build(self, stack: &mut terrars::Stack) -> DnsFirewall {
        let out = DnsFirewall(std::rc::Rc::new(DnsFirewall_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DnsFirewallData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                attack_mitigation: core::default::Default::default(),
                deprecate_any_requests: core::default::Default::default(),
                dns_firewall_ip_count: core::default::Default::default(),
                ecs_fallback: core::default::Default::default(),
                maximum_cache_ttl: core::default::Default::default(),
                minimum_cache_ttl: core::default::Default::default(),
                name: self.name,
                negative_cache_ttl: core::default::Default::default(),
                ratelimit: core::default::Default::default(),
                retries: core::default::Default::default(),
                upstream_ips: self.upstream_ips,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DnsFirewallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsFirewallRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DnsFirewallRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attack_mitigation` after provisioning.\nAttack mitigation settings"]
    pub fn attack_mitigation(&self) -> DnsFirewall_AttackMitigationRef {
        DnsFirewall_AttackMitigationRef::new(
            self.shared().clone(),
            format!("{}.attack_mitigation", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deprecate_any_requests` after provisioning.\nWhether to refuse to answer queries for the ANY type"]
    pub fn deprecate_any_requests(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecate_any_requests", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_firewall_ip_count` after provisioning.\nNumber of IPv4 addresses to assign to the DNS Firewall cluster. Only used during cluster creation and cannot be changed later."]
    pub fn dns_firewall_ip_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_firewall_ip_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_firewall_ips` after provisioning.\n"]
    pub fn dns_firewall_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_firewall_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ecs_fallback` after provisioning.\nWhether to forward client IP (resolver) subnet if no EDNS Client Subnet is sent"]
    pub fn ecs_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_fallback", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maximum_cache_ttl` after provisioning.\nBy default, Cloudflare attempts to cache responses for as long as\nindicated by the TTL received from upstream nameservers. This setting\nsets an upper bound on this duration. For caching purposes, higher TTLs\nwill be decreased to the maximum value defined by this setting.\n\nThis setting does not affect the TTL value in the DNS response\nCloudflare returns to clients. Cloudflare will always forward the TTL\nvalue received from upstream nameservers."]
    pub fn maximum_cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_cache_ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `minimum_cache_ttl` after provisioning.\nBy default, Cloudflare attempts to cache responses for as long as\nindicated by the TTL received from upstream nameservers. This setting\nsets a lower bound on this duration. For caching purposes, lower TTLs\nwill be increased to the minimum value defined by this setting.\n\nThis setting does not affect the TTL value in the DNS response\nCloudflare returns to clients. Cloudflare will always forward the TTL\nvalue received from upstream nameservers.\n\nNote that, even with this setting, there is no guarantee that a\nresponse will be cached for at least the specified duration. Cached\nresponses may be removed earlier for capacity or other operational\nreasons."]
    pub fn minimum_cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_cache_ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nLast modification of DNS Firewall cluster"]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDNS Firewall cluster name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `negative_cache_ttl` after provisioning.\nThis setting controls how long DNS Firewall should cache negative\nresponses (e.g., NXDOMAIN) from the upstream servers.\n\nThis setting does not affect the TTL value in the DNS response\nCloudflare returns to clients. Cloudflare will always forward the TTL\nvalue received from upstream nameservers."]
    pub fn negative_cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.negative_cache_ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ratelimit` after provisioning.\nMaximum number of DNS queries per second that will be forwarded to your upstream nameservers. The limit is enforced per server, where each server receives a fraction of the configured value. The actual aggregate rate for a data center may vary depending on how many servers are present. Responses served from cache do not count toward this limit. Set to null to disable rate limiting."]
    pub fn ratelimit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ratelimit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retries` after provisioning.\nNumber of retries for fetching DNS responses from upstream nameservers (not counting the initial attempt)"]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upstream_ips` after provisioning.\n"]
    pub fn upstream_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.upstream_ips", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DnsFirewall_AttackMitigation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    only_when_upstream_unhealthy: std::option::Option<terrars::PrimField<bool>>,
}

impl DnsFirewall_AttackMitigation {
    #[doc =
        "Set the field `enabled`.\nWhen enabled, automatically mitigate random-prefix attacks to protect upstream DNS servers"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `only_when_upstream_unhealthy`.\nOnly mitigate attacks when upstream servers seem unhealthy"]
    pub fn set_only_when_upstream_unhealthy(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.only_when_upstream_unhealthy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsFirewall_AttackMitigation {
    type O = terrars::BlockAssignable<DnsFirewall_AttackMitigation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsFirewall_AttackMitigation {}

impl BuildDnsFirewall_AttackMitigation {
    pub fn build(self) -> DnsFirewall_AttackMitigation {
        DnsFirewall_AttackMitigation {
            enabled: core::default::Default::default(),
            only_when_upstream_unhealthy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsFirewall_AttackMitigationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsFirewall_AttackMitigationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsFirewall_AttackMitigationRef {
        DnsFirewall_AttackMitigationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsFirewall_AttackMitigationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhen enabled, automatically mitigate random-prefix attacks to protect upstream DNS servers"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `only_when_upstream_unhealthy` after provisioning.\nOnly mitigate attacks when upstream servers seem unhealthy"]
    pub fn only_when_upstream_unhealthy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.only_when_upstream_unhealthy", self.base))
    }
}
