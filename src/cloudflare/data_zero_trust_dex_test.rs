#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustDexTestData_ {
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
    dex_test_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataZeroTrustDexTest_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_policies: std::option::Option<std::vec::Vec<DataZeroTrustDexTest_TargetPolicies>>,
}

struct DataZeroTrustDexTest_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustDexTestData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustDexTest(std::rc::Rc<DataZeroTrustDexTest_>);

impl DataZeroTrustDexTest {
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

    #[doc = "Set the field `dex_test_id`.\nThe unique identifier for the test."]
    pub fn set_dex_test_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dex_test_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataZeroTrustDexTest_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_policies`.\nDEX rules targeted by this test"]
    pub fn set_target_policies(
        self,
        v: impl std::convert::Into<std::vec::Vec<DataZeroTrustDexTest_TargetPolicies>>,
    ) -> Self {
        self.0.data.borrow_mut().target_policies = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nUnique identifier linked to an account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nDate the test was created, in RFC 3339 format."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data` after provisioning.\nThe configuration object which contains the details for the WARP client to conduct the test."]
    pub fn data(&self) -> DataZeroTrustDexTest_DataRef {
        DataZeroTrustDexTest_DataRef::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAdditional details about the test."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dex_test_id` after provisioning.\nThe unique identifier for the test."]
    pub fn dex_test_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dex_test_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nDetermines whether or not the test is active."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustDexTest_FilterRef {
        DataZeroTrustDexTest_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the test."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\nHow often the test will run."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the DEX test. Must be unique."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_policies` after provisioning.\nDEX rules targeted by this test"]
    pub fn target_policies(&self) -> terrars::ListRef<DataZeroTrustDexTest_TargetPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targeted` after provisioning.\n"]
    pub fn targeted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.targeted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test_id` after provisioning.\nThe unique identifier for the test."]
    pub fn test_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nDate the test was last updated, in RFC 3339 format."]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustDexTest {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustDexTest { }

impl terrars::ToListMappable for DataZeroTrustDexTest {
    type O = terrars::ListRef<DataZeroTrustDexTestRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustDexTest_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dex_test".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustDexTest {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustDexTest {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustDexTest {
        let out = DataZeroTrustDexTest(std::rc::Rc::new(DataZeroTrustDexTest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustDexTestData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                dex_test_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                target_policies: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustDexTestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexTestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustDexTestRef {
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
        "Get a reference to the value of field `created` after provisioning.\nDate the test was created, in RFC 3339 format."]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data` after provisioning.\nThe configuration object which contains the details for the WARP client to conduct the test."]
    pub fn data(&self) -> DataZeroTrustDexTest_DataRef {
        DataZeroTrustDexTest_DataRef::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAdditional details about the test."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dex_test_id` after provisioning.\nThe unique identifier for the test."]
    pub fn dex_test_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dex_test_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nDetermines whether or not the test is active."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataZeroTrustDexTest_FilterRef {
        DataZeroTrustDexTest_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the test."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\nHow often the test will run."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the DEX test. Must be unique."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_policies` after provisioning.\nDEX rules targeted by this test"]
    pub fn target_policies(&self) -> terrars::ListRef<DataZeroTrustDexTest_TargetPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targeted` after provisioning.\n"]
    pub fn targeted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.targeted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test_id` after provisioning.\nThe unique identifier for the test."]
    pub fn test_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nDate the test was last updated, in RFC 3339 format."]
    pub fn updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDexTest_Data {}

impl DataZeroTrustDexTest_Data { }

impl terrars::ToListMappable for DataZeroTrustDexTest_Data {
    type O = terrars::BlockAssignable<DataZeroTrustDexTest_Data>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDexTest_Data {}

impl BuildDataZeroTrustDexTest_Data {
    pub fn build(self) -> DataZeroTrustDexTest_Data {
        DataZeroTrustDexTest_Data {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDexTest_DataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexTest_DataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDexTest_DataRef {
        DataZeroTrustDexTest_DataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDexTest_DataRef {
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
pub struct DataZeroTrustDexTest_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataZeroTrustDexTest_Filter {
    #[doc = "Set the field `kind`.\nFilter by test type.\nAvailable values: \"http\", \"traceroute\"."]
    pub fn set_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kind = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_name`.\nFilter by test name."]
    pub fn set_test_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.test_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataZeroTrustDexTest_Filter {
    type O = terrars::BlockAssignable<DataZeroTrustDexTest_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDexTest_Filter {}

impl BuildDataZeroTrustDexTest_Filter {
    pub fn build(self) -> DataZeroTrustDexTest_Filter {
        DataZeroTrustDexTest_Filter {
            kind: core::default::Default::default(),
            test_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDexTest_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexTest_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDexTest_FilterRef {
        DataZeroTrustDexTest_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDexTest_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nFilter by test type.\nAvailable values: \"http\", \"traceroute\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc = "Get a reference to the value of field `test_name` after provisioning.\nFilter by test name."]
    pub fn test_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustDexTest_TargetPolicies {}

impl DataZeroTrustDexTest_TargetPolicies { }

impl terrars::ToListMappable for DataZeroTrustDexTest_TargetPolicies {
    type O = terrars::BlockAssignable<DataZeroTrustDexTest_TargetPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustDexTest_TargetPolicies {}

impl BuildDataZeroTrustDexTest_TargetPolicies {
    pub fn build(self) -> DataZeroTrustDexTest_TargetPolicies {
        DataZeroTrustDexTest_TargetPolicies {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustDexTest_TargetPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustDexTest_TargetPoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZeroTrustDexTest_TargetPoliciesRef {
        DataZeroTrustDexTest_TargetPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustDexTest_TargetPoliciesRef {
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
