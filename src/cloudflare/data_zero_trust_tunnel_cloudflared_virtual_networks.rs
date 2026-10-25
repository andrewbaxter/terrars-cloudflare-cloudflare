#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustTunnelCloudflaredVirtualNetworksData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_default_network: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_deleted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustTunnelCloudflaredVirtualNetworks_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustTunnelCloudflaredVirtualNetworksData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustTunnelCloudflaredVirtualNetworks(
    std::rc::Rc<DataZeroTrustTunnelCloudflaredVirtualNetworks_>,
);

impl DataZeroTrustTunnelCloudflaredVirtualNetworks {
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

    #[doc = "Set the field `id`.\nUUID of the virtual network."]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_default`.\nIf `true`, only include the default virtual network. If `false`, exclude the default virtual network. If empty, all virtual networks will be included."]
    pub fn set_is_default(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_default = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_default_network`.\nIf `true`, only include the default virtual network. If `false`, exclude the default virtual network. If empty, all virtual networks will be included."]
    pub fn set_is_default_network(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_default_network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_deleted`.\nIf `true`, only include deleted virtual networks. If `false`, exclude deleted virtual networks. If empty, all virtual networks will be included."]
    pub fn set_is_deleted(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_deleted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nA user-friendly name for the virtual network."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the virtual network."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_default` after provisioning.\nIf `true`, only include the default virtual network. If `false`, exclude the default virtual network. If empty, all virtual networks will be included."]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_default_network` after provisioning.\nIf `true`, only include the default virtual network. If `false`, exclude the default virtual network. If empty, all virtual networks will be included."]
    pub fn is_default_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_deleted` after provisioning.\nIf `true`, only include deleted virtual networks. If `false`, exclude deleted virtual networks. If empty, all virtual networks will be included."]
    pub fn is_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-friendly name for the virtual network."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustTunnelCloudflaredVirtualNetworks_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustTunnelCloudflaredVirtualNetworks {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustTunnelCloudflaredVirtualNetworks { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredVirtualNetworks {
    type O = terrars::ListRef<DataZeroTrustTunnelCloudflaredVirtualNetworksRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustTunnelCloudflaredVirtualNetworks_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_tunnel_cloudflared_virtual_networks".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredVirtualNetworks {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustTunnelCloudflaredVirtualNetworks {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustTunnelCloudflaredVirtualNetworks {
        let out =
            DataZeroTrustTunnelCloudflaredVirtualNetworks(
                std::rc::Rc::new(DataZeroTrustTunnelCloudflaredVirtualNetworks_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataZeroTrustTunnelCloudflaredVirtualNetworksData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        account_id: core::default::Default::default(),
                        id: core::default::Default::default(),
                        is_default: core::default::Default::default(),
                        is_default_network: core::default::Default::default(),
                        is_deleted: core::default::Default::default(),
                        max_items: core::default::Default::default(),
                        name: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustTunnelCloudflaredVirtualNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredVirtualNetworksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustTunnelCloudflaredVirtualNetworksRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the virtual network."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_default` after provisioning.\nIf `true`, only include the default virtual network. If `false`, exclude the default virtual network. If empty, all virtual networks will be included."]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_default_network` after provisioning.\nIf `true`, only include the default virtual network. If `false`, exclude the default virtual network. If empty, all virtual networks will be included."]
    pub fn is_default_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_deleted` after provisioning.\nIf `true`, only include deleted virtual networks. If `false`, exclude deleted virtual networks. If empty, all virtual networks will be included."]
    pub fn is_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-friendly name for the virtual network."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustTunnelCloudflaredVirtualNetworks_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustTunnelCloudflaredVirtualNetworks_Result {}

impl DataZeroTrustTunnelCloudflaredVirtualNetworks_Result { }

impl terrars::ToListMappable for DataZeroTrustTunnelCloudflaredVirtualNetworks_Result {
    type O = terrars::BlockAssignable<DataZeroTrustTunnelCloudflaredVirtualNetworks_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustTunnelCloudflaredVirtualNetworks_Result {}

impl BuildDataZeroTrustTunnelCloudflaredVirtualNetworks_Result {
    pub fn build(self) -> DataZeroTrustTunnelCloudflaredVirtualNetworks_Result {
        DataZeroTrustTunnelCloudflaredVirtualNetworks_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustTunnelCloudflaredVirtualNetworks_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustTunnelCloudflaredVirtualNetworks_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustTunnelCloudflaredVirtualNetworks_ResultRef {
        DataZeroTrustTunnelCloudflaredVirtualNetworks_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustTunnelCloudflaredVirtualNetworks_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nOptional remark describing the virtual network."]
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID of the virtual network."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_default_network` after provisioning.\nIf `true`, this virtual network is the default for the account."]
    pub fn is_default_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-friendly name for the virtual network."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
