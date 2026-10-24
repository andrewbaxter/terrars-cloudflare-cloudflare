use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DeviceSettingsPolicyData {
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
    default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_auto_fallback: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_office_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    precedence: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_mode_v2_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_mode_v2_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    switch_locked: std::option::Option<terrars::PrimField<bool>>,
}

struct DeviceSettingsPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DeviceSettingsPolicyData>,
}

#[derive(Clone)]
pub struct DeviceSettingsPolicy(std::rc::Rc<DeviceSettingsPolicy_>);

impl DeviceSettingsPolicy {
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

    #[doc = "Set the field `allow_mode_switch`.\nWhether to allow mode switch for this policy."]
    pub fn set_allow_mode_switch(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_mode_switch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_updates`.\nWhether to allow updates under this policy."]
    pub fn set_allow_updates(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_updates = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_to_leave`.\nWhether to allow devices to leave the organization. Defaults to `true`."]
    pub fn set_allowed_to_leave(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allowed_to_leave = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_connect`.\nThe amount of time in minutes to reconnect after having been disabled."]
    pub fn set_auto_connect(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().auto_connect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `captive_portal`.\nThe captive portal value for this policy. Defaults to `180`."]
    pub fn set_captive_portal(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().captive_portal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default`.\nWhether the policy refers to the default account policy."]
    pub fn set_default(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_auto_fallback`.\nWhether to disable auto fallback for this policy."]
    pub fn set_disable_auto_fallback(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_auto_fallback = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enabled`.\nWhether the policy is enabled (cannot be set for default policies). Defaults to `true`."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_office_ips`.\nWhether to add Microsoft IPs to split tunnel exclusions."]
    pub fn set_exclude_office_ips(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_office_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `match_`.\nWirefilter expression to match a device against when evaluating whether this policy should take effect for that device."]
    pub fn set_match(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `precedence`.\nThe precedence of the policy. Lower values indicate higher precedence."]
    pub fn set_precedence(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().precedence = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_mode_v2_mode`.\nThe service mode. Available values: `1dot1`, `warp`, `proxy`, `posture_only`, `warp_tunnel_only`. Defaults to `warp`."]
    pub fn set_service_mode_v2_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_mode_v2_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_mode_v2_port`.\nThe port to use for the proxy service mode. Required when using `service_mode_v2_mode`."]
    pub fn set_service_mode_v2_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().service_mode_v2_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `support_url`.\nThe support URL that will be opened when sending feedback."]
    pub fn set_support_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().support_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `switch_locked`.\nEnablement of the ZT client switch lock."]
    pub fn set_switch_locked(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().switch_locked = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_mode_switch` after provisioning.\nWhether to allow mode switch for this policy."]
    pub fn allow_mode_switch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_mode_switch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_updates` after provisioning.\nWhether to allow updates under this policy."]
    pub fn allow_updates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_updates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_to_leave` after provisioning.\nWhether to allow devices to leave the organization. Defaults to `true`."]
    pub fn allowed_to_leave(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_to_leave", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_connect` after provisioning.\nThe amount of time in minutes to reconnect after having been disabled."]
    pub fn auto_connect(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_connect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `captive_portal` after provisioning.\nThe captive portal value for this policy. Defaults to `180`."]
    pub fn captive_portal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.captive_portal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nWhether the policy refers to the default account policy."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_auto_fallback` after provisioning.\nWhether to disable auto fallback for this policy."]
    pub fn disable_auto_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_auto_fallback", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the policy is enabled (cannot be set for default policies). Defaults to `true`."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_office_ips` after provisioning.\nWhether to add Microsoft IPs to split tunnel exclusions."]
    pub fn exclude_office_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_office_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nWirefilter expression to match a device against when evaluating whether this policy should take effect for that device."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nThe precedence of the policy. Lower values indicate higher precedence."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_mode_v2_mode` after provisioning.\nThe service mode. Available values: `1dot1`, `warp`, `proxy`, `posture_only`, `warp_tunnel_only`. Defaults to `warp`."]
    pub fn service_mode_v2_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_mode_v2_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_mode_v2_port` after provisioning.\nThe port to use for the proxy service mode. Required when using `service_mode_v2_mode`."]
    pub fn service_mode_v2_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_mode_v2_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `support_url` after provisioning.\nThe support URL that will be opened when sending feedback."]
    pub fn support_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `switch_locked` after provisioning.\nEnablement of the ZT client switch lock."]
    pub fn switch_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.switch_locked", self.extract_ref()))
    }
}

impl terrars::Referable for DeviceSettingsPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DeviceSettingsPolicy { }

impl terrars::ToListMappable for DeviceSettingsPolicy {
    type O = terrars::ListRef<DeviceSettingsPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DeviceSettingsPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_device_settings_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDeviceSettingsPolicy {
    pub tf_id: std::string::String,
    #[doc = "The account identifier to target for the resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the policy."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDeviceSettingsPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DeviceSettingsPolicy {
        let out = DeviceSettingsPolicy(std::rc::Rc::new(DeviceSettingsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DeviceSettingsPolicyData {
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
                default: core::default::Default::default(),
                disable_auto_fallback: core::default::Default::default(),
                enabled: core::default::Default::default(),
                exclude_office_ips: core::default::Default::default(),
                id: core::default::Default::default(),
                match_: core::default::Default::default(),
                name: self.name,
                precedence: core::default::Default::default(),
                service_mode_v2_mode: core::default::Default::default(),
                service_mode_v2_port: core::default::Default::default(),
                support_url: core::default::Default::default(),
                switch_locked: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DeviceSettingsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeviceSettingsPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DeviceSettingsPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_mode_switch` after provisioning.\nWhether to allow mode switch for this policy."]
    pub fn allow_mode_switch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_mode_switch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_updates` after provisioning.\nWhether to allow updates under this policy."]
    pub fn allow_updates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_updates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_to_leave` after provisioning.\nWhether to allow devices to leave the organization. Defaults to `true`."]
    pub fn allowed_to_leave(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_to_leave", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_connect` after provisioning.\nThe amount of time in minutes to reconnect after having been disabled."]
    pub fn auto_connect(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_connect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `captive_portal` after provisioning.\nThe captive portal value for this policy. Defaults to `180`."]
    pub fn captive_portal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.captive_portal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nWhether the policy refers to the default account policy."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_auto_fallback` after provisioning.\nWhether to disable auto fallback for this policy."]
    pub fn disable_auto_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_auto_fallback", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the policy is enabled (cannot be set for default policies). Defaults to `true`."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exclude_office_ips` after provisioning.\nWhether to add Microsoft IPs to split tunnel exclusions."]
    pub fn exclude_office_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_office_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `match_` after provisioning.\nWirefilter expression to match a device against when evaluating whether this policy should take effect for that device."]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `precedence` after provisioning.\nThe precedence of the policy. Lower values indicate higher precedence."]
    pub fn precedence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precedence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_mode_v2_mode` after provisioning.\nThe service mode. Available values: `1dot1`, `warp`, `proxy`, `posture_only`, `warp_tunnel_only`. Defaults to `warp`."]
    pub fn service_mode_v2_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_mode_v2_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_mode_v2_port` after provisioning.\nThe port to use for the proxy service mode. Required when using `service_mode_v2_mode`."]
    pub fn service_mode_v2_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_mode_v2_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `support_url` after provisioning.\nThe support URL that will be opened when sending feedback."]
    pub fn support_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `switch_locked` after provisioning.\nEnablement of the ZT client switch lock."]
    pub fn switch_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.switch_locked", self.extract_ref()))
    }
}
