use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct LogpushJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    dataset: PrimField<String>,
    destination_conf: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logpull_options: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_upload_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_upload_interval_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_upload_records: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ownership_challenge: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<PrimField<String>>,
}

struct LogpushJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LogpushJobData>,
}

#[derive(Clone)]
pub struct LogpushJob(Rc<LogpushJob_>);

impl LogpushJob {
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

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`."]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nWhether to enable the job."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `filter`.\nUse filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/logpush-api-configuration/filters/)."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }

    #[doc= "Set the field `frequency`.\nA higher frequency will result in logs being pushed on faster with smaller files. `low` frequency will push logs less often with larger files. Available values: `high`, `low`. Defaults to `high`."]
    pub fn set_frequency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().frequency = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `kind`.\nThe kind of logpush job to create. Available values: `edge`, `instant-logs`, `\"\"`."]
    pub fn set_kind(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kind = Some(v.into());
        self
    }

    #[doc= "Set the field `logpull_options`.\nConfiguration string for the Logshare API. It specifies things like requested fields and timestamp formats. See [Logpush options documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#options)."]
    pub fn set_logpull_options(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().logpull_options = Some(v.into());
        self
    }

    #[doc= "Set the field `max_upload_bytes`.\nThe maximum uncompressed file size of a batch of logs. Value must be between 5MB and 1GB."]
    pub fn set_max_upload_bytes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_upload_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `max_upload_interval_seconds`.\nThe maximum interval in seconds for log batches. Value must be between 30 and 300."]
    pub fn set_max_upload_interval_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_upload_interval_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `max_upload_records`.\nThe maximum number of log lines per batch. Value must be between 1000 and 1,000,000."]
    pub fn set_max_upload_records(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_upload_records = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nThe name of the logpush job to create."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `ownership_challenge`.\nOwnership challenge token to prove destination ownership, required when destination is Amazon S3, Google Cloud Storage, Microsoft Azure or Sumo Logic. See [Developer documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#usage)."]
    pub fn set_ownership_challenge(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ownership_challenge = Some(v.into());
        self
    }

    #[doc= "Set the field `zone_id`.\nThe zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`."]
    pub fn set_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone_id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dataset` after provisioning.\nThe kind of the dataset to use with the logpush job. Available values: `access_requests`, `firewall_events`, `http_requests`, `spectrum_events`, `nel_reports`, `audit_logs`, `gateway_dns`, `gateway_http`, `gateway_network`, `dns_logs`, `network_analytics_logs`, `workers_trace_events`, `device_posture_results`, `zero_trust_network_sessions`."]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination_conf` after provisioning.\nUniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/reference/logpush-api-configuration#destination)."]
    pub fn destination_conf(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_conf", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable the job."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\nUse filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/logpush-api-configuration/filters/)."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `frequency` after provisioning.\nA higher frequency will result in logs being pushed on faster with smaller files. `low` frequency will push logs less often with larger files. Available values: `high`, `low`. Defaults to `high`."]
    pub fn frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nThe kind of logpush job to create. Available values: `edge`, `instant-logs`, `\"\"`."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logpull_options` after provisioning.\nConfiguration string for the Logshare API. It specifies things like requested fields and timestamp formats. See [Logpush options documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#options)."]
    pub fn logpull_options(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logpull_options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_upload_bytes` after provisioning.\nThe maximum uncompressed file size of a batch of logs. Value must be between 5MB and 1GB."]
    pub fn max_upload_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_upload_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_upload_interval_seconds` after provisioning.\nThe maximum interval in seconds for log batches. Value must be between 30 and 300."]
    pub fn max_upload_interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_upload_interval_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_upload_records` after provisioning.\nThe maximum number of log lines per batch. Value must be between 1000 and 1,000,000."]
    pub fn max_upload_records(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_upload_records", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the logpush job to create."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ownership_challenge` after provisioning.\nOwnership challenge token to prove destination ownership, required when destination is Amazon S3, Google Cloud Storage, Microsoft Azure or Sumo Logic. See [Developer documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#usage)."]
    pub fn ownership_challenge(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ownership_challenge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for LogpushJob {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LogpushJob { }

impl ToListMappable for LogpushJob {
    type O = ListRef<LogpushJobRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LogpushJob_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_logpush_job".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLogpushJob {
    pub tf_id: String,
    #[doc= "The kind of the dataset to use with the logpush job. Available values: `access_requests`, `firewall_events`, `http_requests`, `spectrum_events`, `nel_reports`, `audit_logs`, `gateway_dns`, `gateway_http`, `gateway_network`, `dns_logs`, `network_analytics_logs`, `workers_trace_events`, `device_posture_results`, `zero_trust_network_sessions`."]
    pub dataset: PrimField<String>,
    #[doc= "Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/reference/logpush-api-configuration#destination)."]
    pub destination_conf: PrimField<String>,
}

impl BuildLogpushJob {
    pub fn build(self, stack: &mut Stack) -> LogpushJob {
        let out = LogpushJob(Rc::new(LogpushJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LogpushJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                dataset: self.dataset,
                destination_conf: self.destination_conf,
                enabled: core::default::Default::default(),
                filter: core::default::Default::default(),
                frequency: core::default::Default::default(),
                id: core::default::Default::default(),
                kind: core::default::Default::default(),
                logpull_options: core::default::Default::default(),
                max_upload_bytes: core::default::Default::default(),
                max_upload_interval_seconds: core::default::Default::default(),
                max_upload_records: core::default::Default::default(),
                name: core::default::Default::default(),
                ownership_challenge: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LogpushJobRef {
    shared: StackShared,
    base: String,
}

impl Ref for LogpushJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LogpushJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dataset` after provisioning.\nThe kind of the dataset to use with the logpush job. Available values: `access_requests`, `firewall_events`, `http_requests`, `spectrum_events`, `nel_reports`, `audit_logs`, `gateway_dns`, `gateway_http`, `gateway_network`, `dns_logs`, `network_analytics_logs`, `workers_trace_events`, `device_posture_results`, `zero_trust_network_sessions`."]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination_conf` after provisioning.\nUniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/reference/logpush-api-configuration#destination)."]
    pub fn destination_conf(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_conf", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable the job."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\nUse filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/logpush-api-configuration/filters/)."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `frequency` after provisioning.\nA higher frequency will result in logs being pushed on faster with smaller files. `low` frequency will push logs less often with larger files. Available values: `high`, `low`. Defaults to `high`."]
    pub fn frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nThe kind of logpush job to create. Available values: `edge`, `instant-logs`, `\"\"`."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logpull_options` after provisioning.\nConfiguration string for the Logshare API. It specifies things like requested fields and timestamp formats. See [Logpush options documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#options)."]
    pub fn logpull_options(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logpull_options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_upload_bytes` after provisioning.\nThe maximum uncompressed file size of a batch of logs. Value must be between 5MB and 1GB."]
    pub fn max_upload_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_upload_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_upload_interval_seconds` after provisioning.\nThe maximum interval in seconds for log batches. Value must be between 30 and 300."]
    pub fn max_upload_interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_upload_interval_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_upload_records` after provisioning.\nThe maximum number of log lines per batch. Value must be between 1000 and 1,000,000."]
    pub fn max_upload_records(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_upload_records", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the logpush job to create."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ownership_challenge` after provisioning.\nOwnership challenge token to prove destination ownership, required when destination is Amazon S3, Google Cloud Storage, Microsoft Azure or Sumo Logic. See [Developer documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#usage)."]
    pub fn ownership_challenge(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ownership_challenge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
