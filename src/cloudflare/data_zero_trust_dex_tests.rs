#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDexTestsData_ {
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
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustDexTests_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDexTestsData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDexTests(std::rc::Rc<DataZeroTrustDexTests_>);

impl DataZeroTrustDexTests {
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

    #[doc = "Set the field `kind`.\nFilter by test type.\nAvailable values: \"http\", \"traceroute\"."]
    pub fn set_kind(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kind = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_name`.\nFilter by test name."]
    pub fn set_test_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().test_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nUnique identifier linked to an account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nFilter by test type.\nAvailable values: \"http\", \"traceroute\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustDexTests_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test_name` after provisioning.\nFilter by test name."]
    pub fn test_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustDexTests {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDexTests { }

impl terrars::ToListMappable for DataZeroTrustDexTests {
    type O = terrars::ListRef<DataZeroTrustDexTestsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDexTests_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dex_tests".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDexTests {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustDexTests {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDexTests {
        let out = DataZeroTrustDexTests(std::rc::Rc::new(DataZeroTrustDexTests_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDexTestsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                kind: core::default::Default::default(),
                max_items: core::default::Default::default(),
                test_name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDexTestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexTestsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDexTestsRef {
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

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nFilter by test type.\nAvailable values: \"http\", \"traceroute\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustDexTests_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test_name` after provisioning.\nFilter by test name."]
    pub fn test_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDexTests_Result_Data {}

impl DataZeroTrustDexTests_Result_Data { }

impl terrars::ToListMappable for DataZeroTrustDexTests_Result_Data {
    type O = terrars::BlockAssignable<DataZeroTrustDexTests_Result_Data>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDexTests_Result_Data {}

impl BuildDataZeroTrustDexTests_Result_Data {
    pub fn build(self) -> DataZeroTrustDexTests_Result_Data {
        DataZeroTrustDexTests_Result_Data {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDexTests_Result_DataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexTests_Result_DataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDexTests_Result_DataRef {
        DataZeroTrustDexTests_Result_DataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDexTests_Result_DataRef {
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
pub struct DataZeroTrustDexTests_Result_TargetPolicies {}

impl DataZeroTrustDexTests_Result_TargetPolicies { }

impl terrars::ToListMappable for DataZeroTrustDexTests_Result_TargetPolicies {
    type O = terrars::BlockAssignable<DataZeroTrustDexTests_Result_TargetPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDexTests_Result_TargetPolicies {}

impl BuildDataZeroTrustDexTests_Result_TargetPolicies {
    pub fn build(self) -> DataZeroTrustDexTests_Result_TargetPolicies {
        DataZeroTrustDexTests_Result_TargetPolicies {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDexTests_Result_TargetPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexTests_Result_TargetPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustDexTests_Result_TargetPoliciesRef {
        DataZeroTrustDexTests_Result_TargetPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDexTests_Result_TargetPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nWhether the DEX rule is the account default."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id of the DEX rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the DEX rule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDexTests_Result {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_policies: std::option::Option<std::vec::Vec<DataZeroTrustDexTests_Result_TargetPolicies>>,
}

impl DataZeroTrustDexTests_Result {
    #[doc = "Set the field `target_policies`.\nDEX rules targeted by this test"]
    pub fn set_target_policies(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<DataZeroTrustDexTests_Result_TargetPolicies>>,
    ) -> Self {
        self.target_policies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustDexTests_Result {
    type O = terrars::BlockAssignable<DataZeroTrustDexTests_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDexTests_Result {}

impl BuildDataZeroTrustDexTests_Result {
    pub fn build(self) -> DataZeroTrustDexTests_Result {
        DataZeroTrustDexTests_Result { target_policies: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDexTests_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexTests_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDexTests_ResultRef {
        DataZeroTrustDexTests_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDexTests_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nDate the test was created, in RFC 3339 format."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data` after provisioning.\nThe configuration object which contains the details for the WARP client to conduct the test."]
    pub fn data(&self) -> DataZeroTrustDexTests_Result_DataRef {
        DataZeroTrustDexTests_Result_DataRef::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAdditional details about the test."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nDetermines whether or not the test is active."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the test."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\nHow often the test will run."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the DEX test. Must be unique."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_policies` after provisioning.\nDEX rules targeted by this test"]
    pub fn target_policies(&self) -> terrars::ListRef<DataZeroTrustDexTests_Result_TargetPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `targeted` after provisioning.\n"]
    pub fn targeted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.targeted", self.base))
    }

    #[doc = "Get a reference to the value of field `test_id` after provisioning.\nThe unique identifier for the test."]
    pub fn test_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nDate the test was last updated, in RFC 3339 format."]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.base))
    }
}
