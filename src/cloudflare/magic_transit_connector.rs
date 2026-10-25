#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct MagicTransitConnectorData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activated: std::option::Option<terrars::PrimField<bool>>,
    device: MagicTransitConnector_Device,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interrupt_window_duration_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interrupt_window_hour_of_day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct MagicTransitConnector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MagicTransitConnectorData_>,
}

#[derive(Clone)]
pub struct MagicTransitConnector(std::rc::Rc<MagicTransitConnector_>);

impl MagicTransitConnector {
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

    #[doc = "Set the field `activated`.\n"]
    pub fn set_activated(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().activated = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interrupt_window_duration_hours`.\n"]
    pub fn set_interrupt_window_duration_hours(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().interrupt_window_duration_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interrupt_window_hour_of_day`.\n"]
    pub fn set_interrupt_window_hour_of_day(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().interrupt_window_hour_of_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notes`.\n"]
    pub fn set_notes(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().notes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `activated` after provisioning.\n"]
    pub fn activated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device` after provisioning.\n"]
    pub fn device(&self) -> MagicTransitConnector_DeviceRef {
        MagicTransitConnector_DeviceRef::new(self.shared().clone(), format!("{}.device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interrupt_window_duration_hours` after provisioning.\n"]
    pub fn interrupt_window_duration_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.interrupt_window_duration_hours", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `interrupt_window_hour_of_day` after provisioning.\n"]
    pub fn interrupt_window_hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interrupt_window_hour_of_day", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_key` after provisioning.\nLicense key for the connector. This is only returned on creation and will not be available in subsequent reads."]
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

impl terrars::Referable for MagicTransitConnector {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MagicTransitConnector { }

impl terrars::ToListMappable for MagicTransitConnector {
    type O = terrars::ListRef<MagicTransitConnectorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MagicTransitConnector_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_magic_transit_connector".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMagicTransitConnector {
    pub tf_id: std::string::String,
    #[doc = "Account identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub device: MagicTransitConnector_Device,
}

impl BuildMagicTransitConnector {
    pub fn build(self, stack: &mut terrars::Stack) -> MagicTransitConnector {
        let out = MagicTransitConnector(std::rc::Rc::new(MagicTransitConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MagicTransitConnectorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                activated: core::default::Default::default(),
                device: self.device,
                interrupt_window_duration_hours: core::default::Default::default(),
                interrupt_window_hour_of_day: core::default::Default::default(),
                notes: core::default::Default::default(),
                timezone: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MagicTransitConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitConnectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MagicTransitConnectorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `activated` after provisioning.\n"]
    pub fn activated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device` after provisioning.\n"]
    pub fn device(&self) -> MagicTransitConnector_DeviceRef {
        MagicTransitConnector_DeviceRef::new(self.shared().clone(), format!("{}.device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interrupt_window_duration_hours` after provisioning.\n"]
    pub fn interrupt_window_duration_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.interrupt_window_duration_hours", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `interrupt_window_hour_of_day` after provisioning.\n"]
    pub fn interrupt_window_hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interrupt_window_hour_of_day", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_key` after provisioning.\nLicense key for the connector. This is only returned on creation and will not be available in subsequent reads."]
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
pub struct MagicTransitConnector_Device {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provision_license: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serial_number: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MagicTransitConnector_Device {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provision_license`.\nSet to true to provision a license key for this connector. Only used during resource creation. This is a write-only field that will not be stored in state."]
    pub fn set_provision_license(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.provision_license = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serial_number`.\n"]
    pub fn set_serial_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.serial_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MagicTransitConnector_Device {
    type O = terrars::BlockAssignable<MagicTransitConnector_Device>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMagicTransitConnector_Device {}

impl BuildMagicTransitConnector_Device {
    pub fn build(self) -> MagicTransitConnector_Device {
        MagicTransitConnector_Device {
            id: core::default::Default::default(),
            provision_license: core::default::Default::default(),
            serial_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MagicTransitConnector_DeviceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MagicTransitConnector_DeviceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MagicTransitConnector_DeviceRef {
        MagicTransitConnector_DeviceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MagicTransitConnector_DeviceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provision_license` after provisioning.\nSet to true to provision a license key for this connector. Only used during resource creation. This is a write-only field that will not be stored in state."]
    pub fn provision_license(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provision_license", self.base))
    }

    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.base))
    }
}
