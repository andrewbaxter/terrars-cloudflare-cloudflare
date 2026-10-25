#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataEmailSecurityDomainData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataEmailSecurityDomain_Filter>,
}

struct DataEmailSecurityDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmailSecurityDomainData_>,
}

#[derive(Clone)]
pub struct DataEmailSecurityDomain(std::rc::Rc<DataEmailSecurityDomain_>);

impl DataEmailSecurityDomain {
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

    #[doc = "Set the field `domain_id`.\nDomain identifier."]
    pub fn set_domain_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().domain_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataEmailSecurityDomain_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_delivery_modes` after provisioning.\n"]
    pub fn allowed_delivery_modes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_delivery_modes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> DataEmailSecurityDomain_AuthorizationRef {
        DataEmailSecurityDomain_AuthorizationRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dmarc_status` after provisioning.\nAvailable values: \"none\", \"good\", \"invalid\"."]
    pub fn dmarc_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dmarc_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\nDomain identifier."]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `drop_dispositions` after provisioning.\n"]
    pub fn drop_dispositions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.drop_dispositions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails_processed` after provisioning.\n"]
    pub fn emails_processed(&self) -> DataEmailSecurityDomain_EmailsProcessedRef {
        DataEmailSecurityDomain_EmailsProcessedRef::new(
            self.shared().clone(),
            format!("{}.emails_processed", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataEmailSecurityDomain_FilterRef {
        DataEmailSecurityDomain_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `folder` after provisioning.\nThe mailbox folder to scan, for API-scanning domains.\nAvailable values: \"AllItems\", \"Inbox\"."]
    pub fn folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nDomain identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inbox_provider` after provisioning.\nAvailable values: \"Microsoft\", \"Google\"."]
    pub fn inbox_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbox_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\n"]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_restrictions` after provisioning.\n"]
    pub fn ip_restrictions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_restrictions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lookback_hops` after provisioning.\n"]
    pub fn lookback_hops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookback_hops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `o365_tenant_id` after provisioning.\n"]
    pub fn o365_tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.o365_tenant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_tls_inbound` after provisioning.\n"]
    pub fn require_tls_inbound(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls_inbound", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_tls_outbound` after provisioning.\n"]
    pub fn require_tls_outbound(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls_outbound", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `spf_status` after provisioning.\nAvailable values: \"none\", \"good\", \"neutral\", \"open\", \"invalid\"."]
    pub fn spf_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spf_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"PENDING\", \"ACTIVE\", \"FAILED\", \"TIMEOUT\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transport` after provisioning.\n"]
    pub fn transport(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport", self.extract_ref()))
    }
}

impl terrars::Referable for DataEmailSecurityDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmailSecurityDomain { }

