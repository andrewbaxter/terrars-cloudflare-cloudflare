#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct NotificationPolicyData_ {
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
    alert_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    alert_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std::option::Option<NotificationPolicy_Filters>,
    mechanisms: NotificationPolicy_Mechanisms,
    name: terrars::PrimField<std::string::String>,
}

struct NotificationPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NotificationPolicyData_>,
}

#[derive(Clone)]
pub struct NotificationPolicy(std::rc::Rc<NotificationPolicy_>);

impl NotificationPolicy {
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
        "Set the field `alert_interval`.\nOptional specification of how often to re-alert from the same incident, not support on all alert types."]
    pub fn set_alert_interval(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().alert_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional description for the Notification policy."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether or not the Notification policy is enabled."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filters`.\nOptional filters that allow you to be alerted only on a subset of events for that alert type based on some criteria. This is only available for select alert types. See alert type documentation for more details."]
    pub fn set_filters(self, v: impl std::convert::Into<NotificationPolicy_Filters>) -> Self {
        self.0.data.borrow_mut().filters = std::option::Option::Some(v.into());
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
    pub fn filters(&self) -> NotificationPolicy_FiltersRef {
        NotificationPolicy_FiltersRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mechanisms` after provisioning.\nList of IDs that will be used when dispatching a notification. IDs for email type will be the email address."]
    pub fn mechanisms(&self) -> NotificationPolicy_MechanismsRef {
        NotificationPolicy_MechanismsRef::new(self.shared().clone(), format!("{}.mechanisms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\n"]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl terrars::Referable for NotificationPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NotificationPolicy { }

impl terrars::ToListMappable for NotificationPolicy {
    type O = terrars::ListRef<NotificationPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NotificationPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_notification_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNotificationPolicy {
    pub tf_id: std::string::String,
    #[doc = "The account id"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Refers to which event will trigger a Notification dispatch. You can use the endpoint to get available alert types which then will give you a list of possible values.\nAvailable values: \"abuse_report_alert\", \"access_custom_certificate_expiration_type\", \"advanced_ddos_attack_l4_alert\", \"advanced_ddos_attack_l7_alert\", \"advanced_http_alert_error\", \"bgp_hijack_notification\", \"billing_usage_alert\", \"block_notification_block_removed\", \"block_notification_new_block\", \"block_notification_review_rejected\", \"bot_traffic_basic_alert\", \"brand_protection_alert\", \"brand_protection_digest\", \"clickhouse_alert_fw_anomaly\", \"clickhouse_alert_fw_ent_anomaly\", \"cloudforce_one_request_notification\", \"cni_maintenance_notification\", \"custom_analytics\", \"custom_bot_detection_alert\", \"custom_ssl_certificate_event_type\", \"dedicated_ssl_certificate_event_type\", \"device_connectivity_anomaly_alert\", \"dos_attack_l4\", \"dos_attack_l7\", \"expiring_service_token_alert\", \"failing_logpush_job_disabled_alert\", \"fbm_auto_advertisement\", \"fbm_dosd_attack\", \"fbm_volumetric_attack\", \"health_check_status_notification\", \"hostname_aop_custom_certificate_expiration_type\", \"http_alert_edge_error\", \"http_alert_origin_error\", \"image_notification\", \"image_resizing_notification\", \"incident_alert\", \"load_balancing_health_alert\", \"load_balancing_pool_enablement_alert\", \"logo_match_alert\", \"magic_tunnel_health_check_event\", \"magic_wan_tunnel_health\", \"maintenance_event_notification\", \"mtls_certificate_store_certificate_expiration_type\", \"pages_event_alert\", \"radar_notification\", \"real_origin_monitoring\", \"scriptmonitor_alert_new_code_change_detections\", \"scriptmonitor_alert_new_hosts\", \"scriptmonitor_alert_new_malicious_hosts\", \"scriptmonitor_alert_new_malicious_scripts\", \"scriptmonitor_alert_new_malicious_url\", \"scriptmonitor_alert_new_max_length_resource_url\", \"scriptmonitor_alert_new_resources\", \"secondary_dns_all_primaries_failing\", \"secondary_dns_primaries_failing\", \"secondary_dns_warning\", \"secondary_dns_zone_successfully_updated\", \"secondary_dns_zone_validation_warning\", \"security_insights_alert\", \"sentinel_alert\", \"stream_live_notifications\", \"synthetic_test_latency_alert\", \"synthetic_test_low_availability_alert\", \"traffic_anomalies_alert\", \"tunnel_health_event\", \"tunnel_update_event\", \"universal_ssl_event_type\", \"web_analytics_metrics_update\", \"zone_aop_custom_certificate_expiration_type\"."]
    pub alert_type: terrars::PrimField<std::string::String>,
    #[doc =
        "List of IDs that will be used when dispatching a notification. IDs for email type will be the email address."]
    pub mechanisms: NotificationPolicy_Mechanisms,
    #[doc = "Name of the policy."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNotificationPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> NotificationPolicy {
        let out = NotificationPolicy(std::rc::Rc::new(NotificationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NotificationPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                alert_interval: core::default::Default::default(),
                alert_type: self.alert_type,
                description: core::default::Default::default(),
                enabled: core::default::Default::default(),
                filters: core::default::Default::default(),
                mechanisms: self.mechanisms,
                name: self.name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NotificationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NotificationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NotificationPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn filters(&self) -> NotificationPolicy_FiltersRef {
        NotificationPolicy_FiltersRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mechanisms` after provisioning.\nList of IDs that will be used when dispatching a notification. IDs for email type will be the email address."]
    pub fn mechanisms(&self) -> NotificationPolicy_MechanismsRef {
        NotificationPolicy_MechanismsRef::new(self.shared().clone(), format!("{}.mechanisms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified` after provisioning.\n"]
    pub fn modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NotificationPolicy_Filters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    affected_asns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    affected_components: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    affected_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    airport_code: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert_trigger_preferences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert_trigger_preferences_value: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_source: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_type: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_by: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incident_impact: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insight_class: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logo_tag: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    megabits_per_second: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_health: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_status: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    packets_per_second: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pop_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_tag: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests_per_second: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selectors: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slo: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_hostname: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_ip: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_zone_name: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_exclusions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_name: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "where", skip_serializing_if = "std::option::Option::is_none")]
    where_: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zones: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NotificationPolicy_Filters {
    #[doc = "Set the field `actions`.\nUsage depends on specific alert type"]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `affected_asns`.\nUsed for configuring radar_notification"]
    pub fn set_affected_asns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.affected_asns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `affected_components`.\nUsed for configuring incident_alert"]
    pub fn set_affected_components(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.affected_components = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `affected_locations`.\nUsed for configuring radar_notification"]
    pub fn set_affected_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.affected_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `airport_code`.\nUsed for configuring maintenance_event_notification"]
    pub fn set_airport_code(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.airport_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alert_trigger_preferences`.\nUsage depends on specific alert type"]
    pub fn set_alert_trigger_preferences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alert_trigger_preferences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alert_trigger_preferences_value`.\nUsage depends on specific alert type"]
    pub fn set_alert_trigger_preferences_value(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alert_trigger_preferences_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nUsed for configuring load_balancing_pool_enablement_alert"]
    pub fn set_enabled(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\nUsed for configuring pages_event_alert"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event`.\nUsed for configuring pages_event_alert"]
    pub fn set_event(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.event = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_source`.\nUsed for configuring load_balancing_health_alert"]
    pub fn set_event_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.event_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_type`.\nUsage depends on specific alert type"]
    pub fn set_event_type(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.event_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_by`.\nUsage depends on specific alert type"]
    pub fn set_group_by(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.group_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_id`.\nUsed for configuring health_check_status_notification"]
    pub fn set_health_check_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.health_check_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `incident_impact`.\nUsed for configuring incident_alert"]
    pub fn set_incident_impact(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.incident_impact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_id`.\nUsed for configuring stream_live_notifications"]
    pub fn set_input_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `insight_class`.\nUsed for configuring security_insights_alert"]
    pub fn set_insight_class(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.insight_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit`.\nUsed for configuring billing_usage_alert"]
    pub fn set_limit(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logo_tag`.\nUsed for configuring logo_match_alert"]
    pub fn set_logo_tag(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.logo_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `megabits_per_second`.\nUsed for configuring advanced_ddos_attack_l4_alert"]
    pub fn set_megabits_per_second(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.megabits_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `new_health`.\nUsed for configuring load_balancing_health_alert"]
    pub fn set_new_health(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.new_health = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `new_status`.\nUsed for configuring tunnel_health_event"]
    pub fn set_new_status(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.new_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `packets_per_second`.\nUsed for configuring advanced_ddos_attack_l4_alert"]
    pub fn set_packets_per_second(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.packets_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_id`.\nUsage depends on specific alert type"]
    pub fn set_pool_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.pool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pop_names`.\nUsage depends on specific alert type"]
    pub fn set_pop_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.pop_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product`.\nUsed for configuring billing_usage_alert"]
    pub fn set_product(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.product = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\nUsed for configuring pages_event_alert"]
    pub fn set_project_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\nUsed for configuring advanced_ddos_attack_l4_alert"]
    pub fn set_protocol(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_tag`.\nUsage depends on specific alert type"]
    pub fn set_query_tag(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requests_per_second`.\nUsed for configuring advanced_ddos_attack_l7_alert"]
    pub fn set_requests_per_second(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.requests_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `selectors`.\nUsage depends on specific alert type"]
    pub fn set_selectors(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.selectors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `services`.\nUsed for configuring clickhouse_alert_fw_ent_anomaly"]
    pub fn set_services(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.services = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slo`.\nUsage depends on specific alert type"]
    pub fn set_slo(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.slo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\nUsed for configuring health_check_status_notification"]
    pub fn set_status(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_hostname`.\nUsed for configuring advanced_ddos_attack_l7_alert"]
    pub fn set_target_hostname(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_ip`.\nUsed for configuring advanced_ddos_attack_l4_alert"]
    pub fn set_target_ip(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_zone_name`.\nUsed for configuring advanced_ddos_attack_l7_alert"]
    pub fn set_target_zone_name(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_zone_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `token_id`.\nAccess service token IDs to include for expiring_service_token_alert. Omit this property to include all current and future service tokens."]
    pub fn set_token_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.token_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_exclusions`.\nUsed for configuring traffic_anomalies_alert"]
    pub fn set_traffic_exclusions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.traffic_exclusions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_id`.\nUsed for configuring tunnel_health_event"]
    pub fn set_tunnel_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tunnel_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_name`.\nUsage depends on specific alert type"]
    pub fn set_tunnel_name(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tunnel_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nUsage depends on specific alert type"]
    pub fn set_type(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `where_`.\nUsage depends on specific alert type"]
    pub fn set_where(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.where_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zones`.\nUsage depends on specific alert type"]
    pub fn set_zones(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.zones = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NotificationPolicy_Filters {
    type O = terrars::BlockAssignable<NotificationPolicy_Filters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNotificationPolicy_Filters {}

impl BuildNotificationPolicy_Filters {
    pub fn build(self) -> NotificationPolicy_Filters {
        NotificationPolicy_Filters {
            actions: core::default::Default::default(),
            affected_asns: core::default::Default::default(),
            affected_components: core::default::Default::default(),
            affected_locations: core::default::Default::default(),
            airport_code: core::default::Default::default(),
            alert_trigger_preferences: core::default::Default::default(),
            alert_trigger_preferences_value: core::default::Default::default(),
            enabled: core::default::Default::default(),
            environment: core::default::Default::default(),
            event: core::default::Default::default(),
            event_source: core::default::Default::default(),
            event_type: core::default::Default::default(),
            group_by: core::default::Default::default(),
            health_check_id: core::default::Default::default(),
            incident_impact: core::default::Default::default(),
            input_id: core::default::Default::default(),
            insight_class: core::default::Default::default(),
            limit: core::default::Default::default(),
            logo_tag: core::default::Default::default(),
            megabits_per_second: core::default::Default::default(),
            new_health: core::default::Default::default(),
            new_status: core::default::Default::default(),
            packets_per_second: core::default::Default::default(),
            pool_id: core::default::Default::default(),
            pop_names: core::default::Default::default(),
            product: core::default::Default::default(),
            project_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query_tag: core::default::Default::default(),
            requests_per_second: core::default::Default::default(),
            selectors: core::default::Default::default(),
            services: core::default::Default::default(),
            slo: core::default::Default::default(),
            status: core::default::Default::default(),
            target_hostname: core::default::Default::default(),
            target_ip: core::default::Default::default(),
            target_zone_name: core::default::Default::default(),
            token_id: core::default::Default::default(),
            traffic_exclusions: core::default::Default::default(),
            tunnel_id: core::default::Default::default(),
            tunnel_name: core::default::Default::default(),
            type_: core::default::Default::default(),
            where_: core::default::Default::default(),
            zones: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NotificationPolicy_FiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NotificationPolicy_FiltersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NotificationPolicy_FiltersRef {
        NotificationPolicy_FiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NotificationPolicy_FiltersRef {
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
pub struct NotificationPolicy_Mechanisms_Email {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NotificationPolicy_Mechanisms_Email {
    #[doc = "Set the field `id`.\nThe email address"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NotificationPolicy_Mechanisms_Email {
    type O = terrars::BlockAssignable<NotificationPolicy_Mechanisms_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNotificationPolicy_Mechanisms_Email {}

impl BuildNotificationPolicy_Mechanisms_Email {
    pub fn build(self) -> NotificationPolicy_Mechanisms_Email {
        NotificationPolicy_Mechanisms_Email { id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NotificationPolicy_Mechanisms_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NotificationPolicy_Mechanisms_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NotificationPolicy_Mechanisms_EmailRef {
        NotificationPolicy_Mechanisms_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NotificationPolicy_Mechanisms_EmailRef {
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
pub struct NotificationPolicy_Mechanisms_Pagerduty {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NotificationPolicy_Mechanisms_Pagerduty {
    #[doc = "Set the field `id`.\nUUID"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NotificationPolicy_Mechanisms_Pagerduty {
    type O = terrars::BlockAssignable<NotificationPolicy_Mechanisms_Pagerduty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNotificationPolicy_Mechanisms_Pagerduty {}

impl BuildNotificationPolicy_Mechanisms_Pagerduty {
    pub fn build(self) -> NotificationPolicy_Mechanisms_Pagerduty {
        NotificationPolicy_Mechanisms_Pagerduty { id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NotificationPolicy_Mechanisms_PagerdutyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NotificationPolicy_Mechanisms_PagerdutyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NotificationPolicy_Mechanisms_PagerdutyRef {
        NotificationPolicy_Mechanisms_PagerdutyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NotificationPolicy_Mechanisms_PagerdutyRef {
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
pub struct NotificationPolicy_Mechanisms_Webhooks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NotificationPolicy_Mechanisms_Webhooks {
    #[doc = "Set the field `id`.\nUUID"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NotificationPolicy_Mechanisms_Webhooks {
    type O = terrars::BlockAssignable<NotificationPolicy_Mechanisms_Webhooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNotificationPolicy_Mechanisms_Webhooks {}

impl BuildNotificationPolicy_Mechanisms_Webhooks {
    pub fn build(self) -> NotificationPolicy_Mechanisms_Webhooks {
        NotificationPolicy_Mechanisms_Webhooks { id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NotificationPolicy_Mechanisms_WebhooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NotificationPolicy_Mechanisms_WebhooksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NotificationPolicy_Mechanisms_WebhooksRef {
        NotificationPolicy_Mechanisms_WebhooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NotificationPolicy_Mechanisms_WebhooksRef {
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
pub struct NotificationPolicy_Mechanisms {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<std::vec::Vec<NotificationPolicy_Mechanisms_Email>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pagerduty: std::option::Option<std::vec::Vec<NotificationPolicy_Mechanisms_Pagerduty>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhooks: std::option::Option<std::vec::Vec<NotificationPolicy_Mechanisms_Webhooks>>,
}

impl NotificationPolicy_Mechanisms {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<std::vec::Vec<NotificationPolicy_Mechanisms_Email>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pagerduty`.\n"]
    pub fn set_pagerduty(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<NotificationPolicy_Mechanisms_Pagerduty>>,
    ) -> Self {
        self.pagerduty = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `webhooks`.\n"]
    pub fn set_webhooks(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<NotificationPolicy_Mechanisms_Webhooks>>,
    ) -> Self {
        self.webhooks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NotificationPolicy_Mechanisms {
    type O = terrars::BlockAssignable<NotificationPolicy_Mechanisms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNotificationPolicy_Mechanisms {}

impl BuildNotificationPolicy_Mechanisms {
    pub fn build(self) -> NotificationPolicy_Mechanisms {
        NotificationPolicy_Mechanisms {
            email: core::default::Default::default(),
            pagerduty: core::default::Default::default(),
            webhooks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NotificationPolicy_MechanismsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NotificationPolicy_MechanismsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NotificationPolicy_MechanismsRef {
        NotificationPolicy_MechanismsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NotificationPolicy_MechanismsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::SetRef<NotificationPolicy_Mechanisms_EmailRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `pagerduty` after provisioning.\n"]
    pub fn pagerduty(&self) -> terrars::SetRef<NotificationPolicy_Mechanisms_PagerdutyRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.pagerduty", self.base))
    }

    #[doc = "Get a reference to the value of field `webhooks` after provisioning.\n"]
    pub fn webhooks(&self) -> terrars::SetRef<NotificationPolicy_Mechanisms_WebhooksRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.webhooks", self.base))
    }
}
