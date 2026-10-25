#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataEmailSecurityDomainsData_ {
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
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataEmailSecurityDomains_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmailSecurityDomainsData_>,
}

#[derive(Clone)]
pub struct DataEmailSecurityDomains(std::rc::Rc<DataEmailSecurityDomains_>);

impl DataEmailSecurityDomains {
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
        "Set the field `active_delivery_mode`.\nCurrently active delivery mode to filter by.\nAvailable values: \"DIRECT\", \"BCC\", \"JOURNAL\", \"API\", \"RETRO_SCAN\"."]
    pub fn set_active_delivery_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().active_delivery_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_delivery_mode`.\nDelivery mode to filter by.\nAvailable values: \"DIRECT\", \"BCC\", \"JOURNAL\", \"API\", \"RETRO_SCAN\"."]
    pub fn set_allowed_delivery_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().allowed_delivery_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\nDomain names to filter by."]
    pub fn set_domain(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integration_id`.\nIntegration ID to filter by."]
    pub fn set_integration_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().integration_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\nField to sort by.\nAvailable values: \"domain\", \"created_at\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nSearch term for filtering records. Behavior may change."]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nFilters response to domains with the provided status.\nAvailable values: \"PENDING\", \"ACTIVE\", \"FAILED\", \"TIMEOUT\"."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `active_delivery_mode` after provisioning.\nCurrently active delivery mode to filter by.\nAvailable values: \"DIRECT\", \"BCC\", \"JOURNAL\", \"API\", \"RETRO_SCAN\"."]
    pub fn active_delivery_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_delivery_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_delivery_mode` after provisioning.\nDelivery mode to filter by.\nAvailable values: \"DIRECT\", \"BCC\", \"JOURNAL\", \"API\", \"RETRO_SCAN\"."]
    pub fn allowed_delivery_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_delivery_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain names to filter by."]
    pub fn domain(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\nIntegration ID to filter by."]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to sort by.\nAvailable values: \"domain\", \"created_at\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataEmailSecurityDomains_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch term for filtering records. Behavior may change."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nFilters response to domains with the provided status.\nAvailable values: \"PENDING\", \"ACTIVE\", \"FAILED\", \"TIMEOUT\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl terrars::Referable for DataEmailSecurityDomains {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmailSecurityDomains { }

impl terrars::ToListMappable for DataEmailSecurityDomains {
    type O = terrars::ListRef<DataEmailSecurityDomainsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmailSecurityDomains_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_email_security_domains".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmailSecurityDomains {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataEmailSecurityDomains {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmailSecurityDomains {
        let out = DataEmailSecurityDomains(std::rc::Rc::new(DataEmailSecurityDomains_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEmailSecurityDomainsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                active_delivery_mode: core::default::Default::default(),
                allowed_delivery_mode: core::default::Default::default(),
                direction: core::default::Default::default(),
                domain: core::default::Default::default(),
                integration_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                order: core::default::Default::default(),
                search: core::default::Default::default(),
                status: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmailSecurityDomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityDomainsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmailSecurityDomainsRef {
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

    #[doc =
        "Get a reference to the value of field `active_delivery_mode` after provisioning.\nCurrently active delivery mode to filter by.\nAvailable values: \"DIRECT\", \"BCC\", \"JOURNAL\", \"API\", \"RETRO_SCAN\"."]
    pub fn active_delivery_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_delivery_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_delivery_mode` after provisioning.\nDelivery mode to filter by.\nAvailable values: \"DIRECT\", \"BCC\", \"JOURNAL\", \"API\", \"RETRO_SCAN\"."]
    pub fn allowed_delivery_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_delivery_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sorting direction.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain names to filter by."]
    pub fn domain(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\nIntegration ID to filter by."]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to sort by.\nAvailable values: \"domain\", \"created_at\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataEmailSecurityDomains_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `search` after provisioning.\nSearch term for filtering records. Behavior may change."]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nFilters response to domains with the provided status.\nAvailable values: \"PENDING\", \"ACTIVE\", \"FAILED\", \"TIMEOUT\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmailSecurityDomains_Result_Authorization {}

impl DataEmailSecurityDomains_Result_Authorization { }

