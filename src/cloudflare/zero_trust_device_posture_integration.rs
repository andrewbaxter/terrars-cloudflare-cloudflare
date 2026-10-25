#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDevicePostureIntegrationData_ {
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
    config: ZeroTrustDevicePostureIntegration_Config,
    interval: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

struct ZeroTrustDevicePostureIntegration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDevicePostureIntegrationData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDevicePostureIntegration(std::rc::Rc<ZeroTrustDevicePostureIntegration_>);

impl ZeroTrustDevicePostureIntegration {
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
        "Get a reference to the value of field `config` after provisioning.\nThe configuration object containing third-party integration information."]
    pub fn config(&self) -> ZeroTrustDevicePostureIntegration_ConfigRef {
        ZeroTrustDevicePostureIntegration_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAPI UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nThe interval between each posture check with the third-party API. Use `m` for minutes (e.g. `5m`) and `h` for hours (e.g. `12h`)."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the device posture integration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of device posture integration.\nAvailable values: \"workspace_one\", \"crowdstrike_s2s\", \"uptycs\", \"intune\", \"kolide\", \"tanium_s2s\", \"sentinelone_s2s\", \"custom_s2s\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustDevicePostureIntegration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDevicePostureIntegration { }

impl terrars::ToListMappable for ZeroTrustDevicePostureIntegration {
    type O = terrars::ListRef<ZeroTrustDevicePostureIntegrationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDevicePostureIntegration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_posture_integration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDevicePostureIntegration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "The configuration object containing third-party integration information."]
    pub config: ZeroTrustDevicePostureIntegration_Config,
    #[doc =
        "The interval between each posture check with the third-party API. Use `m` for minutes (e.g. `5m`) and `h` for hours (e.g. `12h`)."]
    pub interval: terrars::PrimField<std::string::String>,
    #[doc = "The name of the device posture integration."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of device posture integration.\nAvailable values: \"workspace_one\", \"crowdstrike_s2s\", \"uptycs\", \"intune\", \"kolide\", \"tanium_s2s\", \"sentinelone_s2s\", \"custom_s2s\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDevicePostureIntegration {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDevicePostureIntegration {
        let out = ZeroTrustDevicePostureIntegration(std::rc::Rc::new(ZeroTrustDevicePostureIntegration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDevicePostureIntegrationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                config: self.config,
                interval: self.interval,
                name: self.name,
                type_: self.type_,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDevicePostureIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDevicePostureIntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDevicePostureIntegrationRef {
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
        "Get a reference to the value of field `config` after provisioning.\nThe configuration object containing third-party integration information."]
    pub fn config(&self) -> ZeroTrustDevicePostureIntegration_ConfigRef {
        ZeroTrustDevicePostureIntegration_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAPI UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nThe interval between each posture check with the third-party API. Use `m` for minutes (e.g. `5m`) and `h` for hours (e.g. `12h`)."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the device posture integration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of device posture integration.\nAvailable values: \"workspace_one\", \"crowdstrike_s2s\", \"uptycs\", \"intune\", \"kolide\", \"tanium_s2s\", \"sentinelone_s2s\", \"custom_s2s\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDevicePostureIntegration_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustDevicePostureIntegration_Config {
    #[doc =
        "Set the field `access_client_id`.\nIf present, this id will be passed in the `CF-Access-Client-ID` header when hitting the `api_url`."]
    pub fn set_access_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `access_client_secret`.\nIf present, this secret will be passed in the `CF-Access-Client-Secret` header when hitting the `api_url`."]
    pub fn set_access_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.access_client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_url`.\nThe Workspace One API URL provided in the Workspace One Admin Dashboard."]
    pub fn set_api_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_url`.\nThe Workspace One Authorization URL depending on your region."]
    pub fn set_auth_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\nThe Workspace One client ID provided in the Workspace One Admin Dashboard."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_key`.\nThe Uptycs client secret."]
    pub fn set_client_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_secret`.\nThe Workspace One client secret provided in the Workspace One Admin Dashboard."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_id`.\nThe Crowdstrike customer ID."]
    pub fn set_customer_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.customer_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDevicePostureIntegration_Config {
    type O = terrars::BlockAssignable<ZeroTrustDevicePostureIntegration_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDevicePostureIntegration_Config {}

impl BuildZeroTrustDevicePostureIntegration_Config {
    pub fn build(self) -> ZeroTrustDevicePostureIntegration_Config {
        ZeroTrustDevicePostureIntegration_Config {
            access_client_id: core::default::Default::default(),
            access_client_secret: core::default::Default::default(),
            api_url: core::default::Default::default(),
            auth_url: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_key: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            customer_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDevicePostureIntegration_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDevicePostureIntegration_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDevicePostureIntegration_ConfigRef {
        ZeroTrustDevicePostureIntegration_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDevicePostureIntegration_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_client_id` after provisioning.\nIf present, this id will be passed in the `CF-Access-Client-ID` header when hitting the `api_url`."]
    pub fn access_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `access_client_secret` after provisioning.\nIf present, this secret will be passed in the `CF-Access-Client-Secret` header when hitting the `api_url`."]
    pub fn access_client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `api_url` after provisioning.\nThe Workspace One API URL provided in the Workspace One Admin Dashboard."]
    pub fn api_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_url` after provisioning.\nThe Workspace One Authorization URL depending on your region."]
    pub fn auth_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe Workspace One client ID provided in the Workspace One Admin Dashboard."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_key` after provisioning.\nThe Uptycs client secret."]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nThe Workspace One client secret provided in the Workspace One Admin Dashboard."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_id` after provisioning.\nThe Crowdstrike customer ID."]
    pub fn customer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_id", self.base))
    }
}
