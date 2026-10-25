#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataBotManagementData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBotManagement_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBotManagementData_>,
}

#[derive(Clone)]
pub struct DataBotManagement(std::rc::Rc<DataBotManagement_>);

impl DataBotManagement {
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

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `ai_bots_migration_opt_out` after provisioning.\nTemporary migration flag tracking zones opted out of AI bots managed-rule updates."]
    pub fn ai_bots_migration_opt_out(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_bots_migration_opt_out", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ai_bots_protection` after provisioning.\nEnable rule to block AI Scrapers and Crawlers.\nAvailable values: \"block\", \"disabled\", \"only_on_ad_pages\"."]
    pub fn ai_bots_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_bots_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ai_training` after provisioning.\nConfigure robots.txt policy for AI model training bots.\nAvailable values: \"disabled\", \"disallow\", \"block\", \"only_on_ad_pages\"."]
    pub fn ai_training(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_training", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ai_user` after provisioning.\nConfigure robots.txt policy for AI assistant and agent bots.\nAvailable values: \"disabled\", \"block\", \"only_on_ad_pages\"."]
    pub fn ai_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `aisearch` after provisioning.\nConfigure robots.txt policy for AI search bots.\nAvailable values: \"disabled\", \"block\", \"only_on_ad_pages\"."]
    pub fn aisearch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aisearch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_update_model` after provisioning.\nAutomatically update to the newest bot detection models created by Cloudflare as they are released. [Learn more.](https://developers.cloudflare.com/bots/reference/machine-learning-models#model-versions-and-release-notes)"]
    pub fn auto_update_model(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_update_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bm_cookie_enabled` after provisioning.\nIndicates that the bot management cookie can be placed on end user devices accessing the site. Defaults to true"]
    pub fn bm_cookie_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bm_cookie_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bot_preference_sync_enabled` after provisioning.\nEnable Bot Preference Sync for this zone. When enabled, Cloudflare can serve robots.txt content derived from the zone's AI Search, AI User, and AI Training preferences."]
    pub fn bot_preference_sync_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_preference_sync_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cf_robots_variant` after provisioning.\nSpecifies the Robots Access Control License variant to use.\nAvailable values: \"off\", \"policy_only\"."]
    pub fn cf_robots_variant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cf_robots_variant", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_bots_protection` after provisioning.\nEnable rule to block content bots. When enabled, blocks automated traffic with low bot scores, excluding safe verified bot categories. Exceptions should be managed via skip rules.\nAvailable values: \"block\", \"disabled\"."]
    pub fn content_bots_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_bots_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crawler_protection` after provisioning.\nEnable rule to punish AI Scrapers and Crawlers via a link maze.\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn crawler_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawler_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_js` after provisioning.\nUse lightweight, invisible JavaScript detections to improve Bot Management. [Learn more about JavaScript Detections](https://developers.cloudflare.com/bots/reference/javascript-detections/)."]
    pub fn enable_js(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_js", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fight_mode` after provisioning.\nWhether to enable Bot Fight Mode."]
    pub fn fight_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fight_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_robots_txt_managed` after provisioning.\nEnable cloudflare managed robots.txt. If an existing robots.txt is detected, then managed robots.txt will be prepended to the existing robots.txt."]
    pub fn is_robots_txt_managed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_robots_txt_managed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jsd_api_results_enabled` after provisioning.\nWhether to use JavaScript Detection results submitted through the API for this zone."]
    pub fn jsd_api_results_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jsd_api_results_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `optimize_wordpress` after provisioning.\nWhether to optimize Super Bot Fight Mode protections for Wordpress."]
    pub fn optimize_wordpress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optimize_wordpress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sbfm_definitely_automated` after provisioning.\nSuper Bot Fight Mode (SBFM) action to take on definitely automated requests.\nAvailable values: \"allow\", \"block\", \"managed_challenge\"."]
    pub fn sbfm_definitely_automated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbfm_definitely_automated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sbfm_likely_automated` after provisioning.\nSuper Bot Fight Mode (SBFM) action to take on likely automated requests.\nAvailable values: \"allow\", \"block\", \"managed_challenge\"."]
    pub fn sbfm_likely_automated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbfm_likely_automated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sbfm_static_resource_protection` after provisioning.\nSuper Bot Fight Mode (SBFM) to enable static resource protection.\nEnable if static resources on your application need bot protection.\nNote: Static resource protection can also result in legitimate traffic being blocked."]
    pub fn sbfm_static_resource_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.sbfm_static_resource_protection", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `sbfm_verified_bots` after provisioning.\nSuper Bot Fight Mode (SBFM) action to take on verified bots requests.\nAvailable values: \"allow\", \"block\"."]
    pub fn sbfm_verified_bots(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbfm_verified_bots", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stale_zone_configuration` after provisioning.\nA read-only field that shows which unauthorized settings are currently active on the zone. These settings typically result from upgrades or downgrades."]
    pub fn stale_zone_configuration(&self) -> DataBotManagement_StaleZoneConfigurationRef {
        DataBotManagement_StaleZoneConfigurationRef::new(
            self.shared().clone(),
            format!("{}.stale_zone_configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `suppress_session_score` after provisioning.\nWhether to disable tracking the highest bot score for a session in the Bot Management cookie."]
    pub fn suppress_session_score(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suppress_session_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `using_latest_model` after provisioning.\nA read-only field that indicates whether the zone currently is running the latest ML model."]
    pub fn using_latest_model(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.using_latest_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataBotManagement {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBotManagement { }

impl terrars::ToListMappable for DataBotManagement {
    type O = terrars::ListRef<DataBotManagementRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBotManagement_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_bot_management".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBotManagement {
    pub tf_id: std::string::String,
}

impl BuildDataBotManagement {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBotManagement {
        let out = DataBotManagement(std::rc::Rc::new(DataBotManagement_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBotManagementData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBotManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBotManagementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBotManagementRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `ai_bots_migration_opt_out` after provisioning.\nTemporary migration flag tracking zones opted out of AI bots managed-rule updates."]
    pub fn ai_bots_migration_opt_out(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_bots_migration_opt_out", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ai_bots_protection` after provisioning.\nEnable rule to block AI Scrapers and Crawlers.\nAvailable values: \"block\", \"disabled\", \"only_on_ad_pages\"."]
    pub fn ai_bots_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_bots_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ai_training` after provisioning.\nConfigure robots.txt policy for AI model training bots.\nAvailable values: \"disabled\", \"disallow\", \"block\", \"only_on_ad_pages\"."]
    pub fn ai_training(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_training", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ai_user` after provisioning.\nConfigure robots.txt policy for AI assistant and agent bots.\nAvailable values: \"disabled\", \"block\", \"only_on_ad_pages\"."]
    pub fn ai_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `aisearch` after provisioning.\nConfigure robots.txt policy for AI search bots.\nAvailable values: \"disabled\", \"block\", \"only_on_ad_pages\"."]
    pub fn aisearch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aisearch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_update_model` after provisioning.\nAutomatically update to the newest bot detection models created by Cloudflare as they are released. [Learn more.](https://developers.cloudflare.com/bots/reference/machine-learning-models#model-versions-and-release-notes)"]
    pub fn auto_update_model(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_update_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bm_cookie_enabled` after provisioning.\nIndicates that the bot management cookie can be placed on end user devices accessing the site. Defaults to true"]
    pub fn bm_cookie_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bm_cookie_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bot_preference_sync_enabled` after provisioning.\nEnable Bot Preference Sync for this zone. When enabled, Cloudflare can serve robots.txt content derived from the zone's AI Search, AI User, and AI Training preferences."]
    pub fn bot_preference_sync_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_preference_sync_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cf_robots_variant` after provisioning.\nSpecifies the Robots Access Control License variant to use.\nAvailable values: \"off\", \"policy_only\"."]
    pub fn cf_robots_variant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cf_robots_variant", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_bots_protection` after provisioning.\nEnable rule to block content bots. When enabled, blocks automated traffic with low bot scores, excluding safe verified bot categories. Exceptions should be managed via skip rules.\nAvailable values: \"block\", \"disabled\"."]
    pub fn content_bots_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_bots_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crawler_protection` after provisioning.\nEnable rule to punish AI Scrapers and Crawlers via a link maze.\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn crawler_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawler_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_js` after provisioning.\nUse lightweight, invisible JavaScript detections to improve Bot Management. [Learn more about JavaScript Detections](https://developers.cloudflare.com/bots/reference/javascript-detections/)."]
    pub fn enable_js(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_js", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fight_mode` after provisioning.\nWhether to enable Bot Fight Mode."]
    pub fn fight_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fight_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_robots_txt_managed` after provisioning.\nEnable cloudflare managed robots.txt. If an existing robots.txt is detected, then managed robots.txt will be prepended to the existing robots.txt."]
    pub fn is_robots_txt_managed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_robots_txt_managed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jsd_api_results_enabled` after provisioning.\nWhether to use JavaScript Detection results submitted through the API for this zone."]
    pub fn jsd_api_results_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jsd_api_results_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `optimize_wordpress` after provisioning.\nWhether to optimize Super Bot Fight Mode protections for Wordpress."]
    pub fn optimize_wordpress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optimize_wordpress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sbfm_definitely_automated` after provisioning.\nSuper Bot Fight Mode (SBFM) action to take on definitely automated requests.\nAvailable values: \"allow\", \"block\", \"managed_challenge\"."]
    pub fn sbfm_definitely_automated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbfm_definitely_automated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sbfm_likely_automated` after provisioning.\nSuper Bot Fight Mode (SBFM) action to take on likely automated requests.\nAvailable values: \"allow\", \"block\", \"managed_challenge\"."]
    pub fn sbfm_likely_automated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbfm_likely_automated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sbfm_static_resource_protection` after provisioning.\nSuper Bot Fight Mode (SBFM) to enable static resource protection.\nEnable if static resources on your application need bot protection.\nNote: Static resource protection can also result in legitimate traffic being blocked."]
    pub fn sbfm_static_resource_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.sbfm_static_resource_protection", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `sbfm_verified_bots` after provisioning.\nSuper Bot Fight Mode (SBFM) action to take on verified bots requests.\nAvailable values: \"allow\", \"block\"."]
    pub fn sbfm_verified_bots(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbfm_verified_bots", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stale_zone_configuration` after provisioning.\nA read-only field that shows which unauthorized settings are currently active on the zone. These settings typically result from upgrades or downgrades."]
    pub fn stale_zone_configuration(&self) -> DataBotManagement_StaleZoneConfigurationRef {
        DataBotManagement_StaleZoneConfigurationRef::new(
            self.shared().clone(),
            format!("{}.stale_zone_configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `suppress_session_score` after provisioning.\nWhether to disable tracking the highest bot score for a session in the Bot Management cookie."]
    pub fn suppress_session_score(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suppress_session_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `using_latest_model` after provisioning.\nA read-only field that indicates whether the zone currently is running the latest ML model."]
    pub fn using_latest_model(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.using_latest_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBotManagement_StaleZoneConfiguration {}

impl DataBotManagement_StaleZoneConfiguration { }

impl terrars::ToListMappable for DataBotManagement_StaleZoneConfiguration {
    type O = terrars::BlockAssignable<DataBotManagement_StaleZoneConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBotManagement_StaleZoneConfiguration {}

impl BuildDataBotManagement_StaleZoneConfiguration {
    pub fn build(self) -> DataBotManagement_StaleZoneConfiguration {
        DataBotManagement_StaleZoneConfiguration {}
    }
}

#[allow(dead_code)]
pub struct DataBotManagement_StaleZoneConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBotManagement_StaleZoneConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBotManagement_StaleZoneConfigurationRef {
        DataBotManagement_StaleZoneConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBotManagement_StaleZoneConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fight_mode` after provisioning.\nIndicates that the zone's Bot Fight Mode is turned on."]
    pub fn fight_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fight_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `optimize_wordpress` after provisioning.\nIndicates that the zone's wordpress optimization for SBFM is turned on."]
    pub fn optimize_wordpress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optimize_wordpress", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sbfm_definitely_automated` after provisioning.\nIndicates that the zone's definitely automated requests are being blocked or challenged."]
    pub fn sbfm_definitely_automated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbfm_definitely_automated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sbfm_likely_automated` after provisioning.\nIndicates that the zone's likely automated requests are being blocked or challenged."]
    pub fn sbfm_likely_automated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbfm_likely_automated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sbfm_static_resource_protection` after provisioning.\nIndicates that the zone's static resource protection is turned on."]
    pub fn sbfm_static_resource_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbfm_static_resource_protection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sbfm_verified_bots` after provisioning.\nIndicates that the zone's verified bot requests are being blocked."]
    pub fn sbfm_verified_bots(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbfm_verified_bots", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suppress_session_score` after provisioning.\nIndicates that the zone's session score tracking is disabled."]
    pub fn suppress_session_score(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suppress_session_score", self.base))
    }
}
