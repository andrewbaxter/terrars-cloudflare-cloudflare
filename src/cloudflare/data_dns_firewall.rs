#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataDnsFirewallData_ {
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
    dns_firewall_id: terrars::PrimField<std::string::String>,
}

struct DataDnsFirewall_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDnsFirewallData_>,
}

#[derive(Clone)]
pub struct DataDnsFirewall(std::rc::Rc<DataDnsFirewall_>);

impl DataDnsFirewall {
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

    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attack_mitigation` after provisioning.\nAttack mitigation settings"]
    pub fn attack_mitigation(&self) -> DataDnsFirewall_AttackMitigationRef {
        DataDnsFirewall_AttackMitigationRef::new(
            self.shared().clone(),
            format!("{}.attack_mitigation", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deprecate_any_requests` after provisioning.\nWhether to refuse to answer queries for the ANY type"]
    pub fn deprecate_any_requests(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecate_any_requests", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_firewall_id` after provisioning.\nIdentifier."]
    pub fn dns_firewall_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_firewall_id", self.extract_ref()))
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

impl terrars::Referable for DataDnsFirewall {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDnsFirewall { }

impl terrars::ToListMappable for DataDnsFirewall {
    type O = terrars::ListRef<DataDnsFirewallRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDnsFirewall_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_dns_firewall".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDnsFirewall {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub dns_firewall_id: terrars::PrimField<std::string::String>,
}

impl BuildDataDnsFirewall {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDnsFirewall {
        let out = DataDnsFirewall(std::rc::Rc::new(DataDnsFirewall_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDnsFirewallData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                dns_firewall_id: self.dns_firewall_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDnsFirewallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsFirewallRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDnsFirewallRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attack_mitigation` after provisioning.\nAttack mitigation settings"]
    pub fn attack_mitigation(&self) -> DataDnsFirewall_AttackMitigationRef {
        DataDnsFirewall_AttackMitigationRef::new(
            self.shared().clone(),
            format!("{}.attack_mitigation", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deprecate_any_requests` after provisioning.\nWhether to refuse to answer queries for the ANY type"]
    pub fn deprecate_any_requests(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecate_any_requests", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_firewall_id` after provisioning.\nIdentifier."]
    pub fn dns_firewall_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_firewall_id", self.extract_ref()))
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
pub struct DataDnsFirewall_AttackMitigation {}

impl DataDnsFirewall_AttackMitigation { }

impl terrars::ToListMappable for DataDnsFirewall_AttackMitigation {
    type O = terrars::BlockAssignable<DataDnsFirewall_AttackMitigation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsFirewall_AttackMitigation {}

impl BuildDataDnsFirewall_AttackMitigation {
    pub fn build(self) -> DataDnsFirewall_AttackMitigation {
        DataDnsFirewall_AttackMitigation {}
    }
}

#[allow(dead_code)]
pub struct DataDnsFirewall_AttackMitigationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsFirewall_AttackMitigationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsFirewall_AttackMitigationRef {
        DataDnsFirewall_AttackMitigationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsFirewall_AttackMitigationRef {
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
