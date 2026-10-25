#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataCloudforceOneRequestPriorityData_ {
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
    priority_id: terrars::PrimField<std::string::String>,
}

struct DataCloudforceOneRequestPriority_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudforceOneRequestPriorityData_>,
}

#[derive(Clone)]
pub struct DataCloudforceOneRequestPriority(std::rc::Rc<DataCloudforceOneRequestPriority_>);

impl DataCloudforceOneRequestPriority {
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

    #[doc = "Get a reference to the value of field `priority_id` after provisioning.\nUUID."]
    pub fn priority_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `readable_id` after provisioning.\nReadable Request ID."]
    pub fn readable_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.readable_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request` after provisioning.\nRequested information from request."]
    pub fn request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request", self.extract_ref()))
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

impl terrars::Referable for DataCloudforceOneRequestPriority {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudforceOneRequestPriority { }

impl terrars::ToListMappable for DataCloudforceOneRequestPriority {
    type O = terrars::ListRef<DataCloudforceOneRequestPriorityRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudforceOneRequestPriority_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_cloudforce_one_request_priority".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudforceOneRequestPriority {
    pub tf_id: std::string::String,
    #[doc = "UUID."]
    pub priority_id: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudforceOneRequestPriority {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudforceOneRequestPriority {
        let out = DataCloudforceOneRequestPriority(std::rc::Rc::new(DataCloudforceOneRequestPriority_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudforceOneRequestPriorityData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                priority_id: self.priority_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudforceOneRequestPriorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudforceOneRequestPriorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudforceOneRequestPriorityRef {
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

    #[doc = "Get a reference to the value of field `priority_id` after provisioning.\nUUID."]
    pub fn priority_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `readable_id` after provisioning.\nReadable Request ID."]
    pub fn readable_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.readable_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request` after provisioning.\nRequested information from request."]
    pub fn request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request", self.extract_ref()))
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
