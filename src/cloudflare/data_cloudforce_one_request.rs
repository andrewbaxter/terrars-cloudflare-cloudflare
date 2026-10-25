#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataCloudforceOneRequestData_ {
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
    filter: std::option::Option<DataCloudforceOneRequest_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudforceOneRequest_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudforceOneRequestData_>,
}

#[derive(Clone)]
pub struct DataCloudforceOneRequest(std::rc::Rc<DataCloudforceOneRequest_>);

impl DataCloudforceOneRequest {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataCloudforceOneRequest_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_id`.\nUUID."]
    pub fn set_request_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().request_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `completed` after provisioning.\n"]
    pub fn completed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nRequest content."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataCloudforceOneRequest_FilterRef {
        DataCloudforceOneRequest_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `message_tokens` after provisioning.\nTokens for the request messages."]
    pub fn message_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `readable_id` after provisioning.\nReadable Request ID."]
    pub fn readable_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.readable_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request` after provisioning.\nRequested information from request."]
    pub fn request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_id` after provisioning.\nUUID."]
    pub fn request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nRequest Status.\nAvailable values: \"open\", \"accepted\", \"reported\", \"approved\", \"completed\", \"declined\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `summary` after provisioning.\nBrief description of the request."]
    pub fn summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summary", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tlp` after provisioning.\nThe CISA defined Traffic Light Protocol (TLP).\nAvailable values: \"clear\", \"amber\", \"amber-strict\", \"green\", \"red\"."]
    pub fn tlp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tlp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tokens` after provisioning.\nTokens for the request."]
    pub fn tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated` after provisioning.\n"]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudforceOneRequest {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudforceOneRequest { }

impl terrars::ToListMappable for DataCloudforceOneRequest {
    type O = terrars::ListRef<DataCloudforceOneRequestRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudforceOneRequest_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_cloudforce_one_request".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudforceOneRequest {
    pub tf_id: std::string::String,
}

impl BuildDataCloudforceOneRequest {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudforceOneRequest {
        let out = DataCloudforceOneRequest(std::rc::Rc::new(DataCloudforceOneRequest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudforceOneRequestData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                request_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudforceOneRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudforceOneRequestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudforceOneRequestRef {
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

    #[doc = "Get a reference to the value of field `completed` after provisioning.\n"]
    pub fn completed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nRequest content."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataCloudforceOneRequest_FilterRef {
        DataCloudforceOneRequest_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `message_tokens` after provisioning.\nTokens for the request messages."]
    pub fn message_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `readable_id` after provisioning.\nReadable Request ID."]
    pub fn readable_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.readable_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request` after provisioning.\nRequested information from request."]
    pub fn request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_id` after provisioning.\nUUID."]
    pub fn request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nRequest Status.\nAvailable values: \"open\", \"accepted\", \"reported\", \"approved\", \"completed\", \"declined\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `summary` after provisioning.\nBrief description of the request."]
    pub fn summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summary", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tlp` after provisioning.\nThe CISA defined Traffic Light Protocol (TLP).\nAvailable values: \"clear\", \"amber\", \"amber-strict\", \"green\", \"red\"."]
    pub fn tlp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tlp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tokens` after provisioning.\nTokens for the request."]
    pub fn tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated` after provisioning.\n"]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudforceOneRequest_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completed_after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completed_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_before: std::option::Option<terrars::PrimField<std::string::String>>,
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

impl DataCloudforceOneRequest_Filter {
    #[doc = "Set the field `completed_after`.\nRetrieve requests completed after this time."]
    pub fn set_completed_after(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.completed_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `completed_before`.\nRetrieve requests completed before this time."]
    pub fn set_completed_before(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.completed_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_after`.\nRetrieve requests created after this time."]
    pub fn set_created_after(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_before`.\nRetrieve requests created before this time."]
    pub fn set_created_before(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_type`.\nRequested information from request."]
    pub fn set_request_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_by`.\nField to sort results by."]
    pub fn set_sort_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sort_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_order`.\nSort order (asc or desc).\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_sort_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sort_order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nRequest Status.\nAvailable values: \"open\", \"accepted\", \"reported\", \"approved\", \"completed\", \"declined\"."]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudforceOneRequest_Filter {
    type O = terrars::BlockAssignable<DataCloudforceOneRequest_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudforceOneRequest_Filter {
    #[doc = "Page number of results."]
    pub page: terrars::PrimField<f64>,
    #[doc = "Number of results per page."]
    pub per_page: terrars::PrimField<f64>,
}

impl BuildDataCloudforceOneRequest_Filter {
    pub fn build(self) -> DataCloudforceOneRequest_Filter {
        DataCloudforceOneRequest_Filter {
            completed_after: core::default::Default::default(),
            completed_before: core::default::Default::default(),
            created_after: core::default::Default::default(),
            created_before: core::default::Default::default(),
            page: self.page,
            per_page: self.per_page,
            request_type: core::default::Default::default(),
            sort_by: core::default::Default::default(),
            sort_order: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudforceOneRequest_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudforceOneRequest_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudforceOneRequest_FilterRef {
        DataCloudforceOneRequest_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudforceOneRequest_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `completed_after` after provisioning.\nRetrieve requests completed after this time."]
    pub fn completed_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `completed_before` after provisioning.\nRetrieve requests completed before this time."]
    pub fn completed_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_before", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_after` after provisioning.\nRetrieve requests created after this time."]
    pub fn created_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_before` after provisioning.\nRetrieve requests created before this time."]
    pub fn created_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.base))
    }

    #[doc = "Get a reference to the value of field `page` after provisioning.\nPage number of results."]
    pub fn page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.base))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nNumber of results per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_type` after provisioning.\nRequested information from request."]
    pub fn request_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_by` after provisioning.\nField to sort results by."]
    pub fn sort_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sort_order` after provisioning.\nSort order (asc or desc).\nAvailable values: \"asc\", \"desc\"."]
    pub fn sort_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nRequest Status.\nAvailable values: \"open\", \"accepted\", \"reported\", \"approved\", \"completed\", \"declined\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
