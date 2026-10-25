#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustGatewaySettingsData_ {
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
    settings: std::option::Option<ZeroTrustGatewaySettings_Settings>,
}

struct ZeroTrustGatewaySettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustGatewaySettingsData_>,
}

#[derive(Clone)]
pub struct ZeroTrustGatewaySettings(std::rc::Rc<ZeroTrustGatewaySettings_>);

impl ZeroTrustGatewaySettings {
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

    #[doc = "Set the field `settings`.\nSpecify account settings."]
    pub fn set_settings(self, v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings>) -> Self {
        self.0.data.borrow_mut().settings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\nSpecify account settings."]
    pub fn settings(&self) -> ZeroTrustGatewaySettings_SettingsRef {
        ZeroTrustGatewaySettings_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustGatewaySettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustGatewaySettings { }

impl terrars::ToListMappable for ZeroTrustGatewaySettings {
    type O = terrars::ListRef<ZeroTrustGatewaySettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustGatewaySettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_gateway_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustGatewaySettings {
    pub tf_id: std::string::String,
    #[doc = "Specify the Cloudflare account identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustGatewaySettings {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustGatewaySettings {
        let out = ZeroTrustGatewaySettings(std::rc::Rc::new(ZeroTrustGatewaySettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustGatewaySettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                settings: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustGatewaySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustGatewaySettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nSpecify the Cloudflare account identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\nSpecify account settings."]
    pub fn settings(&self) -> ZeroTrustGatewaySettings_SettingsRef {
        ZeroTrustGatewaySettings_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_ActivityLog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewaySettings_Settings_ActivityLog {
    #[doc = "Set the field `enabled`.\nSpecify whether to log activity."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_ActivityLog {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_ActivityLog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_ActivityLog {}

impl BuildZeroTrustGatewaySettings_Settings_ActivityLog {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_ActivityLog {
        ZeroTrustGatewaySettings_Settings_ActivityLog { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_ActivityLogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_ActivityLogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_ActivityLogRef {
        ZeroTrustGatewaySettings_Settings_ActivityLogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_ActivityLogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether to log activity."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_context: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    msg: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {
    #[doc = "Set the field `enabled`.\nSpecify whether to enable notifications."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_context`.\nSpecify whether to include context information as query parameters."]
    pub fn set_include_context(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `msg`.\nSpecify the message to show in the notification."]
    pub fn set_msg(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.msg = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `support_url`.\nSpecify a URL that directs users to more information. If unset, the notification opens a block page."]
    pub fn set_support_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.support_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {}

impl BuildZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {
        ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {
            enabled: core::default::Default::default(),
            include_context: core::default::Default::default(),
            msg: core::default::Default::default(),
            support_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
        ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether to enable notifications."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_context` after provisioning.\nSpecify whether to include context information as query parameters."]
    pub fn include_context(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_context", self.base))
    }

    #[doc =
        "Get a reference to the value of field `msg` after provisioning.\nSpecify the message to show in the notification."]
    pub fn msg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.msg", self.base))
    }

    #[doc =
        "Get a reference to the value of field `support_url` after provisioning.\nSpecify a URL that directs users to more information. If unset, the notification opens a block page."]
    pub fn support_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_url", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_Antivirus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_download_phase: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_upload_phase: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_closed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_settings: std::option::Option<ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings>,
}

impl ZeroTrustGatewaySettings_Settings_Antivirus {
    #[doc = "Set the field `enabled_download_phase`.\nSpecify whether to enable anti-virus scanning on downloads."]
    pub fn set_enabled_download_phase(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled_download_phase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_upload_phase`.\nSpecify whether to enable anti-virus scanning on uploads."]
    pub fn set_enabled_upload_phase(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled_upload_phase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_closed`.\nSpecify whether to block requests for unscannable files."]
    pub fn set_fail_closed(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_closed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notification_settings`.\nConfigure the message the user's device shows during an antivirus scan."]
    pub fn set_notification_settings(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings>,
    ) -> Self {
        self.notification_settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_Antivirus {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_Antivirus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_Antivirus {}

impl BuildZeroTrustGatewaySettings_Settings_Antivirus {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_Antivirus {
        ZeroTrustGatewaySettings_Settings_Antivirus {
            enabled_download_phase: core::default::Default::default(),
            enabled_upload_phase: core::default::Default::default(),
            fail_closed: core::default::Default::default(),
            notification_settings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_AntivirusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_AntivirusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_AntivirusRef {
        ZeroTrustGatewaySettings_Settings_AntivirusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_AntivirusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled_download_phase` after provisioning.\nSpecify whether to enable anti-virus scanning on downloads."]
    pub fn enabled_download_phase(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_download_phase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled_upload_phase` after provisioning.\nSpecify whether to enable anti-virus scanning on uploads."]
    pub fn enabled_upload_phase(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_upload_phase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fail_closed` after provisioning.\nSpecify whether to block requests for unscannable files."]
    pub fn fail_closed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_closed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `notification_settings` after provisioning.\nConfigure the message the user's device shows during an antivirus scan."]
    pub fn notification_settings(&self) -> ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
        ZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef::new(
            self.shared().clone(),
            format!("{}.notification_settings", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_BlockPage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    background_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    footer_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_context: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logo_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mailto_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mailto_subject: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suppress_footer: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<f64>>,
}

impl ZeroTrustGatewaySettings_Settings_BlockPage {
    #[doc =
        "Set the field `background_color`.\nSpecify the block page background color in `#rrggbb` format when the mode is customized_block_page."]
    pub fn set_background_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.background_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nSpecify whether to enable the custom block page."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `footer_text`.\nSpecify the block page footer text when the mode is customized_block_page."]
    pub fn set_footer_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.footer_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_text`.\nSpecify the block page header text when the mode is customized_block_page."]
    pub fn set_header_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_text = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_context`.\nSpecify whether to append context to target_uri as query parameters. This applies only when the mode is redirect_uri."]
    pub fn set_include_context(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logo_path`.\nSpecify the full URL to the logo file when the mode is customized_block_page."]
    pub fn set_logo_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logo_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mailto_address`.\nSpecify the admin email for users to contact when the mode is customized_block_page."]
    pub fn set_mailto_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mailto_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mailto_subject`.\nSpecify the subject line for emails created from the block page when the mode is customized_block_page."]
    pub fn set_mailto_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mailto_subject = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nSpecify whether to redirect users to a Cloudflare-hosted block page or a customer-provided URI.\nAvailable values: \"\", \"customized_block_page\", \"redirect_uri\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nSpecify the block page title when the mode is customized_block_page."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `read_only`.\nIndicate that this setting was shared via the Orgs API and read only for the current account."]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_account`.\nIndicate the account tag of the account that shared this setting."]
    pub fn set_source_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suppress_footer`.\nSpecify whether to suppress detailed information at the bottom of the block page when the mode is customized_block_page."]
    pub fn set_suppress_footer(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.suppress_footer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_uri`.\nSpecify the URI to redirect users to when the mode is redirect_uri."]
    pub fn set_target_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nIndicate the version number of the setting."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_BlockPage {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_BlockPage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_BlockPage {}

impl BuildZeroTrustGatewaySettings_Settings_BlockPage {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_BlockPage {
        ZeroTrustGatewaySettings_Settings_BlockPage {
            background_color: core::default::Default::default(),
            enabled: core::default::Default::default(),
            footer_text: core::default::Default::default(),
            header_text: core::default::Default::default(),
            include_context: core::default::Default::default(),
            logo_path: core::default::Default::default(),
            mailto_address: core::default::Default::default(),
            mailto_subject: core::default::Default::default(),
            mode: core::default::Default::default(),
            name: core::default::Default::default(),
            read_only: core::default::Default::default(),
            source_account: core::default::Default::default(),
            suppress_footer: core::default::Default::default(),
            target_uri: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_BlockPageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_BlockPageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_BlockPageRef {
        ZeroTrustGatewaySettings_Settings_BlockPageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_BlockPageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `background_color` after provisioning.\nSpecify the block page background color in `#rrggbb` format when the mode is customized_block_page."]
    pub fn background_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.background_color", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether to enable the custom block page."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `footer_text` after provisioning.\nSpecify the block page footer text when the mode is customized_block_page."]
    pub fn footer_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.footer_text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_text` after provisioning.\nSpecify the block page header text when the mode is customized_block_page."]
    pub fn header_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_context` after provisioning.\nSpecify whether to append context to target_uri as query parameters. This applies only when the mode is redirect_uri."]
    pub fn include_context(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_context", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logo_path` after provisioning.\nSpecify the full URL to the logo file when the mode is customized_block_page."]
    pub fn logo_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mailto_address` after provisioning.\nSpecify the admin email for users to contact when the mode is customized_block_page."]
    pub fn mailto_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mailto_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mailto_subject` after provisioning.\nSpecify the subject line for emails created from the block page when the mode is customized_block_page."]
    pub fn mailto_subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mailto_subject", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSpecify whether to redirect users to a Cloudflare-hosted block page or a customer-provided URI.\nAvailable values: \"\", \"customized_block_page\", \"redirect_uri\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSpecify the block page title when the mode is customized_block_page."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIndicate that this setting was shared via the Orgs API and read only for the current account."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_account` after provisioning.\nIndicate the account tag of the account that shared this setting."]
    pub fn source_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suppress_footer` after provisioning.\nSpecify whether to suppress detailed information at the bottom of the block page when the mode is customized_block_page."]
    pub fn suppress_footer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suppress_footer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_uri` after provisioning.\nSpecify the URI to redirect users to when the mode is redirect_uri."]
    pub fn target_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIndicate the version number of the setting."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_BodyScanning {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspection_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewaySettings_Settings_BodyScanning {
    #[doc =
        "Set the field `inspection_mode`.\nSpecify the inspection mode as either `deep` or `shallow`.\nAvailable values: \"deep\", \"shallow\"."]
    pub fn set_inspection_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inspection_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_BodyScanning {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_BodyScanning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_BodyScanning {}

impl BuildZeroTrustGatewaySettings_Settings_BodyScanning {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_BodyScanning {
        ZeroTrustGatewaySettings_Settings_BodyScanning { inspection_mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_BodyScanningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_BodyScanningRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_BodyScanningRef {
        ZeroTrustGatewaySettings_Settings_BodyScanningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_BodyScanningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `inspection_mode` after provisioning.\nSpecify the inspection mode as either `deep` or `shallow`.\nAvailable values: \"deep\", \"shallow\"."]
    pub fn inspection_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspection_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_BrowserIsolation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_identity_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_browser_isolation_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewaySettings_Settings_BrowserIsolation {
    #[doc =
        "Set the field `non_identity_enabled`.\nSpecify whether to enable non-identity onramp support for Browser Isolation."]
    pub fn set_non_identity_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.non_identity_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url_browser_isolation_enabled`.\nSpecify whether to enable Clientless Browser Isolation."]
    pub fn set_url_browser_isolation_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.url_browser_isolation_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_BrowserIsolation {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_BrowserIsolation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_BrowserIsolation {}

impl BuildZeroTrustGatewaySettings_Settings_BrowserIsolation {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_BrowserIsolation {
        ZeroTrustGatewaySettings_Settings_BrowserIsolation {
            non_identity_enabled: core::default::Default::default(),
            url_browser_isolation_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
        ZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `non_identity_enabled` after provisioning.\nSpecify whether to enable non-identity onramp support for Browser Isolation."]
    pub fn non_identity_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.non_identity_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url_browser_isolation_enabled` after provisioning.\nSpecify whether to enable Clientless Browser Isolation."]
    pub fn url_browser_isolation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_browser_isolation_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_Certificate {
    id: terrars::PrimField<std::string::String>,
}

impl ZeroTrustGatewaySettings_Settings_Certificate { }

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_Certificate {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_Certificate {
    #[doc =
        "Specify the UUID of the certificate used for interception. Ensure the certificate is available at the edge(previously called 'active'). A nil UUID directs Cloudflare to use the Root CA."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustGatewaySettings_Settings_Certificate {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_Certificate {
        ZeroTrustGatewaySettings_Settings_Certificate { id: self.id }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_CertificateRef {
        ZeroTrustGatewaySettings_Settings_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nSpecify the UUID of the certificate used for interception. Ensure the certificate is available at the edge(previously called 'active'). A nil UUID directs Cloudflare to use the Root CA."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_CustomCertificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binding_status: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewaySettings_Settings_CustomCertificate {
    #[doc = "Set the field `binding_status`.\nIndicate the internal certificate status."]
    pub fn set_binding_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.binding_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nSpecify the UUID of the certificate (ID from MTLS certificate store)."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_CustomCertificate {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_CustomCertificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_CustomCertificate {
    #[doc = "Specify whether to enable a custom certificate authority for signing Gateway traffic."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildZeroTrustGatewaySettings_Settings_CustomCertificate {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_CustomCertificate {
        ZeroTrustGatewaySettings_Settings_CustomCertificate {
            binding_status: core::default::Default::default(),
            enabled: self.enabled,
            id: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_CustomCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_CustomCertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_CustomCertificateRef {
        ZeroTrustGatewaySettings_Settings_CustomCertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_CustomCertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `binding_status` after provisioning.\nIndicate the internal certificate status."]
    pub fn binding_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.binding_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether to enable a custom certificate authority for signing Gateway traffic."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nSpecify the UUID of the certificate (ID from MTLS certificate store)."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {
    #[doc =
        "Set the field `enabled`.\nSpecify whether to match all variants of user emails (with + or . modifiers) used as criteria in Firewall policies."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_ExtendedEmailMatching>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {}

impl BuildZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {
        ZeroTrustGatewaySettings_Settings_ExtendedEmailMatching { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
        ZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether to match all variants of user emails (with + or . modifiers) used as criteria in Firewall policies."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIndicate that this setting was shared via the Orgs API and read only for the current account."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_account` after provisioning.\nIndicate the account tag of the account that shared this setting."]
    pub fn source_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIndicate the version number of the setting."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_Fips {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewaySettings_Settings_Fips {
    #[doc = "Set the field `tls`.\nEnforce cipher suites and TLS versions compliant with FIPS 140-2."]
    pub fn set_tls(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.tls = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_Fips {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_Fips>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_Fips {}

impl BuildZeroTrustGatewaySettings_Settings_Fips {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_Fips {
        ZeroTrustGatewaySettings_Settings_Fips { tls: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_FipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_FipsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustGatewaySettings_Settings_FipsRef {
        ZeroTrustGatewaySettings_Settings_FipsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_FipsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tls` after provisioning.\nEnforce cipher suites and TLS versions compliant with FIPS 140-2."]
    pub fn tls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_HostSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewaySettings_Settings_HostSelector {
    #[doc = "Set the field `enabled`.\nSpecify whether to enable filtering via hosts for egress policies."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_HostSelector {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_HostSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_HostSelector {}

impl BuildZeroTrustGatewaySettings_Settings_HostSelector {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_HostSelector {
        ZeroTrustGatewaySettings_Settings_HostSelector { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_HostSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_HostSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_HostSelectorRef {
        ZeroTrustGatewaySettings_Settings_HostSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_HostSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether to enable filtering via hosts for egress policies."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_Inspection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewaySettings_Settings_Inspection {
    #[doc =
        "Set the field `mode`.\nDefine the proxy inspection mode.   1. static: Gateway applies static inspection to HTTP on TCP(80). With TLS decryption on, Gateway inspects HTTPS traffic on TCP(443) and UDP(443).   2. dynamic: Gateway applies protocol detection to inspect HTTP and HTTPS traffic on any port. TLS decryption must remain on to inspect HTTPS traffic.\nAvailable values: \"static\", \"dynamic\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_Inspection {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_Inspection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_Inspection {}

impl BuildZeroTrustGatewaySettings_Settings_Inspection {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_Inspection {
        ZeroTrustGatewaySettings_Settings_Inspection { mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_InspectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_InspectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_InspectionRef {
        ZeroTrustGatewaySettings_Settings_InspectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_InspectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nDefine the proxy inspection mode.   1. static: Gateway applies static inspection to HTTP on TCP(80). With TLS decryption on, Gateway inspects HTTPS traffic on TCP(443) and UDP(443).   2. dynamic: Gateway applies protocol detection to inspect HTTP and HTTPS traffic on any port. TLS decryption must remain on to inspect HTTPS traffic.\nAvailable values: \"static\", \"dynamic\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_ProtocolDetection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewaySettings_Settings_ProtocolDetection {
    #[doc = "Set the field `enabled`.\nSpecify whether to detect protocols from the initial bytes of client traffic."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_ProtocolDetection {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_ProtocolDetection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_ProtocolDetection {}

impl BuildZeroTrustGatewaySettings_Settings_ProtocolDetection {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_ProtocolDetection {
        ZeroTrustGatewaySettings_Settings_ProtocolDetection { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
        ZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether to detect protocols from the initial bytes of client traffic."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_Sandbox {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback_action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZeroTrustGatewaySettings_Settings_Sandbox {
    #[doc = "Set the field `enabled`.\nSpecify whether to enable the sandbox."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fallback_action`.\nSpecify the action to take when the system cannot scan the file.\nAvailable values: \"allow\", \"block\"."]
    pub fn set_fallback_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fallback_action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_Sandbox {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_Sandbox>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_Sandbox {}

impl BuildZeroTrustGatewaySettings_Settings_Sandbox {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_Sandbox {
        ZeroTrustGatewaySettings_Settings_Sandbox {
            enabled: core::default::Default::default(),
            fallback_action: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_SandboxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_SandboxRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustGatewaySettings_Settings_SandboxRef {
        ZeroTrustGatewaySettings_Settings_SandboxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_SandboxRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether to enable the sandbox."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fallback_action` after provisioning.\nSpecify the action to take when the system cannot scan the file.\nAvailable values: \"allow\", \"block\"."]
    pub fn fallback_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_action", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings_TlsDecrypt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ZeroTrustGatewaySettings_Settings_TlsDecrypt {
    #[doc = "Set the field `enabled`.\nSpecify whether to inspect encrypted HTTP traffic."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings_TlsDecrypt {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings_TlsDecrypt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings_TlsDecrypt {}

impl BuildZeroTrustGatewaySettings_Settings_TlsDecrypt {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings_TlsDecrypt {
        ZeroTrustGatewaySettings_Settings_TlsDecrypt { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_Settings_TlsDecryptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_Settings_TlsDecryptRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZeroTrustGatewaySettings_Settings_TlsDecryptRef {
        ZeroTrustGatewaySettings_Settings_TlsDecryptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_Settings_TlsDecryptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nSpecify whether to inspect encrypted HTTP traffic."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustGatewaySettings_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activity_log: std::option::Option<ZeroTrustGatewaySettings_Settings_ActivityLog>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    antivirus: std::option::Option<ZeroTrustGatewaySettings_Settings_Antivirus>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_page: std::option::Option<ZeroTrustGatewaySettings_Settings_BlockPage>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body_scanning: std::option::Option<ZeroTrustGatewaySettings_Settings_BodyScanning>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_isolation: std::option::Option<ZeroTrustGatewaySettings_Settings_BrowserIsolation>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<ZeroTrustGatewaySettings_Settings_Certificate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_certificate: std::option::Option<ZeroTrustGatewaySettings_Settings_CustomCertificate>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_email_matching: std::option::Option<ZeroTrustGatewaySettings_Settings_ExtendedEmailMatching>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fips: std::option::Option<ZeroTrustGatewaySettings_Settings_Fips>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_selector: std::option::Option<ZeroTrustGatewaySettings_Settings_HostSelector>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspection: std::option::Option<ZeroTrustGatewaySettings_Settings_Inspection>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_ttl_secs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol_detection: std::option::Option<ZeroTrustGatewaySettings_Settings_ProtocolDetection>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sandbox: std::option::Option<ZeroTrustGatewaySettings_Settings_Sandbox>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_decrypt: std::option::Option<ZeroTrustGatewaySettings_Settings_TlsDecrypt>,
}

impl ZeroTrustGatewaySettings_Settings {
    #[doc = "Set the field `activity_log`.\nSpecify activity log settings."]
    pub fn set_activity_log(mut self, v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_ActivityLog>) -> Self {
        self.activity_log = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `antivirus`.\nSpecify anti-virus settings."]
    pub fn set_antivirus(mut self, v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_Antivirus>) -> Self {
        self.antivirus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_page`.\nSpecify block page layout settings."]
    pub fn set_block_page(mut self, v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_BlockPage>) -> Self {
        self.block_page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `body_scanning`.\nSpecify the DLP inspection mode."]
    pub fn set_body_scanning(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_BodyScanning>,
    ) -> Self {
        self.body_scanning = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `browser_isolation`.\nSpecify Clientless Browser Isolation settings."]
    pub fn set_browser_isolation(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_BrowserIsolation>,
    ) -> Self {
        self.browser_isolation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `certificate`.\nSpecify certificate settings for Gateway TLS interception. If unset, the Cloudflare Root CA handles interception."]
    pub fn set_certificate(mut self, v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_Certificate>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_certificate`.\nSpecify custom certificate settings for BYO-PKI. This field is deprecated; use `certificate` instead."]
    pub fn set_custom_certificate(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_CustomCertificate>,
    ) -> Self {
        self.custom_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `extended_email_matching`.\nConfigures user email settings for firewall policies. When you enable this, the system standardizes email addresses in the identity portion of the rule to match extended email variants in firewall policies. When you disable this setting, the system matches email addresses exactly as you provide them. Enable this setting if your email uses `.` or `+` modifiers."]
    pub fn set_extended_email_matching(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_ExtendedEmailMatching>,
    ) -> Self {
        self.extended_email_matching = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fips`.\nSpecify FIPS settings."]
    pub fn set_fips(mut self, v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_Fips>) -> Self {
        self.fips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_selector`.\nEnable host selection in egress policies."]
    pub fn set_host_selector(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_HostSelector>,
    ) -> Self {
        self.host_selector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inspection`.\nDefine the proxy inspection mode."]
    pub fn set_inspection(mut self, v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_Inspection>) -> Self {
        self.inspection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_ttl_secs`.\nAccount-level cap on DNS response TTLs, in seconds. Gateway rewrites DNS responses so returned record TTLs do not exceed this value. Null means no cap. Each DNS location can inherit, override, or disable it through the location `max_ttl` setting."]
    pub fn set_max_ttl_secs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_ttl_secs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `protocol_detection`.\nSpecify whether to detect protocols from the initial bytes of client traffic."]
    pub fn set_protocol_detection(
        mut self,
        v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_ProtocolDetection>,
    ) -> Self {
        self.protocol_detection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sandbox`.\nSpecify whether to enable the sandbox."]
    pub fn set_sandbox(mut self, v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_Sandbox>) -> Self {
        self.sandbox = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_decrypt`.\nSpecify whether to inspect encrypted HTTP traffic."]
    pub fn set_tls_decrypt(mut self, v: impl std::convert::Into<ZeroTrustGatewaySettings_Settings_TlsDecrypt>) -> Self {
        self.tls_decrypt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZeroTrustGatewaySettings_Settings {
    type O = terrars::BlockAssignable<ZeroTrustGatewaySettings_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustGatewaySettings_Settings {}

impl BuildZeroTrustGatewaySettings_Settings {
    pub fn build(self) -> ZeroTrustGatewaySettings_Settings {
        ZeroTrustGatewaySettings_Settings {
            activity_log: core::default::Default::default(),
            antivirus: core::default::Default::default(),
            block_page: core::default::Default::default(),
            body_scanning: core::default::Default::default(),
            browser_isolation: core::default::Default::default(),
            certificate: core::default::Default::default(),
            custom_certificate: core::default::Default::default(),
            extended_email_matching: core::default::Default::default(),
            fips: core::default::Default::default(),
            host_selector: core::default::Default::default(),
            inspection: core::default::Default::default(),
            max_ttl_secs: core::default::Default::default(),
            protocol_detection: core::default::Default::default(),
            sandbox: core::default::Default::default(),
            tls_decrypt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZeroTrustGatewaySettings_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustGatewaySettings_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustGatewaySettings_SettingsRef {
        ZeroTrustGatewaySettings_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustGatewaySettings_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activity_log` after provisioning.\nSpecify activity log settings."]
    pub fn activity_log(&self) -> ZeroTrustGatewaySettings_Settings_ActivityLogRef {
        ZeroTrustGatewaySettings_Settings_ActivityLogRef::new(
            self.shared().clone(),
            format!("{}.activity_log", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `antivirus` after provisioning.\nSpecify anti-virus settings."]
    pub fn antivirus(&self) -> ZeroTrustGatewaySettings_Settings_AntivirusRef {
        ZeroTrustGatewaySettings_Settings_AntivirusRef::new(
            self.shared().clone(),
            format!("{}.antivirus", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `block_page` after provisioning.\nSpecify block page layout settings."]
    pub fn block_page(&self) -> ZeroTrustGatewaySettings_Settings_BlockPageRef {
        ZeroTrustGatewaySettings_Settings_BlockPageRef::new(
            self.shared().clone(),
            format!("{}.block_page", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `body_scanning` after provisioning.\nSpecify the DLP inspection mode."]
    pub fn body_scanning(&self) -> ZeroTrustGatewaySettings_Settings_BodyScanningRef {
        ZeroTrustGatewaySettings_Settings_BodyScanningRef::new(
            self.shared().clone(),
            format!("{}.body_scanning", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `browser_isolation` after provisioning.\nSpecify Clientless Browser Isolation settings."]
    pub fn browser_isolation(&self) -> ZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
        ZeroTrustGatewaySettings_Settings_BrowserIsolationRef::new(
            self.shared().clone(),
            format!("{}.browser_isolation", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `certificate` after provisioning.\nSpecify certificate settings for Gateway TLS interception. If unset, the Cloudflare Root CA handles interception."]
    pub fn certificate(&self) -> ZeroTrustGatewaySettings_Settings_CertificateRef {
        ZeroTrustGatewaySettings_Settings_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `custom_certificate` after provisioning.\nSpecify custom certificate settings for BYO-PKI. This field is deprecated; use `certificate` instead."]
    pub fn custom_certificate(&self) -> ZeroTrustGatewaySettings_Settings_CustomCertificateRef {
        ZeroTrustGatewaySettings_Settings_CustomCertificateRef::new(
            self.shared().clone(),
            format!("{}.custom_certificate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `extended_email_matching` after provisioning.\nConfigures user email settings for firewall policies. When you enable this, the system standardizes email addresses in the identity portion of the rule to match extended email variants in firewall policies. When you disable this setting, the system matches email addresses exactly as you provide them. Enable this setting if your email uses `.` or `+` modifiers."]
    pub fn extended_email_matching(&self) -> ZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
        ZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef::new(
            self.shared().clone(),
            format!("{}.extended_email_matching", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `fips` after provisioning.\nSpecify FIPS settings."]
    pub fn fips(&self) -> ZeroTrustGatewaySettings_Settings_FipsRef {
        ZeroTrustGatewaySettings_Settings_FipsRef::new(self.shared().clone(), format!("{}.fips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_selector` after provisioning.\nEnable host selection in egress policies."]
    pub fn host_selector(&self) -> ZeroTrustGatewaySettings_Settings_HostSelectorRef {
        ZeroTrustGatewaySettings_Settings_HostSelectorRef::new(
            self.shared().clone(),
            format!("{}.host_selector", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `inspection` after provisioning.\nDefine the proxy inspection mode."]
    pub fn inspection(&self) -> ZeroTrustGatewaySettings_Settings_InspectionRef {
        ZeroTrustGatewaySettings_Settings_InspectionRef::new(
            self.shared().clone(),
            format!("{}.inspection", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `max_ttl_secs` after provisioning.\nAccount-level cap on DNS response TTLs, in seconds. Gateway rewrites DNS responses so returned record TTLs do not exceed this value. Null means no cap. Each DNS location can inherit, override, or disable it through the location `max_ttl` setting."]
    pub fn max_ttl_secs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ttl_secs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol_detection` after provisioning.\nSpecify whether to detect protocols from the initial bytes of client traffic."]
    pub fn protocol_detection(&self) -> ZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
        ZeroTrustGatewaySettings_Settings_ProtocolDetectionRef::new(
            self.shared().clone(),
            format!("{}.protocol_detection", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `sandbox` after provisioning.\nSpecify whether to enable the sandbox."]
    pub fn sandbox(&self) -> ZeroTrustGatewaySettings_Settings_SandboxRef {
        ZeroTrustGatewaySettings_Settings_SandboxRef::new(self.shared().clone(), format!("{}.sandbox", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_decrypt` after provisioning.\nSpecify whether to inspect encrypted HTTP traffic."]
    pub fn tls_decrypt(&self) -> ZeroTrustGatewaySettings_Settings_TlsDecryptRef {
        ZeroTrustGatewaySettings_Settings_TlsDecryptRef::new(
            self.shared().clone(),
            format!("{}.tls_decrypt", self.base),
        )
    }
}
