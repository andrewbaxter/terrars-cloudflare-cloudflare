#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataCloudforceOneRequestsData_ {
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
    completed_after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completed_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    page: terrars::PrimField<f64>,
    per_page: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudforceOneRequests_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudforceOneRequestsData_>,
}

#[derive(Clone)]
pub struct DataCloudforceOneRequests(std::rc::Rc<DataCloudforceOneRequests_>);

impl DataCloudforceOneRequests {
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

    #[doc = "Set the field `account_id`.\nIdentifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `completed_after`.\nRetrieve requests completed after this time."]
    pub fn set_completed_after(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().completed_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `completed_before`.\nRetrieve requests completed before this time."]
    pub fn set_completed_before(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().completed_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_after`.\nRetrieve requests created after this time."]
    pub fn set_created_after(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().created_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_before`.\nRetrieve requests created before this time."]
    pub fn set_created_before(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().created_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_type`.\nRequested information from request."]
    pub fn set_request_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().request_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_by`.\nField to sort results by."]
    pub fn set_sort_by(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sort_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_order`.\nSort order (asc or desc).\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_sort_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sort_order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nRequest Status.\nAvailable values: \"open\", \"accepted\", \"reported\", \"approved\", \"completed\", \"declined\"."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `completed_after` after provisioning.\nRetrieve requests completed after this time."]
    pub fn completed_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_after", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `completed_before` after provisioning.\nRetrieve requests completed before this time."]
    pub fn completed_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_before", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_after` after provisioning.\nRetrieve requests created after this time."]
    pub fn created_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_before` after provisioning.\nRetrieve requests created before this time."]
    pub fn created_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `page` after provisioning.\nPage number of results."]
    pub fn page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nNumber of results per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `request_type` after provisioning.\nRequested information from request."]
    pub fn request_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataCloudforceOneRequests_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort_by` after provisioning.\nField to sort results by."]
    pub fn sort_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sort_order` after provisioning.\nSort order (asc or desc).\nAvailable values: \"asc\", \"desc\"."]
    pub fn sort_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nRequest Status.\nAvailable values: \"open\", \"accepted\", \"reported\", \"approved\", \"completed\", \"declined\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudforceOneRequests {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudforceOneRequests { }

impl terrars::ToListMappable for DataCloudforceOneRequests {
    type O = terrars::ListRef<DataCloudforceOneRequestsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudforceOneRequests_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_cloudforce_one_requests".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudforceOneRequests {
    pub tf_id: std::string::String,
    #[doc = "Page number of results."]
    pub page: terrars::PrimField<f64>,
    #[doc = "Number of results per page."]
    pub per_page: terrars::PrimField<f64>,
}

impl BuildDataCloudforceOneRequests {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudforceOneRequests {
        let out = DataCloudforceOneRequests(std::rc::Rc::new(DataCloudforceOneRequests_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudforceOneRequestsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                completed_after: core::default::Default::default(),
                completed_before: core::default::Default::default(),
                created_after: core::default::Default::default(),
                created_before: core::default::Default::default(),
                max_items: core::default::Default::default(),
                page: self.page,
                per_page: self.per_page,
                request_type: core::default::Default::default(),
                sort_by: core::default::Default::default(),
                sort_order: core::default::Default::default(),
                status: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudforceOneRequestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudforceOneRequestsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudforceOneRequestsRef {
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
        "Get a reference to the value of field `completed_after` after provisioning.\nRetrieve requests completed after this time."]
    pub fn completed_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_after", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `completed_before` after provisioning.\nRetrieve requests completed before this time."]
    pub fn completed_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_before", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_after` after provisioning.\nRetrieve requests created after this time."]
    pub fn created_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_before` after provisioning.\nRetrieve requests created before this time."]
    pub fn created_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `page` after provisioning.\nPage number of results."]
    pub fn page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nNumber of results per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `request_type` after provisioning.\nRequested information from request."]
    pub fn request_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataCloudforceOneRequests_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort_by` after provisioning.\nField to sort results by."]
    pub fn sort_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sort_order` after provisioning.\nSort order (asc or desc).\nAvailable values: \"asc\", \"desc\"."]
    pub fn sort_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nRequest Status.\nAvailable values: \"open\", \"accepted\", \"reported\", \"approved\", \"completed\", \"declined\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudforceOneRequests_Result {}

impl DataCloudforceOneRequests_Result { }

impl terrars::ToListMappable for DataCloudforceOneRequests_Result {
    type O = terrars::BlockAssignable<DataCloudforceOneRequests_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudforceOneRequests_Result {}

impl BuildDataCloudforceOneRequests_Result {
    pub fn build(self) -> DataCloudforceOneRequests_Result {
        DataCloudforceOneRequests_Result {}
    }
}

#[allow(dead_code)]
pub struct DataCloudforceOneRequests_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudforceOneRequests_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudforceOneRequests_ResultRef {
        DataCloudforceOneRequests_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudforceOneRequests_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completed` after provisioning.\nRequest completion time."]
    pub fn completed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed", self.base))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\nRequest creation time."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `message_tokens` after provisioning.\nTokens for the request messages."]
    pub fn message_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_tokens", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nAvailable values: \"routine\", \"high\", \"urgent\"."]
    pub fn priority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `readable_id` after provisioning.\nReadable Request ID."]
    pub fn readable_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.readable_id", self.base))
    }

    #[doc = "Get a reference to the value of field `request` after provisioning.\nRequested information from request."]
    pub fn request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nRequest Status.\nAvailable values: \"open\", \"accepted\", \"reported\", \"approved\", \"completed\", \"declined\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `summary` after provisioning.\nBrief description of the request."]
    pub fn summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summary", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tlp` after provisioning.\nThe CISA defined Traffic Light Protocol (TLP).\nAvailable values: \"clear\", \"amber\", \"amber-strict\", \"green\", \"red\"."]
    pub fn tlp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tlp", self.base))
    }

    #[doc = "Get a reference to the value of field `tokens` after provisioning.\nTokens for the request."]
    pub fn tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `updated` after provisioning.\nRequest last updated time."]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.base))
    }
}