impl terrars::ToListMappable for DataEmailSecurityDomain {
    type O = terrars::ListRef<DataEmailSecurityDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmailSecurityDomain_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_email_security_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmailSecurityDomain {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataEmailSecurityDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmailSecurityDomain {
        let out = DataEmailSecurityDomain(std::rc::Rc::new(DataEmailSecurityDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEmailSecurityDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                domain_id: core::default::Default::default(),
                filter: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmailSecurityDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmailSecurityDomainRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_delivery_modes` after provisioning.\n"]
    pub fn allowed_delivery_modes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_delivery_modes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> DataEmailSecurityDomain_AuthorizationRef {
        DataEmailSecurityDomain_AuthorizationRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dmarc_status` after provisioning.\nAvailable values: \"none\", \"good\", \"invalid\"."]
    pub fn dmarc_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dmarc_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\nDomain identifier."]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `drop_dispositions` after provisioning.\n"]
    pub fn drop_dispositions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.drop_dispositions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails_processed` after provisioning.\n"]
    pub fn emails_processed(&self) -> DataEmailSecurityDomain_EmailsProcessedRef {
        DataEmailSecurityDomain_EmailsProcessedRef::new(
            self.shared().clone(),
            format!("{}.emails_processed", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataEmailSecurityDomain_FilterRef {
        DataEmailSecurityDomain_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `folder` after provisioning.\nThe mailbox folder to scan, for API-scanning domains.\nAvailable values: \"AllItems\", \"Inbox\"."]
    pub fn folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nDomain identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inbox_provider` after provisioning.\nAvailable values: \"Microsoft\", \"Google\"."]
    pub fn inbox_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbox_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\n"]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_restrictions` after provisioning.\n"]
    pub fn ip_restrictions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_restrictions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lookback_hops` after provisioning.\n"]
    pub fn lookback_hops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookback_hops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `o365_tenant_id` after provisioning.\n"]
    pub fn o365_tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.o365_tenant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_tls_inbound` after provisioning.\n"]
    pub fn require_tls_inbound(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls_inbound", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_tls_outbound` after provisioning.\n"]
    pub fn require_tls_outbound(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls_outbound", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `spf_status` after provisioning.\nAvailable values: \"none\", \"good\", \"neutral\", \"open\", \"invalid\"."]
    pub fn spf_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spf_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"PENDING\", \"ACTIVE\", \"FAILED\", \"TIMEOUT\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transport` after provisioning.\n"]
    pub fn transport(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmailSecurityDomain_Authorization {}

impl DataEmailSecurityDomain_Authorization { }

impl terrars::ToListMappable for DataEmailSecurityDomain_Authorization {
    type O = terrars::BlockAssignable<DataEmailSecurityDomain_Authorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailSecurityDomain_Authorization {}

impl BuildDataEmailSecurityDomain_Authorization {
    pub fn build(self) -> DataEmailSecurityDomain_Authorization {
        DataEmailSecurityDomain_Authorization {}
    }
}

#[allow(dead_code)]
pub struct DataEmailSecurityDomain_AuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityDomain_AuthorizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailSecurityDomain_AuthorizationRef {
        DataEmailSecurityDomain_AuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailSecurityDomain_AuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorized` after provisioning.\n"]
    pub fn authorized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorized", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEmailSecurityDomain_EmailsProcessed {}

impl DataEmailSecurityDomain_EmailsProcessed { }

impl terrars::ToListMappable for DataEmailSecurityDomain_EmailsProcessed {
    type O = terrars::BlockAssignable<DataEmailSecurityDomain_EmailsProcessed>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailSecurityDomain_EmailsProcessed {}

impl BuildDataEmailSecurityDomain_EmailsProcessed {
    pub fn build(self) -> DataEmailSecurityDomain_EmailsProcessed {
        DataEmailSecurityDomain_EmailsProcessed {}
    }
}

#[allow(dead_code)]
pub struct DataEmailSecurityDomain_EmailsProcessedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityDomain_EmailsProcessedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailSecurityDomain_EmailsProcessedRef {
        DataEmailSecurityDomain_EmailsProcessedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailSecurityDomain_EmailsProcessedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `total_emails_processed` after provisioning.\n"]
    pub fn total_emails_processed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_emails_processed", self.base))
    }

    #[doc = "Get a reference to the value of field `total_emails_processed_previous` after provisioning.\n"]
    pub fn total_emails_processed_previous(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_emails_processed_previous", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEmailSecurityDomain_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_delivery_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_delivery_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEmailSecurityDomain_Filter {
    #[doc =
        "Set the field `active_delivery_mode`.\nCurrently active delivery mode to filter by.\nAvailable values: \"DIRECT\", \"BCC\", \"JOURNAL\", \"API\", \"RETRO_SCAN\"."]
    pub fn set_active_delivery_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.active_delivery_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_delivery_mode`.\nDelivery mode to filter by.\nAvailable values: \"DIRECT\", \"BCC\", \"JOURNAL\", \"API\", \"RETRO_SCAN\"."]
    pub fn set_allowed_delivery_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allowed_delivery_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\nDomain names to filter by."]
    pub fn set_domain(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integration_id`.\nIntegration ID to filter by."]
    pub fn set_integration_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.integration_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\nField to sort by.\nAvailable values: \"domain\", \"created_at\"."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nSearch term for filtering records. Behavior may change."]
    pub fn set_search(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nFilters response to domains with the provided status.\nAvailable values: \"PENDING\", \"ACTIVE\", \"FAILED\", \"TIMEOUT\"."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEmailSecurityDomain_Filter {
    type O = terrars::BlockAssignable<DataEmailSecurityDomain_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailSecurityDomain_Filter {}

impl BuildDataEmailSecurityDomain_Filter {
    pub fn build(self) -> DataEmailSecurityDomain_Filter {
        DataEmailSecurityDomain_Filter {
            active_delivery_mode: core::default::Default::default(),
            allowed_delivery_mode: core::default::Default::default(),
            direction: core::default::Default::default(),
            domain: core::default::Default::default(),
            integration_id: core::default::Default::default(),
            order: core::default::Default::default(),
            search: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEmailSecurityDomain_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityDomain_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailSecurityDomain_FilterRef {
        DataEmailSecurityDomain_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailSecurityDomain_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `active_delivery_mode` after provisioning.\nCurrently active delivery mode to filter by.\nAvailable values: \"DIRECT\", \"BCC\", \"JOURNAL\", \"API\", \"RETRO_SCAN\"."]
    pub fn active_delivery_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_delivery_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_delivery_mode` after provisioning.\nDelivery mode to filter by.\nAvailable values: \"DIRECT\", \"BCC\", \"JOURNAL\", \"API\", \"RETRO_SCAN\"."]
    pub fn allowed_delivery_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_delivery_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain names to filter by."]
    pub fn domain(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\nIntegration ID to filter by."]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to sort by.\nAvailable values: \"domain\", \"created_at\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch term for filtering records. Behavior may change."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nFilters response to domains with the provided status.\nAvailable values: \"PENDING\", \"ACTIVE\", \"FAILED\", \"TIMEOUT\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
