#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataFlagshipFlagsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    app_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
}

struct DataFlagshipFlags_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataFlagshipFlagsData_>,
}

#[derive(Clone)]
pub struct DataFlagshipFlags(std::rc::Rc<DataFlagshipFlags_>);

impl DataFlagshipFlags {
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

    #[doc = "Set the field `limit`.\nMax items to return (1–200)."]
    pub fn set_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID that owns the Flagship app."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nFlagship app ID returned when the app was created."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limit` after provisioning.\nMax items to return (1–200)."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataFlagshipFlags_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataFlagshipFlags {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataFlagshipFlags { }

impl terrars::ToListMappable for DataFlagshipFlags {
    type O = terrars::ListRef<DataFlagshipFlagsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataFlagshipFlags_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_flagship_flags".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFlagshipFlags {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID that owns the Flagship app."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Flagship app ID returned when the app was created."]
    pub app_id: terrars::PrimField<std::string::String>,
}

impl BuildDataFlagshipFlags {
    pub fn build(self, stack: &mut terrars::Stack) -> DataFlagshipFlags {
        let out = DataFlagshipFlags(std::rc::Rc::new(DataFlagshipFlags_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataFlagshipFlagsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                app_id: self.app_id,
                limit: core::default::Default::default(),
                max_items: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataFlagshipFlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFlagshipFlagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataFlagshipFlagsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID that owns the Flagship app."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nFlagship app ID returned when the app was created."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limit` after provisioning.\nMax items to return (1–200)."]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataFlagshipFlags_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {}

impl DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses { }

impl terrars::ToListMappable for DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {
    type O =
        terrars::BlockAssignable<DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {}

impl BuildDataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {
    pub fn build(self) -> DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {
        DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_Clauses {}
    }
}

#[allow(dead_code)]
pub struct DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef {
        DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {}

impl DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses { }

impl terrars::ToListMappable for DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {
    type O = terrars::BlockAssignable<DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {}

impl BuildDataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {
    pub fn build(self) -> DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {
        DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses {}
    }
}

#[allow(dead_code)]
pub struct DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef {
        DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(
        &self,
    ) -> terrars::ListRef<DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_Clauses_ClausesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses {}

impl DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses { }

impl terrars::ToListMappable for DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses {
    type O = terrars::BlockAssignable<DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses {}

impl BuildDataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses {
    pub fn build(self) -> DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses {
        DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses {}
    }
}

#[allow(dead_code)]
pub struct DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_ClausesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_ClausesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_ClausesRef {
        DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_ClausesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_ClausesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(
        &self,
    ) -> terrars::ListRef<DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_Clauses_ClausesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses {}

impl DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses { }

impl terrars::ToListMappable for DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses {
    type O = terrars::BlockAssignable<DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses {}

impl BuildDataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses {
    pub fn build(self) -> DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses {
        DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses {}
    }
}

#[allow(dead_code)]
pub struct DataFlagshipFlags_Result_Rules_Conditions_Clauses_ClausesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFlagshipFlags_Result_Rules_Conditions_Clauses_ClausesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFlagshipFlags_Result_Rules_Conditions_Clauses_ClausesRef {
        DataFlagshipFlags_Result_Rules_Conditions_Clauses_ClausesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFlagshipFlags_Result_Rules_Conditions_Clauses_ClausesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(&self) -> terrars::ListRef<DataFlagshipFlags_Result_Rules_Conditions_Clauses_Clauses_ClausesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFlagshipFlags_Result_Rules_Conditions_Clauses {}

impl DataFlagshipFlags_Result_Rules_Conditions_Clauses { }

impl terrars::ToListMappable for DataFlagshipFlags_Result_Rules_Conditions_Clauses {
    type O = terrars::BlockAssignable<DataFlagshipFlags_Result_Rules_Conditions_Clauses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFlagshipFlags_Result_Rules_Conditions_Clauses {}

impl BuildDataFlagshipFlags_Result_Rules_Conditions_Clauses {
    pub fn build(self) -> DataFlagshipFlags_Result_Rules_Conditions_Clauses {
        DataFlagshipFlags_Result_Rules_Conditions_Clauses {}
    }
}

#[allow(dead_code)]
pub struct DataFlagshipFlags_Result_Rules_Conditions_ClausesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFlagshipFlags_Result_Rules_Conditions_ClausesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFlagshipFlags_Result_Rules_Conditions_ClausesRef {
        DataFlagshipFlags_Result_Rules_Conditions_ClausesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFlagshipFlags_Result_Rules_Conditions_ClausesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(&self) -> terrars::ListRef<DataFlagshipFlags_Result_Rules_Conditions_Clauses_ClausesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFlagshipFlags_Result_Rules_Conditions {}

impl DataFlagshipFlags_Result_Rules_Conditions { }

impl terrars::ToListMappable for DataFlagshipFlags_Result_Rules_Conditions {
    type O = terrars::BlockAssignable<DataFlagshipFlags_Result_Rules_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFlagshipFlags_Result_Rules_Conditions {}

impl BuildDataFlagshipFlags_Result_Rules_Conditions {
    pub fn build(self) -> DataFlagshipFlags_Result_Rules_Conditions {
        DataFlagshipFlags_Result_Rules_Conditions {}
    }
}

#[allow(dead_code)]
pub struct DataFlagshipFlags_Result_Rules_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFlagshipFlags_Result_Rules_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFlagshipFlags_Result_Rules_ConditionsRef {
        DataFlagshipFlags_Result_Rules_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFlagshipFlags_Result_Rules_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `clauses` after provisioning.\n"]
    pub fn clauses(&self) -> terrars::ListRef<DataFlagshipFlags_Result_Rules_Conditions_ClausesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clauses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nAvailable values: \"AND\", \"OR\"."]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nAvailable values: \"equals\", \"not_equals\", \"greater_than\", \"less_than\", \"greater_than_or_equals\", \"less_than_or_equals\", \"contains\", \"starts_with\", \"ends_with\", \"in\", \"not_in\", \"has\", \"not_has\"."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to compare against the context attribute. Must be an array for `in` and `not_in`; numeric and ISO-8601 datetime strings are accepted by the ordering operators."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFlagshipFlags_Result_Rules_Rollout {}

impl DataFlagshipFlags_Result_Rules_Rollout { }

impl terrars::ToListMappable for DataFlagshipFlags_Result_Rules_Rollout {
    type O = terrars::BlockAssignable<DataFlagshipFlags_Result_Rules_Rollout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFlagshipFlags_Result_Rules_Rollout {}

impl BuildDataFlagshipFlags_Result_Rules_Rollout {
    pub fn build(self) -> DataFlagshipFlags_Result_Rules_Rollout {
        DataFlagshipFlags_Result_Rules_Rollout {}
    }
}

#[allow(dead_code)]
pub struct DataFlagshipFlags_Result_Rules_RolloutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFlagshipFlags_Result_Rules_RolloutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFlagshipFlags_Result_Rules_RolloutRef {
        DataFlagshipFlags_Result_Rules_RolloutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFlagshipFlags_Result_Rules_RolloutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute` after provisioning.\nContext attribute used for sticky bucketing. Defaults to `targetingKey`. If absent at evaluation time, bucketing is random per request."]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nPercentage of matching traffic (0–100, up to 2 decimal places) served this variation. For multi-way splits, use cumulative upper bounds across rules (e.g. 30, 70, 100)."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFlagshipFlags_Result_Rules {}

impl DataFlagshipFlags_Result_Rules { }

impl terrars::ToListMappable for DataFlagshipFlags_Result_Rules {
    type O = terrars::BlockAssignable<DataFlagshipFlags_Result_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFlagshipFlags_Result_Rules {}

impl BuildDataFlagshipFlags_Result_Rules {
    pub fn build(self) -> DataFlagshipFlags_Result_Rules {
        DataFlagshipFlags_Result_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataFlagshipFlags_Result_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFlagshipFlags_Result_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFlagshipFlags_Result_RulesRef {
        DataFlagshipFlags_Result_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFlagshipFlags_Result_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `conditions` after provisioning.\nConditions the context must satisfy for this rule to match. An empty array matches all contexts."]
    pub fn conditions(&self) -> terrars::ListRef<DataFlagshipFlags_Result_Rules_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nEvaluation order; lower numbers are evaluated first. Must be unique across the flag's rules."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> DataFlagshipFlags_Result_Rules_RolloutRef {
        DataFlagshipFlags_Result_Rules_RolloutRef::new(self.shared().clone(), format!("{}.rollout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serve_variation` after provisioning.\nVariation served when this rule matches. Must be a key in `variations`."]
    pub fn serve_variation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serve_variation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFlagshipFlags_Result {}

impl DataFlagshipFlags_Result { }

impl terrars::ToListMappable for DataFlagshipFlags_Result {
    type O = terrars::BlockAssignable<DataFlagshipFlags_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFlagshipFlags_Result {}

impl BuildDataFlagshipFlags_Result {
    pub fn build(self) -> DataFlagshipFlags_Result {
        DataFlagshipFlags_Result {}
    }
}

#[allow(dead_code)]
pub struct DataFlagshipFlags_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFlagshipFlags_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFlagshipFlags_ResultRef {
        DataFlagshipFlags_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFlagshipFlags_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_variation` after provisioning.\nVariation served when no rule matches or the flag is disabled. Must be a key in `variations`."]
    pub fn default_variation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_variation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional operator-facing description. It does not affect flag evaluation."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhen false, the flag bypasses all rules and always serves `default_variation`."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the flag within an app. Used in all evaluation and SDK calls."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nUnique identifier for the flag within an app. Used in all evaluation and SDK calls."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nTargeting rules evaluated in ascending `priority`; the first matching rule wins. An empty array means the flag always serves `default_variation`."]
    pub fn rules(&self) -> terrars::ListRef<DataFlagshipFlags_Result_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nServer-inferred value type shared by all of the flag's variations.\nAvailable values: \"boolean\", \"string\", \"number\", \"json\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_by` after provisioning.\n"]
    pub fn updated_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `variations` after provisioning.\nMap of variation name to value. All values share the same type (boolean, string, number, or JSON object/array), and each serialized value stays within 10KB."]
    pub fn variations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.variations", self.base))
    }
}
