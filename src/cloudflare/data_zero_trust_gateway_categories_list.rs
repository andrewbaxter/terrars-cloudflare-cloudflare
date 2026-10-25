#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustGatewayCategoriesListData_ {
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
    max_items: std::option::Option<terrars::PrimField<f64>>,
}

struct DataZeroTrustGatewayCategoriesList_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustGatewayCategoriesListData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustGatewayCategoriesList(std::rc::Rc<DataZeroTrustGatewayCategoriesList_>);

impl DataZeroTrustGatewayCategoriesList {
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

    #[doc = "Set the field `account_id`.\nProvide the identifier string."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nProvide the identifier string."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustGatewayCategoriesList_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustGatewayCategoriesList {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustGatewayCategoriesList { }

impl terrars::ToListMappable for DataZeroTrustGatewayCategoriesList {
    type O = terrars::ListRef<DataZeroTrustGatewayCategoriesListRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustGatewayCategoriesList_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_gateway_categories_list".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustGatewayCategoriesList {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustGatewayCategoriesList {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustGatewayCategoriesList {
        let out = DataZeroTrustGatewayCategoriesList(std::rc::Rc::new(DataZeroTrustGatewayCategoriesList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustGatewayCategoriesListData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustGatewayCategoriesListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayCategoriesListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustGatewayCategoriesListRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nProvide the identifier string."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustGatewayCategoriesList_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewayCategoriesList_Result_Subcategories {}

impl DataZeroTrustGatewayCategoriesList_Result_Subcategories { }

impl terrars::ToListMappable for DataZeroTrustGatewayCategoriesList_Result_Subcategories {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayCategoriesList_Result_Subcategories>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayCategoriesList_Result_Subcategories {}

impl BuildDataZeroTrustGatewayCategoriesList_Result_Subcategories {
    pub fn build(self) -> DataZeroTrustGatewayCategoriesList_Result_Subcategories {
        DataZeroTrustGatewayCategoriesList_Result_Subcategories {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayCategoriesList_Result_SubcategoriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayCategoriesList_Result_SubcategoriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustGatewayCategoriesList_Result_SubcategoriesRef {
        DataZeroTrustGatewayCategoriesList_Result_SubcategoriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayCategoriesList_Result_SubcategoriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `beta` after provisioning.\nIndicate whether the category is in beta and subject to change."]
    pub fn beta(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.beta", self.base))
    }

    #[doc =
        "Get a reference to the value of field `class` after provisioning.\nSpecify which account types can create policies for this category. `blocked` Blocks unconditionally for all accounts. `removalPending` Allows removal from policies but disables addition. `noBlock` Prevents blocking.\nAvailable values: \"free\", \"premium\", \"blocked\", \"removalPending\", \"noBlock\"."]
    pub fn class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nProvide a short summary of domains in the category."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIdentify this category. Only one category per ID."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecify the category name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustGatewayCategoriesList_Result {}

impl DataZeroTrustGatewayCategoriesList_Result { }

impl terrars::ToListMappable for DataZeroTrustGatewayCategoriesList_Result {
    type O = terrars::BlockAssignable<DataZeroTrustGatewayCategoriesList_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustGatewayCategoriesList_Result {}

impl BuildDataZeroTrustGatewayCategoriesList_Result {
    pub fn build(self) -> DataZeroTrustGatewayCategoriesList_Result {
        DataZeroTrustGatewayCategoriesList_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustGatewayCategoriesList_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustGatewayCategoriesList_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustGatewayCategoriesList_ResultRef {
        DataZeroTrustGatewayCategoriesList_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustGatewayCategoriesList_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `beta` after provisioning.\nIndicate whether the category is in beta and subject to change."]
    pub fn beta(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.beta", self.base))
    }

    #[doc =
        "Get a reference to the value of field `class` after provisioning.\nSpecify which account types can create policies for this category. `blocked` Blocks unconditionally for all accounts. `removalPending` Allows removal from policies but disables addition. `noBlock` Prevents blocking.\nAvailable values: \"free\", \"premium\", \"blocked\", \"removalPending\", \"noBlock\"."]
    pub fn class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nProvide a short summary of domains in the category."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIdentify this category. Only one category per ID."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecify the category name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subcategories` after provisioning.\nProvide all subcategories for this category."]
    pub fn subcategories(&self) -> terrars::ListRef<DataZeroTrustGatewayCategoriesList_Result_SubcategoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subcategories", self.base))
    }
}
