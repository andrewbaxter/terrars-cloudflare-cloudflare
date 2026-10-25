#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAiGatewayDynamicRoutingData_ {
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
    gateway_id: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
}

struct DataAiGatewayDynamicRouting_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAiGatewayDynamicRoutingData_>,
}

#[derive(Clone)]
pub struct DataAiGatewayDynamicRouting(std::rc::Rc<DataAiGatewayDynamicRouting_>);

impl DataAiGatewayDynamicRouting {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> DataAiGatewayDynamicRouting_DeploymentRef {
        DataAiGatewayDynamicRouting_DeploymentRef::new(
            self.shared().clone(),
            format!("{}.deployment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `elements` after provisioning.\n"]
    pub fn elements(&self) -> terrars::ListRef<DataAiGatewayDynamicRouting_ElementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> DataAiGatewayDynamicRouting_VersionRef {
        DataAiGatewayDynamicRouting_VersionRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for DataAiGatewayDynamicRouting {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAiGatewayDynamicRouting { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting {
    type O = terrars::ListRef<DataAiGatewayDynamicRoutingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAiGatewayDynamicRouting_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_ai_gateway_dynamic_routing".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAiGatewayDynamicRouting {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub gateway_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataAiGatewayDynamicRouting {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAiGatewayDynamicRouting {
        let out = DataAiGatewayDynamicRouting(std::rc::Rc::new(DataAiGatewayDynamicRouting_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAiGatewayDynamicRoutingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                gateway_id: self.gateway_id,
                id: self.id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAiGatewayDynamicRoutingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRoutingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAiGatewayDynamicRoutingRef {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> DataAiGatewayDynamicRouting_DeploymentRef {
        DataAiGatewayDynamicRouting_DeploymentRef::new(
            self.shared().clone(),
            format!("{}.deployment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `elements` after provisioning.\n"]
    pub fn elements(&self) -> terrars::ListRef<DataAiGatewayDynamicRouting_ElementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> DataAiGatewayDynamicRouting_VersionRef {
        DataAiGatewayDynamicRouting_VersionRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAiGatewayDynamicRouting_Deployment {}

impl DataAiGatewayDynamicRouting_Deployment { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting_Deployment {
    type O = terrars::BlockAssignable<DataAiGatewayDynamicRouting_Deployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGatewayDynamicRouting_Deployment {}

impl BuildDataAiGatewayDynamicRouting_Deployment {
    pub fn build(self) -> DataAiGatewayDynamicRouting_Deployment {
        DataAiGatewayDynamicRouting_Deployment {}
    }
}

#[allow(dead_code)]
pub struct DataAiGatewayDynamicRouting_DeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRouting_DeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGatewayDynamicRouting_DeploymentRef {
        DataAiGatewayDynamicRouting_DeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGatewayDynamicRouting_DeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\n"]
    pub fn deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs_Fallback {}

impl DataAiGatewayDynamicRouting_Elements_Outputs_Fallback { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting_Elements_Outputs_Fallback {
    type O = terrars::BlockAssignable<DataAiGatewayDynamicRouting_Elements_Outputs_Fallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGatewayDynamicRouting_Elements_Outputs_Fallback {}

impl BuildDataAiGatewayDynamicRouting_Elements_Outputs_Fallback {
    pub fn build(self) -> DataAiGatewayDynamicRouting_Elements_Outputs_Fallback {
        DataAiGatewayDynamicRouting_Elements_Outputs_Fallback {}
    }
}

#[allow(dead_code)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
        DataAiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `element_id` after provisioning.\n"]
    pub fn element_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.element_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs_False {}

impl DataAiGatewayDynamicRouting_Elements_Outputs_False { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting_Elements_Outputs_False {
    type O = terrars::BlockAssignable<DataAiGatewayDynamicRouting_Elements_Outputs_False>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGatewayDynamicRouting_Elements_Outputs_False {}

impl BuildDataAiGatewayDynamicRouting_Elements_Outputs_False {
    pub fn build(self) -> DataAiGatewayDynamicRouting_Elements_Outputs_False {
        DataAiGatewayDynamicRouting_Elements_Outputs_False {}
    }
}

#[allow(dead_code)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs_FalseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRouting_Elements_Outputs_FalseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGatewayDynamicRouting_Elements_Outputs_FalseRef {
        DataAiGatewayDynamicRouting_Elements_Outputs_FalseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGatewayDynamicRouting_Elements_Outputs_FalseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `element_id` after provisioning.\n"]
    pub fn element_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.element_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs_Next {}

impl DataAiGatewayDynamicRouting_Elements_Outputs_Next { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting_Elements_Outputs_Next {
    type O = terrars::BlockAssignable<DataAiGatewayDynamicRouting_Elements_Outputs_Next>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGatewayDynamicRouting_Elements_Outputs_Next {}

impl BuildDataAiGatewayDynamicRouting_Elements_Outputs_Next {
    pub fn build(self) -> DataAiGatewayDynamicRouting_Elements_Outputs_Next {
        DataAiGatewayDynamicRouting_Elements_Outputs_Next {}
    }
}

#[allow(dead_code)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs_NextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRouting_Elements_Outputs_NextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGatewayDynamicRouting_Elements_Outputs_NextRef {
        DataAiGatewayDynamicRouting_Elements_Outputs_NextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGatewayDynamicRouting_Elements_Outputs_NextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `element_id` after provisioning.\n"]
    pub fn element_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.element_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs_Success {}

impl DataAiGatewayDynamicRouting_Elements_Outputs_Success { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting_Elements_Outputs_Success {
    type O = terrars::BlockAssignable<DataAiGatewayDynamicRouting_Elements_Outputs_Success>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGatewayDynamicRouting_Elements_Outputs_Success {}

impl BuildDataAiGatewayDynamicRouting_Elements_Outputs_Success {
    pub fn build(self) -> DataAiGatewayDynamicRouting_Elements_Outputs_Success {
        DataAiGatewayDynamicRouting_Elements_Outputs_Success {}
    }
}

#[allow(dead_code)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
        DataAiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `element_id` after provisioning.\n"]
    pub fn element_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.element_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs_True {}

impl DataAiGatewayDynamicRouting_Elements_Outputs_True { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting_Elements_Outputs_True {
    type O = terrars::BlockAssignable<DataAiGatewayDynamicRouting_Elements_Outputs_True>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGatewayDynamicRouting_Elements_Outputs_True {}

impl BuildDataAiGatewayDynamicRouting_Elements_Outputs_True {
    pub fn build(self) -> DataAiGatewayDynamicRouting_Elements_Outputs_True {
        DataAiGatewayDynamicRouting_Elements_Outputs_True {}
    }
}

#[allow(dead_code)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs_TrueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRouting_Elements_Outputs_TrueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGatewayDynamicRouting_Elements_Outputs_TrueRef {
        DataAiGatewayDynamicRouting_Elements_Outputs_TrueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGatewayDynamicRouting_Elements_Outputs_TrueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `element_id` after provisioning.\n"]
    pub fn element_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.element_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGatewayDynamicRouting_Elements_Outputs {}

impl DataAiGatewayDynamicRouting_Elements_Outputs { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting_Elements_Outputs {
    type O = terrars::BlockAssignable<DataAiGatewayDynamicRouting_Elements_Outputs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGatewayDynamicRouting_Elements_Outputs {}

impl BuildDataAiGatewayDynamicRouting_Elements_Outputs {
    pub fn build(self) -> DataAiGatewayDynamicRouting_Elements_Outputs {
        DataAiGatewayDynamicRouting_Elements_Outputs {}
    }
}

#[allow(dead_code)]
pub struct DataAiGatewayDynamicRouting_Elements_OutputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRouting_Elements_OutputsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGatewayDynamicRouting_Elements_OutputsRef {
        DataAiGatewayDynamicRouting_Elements_OutputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGatewayDynamicRouting_Elements_OutputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `element_id` after provisioning.\n"]
    pub fn element_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.element_id", self.base))
    }

    #[doc = "Get a reference to the value of field `fallback` after provisioning.\n"]
    pub fn fallback(&self) -> DataAiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
        DataAiGatewayDynamicRouting_Elements_Outputs_FallbackRef::new(
            self.shared().clone(),
            format!("{}.fallback", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `false_` after provisioning.\n"]
    pub fn false_(&self) -> DataAiGatewayDynamicRouting_Elements_Outputs_FalseRef {
        DataAiGatewayDynamicRouting_Elements_Outputs_FalseRef::new(
            self.shared().clone(),
            format!("{}.false", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `next` after provisioning.\n"]
    pub fn next(&self) -> DataAiGatewayDynamicRouting_Elements_Outputs_NextRef {
        DataAiGatewayDynamicRouting_Elements_Outputs_NextRef::new(
            self.shared().clone(),
            format!("{}.next", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `success` after provisioning.\n"]
    pub fn success(&self) -> DataAiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
        DataAiGatewayDynamicRouting_Elements_Outputs_SuccessRef::new(
            self.shared().clone(),
            format!("{}.success", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `true_` after provisioning.\n"]
    pub fn true_(&self) -> DataAiGatewayDynamicRouting_Elements_Outputs_TrueRef {
        DataAiGatewayDynamicRouting_Elements_Outputs_TrueRef::new(
            self.shared().clone(),
            format!("{}.true", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataAiGatewayDynamicRouting_Elements_Properties {}

impl DataAiGatewayDynamicRouting_Elements_Properties { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting_Elements_Properties {
    type O = terrars::BlockAssignable<DataAiGatewayDynamicRouting_Elements_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGatewayDynamicRouting_Elements_Properties {}

impl BuildDataAiGatewayDynamicRouting_Elements_Properties {
    pub fn build(self) -> DataAiGatewayDynamicRouting_Elements_Properties {
        DataAiGatewayDynamicRouting_Elements_Properties {}
    }
}

#[allow(dead_code)]
pub struct DataAiGatewayDynamicRouting_Elements_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRouting_Elements_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAiGatewayDynamicRouting_Elements_PropertiesRef {
        DataAiGatewayDynamicRouting_Elements_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGatewayDynamicRouting_Elements_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ai_gateway_dynamic_routing_provider` after provisioning.\n"]
    pub fn ai_gateway_dynamic_routing_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ai_gateway_dynamic_routing_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `limit` after provisioning.\n"]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `limit_type` after provisioning.\nAvailable values: \"count\", \"cost\"."]
    pub fn limit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit_type", self.base))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc = "Get a reference to the value of field `retries` after provisioning.\n"]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGatewayDynamicRouting_Elements {}

impl DataAiGatewayDynamicRouting_Elements { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting_Elements {
    type O = terrars::BlockAssignable<DataAiGatewayDynamicRouting_Elements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGatewayDynamicRouting_Elements {}

impl BuildDataAiGatewayDynamicRouting_Elements {
    pub fn build(self) -> DataAiGatewayDynamicRouting_Elements {
        DataAiGatewayDynamicRouting_Elements {}
    }
}

#[allow(dead_code)]
pub struct DataAiGatewayDynamicRouting_ElementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRouting_ElementsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGatewayDynamicRouting_ElementsRef {
        DataAiGatewayDynamicRouting_ElementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGatewayDynamicRouting_ElementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> DataAiGatewayDynamicRouting_Elements_OutputsRef {
        DataAiGatewayDynamicRouting_Elements_OutputsRef::new(self.shared().clone(), format!("{}.outputs", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> DataAiGatewayDynamicRouting_Elements_PropertiesRef {
        DataAiGatewayDynamicRouting_Elements_PropertiesRef::new(
            self.shared().clone(),
            format!("{}.properties", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"start\", \"conditional\", \"percentage\", \"rate\", \"model\", \"end\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAiGatewayDynamicRouting_Version {}

impl DataAiGatewayDynamicRouting_Version { }

impl terrars::ToListMappable for DataAiGatewayDynamicRouting_Version {
    type O = terrars::BlockAssignable<DataAiGatewayDynamicRouting_Version>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAiGatewayDynamicRouting_Version {}

impl BuildDataAiGatewayDynamicRouting_Version {
    pub fn build(self) -> DataAiGatewayDynamicRouting_Version {
        DataAiGatewayDynamicRouting_Version {}
    }
}

#[allow(dead_code)]
pub struct DataAiGatewayDynamicRouting_VersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAiGatewayDynamicRouting_VersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAiGatewayDynamicRouting_VersionRef {
        DataAiGatewayDynamicRouting_VersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAiGatewayDynamicRouting_VersionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `active` after provisioning.\nAvailable values: \"true\", \"false\"."]
    pub fn active(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `is_valid` after provisioning.\n"]
    pub fn is_valid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_valid", self.base))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
}
