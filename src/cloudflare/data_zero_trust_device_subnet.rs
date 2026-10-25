#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDeviceSubnetData_ {
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
    subnet_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustDeviceSubnet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDeviceSubnetData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDeviceSubnet(std::rc::Rc<DataZeroTrustDeviceSubnet_>);

impl DataZeroTrustDeviceSubnet {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `capacity` after provisioning.\nIP capacity information for the subnet."]
    pub fn capacity(&self) -> DataZeroTrustDeviceSubnet_CapacityRef {
        DataZeroTrustDeviceSubnet_CapacityRef::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nAn optional description of the subnet."]
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe UUID of the subnet."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_default_network` after provisioning.\nIf `true`, this is the default subnet for the account. There can only be one default subnet per account."]
    pub fn is_default_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default_network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-friendly name for the subnet."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe private IPv4 or IPv6 range defining the subnet, in CIDR notation."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\nThe UUID of the subnet."]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnet_type` after provisioning.\nThe type of subnet.\nAvailable values: \"cloudflare_source\", \"initial_resolved_ip\", \"warp\"."]
    pub fn subnet_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_type", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustDeviceSubnet {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDeviceSubnet { }

impl terrars::ToListMappable for DataZeroTrustDeviceSubnet {
    type O = terrars::ListRef<DataZeroTrustDeviceSubnetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDeviceSubnet_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_subnet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDeviceSubnet {
    pub tf_id: std::string::String,
    #[doc = "The UUID of the subnet."]
    pub subnet_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustDeviceSubnet {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDeviceSubnet {
        let out = DataZeroTrustDeviceSubnet(std::rc::Rc::new(DataZeroTrustDeviceSubnet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDeviceSubnetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                subnet_id: self.subnet_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDeviceSubnetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceSubnetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDeviceSubnetRef {
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
        "Get a reference to the value of field `capacity` after provisioning.\nIP capacity information for the subnet."]
    pub fn capacity(&self) -> DataZeroTrustDeviceSubnet_CapacityRef {
        DataZeroTrustDeviceSubnet_CapacityRef::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nAn optional description of the subnet."]
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe UUID of the subnet."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_default_network` after provisioning.\nIf `true`, this is the default subnet for the account. There can only be one default subnet per account."]
    pub fn is_default_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default_network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA user-friendly name for the subnet."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe private IPv4 or IPv6 range defining the subnet, in CIDR notation."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\nThe UUID of the subnet."]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnet_type` after provisioning.\nThe type of subnet.\nAvailable values: \"cloudflare_source\", \"initial_resolved_ip\", \"warp\"."]
    pub fn subnet_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDeviceSubnet_Capacity {}

impl DataZeroTrustDeviceSubnet_Capacity { }

impl terrars::ToListMappable for DataZeroTrustDeviceSubnet_Capacity {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceSubnet_Capacity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceSubnet_Capacity {}

impl BuildDataZeroTrustDeviceSubnet_Capacity {
    pub fn build(self) -> DataZeroTrustDeviceSubnet_Capacity {
        DataZeroTrustDeviceSubnet_Capacity {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceSubnet_CapacityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceSubnet_CapacityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDeviceSubnet_CapacityRef {
        DataZeroTrustDeviceSubnet_CapacityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceSubnet_CapacityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `total` after provisioning.\nTotal number of assignable IPs in the subnet."]
    pub fn total(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total", self.base))
    }

    #[doc = "Get a reference to the value of field `used` after provisioning.\nNumber of assigned IPs in the subnet."]
    pub fn used(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.used", self.base))
    }
}
