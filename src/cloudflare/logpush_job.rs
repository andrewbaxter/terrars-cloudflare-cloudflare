#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct LogpushJobData_ {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    destination_conf: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_attack_traffic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logpull_options: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_upload_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_upload_interval_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_upload_records: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_options: std::option::Option<LogpushJob_OutputOptions>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ownership_challenge: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct LogpushJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LogpushJobData_>,
}

#[derive(Clone)]
pub struct LogpushJob(std::rc::Rc<LogpushJob_>);

impl LogpushJob {
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
        "Set the field `account_id`.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dataset`.\nName of the dataset. A list of supported datasets can be found on the [Developer Docs](https://developers.cloudflare.com/logs/reference/log-fields/).\nAvailable values: \"access_requests\", \"account_abuse_protection_events\", \"audit_logs\", \"audit_logs_v2\", \"biso_user_actions\", \"casb_findings\", \"device_posture_results\", \"dex_application_tests\", \"dex_device_state_events\", \"dlp_forensic_copies\", \"dns_firewall_logs\", \"dns_logs\", \"email_security_alerts\", \"email_security_post_delivery_events\", \"firewall_events\", \"gateway_dns\", \"gateway_http\", \"gateway_network\", \"http_requests\", \"ipsec_logs\", \"magic_bgp_logs\", \"magic_ids_detections\", \"mcp_portal_logs\", \"mnm_flow_logs\", \"nel_reports\", \"network_analytics_logs\", \"page_shield_events\", \"sinkhole_http_logs\", \"spectrum_events\", \"ssh_logs\", \"turnstile_events\", \"warp_config_changes\", \"warp_toggle_changes\", \"websocket_analytics\", \"workers_trace_events\", \"zaraz_events\", \"zero_trust_network_sessions\"."]
    pub fn set_dataset(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nFlag that indicates if the job is enabled."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nThe filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/filters/)."]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter_attack_traffic`.\nWhen true, excludes DDoS attack traffic from logs. This option is supported for the `http_requests`, `firewall_events`, and `network_analytics_logs` datasets."]
    pub fn set_filter_attack_traffic(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().filter_attack_traffic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `frequency`.\nThis field is deprecated. Please use `max_upload_*` parameters instead. . The frequency at which Cloudflare sends batches of logs to your destination. Setting frequency to high sends your logs in larger quantities of smaller files. Setting frequency to low sends logs in smaller quantities of larger files.\nAvailable values: \"high\", \"low\"."]
    pub fn set_frequency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kind`.\nThe kind parameter (optional) is used to differentiate between Logpush and Edge Log Delivery jobs (when supported by the dataset).\nAvailable values: \"\", \"edge\"."]
    pub fn set_kind(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kind = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `logpull_options`.\nThis field is deprecated. Use `output_options` instead. Configuration string. It specifies things like requested fields and timestamp formats. If migrating from the logpull api, copy the url (full url or just the query string) of your call here, and logpush will keep on making this call for you, setting start and end times appropriately."]
    pub fn set_logpull_options(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().logpull_options = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_upload_bytes`.\nThe maximum uncompressed file size of a batch of logs. This setting value must be between `5 MB` and `1 GB`, or `0` to disable it. Note that you cannot set a minimum file size; this means that log files may be much smaller than this batch size."]
    pub fn set_max_upload_bytes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_upload_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_upload_interval_seconds`.\nThe maximum interval in seconds for log batches. This setting must be between 30 and 300 seconds (5 minutes), or `0` to disable it. Note that you cannot specify a minimum interval for log batches; this means that log files may be sent in shorter intervals than this."]
    pub fn set_max_upload_interval_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_upload_interval_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_upload_records`.\nThe maximum number of log lines per batch. This setting must be between 1000 and 1,000,000 lines, or `0` to disable it. Note that you cannot specify a minimum number of log lines per batch; this means that log files may contain many fewer lines than this."]
    pub fn set_max_upload_records(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_upload_records = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nOptional human readable job name. Not unique. Cloudflare suggests. that you set this to a meaningful string, like the domain name, to make it easier to identify your job."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_options`.\nThe structured replacement for `logpull_options`. When including this field, the `logpull_option` field will be ignored."]
    pub fn set_output_options(self, v: impl std::convert::Into<LogpushJob_OutputOptions>) -> Self {
        self.0.data.borrow_mut().output_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ownership_challenge`.\nOwnership challenge token to prove destination ownership."]
    pub fn set_ownership_challenge(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ownership_challenge = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nName of the dataset. A list of supported datasets can be found on the [Developer Docs](https://developers.cloudflare.com/logs/reference/log-fields/).\nAvailable values: \"access_requests\", \"account_abuse_protection_events\", \"audit_logs\", \"audit_logs_v2\", \"biso_user_actions\", \"casb_findings\", \"device_posture_results\", \"dex_application_tests\", \"dex_device_state_events\", \"dlp_forensic_copies\", \"dns_firewall_logs\", \"dns_logs\", \"email_security_alerts\", \"email_security_post_delivery_events\", \"firewall_events\", \"gateway_dns\", \"gateway_http\", \"gateway_network\", \"http_requests\", \"ipsec_logs\", \"magic_bgp_logs\", \"magic_ids_detections\", \"mcp_portal_logs\", \"mnm_flow_logs\", \"nel_reports\", \"network_analytics_logs\", \"page_shield_events\", \"sinkhole_http_logs\", \"spectrum_events\", \"ssh_logs\", \"turnstile_events\", \"warp_config_changes\", \"warp_toggle_changes\", \"websocket_analytics\", \"workers_trace_events\", \"zaraz_events\", \"zero_trust_network_sessions\"."]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destination_conf` after provisioning.\nUniquely identifies a resource (such as an s3 bucket) where data. will be pushed. Additional configuration parameters supported by the destination may be included."]
    pub fn destination_conf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_conf", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that indicates if the job is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error_message` after provisioning.\nIf not null, the job is currently failing. Failures are usually. repetitive (example: no permissions to write to destination bucket). Only the last failure is recorded. On successful execution of a job the error_message and last_error are set to null."]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nThe filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/filters/)."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter_attack_traffic` after provisioning.\nWhen true, excludes DDoS attack traffic from logs. This option is supported for the `http_requests`, `firewall_events`, and `network_analytics_logs` datasets."]
    pub fn filter_attack_traffic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_attack_traffic", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nThis field is deprecated. Please use `max_upload_*` parameters instead. . The frequency at which Cloudflare sends batches of logs to your destination. Setting frequency to high sends your logs in larger quantities of smaller files. Setting frequency to low sends logs in smaller quantities of larger files.\nAvailable values: \"high\", \"low\"."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique id of the job."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe kind parameter (optional) is used to differentiate between Logpush and Edge Log Delivery jobs (when supported by the dataset).\nAvailable values: \"\", \"edge\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_complete` after provisioning.\nRecords the last time for which logs have been successfully pushed. If the last successful push was for logs range 2018-07-23T10:00:00Z to 2018-07-23T10:01:00Z then the value of this field will be 2018-07-23T10:01:00Z. If the job has never run or has just been enabled and hasn't run yet then the field will be empty."]
    pub fn last_complete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_complete", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_error` after provisioning.\nRecords the last time the job failed. If not null, the job is currently. failing. If null, the job has either never failed or has run successfully at least once since last failure. See also the error_message field."]
    pub fn last_error(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_error", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logpull_options` after provisioning.\nThis field is deprecated. Use `output_options` instead. Configuration string. It specifies things like requested fields and timestamp formats. If migrating from the logpull api, copy the url (full url or just the query string) of your call here, and logpush will keep on making this call for you, setting start and end times appropriately."]
    pub fn logpull_options(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpull_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_upload_bytes` after provisioning.\nThe maximum uncompressed file size of a batch of logs. This setting value must be between `5 MB` and `1 GB`, or `0` to disable it. Note that you cannot set a minimum file size; this means that log files may be much smaller than this batch size."]
    pub fn max_upload_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_upload_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_upload_interval_seconds` after provisioning.\nThe maximum interval in seconds for log batches. This setting must be between 30 and 300 seconds (5 minutes), or `0` to disable it. Note that you cannot specify a minimum interval for log batches; this means that log files may be sent in shorter intervals than this."]
    pub fn max_upload_interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_upload_interval_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_upload_records` after provisioning.\nThe maximum number of log lines per batch. This setting must be between 1000 and 1,000,000 lines, or `0` to disable it. Note that you cannot specify a minimum number of log lines per batch; this means that log files may contain many fewer lines than this."]
    pub fn max_upload_records(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_upload_records", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOptional human readable job name. Not unique. Cloudflare suggests. that you set this to a meaningful string, like the domain name, to make it easier to identify your job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `output_options` after provisioning.\nThe structured replacement for `logpull_options`. When including this field, the `logpull_option` field will be ignored."]
    pub fn output_options(&self) -> LogpushJob_OutputOptionsRef {
        LogpushJob_OutputOptionsRef::new(self.shared().clone(), format!("{}.output_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_challenge` after provisioning.\nOwnership challenge token to prove destination ownership."]
    pub fn ownership_challenge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_challenge", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for LogpushJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LogpushJob { }

impl terrars::ToListMappable for LogpushJob {
    type O = terrars::ListRef<LogpushJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LogpushJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_logpush_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLogpushJob {
    pub tf_id: std::string::String,
    #[doc =
        "Uniquely identifies a resource (such as an s3 bucket) where data. will be pushed. Additional configuration parameters supported by the destination may be included."]
    pub destination_conf: terrars::PrimField<std::string::String>,
}

impl BuildLogpushJob {
    pub fn build(self, stack: &mut terrars::Stack) -> LogpushJob {
        let out = LogpushJob(std::rc::Rc::new(LogpushJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LogpushJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                dataset: core::default::Default::default(),
                destination_conf: self.destination_conf,
                enabled: core::default::Default::default(),
                filter: core::default::Default::default(),
                filter_attack_traffic: core::default::Default::default(),
                frequency: core::default::Default::default(),
                kind: core::default::Default::default(),
                logpull_options: core::default::Default::default(),
                max_upload_bytes: core::default::Default::default(),
                max_upload_interval_seconds: core::default::Default::default(),
                max_upload_records: core::default::Default::default(),
                name: core::default::Default::default(),
                output_options: core::default::Default::default(),
                ownership_challenge: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LogpushJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LogpushJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LogpushJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nName of the dataset. A list of supported datasets can be found on the [Developer Docs](https://developers.cloudflare.com/logs/reference/log-fields/).\nAvailable values: \"access_requests\", \"account_abuse_protection_events\", \"audit_logs\", \"audit_logs_v2\", \"biso_user_actions\", \"casb_findings\", \"device_posture_results\", \"dex_application_tests\", \"dex_device_state_events\", \"dlp_forensic_copies\", \"dns_firewall_logs\", \"dns_logs\", \"email_security_alerts\", \"email_security_post_delivery_events\", \"firewall_events\", \"gateway_dns\", \"gateway_http\", \"gateway_network\", \"http_requests\", \"ipsec_logs\", \"magic_bgp_logs\", \"magic_ids_detections\", \"mcp_portal_logs\", \"mnm_flow_logs\", \"nel_reports\", \"network_analytics_logs\", \"page_shield_events\", \"sinkhole_http_logs\", \"spectrum_events\", \"ssh_logs\", \"turnstile_events\", \"warp_config_changes\", \"warp_toggle_changes\", \"websocket_analytics\", \"workers_trace_events\", \"zaraz_events\", \"zero_trust_network_sessions\"."]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destination_conf` after provisioning.\nUniquely identifies a resource (such as an s3 bucket) where data. will be pushed. Additional configuration parameters supported by the destination may be included."]
    pub fn destination_conf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_conf", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that indicates if the job is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error_message` after provisioning.\nIf not null, the job is currently failing. Failures are usually. repetitive (example: no permissions to write to destination bucket). Only the last failure is recorded. On successful execution of a job the error_message and last_error are set to null."]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nThe filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/filters/)."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter_attack_traffic` after provisioning.\nWhen true, excludes DDoS attack traffic from logs. This option is supported for the `http_requests`, `firewall_events`, and `network_analytics_logs` datasets."]
    pub fn filter_attack_traffic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_attack_traffic", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nThis field is deprecated. Please use `max_upload_*` parameters instead. . The frequency at which Cloudflare sends batches of logs to your destination. Setting frequency to high sends your logs in larger quantities of smaller files. Setting frequency to low sends logs in smaller quantities of larger files.\nAvailable values: \"high\", \"low\"."]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique id of the job."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe kind parameter (optional) is used to differentiate between Logpush and Edge Log Delivery jobs (when supported by the dataset).\nAvailable values: \"\", \"edge\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_complete` after provisioning.\nRecords the last time for which logs have been successfully pushed. If the last successful push was for logs range 2018-07-23T10:00:00Z to 2018-07-23T10:01:00Z then the value of this field will be 2018-07-23T10:01:00Z. If the job has never run or has just been enabled and hasn't run yet then the field will be empty."]
    pub fn last_complete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_complete", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_error` after provisioning.\nRecords the last time the job failed. If not null, the job is currently. failing. If null, the job has either never failed or has run successfully at least once since last failure. See also the error_message field."]
    pub fn last_error(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_error", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logpull_options` after provisioning.\nThis field is deprecated. Use `output_options` instead. Configuration string. It specifies things like requested fields and timestamp formats. If migrating from the logpull api, copy the url (full url or just the query string) of your call here, and logpush will keep on making this call for you, setting start and end times appropriately."]
    pub fn logpull_options(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpull_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_upload_bytes` after provisioning.\nThe maximum uncompressed file size of a batch of logs. This setting value must be between `5 MB` and `1 GB`, or `0` to disable it. Note that you cannot set a minimum file size; this means that log files may be much smaller than this batch size."]
    pub fn max_upload_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_upload_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_upload_interval_seconds` after provisioning.\nThe maximum interval in seconds for log batches. This setting must be between 30 and 300 seconds (5 minutes), or `0` to disable it. Note that you cannot specify a minimum interval for log batches; this means that log files may be sent in shorter intervals than this."]
    pub fn max_upload_interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_upload_interval_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_upload_records` after provisioning.\nThe maximum number of log lines per batch. This setting must be between 1000 and 1,000,000 lines, or `0` to disable it. Note that you cannot specify a minimum number of log lines per batch; this means that log files may contain many fewer lines than this."]
    pub fn max_upload_records(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_upload_records", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOptional human readable job name. Not unique. Cloudflare suggests. that you set this to a meaningful string, like the domain name, to make it easier to identify your job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `output_options` after provisioning.\nThe structured replacement for `logpull_options`. When including this field, the `logpull_option` field will be ignored."]
    pub fn output_options(&self) -> LogpushJob_OutputOptionsRef {
        LogpushJob_OutputOptionsRef::new(self.shared().clone(), format!("{}.output_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_challenge` after provisioning.\nOwnership challenge token to prove destination ownership."]
    pub fn ownership_challenge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_challenge", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LogpushJob_OutputOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cve_2021_44228: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    merge_subrequests: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LogpushJob_OutputOptions {
    #[doc = "Set the field `batch_prefix`.\nString to be prepended before each batch."]
    pub fn set_batch_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.batch_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `batch_suffix`.\nString to be appended after each batch."]
    pub fn set_batch_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.batch_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cve_2021_44228`.\nIf set to true, will cause all occurrences of `${` in the generated files to be replaced with `x{`."]
    pub fn set_cve_2021_44228(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cve_2021_44228 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `field_delimiter`.\nString to join fields. This field be ignored when `record_template` is set."]
    pub fn set_field_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `field_names`.\nList of field names to be included in the Logpush output. For the moment, there is no option to add all fields at once, so you must specify all the fields names you are interested in."]
    pub fn set_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.field_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `merge_subrequests`.\nIf set to true, subrequests will be merged into the parent request. Only supported for the `http_requests` dataset."]
    pub fn set_merge_subrequests(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.merge_subrequests = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_type`.\nSpecifies the output type, such as `ndjson` or `csv`. This sets default values for the rest of the settings, depending on the chosen output type. Some formatting rules, like string quoting, are different between output types.\nAvailable values: \"ndjson\", \"csv\"."]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_delimiter`.\nString to be inserted in-between the records as separator."]
    pub fn set_record_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_prefix`.\nString to be prepended before each record."]
    pub fn set_record_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_suffix`.\nString to be appended after each record."]
    pub fn set_record_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `record_template`.\nString to use as template for each record instead of the default json key value mapping. All fields used in the template must be present in `field_names` as well, otherwise they will end up as null. Format as a Go `text/template` without any standard functions, like conditionals, loops, sub-templates, etc."]
    pub fn set_record_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_template = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sample_rate`.\nSpecifies the sampling rate as a floating number greater than 0 and at most 1. Sampling is applied on top of filtering, and regardless of the current `sample_interval` of the data."]
    pub fn set_sample_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timestamp_format`.\nString to specify the format for timestamps, such as `unixnano`, `unix`, `rfc3339`, `rfc3339ms` or `rfc3339ns`.\nAvailable values: \"unixnano\", \"unix\", \"rfc3339\", \"rfc3339ms\", \"rfc3339ns\"."]
    pub fn set_timestamp_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timestamp_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LogpushJob_OutputOptions {
    type O = terrars::BlockAssignable<LogpushJob_OutputOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLogpushJob_OutputOptions {}

impl BuildLogpushJob_OutputOptions {
    pub fn build(self) -> LogpushJob_OutputOptions {
        LogpushJob_OutputOptions {
            batch_prefix: core::default::Default::default(),
            batch_suffix: core::default::Default::default(),
            cve_2021_44228: core::default::Default::default(),
            field_delimiter: core::default::Default::default(),
            field_names: core::default::Default::default(),
            merge_subrequests: core::default::Default::default(),
            output_type: core::default::Default::default(),
            record_delimiter: core::default::Default::default(),
            record_prefix: core::default::Default::default(),
            record_suffix: core::default::Default::default(),
            record_template: core::default::Default::default(),
            sample_rate: core::default::Default::default(),
            timestamp_format: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LogpushJob_OutputOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LogpushJob_OutputOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LogpushJob_OutputOptionsRef {
        LogpushJob_OutputOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LogpushJob_OutputOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `batch_prefix` after provisioning.\nString to be prepended before each batch."]
    pub fn batch_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `batch_suffix` after provisioning.\nString to be appended after each batch."]
    pub fn batch_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_suffix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cve_2021_44228` after provisioning.\nIf set to true, will cause all occurrences of `${` in the generated files to be replaced with `x{`."]
    pub fn cve_2021_44228(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cve_2021_44228", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field_delimiter` after provisioning.\nString to join fields. This field be ignored when `record_template` is set."]
    pub fn field_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_delimiter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field_names` after provisioning.\nList of field names to be included in the Logpush output. For the moment, there is no option to add all fields at once, so you must specify all the fields names you are interested in."]
    pub fn field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `merge_subrequests` after provisioning.\nIf set to true, subrequests will be merged into the parent request. Only supported for the `http_requests` dataset."]
    pub fn merge_subrequests(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.merge_subrequests", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_type` after provisioning.\nSpecifies the output type, such as `ndjson` or `csv`. This sets default values for the rest of the settings, depending on the chosen output type. Some formatting rules, like string quoting, are different between output types.\nAvailable values: \"ndjson\", \"csv\"."]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_delimiter` after provisioning.\nString to be inserted in-between the records as separator."]
    pub fn record_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_delimiter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_prefix` after provisioning.\nString to be prepended before each record."]
    pub fn record_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_suffix` after provisioning.\nString to be appended after each record."]
    pub fn record_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_suffix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_template` after provisioning.\nString to use as template for each record instead of the default json key value mapping. All fields used in the template must be present in `field_names` as well, otherwise they will end up as null. Format as a Go `text/template` without any standard functions, like conditionals, loops, sub-templates, etc."]
    pub fn record_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_template", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_rate` after provisioning.\nSpecifies the sampling rate as a floating number greater than 0 and at most 1. Sampling is applied on top of filtering, and regardless of the current `sample_interval` of the data."]
    pub fn sample_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timestamp_format` after provisioning.\nString to specify the format for timestamps, such as `unixnano`, `unix`, `rfc3339`, `rfc3339ms` or `rfc3339ns`.\nAvailable values: \"unixnano\", \"unix\", \"rfc3339\", \"rfc3339ms\", \"rfc3339ns\"."]
    pub fn timestamp_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp_format", self.base))
    }
}
