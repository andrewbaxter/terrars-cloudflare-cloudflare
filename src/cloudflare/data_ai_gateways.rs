#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAiGatewaysData_ {
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
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAiGateways_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAiGatewaysData_>,
}

#[derive(Clone)]
pub struct DataAiGateways(std::rc::Rc<DataAiGateways_>);

impl DataAiGateways {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nSearch by id"]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAiGateways_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\nSearch by id"]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

impl terrars::Referable for DataAiGateways {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAiGateways { }

impl terrars::ToListMappable for DataAiGateways {
    type O = terrars::ListRef<DataAiGatewaysRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAiGateways_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_ai_gateways".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAiGateways {
    pub tf_id: std::string::String,
}

impl BuildDataAiGateways {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAiGateways {
        let out = DataAiGateways(std::rc::Rc::new(DataAiGateways_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAiGatewaysData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                search: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAiGatewaysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewaysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAiGatewaysRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataAiGateways_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\nSearch by id"]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_Dlp_Policies {}

impl DataAiGateways_Result_Dlp_Policies { }

impl terrars::ToListMappable for DataAiGateways_Result_Dlp_Policies {
    type O = terrars::BlockAssignable<DataAiGateways_Result_Dlp_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_Dlp_Policies {}

impl BuildDataAiGateways_Result_Dlp_Policies {
    pub fn build(self) -> DataAiGateways_Result_Dlp_Policies {
        DataAiGateways_Result_Dlp_Policies {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_Dlp_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_Dlp_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_Result_Dlp_PoliciesRef {
        DataAiGateways_Result_Dlp_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_Dlp_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `check` after provisioning.\n"]
    pub fn check(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_Dlp {}

impl DataAiGateways_Result_Dlp { }

impl terrars::ToListMappable for DataAiGateways_Result_Dlp {
    type O = terrars::BlockAssignable<DataAiGateways_Result_Dlp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_Dlp {}

impl BuildDataAiGateways_Result_Dlp {
    pub fn build(self) -> DataAiGateways_Result_Dlp {
        DataAiGateways_Result_Dlp {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_DlpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_DlpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_Result_DlpRef {
        DataAiGateways_Result_DlpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_DlpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAvailable values: \"BLOCK\", \"FLAG\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> terrars::ListRef<DataAiGateways_Result_Dlp_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.base))
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_Guardrails_Prompt {}

impl DataAiGateways_Result_Guardrails_Prompt { }

impl terrars::ToListMappable for DataAiGateways_Result_Guardrails_Prompt {
    type O = terrars::BlockAssignable<DataAiGateways_Result_Guardrails_Prompt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_Guardrails_Prompt {}

impl BuildDataAiGateways_Result_Guardrails_Prompt {
    pub fn build(self) -> DataAiGateways_Result_Guardrails_Prompt {
        DataAiGateways_Result_Guardrails_Prompt {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_Guardrails_PromptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_Guardrails_PromptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_Result_Guardrails_PromptRef {
        DataAiGateways_Result_Guardrails_PromptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_Guardrails_PromptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `p1` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn p1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p1", self.base))
    }

    #[doc = "Get a reference to the value of field `s1` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s1", self.base))
    }

    #[doc = "Get a reference to the value of field `s10` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s10(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s10", self.base))
    }

    #[doc = "Get a reference to the value of field `s11` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s11(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s11", self.base))
    }

    #[doc = "Get a reference to the value of field `s12` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s12(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s12", self.base))
    }

    #[doc = "Get a reference to the value of field `s13` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s13(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s13", self.base))
    }

    #[doc = "Get a reference to the value of field `s2` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s2", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `s4` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s4", self.base))
    }

    #[doc = "Get a reference to the value of field `s5` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s5(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s5", self.base))
    }

    #[doc = "Get a reference to the value of field `s6` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s6", self.base))
    }

    #[doc = "Get a reference to the value of field `s7` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s7(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s7", self.base))
    }

    #[doc = "Get a reference to the value of field `s8` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s8(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s8", self.base))
    }

    #[doc = "Get a reference to the value of field `s9` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s9(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s9", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_Guardrails_Response {}

impl DataAiGateways_Result_Guardrails_Response { }

impl terrars::ToListMappable for DataAiGateways_Result_Guardrails_Response {
    type O = terrars::BlockAssignable<DataAiGateways_Result_Guardrails_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_Guardrails_Response {}

impl BuildDataAiGateways_Result_Guardrails_Response {
    pub fn build(self) -> DataAiGateways_Result_Guardrails_Response {
        DataAiGateways_Result_Guardrails_Response {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_Guardrails_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_Guardrails_ResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_Result_Guardrails_ResponseRef {
        DataAiGateways_Result_Guardrails_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_Guardrails_ResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `p1` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn p1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p1", self.base))
    }

    #[doc = "Get a reference to the value of field `s1` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s1", self.base))
    }

    #[doc = "Get a reference to the value of field `s10` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s10(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s10", self.base))
    }

    #[doc = "Get a reference to the value of field `s11` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s11(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s11", self.base))
    }

    #[doc = "Get a reference to the value of field `s12` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s12(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s12", self.base))
    }

    #[doc = "Get a reference to the value of field `s13` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s13(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s13", self.base))
    }

    #[doc = "Get a reference to the value of field `s2` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s2", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `s4` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s4", self.base))
    }

    #[doc = "Get a reference to the value of field `s5` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s5(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s5", self.base))
    }

    #[doc = "Get a reference to the value of field `s6` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s6", self.base))
    }

    #[doc = "Get a reference to the value of field `s7` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s7(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s7", self.base))
    }

    #[doc = "Get a reference to the value of field `s8` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s8(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s8", self.base))
    }

    #[doc = "Get a reference to the value of field `s9` after provisioning.\nAvailable values: \"FLAG\", \"BLOCK\"."]
    pub fn s9(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s9", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_Guardrails {}

impl DataAiGateways_Result_Guardrails { }

impl terrars::ToListMappable for DataAiGateways_Result_Guardrails {
    type O = terrars::BlockAssignable<DataAiGateways_Result_Guardrails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_Guardrails {}

impl BuildDataAiGateways_Result_Guardrails {
    pub fn build(self) -> DataAiGateways_Result_Guardrails {
        DataAiGateways_Result_Guardrails {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_GuardrailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_GuardrailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_Result_GuardrailsRef {
        DataAiGateways_Result_GuardrailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_GuardrailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> DataAiGateways_Result_Guardrails_PromptRef {
        DataAiGateways_Result_Guardrails_PromptRef::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> DataAiGateways_Result_Guardrails_ResponseRef {
        DataAiGateways_Result_Guardrails_ResponseRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_Otel {}

impl DataAiGateways_Result_Otel { }

impl terrars::ToListMappable for DataAiGateways_Result_Otel {
    type O = terrars::BlockAssignable<DataAiGateways_Result_Otel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_Otel {}

impl BuildDataAiGateways_Result_Otel {
    pub fn build(self) -> DataAiGateways_Result_Otel {
        DataAiGateways_Result_Otel {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_OtelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_OtelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_Result_OtelRef {
        DataAiGateways_Result_OtelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_OtelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nAvailable values: \"json\", \"protobuf\"."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_SpendLimits_Rules_AiGatewayProvider {}

impl DataAiGateways_Result_SpendLimits_Rules_AiGatewayProvider { }

impl terrars::ToListMappable for DataAiGateways_Result_SpendLimits_Rules_AiGatewayProvider {
    type O = terrars::BlockAssignable<DataAiGateways_Result_SpendLimits_Rules_AiGatewayProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_SpendLimits_Rules_AiGatewayProvider {}

impl BuildDataAiGateways_Result_SpendLimits_Rules_AiGatewayProvider {
    pub fn build(self) -> DataAiGateways_Result_SpendLimits_Rules_AiGatewayProvider {
        DataAiGateways_Result_SpendLimits_Rules_AiGatewayProvider {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_SpendLimits_Rules_AiGatewayProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_SpendLimits_Rules_AiGatewayProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGateways_Result_SpendLimits_Rules_AiGatewayProviderRef {
        DataAiGateways_Result_SpendLimits_Rules_AiGatewayProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_SpendLimits_Rules_AiGatewayProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nAvailable values: \"filter\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_SpendLimits_Rules_Metadata {}

impl DataAiGateways_Result_SpendLimits_Rules_Metadata { }

impl terrars::ToListMappable for DataAiGateways_Result_SpendLimits_Rules_Metadata {
    type O = terrars::BlockAssignable<DataAiGateways_Result_SpendLimits_Rules_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_SpendLimits_Rules_Metadata {}

impl BuildDataAiGateways_Result_SpendLimits_Rules_Metadata {
    pub fn build(self) -> DataAiGateways_Result_SpendLimits_Rules_Metadata {
        DataAiGateways_Result_SpendLimits_Rules_Metadata {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_SpendLimits_Rules_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_SpendLimits_Rules_MetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGateways_Result_SpendLimits_Rules_MetadataRef {
        DataAiGateways_Result_SpendLimits_Rules_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_SpendLimits_Rules_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nAvailable values: \"partition\", \"filter\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_SpendLimits_Rules_Model {}

impl DataAiGateways_Result_SpendLimits_Rules_Model { }

impl terrars::ToListMappable for DataAiGateways_Result_SpendLimits_Rules_Model {
    type O = terrars::BlockAssignable<DataAiGateways_Result_SpendLimits_Rules_Model>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_SpendLimits_Rules_Model {}

impl BuildDataAiGateways_Result_SpendLimits_Rules_Model {
    pub fn build(self) -> DataAiGateways_Result_SpendLimits_Rules_Model {
        DataAiGateways_Result_SpendLimits_Rules_Model {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_SpendLimits_Rules_ModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_SpendLimits_Rules_ModelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGateways_Result_SpendLimits_Rules_ModelRef {
        DataAiGateways_Result_SpendLimits_Rules_ModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_SpendLimits_Rules_ModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nAvailable values: \"filter\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_SpendLimits_Rules {}

impl DataAiGateways_Result_SpendLimits_Rules { }

impl terrars::ToListMappable for DataAiGateways_Result_SpendLimits_Rules {
    type O = terrars::BlockAssignable<DataAiGateways_Result_SpendLimits_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_SpendLimits_Rules {}

impl BuildDataAiGateways_Result_SpendLimits_Rules {
    pub fn build(self) -> DataAiGateways_Result_SpendLimits_Rules {
        DataAiGateways_Result_SpendLimits_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_SpendLimits_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_SpendLimits_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_Result_SpendLimits_RulesRef {
        DataAiGateways_Result_SpendLimits_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_SpendLimits_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ai_gateway_provider` after provisioning.\n"]
    pub fn ai_gateway_provider(&self) -> DataAiGateways_Result_SpendLimits_Rules_AiGatewayProviderRef {
        DataAiGateways_Result_SpendLimits_Rules_AiGatewayProviderRef::new(
            self.shared().clone(),
            format!("{}.ai_gateway_provider", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `limit` after provisioning.\n"]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc = "Get a reference to the value of field `limit_type` after provisioning.\nAvailable values: \"cost\"."]
    pub fn limit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit_type", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<DataAiGateways_Result_SpendLimits_Rules_MetadataRef> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> DataAiGateways_Result_SpendLimits_Rules_ModelRef {
        DataAiGateways_Result_SpendLimits_Rules_ModelRef::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technique` after provisioning.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn technique(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technique", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_SpendLimits {}

impl DataAiGateways_Result_SpendLimits { }

impl terrars::ToListMappable for DataAiGateways_Result_SpendLimits {
    type O = terrars::BlockAssignable<DataAiGateways_Result_SpendLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_SpendLimits {}

impl BuildDataAiGateways_Result_SpendLimits {
    pub fn build(self) -> DataAiGateways_Result_SpendLimits {
        DataAiGateways_Result_SpendLimits {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_SpendLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_SpendLimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_Result_SpendLimitsRef {
        DataAiGateways_Result_SpendLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_SpendLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataAiGateways_Result_SpendLimits_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_Stripe_UsageEvents {}

impl DataAiGateways_Result_Stripe_UsageEvents { }

impl terrars::ToListMappable for DataAiGateways_Result_Stripe_UsageEvents {
    type O = terrars::BlockAssignable<DataAiGateways_Result_Stripe_UsageEvents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_Stripe_UsageEvents {}

impl BuildDataAiGateways_Result_Stripe_UsageEvents {
    pub fn build(self) -> DataAiGateways_Result_Stripe_UsageEvents {
        DataAiGateways_Result_Stripe_UsageEvents {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_Stripe_UsageEventsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_Stripe_UsageEventsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_Result_Stripe_UsageEventsRef {
        DataAiGateways_Result_Stripe_UsageEventsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_Stripe_UsageEventsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result_Stripe {}

impl DataAiGateways_Result_Stripe { }

impl terrars::ToListMappable for DataAiGateways_Result_Stripe {
    type O = terrars::BlockAssignable<DataAiGateways_Result_Stripe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result_Stripe {}

impl BuildDataAiGateways_Result_Stripe {
    pub fn build(self) -> DataAiGateways_Result_Stripe {
        DataAiGateways_Result_Stripe {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_Result_StripeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_Result_StripeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_Result_StripeRef {
        DataAiGateways_Result_StripeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_Result_StripeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization", self.base))
    }

    #[doc = "Get a reference to the value of field `usage_events` after provisioning.\n"]
    pub fn usage_events(&self) -> terrars::ListRef<DataAiGateways_Result_Stripe_UsageEventsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usage_events", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGateways_Result {}

impl DataAiGateways_Result { }

impl terrars::ToListMappable for DataAiGateways_Result {
    type O = terrars::BlockAssignable<DataAiGateways_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGateways_Result {}

impl BuildDataAiGateways_Result {
    pub fn build(self) -> DataAiGateways_Result {
        DataAiGateways_Result {}
    }
}

#[allow(dead_code)]
pub struct DataAiGateways_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGateways_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGateways_ResultRef {
        DataAiGateways_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGateways_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication", self.base))
    }

    #[doc =
        "Get a reference to the value of field `byok_only` after provisioning.\nRequires customer-provided provider credentials and prevents fallback to Unified Billing."]
    pub fn byok_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.byok_only", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_invalidate_on_update` after provisioning.\n"]
    pub fn cache_invalidate_on_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_invalidate_on_update", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_ttl` after provisioning.\n"]
    pub fn cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `collect_logs` after provisioning.\n"]
    pub fn collect_logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collect_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `dlp` after provisioning.\n"]
    pub fn dlp(&self) -> DataAiGateways_Result_DlpRef {
        DataAiGateways_Result_DlpRef::new(self.shared().clone(), format!("{}.dlp", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> DataAiGateways_Result_GuardrailsRef {
        DataAiGateways_Result_GuardrailsRef::new(self.shared().clone(), format!("{}.guardrails", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier of the AI Gateway within the account."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `is_default` after provisioning.\n"]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.base))
    }

    #[doc = "Get a reference to the value of field `log_classification` after provisioning.\n"]
    pub fn log_classification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_classification", self.base))
    }

    #[doc = "Get a reference to the value of field `log_management` after provisioning.\n"]
    pub fn log_management(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_management", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_management_strategy` after provisioning.\nAvailable values: \"STOP_INSERTING\", \"DELETE_OLDEST\"."]
    pub fn log_management_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_management_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `logpush` after provisioning.\n"]
    pub fn logpush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush", self.base))
    }

    #[doc = "Get a reference to the value of field `logpush_public_key` after provisioning.\n"]
    pub fn logpush_public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logpush_public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.base))
    }

    #[doc = "Get a reference to the value of field `otel` after provisioning.\n"]
    pub fn otel(&self) -> terrars::ListRef<DataAiGateways_Result_OtelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.otel", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limiting_interval` after provisioning.\n"]
    pub fn rate_limiting_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limiting_limit` after provisioning.\n"]
    pub fn rate_limiting_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rate_limiting_technique` after provisioning.\nAvailable values: \"fixed\", \"sliding\"."]
    pub fn rate_limiting_technique(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_technique", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_backoff` after provisioning.\nBackoff strategy for retry delays\nAvailable values: \"constant\", \"linear\", \"exponential\"."]
    pub fn retry_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_backoff", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_delay` after provisioning.\nDelay between retry attempts in milliseconds (0-60000)"]
    pub fn retry_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_delay", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_max_attempts` after provisioning.\nMaximum number of retry attempts for failed requests (1-5)"]
    pub fn retry_max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_max_attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `spend_limits` after provisioning.\n"]
    pub fn spend_limits(&self) -> DataAiGateways_Result_SpendLimitsRef {
        DataAiGateways_Result_SpendLimitsRef::new(self.shared().clone(), format!("{}.spend_limits", self.base))
    }

    #[doc = "Get a reference to the value of field `store_id` after provisioning.\n"]
    pub fn store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.store_id", self.base))
    }

    #[doc = "Get a reference to the value of field `stripe` after provisioning.\n"]
    pub fn stripe(&self) -> DataAiGateways_Result_StripeRef {
        DataAiGateways_Result_StripeRef::new(self.shared().clone(), format!("{}.stripe", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workers_ai_billing_mode` after provisioning.\nControls how Workers AI inference calls routed through this gateway are billed. 'postpaid' bills the account directly through Workers AI; 'unified' deducts credits via AI Gateway using neuron-based pricing and delegates billing to AI Gateway.\nAvailable values: \"postpaid\", \"unified\"."]
    pub fn workers_ai_billing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workers_ai_billing_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `zdr` after provisioning.\n"]
    pub fn zdr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zdr", self.base))
    }
}
