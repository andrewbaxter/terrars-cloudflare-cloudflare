#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicTransitSiteWanData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    site_id: terrars::PrimField<std::string::String>,
    wan_id: terrars::PrimField<std::string::String>,
}

struct DataMagicTransitSiteWan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicTransitSiteWanData_>,
}

#[derive(Clone)]
pub struct DataMagicTransitSiteWan(std::rc::Rc<DataMagicTransitSiteWan_>);

impl DataMagicTransitSiteWan {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_check_rate` after provisioning.\nMagic WAN health check rate for tunnels created on this link. The default value is `mid`.\nAvailable values: \"low\", \"mid\", \"high\"."]
    pub fn health_check_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_rate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balance_inner_flows` after provisioning.\n"]
    pub fn load_balance_inner_flows(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balance_inner_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `physport` after provisioning.\n"]
    pub fn physport(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physport", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nPriority of WAN for traffic loadbalancing."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `static_addressing` after provisioning.\n(optional) if omitted, use DHCP. Submit secondary_address when site is in high availability mode."]
    pub fn static_addressing(&self) -> DataMagicTransitSiteWan_StaticAddressingRef {
        DataMagicTransitSiteWan_StaticAddressingRef::new(
            self.shared().clone(),
            format!("{}.static_addressing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vlan_tag` after provisioning.\nVLAN ID. Use zero for untagged."]
    pub fn vlan_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wan_id` after provisioning.\nIdentifier"]
    pub fn wan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wan_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataMagicTransitSiteWan {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicTransitSiteWan { }

impl terrars::ToListMappable for DataMagicTransitSiteWan {
    type O = terrars::ListRef<DataMagicTransitSiteWanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicTransitSiteWan_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_site_wan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicTransitSiteWan {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Identifier"]
    pub site_id: terrars::PrimField<std::string::String>,
    #[doc = "Identifier"]
    pub wan_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMagicTransitSiteWan {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicTransitSiteWan {
        let out = DataMagicTransitSiteWan(std::rc::Rc::new(DataMagicTransitSiteWan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicTransitSiteWanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                site_id: self.site_id,
                wan_id: self.wan_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMagicTransitSiteWanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteWanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicTransitSiteWanRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_check_rate` after provisioning.\nMagic WAN health check rate for tunnels created on this link. The default value is `mid`.\nAvailable values: \"low\", \"mid\", \"high\"."]
    pub fn health_check_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_rate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balance_inner_flows` after provisioning.\n"]
    pub fn load_balance_inner_flows(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balance_inner_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `physport` after provisioning.\n"]
    pub fn physport(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physport", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nPriority of WAN for traffic loadbalancing."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `static_addressing` after provisioning.\n(optional) if omitted, use DHCP. Submit secondary_address when site is in high availability mode."]
    pub fn static_addressing(&self) -> DataMagicTransitSiteWan_StaticAddressingRef {
        DataMagicTransitSiteWan_StaticAddressingRef::new(
            self.shared().clone(),
            format!("{}.static_addressing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vlan_tag` after provisioning.\nVLAN ID. Use zero for untagged."]
    pub fn vlan_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wan_id` after provisioning.\nIdentifier"]
    pub fn wan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wan_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteWan_StaticAddressing {}

impl DataMagicTransitSiteWan_StaticAddressing { }

impl terrars::ToListMappable for DataMagicTransitSiteWan_StaticAddressing {
    type O = terrars::BlockAssignable<DataMagicTransitSiteWan_StaticAddressing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteWan_StaticAddressing {}

impl BuildDataMagicTransitSiteWan_StaticAddressing {
    pub fn build(self) -> DataMagicTransitSiteWan_StaticAddressing {
        DataMagicTransitSiteWan_StaticAddressing {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteWan_StaticAddressingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteWan_StaticAddressingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitSiteWan_StaticAddressingRef {
        DataMagicTransitSiteWan_StaticAddressingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteWan_StaticAddressingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_address` after provisioning.\nA valid IPv4 address."]
    pub fn gateway_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secondary_address` after provisioning.\nA valid CIDR notation representing an IP range."]
    pub fn secondary_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_address", self.base))
    }
}
