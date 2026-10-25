#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustTunnelCloudflaredRoutesData_ {
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
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existed_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_deleted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_subset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_superset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tun_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustTunnelCloudflaredRoutes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustTunnelCloudflaredRoutesData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustTunnelCloudflaredRoutes(std::rc::Rc<DataZeroTrustTunnelCloudflaredRoutes_>);

impl DataZeroTrustTunnelCloudflaredRoutes {
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

    #[doc = "Set the field `account_id`.\nCloudflare account ID"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `comment`.\nOptional remark describing the route."]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `existed_at`.\nIf provided, include only resources that were created (and not deleted) before this time. URL encoded."]
    pub fn set_existed_at(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().existed_at = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_deleted`.\nIf `true`, only include deleted routes. If `false`, exclude deleted routes. If empty, all routes will be included."]
    pub fn set_is_deleted(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_deleted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_subset`.\nIf set, only list routes that are contained within this IP range."]
    pub fn set_network_subset(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_subset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_superset`.\nIf set, only list routes that contain this IP range."]
    pub fn set_network_superset(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_superset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_id`.\nUUID of the route."]
    pub fn set_route_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().route_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tun_types`.\nThe types of tunnels to filter by, separated by commas."]
    pub fn set_tun_types(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tun_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_id`.\nUUID of the tunnel."]
    pub fn set_tunnel_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tunnel_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_network_id`.\nUUID of the virtual network."]
    pub fn set_virtual_network_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().virtual_network_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nOptional remark describing the route."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `existed_at` after provisioning.\nIf provided, include only resources that were created (and not deleted) before this time. URL encoded."]
    pub fn existed_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.existed_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_deleted` after provisioning.\nIf `true`, only include deleted routes. If `false`, exclude deleted routes. If empty, all routes will be included."]
    pub fn is_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_subset` after provisioning.\nIf set, only list routes that are contained within this IP range."]
    pub fn network_subset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_subset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_superset` after provisioning.\nIf set, only list routes that contain this IP range."]
    pub fn network_superset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_superset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustTunnelCloudflaredRoutes_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_id` after provisioning.\nUUID of the route."]
    pub fn route_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tun_types` after provisioning.\nThe types of tunnels to filter by, separated by commas."]
    pub fn tun_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tun_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the tunnel."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\nUUID of the virtual network."]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustTunnelCloudflaredRoutes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustTunnelCloudflaredRoutes { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredRoutes {
    type O = terrars::ListRef<DataZeroTrustTunnelCloudflaredRoutesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustTunnelCloudflaredRoutes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_tunnel_cloudflared_routes".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredRoutes {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustTunnelCloudflaredRoutes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustTunnelCloudflaredRoutes {
        let out = DataZeroTrustTunnelCloudflaredRoutes(std::rc::Rc::new(DataZeroTrustTunnelCloudflaredRoutes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustTunnelCloudflaredRoutesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                comment: core::default::Default::default(),
                existed_at: core::default::Default::default(),
                is_deleted: core::default::Default::default(),
                max_items: core::default::Default::default(),
                network_subset: core::default::Default::default(),
                network_superset: core::default::Default::default(),
                route_id: core::default::Default::default(),
                tun_types: core::default::Default::default(),
                tunnel_id: core::default::Default::default(),
                virtual_network_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustTunnelCloudflaredRoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredRoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustTunnelCloudflaredRoutesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nOptional remark describing the route."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `existed_at` after provisioning.\nIf provided, include only resources that were created (and not deleted) before this time. URL encoded."]
    pub fn existed_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.existed_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_deleted` after provisioning.\nIf `true`, only include deleted routes. If `false`, exclude deleted routes. If empty, all routes will be included."]
    pub fn is_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_subset` after provisioning.\nIf set, only list routes that are contained within this IP range."]
    pub fn network_subset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_subset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_superset` after provisioning.\nIf set, only list routes that contain this IP range."]
    pub fn network_superset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_superset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustTunnelCloudflaredRoutes_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_id` after provisioning.\nUUID of the route."]
    pub fn route_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tun_types` after provisioning.\nThe types of tunnels to filter by, separated by commas."]
    pub fn tun_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tun_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the tunnel."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\nUUID of the virtual network."]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustTunnelCloudflaredRoutes_Result {}

impl DataZeroTrustTunnelCloudflaredRoutes_Result { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredRoutes_Result {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelCloudflaredRoutes_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredRoutes_Result {}

impl BuildDataZeroTrustTunnelCloudflaredRoutes_Result {
    pub fn build(self) -> DataZeroTrustTunnelCloudflaredRoutes_Result {
        DataZeroTrustTunnelCloudflaredRoutes_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelCloudflaredRoutes_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredRoutes_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelCloudflaredRoutes_ResultRef {
        DataZeroTrustTunnelCloudflaredRoutes_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelCloudflaredRoutes_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nOptional remark describing the route."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp of when the resource was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deleted_at` after provisioning.\nTimestamp of when the resource was deleted. If `null`, the resource has not been deleted."]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the route."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe private IPv4 or IPv6 range connected by the route, in CIDR notation."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tun_type` after provisioning.\nThe type of tunnel.\nAvailable values: \"cfd_tunnel\", \"warp_connector\", \"warp\", \"magic\", \"ip_sec\", \"gre\", \"cni\"."]
    pub fn tun_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tun_type", self.base))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the tunnel."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_name` after provisioning.\nA user-friendly name for a tunnel."]
    pub fn tunnel_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\nUUID of the virtual network."]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_name` after provisioning.\nA user-friendly name for the virtual network."]
    pub fn virtual_network_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_name", self.base))
    }
}
