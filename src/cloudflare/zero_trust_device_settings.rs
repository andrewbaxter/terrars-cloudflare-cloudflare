#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDeviceSettingsData_ {
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
    disable_for_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_emergency_signal_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_emergency_signal_fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_emergency_signal_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_emergency_signal_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_proxy_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_udp_proxy_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_certificate_installation_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_zt_virtual_ip: std::option::Option<terrars::PrimField<bool>>,
}

struct ZeroTrustDeviceSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDeviceSettingsData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDeviceSettings(std::rc::Rc<ZeroTrustDeviceSettings_>);

impl ZeroTrustDeviceSettings {
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

    #[doc =
        "Set the field `disable_for_time`.\nSets the time limit, in seconds, that a user can use an override code to bypass WARP."]
    pub fn set_disable_for_time(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().disable_for_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `external_emergency_signal_enabled`.\nControls whether the external emergency disconnect feature is enabled."]
    pub fn set_external_emergency_signal_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().external_emergency_signal_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `external_emergency_signal_fingerprint`.\nThe SHA256 fingerprint (64 hexadecimal characters) of the HTTPS server certificate for the external_emergency_signal_url. If provided, the WARP client will use this value to verify the server's identity. The device will ignore any response if the server's certificate fingerprint does not exactly match this value."]
    pub fn set_external_emergency_signal_fingerprint(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().external_emergency_signal_fingerprint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `external_emergency_signal_interval`.\nThe interval at which the WARP client fetches the emergency disconnect signal, formatted as a duration string (e.g., \"5m\", \"2m30s\", \"1h\"). Minimum 30 seconds."]
    pub fn set_external_emergency_signal_interval(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().external_emergency_signal_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `external_emergency_signal_url`.\nThe HTTPS URL from which to fetch the emergency disconnect signal. Must use HTTPS and have an IPv4 or IPv6 address as the host."]
    pub fn set_external_emergency_signal_url(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().external_emergency_signal_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_proxy_enabled`.\nEnable gateway proxy filtering on TCP."]
    pub fn set_gateway_proxy_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().gateway_proxy_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_udp_proxy_enabled`.\nEnable gateway proxy filtering on UDP."]
    pub fn set_gateway_udp_proxy_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().gateway_udp_proxy_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `root_certificate_installation_enabled`.\nEnable installation of cloudflare managed root certificate."]
    pub fn set_root_certificate_installation_enabled(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().root_certificate_installation_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_zt_virtual_ip`.\nEnable using CGNAT virtual IPv4."]
    pub fn set_use_zt_virtual_ip(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_zt_virtual_ip = std::option::Option::Some(v.into());
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

impl terrars::Referable for ZeroTrustDeviceSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDeviceSettings { }

impl terrars::ToListMappable for ZeroTrustDeviceSettings {
    type O = terrars::ListRef<ZeroTrustDeviceSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDeviceSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDeviceSettings {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDeviceSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDeviceSettings {
        let out = ZeroTrustDeviceSettings(std::rc::Rc::new(ZeroTrustDeviceSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDeviceSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                disable_for_time: core::default::Default::default(),
                external_emergency_signal_enabled: core::default::Default::default(),
                external_emergency_signal_fingerprint: core::default::Default::default(),
                external_emergency_signal_interval: core::default::Default::default(),
                external_emergency_signal_url: core::default::Default::default(),
                gateway_proxy_enabled: core::default::Default::default(),
                gateway_udp_proxy_enabled: core::default::Default::default(),
                root_certificate_installation_enabled: core::default::Default::default(),
                use_zt_virtual_ip: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDeviceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDeviceSettingsRef {
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
