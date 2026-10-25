#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataStreamCaptionLanguageData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    identifier: terrars::PrimField<std::string::String>,
    language: terrars::PrimField<std::string::String>,
}

struct DataStreamCaptionLanguage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStreamCaptionLanguageData_>,
}

#[derive(Clone)]
pub struct DataStreamCaptionLanguage(std::rc::Rc<DataStreamCaptionLanguage_>);

impl DataStreamCaptionLanguage {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated` after provisioning.\nWhether the caption was generated via AI."]
    pub fn generated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nThe language label displayed in the native language to users."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language` after provisioning.\nThe language tag in BCP 47 format."]
    pub fn language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of a generated caption.\nAvailable values: \"ready\", \"inprogress\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl terrars::Referable for DataStreamCaptionLanguage {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStreamCaptionLanguage { }

impl terrars::ToListMappable for DataStreamCaptionLanguage {
    type O = terrars::ListRef<DataStreamCaptionLanguageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStreamCaptionLanguage_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_stream_caption_language".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStreamCaptionLanguage {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "A Cloudflare-generated unique identifier for a media item."]
    pub identifier: terrars::PrimField<std::string::String>,
    #[doc = "The language tag in BCP 47 format."]
    pub language: terrars::PrimField<std::string::String>,
}

impl BuildDataStreamCaptionLanguage {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStreamCaptionLanguage {
        let out = DataStreamCaptionLanguage(std::rc::Rc::new(DataStreamCaptionLanguage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataStreamCaptionLanguageData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                identifier: self.identifier,
                language: self.language,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStreamCaptionLanguageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStreamCaptionLanguageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStreamCaptionLanguageRef {
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
        "Get a reference to the value of field `generated` after provisioning.\nWhether the caption was generated via AI."]
    pub fn generated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nThe language label displayed in the native language to users."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language` after provisioning.\nThe language tag in BCP 47 format."]
    pub fn language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of a generated caption.\nAvailable values: \"ready\", \"inprogress\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}
