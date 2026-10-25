#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicTransitSiteWansData_ {
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
    max_items: std::option::Option<terrars::PrimField<f64>>,
    site_id: terrars::PrimField<std::string::String>,
}

struct DataMagicTransitSiteWans_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicTransitSiteWansData_>,
}

#[derive(Clone)]
pub struct DataMagicTransitSiteWans(std::rc::Rc<DataMagicTransitSiteWans_>);

impl DataMagicTransitSiteWans {
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

    #[doc = "Set the field `account_id`.\nIdentifier"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataMagicTransitSiteWans_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataMagicTransitSiteWans {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicTransitSiteWans { }

impl terrars::ToListMappable for DataMagicTransitSiteWans {
    type O = terrars::ListRef<DataMagicTransitSiteWansRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicTransitSiteWans_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_site_wans".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicTransitSiteWans {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub site_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMagicTransitSiteWans {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicTransitSiteWans {
        let out = DataMagicTransitSiteWans(std::rc::Rc::new(DataMagicTransitSiteWans_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicTransitSiteWansData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                site_id: self.site_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMagicTransitSiteWansRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteWansRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicTransitSiteWansRef {
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

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataMagicTransitSiteWans_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitSiteWans_Result_StaticAddressing {}

impl DataMagicTransitSiteWans_Result_StaticAddressing { }

impl terrars::ToListMappable for DataMagicTransitSiteWans_Result_StaticAddressing {
    type O = terrars::BlockAssignable<DataMagicTransitSiteWans_Result_StaticAddressing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteWans_Result_StaticAddressing {}

impl BuildDataMagicTransitSiteWans_Result_StaticAddressing {
    pub fn build(self) -> DataMagicTransitSiteWans_Result_StaticAddressing {
        DataMagicTransitSiteWans_Result_StaticAddressing {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteWans_Result_StaticAddressingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteWans_Result_StaticAddressingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMagicTransitSiteWans_Result_StaticAddressingRef {
        DataMagicTransitSiteWans_Result_StaticAddressingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteWans_Result_StaticAddressingRef {
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

#[derive(Serialize)]
pub struct DataMagicTransitSiteWans_Result {}

impl DataMagicTransitSiteWans_Result { }

impl terrars::ToListMappable for DataMagicTransitSiteWans_Result {
    type O = terrars::BlockAssignable<DataMagicTransitSiteWans_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitSiteWans_Result {}

impl BuildDataMagicTransitSiteWans_Result {
    pub fn build(self) -> DataMagicTransitSiteWans_Result {
        DataMagicTransitSiteWans_Result {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitSiteWans_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitSiteWans_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitSiteWans_ResultRef {
        DataMagicTransitSiteWans_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitSiteWans_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `health_check_rate` after provisioning.\nMagic WAN health check rate for tunnels created on this link. The default value is `mid`.\nAvailable values: \"low\", \"mid\", \"high\"."]
    pub fn health_check_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balance_inner_flows` after provisioning.\n"]
    pub fn load_balance_inner_flows(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balance_inner_flows", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `physport` after provisioning.\n"]
    pub fn physport(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physport", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nPriority of WAN for traffic loadbalancing."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\nIdentifier"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `static_addressing` after provisioning.\n(optional) if omitted, use DHCP. Submit secondary_address when site is in high availability mode."]
    pub fn static_addressing(&self) -> DataMagicTransitSiteWans_Result_StaticAddressingRef {
        DataMagicTransitSiteWans_Result_StaticAddressingRef::new(
            self.shared().clone(),
            format!("{}.static_addressing", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vlan_tag` after provisioning.\nVLAN ID. Use zero for untagged."]
    pub fn vlan_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag", self.base))
    }
}
