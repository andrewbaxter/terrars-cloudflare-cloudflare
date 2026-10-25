#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AiGatewayDynamicRoutingData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    account_id: terrars::PrimField<std::string::String>,
    elements: std::vec::Vec<AiGatewayDynamicRouting_Elements>,
    gateway_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

struct AiGatewayDynamicRouting_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AiGatewayDynamicRoutingData_>,
}

#[derive(Clone)]
pub struct AiGatewayDynamicRouting(std::rc::Rc<AiGatewayDynamicRouting_>);

impl AiGatewayDynamicRouting {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> AiGatewayDynamicRouting_DeploymentRef {
        AiGatewayDynamicRouting_DeploymentRef::new(
            self.shared().clone(),
            format!("{}.deployment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `elements` after provisioning.\n"]
    pub fn elements(&self) -> terrars::ListRef<AiGatewayDynamicRouting_ElementsRef> {
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

    #[doc = "Get a reference to the value of field `route` after provisioning.\n"]
    pub fn route(&self) -> AiGatewayDynamicRouting_RouteRef {
        AiGatewayDynamicRouting_RouteRef::new(self.shared().clone(), format!("{}.route", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success` after provisioning.\n"]
    pub fn success(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> AiGatewayDynamicRouting_VersionRef {
        AiGatewayDynamicRouting_VersionRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for AiGatewayDynamicRouting {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AiGatewayDynamicRouting { }

impl terrars::ToListMappable for AiGatewayDynamicRouting {
    type O = terrars::ListRef<AiGatewayDynamicRoutingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AiGatewayDynamicRouting_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_ai_gateway_dynamic_routing".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAiGatewayDynamicRouting {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub elements: std::vec::Vec<AiGatewayDynamicRouting_Elements>,
    #[doc = ""]
    pub gateway_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAiGatewayDynamicRouting {
    pub fn build(self, stack: &mut terrars::Stack) -> AiGatewayDynamicRouting {
        let out = AiGatewayDynamicRouting(std::rc::Rc::new(AiGatewayDynamicRouting_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AiGatewayDynamicRoutingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                elements: self.elements,
                gateway_id: self.gateway_id,
                name: self.name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AiGatewayDynamicRoutingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRoutingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AiGatewayDynamicRoutingRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn deployment(&self) -> AiGatewayDynamicRouting_DeploymentRef {
        AiGatewayDynamicRouting_DeploymentRef::new(
            self.shared().clone(),
            format!("{}.deployment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `elements` after provisioning.\n"]
    pub fn elements(&self) -> terrars::ListRef<AiGatewayDynamicRouting_ElementsRef> {
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

    #[doc = "Get a reference to the value of field `route` after provisioning.\n"]
    pub fn route(&self) -> AiGatewayDynamicRouting_RouteRef {
        AiGatewayDynamicRouting_RouteRef::new(self.shared().clone(), format!("{}.route", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success` after provisioning.\n"]
    pub fn success(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> AiGatewayDynamicRouting_VersionRef {
        AiGatewayDynamicRouting_VersionRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AiGatewayDynamicRouting_Deployment {}

impl AiGatewayDynamicRouting_Deployment { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Deployment {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Deployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Deployment {}

impl BuildAiGatewayDynamicRouting_Deployment {
    pub fn build(self) -> AiGatewayDynamicRouting_Deployment {
        AiGatewayDynamicRouting_Deployment {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_DeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_DeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGatewayDynamicRouting_DeploymentRef {
        AiGatewayDynamicRouting_DeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_DeploymentRef {
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
pub struct AiGatewayDynamicRouting_Elements_Outputs_Fallback {
    element_id: terrars::PrimField<std::string::String>,
}

impl AiGatewayDynamicRouting_Elements_Outputs_Fallback { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Elements_Outputs_Fallback {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Elements_Outputs_Fallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Elements_Outputs_Fallback {
    #[doc = ""]
    pub element_id: terrars::PrimField<std::string::String>,
}

impl BuildAiGatewayDynamicRouting_Elements_Outputs_Fallback {
    pub fn build(self) -> AiGatewayDynamicRouting_Elements_Outputs_Fallback {
        AiGatewayDynamicRouting_Elements_Outputs_Fallback { element_id: self.element_id }
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
        AiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
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
pub struct AiGatewayDynamicRouting_Elements_Outputs_False {
    element_id: terrars::PrimField<std::string::String>,
}

impl AiGatewayDynamicRouting_Elements_Outputs_False { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Elements_Outputs_False {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Elements_Outputs_False>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Elements_Outputs_False {
    #[doc = ""]
    pub element_id: terrars::PrimField<std::string::String>,
}

impl BuildAiGatewayDynamicRouting_Elements_Outputs_False {
    pub fn build(self) -> AiGatewayDynamicRouting_Elements_Outputs_False {
        AiGatewayDynamicRouting_Elements_Outputs_False { element_id: self.element_id }
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Elements_Outputs_FalseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Elements_Outputs_FalseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Elements_Outputs_FalseRef {
        AiGatewayDynamicRouting_Elements_Outputs_FalseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Elements_Outputs_FalseRef {
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
pub struct AiGatewayDynamicRouting_Elements_Outputs_Next {
    element_id: terrars::PrimField<std::string::String>,
}

impl AiGatewayDynamicRouting_Elements_Outputs_Next { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Elements_Outputs_Next {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Elements_Outputs_Next>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Elements_Outputs_Next {
    #[doc = ""]
    pub element_id: terrars::PrimField<std::string::String>,
}

impl BuildAiGatewayDynamicRouting_Elements_Outputs_Next {
    pub fn build(self) -> AiGatewayDynamicRouting_Elements_Outputs_Next {
        AiGatewayDynamicRouting_Elements_Outputs_Next { element_id: self.element_id }
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Elements_Outputs_NextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Elements_Outputs_NextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Elements_Outputs_NextRef {
        AiGatewayDynamicRouting_Elements_Outputs_NextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Elements_Outputs_NextRef {
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
pub struct AiGatewayDynamicRouting_Elements_Outputs_Success {
    element_id: terrars::PrimField<std::string::String>,
}

impl AiGatewayDynamicRouting_Elements_Outputs_Success { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Elements_Outputs_Success {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Elements_Outputs_Success>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Elements_Outputs_Success {
    #[doc = ""]
    pub element_id: terrars::PrimField<std::string::String>,
}

impl BuildAiGatewayDynamicRouting_Elements_Outputs_Success {
    pub fn build(self) -> AiGatewayDynamicRouting_Elements_Outputs_Success {
        AiGatewayDynamicRouting_Elements_Outputs_Success { element_id: self.element_id }
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
        AiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
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
pub struct AiGatewayDynamicRouting_Elements_Outputs_True {
    element_id: terrars::PrimField<std::string::String>,
}

impl AiGatewayDynamicRouting_Elements_Outputs_True { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Elements_Outputs_True {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Elements_Outputs_True>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Elements_Outputs_True {
    #[doc = ""]
    pub element_id: terrars::PrimField<std::string::String>,
}

impl BuildAiGatewayDynamicRouting_Elements_Outputs_True {
    pub fn build(self) -> AiGatewayDynamicRouting_Elements_Outputs_True {
        AiGatewayDynamicRouting_Elements_Outputs_True { element_id: self.element_id }
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Elements_Outputs_TrueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Elements_Outputs_TrueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Elements_Outputs_TrueRef {
        AiGatewayDynamicRouting_Elements_Outputs_TrueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Elements_Outputs_TrueRef {
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
pub struct AiGatewayDynamicRouting_Elements_Outputs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    element_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback: std::option::Option<AiGatewayDynamicRouting_Elements_Outputs_Fallback>,
    #[serde(rename = "false", skip_serializing_if = "std::option::Option::is_none")]
    false_: std::option::Option<AiGatewayDynamicRouting_Elements_Outputs_False>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next: std::option::Option<AiGatewayDynamicRouting_Elements_Outputs_Next>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success: std::option::Option<AiGatewayDynamicRouting_Elements_Outputs_Success>,
    #[serde(rename = "true", skip_serializing_if = "std::option::Option::is_none")]
    true_: std::option::Option<AiGatewayDynamicRouting_Elements_Outputs_True>,
}

impl AiGatewayDynamicRouting_Elements_Outputs {
    #[doc = "Set the field `element_id`.\n"]
    pub fn set_element_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.element_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fallback`.\n"]
    pub fn set_fallback(mut self, v: impl std::convert::Into<AiGatewayDynamicRouting_Elements_Outputs_Fallback>) -> Self {
        self.fallback = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `false_`.\n"]
    pub fn set_false(mut self, v: impl std::convert::Into<AiGatewayDynamicRouting_Elements_Outputs_False>) -> Self {
        self.false_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next`.\n"]
    pub fn set_next(mut self, v: impl std::convert::Into<AiGatewayDynamicRouting_Elements_Outputs_Next>) -> Self {
        self.next = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success`.\n"]
    pub fn set_success(mut self, v: impl std::convert::Into<AiGatewayDynamicRouting_Elements_Outputs_Success>) -> Self {
        self.success = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `true_`.\n"]
    pub fn set_true(mut self, v: impl std::convert::Into<AiGatewayDynamicRouting_Elements_Outputs_True>) -> Self {
        self.true_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiGatewayDynamicRouting_Elements_Outputs {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Elements_Outputs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Elements_Outputs {}

impl BuildAiGatewayDynamicRouting_Elements_Outputs {
    pub fn build(self) -> AiGatewayDynamicRouting_Elements_Outputs {
        AiGatewayDynamicRouting_Elements_Outputs {
            element_id: core::default::Default::default(),
            fallback: core::default::Default::default(),
            false_: core::default::Default::default(),
            next: core::default::Default::default(),
            success: core::default::Default::default(),
            true_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Elements_OutputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Elements_OutputsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGatewayDynamicRouting_Elements_OutputsRef {
        AiGatewayDynamicRouting_Elements_OutputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Elements_OutputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `element_id` after provisioning.\n"]
    pub fn element_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.element_id", self.base))
    }

    #[doc = "Get a reference to the value of field `fallback` after provisioning.\n"]
    pub fn fallback(&self) -> AiGatewayDynamicRouting_Elements_Outputs_FallbackRef {
        AiGatewayDynamicRouting_Elements_Outputs_FallbackRef::new(
            self.shared().clone(),
            format!("{}.fallback", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `false_` after provisioning.\n"]
    pub fn false_(&self) -> AiGatewayDynamicRouting_Elements_Outputs_FalseRef {
        AiGatewayDynamicRouting_Elements_Outputs_FalseRef::new(self.shared().clone(), format!("{}.false", self.base))
    }

    #[doc = "Get a reference to the value of field `next` after provisioning.\n"]
    pub fn next(&self) -> AiGatewayDynamicRouting_Elements_Outputs_NextRef {
        AiGatewayDynamicRouting_Elements_Outputs_NextRef::new(self.shared().clone(), format!("{}.next", self.base))
    }

    #[doc = "Get a reference to the value of field `success` after provisioning.\n"]
    pub fn success(&self) -> AiGatewayDynamicRouting_Elements_Outputs_SuccessRef {
        AiGatewayDynamicRouting_Elements_Outputs_SuccessRef::new(
            self.shared().clone(),
            format!("{}.success", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `true_` after provisioning.\n"]
    pub fn true_(&self) -> AiGatewayDynamicRouting_Elements_Outputs_TrueRef {
        AiGatewayDynamicRouting_Elements_Outputs_TrueRef::new(self.shared().clone(), format!("{}.true", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGatewayDynamicRouting_Elements_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_gateway_dynamic_routing_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<terrars::PrimField<f64>>,
}

impl AiGatewayDynamicRouting_Elements_Properties {
    #[doc = "Set the field `ai_gateway_dynamic_routing_provider`.\n"]
    pub fn set_ai_gateway_dynamic_routing_provider(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ai_gateway_dynamic_routing_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit`.\n"]
    pub fn set_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit_type`.\nAvailable values: \"count\", \"cost\"."]
    pub fn set_limit_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.limit_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retries`.\n"]
    pub fn set_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window`.\n"]
    pub fn set_window(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiGatewayDynamicRouting_Elements_Properties {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Elements_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Elements_Properties {}

impl BuildAiGatewayDynamicRouting_Elements_Properties {
    pub fn build(self) -> AiGatewayDynamicRouting_Elements_Properties {
        AiGatewayDynamicRouting_Elements_Properties {
            ai_gateway_dynamic_routing_provider: core::default::Default::default(),
            conditions: core::default::Default::default(),
            key: core::default::Default::default(),
            limit: core::default::Default::default(),
            limit_type: core::default::Default::default(),
            model: core::default::Default::default(),
            retries: core::default::Default::default(),
            timeout: core::default::Default::default(),
            window: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Elements_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Elements_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Elements_PropertiesRef {
        AiGatewayDynamicRouting_Elements_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Elements_PropertiesRef {
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
pub struct AiGatewayDynamicRouting_Elements {
    id: terrars::PrimField<std::string::String>,
    outputs: AiGatewayDynamicRouting_Elements_Outputs,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<AiGatewayDynamicRouting_Elements_Properties>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl AiGatewayDynamicRouting_Elements {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl std::convert::Into<AiGatewayDynamicRouting_Elements_Properties>) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AiGatewayDynamicRouting_Elements {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Elements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Elements {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub outputs: AiGatewayDynamicRouting_Elements_Outputs,
    #[doc = "Available values: \"start\", \"conditional\", \"percentage\", \"rate\", \"model\", \"end\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAiGatewayDynamicRouting_Elements {
    pub fn build(self) -> AiGatewayDynamicRouting_Elements {
        AiGatewayDynamicRouting_Elements {
            id: self.id,
            outputs: self.outputs,
            properties: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_ElementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_ElementsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGatewayDynamicRouting_ElementsRef {
        AiGatewayDynamicRouting_ElementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_ElementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> AiGatewayDynamicRouting_Elements_OutputsRef {
        AiGatewayDynamicRouting_Elements_OutputsRef::new(self.shared().clone(), format!("{}.outputs", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> AiGatewayDynamicRouting_Elements_PropertiesRef {
        AiGatewayDynamicRouting_Elements_PropertiesRef::new(
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
pub struct AiGatewayDynamicRouting_Route_Deployment {}

impl AiGatewayDynamicRouting_Route_Deployment { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route_Deployment {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route_Deployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route_Deployment {}

impl BuildAiGatewayDynamicRouting_Route_Deployment {
    pub fn build(self) -> AiGatewayDynamicRouting_Route_Deployment {
        AiGatewayDynamicRouting_Route_Deployment {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Route_DeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Route_DeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGatewayDynamicRouting_Route_DeploymentRef {
        AiGatewayDynamicRouting_Route_DeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Route_DeploymentRef {
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
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs_Fallback {}

impl AiGatewayDynamicRouting_Route_Elements_Outputs_Fallback { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route_Elements_Outputs_Fallback {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route_Elements_Outputs_Fallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route_Elements_Outputs_Fallback {}

impl BuildAiGatewayDynamicRouting_Route_Elements_Outputs_Fallback {
    pub fn build(self) -> AiGatewayDynamicRouting_Route_Elements_Outputs_Fallback {
        AiGatewayDynamicRouting_Route_Elements_Outputs_Fallback {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs_FallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Route_Elements_Outputs_FallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Route_Elements_Outputs_FallbackRef {
        AiGatewayDynamicRouting_Route_Elements_Outputs_FallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Route_Elements_Outputs_FallbackRef {
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
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs_False {}

impl AiGatewayDynamicRouting_Route_Elements_Outputs_False { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route_Elements_Outputs_False {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route_Elements_Outputs_False>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route_Elements_Outputs_False {}

impl BuildAiGatewayDynamicRouting_Route_Elements_Outputs_False {
    pub fn build(self) -> AiGatewayDynamicRouting_Route_Elements_Outputs_False {
        AiGatewayDynamicRouting_Route_Elements_Outputs_False {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs_FalseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Route_Elements_Outputs_FalseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Route_Elements_Outputs_FalseRef {
        AiGatewayDynamicRouting_Route_Elements_Outputs_FalseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Route_Elements_Outputs_FalseRef {
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
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs_Next {}

impl AiGatewayDynamicRouting_Route_Elements_Outputs_Next { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route_Elements_Outputs_Next {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route_Elements_Outputs_Next>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route_Elements_Outputs_Next {}

impl BuildAiGatewayDynamicRouting_Route_Elements_Outputs_Next {
    pub fn build(self) -> AiGatewayDynamicRouting_Route_Elements_Outputs_Next {
        AiGatewayDynamicRouting_Route_Elements_Outputs_Next {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs_NextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Route_Elements_Outputs_NextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Route_Elements_Outputs_NextRef {
        AiGatewayDynamicRouting_Route_Elements_Outputs_NextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Route_Elements_Outputs_NextRef {
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
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs_Success {}

impl AiGatewayDynamicRouting_Route_Elements_Outputs_Success { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route_Elements_Outputs_Success {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route_Elements_Outputs_Success>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route_Elements_Outputs_Success {}

impl BuildAiGatewayDynamicRouting_Route_Elements_Outputs_Success {
    pub fn build(self) -> AiGatewayDynamicRouting_Route_Elements_Outputs_Success {
        AiGatewayDynamicRouting_Route_Elements_Outputs_Success {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs_SuccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Route_Elements_Outputs_SuccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Route_Elements_Outputs_SuccessRef {
        AiGatewayDynamicRouting_Route_Elements_Outputs_SuccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Route_Elements_Outputs_SuccessRef {
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
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs_True {}

impl AiGatewayDynamicRouting_Route_Elements_Outputs_True { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route_Elements_Outputs_True {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route_Elements_Outputs_True>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route_Elements_Outputs_True {}

impl BuildAiGatewayDynamicRouting_Route_Elements_Outputs_True {
    pub fn build(self) -> AiGatewayDynamicRouting_Route_Elements_Outputs_True {
        AiGatewayDynamicRouting_Route_Elements_Outputs_True {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs_TrueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Route_Elements_Outputs_TrueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Route_Elements_Outputs_TrueRef {
        AiGatewayDynamicRouting_Route_Elements_Outputs_TrueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Route_Elements_Outputs_TrueRef {
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
pub struct AiGatewayDynamicRouting_Route_Elements_Outputs {}

impl AiGatewayDynamicRouting_Route_Elements_Outputs { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route_Elements_Outputs {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route_Elements_Outputs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route_Elements_Outputs {}

impl BuildAiGatewayDynamicRouting_Route_Elements_Outputs {
    pub fn build(self) -> AiGatewayDynamicRouting_Route_Elements_Outputs {
        AiGatewayDynamicRouting_Route_Elements_Outputs {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Route_Elements_OutputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Route_Elements_OutputsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Route_Elements_OutputsRef {
        AiGatewayDynamicRouting_Route_Elements_OutputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Route_Elements_OutputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `element_id` after provisioning.\n"]
    pub fn element_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.element_id", self.base))
    }

    #[doc = "Get a reference to the value of field `fallback` after provisioning.\n"]
    pub fn fallback(&self) -> AiGatewayDynamicRouting_Route_Elements_Outputs_FallbackRef {
        AiGatewayDynamicRouting_Route_Elements_Outputs_FallbackRef::new(
            self.shared().clone(),
            format!("{}.fallback", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `false_` after provisioning.\n"]
    pub fn false_(&self) -> AiGatewayDynamicRouting_Route_Elements_Outputs_FalseRef {
        AiGatewayDynamicRouting_Route_Elements_Outputs_FalseRef::new(
            self.shared().clone(),
            format!("{}.false", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `next` after provisioning.\n"]
    pub fn next(&self) -> AiGatewayDynamicRouting_Route_Elements_Outputs_NextRef {
        AiGatewayDynamicRouting_Route_Elements_Outputs_NextRef::new(
            self.shared().clone(),
            format!("{}.next", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `success` after provisioning.\n"]
    pub fn success(&self) -> AiGatewayDynamicRouting_Route_Elements_Outputs_SuccessRef {
        AiGatewayDynamicRouting_Route_Elements_Outputs_SuccessRef::new(
            self.shared().clone(),
            format!("{}.success", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `true_` after provisioning.\n"]
    pub fn true_(&self) -> AiGatewayDynamicRouting_Route_Elements_Outputs_TrueRef {
        AiGatewayDynamicRouting_Route_Elements_Outputs_TrueRef::new(
            self.shared().clone(),
            format!("{}.true", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AiGatewayDynamicRouting_Route_Elements_Properties {}

impl AiGatewayDynamicRouting_Route_Elements_Properties { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route_Elements_Properties {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route_Elements_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route_Elements_Properties {}

impl BuildAiGatewayDynamicRouting_Route_Elements_Properties {
    pub fn build(self) -> AiGatewayDynamicRouting_Route_Elements_Properties {
        AiGatewayDynamicRouting_Route_Elements_Properties {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Route_Elements_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Route_Elements_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AiGatewayDynamicRouting_Route_Elements_PropertiesRef {
        AiGatewayDynamicRouting_Route_Elements_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Route_Elements_PropertiesRef {
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
pub struct AiGatewayDynamicRouting_Route_Elements {}

impl AiGatewayDynamicRouting_Route_Elements { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route_Elements {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route_Elements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route_Elements {}

impl BuildAiGatewayDynamicRouting_Route_Elements {
    pub fn build(self) -> AiGatewayDynamicRouting_Route_Elements {
        AiGatewayDynamicRouting_Route_Elements {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Route_ElementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Route_ElementsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGatewayDynamicRouting_Route_ElementsRef {
        AiGatewayDynamicRouting_Route_ElementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Route_ElementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> AiGatewayDynamicRouting_Route_Elements_OutputsRef {
        AiGatewayDynamicRouting_Route_Elements_OutputsRef::new(
            self.shared().clone(),
            format!("{}.outputs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> AiGatewayDynamicRouting_Route_Elements_PropertiesRef {
        AiGatewayDynamicRouting_Route_Elements_PropertiesRef::new(
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
pub struct AiGatewayDynamicRouting_Route_Version {}

impl AiGatewayDynamicRouting_Route_Version { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route_Version {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route_Version>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route_Version {}

impl BuildAiGatewayDynamicRouting_Route_Version {
    pub fn build(self) -> AiGatewayDynamicRouting_Route_Version {
        AiGatewayDynamicRouting_Route_Version {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_Route_VersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_Route_VersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGatewayDynamicRouting_Route_VersionRef {
        AiGatewayDynamicRouting_Route_VersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_Route_VersionRef {
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

#[derive(Serialize)]
pub struct AiGatewayDynamicRouting_Route {}

impl AiGatewayDynamicRouting_Route { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Route {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Route>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Route {}

impl BuildAiGatewayDynamicRouting_Route {
    pub fn build(self) -> AiGatewayDynamicRouting_Route {
        AiGatewayDynamicRouting_Route {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_RouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_RouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGatewayDynamicRouting_RouteRef {
        AiGatewayDynamicRouting_RouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_RouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_tag` after provisioning.\n"]
    pub fn account_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_tag", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> AiGatewayDynamicRouting_Route_DeploymentRef {
        AiGatewayDynamicRouting_Route_DeploymentRef::new(self.shared().clone(), format!("{}.deployment", self.base))
    }

    #[doc = "Get a reference to the value of field `elements` after provisioning.\n"]
    pub fn elements(&self) -> terrars::ListRef<AiGatewayDynamicRouting_Route_ElementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elements", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> AiGatewayDynamicRouting_Route_VersionRef {
        AiGatewayDynamicRouting_Route_VersionRef::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct AiGatewayDynamicRouting_Version {}

impl AiGatewayDynamicRouting_Version { }

impl terrars::ToListMappable for AiGatewayDynamicRouting_Version {
    type O = terrars::BlockAssignable<AiGatewayDynamicRouting_Version>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAiGatewayDynamicRouting_Version {}

impl BuildAiGatewayDynamicRouting_Version {
    pub fn build(self) -> AiGatewayDynamicRouting_Version {
        AiGatewayDynamicRouting_Version {}
    }
}

#[allow(dead_code)]
pub struct AiGatewayDynamicRouting_VersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AiGatewayDynamicRouting_VersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AiGatewayDynamicRouting_VersionRef {
        AiGatewayDynamicRouting_VersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AiGatewayDynamicRouting_VersionRef {
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
