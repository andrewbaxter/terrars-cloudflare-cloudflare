#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataNotificationPolicyData_ {
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
    policy_id: terrars::PrimField<std::string::String>,
}

struct DataNotificationPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNotificationPolicyData_>,
}

#[derive(Clone)]
pub struct DataNotificationPolicy(std::rc::Rc<DataNotificationPolicy_>);

impl DataNotificationPolicy {
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

    #[doc = "Set the field `account_id`.\nThe account id"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe account id"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `alert_interval` after provisioning.\nOptional specification of how often to re-alert from the same incident, not support on all alert types."]
    pub fn alert_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alert_interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `alert_type` after provisioning.\nRefers to which event will trigger a Notification dispatch. You can use the endpoint to get available alert types which then will give you a list of possible values.\nAvailable values: \"abuse_report_alert\", \"access_custom_certificate_expiration_type\", \"advanced_ddos_attack_l4_alert\", \"advanced_ddos_attack_l7_alert\", \"advanced_http_alert_error\", \"bgp_hijack_notification\", \"billing_usage_alert\", \"block_notification_block_removed\", \"block_notification_new_block\", \"block_notification_review_rejected\", \"bot_traffic_basic_alert\", \"brand_protection_alert\", \"brand_protection_digest\", \"clickhouse_alert_fw_anomaly\", \"clickhouse_alert_fw_ent_anomaly\", \"cloudforce_one_request_notification\", \"cni_maintenance_notification\", \"custom_analytics\", \"custom_bot_detection_alert\", \"custom_ssl_certificate_event_type\", \"dedicated_ssl_certificate_event_type\", \"device_connectivity_anomaly_alert\", \"dos_attack_l4\", \"dos_attack_l7\", \"expiring_service_token_alert\", \"failing_logpush_job_disabled_alert\", \"fbm_auto_advertisement\", \"fbm_dosd_attack\", \"fbm_volumetric_attack\", \"health_check_status_notification\", \"hostname_aop_custom_certificate_expiration_type\", \"http_alert_edge_error\", \"http_alert_origin_error\", \"image_notification\", \"image_resizing_notification\", \"incident_alert\", \"load_balancing_health_alert\", \"load_balancing_pool_enablement_alert\", \"logo_match_alert\", \"magic_tunnel_health_check_event\", \"magic_wan_tunnel_health\", \"maintenance_event_notification\", \"mtls_certificate_store_certificate_expiration_type\", \"pages_event_alert\", \"radar_notification\", \"real_origin_monitoring\", \"scriptmonitor_alert_new_code_change_detections\", \"scriptmonitor_alert_new_hosts\", \"scriptmonitor_alert_new_malicious_hosts\", \"scriptmonitor_alert_new_malicious_scripts\", \"scriptmonitor_alert_new_malicious_url\", \"scriptmonitor_alert_new_max_length_resource_url\", \"scriptmonitor_alert_new_resources\", \"secondary_dns_all_primaries_failing\", \"secondary_dns_primaries_failing\", \"secondary_dns_warning\", \"secondary_dns_zone_successfully_updated\", \"secondary_dns_zone_validation_warning\", \"security_insights_alert\", \"sentinel_alert\", \"stream_live_notifications\", \"synthetic_test_latency_alert\", \"synthetic_test_low_availability_alert\", \"traffic_anomalies_alert\", \"tunnel_health_event\", \"tunnel_update_event\", \"universal_ssl_event_type\", \"web_analytics_metrics_update\", \"zone_aop_custom_certificate_expiration_type\"."]
    pub fn alert_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alert_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional description for the Notification policy."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not the Notification policy is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filters` after provisioning.\nOptional filters that allow you to be alerted only on a subset of events for that alert type based on some criteria. This is only available for select alert types. See alert type documentation for more details."]
    pub fn filters(&self) -> DataNotificationPolicy_FiltersRef {
        DataNotificationPolicy_FiltersRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of a notification policy"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mechanisms` after provisioning.\nList of IDs that will be used when dispatching a notification. IDs for email type will be the email address."]
    pub fn mechanisms(&self) -> DataNotificationPolicy_MechanismsRef {
        DataNotificationPolicy_MechanismsRef::new(
            self.shared().clone(),
            format!("{}.mechanisms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\n"]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_id` after provisioning.\nThe unique identifier of a notification policy"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataNotificationPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNotificationPolicy { }

impl terrars::ToListMappable for DataNotificationPolicy {
    type O = terrars::ListRef<DataNotificationPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNotificationPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_notification_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNotificationPolicy {
    pub tf_id: std::string::String,
    #[doc = "The unique identifier of a notification policy"]
    pub policy_id: terrars::PrimField<std::string::String>,
}

impl BuildDataNotificationPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNotificationPolicy {
        let out = DataNotificationPolicy(std::rc::Rc::new(DataNotificationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataNotificationPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                policy_id: self.policy_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNotificationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNotificationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNotificationPolicyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe account id"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `alert_interval` after provisioning.\nOptional specification of how often to re-alert from the same incident, not support on all alert types."]
    pub fn alert_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alert_interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `alert_type` after provisioning.\nRefers to which event will trigger a Notification dispatch. You can use the endpoint to get available alert types which then will give you a list of possible values.\nAvailable values: \"abuse_report_alert\", \"access_custom_certificate_expiration_type\", \"advanced_ddos_attack_l4_alert\", \"advanced_ddos_attack_l7_alert\", \"advanced_http_alert_error\", \"bgp_hijack_notification\", \"billing_usage_alert\", \"block_notification_block_removed\", \"block_notification_new_block\", \"block_notification_review_rejected\", \"bot_traffic_basic_alert\", \"brand_protection_alert\", \"brand_protection_digest\", \"clickhouse_alert_fw_anomaly\", \"clickhouse_alert_fw_ent_anomaly\", \"cloudforce_one_request_notification\", \"cni_maintenance_notification\", \"custom_analytics\", \"custom_bot_detection_alert\", \"custom_ssl_certificate_event_type\", \"dedicated_ssl_certificate_event_type\", \"device_connectivity_anomaly_alert\", \"dos_attack_l4\", \"dos_attack_l7\", \"expiring_service_token_alert\", \"failing_logpush_job_disabled_alert\", \"fbm_auto_advertisement\", \"fbm_dosd_attack\", \"fbm_volumetric_attack\", \"health_check_status_notification\", \"hostname_aop_custom_certificate_expiration_type\", \"http_alert_edge_error\", \"http_alert_origin_error\", \"image_notification\", \"image_resizing_notification\", \"incident_alert\", \"load_balancing_health_alert\", \"load_balancing_pool_enablement_alert\", \"logo_match_alert\", \"magic_tunnel_health_check_event\", \"magic_wan_tunnel_health\", \"maintenance_event_notification\", \"mtls_certificate_store_certificate_expiration_type\", \"pages_event_alert\", \"radar_notification\", \"real_origin_monitoring\", \"scriptmonitor_alert_new_code_change_detections\", \"scriptmonitor_alert_new_hosts\", \"scriptmonitor_alert_new_malicious_hosts\", \"scriptmonitor_alert_new_malicious_scripts\", \"scriptmonitor_alert_new_malicious_url\", \"scriptmonitor_alert_new_max_length_resource_url\", \"scriptmonitor_alert_new_resources\", \"secondary_dns_all_primaries_failing\", \"secondary_dns_primaries_failing\", \"secondary_dns_warning\", \"secondary_dns_zone_successfully_updated\", \"secondary_dns_zone_validation_warning\", \"security_insights_alert\", \"sentinel_alert\", \"stream_live_notifications\", \"synthetic_test_latency_alert\", \"synthetic_test_low_availability_alert\", \"traffic_anomalies_alert\", \"tunnel_health_event\", \"tunnel_update_event\", \"universal_ssl_event_type\", \"web_analytics_metrics_update\", \"zone_aop_custom_certificate_expiration_type\"."]
    pub fn alert_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alert_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional description for the Notification policy."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not the Notification policy is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filters` after provisioning.\nOptional filters that allow you to be alerted only on a subset of events for that alert type based on some criteria. This is only available for select alert types. See alert type documentation for more details."]
    pub fn filters(&self) -> DataNotificationPolicy_FiltersRef {
        DataNotificationPolicy_FiltersRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of a notification policy"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mechanisms` after provisioning.\nList of IDs that will be used when dispatching a notification. IDs for email type will be the email address."]
    pub fn mechanisms(&self) -> DataNotificationPolicy_MechanismsRef {
        DataNotificationPolicy_MechanismsRef::new(
            self.shared().clone(),
            format!("{}.mechanisms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\n"]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_id` after provisioning.\nThe unique identifier of a notification policy"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataNotificationPolicy_Filters {}

impl DataNotificationPolicy_Filters { }

impl terrars::ToListMappable for DataNotificationPolicy_Filters {
    type O = terrars::BlockAssignable<DataNotificationPolicy_Filters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNotificationPolicy_Filters {}

impl BuildDataNotificationPolicy_Filters {
    pub fn build(self) -> DataNotificationPolicy_Filters {
        DataNotificationPolicy_Filters {}
    }
}

#[allow(dead_code)]
pub struct DataNotificationPolicy_FiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNotificationPolicy_FiltersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNotificationPolicy_FiltersRef {
        DataNotificationPolicy_FiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNotificationPolicy_FiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\nUsage depends on specific alert type"]
    pub fn actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `affected_asns` after provisioning.\nUsed for configuring radar_notification"]
    pub fn affected_asns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.affected_asns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `affected_components` after provisioning.\nUsed for configuring incident_alert"]
    pub fn affected_components(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.affected_components", self.base))
    }

    #[doc =
        "Get a reference to the value of field `affected_locations` after provisioning.\nUsed for configuring radar_notification"]
    pub fn affected_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.affected_locations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `airport_code` after provisioning.\nUsed for configuring maintenance_event_notification"]
    pub fn airport_code(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.airport_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `alert_trigger_preferences` after provisioning.\nUsage depends on specific alert type"]
    pub fn alert_trigger_preferences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert_trigger_preferences", self.base))
    }

    #[doc =
        "Get a reference to the value of field `alert_trigger_preferences_value` after provisioning.\nUsage depends on specific alert type"]
    pub fn alert_trigger_preferences_value(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert_trigger_preferences_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nUsed for configuring load_balancing_pool_enablement_alert"]
    pub fn enabled(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nUsed for configuring pages_event_alert"]
    pub fn environment(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\nUsed for configuring pages_event_alert"]
    pub fn event(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event", self.base))
    }

    #[doc =
        "Get a reference to the value of field `event_source` after provisioning.\nUsed for configuring load_balancing_health_alert"]
    pub fn event_source(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_source", self.base))
    }

    #[doc =
        "Get a reference to the value of field `event_type` after provisioning.\nUsage depends on specific alert type"]
    pub fn event_type(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `group_by` after provisioning.\nUsage depends on specific alert type"]
    pub fn group_by(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `health_check_id` after provisioning.\nUsed for configuring health_check_status_notification"]
    pub fn health_check_id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `incident_impact` after provisioning.\nUsed for configuring incident_alert"]
    pub fn incident_impact(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.incident_impact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `input_id` after provisioning.\nUsed for configuring stream_live_notifications"]
    pub fn input_id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `insight_class` after provisioning.\nUsed for configuring security_insights_alert"]
    pub fn insight_class(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.insight_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nUsed for configuring billing_usage_alert"]
    pub fn limit(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logo_tag` after provisioning.\nUsed for configuring logo_match_alert"]
    pub fn logo_tag(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logo_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `megabits_per_second` after provisioning.\nUsed for configuring advanced_ddos_attack_l4_alert"]
    pub fn megabits_per_second(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.megabits_per_second", self.base))
    }

    #[doc =
        "Get a reference to the value of field `new_health` after provisioning.\nUsed for configuring load_balancing_health_alert"]
    pub fn new_health(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_health", self.base))
    }

    #[doc =
        "Get a reference to the value of field `new_status` after provisioning.\nUsed for configuring tunnel_health_event"]
    pub fn new_status(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `packets_per_second` after provisioning.\nUsed for configuring advanced_ddos_attack_l4_alert"]
    pub fn packets_per_second(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.packets_per_second", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_id` after provisioning.\nUsage depends on specific alert type"]
    pub fn pool_id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pool_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pop_names` after provisioning.\nUsage depends on specific alert type"]
    pub fn pop_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pop_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `product` after provisioning.\nUsed for configuring billing_usage_alert"]
    pub fn product(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.product", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nUsed for configuring pages_event_alert"]
    pub fn project_id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nUsed for configuring advanced_ddos_attack_l4_alert"]
    pub fn protocol(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_tag` after provisioning.\nUsage depends on specific alert type"]
    pub fn query_tag(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests_per_second` after provisioning.\nUsed for configuring advanced_ddos_attack_l7_alert"]
    pub fn requests_per_second(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requests_per_second", self.base))
    }

    #[doc =
        "Get a reference to the value of field `selectors` after provisioning.\nUsage depends on specific alert type"]
    pub fn selectors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selectors", self.base))
    }

    #[doc =
        "Get a reference to the value of field `services` after provisioning.\nUsed for configuring clickhouse_alert_fw_ent_anomaly"]
    pub fn services(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.services", self.base))
    }

    #[doc = "Get a reference to the value of field `slo` after provisioning.\nUsage depends on specific alert type"]
    pub fn slo(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nUsed for configuring health_check_status_notification"]
    pub fn status(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_hostname` after provisioning.\nUsed for configuring advanced_ddos_attack_l7_alert"]
    pub fn target_hostname(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_ip` after provisioning.\nUsed for configuring advanced_ddos_attack_l4_alert"]
    pub fn target_ip(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_zone_name` after provisioning.\nUsed for configuring advanced_ddos_attack_l7_alert"]
    pub fn target_zone_name(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_zone_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_id` after provisioning.\nAccess service token IDs to include for expiring_service_token_alert. Omit this property to include all current and future service tokens."]
    pub fn token_id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `traffic_exclusions` after provisioning.\nUsed for configuring traffic_anomalies_alert"]
    pub fn traffic_exclusions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_exclusions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_id` after provisioning.\nUsed for configuring tunnel_health_event"]
    pub fn tunnel_id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tunnel_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tunnel_name` after provisioning.\nUsage depends on specific alert type"]
    pub fn tunnel_name(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tunnel_name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nUsage depends on specific alert type"]
    pub fn type_(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `where_` after provisioning.\nUsage depends on specific alert type"]
    pub fn where_(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.where", self.base))
    }

    #[doc = "Get a reference to the value of field `zones` after provisioning.\nUsage depends on specific alert type"]
    pub fn zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNotificationPolicy_Mechanisms_Email {}

impl DataNotificationPolicy_Mechanisms_Email { }

impl terrars::ToListMappable for DataNotificationPolicy_Mechanisms_Email {
    type O = terrars::BlockAssignable<DataNotificationPolicy_Mechanisms_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNotificationPolicy_Mechanisms_Email {}

impl BuildDataNotificationPolicy_Mechanisms_Email {
    pub fn build(self) -> DataNotificationPolicy_Mechanisms_Email {
        DataNotificationPolicy_Mechanisms_Email {}
    }
}

#[allow(dead_code)]
pub struct DataNotificationPolicy_Mechanisms_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNotificationPolicy_Mechanisms_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNotificationPolicy_Mechanisms_EmailRef {
        DataNotificationPolicy_Mechanisms_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNotificationPolicy_Mechanisms_EmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe email address"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNotificationPolicy_Mechanisms_Pagerduty {}

impl DataNotificationPolicy_Mechanisms_Pagerduty { }

impl terrars::ToListMappable for DataNotificationPolicy_Mechanisms_Pagerduty {
    type O = terrars::BlockAssignable<DataNotificationPolicy_Mechanisms_Pagerduty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNotificationPolicy_Mechanisms_Pagerduty {}

impl BuildDataNotificationPolicy_Mechanisms_Pagerduty {
    pub fn build(self) -> DataNotificationPolicy_Mechanisms_Pagerduty {
        DataNotificationPolicy_Mechanisms_Pagerduty {}
    }
}

#[allow(dead_code)]
pub struct DataNotificationPolicy_Mechanisms_PagerdutyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNotificationPolicy_Mechanisms_PagerdutyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNotificationPolicy_Mechanisms_PagerdutyRef {
        DataNotificationPolicy_Mechanisms_PagerdutyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNotificationPolicy_Mechanisms_PagerdutyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNotificationPolicy_Mechanisms_Webhooks {}

impl DataNotificationPolicy_Mechanisms_Webhooks { }

impl terrars::ToListMappable for DataNotificationPolicy_Mechanisms_Webhooks {
    type O = terrars::BlockAssignable<DataNotificationPolicy_Mechanisms_Webhooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNotificationPolicy_Mechanisms_Webhooks {}

impl BuildDataNotificationPolicy_Mechanisms_Webhooks {
    pub fn build(self) -> DataNotificationPolicy_Mechanisms_Webhooks {
        DataNotificationPolicy_Mechanisms_Webhooks {}
    }
}

#[allow(dead_code)]
pub struct DataNotificationPolicy_Mechanisms_WebhooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNotificationPolicy_Mechanisms_WebhooksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNotificationPolicy_Mechanisms_WebhooksRef {
        DataNotificationPolicy_Mechanisms_WebhooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNotificationPolicy_Mechanisms_WebhooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNotificationPolicy_Mechanisms {}

impl DataNotificationPolicy_Mechanisms { }

impl terrars::ToListMappable for DataNotificationPolicy_Mechanisms {
    type O = terrars::BlockAssignable<DataNotificationPolicy_Mechanisms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNotificationPolicy_Mechanisms {}

impl BuildDataNotificationPolicy_Mechanisms {
    pub fn build(self) -> DataNotificationPolicy_Mechanisms {
        DataNotificationPolicy_Mechanisms {}
    }
}

#[allow(dead_code)]
pub struct DataNotificationPolicy_MechanismsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNotificationPolicy_MechanismsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNotificationPolicy_MechanismsRef {
        DataNotificationPolicy_MechanismsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNotificationPolicy_MechanismsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<DataNotificationPolicy_Mechanisms_EmailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `pagerduty` after provisioning.\n"]
    pub fn pagerduty(&self) -> terrars::ListRef<DataNotificationPolicy_Mechanisms_PagerdutyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pagerduty", self.base))
    }

    #[doc = "Get a reference to the value of field `webhooks` after provisioning.\n"]
    pub fn webhooks(&self) -> terrars::ListRef<DataNotificationPolicy_Mechanisms_WebhooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhooks", self.base))
    }
}
