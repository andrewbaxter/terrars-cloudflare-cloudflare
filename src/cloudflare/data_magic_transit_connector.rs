#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataMagicTransitConnectorData_ {
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
    connector_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataMagicTransitConnector_Filter>,
}

struct DataMagicTransitConnector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMagicTransitConnectorData_>,
}

#[derive(Clone)]
pub struct DataMagicTransitConnector(std::rc::Rc<DataMagicTransitConnector_>);

impl DataMagicTransitConnector {
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

    #[doc = "Set the field `connector_id`.\n"]
    pub fn set_connector_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().connector_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataMagicTransitConnector_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `activated` after provisioning.\n"]
    pub fn activated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\n"]
    pub fn connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device` after provisioning.\n"]
    pub fn device(&self) -> DataMagicTransitConnector_DeviceRef {
        DataMagicTransitConnector_DeviceRef::new(self.shared().clone(), format!("{}.device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataMagicTransitConnector_FilterRef {
        DataMagicTransitConnector_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interrupt_window_days_of_week` after provisioning.\nAllowed days of the week for upgrades. Default is all days."]
    pub fn interrupt_window_days_of_week(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interrupt_window_days_of_week", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interrupt_window_duration_hours` after provisioning.\n"]
    pub fn interrupt_window_duration_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.interrupt_window_duration_hours", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `interrupt_window_embargo_dates` after provisioning.\nList of dates (YYYY-MM-DD) when upgrades are blocked."]
    pub fn interrupt_window_embargo_dates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.interrupt_window_embargo_dates", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `interrupt_window_hour_of_day` after provisioning.\n"]
    pub fn interrupt_window_hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interrupt_window_hour_of_day", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_heartbeat` after provisioning.\n"]
    pub fn last_heartbeat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_heartbeat", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_seen_version` after provisioning.\n"]
    pub fn last_seen_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_seen_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_key` after provisioning.\n"]
    pub fn license_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notes` after provisioning.\n"]
    pub fn notes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }
}

impl terrars::Referable for DataMagicTransitConnector {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMagicTransitConnector { }

impl terrars::ToListMappable for DataMagicTransitConnector {
    type O = terrars::ListRef<DataMagicTransitConnectorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMagicTransitConnector_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_connector".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMagicTransitConnector {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataMagicTransitConnector {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMagicTransitConnector {
        let out = DataMagicTransitConnector(std::rc::Rc::new(DataMagicTransitConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMagicTransitConnectorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                connector_id: core::default::Default::default(),
                filter: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMagicTransitConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitConnectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMagicTransitConnectorRef {
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

    #[doc = "Get a reference to the value of field `activated` after provisioning.\n"]
    pub fn activated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\n"]
    pub fn connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device` after provisioning.\n"]
    pub fn device(&self) -> DataMagicTransitConnector_DeviceRef {
        DataMagicTransitConnector_DeviceRef::new(self.shared().clone(), format!("{}.device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataMagicTransitConnector_FilterRef {
        DataMagicTransitConnector_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interrupt_window_days_of_week` after provisioning.\nAllowed days of the week for upgrades. Default is all days."]
    pub fn interrupt_window_days_of_week(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interrupt_window_days_of_week", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interrupt_window_duration_hours` after provisioning.\n"]
    pub fn interrupt_window_duration_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.interrupt_window_duration_hours", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `interrupt_window_embargo_dates` after provisioning.\nList of dates (YYYY-MM-DD) when upgrades are blocked."]
    pub fn interrupt_window_embargo_dates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.interrupt_window_embargo_dates", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `interrupt_window_hour_of_day` after provisioning.\n"]
    pub fn interrupt_window_hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interrupt_window_hour_of_day", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_heartbeat` after provisioning.\n"]
    pub fn last_heartbeat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_heartbeat", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_seen_version` after provisioning.\n"]
    pub fn last_seen_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_seen_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_key` after provisioning.\n"]
    pub fn license_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notes` after provisioning.\n"]
    pub fn notes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitConnector_Device {}

impl DataMagicTransitConnector_Device { }

impl terrars::ToListMappable for DataMagicTransitConnector_Device {
    type O = terrars::BlockAssignable<DataMagicTransitConnector_Device>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitConnector_Device {}

impl BuildDataMagicTransitConnector_Device {
    pub fn build(self) -> DataMagicTransitConnector_Device {
        DataMagicTransitConnector_Device {}
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitConnector_DeviceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitConnector_DeviceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitConnector_DeviceRef {
        DataMagicTransitConnector_DeviceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitConnector_DeviceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"MANAGED\", \"LICENSED\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMagicTransitConnector_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMagicTransitConnector_Filter {
    #[doc =
        "Set the field `device_type`.\nFilter connectors by device type.\nAvailable values: \"MANAGED\", \"LICENSED\"."]
    pub fn set_device_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMagicTransitConnector_Filter {
    type O = terrars::BlockAssignable<DataMagicTransitConnector_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMagicTransitConnector_Filter {}

impl BuildDataMagicTransitConnector_Filter {
    pub fn build(self) -> DataMagicTransitConnector_Filter {
        DataMagicTransitConnector_Filter { device_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMagicTransitConnector_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMagicTransitConnector_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMagicTransitConnector_FilterRef {
        DataMagicTransitConnector_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMagicTransitConnector_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `device_type` after provisioning.\nFilter connectors by device type.\nAvailable values: \"MANAGED\", \"LICENSED\"."]
    pub fn device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }
}
