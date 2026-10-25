#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct BotManagementData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_bots_migration_opt_out: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_bots_protection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_training: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aisearch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_update_model: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bm_cookie_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bot_preference_sync_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cf_robots_variant: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_bots_protection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_protection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_js: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fight_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_robots_txt_managed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jsd_api_results_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optimize_wordpress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sbfm_definitely_automated: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sbfm_likely_automated: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sbfm_static_resource_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sbfm_verified_bots: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suppress_session_score: std::option::Option<terrars::PrimField<bool>>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct BotManagement_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BotManagementData_>,
}

#[derive(Clone)]
pub struct BotManagement(std::rc::Rc<BotManagement_>);

impl BotManagement {
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
        "Set the field `ai_bots_migration_opt_out`.\nTemporary migration flag tracking zones opted out of AI bots managed-rule updates."]
    pub fn set_ai_bots_migration_opt_out(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ai_bots_migration_opt_out = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ai_bots_protection`.\nEnable rule to block AI Scrapers and Crawlers.\nAvailable values: \"block\", \"disabled\", \"only_on_ad_pages\"."]
    pub fn set_ai_bots_protection(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ai_bots_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ai_training`.\nConfigure robots.txt policy for AI model training bots.\nAvailable values: \"disabled\", \"disallow\", \"block\", \"only_on_ad_pages\"."]
    pub fn set_ai_training(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ai_training = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ai_user`.\nConfigure robots.txt policy for AI assistant and agent bots.\nAvailable values: \"disabled\", \"block\", \"only_on_ad_pages\"."]
    pub fn set_ai_user(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ai_user = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `aisearch`.\nConfigure robots.txt policy for AI search bots.\nAvailable values: \"disabled\", \"block\", \"only_on_ad_pages\"."]
    pub fn set_aisearch(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aisearch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_update_model`.\nAutomatically update to the newest bot detection models created by Cloudflare as they are released. [Learn more.](https://developers.cloudflare.com/bots/reference/machine-learning-models#model-versions-and-release-notes)"]
    pub fn set_auto_update_model(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_update_model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bm_cookie_enabled`.\nIndicates that the bot management cookie can be placed on end user devices accessing the site. Defaults to true"]
    pub fn set_bm_cookie_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bm_cookie_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bot_preference_sync_enabled`.\nEnable Bot Preference Sync for this zone. When enabled, Cloudflare can serve robots.txt content derived from the zone's AI Search, AI User, and AI Training preferences."]
    pub fn set_bot_preference_sync_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bot_preference_sync_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cf_robots_variant`.\nSpecifies the Robots Access Control License variant to use.\nAvailable values: \"off\", \"policy_only\"."]
    pub fn set_cf_robots_variant(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cf_robots_variant = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_bots_protection`.\nEnable rule to block content bots. When enabled, blocks automated traffic with low bot scores, excluding safe verified bot categories. Exceptions should be managed via skip rules.\nAvailable values: \"block\", \"disabled\"."]
    pub fn set_content_bots_protection(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().content_bots_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `crawler_protection`.\nEnable rule to punish AI Scrapers and Crawlers via a link maze.\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn set_crawler_protection(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().crawler_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_js`.\nUse lightweight, invisible JavaScript detections to improve Bot Management. [Learn more about JavaScript Detections](https://developers.cloudflare.com/bots/reference/javascript-detections/)."]
    pub fn set_enable_js(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_js = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fight_mode`.\nWhether to enable Bot Fight Mode."]
    pub fn set_fight_mode(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().fight_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_robots_txt_managed`.\nEnable cloudflare managed robots.txt. If an existing robots.txt is detected, then managed robots.txt will be prepended to the existing robots.txt."]
    pub fn set_is_robots_txt_managed(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_robots_txt_managed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jsd_api_results_enabled`.\nWhether to use JavaScript Detection results submitted through the API for this zone."]
    pub fn set_jsd_api_results_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().jsd_api_results_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `optimize_wordpress`.\nWhether to optimize Super Bot Fight Mode protections for Wordpress."]
    pub fn set_optimize_wordpress(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().optimize_wordpress = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sbfm_definitely_automated`.\nSuper Bot Fight Mode (SBFM) action to take on definitely automated requests.\nAvailable values: \"allow\", \"block\", \"managed_challenge\"."]
    pub fn set_sbfm_definitely_automated(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().sbfm_definitely_automated = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sbfm_likely_automated`.\nSuper Bot Fight Mode (SBFM) action to take on likely automated requests.\nAvailable values: \"allow\", \"block\", \"managed_challenge\"."]
    pub fn set_sbfm_likely_automated(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sbfm_likely_automated = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sbfm_static_resource_protection`.\nSuper Bot Fight Mode (SBFM) to enable static resource protection.\nEnable if static resources on your application need bot protection.\nNote: Static resource protection can also result in legitimate traffic being blocked."]
    pub fn set_sbfm_static_resource_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sbfm_static_resource_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sbfm_verified_bots`.\nSuper Bot Fight Mode (SBFM) action to take on verified bots requests.\nAvailable values: \"allow\", \"block\"."]
    pub fn set_sbfm_verified_bots(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sbfm_verified_bots = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suppress_session_score`.\nWhether to disable tracking the highest bot score for a session in the Bot Management cookie."]
    pub fn set_suppress_session_score(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suppress_session_score = std::option::Option::Some(v.into());
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
    pub fn stale_zone_configuration(&self) -> BotManagement_StaleZoneConfigurationRef {
        BotManagement_StaleZoneConfigurationRef::new(
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

impl terrars::Referable for BotManagement {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BotManagement { }

impl terrars::ToListMappable for BotManagement {
    type O = terrars::ListRef<BotManagementRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BotManagement_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_bot_management".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBotManagement {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildBotManagement {
    pub fn build(self, stack: &mut terrars::Stack) -> BotManagement {
        let out = BotManagement(std::rc::Rc::new(BotManagement_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BotManagementData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                ai_bots_migration_opt_out: core::default::Default::default(),
                ai_bots_protection: core::default::Default::default(),
                ai_training: core::default::Default::default(),
                ai_user: core::default::Default::default(),
                aisearch: core::default::Default::default(),
                auto_update_model: core::default::Default::default(),
                bm_cookie_enabled: core::default::Default::default(),
                bot_preference_sync_enabled: core::default::Default::default(),
                cf_robots_variant: core::default::Default::default(),
                content_bots_protection: core::default::Default::default(),
                crawler_protection: core::default::Default::default(),
                enable_js: core::default::Default::default(),
                fight_mode: core::default::Default::default(),
                is_robots_txt_managed: core::default::Default::default(),
                jsd_api_results_enabled: core::default::Default::default(),
                optimize_wordpress: core::default::Default::default(),
                sbfm_definitely_automated: core::default::Default::default(),
                sbfm_likely_automated: core::default::Default::default(),
                sbfm_static_resource_protection: core::default::Default::default(),
                sbfm_verified_bots: core::default::Default::default(),
                suppress_session_score: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BotManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BotManagementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BotManagementRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn stale_zone_configuration(&self) -> BotManagement_StaleZoneConfigurationRef {
        BotManagement_StaleZoneConfigurationRef::new(
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
pub struct BotManagement_StaleZoneConfiguration {}

impl BotManagement_StaleZoneConfiguration { }

impl terrars::ToListMappable for BotManagement_StaleZoneConfiguration {
    type O = terrars::BlockAssignable<BotManagement_StaleZoneConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBotManagement_StaleZoneConfiguration {}

impl BuildBotManagement_StaleZoneConfiguration {
    pub fn build(self) -> BotManagement_StaleZoneConfiguration {
        BotManagement_StaleZoneConfiguration {}
    }
}

#[allow(dead_code)]
pub struct BotManagement_StaleZoneConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BotManagement_StaleZoneConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BotManagement_StaleZoneConfigurationRef {
        BotManagement_StaleZoneConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BotManagement_StaleZoneConfigurationRef {
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
