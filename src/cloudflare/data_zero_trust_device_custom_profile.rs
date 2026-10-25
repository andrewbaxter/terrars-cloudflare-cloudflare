#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDeviceCustomProfileData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataZeroTrustDeviceCustomProfile_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustDeviceCustomProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDeviceCustomProfileData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDeviceCustomProfile(std::rc::Rc<DataZeroTrustDeviceCustomProfile_>);

impl DataZeroTrustDeviceCustomProfile {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataZeroTrustDeviceCustomProfile_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_id`.\n"]
    pub fn set_policy_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy_id = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `browser_extension_config` after provisioning.\nBrowser extension proxy settings. Required when profile_type is browser_extension and invalid for WARP profiles."]
    pub fn browser_extension_config(&self) -> DataZeroTrustDeviceCustomProfile_BrowserExtensionConfigRef {
        DataZeroTrustDeviceCustomProfile_BrowserExtensionConfigRef::new(
            self.shared().clone(),
            format!("{}.browser_extension_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `captive_portal` after provisioning.\nTurn on the captive portal after the specified amount of time."]
    pub fn captive_portal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.captive_portal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nWhether the policy is the account default. WARP group profiles cannot set this field."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the policy."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_auto_fallback` after provisioning.\nIf the `dns_server` field of a fallback domain is not present, the client will fall back to a best guess of the default/system DNS resolvers unless this policy option is set to `true`."]
    pub fn disable_auto_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_auto_fallback", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_search_suffixes` after provisioning.\nList of DNS search suffixes to apply to clients. Suffixes are evaluated in order. Use an empty array to clear."]
    pub fn dns_search_suffixes(&self) -> terrars::ListRef<DataZeroTrustDeviceCustomProfile_DnsSearchSuffixesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_search_suffixes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the policy will be applied to matching devices."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nList of routes excluded in the WARP client's tunnel."]
    pub fn exclude(&self) -> terrars::ListRef<DataZeroTrustDeviceCustomProfile_ExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_office_ips` after provisioning.\nWhether to add Microsoft IPs to Split Tunnel exclusions."]
    pub fn exclude_office_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_office_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fallback_domains` after provisioning.\n"]
    pub fn fallback_domains(&self) -> terrars::ListRef<DataZeroTrustDeviceCustomProfile_FallbackDomainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fallback_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustDeviceCustomProfile_FilterRef {
        DataZeroTrustDeviceCustomProfile_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_unique_id` after provisioning.\n"]
    pub fn gateway_unique_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_unique_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nList of routes included in the WARP client's tunnel."]
    pub fn include(&self) -> terrars::ListRef<DataZeroTrustDeviceCustomProfile_IncludeRef> {
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

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nThe wirefilter expression to match devices. Available values: \"identity.email\", \"identity.groups.id\", \"identity.groups.name\", \"identity.groups.email\", \"identity.service_token_uuid\", \"identity.saml_attributes\", \"network\", \"os.name\", \"os.version\", \"device.tags\"."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the device settings profile."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nThe precedence of the policy. Lower values indicate higher precedence. Policies will be evaluated in ascending order of this field."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
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
    pub fn service_mode_v2(&self) -> DataZeroTrustDeviceCustomProfile_ServiceModeV2Ref {
        DataZeroTrustDeviceCustomProfile_ServiceModeV2Ref::new(
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

    #[doc = "Get a reference to the value of field `target_tests` after provisioning.\n"]
    pub fn target_tests(&self) -> terrars::ListRef<DataZeroTrustDeviceCustomProfile_TargetTestsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_tests", self.extract_ref()))
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
    pub fn virtual_networks(&self) -> DataZeroTrustDeviceCustomProfile_VirtualNetworksRef {
        DataZeroTrustDeviceCustomProfile_VirtualNetworksRef::new(
            self.shared().clone(),
            format!("{}.virtual_networks", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataZeroTrustDeviceCustomProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDeviceCustomProfile { }

impl terrars::ToListMappable for DataZeroTrustDeviceCustomProfile {
    type O = terrars::ListRef<DataZeroTrustDeviceCustomProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDeviceCustomProfile_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_device_custom_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDeviceCustomProfile {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustDeviceCustomProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDeviceCustomProfile {
        let out = DataZeroTrustDeviceCustomProfile(std::rc::Rc::new(DataZeroTrustDeviceCustomProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDeviceCustomProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                policy_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDeviceCustomProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceCustomProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDeviceCustomProfileRef {
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
        "Get a reference to the value of field `browser_extension_config` after provisioning.\nBrowser extension proxy settings. Required when profile_type is browser_extension and invalid for WARP profiles."]
    pub fn browser_extension_config(&self) -> DataZeroTrustDeviceCustomProfile_BrowserExtensionConfigRef {
        DataZeroTrustDeviceCustomProfile_BrowserExtensionConfigRef::new(
            self.shared().clone(),
            format!("{}.browser_extension_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `captive_portal` after provisioning.\nTurn on the captive portal after the specified amount of time."]
    pub fn captive_portal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.captive_portal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nWhether the policy is the account default. WARP group profiles cannot set this field."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the policy."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_auto_fallback` after provisioning.\nIf the `dns_server` field of a fallback domain is not present, the client will fall back to a best guess of the default/system DNS resolvers unless this policy option is set to `true`."]
    pub fn disable_auto_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_auto_fallback", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_search_suffixes` after provisioning.\nList of DNS search suffixes to apply to clients. Suffixes are evaluated in order. Use an empty array to clear."]
    pub fn dns_search_suffixes(&self) -> terrars::ListRef<DataZeroTrustDeviceCustomProfile_DnsSearchSuffixesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_search_suffixes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the policy will be applied to matching devices."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude` after provisioning.\nList of routes excluded in the WARP client's tunnel."]
    pub fn exclude(&self) -> terrars::ListRef<DataZeroTrustDeviceCustomProfile_ExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_office_ips` after provisioning.\nWhether to add Microsoft IPs to Split Tunnel exclusions."]
    pub fn exclude_office_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_office_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fallback_domains` after provisioning.\n"]
    pub fn fallback_domains(&self) -> terrars::ListRef<DataZeroTrustDeviceCustomProfile_FallbackDomainsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fallback_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustDeviceCustomProfile_FilterRef {
        DataZeroTrustDeviceCustomProfile_FilterRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_unique_id` after provisioning.\n"]
    pub fn gateway_unique_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_unique_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `include` after provisioning.\nList of routes included in the WARP client's tunnel."]
    pub fn include(&self) -> terrars::ListRef<DataZeroTrustDeviceCustomProfile_IncludeRef> {
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

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nThe wirefilter expression to match devices. Available values: \"identity.email\", \"identity.groups.id\", \"identity.groups.name\", \"identity.groups.email\", \"identity.service_token_uuid\", \"identity.saml_attributes\", \"network\", \"os.name\", \"os.version\", \"device.tags\"."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the device settings profile."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nThe precedence of the policy. Lower values indicate higher precedence. Policies will be evaluated in ascending order of this field."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
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
    pub fn service_mode_v2(&self) -> DataZeroTrustDeviceCustomProfile_ServiceModeV2Ref {
        DataZeroTrustDeviceCustomProfile_ServiceModeV2Ref::new(
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

    #[doc = "Get a reference to the value of field `target_tests` after provisioning.\n"]
    pub fn target_tests(&self) -> terrars::ListRef<DataZeroTrustDeviceCustomProfile_TargetTestsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_tests", self.extract_ref()))
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
    pub fn virtual_networks(&self) -> DataZeroTrustDeviceCustomProfile_VirtualNetworksRef {
        DataZeroTrustDeviceCustomProfile_VirtualNetworksRef::new(
            self.shared().clone(),
            format!("{}.virtual_networks", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDeviceCustomProfile_BrowserExtensionConfig {}

impl DataZeroTrustDeviceCustomProfile_BrowserExtensionConfig { }

impl terrars::ToListMappable for DataZeroTrustDeviceCustomProfile_BrowserExtensionConfig {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceCustomProfile_BrowserExtensionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceCustomProfile_BrowserExtensionConfig {}

impl BuildDataZeroTrustDeviceCustomProfile_BrowserExtensionConfig {
    pub fn build(self) -> DataZeroTrustDeviceCustomProfile_BrowserExtensionConfig {
        DataZeroTrustDeviceCustomProfile_BrowserExtensionConfig {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceCustomProfile_BrowserExtensionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceCustomProfile_BrowserExtensionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDeviceCustomProfile_BrowserExtensionConfigRef {
        DataZeroTrustDeviceCustomProfile_BrowserExtensionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceCustomProfile_BrowserExtensionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `proxy_control` after provisioning.\nWhether the user may disable the browser extension proxy.\nAvailable values: \"unlocked\", \"locked\"."]
    pub fn proxy_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_enabled` after provisioning.\nWhether the browser extension proxy is active."]
    pub fn proxy_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDeviceCustomProfile_DnsSearchSuffixes {}

impl DataZeroTrustDeviceCustomProfile_DnsSearchSuffixes { }

impl terrars::ToListMappable for DataZeroTrustDeviceCustomProfile_DnsSearchSuffixes {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceCustomProfile_DnsSearchSuffixes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceCustomProfile_DnsSearchSuffixes {}

impl BuildDataZeroTrustDeviceCustomProfile_DnsSearchSuffixes {
    pub fn build(self) -> DataZeroTrustDeviceCustomProfile_DnsSearchSuffixes {
        DataZeroTrustDeviceCustomProfile_DnsSearchSuffixes {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceCustomProfile_DnsSearchSuffixesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceCustomProfile_DnsSearchSuffixesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDeviceCustomProfile_DnsSearchSuffixesRef {
        DataZeroTrustDeviceCustomProfile_DnsSearchSuffixesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceCustomProfile_DnsSearchSuffixesRef {
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
pub struct DataZeroTrustDeviceCustomProfile_Exclude {}

impl DataZeroTrustDeviceCustomProfile_Exclude { }

impl terrars::ToListMappable for DataZeroTrustDeviceCustomProfile_Exclude {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceCustomProfile_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceCustomProfile_Exclude {}

impl BuildDataZeroTrustDeviceCustomProfile_Exclude {
    pub fn build(self) -> DataZeroTrustDeviceCustomProfile_Exclude {
        DataZeroTrustDeviceCustomProfile_Exclude {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceCustomProfile_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceCustomProfile_ExcludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDeviceCustomProfile_ExcludeRef {
        DataZeroTrustDeviceCustomProfile_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceCustomProfile_ExcludeRef {
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
pub struct DataZeroTrustDeviceCustomProfile_FallbackDomains {}

impl DataZeroTrustDeviceCustomProfile_FallbackDomains { }

impl terrars::ToListMappable for DataZeroTrustDeviceCustomProfile_FallbackDomains {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceCustomProfile_FallbackDomains>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceCustomProfile_FallbackDomains {}

impl BuildDataZeroTrustDeviceCustomProfile_FallbackDomains {
    pub fn build(self) -> DataZeroTrustDeviceCustomProfile_FallbackDomains {
        DataZeroTrustDeviceCustomProfile_FallbackDomains {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceCustomProfile_FallbackDomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceCustomProfile_FallbackDomainsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDeviceCustomProfile_FallbackDomainsRef {
        DataZeroTrustDeviceCustomProfile_FallbackDomainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceCustomProfile_FallbackDomainsRef {
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
pub struct DataZeroTrustDeviceCustomProfile_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustDeviceCustomProfile_Filter {
    #[doc =
        "Set the field `profile_type`.\nFilter profiles by client type. When omitted, only WARP profiles are returned.\nAvailable values: \"warp\", \"browser_extension\"."]
    pub fn set_profile_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustDeviceCustomProfile_Filter {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceCustomProfile_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceCustomProfile_Filter {}

impl BuildDataZeroTrustDeviceCustomProfile_Filter {
    pub fn build(self) -> DataZeroTrustDeviceCustomProfile_Filter {
        DataZeroTrustDeviceCustomProfile_Filter { profile_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceCustomProfile_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceCustomProfile_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDeviceCustomProfile_FilterRef {
        DataZeroTrustDeviceCustomProfile_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceCustomProfile_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `profile_type` after provisioning.\nFilter profiles by client type. When omitted, only WARP profiles are returned.\nAvailable values: \"warp\", \"browser_extension\"."]
    pub fn profile_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDeviceCustomProfile_Include {}

impl DataZeroTrustDeviceCustomProfile_Include { }

impl terrars::ToListMappable for DataZeroTrustDeviceCustomProfile_Include {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceCustomProfile_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceCustomProfile_Include {}

impl BuildDataZeroTrustDeviceCustomProfile_Include {
    pub fn build(self) -> DataZeroTrustDeviceCustomProfile_Include {
        DataZeroTrustDeviceCustomProfile_Include {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceCustomProfile_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceCustomProfile_IncludeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDeviceCustomProfile_IncludeRef {
        DataZeroTrustDeviceCustomProfile_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceCustomProfile_IncludeRef {
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
pub struct DataZeroTrustDeviceCustomProfile_ServiceModeV2 {}

impl DataZeroTrustDeviceCustomProfile_ServiceModeV2 { }

impl terrars::ToListMappable for DataZeroTrustDeviceCustomProfile_ServiceModeV2 {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceCustomProfile_ServiceModeV2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceCustomProfile_ServiceModeV2 {}

impl BuildDataZeroTrustDeviceCustomProfile_ServiceModeV2 {
    pub fn build(self) -> DataZeroTrustDeviceCustomProfile_ServiceModeV2 {
        DataZeroTrustDeviceCustomProfile_ServiceModeV2 {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceCustomProfile_ServiceModeV2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceCustomProfile_ServiceModeV2Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDeviceCustomProfile_ServiceModeV2Ref {
        DataZeroTrustDeviceCustomProfile_ServiceModeV2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceCustomProfile_ServiceModeV2Ref {
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
pub struct DataZeroTrustDeviceCustomProfile_TargetTests {}

impl DataZeroTrustDeviceCustomProfile_TargetTests { }

impl terrars::ToListMappable for DataZeroTrustDeviceCustomProfile_TargetTests {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceCustomProfile_TargetTests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceCustomProfile_TargetTests {}

impl BuildDataZeroTrustDeviceCustomProfile_TargetTests {
    pub fn build(self) -> DataZeroTrustDeviceCustomProfile_TargetTests {
        DataZeroTrustDeviceCustomProfile_TargetTests {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceCustomProfile_TargetTestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceCustomProfile_TargetTestsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDeviceCustomProfile_TargetTestsRef {
        DataZeroTrustDeviceCustomProfile_TargetTestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceCustomProfile_TargetTestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe id of the DEX test targeting this policy."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the DEX test targeting this policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDeviceCustomProfile_VirtualNetworks {}

impl DataZeroTrustDeviceCustomProfile_VirtualNetworks { }

impl terrars::ToListMappable for DataZeroTrustDeviceCustomProfile_VirtualNetworks {
    type O = terrars::BlockAssignable<DataZeroTrustDeviceCustomProfile_VirtualNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDeviceCustomProfile_VirtualNetworks {}

impl BuildDataZeroTrustDeviceCustomProfile_VirtualNetworks {
    pub fn build(self) -> DataZeroTrustDeviceCustomProfile_VirtualNetworks {
        DataZeroTrustDeviceCustomProfile_VirtualNetworks {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDeviceCustomProfile_VirtualNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDeviceCustomProfile_VirtualNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDeviceCustomProfile_VirtualNetworksRef {
        DataZeroTrustDeviceCustomProfile_VirtualNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDeviceCustomProfile_VirtualNetworksRef {
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
