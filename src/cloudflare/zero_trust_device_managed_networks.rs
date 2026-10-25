#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDeviceManagedNetworksData_ {
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
    config: ZeroTrustDeviceManagedNetworks_Config,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

struct ZeroTrustDeviceManagedNetworks_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDeviceManagedNetworksData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDeviceManagedNetworks(std::rc::Rc<ZeroTrustDeviceManagedNetworks_>);

impl ZeroTrustDeviceManagedNetworks {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe configuration object containing information for the WARP client to detect the managed network."]
    pub fn config(&self) -> ZeroTrustDeviceManagedNetworks_ConfigRef {
        ZeroTrustDeviceManagedNetworks_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAPI UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the device managed network. This name must be unique."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_id` after provisioning.\nAPI UUID."]
    pub fn network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of device managed network.\nAvailable values: \"tls\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustDeviceManagedNetworks {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDeviceManagedNetworks { }

impl terrars::ToListMappable for ZeroTrustDeviceManagedNetworks {
    type O = terrars::ListRef<ZeroTrustDeviceManagedNetworksRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDeviceManagedNetworks_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_managed_networks".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDeviceManagedNetworks {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "The configuration object containing information for the WARP client to detect the managed network."]
    pub config: ZeroTrustDeviceManagedNetworks_Config,
    #[doc = "The name of the device managed network. This name must be unique."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The type of device managed network.\nAvailable values: \"tls\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDeviceManagedNetworks {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDeviceManagedNetworks {
        let out = ZeroTrustDeviceManagedNetworks(std::rc::Rc::new(ZeroTrustDeviceManagedNetworks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDeviceManagedNetworksData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                config: self.config,
                name: self.name,
                type_: self.type_,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDeviceManagedNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceManagedNetworksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDeviceManagedNetworksRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe configuration object containing information for the WARP client to detect the managed network."]
    pub fn config(&self) -> ZeroTrustDeviceManagedNetworks_ConfigRef {
        ZeroTrustDeviceManagedNetworks_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAPI UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the device managed network. This name must be unique."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_id` after provisioning.\nAPI UUID."]
    pub fn network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of device managed network.\nAvailable values: \"tls\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDeviceManagedNetworks_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256: std::option::Option<terrars::PrimField<std::string::String>>,
    tls_sockaddr: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDeviceManagedNetworks_Config {
    #[doc =
        "Set the field `sha256`.\nThe SHA-256 hash of the TLS certificate presented by the host found at tls_sockaddr. If absent, regular certificate verification (trusted roots, valid timestamp, etc) will be used to validate the certificate."]
    pub fn set_sha256(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDeviceManagedNetworks_Config {
    type O = terrars::BlockAssignable<ZeroTrustDeviceManagedNetworks_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDeviceManagedNetworks_Config {
    #[doc =
        "A network address of the form \"host:port\" that the WARP client will use to detect the presence of a TLS host."]
    pub tls_sockaddr: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDeviceManagedNetworks_Config {
    pub fn build(self) -> ZeroTrustDeviceManagedNetworks_Config {
        ZeroTrustDeviceManagedNetworks_Config {
            sha256: core::default::Default::default(),
            tls_sockaddr: self.tls_sockaddr,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDeviceManagedNetworks_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceManagedNetworks_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDeviceManagedNetworks_ConfigRef {
        ZeroTrustDeviceManagedNetworks_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDeviceManagedNetworks_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sha256` after provisioning.\nThe SHA-256 hash of the TLS certificate presented by the host found at tls_sockaddr. If absent, regular certificate verification (trusted roots, valid timestamp, etc) will be used to validate the certificate."]
    pub fn sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_sockaddr` after provisioning.\nA network address of the form \"host:port\" that the WARP client will use to detect the presence of a TLS host."]
    pub fn tls_sockaddr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_sockaddr", self.base))
    }
}