impl terrars::ToListMappable for DataEmailSecurityDomains_Result_Authorization {
    type O = terrars::BlockAssignable<DataEmailSecurityDomains_Result_Authorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailSecurityDomains_Result_Authorization {}

impl BuildDataEmailSecurityDomains_Result_Authorization {
    pub fn build(self) -> DataEmailSecurityDomains_Result_Authorization {
        DataEmailSecurityDomains_Result_Authorization {}
    }
}

#[allow(dead_code)]
pub struct DataEmailSecurityDomains_Result_AuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityDomains_Result_AuthorizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEmailSecurityDomains_Result_AuthorizationRef {
        DataEmailSecurityDomains_Result_AuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailSecurityDomains_Result_AuthorizationRef {
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
pub struct DataEmailSecurityDomains_Result_EmailsProcessed {}

impl DataEmailSecurityDomains_Result_EmailsProcessed { }

impl terrars::ToListMappable for DataEmailSecurityDomains_Result_EmailsProcessed {
    type O = terrars::BlockAssignable<DataEmailSecurityDomains_Result_EmailsProcessed>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailSecurityDomains_Result_EmailsProcessed {}

impl BuildDataEmailSecurityDomains_Result_EmailsProcessed {
    pub fn build(self) -> DataEmailSecurityDomains_Result_EmailsProcessed {
        DataEmailSecurityDomains_Result_EmailsProcessed {}
    }
}

#[allow(dead_code)]
pub struct DataEmailSecurityDomains_Result_EmailsProcessedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityDomains_Result_EmailsProcessedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEmailSecurityDomains_Result_EmailsProcessedRef {
        DataEmailSecurityDomains_Result_EmailsProcessedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailSecurityDomains_Result_EmailsProcessedRef {
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
pub struct DataEmailSecurityDomains_Result {}

impl DataEmailSecurityDomains_Result { }

impl terrars::ToListMappable for DataEmailSecurityDomains_Result {
    type O = terrars::BlockAssignable<DataEmailSecurityDomains_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmailSecurityDomains_Result {}

impl BuildDataEmailSecurityDomains_Result {
    pub fn build(self) -> DataEmailSecurityDomains_Result {
        DataEmailSecurityDomains_Result {}
    }
}

#[allow(dead_code)]
pub struct DataEmailSecurityDomains_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmailSecurityDomains_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmailSecurityDomains_ResultRef {
        DataEmailSecurityDomains_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmailSecurityDomains_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_delivery_modes` after provisioning.\n"]
    pub fn allowed_delivery_modes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_delivery_modes", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> DataEmailSecurityDomains_Result_AuthorizationRef {
        DataEmailSecurityDomains_Result_AuthorizationRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dmarc_status` after provisioning.\nAvailable values: \"none\", \"good\", \"invalid\"."]
    pub fn dmarc_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dmarc_status", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `drop_dispositions` after provisioning.\n"]
    pub fn drop_dispositions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.drop_dispositions", self.base))
    }

    #[doc = "Get a reference to the value of field `emails_processed` after provisioning.\n"]
    pub fn emails_processed(&self) -> DataEmailSecurityDomains_Result_EmailsProcessedRef {
        DataEmailSecurityDomains_Result_EmailsProcessedRef::new(
            self.shared().clone(),
            format!("{}.emails_processed", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `folder` after provisioning.\nThe mailbox folder to scan, for API-scanning domains.\nAvailable values: \"AllItems\", \"Inbox\"."]
    pub fn folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nDomain identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inbox_provider` after provisioning.\nAvailable values: \"Microsoft\", \"Google\"."]
    pub fn inbox_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbox_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\n"]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_restrictions` after provisioning.\n"]
    pub fn ip_restrictions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_restrictions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.base))
    }

    #[doc = "Get a reference to the value of field `lookback_hops` after provisioning.\n"]
    pub fn lookback_hops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookback_hops", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.base))
    }

    #[doc = "Get a reference to the value of field `o365_tenant_id` after provisioning.\n"]
    pub fn o365_tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.o365_tenant_id", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `require_tls_inbound` after provisioning.\n"]
    pub fn require_tls_inbound(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls_inbound", self.base))
    }

    #[doc = "Get a reference to the value of field `require_tls_outbound` after provisioning.\n"]
    pub fn require_tls_outbound(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls_outbound", self.base))
    }

    #[doc =
        "Get a reference to the value of field `spf_status` after provisioning.\nAvailable values: \"none\", \"good\", \"neutral\", \"open\", \"invalid\"."]
    pub fn spf_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spf_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"PENDING\", \"ACTIVE\", \"FAILED\", \"TIMEOUT\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `transport` after provisioning.\n"]
    pub fn transport(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport", self.base))
    }
}
