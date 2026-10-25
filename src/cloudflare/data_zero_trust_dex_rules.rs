#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDexRulesData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_order: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustDexRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDexRulesData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDexRules(std::rc::Rc<DataZeroTrustDexRules_>);

impl DataZeroTrustDexRules {
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

    #[doc = "Set the field `account_id`.\nUnique identifier linked to an account."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nFilter results by rule name."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sort_by`.\nWhich property to sort results by.\nAvailable values: \"name\", \"created_at\", \"updated_at\"."]
    pub fn set_sort_by(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sort_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_order`.\nSort direction for sort_by property.\nAvailable values: \"ASC\", \"DESC\"."]
    pub fn set_sort_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sort_order = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nUnique identifier linked to an account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nFilter results by rule name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustDexRules_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sort_by` after provisioning.\nWhich property to sort results by.\nAvailable values: \"name\", \"created_at\", \"updated_at\"."]
    pub fn sort_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sort_order` after provisioning.\nSort direction for sort_by property.\nAvailable values: \"ASC\", \"DESC\"."]
    pub fn sort_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustDexRules {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDexRules { }

impl terrars::ToListMappable for DataZeroTrustDexRules {
    type O = terrars::ListRef<DataZeroTrustDexRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDexRules_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dex_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDexRules {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustDexRules {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDexRules {
        let out = DataZeroTrustDexRules(std::rc::Rc::new(DataZeroTrustDexRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDexRulesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                name: core::default::Default::default(),
                sort_by: core::default::Default::default(),
                sort_order: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDexRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDexRulesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nUnique identifier linked to an account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nFilter results by rule name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustDexRules_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sort_by` after provisioning.\nWhich property to sort results by.\nAvailable values: \"name\", \"created_at\", \"updated_at\"."]
    pub fn sort_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sort_order` after provisioning.\nSort direction for sort_by property.\nAvailable values: \"ASC\", \"DESC\"."]
    pub fn sort_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDexRules_Result_Rules_TargetedTests_Data {}

impl DataZeroTrustDexRules_Result_Rules_TargetedTests_Data { }

impl terrars::ToListMappable for DataZeroTrustDexRules_Result_Rules_TargetedTests_Data {
    type O = terrars::BlockAssignable<DataZeroTrustDexRules_Result_Rules_TargetedTests_Data>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDexRules_Result_Rules_TargetedTests_Data {}

impl BuildDataZeroTrustDexRules_Result_Rules_TargetedTests_Data {
    pub fn build(self) -> DataZeroTrustDexRules_Result_Rules_TargetedTests_Data {
        DataZeroTrustDexRules_Result_Rules_TargetedTests_Data {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDexRules_Result_Rules_TargetedTests_DataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexRules_Result_Rules_TargetedTests_DataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDexRules_Result_Rules_TargetedTests_DataRef {
        DataZeroTrustDexRules_Result_Rules_TargetedTests_DataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDexRules_Result_Rules_TargetedTests_DataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe desired endpoint to test."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe type of test.\nAvailable values: \"http\", \"traceroute\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nThe HTTP request method type.\nAvailable values: \"GET\"."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDexRules_Result_Rules_TargetedTests {}

impl DataZeroTrustDexRules_Result_Rules_TargetedTests { }

impl terrars::ToListMappable for DataZeroTrustDexRules_Result_Rules_TargetedTests {
    type O = terrars::BlockAssignable<DataZeroTrustDexRules_Result_Rules_TargetedTests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDexRules_Result_Rules_TargetedTests {}

impl BuildDataZeroTrustDexRules_Result_Rules_TargetedTests {
    pub fn build(self) -> DataZeroTrustDexRules_Result_Rules_TargetedTests {
        DataZeroTrustDexRules_Result_Rules_TargetedTests {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDexRules_Result_Rules_TargetedTestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexRules_Result_Rules_TargetedTestsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDexRules_Result_Rules_TargetedTestsRef {
        DataZeroTrustDexRules_Result_Rules_TargetedTestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDexRules_Result_Rules_TargetedTestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data` after provisioning.\nThe configuration object which contains the details for the WARP client to conduct the test."]
    pub fn data(&self) -> DataZeroTrustDexRules_Result_Rules_TargetedTests_DataRef {
        DataZeroTrustDexRules_Result_Rules_TargetedTests_DataRef::new(
            self.shared().clone(),
            format!("{}.data", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `test_id` after provisioning.\n"]
    pub fn test_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDexRules_Result_Rules {}

impl DataZeroTrustDexRules_Result_Rules { }

impl terrars::ToListMappable for DataZeroTrustDexRules_Result_Rules {
    type O = terrars::BlockAssignable<DataZeroTrustDexRules_Result_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDexRules_Result_Rules {}

impl BuildDataZeroTrustDexRules_Result_Rules {
    pub fn build(self) -> DataZeroTrustDexRules_Result_Rules {
        DataZeroTrustDexRules_Result_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDexRules_Result_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexRules_Result_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDexRules_Result_RulesRef {
        DataZeroTrustDexRules_Result_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDexRules_Result_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAPI Resource UUID tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `targeted_tests` after provisioning.\n"]
    pub fn targeted_tests(&self) -> terrars::ListRef<DataZeroTrustDexRules_Result_Rules_TargetedTestsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targeted_tests", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDexRules_Result {}

impl DataZeroTrustDexRules_Result { }

impl terrars::ToListMappable for DataZeroTrustDexRules_Result {
    type O = terrars::BlockAssignable<DataZeroTrustDexRules_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDexRules_Result {}

impl BuildDataZeroTrustDexRules_Result {
    pub fn build(self) -> DataZeroTrustDexRules_Result {
        DataZeroTrustDexRules_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDexRules_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexRules_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDexRules_ResultRef {
        DataZeroTrustDexRules_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDexRules_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataZeroTrustDexRules_Result_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}
