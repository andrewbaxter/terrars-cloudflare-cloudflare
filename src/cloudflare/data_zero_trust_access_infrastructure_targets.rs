#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustAccessInfrastructureTargetsData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_like: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_v4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_v6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modified_after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modified_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustAccessInfrastructureTargets_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustAccessInfrastructureTargetsData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustAccessInfrastructureTargets(std::rc::Rc<DataZeroTrustAccessInfrastructureTargets_>);

impl DataZeroTrustAccessInfrastructureTargets {
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

    #[doc = "Set the field `account_id`.\nAccount identifier"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_after`.\nDate and time at which the target was created after (inclusive)"]
    pub fn set_created_after(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().created_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_before`.\nDate and time at which the target was created before (inclusive)"]
    pub fn set_created_before(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().created_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nHostname of a target"]
    pub fn set_hostname(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname_contains`.\nPartial match to the hostname of a target"]
    pub fn set_hostname_contains(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hostname_contains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_like`.\nFilters for targets whose IP addresses look like the specified string.\nSupports `*` as a wildcard character"]
    pub fn set_ip_like(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_like = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_v4`.\nIPv4 address of the target"]
    pub fn set_ip_v4(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_v4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_v6`.\nIPv6 address of the target"]
    pub fn set_ip_v6(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_v6 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ips`.\nFilters for targets that have any of the following IP addresses. Specify\n`ips` multiple times in query parameter to build list of candidates."]
    pub fn set_ips(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv4_end`.\nDefines an IPv4 filter range's ending value (inclusive). Requires\n`ipv4_start` to be specified as well."]
    pub fn set_ipv4_end(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipv4_end = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv4_start`.\nDefines an IPv4 filter range's starting value (inclusive). Requires\n`ipv4_end` to be specified as well."]
    pub fn set_ipv4_start(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipv4_start = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv6_end`.\nDefines an IPv6 filter range's ending value (inclusive). Requires\n`ipv6_start` to be specified as well."]
    pub fn set_ipv6_end(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipv6_end = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv6_start`.\nDefines an IPv6 filter range's starting value (inclusive). Requires\n`ipv6_end` to be specified as well."]
    pub fn set_ipv6_start(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipv6_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `modified_after`.\nDate and time at which the target was modified after (inclusive)"]
    pub fn set_modified_after(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().modified_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `modified_before`.\nDate and time at which the target was modified before (inclusive)"]
    pub fn set_modified_before(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().modified_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\nThe field to sort by.\nAvailable values: \"hostname\", \"created_at\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nFilter by tag key:value pairs. Multiple `tag` params are AND'd.\nFormat: `tag=key:value` (e.g., `tag=environment:production`).\nKey and value must both be non-empty; `tag=:value` and `tag=key:` return 400."]
    pub fn set_tag(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_ids`.\nFilters for targets that have any of the following UUIDs. Specify\n`target_ids` multiple times in query parameter to build list of\ncandidates."]
    pub fn set_target_ids(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().target_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_network_id`.\nPrivate virtual network identifier of the target"]
    pub fn set_virtual_network_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().virtual_network_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_after` after provisioning.\nDate and time at which the target was created after (inclusive)"]
    pub fn created_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_before` after provisioning.\nDate and time at which the target was created before (inclusive)"]
    pub fn created_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname of a target"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_contains` after provisioning.\nPartial match to the hostname of a target"]
    pub fn hostname_contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_contains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_like` after provisioning.\nFilters for targets whose IP addresses look like the specified string.\nSupports `*` as a wildcard character"]
    pub fn ip_like(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_like", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_v4` after provisioning.\nIPv4 address of the target"]
    pub fn ip_v4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_v4", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_v6` after provisioning.\nIPv6 address of the target"]
    pub fn ip_v6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_v6", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ips` after provisioning.\nFilters for targets that have any of the following IP addresses. Specify\n`ips` multiple times in query parameter to build list of candidates."]
    pub fn ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_end` after provisioning.\nDefines an IPv4 filter range's ending value (inclusive). Requires\n`ipv4_start` to be specified as well."]
    pub fn ipv4_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_start` after provisioning.\nDefines an IPv4 filter range's starting value (inclusive). Requires\n`ipv4_end` to be specified as well."]
    pub fn ipv4_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_end` after provisioning.\nDefines an IPv6 filter range's ending value (inclusive). Requires\n`ipv6_start` to be specified as well."]
    pub fn ipv6_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_start` after provisioning.\nDefines an IPv6 filter range's starting value (inclusive). Requires\n`ipv6_end` to be specified as well."]
    pub fn ipv6_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_after` after provisioning.\nDate and time at which the target was modified after (inclusive)"]
    pub fn modified_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_after", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_before` after provisioning.\nDate and time at which the target was modified before (inclusive)"]
    pub fn modified_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_before", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nThe field to sort by.\nAvailable values: \"hostname\", \"created_at\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustAccessInfrastructureTargets_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nFilter by tag key:value pairs. Multiple `tag` params are AND'd.\nFormat: `tag=key:value` (e.g., `tag=environment:production`).\nKey and value must both be non-empty; `tag=:value` and `tag=key:` return 400."]
    pub fn tag(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_ids` after provisioning.\nFilters for targets that have any of the following UUIDs. Specify\n`target_ids` multiple times in query parameter to build list of\ncandidates."]
    pub fn target_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\nPrivate virtual network identifier of the target"]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustAccessInfrastructureTargets {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustAccessInfrastructureTargets { }

impl terrars::ToListMappable for DataZeroTrustAccessInfrastructureTargets {
    type O = terrars::ListRef<DataZeroTrustAccessInfrastructureTargetsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustAccessInfrastructureTargets_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_infrastructure_targets".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustAccessInfrastructureTargets {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustAccessInfrastructureTargets {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustAccessInfrastructureTargets {
        let out =
            DataZeroTrustAccessInfrastructureTargets(std::rc::Rc::new(DataZeroTrustAccessInfrastructureTargets_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataZeroTrustAccessInfrastructureTargetsData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    account_id: core::default::Default::default(),
                    created_after: core::default::Default::default(),
                    created_before: core::default::Default::default(),
                    direction: core::default::Default::default(),
                    hostname: core::default::Default::default(),
                    hostname_contains: core::default::Default::default(),
                    ip_like: core::default::Default::default(),
                    ip_v4: core::default::Default::default(),
                    ip_v6: core::default::Default::default(),
                    ips: core::default::Default::default(),
                    ipv4_end: core::default::Default::default(),
                    ipv4_start: core::default::Default::default(),
                    ipv6_end: core::default::Default::default(),
                    ipv6_start: core::default::Default::default(),
                    max_items: core::default::Default::default(),
                    modified_after: core::default::Default::default(),
                    modified_before: core::default::Default::default(),
                    order: core::default::Default::default(),
                    tag: core::default::Default::default(),
                    target_ids: core::default::Default::default(),
                    virtual_network_id: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustAccessInfrastructureTargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessInfrastructureTargetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustAccessInfrastructureTargetsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_after` after provisioning.\nDate and time at which the target was created after (inclusive)"]
    pub fn created_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_before` after provisioning.\nDate and time at which the target was created before (inclusive)"]
    pub fn created_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname of a target"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_contains` after provisioning.\nPartial match to the hostname of a target"]
    pub fn hostname_contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_contains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_like` after provisioning.\nFilters for targets whose IP addresses look like the specified string.\nSupports `*` as a wildcard character"]
    pub fn ip_like(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_like", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_v4` after provisioning.\nIPv4 address of the target"]
    pub fn ip_v4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_v4", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_v6` after provisioning.\nIPv6 address of the target"]
    pub fn ip_v6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_v6", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ips` after provisioning.\nFilters for targets that have any of the following IP addresses. Specify\n`ips` multiple times in query parameter to build list of candidates."]
    pub fn ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_end` after provisioning.\nDefines an IPv4 filter range's ending value (inclusive). Requires\n`ipv4_start` to be specified as well."]
    pub fn ipv4_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_start` after provisioning.\nDefines an IPv4 filter range's starting value (inclusive). Requires\n`ipv4_end` to be specified as well."]
    pub fn ipv4_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_end` after provisioning.\nDefines an IPv6 filter range's ending value (inclusive). Requires\n`ipv6_start` to be specified as well."]
    pub fn ipv6_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_start` after provisioning.\nDefines an IPv6 filter range's starting value (inclusive). Requires\n`ipv6_end` to be specified as well."]
    pub fn ipv6_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_after` after provisioning.\nDate and time at which the target was modified after (inclusive)"]
    pub fn modified_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_after", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_before` after provisioning.\nDate and time at which the target was modified before (inclusive)"]
    pub fn modified_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_before", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nThe field to sort by.\nAvailable values: \"hostname\", \"created_at\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustAccessInfrastructureTargets_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nFilter by tag key:value pairs. Multiple `tag` params are AND'd.\nFormat: `tag=key:value` (e.g., `tag=environment:production`).\nKey and value must both be non-empty; `tag=:value` and `tag=key:` return 400."]
    pub fn tag(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_ids` after provisioning.\nFilters for targets that have any of the following UUIDs. Specify\n`target_ids` multiple times in query parameter to build list of\ncandidates."]
    pub fn target_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\nPrivate virtual network identifier of the target"]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4 {}

impl DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4 { }

impl terrars::ToListMappable for DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4 {
    type O = terrars::BlockAssignable<DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4 {}

impl BuildDataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4 {
    pub fn build(self) -> DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4 {
        DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4 {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4Ref {
        DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4Ref {
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
pub struct DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6 {}

impl DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6 { }

impl terrars::ToListMappable for DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6 {
    type O = terrars::BlockAssignable<DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6 {}

impl BuildDataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6 {
    pub fn build(self) -> DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6 {
        DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6 {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6Ref {
        DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6Ref {
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
pub struct DataZeroTrustAccessInfrastructureTargets_Result_Ip {}

impl DataZeroTrustAccessInfrastructureTargets_Result_Ip { }

impl terrars::ToListMappable for DataZeroTrustAccessInfrastructureTargets_Result_Ip {
    type O = terrars::BlockAssignable<DataZeroTrustAccessInfrastructureTargets_Result_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessInfrastructureTargets_Result_Ip {}

impl BuildDataZeroTrustAccessInfrastructureTargets_Result_Ip {
    pub fn build(self) -> DataZeroTrustAccessInfrastructureTargets_Result_Ip {
        DataZeroTrustAccessInfrastructureTargets_Result_Ip {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessInfrastructureTargets_Result_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessInfrastructureTargets_Result_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessInfrastructureTargets_Result_IpRef {
        DataZeroTrustAccessInfrastructureTargets_Result_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessInfrastructureTargets_Result_IpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4` after provisioning.\nThe target's IPv4 address"]
    pub fn ipv4(&self) -> DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4Ref {
        DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv4Ref::new(
            self.shared().clone(),
            format!("{}.ipv4", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\nThe target's IPv6 address"]
    pub fn ipv6(&self) -> DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6Ref {
        DataZeroTrustAccessInfrastructureTargets_Result_Ip_Ipv6Ref::new(
            self.shared().clone(),
            format!("{}.ipv6", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessInfrastructureTargets_Result {}

impl DataZeroTrustAccessInfrastructureTargets_Result { }

impl terrars::ToListMappable for DataZeroTrustAccessInfrastructureTargets_Result {
    type O = terrars::BlockAssignable<DataZeroTrustAccessInfrastructureTargets_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessInfrastructureTargets_Result {}

impl BuildDataZeroTrustAccessInfrastructureTargets_Result {
    pub fn build(self) -> DataZeroTrustAccessInfrastructureTargets_Result {
        DataZeroTrustAccessInfrastructureTargets_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessInfrastructureTargets_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessInfrastructureTargets_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessInfrastructureTargets_ResultRef {
        DataZeroTrustAccessInfrastructureTargets_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessInfrastructureTargets_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nDate and time at which the target was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nA non-unique field that refers to a target"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nTarget identifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nThe IPv4/IPv6 address that identifies where to reach a target"]
    pub fn ip(&self) -> DataZeroTrustAccessInfrastructureTargets_Result_IpRef {
        DataZeroTrustAccessInfrastructureTargets_Result_IpRef::new(
            self.shared().clone(),
            format!("{}.ip", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `modified_at` after provisioning.\nDate and time at which the target was modified"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nTags assigned to the target. Empty when no tags are assigned."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
