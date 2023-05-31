use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct NotificationPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    alert_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_integration: Option<Vec<NotificationPolicyEmailIntegrationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Vec<NotificationPolicyFiltersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pagerduty_integration: Option<Vec<NotificationPolicyPagerdutyIntegrationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhooks_integration: Option<Vec<NotificationPolicyWebhooksIntegrationEl>>,
    dynamic: NotificationPolicyDynamic,
}

struct NotificationPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NotificationPolicyData>,
}

#[derive(Clone)]
pub struct NotificationPolicy(Rc<NotificationPolicy_>);

impl NotificationPolicy {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `description`.\nDescription of the notification policy."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `email_integration`.\n"]
    pub fn set_email_integration(self, v: impl Into<BlockAssignable<NotificationPolicyEmailIntegrationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().email_integration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.email_integration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `filters`.\n"]
    pub fn set_filters(self, v: impl Into<BlockAssignable<NotificationPolicyFiltersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `pagerduty_integration`.\n"]
    pub fn set_pagerduty_integration(
        self,
        v: impl Into<BlockAssignable<NotificationPolicyPagerdutyIntegrationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pagerduty_integration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pagerduty_integration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `webhooks_integration`.\n"]
    pub fn set_webhooks_integration(
        self,
        v: impl Into<BlockAssignable<NotificationPolicyWebhooksIntegrationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().webhooks_integration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.webhooks_integration = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `alert_type` after provisioning.\nThe event type that will trigger the dispatch of a notification. See the developer documentation for descriptions of [available alert types](https://developers.cloudflare.com/fundamentals/notifications/notification-available/). Available values: `access_custom_certificate_expiration_type`, `advanced_ddos_attack_l4_alert`, `advanced_ddos_attack_l7_alert`, `bgp_hijack_notification`, `billing_usage_alert`, `block_notification_block_removed`, `block_notification_new_block`, `block_notification_review_rejected`, `clickhouse_alert_fw_anomaly`, `clickhouse_alert_fw_ent_anomaly`, `custom_ssl_certificate_event_type`, `dedicated_ssl_certificate_event_type`, `dos_attack_l4`, `dos_attack_l7`, `expiring_service_token_alert`, `failing_logpush_job_disabled_alert`, `fbm_auto_advertisement`, `fbm_dosd_attack`, `fbm_volumetric_attack`, `health_check_status_notification`, `hostname_aop_custom_certificate_expiration_type`, `http_alert_edge_error`, `http_alert_origin_error`, `load_balancing_health_alert`, `load_balancing_pool_enablement_alert`, `real_origin_monitoring`, `scriptmonitor_alert_new_code_change_detections`, `scriptmonitor_alert_new_hosts`, `scriptmonitor_alert_new_malicious_hosts`, `scriptmonitor_alert_new_malicious_scripts`, `scriptmonitor_alert_new_malicious_url`, `scriptmonitor_alert_new_max_length_resource_url`, `scriptmonitor_alert_new_resources`, `secondary_dns_all_primaries_failing`, `secondary_dns_primaries_failing`, `secondary_dns_zone_successfully_updated`, `secondary_dns_zone_validation_warning`, `sentinel_alert`, `stream_live_notifications`, `tunnel_health_event`, `tunnel_update_event`, `universal_ssl_event_type`, `web_analytics_metrics_update`, `weekly_account_overview`, `workers_alert`, `zone_aop_custom_certificate_expiration_type`."]
    pub fn alert_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alert_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nWhen the notification policy was created."]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the notification policy."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nThe status of the notification policy."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified` after provisioning.\nWhen the notification policy was last modified."]
    pub fn modified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the notification policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<NotificationPolicyFiltersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }
}

impl Referable for NotificationPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NotificationPolicy { }

impl ToListMappable for NotificationPolicy {
    type O = ListRef<NotificationPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NotificationPolicy_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_notification_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNotificationPolicy {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "The event type that will trigger the dispatch of a notification. See the developer documentation for descriptions of [available alert types](https://developers.cloudflare.com/fundamentals/notifications/notification-available/). Available values: `access_custom_certificate_expiration_type`, `advanced_ddos_attack_l4_alert`, `advanced_ddos_attack_l7_alert`, `bgp_hijack_notification`, `billing_usage_alert`, `block_notification_block_removed`, `block_notification_new_block`, `block_notification_review_rejected`, `clickhouse_alert_fw_anomaly`, `clickhouse_alert_fw_ent_anomaly`, `custom_ssl_certificate_event_type`, `dedicated_ssl_certificate_event_type`, `dos_attack_l4`, `dos_attack_l7`, `expiring_service_token_alert`, `failing_logpush_job_disabled_alert`, `fbm_auto_advertisement`, `fbm_dosd_attack`, `fbm_volumetric_attack`, `health_check_status_notification`, `hostname_aop_custom_certificate_expiration_type`, `http_alert_edge_error`, `http_alert_origin_error`, `load_balancing_health_alert`, `load_balancing_pool_enablement_alert`, `real_origin_monitoring`, `scriptmonitor_alert_new_code_change_detections`, `scriptmonitor_alert_new_hosts`, `scriptmonitor_alert_new_malicious_hosts`, `scriptmonitor_alert_new_malicious_scripts`, `scriptmonitor_alert_new_malicious_url`, `scriptmonitor_alert_new_max_length_resource_url`, `scriptmonitor_alert_new_resources`, `secondary_dns_all_primaries_failing`, `secondary_dns_primaries_failing`, `secondary_dns_zone_successfully_updated`, `secondary_dns_zone_validation_warning`, `sentinel_alert`, `stream_live_notifications`, `tunnel_health_event`, `tunnel_update_event`, `universal_ssl_event_type`, `web_analytics_metrics_update`, `weekly_account_overview`, `workers_alert`, `zone_aop_custom_certificate_expiration_type`."]
    pub alert_type: PrimField<String>,
    #[doc= "The status of the notification policy."]
    pub enabled: PrimField<bool>,
    #[doc= "The name of the notification policy."]
    pub name: PrimField<String>,
}

impl BuildNotificationPolicy {
    pub fn build(self, stack: &mut Stack) -> NotificationPolicy {
        let out = NotificationPolicy(Rc::new(NotificationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NotificationPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                alert_type: self.alert_type,
                description: core::default::Default::default(),
                enabled: self.enabled,
                id: core::default::Default::default(),
                name: self.name,
                email_integration: core::default::Default::default(),
                filters: core::default::Default::default(),
                pagerduty_integration: core::default::Default::default(),
                webhooks_integration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NotificationPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for NotificationPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NotificationPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `alert_type` after provisioning.\nThe event type that will trigger the dispatch of a notification. See the developer documentation for descriptions of [available alert types](https://developers.cloudflare.com/fundamentals/notifications/notification-available/). Available values: `access_custom_certificate_expiration_type`, `advanced_ddos_attack_l4_alert`, `advanced_ddos_attack_l7_alert`, `bgp_hijack_notification`, `billing_usage_alert`, `block_notification_block_removed`, `block_notification_new_block`, `block_notification_review_rejected`, `clickhouse_alert_fw_anomaly`, `clickhouse_alert_fw_ent_anomaly`, `custom_ssl_certificate_event_type`, `dedicated_ssl_certificate_event_type`, `dos_attack_l4`, `dos_attack_l7`, `expiring_service_token_alert`, `failing_logpush_job_disabled_alert`, `fbm_auto_advertisement`, `fbm_dosd_attack`, `fbm_volumetric_attack`, `health_check_status_notification`, `hostname_aop_custom_certificate_expiration_type`, `http_alert_edge_error`, `http_alert_origin_error`, `load_balancing_health_alert`, `load_balancing_pool_enablement_alert`, `real_origin_monitoring`, `scriptmonitor_alert_new_code_change_detections`, `scriptmonitor_alert_new_hosts`, `scriptmonitor_alert_new_malicious_hosts`, `scriptmonitor_alert_new_malicious_scripts`, `scriptmonitor_alert_new_malicious_url`, `scriptmonitor_alert_new_max_length_resource_url`, `scriptmonitor_alert_new_resources`, `secondary_dns_all_primaries_failing`, `secondary_dns_primaries_failing`, `secondary_dns_zone_successfully_updated`, `secondary_dns_zone_validation_warning`, `sentinel_alert`, `stream_live_notifications`, `tunnel_health_event`, `tunnel_update_event`, `universal_ssl_event_type`, `web_analytics_metrics_update`, `weekly_account_overview`, `workers_alert`, `zone_aop_custom_certificate_expiration_type`."]
    pub fn alert_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alert_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nWhen the notification policy was created."]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the notification policy."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nThe status of the notification policy."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified` after provisioning.\nWhen the notification policy was last modified."]
    pub fn modified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the notification policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<NotificationPolicyFiltersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NotificationPolicyEmailIntegrationEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl NotificationPolicyEmailIntegrationEl {
    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for NotificationPolicyEmailIntegrationEl {
    type O = BlockAssignable<NotificationPolicyEmailIntegrationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNotificationPolicyEmailIntegrationEl {
    #[doc= ""]
    pub id: PrimField<String>,
}

impl BuildNotificationPolicyEmailIntegrationEl {
    pub fn build(self) -> NotificationPolicyEmailIntegrationEl {
        NotificationPolicyEmailIntegrationEl {
            id: self.id,
            name: core::default::Default::default(),
        }
    }
}

pub struct NotificationPolicyEmailIntegrationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NotificationPolicyEmailIntegrationElRef {
    fn new(shared: StackShared, base: String) -> NotificationPolicyEmailIntegrationElRef {
        NotificationPolicyEmailIntegrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NotificationPolicyEmailIntegrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct NotificationPolicyFiltersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_source: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_id: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_id: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    megabits_per_second: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_health: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packets_per_second: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_id: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requests_per_second: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slo: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_hostname: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_zone_name: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zones: Option<SetField<PrimField<String>>>,
}

impl NotificationPolicyFiltersEl {
    #[doc= "Set the field `enabled`.\nState of the pool to alert on."]
    pub fn set_enabled(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `event_source`.\nSource configuration to alert on for pool or origin."]
    pub fn set_event_source(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.event_source = Some(v.into());
        self
    }

    #[doc= "Set the field `event_type`.\nStream event type to alert on."]
    pub fn set_event_type(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.event_type = Some(v.into());
        self
    }

    #[doc= "Set the field `health_check_id`.\nIdentifier health check. Required when using `filters.0.status`."]
    pub fn set_health_check_id(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.health_check_id = Some(v.into());
        self
    }

    #[doc= "Set the field `input_id`.\nStream input id to alert on."]
    pub fn set_input_id(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.input_id = Some(v.into());
        self
    }

    #[doc= "Set the field `limit`.\nA numerical limit. Example: `100`."]
    pub fn set_limit(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.limit = Some(v.into());
        self
    }

    #[doc= "Set the field `megabits_per_second`.\nMegabits per second threshold for dos alert."]
    pub fn set_megabits_per_second(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.megabits_per_second = Some(v.into());
        self
    }

    #[doc= "Set the field `new_health`.\nHealth status to alert on for pool or origin."]
    pub fn set_new_health(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.new_health = Some(v.into());
        self
    }

    #[doc= "Set the field `packets_per_second`.\nPackets per second threshold for dos alert."]
    pub fn set_packets_per_second(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.packets_per_second = Some(v.into());
        self
    }

    #[doc= "Set the field `pool_id`.\nLoad balancer pool identifier."]
    pub fn set_pool_id(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.pool_id = Some(v.into());
        self
    }

    #[doc= "Set the field `product`.\nProduct name. Available values: `worker_requests`, `worker_durable_objects_requests`, `worker_durable_objects_duration`, `worker_durable_objects_data_transfer`, `worker_durable_objects_stored_data`, `worker_durable_objects_storage_deletes`, `worker_durable_objects_storage_writes`, `worker_durable_objects_storage_reads`."]
    pub fn set_product(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.product = Some(v.into());
        self
    }

    #[doc= "Set the field `protocol`.\nProtocol to alert on for dos."]
    pub fn set_protocol(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `requests_per_second`.\nRequests per second threshold for dos alert."]
    pub fn set_requests_per_second(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.requests_per_second = Some(v.into());
        self
    }

    #[doc= "Set the field `services`.\n"]
    pub fn set_services(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.services = Some(v.into());
        self
    }

    #[doc= "Set the field `slo`.\nA numerical limit. Example: `99.9`."]
    pub fn set_slo(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.slo = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\nStatus to alert on."]
    pub fn set_status(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc= "Set the field `target_hostname`.\nTarget host to alert on for dos."]
    pub fn set_target_hostname(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.target_hostname = Some(v.into());
        self
    }

    #[doc= "Set the field `target_zone_name`.\nTarget domain to alert on."]
    pub fn set_target_zone_name(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.target_zone_name = Some(v.into());
        self
    }

    #[doc= "Set the field `zones`.\nA list of zone identifiers."]
    pub fn set_zones(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.zones = Some(v.into());
        self
    }
}

impl ToListMappable for NotificationPolicyFiltersEl {
    type O = BlockAssignable<NotificationPolicyFiltersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNotificationPolicyFiltersEl {}

impl BuildNotificationPolicyFiltersEl {
    pub fn build(self) -> NotificationPolicyFiltersEl {
        NotificationPolicyFiltersEl {
            enabled: core::default::Default::default(),
            event_source: core::default::Default::default(),
            event_type: core::default::Default::default(),
            health_check_id: core::default::Default::default(),
            input_id: core::default::Default::default(),
            limit: core::default::Default::default(),
            megabits_per_second: core::default::Default::default(),
            new_health: core::default::Default::default(),
            packets_per_second: core::default::Default::default(),
            pool_id: core::default::Default::default(),
            product: core::default::Default::default(),
            protocol: core::default::Default::default(),
            requests_per_second: core::default::Default::default(),
            services: core::default::Default::default(),
            slo: core::default::Default::default(),
            status: core::default::Default::default(),
            target_hostname: core::default::Default::default(),
            target_zone_name: core::default::Default::default(),
            zones: core::default::Default::default(),
        }
    }
}

pub struct NotificationPolicyFiltersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NotificationPolicyFiltersElRef {
    fn new(shared: StackShared, base: String) -> NotificationPolicyFiltersElRef {
        NotificationPolicyFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NotificationPolicyFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nState of the pool to alert on."]
    pub fn enabled(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `event_source` after provisioning.\nSource configuration to alert on for pool or origin."]
    pub fn event_source(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.event_source", self.base))
    }

    #[doc= "Get a reference to the value of field `event_type` after provisioning.\nStream event type to alert on."]
    pub fn event_type(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc= "Get a reference to the value of field `health_check_id` after provisioning.\nIdentifier health check. Required when using `filters.0.status`."]
    pub fn health_check_id(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.health_check_id", self.base))
    }

    #[doc= "Get a reference to the value of field `input_id` after provisioning.\nStream input id to alert on."]
    pub fn input_id(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.input_id", self.base))
    }

    #[doc= "Get a reference to the value of field `limit` after provisioning.\nA numerical limit. Example: `100`."]
    pub fn limit(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc= "Get a reference to the value of field `megabits_per_second` after provisioning.\nMegabits per second threshold for dos alert."]
    pub fn megabits_per_second(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.megabits_per_second", self.base))
    }

    #[doc= "Get a reference to the value of field `new_health` after provisioning.\nHealth status to alert on for pool or origin."]
    pub fn new_health(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.new_health", self.base))
    }

    #[doc= "Get a reference to the value of field `packets_per_second` after provisioning.\nPackets per second threshold for dos alert."]
    pub fn packets_per_second(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.packets_per_second", self.base))
    }

    #[doc= "Get a reference to the value of field `pool_id` after provisioning.\nLoad balancer pool identifier."]
    pub fn pool_id(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.pool_id", self.base))
    }

    #[doc= "Get a reference to the value of field `product` after provisioning.\nProduct name. Available values: `worker_requests`, `worker_durable_objects_requests`, `worker_durable_objects_duration`, `worker_durable_objects_data_transfer`, `worker_durable_objects_stored_data`, `worker_durable_objects_storage_deletes`, `worker_durable_objects_storage_writes`, `worker_durable_objects_storage_reads`."]
    pub fn product(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.product", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\nProtocol to alert on for dos."]
    pub fn protocol(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `requests_per_second` after provisioning.\nRequests per second threshold for dos alert."]
    pub fn requests_per_second(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.requests_per_second", self.base))
    }

    #[doc= "Get a reference to the value of field `services` after provisioning.\n"]
    pub fn services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.services", self.base))
    }

    #[doc= "Get a reference to the value of field `slo` after provisioning.\nA numerical limit. Example: `99.9`."]
    pub fn slo(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.slo", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nStatus to alert on."]
    pub fn status(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc= "Get a reference to the value of field `target_hostname` after provisioning.\nTarget host to alert on for dos."]
    pub fn target_hostname(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.target_hostname", self.base))
    }

    #[doc= "Get a reference to the value of field `target_zone_name` after provisioning.\nTarget domain to alert on."]
    pub fn target_zone_name(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.target_zone_name", self.base))
    }

    #[doc= "Get a reference to the value of field `zones` after provisioning.\nA list of zone identifiers."]
    pub fn zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}

#[derive(Serialize)]
pub struct NotificationPolicyPagerdutyIntegrationEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl NotificationPolicyPagerdutyIntegrationEl {
    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for NotificationPolicyPagerdutyIntegrationEl {
    type O = BlockAssignable<NotificationPolicyPagerdutyIntegrationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNotificationPolicyPagerdutyIntegrationEl {
    #[doc= ""]
    pub id: PrimField<String>,
}

impl BuildNotificationPolicyPagerdutyIntegrationEl {
    pub fn build(self) -> NotificationPolicyPagerdutyIntegrationEl {
        NotificationPolicyPagerdutyIntegrationEl {
            id: self.id,
            name: core::default::Default::default(),
        }
    }
}

pub struct NotificationPolicyPagerdutyIntegrationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NotificationPolicyPagerdutyIntegrationElRef {
    fn new(shared: StackShared, base: String) -> NotificationPolicyPagerdutyIntegrationElRef {
        NotificationPolicyPagerdutyIntegrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NotificationPolicyPagerdutyIntegrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct NotificationPolicyWebhooksIntegrationEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl NotificationPolicyWebhooksIntegrationEl {
    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for NotificationPolicyWebhooksIntegrationEl {
    type O = BlockAssignable<NotificationPolicyWebhooksIntegrationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNotificationPolicyWebhooksIntegrationEl {
    #[doc= ""]
    pub id: PrimField<String>,
}

impl BuildNotificationPolicyWebhooksIntegrationEl {
    pub fn build(self) -> NotificationPolicyWebhooksIntegrationEl {
        NotificationPolicyWebhooksIntegrationEl {
            id: self.id,
            name: core::default::Default::default(),
        }
    }
}

pub struct NotificationPolicyWebhooksIntegrationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NotificationPolicyWebhooksIntegrationElRef {
    fn new(shared: StackShared, base: String) -> NotificationPolicyWebhooksIntegrationElRef {
        NotificationPolicyWebhooksIntegrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NotificationPolicyWebhooksIntegrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct NotificationPolicyDynamic {
    email_integration: Option<DynamicBlock<NotificationPolicyEmailIntegrationEl>>,
    filters: Option<DynamicBlock<NotificationPolicyFiltersEl>>,
    pagerduty_integration: Option<DynamicBlock<NotificationPolicyPagerdutyIntegrationEl>>,
    webhooks_integration: Option<DynamicBlock<NotificationPolicyWebhooksIntegrationEl>>,
}
