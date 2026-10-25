#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZoneData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataZone_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZone_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZoneData_>,
}

#[derive(Clone)]
pub struct DataZone(std::rc::Rc<DataZone_>);

impl DataZone {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataZone_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier"]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\nThe account the zone belongs to."]
    pub fn account(&self) -> DataZone_AccountRef {
        DataZone_AccountRef::new(self.shared().clone(), format!("{}.account", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZone_FilterRef {
        DataZone_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\nMetadata about the zone."]
    pub fn meta(&self) -> DataZone_MetaRef {
        DataZone_MetaRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
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
    pub fn owner(&self) -> DataZone_OwnerRef {
        DataZone_OwnerRef::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
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
    pub fn plan(&self) -> DataZone_PlanRef {
        DataZone_PlanRef::new(self.shared().clone(), format!("{}.plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe zone status on Cloudflare.\nAvailable values: \"initializing\", \"pending\", \"active\", \"moved\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant` after provisioning.\nThe root organizational unit that this zone belongs to (such as a tenant or organization)."]
    pub fn tenant(&self) -> DataZone_TenantRef {
        DataZone_TenantRef::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant_unit` after provisioning.\nThe immediate parent organizational unit that this zone belongs to (such as under a tenant or sub-organization)."]
    pub fn tenant_unit(&self) -> DataZone_TenantUnitRef {
        DataZone_TenantUnitRef::new(self.shared().clone(), format!("{}.tenant_unit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nA full zone implies that DNS is hosted with Cloudflare. A partial zone is\ntypically a partner-hosted zone or a CNAME setup.\nAvailable values: \"full\", \"partial\", \"secondary\", \"internal\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vanity_name_servers` after provisioning.\nAn array of domains used for custom name servers. This is only available for Business and Enterprise plans."]
    pub fn vanity_name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vanity_name_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `verification_key` after provisioning.\nVerification key for partial zone setup."]
    pub fn verification_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZone {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZone { }

impl terrars::ToListMappable for DataZone {
    type O = terrars::ListRef<DataZoneRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZone_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zone".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZone {
    pub tf_id: std::string::String,
}

impl BuildDataZone {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZone {
        let out = DataZone(std::rc::Rc::new(DataZone_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZoneData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZoneRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\nThe account the zone belongs to."]
    pub fn account(&self) -> DataZone_AccountRef {
        DataZone_AccountRef::new(self.shared().clone(), format!("{}.account", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZone_FilterRef {
        DataZone_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\nMetadata about the zone."]
    pub fn meta(&self) -> DataZone_MetaRef {
        DataZone_MetaRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
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
    pub fn owner(&self) -> DataZone_OwnerRef {
        DataZone_OwnerRef::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
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
    pub fn plan(&self) -> DataZone_PlanRef {
        DataZone_PlanRef::new(self.shared().clone(), format!("{}.plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe zone status on Cloudflare.\nAvailable values: \"initializing\", \"pending\", \"active\", \"moved\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant` after provisioning.\nThe root organizational unit that this zone belongs to (such as a tenant or organization)."]
    pub fn tenant(&self) -> DataZone_TenantRef {
        DataZone_TenantRef::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant_unit` after provisioning.\nThe immediate parent organizational unit that this zone belongs to (such as under a tenant or sub-organization)."]
    pub fn tenant_unit(&self) -> DataZone_TenantUnitRef {
        DataZone_TenantUnitRef::new(self.shared().clone(), format!("{}.tenant_unit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nA full zone implies that DNS is hosted with Cloudflare. A partial zone is\ntypically a partner-hosted zone or a CNAME setup.\nAvailable values: \"full\", \"partial\", \"secondary\", \"internal\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vanity_name_servers` after provisioning.\nAn array of domains used for custom name servers. This is only available for Business and Enterprise plans."]
    pub fn vanity_name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vanity_name_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `verification_key` after provisioning.\nVerification key for partial zone setup."]
    pub fn verification_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZone_Account {}

impl DataZone_Account { }

impl terrars::ToListMappable for DataZone_Account {
    type O = terrars::BlockAssignable<DataZone_Account>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZone_Account {}

impl BuildDataZone_Account {
    pub fn build(self) -> DataZone_Account {
        DataZone_Account {}
    }
}

#[allow(dead_code)]
pub struct DataZone_AccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZone_AccountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZone_AccountRef {
        DataZone_AccountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZone_AccountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the account."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZone_Filter_Account {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZone_Filter_Account {
    #[doc = "Set the field `id`.\nFilter by an account ID."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nAn account Name. Optional filter operators can be provided to extend refine the search:\n  * `equal` (default)\n  * `not_equal`\n  * `starts_with`\n  * `ends_with`\n  * `contains`\n  * `starts_with_case_sensitive`\n  * `ends_with_case_sensitive`\n  * `contains_case_sensitive`"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZone_Filter_Account {
    type O = terrars::BlockAssignable<DataZone_Filter_Account>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZone_Filter_Account {}

impl BuildDataZone_Filter_Account {
    pub fn build(self) -> DataZone_Filter_Account {
        DataZone_Filter_Account {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZone_Filter_AccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZone_Filter_AccountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZone_Filter_AccountRef {
        DataZone_Filter_AccountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZone_Filter_AccountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nFilter by an account ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nAn account Name. Optional filter operators can be provided to extend refine the search:\n  * `equal` (default)\n  * `not_equal`\n  * `starts_with`\n  * `ends_with`\n  * `contains`\n  * `starts_with_case_sensitive`\n  * `ends_with_case_sensitive`\n  * `contains_case_sensitive`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZone_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account: std::option::Option<DataZone_Filter_Account>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataZone_Filter {
    #[doc = "Set the field `account`.\n"]
    pub fn set_account(mut self, v: impl std::convert::Into<DataZone_Filter_Account>) -> Self {
        self.account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nDirection to order zones.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `match_`.\nWhether to match all search requirements or at least one (any).\nAvailable values: \"any\", \"all\"."]
    pub fn set_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nA domain name. Optional filter operators can be provided to extend refine the search:\n  * `equal` (default)\n  * `not_equal`\n  * `starts_with`\n  * `ends_with`\n  * `contains`\n  * `starts_with_case_sensitive`\n  * `ends_with_case_sensitive`\n  * `contains_case_sensitive`"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nField to order zones by.\nAvailable values: \"name\", \"status\", \"account.id\", \"account.name\", \"plan.id\"."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nSpecify a zone status to filter by.\nAvailable values: \"initializing\", \"pending\", \"active\", \"moved\"."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nZone types to filter by. Multiple types can be specified as a comma-separated list (e.g., ?type=full,partial,secondary). When this parameter is not provided, zones with type \"internal\" are excluded from the results."]
    pub fn set_type(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZone_Filter {
    type O = terrars::BlockAssignable<DataZone_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZone_Filter {}

impl BuildDataZone_Filter {
    pub fn build(self) -> DataZone_Filter {
        DataZone_Filter {
            account: core::default::Default::default(),
            direction: core::default::Default::default(),
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
            order: core::default::Default::default(),
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZone_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZone_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZone_FilterRef {
        DataZone_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZone_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> DataZone_Filter_AccountRef {
        DataZone_Filter_AccountRef::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order zones.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nWhether to match all search requirements or at least one (any).\nAvailable values: \"any\", \"all\"."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA domain name. Optional filter operators can be provided to extend refine the search:\n  * `equal` (default)\n  * `not_equal`\n  * `starts_with`\n  * `ends_with`\n  * `contains`\n  * `starts_with_case_sensitive`\n  * `ends_with_case_sensitive`\n  * `contains_case_sensitive`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order zones by.\nAvailable values: \"name\", \"status\", \"account.id\", \"account.name\", \"plan.id\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nSpecify a zone status to filter by.\nAvailable values: \"initializing\", \"pending\", \"active\", \"moved\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nZone types to filter by. Multiple types can be specified as a comma-separated list (e.g., ?type=full,partial,secondary). When this parameter is not provided, zones with type \"internal\" are excluded from the results."]
    pub fn type_(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZone_Meta {}

impl DataZone_Meta { }

impl terrars::ToListMappable for DataZone_Meta {
    type O = terrars::BlockAssignable<DataZone_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZone_Meta {}

impl BuildDataZone_Meta {
    pub fn build(self) -> DataZone_Meta {
        DataZone_Meta {}
    }
}

#[allow(dead_code)]
pub struct DataZone_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZone_MetaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZone_MetaRef {
        DataZone_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZone_MetaRef {
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
pub struct DataZone_Owner {}

impl DataZone_Owner { }

impl terrars::ToListMappable for DataZone_Owner {
    type O = terrars::BlockAssignable<DataZone_Owner>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZone_Owner {}

impl BuildDataZone_Owner {
    pub fn build(self) -> DataZone_Owner {
        DataZone_Owner {}
    }
}

#[allow(dead_code)]
pub struct DataZone_OwnerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZone_OwnerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZone_OwnerRef {
        DataZone_OwnerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZone_OwnerRef {
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
pub struct DataZone_Plan {}

impl DataZone_Plan { }

impl terrars::ToListMappable for DataZone_Plan {
    type O = terrars::BlockAssignable<DataZone_Plan>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZone_Plan {}

impl BuildDataZone_Plan {
    pub fn build(self) -> DataZone_Plan {
        DataZone_Plan {}
    }
}

#[allow(dead_code)]
pub struct DataZone_PlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZone_PlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZone_PlanRef {
        DataZone_PlanRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZone_PlanRef {
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
pub struct DataZone_Tenant {}

impl DataZone_Tenant { }

impl terrars::ToListMappable for DataZone_Tenant {
    type O = terrars::BlockAssignable<DataZone_Tenant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZone_Tenant {}

impl BuildDataZone_Tenant {
    pub fn build(self) -> DataZone_Tenant {
        DataZone_Tenant {}
    }
}

#[allow(dead_code)]
pub struct DataZone_TenantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZone_TenantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZone_TenantRef {
        DataZone_TenantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZone_TenantRef {
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
pub struct DataZone_TenantUnit {}

impl DataZone_TenantUnit { }

impl terrars::ToListMappable for DataZone_TenantUnit {
    type O = terrars::BlockAssignable<DataZone_TenantUnit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZone_TenantUnit {}

impl BuildDataZone_TenantUnit {
    pub fn build(self) -> DataZone_TenantUnit {
        DataZone_TenantUnit {}
    }
}

#[allow(dead_code)]
pub struct DataZone_TenantUnitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZone_TenantUnitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZone_TenantUnitRef {
        DataZone_TenantUnitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZone_TenantUnitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
