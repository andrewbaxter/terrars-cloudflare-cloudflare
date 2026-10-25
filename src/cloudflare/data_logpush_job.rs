#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataLogpushJobData_ {
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
    job_id: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLogpushJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLogpushJobData_>,
}

#[derive(Clone)]
pub struct DataLogpushJob(std::rc::Rc<DataLogpushJob_>);

impl DataLogpushJob {
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

    #[doc =
        "Set the field `account_id`.\nThe Account ID to use for this endpoint. Mutually exclusive with the Zone ID."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nUnique id of the job."]
    pub fn job_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
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
    pub fn output_options(&self) -> DataLogpushJob_OutputOptionsRef {
        DataLogpushJob_OutputOptionsRef::new(self.shared().clone(), format!("{}.output_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataLogpushJob {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLogpushJob { }

impl terrars::ToListMappable for DataLogpushJob {
    type O = terrars::ListRef<DataLogpushJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLogpushJob_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_logpush_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLogpushJob {
    pub tf_id: std::string::String,
    #[doc = "Unique id of the job."]
    pub job_id: terrars::PrimField<f64>,
}

impl BuildDataLogpushJob {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLogpushJob {
        let out = DataLogpushJob(std::rc::Rc::new(DataLogpushJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLogpushJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                job_id: self.job_id,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLogpushJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLogpushJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLogpushJobRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\nUnique id of the job."]
    pub fn job_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
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
    pub fn output_options(&self) -> DataLogpushJob_OutputOptionsRef {
        DataLogpushJob_OutputOptionsRef::new(self.shared().clone(), format!("{}.output_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLogpushJob_OutputOptions {}

impl DataLogpushJob_OutputOptions { }

impl terrars::ToListMappable for DataLogpushJob_OutputOptions {
    type O = terrars::BlockAssignable<DataLogpushJob_OutputOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLogpushJob_OutputOptions {}

impl BuildDataLogpushJob_OutputOptions {
    pub fn build(self) -> DataLogpushJob_OutputOptions {
        DataLogpushJob_OutputOptions {}
    }
}

#[allow(dead_code)]
pub struct DataLogpushJob_OutputOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLogpushJob_OutputOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLogpushJob_OutputOptionsRef {
        DataLogpushJob_OutputOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLogpushJob_OutputOptionsRef {
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
