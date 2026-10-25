#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDeviceDefaultProfileData_ {
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
    allow_mode_switch: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_updates: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_to_leave: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_connect: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    captive_portal: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_auto_fallback: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_search_suffixes: std::option::Option<std::vec::Vec<ZeroTrustDeviceDefaultProfile_DnsSearchSuffixes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<std::vec::Vec<ZeroTrustDeviceDefaultProfile_Exclude>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_office_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_acceleration: std::option::Option<ZeroTrustDeviceDefaultProfile_GlobalAcceleration>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<std::vec::Vec<ZeroTrustDeviceDefaultProfile_Include>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lan_allow_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lan_allow_subnet_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    register_interface_ip_with_dns: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sccm_vpn_boundary_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_mode_v2: std::option::Option<ZeroTrustDeviceDefaultProfile_ServiceModeV2>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    switch_locked: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uninstall_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_networks: std::option::Option<ZeroTrustDeviceDefaultProfile_VirtualNetworks>,
}

struct ZeroTrustDeviceDefaultProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDeviceDefaultProfileData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDeviceDefaultProfile(std::rc::Rc<ZeroTrustDeviceDefaultProfile_>);

impl ZeroTrustDeviceDefaultProfile {
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

    #[doc = "Set the field `allow_mode_switch`.\nWhether to allow the user to switch WARP between modes."]
    pub fn set_allow_mode_switch(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_mode_switch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_updates`.\nWhether to receive update notifications when a new version of the client is available."]
    pub fn set_allow_updates(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_updates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_to_leave`.\nWhether to allow devices to leave the organization."]
    pub fn set_allowed_to_leave(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allowed_to_leave = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_connect`.\nThe amount of time in seconds to reconnect after having been disabled."]
    pub fn set_auto_connect(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().auto_connect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `captive_portal`.\nTurn on the captive portal after the specified amount of time."]
    pub fn set_captive_portal(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().captive_portal = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_auto_fallback`.\nIf the `dns_server` field of a fallback domain is not present, the client will fall back to a best guess of the default/system DNS resolvers unless this policy option is set to `true`."]
    pub fn set_disable_auto_fallback(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_auto_fallback = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dns_search_suffixes`.\nList of DNS search suffixes to apply to clients. Suffixes are evaluated in order. Use an empty array to clear."]
    pub fn set_dns_search_suffixes(
        self,
        v: impl std::convert::Into<std::vec::Vec<ZeroTrustDeviceDefaultProfile_DnsSearchSuffixes>>,
    ) -> Self {
        self.0.data.borrow_mut().dns_search_suffixes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclude`.\nList of routes excluded in the WARP client's tunnel. Both 'exclude' and 'include' cannot be set in the same request."]
    pub fn set_exclude(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustDeviceDefaultProfile_Exclude>>) -> Self {
        self.0.data.borrow_mut().exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_office_ips`.\nWhether to add Microsoft IPs to Split Tunnel exclusions."]
    pub fn set_exclude_office_ips(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_office_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `global_acceleration`.\nGlobal Acceleration settings for China. When configured, WARP clients connect to the Global Accelerator addresses instead of the default ones. Please contact your account representative to enable this feature on your account. See https://developers.cloudflare.com/china-network/concepts/global-acceleration/."]
    pub fn set_global_acceleration(
        self,
        v: impl std::convert::Into<ZeroTrustDeviceDefaultProfile_GlobalAcceleration>,
    ) -> Self {
        self.0.data.borrow_mut().global_acceleration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include`.\nList of routes included in the WARP client's tunnel. Both 'exclude' and 'include' cannot be set in the same request."]
    pub fn set_include(self, v: impl std::convert::Into<std::vec::Vec<ZeroTrustDeviceDefaultProfile_Include>>) -> Self {
        self.0.data.borrow_mut().include = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lan_allow_minutes`.\nThe amount of time in minutes a user is allowed access to their LAN. A value of 0 will allow LAN access until the next WARP reconnection, such as a reboot or a laptop waking from sleep. Note that this field is omitted from the response if null or unset."]
    pub fn set_lan_allow_minutes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().lan_allow_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lan_allow_subnet_size`.\nThe size of the subnet for the local access network. Note that this field is omitted from the response if null or unset."]
    pub fn set_lan_allow_subnet_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().lan_allow_subnet_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `register_interface_ip_with_dns`.\nDetermines if the operating system will register WARP's local interface IP with your on-premises DNS server."]
    pub fn set_register_interface_ip_with_dns(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().register_interface_ip_with_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sccm_vpn_boundary_support`.\nDetermines whether the WARP client indicates to SCCM that it is inside a VPN boundary. (Windows only)."]
    pub fn set_sccm_vpn_boundary_support(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sccm_vpn_boundary_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_mode_v2`.\n"]
    pub fn set_service_mode_v2(self, v: impl std::convert::Into<ZeroTrustDeviceDefaultProfile_ServiceModeV2>) -> Self {
        self.0.data.borrow_mut().service_mode_v2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `support_url`.\nThe URL to launch when the Send Feedback button is clicked."]
    pub fn set_support_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().support_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `switch_locked`.\nWhether to allow the user to turn off the WARP switch and disconnect the client."]
    pub fn set_switch_locked(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().switch_locked = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_protocol`.\nDetermines which tunnel protocol to use."]
    pub fn set_tunnel_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tunnel_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `uninstall_protection`.\nDetermines whether uninstalling the WARP client requires an override code. (Windows only)."]
    pub fn set_uninstall_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().uninstall_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_networks`.\nVirtual network access settings for the device."]
    pub fn set_virtual_networks(self, v: impl std::convert::Into<ZeroTrustDeviceDefaultProfile_VirtualNetworks>) -> Self {
        self.0.data.borrow_mut().virtual_networks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_mode_switch` after provisioning.\nWhether to allow the user to switch WARP between modes."]
    pub fn allow_mode_switch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_mode_switch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_updates` after provisioning.\nWhether to receive update notifications when a new version of the client is available."]
    pub fn allow_updates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_updates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_to_leave` after provisioning.\nWhether to allow devices to leave the organization."]
    pub fn allowed_to_leave(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_to_leave", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_connect` after provisioning.\nThe amount of time in seconds to reconnect after having been disabled."]
    pub fn auto_connect(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_connect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `captive_portal` after provisioning.\nTurn on the captive portal after the specified amount of time."]
    pub fn captive_portal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.captive_portal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nWhether the policy will be applied to matching devices."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_auto_fallback` after provisioning.\nIf the `dns_server` field of a fallback domain is not present, the client will fall back to a best guess of the default/system DNS resolvers unless this policy option is set to `true`."]
    pub fn disable_auto_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_auto_fallback", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_search_suffixes` after provisioning.\nList of DNS search suffixes to apply to clients. Suffixes are evaluated in order. Use an empty array to clear."]
    pub fn dns_search_suffixes(&self) -> terrars::ListRef<ZeroTrustDeviceDefaultProfile_DnsSearchSuffixesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_search_suffixes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the policy will be applied to matching devices."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nList of routes excluded in the WARP client's tunnel. Both 'exclude' and 'include' cannot be set in the same request."]
    pub fn exclude(&self) -> terrars::ListRef<ZeroTrustDeviceDefaultProfile_ExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_office_ips` after provisioning.\nWhether to add Microsoft IPs to Split Tunnel exclusions."]
    pub fn exclude_office_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_office_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fallback_domains` after provisioning.\n"]
    pub fn fallback_domains(&self) -> terrars::ListRef<ZeroTrustDeviceDefaultProfile_FallbackDomainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fallback_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_unique_id` after provisioning.\n"]
    pub fn gateway_unique_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_unique_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `global_acceleration` after provisioning.\nGlobal Acceleration settings for China. When configured, WARP clients connect to the Global Accelerator addresses instead of the default ones. Please contact your account representative to enable this feature on your account. See https://developers.cloudflare.com/china-network/concepts/global-acceleration/."]
    pub fn global_acceleration(&self) -> ZeroTrustDeviceDefaultProfile_GlobalAccelerationRef {
        ZeroTrustDeviceDefaultProfile_GlobalAccelerationRef::new(
            self.shared().clone(),
            format!("{}.global_acceleration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nList of routes included in the WARP client's tunnel. Both 'exclude' and 'include' cannot be set in the same request."]
    pub fn include(&self) -> terrars::ListRef<ZeroTrustDeviceDefaultProfile_IncludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lan_allow_minutes` after provisioning.\nThe amount of time in minutes a user is allowed access to their LAN. A value of 0 will allow LAN access until the next WARP reconnection, such as a reboot or a laptop waking from sleep. Note that this field is omitted from the response if null or unset."]
    pub fn lan_allow_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_allow_minutes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lan_allow_subnet_size` after provisioning.\nThe size of the subnet for the local access network. Note that this field is omitted from the response if null or unset."]
    pub fn lan_allow_subnet_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_allow_subnet_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `profile_type` after provisioning.\nThe client type to which the device settings profile applies. This field is set when the profile is created and cannot be changed.\nAvailable values: \"warp\", \"browser_extension\"."]
    pub fn profile_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `register_interface_ip_with_dns` after provisioning.\nDetermines if the operating system will register WARP's local interface IP with your on-premises DNS server."]
    pub fn register_interface_ip_with_dns(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.register_interface_ip_with_dns", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `sccm_vpn_boundary_support` after provisioning.\nDetermines whether the WARP client indicates to SCCM that it is inside a VPN boundary. (Windows only)."]
    pub fn sccm_vpn_boundary_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sccm_vpn_boundary_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_mode_v2` after provisioning.\n"]
    pub fn service_mode_v2(&self) -> ZeroTrustDeviceDefaultProfile_ServiceModeV2Ref {
        ZeroTrustDeviceDefaultProfile_ServiceModeV2Ref::new(
            self.shared().clone(),
            format!("{}.service_mode_v2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `support_url` after provisioning.\nThe URL to launch when the Send Feedback button is clicked."]
    pub fn support_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `switch_locked` after provisioning.\nWhether to allow the user to turn off the WARP switch and disconnect the client."]
    pub fn switch_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.switch_locked", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_protocol` after provisioning.\nDetermines which tunnel protocol to use."]
    pub fn tunnel_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uninstall_protection` after provisioning.\nDetermines whether uninstalling the WARP client requires an override code. (Windows only)."]
    pub fn uninstall_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uninstall_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `virtual_networks` after provisioning.\nVirtual network access settings for the device."]
    pub fn virtual_networks(&self) -> ZeroTrustDeviceDefaultProfile_VirtualNetworksRef {
        ZeroTrustDeviceDefaultProfile_VirtualNetworksRef::new(
            self.shared().clone(),
            format!("{}.virtual_networks", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ZeroTrustDeviceDefaultProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDeviceDefaultProfile { }

impl terrars::ToListMappable for ZeroTrustDeviceDefaultProfile {
    type O = terrars::ListRef<ZeroTrustDeviceDefaultProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDeviceDefaultProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_default_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDeviceDefaultProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDeviceDefaultProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDeviceDefaultProfile {
        let out = ZeroTrustDeviceDefaultProfile(std::rc::Rc::new(ZeroTrustDeviceDefaultProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDeviceDefaultProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                allow_mode_switch: core::default::Default::default(),
                allow_updates: core::default::Default::default(),
                allowed_to_leave: core::default::Default::default(),
                auto_connect: core::default::Default::default(),
                captive_portal: core::default::Default::default(),
                disable_auto_fallback: core::default::Default::default(),
                dns_search_suffixes: core::default::Default::default(),
                exclude: core::default::Default::default(),
                exclude_office_ips: core::default::Default::default(),
                global_acceleration: core::default::Default::default(),
                include: core::default::Default::default(),
                lan_allow_minutes: core::default::Default::default(),
                lan_allow_subnet_size: core::default::Default::default(),
                register_interface_ip_with_dns: core::default::Default::default(),
                sccm_vpn_boundary_support: core::default::Default::default(),
                service_mode_v2: core::default::Default::default(),
                support_url: core::default::Default::default(),
                switch_locked: core::default::Default::default(),
                tunnel_protocol: core::default::Default::default(),
                uninstall_protection: core::default::Default::default(),
                virtual_networks: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDeviceDefaultProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceDefaultProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDeviceDefaultProfileRef {
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
        "Get a reference to the value of field `allow_mode_switch` after provisioning.\nWhether to allow the user to switch WARP between modes."]
    pub fn allow_mode_switch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_mode_switch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_updates` after provisioning.\nWhether to receive update notifications when a new version of the client is available."]
    pub fn allow_updates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_updates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_to_leave` after provisioning.\nWhether to allow devices to leave the organization."]
    pub fn allowed_to_leave(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_to_leave", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_connect` after provisioning.\nThe amount of time in seconds to reconnect after having been disabled."]
    pub fn auto_connect(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_connect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `captive_portal` after provisioning.\nTurn on the captive portal after the specified amount of time."]
    pub fn captive_portal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.captive_portal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nWhether the policy will be applied to matching devices."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_auto_fallback` after provisioning.\nIf the `dns_server` field of a fallback domain is not present, the client will fall back to a best guess of the default/system DNS resolvers unless this policy option is set to `true`."]
    pub fn disable_auto_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_auto_fallback", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_search_suffixes` after provisioning.\nList of DNS search suffixes to apply to clients. Suffixes are evaluated in order. Use an empty array to clear."]
    pub fn dns_search_suffixes(&self) -> terrars::ListRef<ZeroTrustDeviceDefaultProfile_DnsSearchSuffixesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_search_suffixes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the policy will be applied to matching devices."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nList of routes excluded in the WARP client's tunnel. Both 'exclude' and 'include' cannot be set in the same request."]
    pub fn exclude(&self) -> terrars::ListRef<ZeroTrustDeviceDefaultProfile_ExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_office_ips` after provisioning.\nWhether to add Microsoft IPs to Split Tunnel exclusions."]
    pub fn exclude_office_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_office_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fallback_domains` after provisioning.\n"]
    pub fn fallback_domains(&self) -> terrars::ListRef<ZeroTrustDeviceDefaultProfile_FallbackDomainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fallback_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_unique_id` after provisioning.\n"]
    pub fn gateway_unique_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_unique_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `global_acceleration` after provisioning.\nGlobal Acceleration settings for China. When configured, WARP clients connect to the Global Accelerator addresses instead of the default ones. Please contact your account representative to enable this feature on your account. See https://developers.cloudflare.com/china-network/concepts/global-acceleration/."]
    pub fn global_acceleration(&self) -> ZeroTrustDeviceDefaultProfile_GlobalAccelerationRef {
        ZeroTrustDeviceDefaultProfile_GlobalAccelerationRef::new(
            self.shared().clone(),
            format!("{}.global_acceleration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nList of routes included in the WARP client's tunnel. Both 'exclude' and 'include' cannot be set in the same request."]
    pub fn include(&self) -> terrars::ListRef<ZeroTrustDeviceDefaultProfile_IncludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lan_allow_minutes` after provisioning.\nThe amount of time in minutes a user is allowed access to their LAN. A value of 0 will allow LAN access until the next WARP reconnection, such as a reboot or a laptop waking from sleep. Note that this field is omitted from the response if null or unset."]
    pub fn lan_allow_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_allow_minutes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lan_allow_subnet_size` after provisioning.\nThe size of the subnet for the local access network. Note that this field is omitted from the response if null or unset."]
    pub fn lan_allow_subnet_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lan_allow_subnet_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `profile_type` after provisioning.\nThe client type to which the device settings profile applies. This field is set when the profile is created and cannot be changed.\nAvailable values: \"warp\", \"browser_extension\"."]
    pub fn profile_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `register_interface_ip_with_dns` after provisioning.\nDetermines if the operating system will register WARP's local interface IP with your on-premises DNS server."]
    pub fn register_interface_ip_with_dns(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.register_interface_ip_with_dns", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `sccm_vpn_boundary_support` after provisioning.\nDetermines whether the WARP client indicates to SCCM that it is inside a VPN boundary. (Windows only)."]
    pub fn sccm_vpn_boundary_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sccm_vpn_boundary_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_mode_v2` after provisioning.\n"]
    pub fn service_mode_v2(&self) -> ZeroTrustDeviceDefaultProfile_ServiceModeV2Ref {
        ZeroTrustDeviceDefaultProfile_ServiceModeV2Ref::new(
            self.shared().clone(),
            format!("{}.service_mode_v2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `support_url` after provisioning.\nThe URL to launch when the Send Feedback button is clicked."]
    pub fn support_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `switch_locked` after provisioning.\nWhether to allow the user to turn off the WARP switch and disconnect the client."]
    pub fn switch_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.switch_locked", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_protocol` after provisioning.\nDetermines which tunnel protocol to use."]
    pub fn tunnel_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uninstall_protection` after provisioning.\nDetermines whether uninstalling the WARP client requires an override code. (Windows only)."]
    pub fn uninstall_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uninstall_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `virtual_networks` after provisioning.\nVirtual network access settings for the device."]
    pub fn virtual_networks(&self) -> ZeroTrustDeviceDefaultProfile_VirtualNetworksRef {
        ZeroTrustDeviceDefaultProfile_VirtualNetworksRef::new(
            self.shared().clone(),
            format!("{}.virtual_networks", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDeviceDefaultProfile_DnsSearchSuffixes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    suffix: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDeviceDefaultProfile_DnsSearchSuffixes {
    #[doc = "Set the field `description`.\nA description of the DNS search suffix."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDeviceDefaultProfile_DnsSearchSuffixes {
    type O = terrars::BlockAssignable<ZeroTrustDeviceDefaultProfile_DnsSearchSuffixes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDeviceDefaultProfile_DnsSearchSuffixes {
    #[doc = "The DNS search suffix to append when resolving short hostnames."]
    pub suffix: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDeviceDefaultProfile_DnsSearchSuffixes {
    pub fn build(self) -> ZeroTrustDeviceDefaultProfile_DnsSearchSuffixes {
        ZeroTrustDeviceDefaultProfile_DnsSearchSuffixes {
            description: core::default::Default::default(),
            suffix: self.suffix,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDeviceDefaultProfile_DnsSearchSuffixesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceDefaultProfile_DnsSearchSuffixesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDeviceDefaultProfile_DnsSearchSuffixesRef {
        ZeroTrustDeviceDefaultProfile_DnsSearchSuffixesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDeviceDefaultProfile_DnsSearchSuffixesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the DNS search suffix."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThe DNS search suffix to append when resolving short hostnames."]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDeviceDefaultProfile_Exclude {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustDeviceDefaultProfile_Exclude {
    #[doc =
        "Set the field `address`.\nThe address in CIDR format to exclude from the tunnel. If `address` is present, `host` must not be present."]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA description of the Split Tunnel item, displayed in the client UI."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `host`.\nThe domain name to exclude from the tunnel. If `host` is present, `address` must not be present."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDeviceDefaultProfile_Exclude {
    type O = terrars::BlockAssignable<ZeroTrustDeviceDefaultProfile_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDeviceDefaultProfile_Exclude {}

impl BuildZeroTrustDeviceDefaultProfile_Exclude {
    pub fn build(self) -> ZeroTrustDeviceDefaultProfile_Exclude {
        ZeroTrustDeviceDefaultProfile_Exclude {
            address: core::default::Default::default(),
            description: core::default::Default::default(),
            host: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDeviceDefaultProfile_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceDefaultProfile_ExcludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDeviceDefaultProfile_ExcludeRef {
        ZeroTrustDeviceDefaultProfile_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDeviceDefaultProfile_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nThe address in CIDR format to exclude from the tunnel. If `address` is present, `host` must not be present."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the Split Tunnel item, displayed in the client UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe domain name to exclude from the tunnel. If `host` is present, `address` must not be present."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDeviceDefaultProfile_FallbackDomains {}

impl ZeroTrustDeviceDefaultProfile_FallbackDomains { }

impl terrars::ToListMappable for ZeroTrustDeviceDefaultProfile_FallbackDomains {
    type O = terrars::BlockAssignable<ZeroTrustDeviceDefaultProfile_FallbackDomains>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDeviceDefaultProfile_FallbackDomains {}

impl BuildZeroTrustDeviceDefaultProfile_FallbackDomains {
    pub fn build(self) -> ZeroTrustDeviceDefaultProfile_FallbackDomains {
        ZeroTrustDeviceDefaultProfile_FallbackDomains {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDeviceDefaultProfile_FallbackDomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceDefaultProfile_FallbackDomainsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDeviceDefaultProfile_FallbackDomainsRef {
        ZeroTrustDeviceDefaultProfile_FallbackDomainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDeviceDefaultProfile_FallbackDomainsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the fallback domain, displayed in the client UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_server` after provisioning.\nA list of IP addresses to handle domain resolution."]
    pub fn dns_server(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_server", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThe domain suffix to match when resolving locally."]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDeviceDefaultProfile_GlobalAcceleration {
    api_endpoints: terrars::ListField<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    masque_endpoints: terrars::ListField<terrars::PrimField<std::string::String>>,
    wireguard_endpoints: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustDeviceDefaultProfile_GlobalAcceleration { }

impl terrars::ToListMappable for ZeroTrustDeviceDefaultProfile_GlobalAcceleration {
    type O = terrars::BlockAssignable<ZeroTrustDeviceDefaultProfile_GlobalAcceleration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDeviceDefaultProfile_GlobalAcceleration {
    #[doc = "IP:port entries for the API endpoints."]
    pub api_endpoints: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Global acceleration settings are used only when \"enabled\"."]
    pub enabled: terrars::PrimField<bool>,
    #[doc =
        "IP:port entries for the MASQUE tunnel endpoints. Either wireguard_endpoints or masque_endpoints must be provided."]
    pub masque_endpoints: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "IP:port entries for the WireGuard tunnel endpoints. Either wireguard_endpoints or masque_endpoints must be provided."]
    pub wireguard_endpoints: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildZeroTrustDeviceDefaultProfile_GlobalAcceleration {
    pub fn build(self) -> ZeroTrustDeviceDefaultProfile_GlobalAcceleration {
        ZeroTrustDeviceDefaultProfile_GlobalAcceleration {
            api_endpoints: self.api_endpoints,
            enabled: self.enabled,
            masque_endpoints: self.masque_endpoints,
            wireguard_endpoints: self.wireguard_endpoints,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDeviceDefaultProfile_GlobalAccelerationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceDefaultProfile_GlobalAccelerationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDeviceDefaultProfile_GlobalAccelerationRef {
        ZeroTrustDeviceDefaultProfile_GlobalAccelerationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDeviceDefaultProfile_GlobalAccelerationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_endpoints` after provisioning.\nIP:port entries for the API endpoints."]
    pub fn api_endpoints(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_endpoints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nGlobal acceleration settings are used only when \"enabled\"."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `masque_endpoints` after provisioning.\nIP:port entries for the MASQUE tunnel endpoints. Either wireguard_endpoints or masque_endpoints must be provided."]
    pub fn masque_endpoints(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.masque_endpoints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wireguard_endpoints` after provisioning.\nIP:port entries for the WireGuard tunnel endpoints. Either wireguard_endpoints or masque_endpoints must be provided."]
    pub fn wireguard_endpoints(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wireguard_endpoints", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDeviceDefaultProfile_Include {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustDeviceDefaultProfile_Include {
    #[doc =
        "Set the field `address`.\nThe address in CIDR format to include in the tunnel. If `address` is present, `host` must not be present."]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA description of the Split Tunnel item, displayed in the client UI."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `host`.\nThe domain name to include in the tunnel. If `host` is present, `address` must not be present."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDeviceDefaultProfile_Include {
    type O = terrars::BlockAssignable<ZeroTrustDeviceDefaultProfile_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDeviceDefaultProfile_Include {}

impl BuildZeroTrustDeviceDefaultProfile_Include {
    pub fn build(self) -> ZeroTrustDeviceDefaultProfile_Include {
        ZeroTrustDeviceDefaultProfile_Include {
            address: core::default::Default::default(),
            description: core::default::Default::default(),
            host: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDeviceDefaultProfile_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceDefaultProfile_IncludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDeviceDefaultProfile_IncludeRef {
        ZeroTrustDeviceDefaultProfile_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDeviceDefaultProfile_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nThe address in CIDR format to include in the tunnel. If `address` is present, `host` must not be present."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the Split Tunnel item, displayed in the client UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe domain name to include in the tunnel. If `host` is present, `address` must not be present."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDeviceDefaultProfile_ServiceModeV2 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl ZeroTrustDeviceDefaultProfile_ServiceModeV2 {
    #[doc = "Set the field `mode`.\nThe mode to run the WARP client under."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe port number when used with proxy mode."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustDeviceDefaultProfile_ServiceModeV2 {
    type O = terrars::BlockAssignable<ZeroTrustDeviceDefaultProfile_ServiceModeV2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDeviceDefaultProfile_ServiceModeV2 {}

impl BuildZeroTrustDeviceDefaultProfile_ServiceModeV2 {
    pub fn build(self) -> ZeroTrustDeviceDefaultProfile_ServiceModeV2 {
        ZeroTrustDeviceDefaultProfile_ServiceModeV2 {
            mode: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDeviceDefaultProfile_ServiceModeV2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceDefaultProfile_ServiceModeV2Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDeviceDefaultProfile_ServiceModeV2Ref {
        ZeroTrustDeviceDefaultProfile_ServiceModeV2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDeviceDefaultProfile_ServiceModeV2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe mode to run the WARP client under."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number when used with proxy mode."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDeviceDefaultProfile_VirtualNetworks {
    allowed: terrars::ListField<terrars::PrimField<std::string::String>>,
    default: terrars::PrimField<std::string::String>,
}

impl ZeroTrustDeviceDefaultProfile_VirtualNetworks { }

impl terrars::ToListMappable for ZeroTrustDeviceDefaultProfile_VirtualNetworks {
    type O = terrars::BlockAssignable<ZeroTrustDeviceDefaultProfile_VirtualNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDeviceDefaultProfile_VirtualNetworks {
    #[doc =
        "List of virtual network IDs the device is allowed to access. When virtual_networks is set, at least one entry is required."]
    pub allowed: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The default virtual network ID. Must be included in the `allowed` list."]
    pub default: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDeviceDefaultProfile_VirtualNetworks {
    pub fn build(self) -> ZeroTrustDeviceDefaultProfile_VirtualNetworks {
        ZeroTrustDeviceDefaultProfile_VirtualNetworks {
            allowed: self.allowed,
            default: self.default,
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDeviceDefaultProfile_VirtualNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDeviceDefaultProfile_VirtualNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustDeviceDefaultProfile_VirtualNetworksRef {
        ZeroTrustDeviceDefaultProfile_VirtualNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDeviceDefaultProfile_VirtualNetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed` after provisioning.\nList of virtual network IDs the device is allowed to access. When virtual_networks is set, at least one entry is required."]
    pub fn allowed(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nThe default virtual network ID. Must be included in the `allowed` list."]
    pub fn default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
}
