#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataLogpushDatasetFieldData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLogpushDatasetField_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLogpushDatasetFieldData_>,
}

#[derive(Clone)]
pub struct DataLogpushDatasetField(std::rc::Rc<DataLogpushDatasetField_>);

impl DataLogpushDatasetField {
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

    #[doc =
        "Set the field `dataset_id`.\nName of the dataset. A list of supported datasets can be found on the [Developer Docs](https://developers.cloudflare.com/logs/reference/log-fields/).\nAvailable values: \"access_requests\", \"account_abuse_protection_events\", \"audit_logs\", \"audit_logs_v2\", \"biso_user_actions\", \"casb_findings\", \"device_posture_results\", \"dex_application_tests\", \"dex_device_state_events\", \"dlp_forensic_copies\", \"dns_firewall_logs\", \"dns_logs\", \"email_security_alerts\", \"email_security_post_delivery_events\", \"firewall_events\", \"gateway_dns\", \"gateway_http\", \"gateway_network\", \"http_requests\", \"ipsec_logs\", \"magic_bgp_logs\", \"magic_ids_detections\", \"mcp_portal_logs\", \"mnm_flow_logs\", \"nel_reports\", \"network_analytics_logs\", \"page_shield_events\", \"sinkhole_http_logs\", \"spectrum_events\", \"ssh_logs\", \"turnstile_events\", \"warp_config_changes\", \"warp_toggle_changes\", \"websocket_analytics\", \"workers_trace_events\", \"zaraz_events\", \"zero_trust_network_sessions\"."]
    pub fn set_dataset_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dataset_id = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `dataset_id` after provisioning.\nName of the dataset. A list of supported datasets can be found on the [Developer Docs](https://developers.cloudflare.com/logs/reference/log-fields/).\nAvailable values: \"access_requests\", \"account_abuse_protection_events\", \"audit_logs\", \"audit_logs_v2\", \"biso_user_actions\", \"casb_findings\", \"device_posture_results\", \"dex_application_tests\", \"dex_device_state_events\", \"dlp_forensic_copies\", \"dns_firewall_logs\", \"dns_logs\", \"email_security_alerts\", \"email_security_post_delivery_events\", \"firewall_events\", \"gateway_dns\", \"gateway_http\", \"gateway_network\", \"http_requests\", \"ipsec_logs\", \"magic_bgp_logs\", \"magic_ids_detections\", \"mcp_portal_logs\", \"mnm_flow_logs\", \"nel_reports\", \"network_analytics_logs\", \"page_shield_events\", \"sinkhole_http_logs\", \"spectrum_events\", \"ssh_logs\", \"turnstile_events\", \"warp_config_changes\", \"warp_toggle_changes\", \"websocket_analytics\", \"workers_trace_events\", \"zaraz_events\", \"zero_trust_network_sessions\"."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataLogpushDatasetField {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLogpushDatasetField { }

impl terrars::ToListMappable for DataLogpushDatasetField {
    type O = terrars::ListRef<DataLogpushDatasetFieldRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLogpushDatasetField_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_logpush_dataset_field".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLogpushDatasetField {
    pub tf_id: std::string::String,
}

impl BuildDataLogpushDatasetField {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLogpushDatasetField {
        let out = DataLogpushDatasetField(std::rc::Rc::new(DataLogpushDatasetField_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLogpushDatasetFieldData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                dataset_id: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLogpushDatasetFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLogpushDatasetFieldRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLogpushDatasetFieldRef {
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
        "Get a reference to the value of field `dataset_id` after provisioning.\nName of the dataset. A list of supported datasets can be found on the [Developer Docs](https://developers.cloudflare.com/logs/reference/log-fields/).\nAvailable values: \"access_requests\", \"account_abuse_protection_events\", \"audit_logs\", \"audit_logs_v2\", \"biso_user_actions\", \"casb_findings\", \"device_posture_results\", \"dex_application_tests\", \"dex_device_state_events\", \"dlp_forensic_copies\", \"dns_firewall_logs\", \"dns_logs\", \"email_security_alerts\", \"email_security_post_delivery_events\", \"firewall_events\", \"gateway_dns\", \"gateway_http\", \"gateway_network\", \"http_requests\", \"ipsec_logs\", \"magic_bgp_logs\", \"magic_ids_detections\", \"mcp_portal_logs\", \"mnm_flow_logs\", \"nel_reports\", \"network_analytics_logs\", \"page_shield_events\", \"sinkhole_http_logs\", \"spectrum_events\", \"ssh_logs\", \"turnstile_events\", \"warp_config_changes\", \"warp_toggle_changes\", \"websocket_analytics\", \"workers_trace_events\", \"zaraz_events\", \"zero_trust_network_sessions\"."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe Zone ID to use for this endpoint. Mutually exclusive with the Account ID."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
