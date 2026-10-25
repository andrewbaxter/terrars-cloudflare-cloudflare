#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDevicePostureIntegrationData_ {
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
    integration_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustDevicePostureIntegration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDevicePostureIntegrationData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDevicePostureIntegration(std::rc::Rc<DataZeroTrustDevicePostureIntegration_>);

impl DataZeroTrustDevicePostureIntegration {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe configuration object containing third-party integration information."]
    pub fn config(&self) -> DataZeroTrustDevicePostureIntegration_ConfigRef {
        DataZeroTrustDevicePostureIntegration_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAPI UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\nAPI UUID."]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
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

impl terrars::Referable for DataZeroTrustDevicePostureIntegration {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDevicePostureIntegration { }

impl terrars::ToListMappable for DataZeroTrustDevicePostureIntegration {
    type O = terrars::ListRef<DataZeroTrustDevicePostureIntegrationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDevicePostureIntegration_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_posture_integration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDevicePostureIntegration {
    pub tf_id: std::string::String,
    #[doc = "API UUID."]
    pub integration_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustDevicePostureIntegration {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDevicePostureIntegration {
        let out = DataZeroTrustDevicePostureIntegration(std::rc::Rc::new(DataZeroTrustDevicePostureIntegration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDevicePostureIntegrationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                integration_id: self.integration_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDevicePostureIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDevicePostureIntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDevicePostureIntegrationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe configuration object containing third-party integration information."]
    pub fn config(&self) -> DataZeroTrustDevicePostureIntegration_ConfigRef {
        DataZeroTrustDevicePostureIntegration_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAPI UUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\nAPI UUID."]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
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
pub struct DataZeroTrustDevicePostureIntegration_Config {}

impl DataZeroTrustDevicePostureIntegration_Config { }

impl terrars::ToListMappable for DataZeroTrustDevicePostureIntegration_Config {
    type O = terrars::BlockAssignable<DataZeroTrustDevicePostureIntegration_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDevicePostureIntegration_Config {}

impl BuildDataZeroTrustDevicePostureIntegration_Config {
    pub fn build(self) -> DataZeroTrustDevicePostureIntegration_Config {
        DataZeroTrustDevicePostureIntegration_Config {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDevicePostureIntegration_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDevicePostureIntegration_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDevicePostureIntegration_ConfigRef {
        DataZeroTrustDevicePostureIntegration_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDevicePostureIntegration_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
}
