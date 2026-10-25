#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDeviceDeploymentGroupsListData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
}

struct DataZeroTrustDeviceDeploymentGroupsList_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDeviceDeploymentGroupsListData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDeviceDeploymentGroupsList(std::rc::Rc<DataZeroTrustDeviceDeploymentGroupsList_>);

impl DataZeroTrustDeviceDeploymentGroupsList {
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

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustDeviceDeploymentGroupsList_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustDeviceDeploymentGroupsList {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDeviceDeploymentGroupsList { }

impl terrars::ToListMappable for DataZeroTrustDeviceDeploymentGroupsList {
    type O = terrars::ListRef<DataZeroTrustDeviceDeploymentGroupsListRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDeviceDeploymentGroupsList_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_deployment_groups_list".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDeviceDeploymentGroupsList {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustDeviceDeploymentGroupsList {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDeviceDeploymentGroupsList {
        let out = DataZeroTrustDeviceDeploymentGroupsList(std::rc::Rc::new(DataZeroTrustDeviceDeploymentGroupsList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDeviceDeploymentGroupsListData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                max_items: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDeviceDeploymentGroupsListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceDeploymentGroupsListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDeviceDeploymentGroupsListRef {
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

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustDeviceDeploymentGroupsList_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfig {}

impl DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfig { }

impl terrars::ToListMappable for DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfig {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfig {}

impl BuildDataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfig {
    pub fn build(self) -> DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfig {
        DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfig {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfigRef {
        DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_environment` after provisioning.\nThe target environment for the client version (e.g., windows, macos)."]
    pub fn target_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_environment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe specific client version to deploy."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDeviceDeploymentGroupsList_Result {}

impl DataZeroTrustDeviceDeploymentGroupsList_Result { }

impl terrars::ToListMappable for DataZeroTrustDeviceDeploymentGroupsList_Result {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceDeploymentGroupsList_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceDeploymentGroupsList_Result {}

impl BuildDataZeroTrustDeviceDeploymentGroupsList_Result {
    pub fn build(self) -> DataZeroTrustDeviceDeploymentGroupsList_Result {
        DataZeroTrustDeviceDeploymentGroupsList_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceDeploymentGroupsList_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceDeploymentGroupsList_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDeviceDeploymentGroupsList_ResultRef {
        DataZeroTrustDeviceDeploymentGroupsList_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceDeploymentGroupsList_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe RFC3339Nano timestamp when the deployment group was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the deployment group."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-friendly name for the deployment group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `policy_ids` after provisioning.\nContains a list of policy IDs assigned to this deployment group."]
    pub fn policy_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nThe RFC3339Nano timestamp when the deployment group was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version_config` after provisioning.\nContains version configurations for different target environments."]
    pub fn version_config(&self) -> terrars::ListRef<DataZeroTrustDeviceDeploymentGroupsList_Result_VersionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_config", self.base))
    }
}
