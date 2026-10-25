#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataCloudforceOneRequestMessageData_ {
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
    after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before: std::option::Option<terrars::PrimField<std::string::String>>,
    page: terrars::PrimField<f64>,
    per_page: terrars::PrimField<f64>,
    request_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_order: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudforceOneRequestMessage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudforceOneRequestMessageData_>,
}

#[derive(Clone)]
pub struct DataCloudforceOneRequestMessage(std::rc::Rc<DataCloudforceOneRequestMessage_>);

impl DataCloudforceOneRequestMessage {
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

    #[doc = "Set the field `after`.\nRetrieve mes  ges created after this time."]
    pub fn set_after(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `before`.\nRetrieve messages created before this time."]
    pub fn set_before(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().before = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `after` after provisioning.\nRetrieve mes  ges created after this time."]
    pub fn after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `author` after provisioning.\nAuthor of message."]
    pub fn author(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `before` after provisioning.\nRetrieve messages created before this time."]
    pub fn before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nContent of message."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\nDefines the message creation time."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_follow_on_request` after provisioning.\nWhether the message is a follow-on request."]
    pub fn is_follow_on_request(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_follow_on_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `page` after provisioning.\nPage number of results."]
    pub fn page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nNumber of results per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_id` after provisioning.\nUUID."]
    pub fn request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_id", self.extract_ref()))
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
        "Get a reference to the value of field `updated` after provisioning.\nDefines the message last updated time."]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudforceOneRequestMessage {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudforceOneRequestMessage { }

impl terrars::ToListMappable for DataCloudforceOneRequestMessage {
    type O = terrars::ListRef<DataCloudforceOneRequestMessageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudforceOneRequestMessage_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_cloudforce_one_request_message".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudforceOneRequestMessage {
    pub tf_id: std::string::String,
    #[doc = "Page number of results."]
    pub page: terrars::PrimField<f64>,
    #[doc = "Number of results per page."]
    pub per_page: terrars::PrimField<f64>,
    #[doc = "UUID."]
    pub request_id: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudforceOneRequestMessage {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudforceOneRequestMessage {
        let out = DataCloudforceOneRequestMessage(std::rc::Rc::new(DataCloudforceOneRequestMessage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudforceOneRequestMessageData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                after: core::default::Default::default(),
                before: core::default::Default::default(),
                page: self.page,
                per_page: self.per_page,
                request_id: self.request_id,
                sort_by: core::default::Default::default(),
                sort_order: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudforceOneRequestMessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudforceOneRequestMessageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudforceOneRequestMessageRef {
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
        "Get a reference to the value of field `after` after provisioning.\nRetrieve mes  ges created after this time."]
    pub fn after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `author` after provisioning.\nAuthor of message."]
    pub fn author(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `before` after provisioning.\nRetrieve messages created before this time."]
    pub fn before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nContent of message."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\nDefines the message creation time."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_follow_on_request` after provisioning.\nWhether the message is a follow-on request."]
    pub fn is_follow_on_request(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_follow_on_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `page` after provisioning.\nPage number of results."]
    pub fn page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nNumber of results per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_id` after provisioning.\nUUID."]
    pub fn request_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_id", self.extract_ref()))
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
        "Get a reference to the value of field `updated` after provisioning.\nDefines the message last updated time."]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }
}
