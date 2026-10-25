#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustTunnelCloudflaredRouteData_ {
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
    filter: std::option::Option<DataZeroTrustTunnelCloudflaredRoute_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustTunnelCloudflaredRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustTunnelCloudflaredRouteData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustTunnelCloudflaredRoute(std::rc::Rc<DataZeroTrustTunnelCloudflaredRoute_>);

impl DataZeroTrustTunnelCloudflaredRoute {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataZeroTrustTunnelCloudflaredRoute_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_id`.\nUUID of the route."]
    pub fn set_route_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().route_id = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp of when the resource was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deleted_at` after provisioning.\nTimestamp of when the resource was deleted. If `null`, the resource has not been deleted."]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustTunnelCloudflaredRoute_FilterRef {
        DataZeroTrustTunnelCloudflaredRoute_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the route."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe private IPv4 or IPv6 range connected by the route, in CIDR notation."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_id` after provisioning.\nUUID of the route."]
    pub fn route_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_id", self.extract_ref()))
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

impl terrars::Referable for DataZeroTrustTunnelCloudflaredRoute {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustTunnelCloudflaredRoute { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredRoute {
    type O = terrars::ListRef<DataZeroTrustTunnelCloudflaredRouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustTunnelCloudflaredRoute_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_tunnel_cloudflared_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredRoute {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustTunnelCloudflaredRoute {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustTunnelCloudflaredRoute {
        let out = DataZeroTrustTunnelCloudflaredRoute(std::rc::Rc::new(DataZeroTrustTunnelCloudflaredRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustTunnelCloudflaredRouteData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                route_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustTunnelCloudflaredRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustTunnelCloudflaredRouteRef {
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
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp of when the resource was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deleted_at` after provisioning.\nTimestamp of when the resource was deleted. If `null`, the resource has not been deleted."]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustTunnelCloudflaredRoute_FilterRef {
        DataZeroTrustTunnelCloudflaredRoute_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the route."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe private IPv4 or IPv6 range connected by the route, in CIDR notation."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_id` after provisioning.\nUUID of the route."]
    pub fn route_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_id", self.extract_ref()))
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
pub struct DataZeroTrustTunnelCloudflaredRoute_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existed_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_deleted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_subset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_superset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tun_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustTunnelCloudflaredRoute_Filter {
    #[doc = "Set the field `comment`.\nOptional remark describing the route."]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `existed_at`.\nIf provided, include only resources that were created (and not deleted) before this time. URL encoded."]
    pub fn set_existed_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.existed_at = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_deleted`.\nIf `true`, only include deleted routes. If `false`, exclude deleted routes. If empty, all routes will be included."]
    pub fn set_is_deleted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_deleted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_subset`.\nIf set, only list routes that are contained within this IP range."]
    pub fn set_network_subset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_subset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_superset`.\nIf set, only list routes that contain this IP range."]
    pub fn set_network_superset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_superset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tun_types`.\nThe types of tunnels to filter by, separated by commas."]
    pub fn set_tun_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tun_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_id`.\nUUID of the tunnel."]
    pub fn set_tunnel_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tunnel_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_network_id`.\nUUID of the virtual network."]
    pub fn set_virtual_network_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.virtual_network_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredRoute_Filter {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelCloudflaredRoute_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredRoute_Filter {}

impl BuildDataZeroTrustTunnelCloudflaredRoute_Filter {
    pub fn build(self) -> DataZeroTrustTunnelCloudflaredRoute_Filter {
        DataZeroTrustTunnelCloudflaredRoute_Filter {
            comment: core::default::Default::default(),
            existed_at: core::default::Default::default(),
            is_deleted: core::default::Default::default(),
            network_subset: core::default::Default::default(),
            network_superset: core::default::Default::default(),
            tun_types: core::default::Default::default(),
            tunnel_id: core::default::Default::default(),
            virtual_network_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelCloudflaredRoute_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredRoute_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustTunnelCloudflaredRoute_FilterRef {
        DataZeroTrustTunnelCloudflaredRoute_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelCloudflaredRoute_FilterRef {
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
        "Get a reference to the value of field `existed_at` after provisioning.\nIf provided, include only resources that were created (and not deleted) before this time. URL encoded."]
    pub fn existed_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.existed_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_deleted` after provisioning.\nIf `true`, only include deleted routes. If `false`, exclude deleted routes. If empty, all routes will be included."]
    pub fn is_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_subset` after provisioning.\nIf set, only list routes that are contained within this IP range."]
    pub fn network_subset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_subset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_superset` after provisioning.\nIf set, only list routes that contain this IP range."]
    pub fn network_superset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_superset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tun_types` after provisioning.\nThe types of tunnels to filter by, separated by commas."]
    pub fn tun_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tun_types", self.base))
    }

    #[doc = "Get a reference to the value of field `tunnel_id` after provisioning.\nUUID of the tunnel."]
    pub fn tunnel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `virtual_network_id` after provisioning.\nUUID of the virtual network."]
    pub fn virtual_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_network_id", self.base))
    }
}
