#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDeviceSettingsData_ {
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
}

struct DataZeroTrustDeviceSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDeviceSettingsData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDeviceSettings(std::rc::Rc<DataZeroTrustDeviceSettings_>);

impl DataZeroTrustDeviceSettings {
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
        "Get a reference to the value of field `disable_for_time` after provisioning.\nSets the time limit, in seconds, that a user can use an override code to bypass WARP."]
    pub fn disable_for_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_for_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_emergency_signal_enabled` after provisioning.\nControls whether the external emergency disconnect feature is enabled."]
    pub fn external_emergency_signal_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_emergency_signal_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `external_emergency_signal_fingerprint` after provisioning.\nThe SHA256 fingerprint (64 hexadecimal characters) of the HTTPS server certificate for the external_emergency_signal_url. If provided, the WARP client will use this value to verify the server's identity. The device will ignore any response if the server's certificate fingerprint does not exactly match this value."]
    pub fn external_emergency_signal_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_emergency_signal_fingerprint", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `external_emergency_signal_interval` after provisioning.\nThe interval at which the WARP client fetches the emergency disconnect signal, formatted as a duration string (e.g., \"5m\", \"2m30s\", \"1h\"). Minimum 30 seconds."]
    pub fn external_emergency_signal_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_emergency_signal_interval", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `external_emergency_signal_url` after provisioning.\nThe HTTPS URL from which to fetch the emergency disconnect signal. Must use HTTPS and have an IPv4 or IPv6 address as the host."]
    pub fn external_emergency_signal_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_emergency_signal_url", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `gateway_proxy_enabled` after provisioning.\nEnable gateway proxy filtering on TCP."]
    pub fn gateway_proxy_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_proxy_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gateway_udp_proxy_enabled` after provisioning.\nEnable gateway proxy filtering on UDP."]
    pub fn gateway_udp_proxy_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_udp_proxy_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_certificate_installation_enabled` after provisioning.\nEnable installation of cloudflare managed root certificate."]
    pub fn root_certificate_installation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_certificate_installation_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `use_zt_virtual_ip` after provisioning.\nEnable using CGNAT virtual IPv4."]
    pub fn use_zt_virtual_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_zt_virtual_ip", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustDeviceSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDeviceSettings { }

impl terrars::ToListMappable for DataZeroTrustDeviceSettings {
    type O = terrars::ListRef<DataZeroTrustDeviceSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDeviceSettings_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDeviceSettings {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustDeviceSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDeviceSettings {
        let out = DataZeroTrustDeviceSettings(std::rc::Rc::new(DataZeroTrustDeviceSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDeviceSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDeviceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDeviceSettingsRef {
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
        "Get a reference to the value of field `disable_for_time` after provisioning.\nSets the time limit, in seconds, that a user can use an override code to bypass WARP."]
    pub fn disable_for_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_for_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_emergency_signal_enabled` after provisioning.\nControls whether the external emergency disconnect feature is enabled."]
    pub fn external_emergency_signal_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_emergency_signal_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `external_emergency_signal_fingerprint` after provisioning.\nThe SHA256 fingerprint (64 hexadecimal characters) of the HTTPS server certificate for the external_emergency_signal_url. If provided, the WARP client will use this value to verify the server's identity. The device will ignore any response if the server's certificate fingerprint does not exactly match this value."]
    pub fn external_emergency_signal_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_emergency_signal_fingerprint", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `external_emergency_signal_interval` after provisioning.\nThe interval at which the WARP client fetches the emergency disconnect signal, formatted as a duration string (e.g., \"5m\", \"2m30s\", \"1h\"). Minimum 30 seconds."]
    pub fn external_emergency_signal_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_emergency_signal_interval", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `external_emergency_signal_url` after provisioning.\nThe HTTPS URL from which to fetch the emergency disconnect signal. Must use HTTPS and have an IPv4 or IPv6 address as the host."]
    pub fn external_emergency_signal_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_emergency_signal_url", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `gateway_proxy_enabled` after provisioning.\nEnable gateway proxy filtering on TCP."]
    pub fn gateway_proxy_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_proxy_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gateway_udp_proxy_enabled` after provisioning.\nEnable gateway proxy filtering on UDP."]
    pub fn gateway_udp_proxy_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_udp_proxy_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_certificate_installation_enabled` after provisioning.\nEnable installation of cloudflare managed root certificate."]
    pub fn root_certificate_installation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_certificate_installation_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `use_zt_virtual_ip` after provisioning.\nEnable using CGNAT virtual IPv4."]
    pub fn use_zt_virtual_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_zt_virtual_ip", self.extract_ref()))
    }
}
