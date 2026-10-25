#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataFieldExtractorData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    extractor: terrars::PrimField<std::string::String>,
}

struct DataFieldExtractor_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataFieldExtractorData_>,
}

#[derive(Clone)]
pub struct DataFieldExtractor(std::rc::Rc<DataFieldExtractor_>);

impl DataFieldExtractor {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extractor` after provisioning.\nExtractor type."]
    pub fn extractor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extractor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataFieldExtractor_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl terrars::Referable for DataFieldExtractor {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataFieldExtractor { }

impl terrars::ToListMappable for DataFieldExtractor {
    type O = terrars::ListRef<DataFieldExtractorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataFieldExtractor_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_field_extractor".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFieldExtractor {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Extractor type."]
    pub extractor: terrars::PrimField<std::string::String>,
}

impl BuildDataFieldExtractor {
    pub fn build(self, stack: &mut terrars::Stack) -> DataFieldExtractor {
        let out = DataFieldExtractor(std::rc::Rc::new(DataFieldExtractor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataFieldExtractorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                extractor: self.extractor,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataFieldExtractorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFieldExtractorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataFieldExtractorRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extractor` after provisioning.\nExtractor type."]
    pub fn extractor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extractor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataFieldExtractor_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataFieldExtractor_Rules_Fields {}

impl DataFieldExtractor_Rules_Fields { }

impl terrars::ToListMappable for DataFieldExtractor_Rules_Fields {
    type O = terrars::BlockAssignable<DataFieldExtractor_Rules_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFieldExtractor_Rules_Fields {}

impl BuildDataFieldExtractor_Rules_Fields {
    pub fn build(self) -> DataFieldExtractor_Rules_Fields {
        DataFieldExtractor_Rules_Fields {}
    }
}

#[allow(dead_code)]
pub struct DataFieldExtractor_Rules_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFieldExtractor_Rules_FieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFieldExtractor_Rules_FieldsRef {
        DataFieldExtractor_Rules_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFieldExtractor_Rules_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\nWirefilter value expression."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nField name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFieldExtractor_Rules {}

impl DataFieldExtractor_Rules { }

impl terrars::ToListMappable for DataFieldExtractor_Rules {
    type O = terrars::BlockAssignable<DataFieldExtractor_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFieldExtractor_Rules {}

impl BuildDataFieldExtractor_Rules {
    pub fn build(self) -> DataFieldExtractor_Rules {
        DataFieldExtractor_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataFieldExtractor_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFieldExtractor_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFieldExtractor_RulesRef {
        DataFieldExtractor_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFieldExtractor_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable rule description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<DataFieldExtractor_Rules_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }

    #[doc = "Get a reference to the value of field `ref_` after provisioning.\nStable rule identifier."]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
}
