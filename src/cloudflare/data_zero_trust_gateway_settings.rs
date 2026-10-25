#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustGatewaySettingsData_ {
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

struct DataZeroTrustGatewaySettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustGatewaySettingsData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustGatewaySettings(std::rc::Rc<DataZeroTrustGatewaySettings_>);

impl DataZeroTrustGatewaySettings {
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

    #[doc = "Set the field `account_id`.\nSpecify the Cloudflare account identifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
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
    pub fn settings(&self) -> DataZeroTrustGatewaySettings_SettingsRef {
        DataZeroTrustGatewaySettings_SettingsRef::new(
            self.shared().clone(),
            format!("{}.settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustGatewaySettings {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustGatewaySettings { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings {
    type O = terrars::ListRef<DataZeroTrustGatewaySettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustGatewaySettings_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_gateway_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustGatewaySettings {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustGatewaySettings {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustGatewaySettings {
        let out = DataZeroTrustGatewaySettings(std::rc::Rc::new(DataZeroTrustGatewaySettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustGatewaySettingsData_ {
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

pub struct DataZeroTrustGatewaySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustGatewaySettingsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
    pub fn settings(&self) -> DataZeroTrustGatewaySettings_SettingsRef {
        DataZeroTrustGatewaySettings_SettingsRef::new(
            self.shared().clone(),
            format!("{}.settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewaySettings_Settings_ActivityLog {}

impl DataZeroTrustGatewaySettings_Settings_ActivityLog { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_ActivityLog {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_ActivityLog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_ActivityLog {}

impl BuildDataZeroTrustGatewaySettings_Settings_ActivityLog {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_ActivityLog {
        DataZeroTrustGatewaySettings_Settings_ActivityLog {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_ActivityLogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_ActivityLogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_ActivityLogRef {
        DataZeroTrustGatewaySettings_Settings_ActivityLogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_ActivityLogRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {}

impl DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {}

impl BuildDataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {
        DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettings {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
        DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_Antivirus {}

impl DataZeroTrustGatewaySettings_Settings_Antivirus { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_Antivirus {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_Antivirus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_Antivirus {}

impl BuildDataZeroTrustGatewaySettings_Settings_Antivirus {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_Antivirus {
        DataZeroTrustGatewaySettings_Settings_Antivirus {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_AntivirusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_AntivirusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_AntivirusRef {
        DataZeroTrustGatewaySettings_Settings_AntivirusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_AntivirusRef {
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
    pub fn notification_settings(&self) -> DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef {
        DataZeroTrustGatewaySettings_Settings_Antivirus_NotificationSettingsRef::new(
            self.shared().clone(),
            format!("{}.notification_settings", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewaySettings_Settings_BlockPage {}

impl DataZeroTrustGatewaySettings_Settings_BlockPage { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_BlockPage {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_BlockPage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_BlockPage {}

impl BuildDataZeroTrustGatewaySettings_Settings_BlockPage {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_BlockPage {
        DataZeroTrustGatewaySettings_Settings_BlockPage {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_BlockPageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_BlockPageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_BlockPageRef {
        DataZeroTrustGatewaySettings_Settings_BlockPageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_BlockPageRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_BodyScanning {}

impl DataZeroTrustGatewaySettings_Settings_BodyScanning { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_BodyScanning {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_BodyScanning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_BodyScanning {}

impl BuildDataZeroTrustGatewaySettings_Settings_BodyScanning {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_BodyScanning {
        DataZeroTrustGatewaySettings_Settings_BodyScanning {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_BodyScanningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_BodyScanningRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_BodyScanningRef {
        DataZeroTrustGatewaySettings_Settings_BodyScanningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_BodyScanningRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_BrowserIsolation {}

impl DataZeroTrustGatewaySettings_Settings_BrowserIsolation { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_BrowserIsolation {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_BrowserIsolation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_BrowserIsolation {}

impl BuildDataZeroTrustGatewaySettings_Settings_BrowserIsolation {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_BrowserIsolation {
        DataZeroTrustGatewaySettings_Settings_BrowserIsolation {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
        DataZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_Certificate {}

impl DataZeroTrustGatewaySettings_Settings_Certificate { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_Certificate {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_Certificate {}

impl BuildDataZeroTrustGatewaySettings_Settings_Certificate {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_Certificate {
        DataZeroTrustGatewaySettings_Settings_Certificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_CertificateRef {
        DataZeroTrustGatewaySettings_Settings_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_CertificateRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_CustomCertificate {}

impl DataZeroTrustGatewaySettings_Settings_CustomCertificate { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_CustomCertificate {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_CustomCertificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_CustomCertificate {}

impl BuildDataZeroTrustGatewaySettings_Settings_CustomCertificate {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_CustomCertificate {
        DataZeroTrustGatewaySettings_Settings_CustomCertificate {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_CustomCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_CustomCertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_CustomCertificateRef {
        DataZeroTrustGatewaySettings_Settings_CustomCertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_CustomCertificateRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {}

impl DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatching { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatching>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {}

impl BuildDataZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {
        DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatching {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
        DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_Fips {}

impl DataZeroTrustGatewaySettings_Settings_Fips { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_Fips {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_Fips>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_Fips {}

impl BuildDataZeroTrustGatewaySettings_Settings_Fips {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_Fips {
        DataZeroTrustGatewaySettings_Settings_Fips {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_FipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_FipsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustGatewaySettings_Settings_FipsRef {
        DataZeroTrustGatewaySettings_Settings_FipsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_FipsRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_HostSelector {}

impl DataZeroTrustGatewaySettings_Settings_HostSelector { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_HostSelector {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_HostSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_HostSelector {}

impl BuildDataZeroTrustGatewaySettings_Settings_HostSelector {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_HostSelector {
        DataZeroTrustGatewaySettings_Settings_HostSelector {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_HostSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_HostSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_HostSelectorRef {
        DataZeroTrustGatewaySettings_Settings_HostSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_HostSelectorRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_Inspection {}

impl DataZeroTrustGatewaySettings_Settings_Inspection { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_Inspection {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_Inspection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_Inspection {}

impl BuildDataZeroTrustGatewaySettings_Settings_Inspection {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_Inspection {
        DataZeroTrustGatewaySettings_Settings_Inspection {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_InspectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_InspectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_InspectionRef {
        DataZeroTrustGatewaySettings_Settings_InspectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_InspectionRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_ProtocolDetection {}

impl DataZeroTrustGatewaySettings_Settings_ProtocolDetection { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_ProtocolDetection {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_ProtocolDetection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_ProtocolDetection {}

impl BuildDataZeroTrustGatewaySettings_Settings_ProtocolDetection {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_ProtocolDetection {
        DataZeroTrustGatewaySettings_Settings_ProtocolDetection {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
        DataZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_Sandbox {}

impl DataZeroTrustGatewaySettings_Settings_Sandbox { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_Sandbox {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_Sandbox>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_Sandbox {}

impl BuildDataZeroTrustGatewaySettings_Settings_Sandbox {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_Sandbox {
        DataZeroTrustGatewaySettings_Settings_Sandbox {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_SandboxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_SandboxRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_SandboxRef {
        DataZeroTrustGatewaySettings_Settings_SandboxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_SandboxRef {
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
pub struct DataZeroTrustGatewaySettings_Settings_TlsDecrypt {}

impl DataZeroTrustGatewaySettings_Settings_TlsDecrypt { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings_TlsDecrypt {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings_TlsDecrypt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings_TlsDecrypt {}

impl BuildDataZeroTrustGatewaySettings_Settings_TlsDecrypt {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings_TlsDecrypt {
        DataZeroTrustGatewaySettings_Settings_TlsDecrypt {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_Settings_TlsDecryptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_Settings_TlsDecryptRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewaySettings_Settings_TlsDecryptRef {
        DataZeroTrustGatewaySettings_Settings_TlsDecryptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_Settings_TlsDecryptRef {
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
pub struct DataZeroTrustGatewaySettings_Settings {}

impl DataZeroTrustGatewaySettings_Settings { }

impl terrars::ToListMappable for DataZeroTrustGatewaySettings_Settings {
    type O = terrars::BlockAssignable<DataZeroTrustGatewaySettings_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewaySettings_Settings {}

impl BuildDataZeroTrustGatewaySettings_Settings {
    pub fn build(self) -> DataZeroTrustGatewaySettings_Settings {
        DataZeroTrustGatewaySettings_Settings {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewaySettings_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewaySettings_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustGatewaySettings_SettingsRef {
        DataZeroTrustGatewaySettings_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewaySettings_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activity_log` after provisioning.\nSpecify activity log settings."]
    pub fn activity_log(&self) -> DataZeroTrustGatewaySettings_Settings_ActivityLogRef {
        DataZeroTrustGatewaySettings_Settings_ActivityLogRef::new(
            self.shared().clone(),
            format!("{}.activity_log", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `antivirus` after provisioning.\nSpecify anti-virus settings."]
    pub fn antivirus(&self) -> DataZeroTrustGatewaySettings_Settings_AntivirusRef {
        DataZeroTrustGatewaySettings_Settings_AntivirusRef::new(
            self.shared().clone(),
            format!("{}.antivirus", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `block_page` after provisioning.\nSpecify block page layout settings."]
    pub fn block_page(&self) -> DataZeroTrustGatewaySettings_Settings_BlockPageRef {
        DataZeroTrustGatewaySettings_Settings_BlockPageRef::new(
            self.shared().clone(),
            format!("{}.block_page", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `body_scanning` after provisioning.\nSpecify the DLP inspection mode."]
    pub fn body_scanning(&self) -> DataZeroTrustGatewaySettings_Settings_BodyScanningRef {
        DataZeroTrustGatewaySettings_Settings_BodyScanningRef::new(
            self.shared().clone(),
            format!("{}.body_scanning", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `browser_isolation` after provisioning.\nSpecify Clientless Browser Isolation settings."]
    pub fn browser_isolation(&self) -> DataZeroTrustGatewaySettings_Settings_BrowserIsolationRef {
        DataZeroTrustGatewaySettings_Settings_BrowserIsolationRef::new(
            self.shared().clone(),
            format!("{}.browser_isolation", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `certificate` after provisioning.\nSpecify certificate settings for Gateway TLS interception. If unset, the Cloudflare Root CA handles interception."]
    pub fn certificate(&self) -> DataZeroTrustGatewaySettings_Settings_CertificateRef {
        DataZeroTrustGatewaySettings_Settings_CertificateRef::new(
            self.shared().clone(),
            format!("{}.certificate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `custom_certificate` after provisioning.\nSpecify custom certificate settings for BYO-PKI. This field is deprecated; use `certificate` instead."]
    pub fn custom_certificate(&self) -> DataZeroTrustGatewaySettings_Settings_CustomCertificateRef {
        DataZeroTrustGatewaySettings_Settings_CustomCertificateRef::new(
            self.shared().clone(),
            format!("{}.custom_certificate", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `extended_email_matching` after provisioning.\nConfigures user email settings for firewall policies. When you enable this, the system standardizes email addresses in the identity portion of the rule to match extended email variants in firewall policies. When you disable this setting, the system matches email addresses exactly as you provide them. Enable this setting if your email uses `.` or `+` modifiers."]
    pub fn extended_email_matching(&self) -> DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef {
        DataZeroTrustGatewaySettings_Settings_ExtendedEmailMatchingRef::new(
            self.shared().clone(),
            format!("{}.extended_email_matching", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `fips` after provisioning.\nSpecify FIPS settings."]
    pub fn fips(&self) -> DataZeroTrustGatewaySettings_Settings_FipsRef {
        DataZeroTrustGatewaySettings_Settings_FipsRef::new(self.shared().clone(), format!("{}.fips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_selector` after provisioning.\nEnable host selection in egress policies."]
    pub fn host_selector(&self) -> DataZeroTrustGatewaySettings_Settings_HostSelectorRef {
        DataZeroTrustGatewaySettings_Settings_HostSelectorRef::new(
            self.shared().clone(),
            format!("{}.host_selector", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `inspection` after provisioning.\nDefine the proxy inspection mode."]
    pub fn inspection(&self) -> DataZeroTrustGatewaySettings_Settings_InspectionRef {
        DataZeroTrustGatewaySettings_Settings_InspectionRef::new(
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
    pub fn protocol_detection(&self) -> DataZeroTrustGatewaySettings_Settings_ProtocolDetectionRef {
        DataZeroTrustGatewaySettings_Settings_ProtocolDetectionRef::new(
            self.shared().clone(),
            format!("{}.protocol_detection", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `sandbox` after provisioning.\nSpecify whether to enable the sandbox."]
    pub fn sandbox(&self) -> DataZeroTrustGatewaySettings_Settings_SandboxRef {
        DataZeroTrustGatewaySettings_Settings_SandboxRef::new(
            self.shared().clone(),
            format!("{}.sandbox", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `tls_decrypt` after provisioning.\nSpecify whether to inspect encrypted HTTP traffic."]
    pub fn tls_decrypt(&self) -> DataZeroTrustGatewaySettings_Settings_TlsDecryptRef {
        DataZeroTrustGatewaySettings_Settings_TlsDecryptRef::new(
            self.shared().clone(),
            format!("{}.tls_decrypt", self.base),
        )
    }
}
