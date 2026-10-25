#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ApiShieldOperationData_ {
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
    endpoint: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    method: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct ApiShieldOperation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApiShieldOperationData_>,
}

#[derive(Clone)]
pub struct ApiShieldOperation(std::rc::Rc<ApiShieldOperation_>);

impl ApiShieldOperation {
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

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with {varN}, starting with {var1}, during insertion. This will further be Cloudflare-normalized upon insertion. See: https://developers.cloudflare.com/rules/normalization/how-it-works/."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> ApiShieldOperation_FeaturesRef {
        ApiShieldOperation_FeaturesRef::new(self.shared().clone(), format!("{}.features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nRFC3986-compliant host."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nThe HTTP method used to access the endpoint.\nAvailable values: \"GET\", \"POST\", \"HEAD\", \"OPTIONS\", \"PUT\", \"DELETE\", \"CONNECT\", \"PATCH\", \"TRACE\"."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_id` after provisioning.\nUUID."]
    pub fn operation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schemas` after provisioning.\nOpenAPI JSON schemas for an operation, including both user-uploaded and Cloudflare-learned schemas."]
    pub fn schemas(&self) -> ApiShieldOperation_SchemasRef {
        ApiShieldOperation_SchemasRef::new(self.shared().clone(), format!("{}.schemas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for ApiShieldOperation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApiShieldOperation { }

impl terrars::ToListMappable for ApiShieldOperation {
    type O = terrars::ListRef<ApiShieldOperationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApiShieldOperation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_api_shield_operation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiShieldOperation {
    pub tf_id: std::string::String,
    #[doc =
        "The endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with {varN}, starting with {var1}, during insertion. This will further be Cloudflare-normalized upon insertion. See: https://developers.cloudflare.com/rules/normalization/how-it-works/."]
    pub endpoint: terrars::PrimField<std::string::String>,
    #[doc = "RFC3986-compliant host."]
    pub host: terrars::PrimField<std::string::String>,
    #[doc =
        "The HTTP method used to access the endpoint.\nAvailable values: \"GET\", \"POST\", \"HEAD\", \"OPTIONS\", \"PUT\", \"DELETE\", \"CONNECT\", \"PATCH\", \"TRACE\"."]
    pub method: terrars::PrimField<std::string::String>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildApiShieldOperation {
    pub fn build(self, stack: &mut terrars::Stack) -> ApiShieldOperation {
        let out = ApiShieldOperation(std::rc::Rc::new(ApiShieldOperation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApiShieldOperationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                endpoint: self.endpoint,
                host: self.host,
                method: self.method,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiShieldOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiShieldOperationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with {varN}, starting with {var1}, during insertion. This will further be Cloudflare-normalized upon insertion. See: https://developers.cloudflare.com/rules/normalization/how-it-works/."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> ApiShieldOperation_FeaturesRef {
        ApiShieldOperation_FeaturesRef::new(self.shared().clone(), format!("{}.features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nRFC3986-compliant host."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nThe HTTP method used to access the endpoint.\nAvailable values: \"GET\", \"POST\", \"HEAD\", \"OPTIONS\", \"PUT\", \"DELETE\", \"CONNECT\", \"PATCH\", \"TRACE\"."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_id` after provisioning.\nUUID."]
    pub fn operation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schemas` after provisioning.\nOpenAPI JSON schemas for an operation, including both user-uploaded and Cloudflare-learned schemas."]
    pub fn schemas(&self) -> ApiShieldOperation_SchemasRef {
        ApiShieldOperation_SchemasRef::new(self.shared().clone(), format!("{}.schemas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_ApiRouting {}

impl ApiShieldOperation_Features_ApiRouting { }

impl terrars::ToListMappable for ApiShieldOperation_Features_ApiRouting {
    type O = terrars::BlockAssignable<ApiShieldOperation_Features_ApiRouting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_ApiRouting {}

impl BuildApiShieldOperation_Features_ApiRouting {
    pub fn build(self) -> ApiShieldOperation_Features_ApiRouting {
        ApiShieldOperation_Features_ApiRouting {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_ApiRoutingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_ApiRoutingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiShieldOperation_Features_ApiRoutingRef {
        ApiShieldOperation_Features_ApiRoutingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_ApiRoutingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc = "Get a reference to the value of field `route` after provisioning.\nTarget route."]
    pub fn route(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {}

impl ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 { }

impl terrars
::ToListMappable for ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {
    type O =
        terrars
        ::BlockAssignable<ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {}

impl BuildApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {
    pub fn build(self) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90 {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower` after provisioning.\nLower bound for percentile estimate"]
    pub fn lower(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower", self.base))
    }

    #[doc = "Get a reference to the value of field `upper` after provisioning.\nUpper bound for percentile estimate"]
    pub fn upper(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {}

impl ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 { }

impl terrars
::ToListMappable for ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {
    type O =
        terrars
        ::BlockAssignable<ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {}

impl BuildApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {
    pub fn build(self) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95 {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower` after provisioning.\nLower bound for percentile estimate"]
    pub fn lower(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower", self.base))
    }

    #[doc = "Get a reference to the value of field `upper` after provisioning.\nUpper bound for percentile estimate"]
    pub fn upper(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {}

impl ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 { }

impl terrars
::ToListMappable for ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {
    type O =
        terrars
        ::BlockAssignable<ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {}

impl BuildApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {
    pub fn build(self) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99 {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower` after provisioning.\nLower bound for percentile estimate"]
    pub fn lower(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower", self.base))
    }

    #[doc = "Get a reference to the value of field `upper` after provisioning.\nUpper bound for percentile estimate"]
    pub fn upper(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {}

impl ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals { }

impl terrars
::ToListMappable for ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {
    type O =
        terrars
        ::BlockAssignable<ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {}

impl BuildApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {
    pub fn build(self) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `p90` after provisioning.\nUpper and lower bound for percentile estimate"]
    pub fn p90(&self) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P90Ref::new(
            self.shared().clone(),
            format!("{}.p90", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `p95` after provisioning.\nUpper and lower bound for percentile estimate"]
    pub fn p95(&self) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P95Ref::new(
            self.shared().clone(),
            format!("{}.p95", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `p99` after provisioning.\nUpper and lower bound for percentile estimate"]
    pub fn p99(&self) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervals_P99Ref::new(
            self.shared().clone(),
            format!("{}.p99", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold {}

impl ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold { }

impl terrars::ToListMappable for ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold {
    type O = terrars::BlockAssignable<ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold {}

impl BuildApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold {
    pub fn build(self) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThresholdRef {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `confidence_intervals` after provisioning.\n"]
    pub fn confidence_intervals(
        &self,
    ) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThreshold_ConfidenceIntervalsRef::new(
            self.shared().clone(),
            format!("{}.confidence_intervals", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `mean` after provisioning.\nSuggested threshold."]
    pub fn mean(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mean", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_ConfidenceIntervals {}

impl ApiShieldOperation_Features_ConfidenceIntervals { }

impl terrars::ToListMappable for ApiShieldOperation_Features_ConfidenceIntervals {
    type O = terrars::BlockAssignable<ApiShieldOperation_Features_ConfidenceIntervals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_ConfidenceIntervals {}

impl BuildApiShieldOperation_Features_ConfidenceIntervals {
    pub fn build(self) -> ApiShieldOperation_Features_ConfidenceIntervals {
        ApiShieldOperation_Features_ConfidenceIntervals {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_ConfidenceIntervalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_ConfidenceIntervalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApiShieldOperation_Features_ConfidenceIntervalsRef {
        ApiShieldOperation_Features_ConfidenceIntervalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_ConfidenceIntervalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc = "Get a reference to the value of field `suggested_threshold` after provisioning.\n"]
    pub fn suggested_threshold(&self) -> ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThresholdRef {
        ApiShieldOperation_Features_ConfidenceIntervals_SuggestedThresholdRef::new(
            self.shared().clone(),
            format!("{}.suggested_threshold", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_ParameterSchemas_ParameterSchemas {}

impl ApiShieldOperation_Features_ParameterSchemas_ParameterSchemas { }

impl terrars::ToListMappable for ApiShieldOperation_Features_ParameterSchemas_ParameterSchemas {
    type O = terrars::BlockAssignable<ApiShieldOperation_Features_ParameterSchemas_ParameterSchemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_ParameterSchemas_ParameterSchemas {}

impl BuildApiShieldOperation_Features_ParameterSchemas_ParameterSchemas {
    pub fn build(self) -> ApiShieldOperation_Features_ParameterSchemas_ParameterSchemas {
        ApiShieldOperation_Features_ParameterSchemas_ParameterSchemas {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_ParameterSchemas_ParameterSchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_ParameterSchemas_ParameterSchemasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApiShieldOperation_Features_ParameterSchemas_ParameterSchemasRef {
        ApiShieldOperation_Features_ParameterSchemas_ParameterSchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_ParameterSchemas_ParameterSchemasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nAn array containing the learned parameter schemas."]
    pub fn parameters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `responses` after provisioning.\nAn empty response object. This field is required to yield a valid operation schema."]
    pub fn responses(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.responses", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_ParameterSchemas {}

impl ApiShieldOperation_Features_ParameterSchemas { }

impl terrars::ToListMappable for ApiShieldOperation_Features_ParameterSchemas {
    type O = terrars::BlockAssignable<ApiShieldOperation_Features_ParameterSchemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_ParameterSchemas {}

impl BuildApiShieldOperation_Features_ParameterSchemas {
    pub fn build(self) -> ApiShieldOperation_Features_ParameterSchemas {
        ApiShieldOperation_Features_ParameterSchemas {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_ParameterSchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_ParameterSchemasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApiShieldOperation_Features_ParameterSchemasRef {
        ApiShieldOperation_Features_ParameterSchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_ParameterSchemasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameter_schemas` after provisioning.\nAn operation schema object containing a response."]
    pub fn parameter_schemas(&self) -> ApiShieldOperation_Features_ParameterSchemas_ParameterSchemasRef {
        ApiShieldOperation_Features_ParameterSchemas_ParameterSchemasRef::new(
            self.shared().clone(),
            format!("{}.parameter_schemas", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_SchemaInfo_ActiveSchema {}

impl ApiShieldOperation_Features_SchemaInfo_ActiveSchema { }

impl terrars::ToListMappable for ApiShieldOperation_Features_SchemaInfo_ActiveSchema {
    type O = terrars::BlockAssignable<ApiShieldOperation_Features_SchemaInfo_ActiveSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_SchemaInfo_ActiveSchema {}

impl BuildApiShieldOperation_Features_SchemaInfo_ActiveSchema {
    pub fn build(self) -> ApiShieldOperation_Features_SchemaInfo_ActiveSchema {
        ApiShieldOperation_Features_SchemaInfo_ActiveSchema {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_SchemaInfo_ActiveSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_SchemaInfo_ActiveSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApiShieldOperation_Features_SchemaInfo_ActiveSchemaRef {
        ApiShieldOperation_Features_SchemaInfo_ActiveSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_SchemaInfo_ActiveSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSchema file name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_SchemaInfo {}

impl ApiShieldOperation_Features_SchemaInfo { }

impl terrars::ToListMappable for ApiShieldOperation_Features_SchemaInfo {
    type O = terrars::BlockAssignable<ApiShieldOperation_Features_SchemaInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_SchemaInfo {}

impl BuildApiShieldOperation_Features_SchemaInfo {
    pub fn build(self) -> ApiShieldOperation_Features_SchemaInfo {
        ApiShieldOperation_Features_SchemaInfo {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_SchemaInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_SchemaInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiShieldOperation_Features_SchemaInfoRef {
        ApiShieldOperation_Features_SchemaInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_SchemaInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_schema` after provisioning.\nSchema active on endpoint."]
    pub fn active_schema(&self) -> ApiShieldOperation_Features_SchemaInfo_ActiveSchemaRef {
        ApiShieldOperation_Features_SchemaInfo_ActiveSchemaRef::new(
            self.shared().clone(),
            format!("{}.active_schema", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `mitigation_action` after provisioning.\nAction taken on requests failing validation.\nAvailable values: \"none\", \"log\", \"block\"."]
    pub fn mitigation_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mitigation_action", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features_Thresholds {}

impl ApiShieldOperation_Features_Thresholds { }

impl terrars::ToListMappable for ApiShieldOperation_Features_Thresholds {
    type O = terrars::BlockAssignable<ApiShieldOperation_Features_Thresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features_Thresholds {}

impl BuildApiShieldOperation_Features_Thresholds {
    pub fn build(self) -> ApiShieldOperation_Features_Thresholds {
        ApiShieldOperation_Features_Thresholds {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Features_ThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Features_ThresholdsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiShieldOperation_Features_ThresholdsRef {
        ApiShieldOperation_Features_ThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Features_ThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_id_tokens` after provisioning.\nThe total number of auth-ids seen across this calculation."]
    pub fn auth_id_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_id_tokens", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_points` after provisioning.\nThe number of data points used for the threshold suggestion calculation."]
    pub fn data_points(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_points", self.base))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc =
        "Get a reference to the value of field `p50` after provisioning.\nThe p50 quantile of requests (in period_seconds)."]
    pub fn p50(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p50", self.base))
    }

    #[doc =
        "Get a reference to the value of field `p90` after provisioning.\nThe p90 quantile of requests (in period_seconds)."]
    pub fn p90(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p90", self.base))
    }

    #[doc =
        "Get a reference to the value of field `p99` after provisioning.\nThe p99 quantile of requests (in period_seconds)."]
    pub fn p99(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p99", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nThe period over which this threshold is suggested."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests` after provisioning.\nThe estimated number of requests covered by these calculations."]
    pub fn requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suggested_threshold` after provisioning.\nThe suggested threshold in requests done by the same auth_id or period_seconds."]
    pub fn suggested_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suggested_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Features {}

impl ApiShieldOperation_Features { }

impl terrars::ToListMappable for ApiShieldOperation_Features {
    type O = terrars::BlockAssignable<ApiShieldOperation_Features>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Features {}

impl BuildApiShieldOperation_Features {
    pub fn build(self) -> ApiShieldOperation_Features {
        ApiShieldOperation_Features {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_FeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_FeaturesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiShieldOperation_FeaturesRef {
        ApiShieldOperation_FeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_FeaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_routing` after provisioning.\nAPI Routing settings on endpoint."]
    pub fn api_routing(&self) -> ApiShieldOperation_Features_ApiRoutingRef {
        ApiShieldOperation_Features_ApiRoutingRef::new(self.shared().clone(), format!("{}.api_routing", self.base))
    }

    #[doc = "Get a reference to the value of field `confidence_intervals` after provisioning.\n"]
    pub fn confidence_intervals(&self) -> ApiShieldOperation_Features_ConfidenceIntervalsRef {
        ApiShieldOperation_Features_ConfidenceIntervalsRef::new(
            self.shared().clone(),
            format!("{}.confidence_intervals", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_schemas` after provisioning.\n"]
    pub fn parameter_schemas(&self) -> ApiShieldOperation_Features_ParameterSchemasRef {
        ApiShieldOperation_Features_ParameterSchemasRef::new(
            self.shared().clone(),
            format!("{}.parameter_schemas", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `schema_info` after provisioning.\n"]
    pub fn schema_info(&self) -> ApiShieldOperation_Features_SchemaInfoRef {
        ApiShieldOperation_Features_SchemaInfoRef::new(self.shared().clone(), format!("{}.schema_info", self.base))
    }

    #[doc = "Get a reference to the value of field `thresholds` after provisioning.\n"]
    pub fn thresholds(&self) -> ApiShieldOperation_Features_ThresholdsRef {
        ApiShieldOperation_Features_ThresholdsRef::new(self.shared().clone(), format!("{}.thresholds", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Schemas_Learned {}

impl ApiShieldOperation_Schemas_Learned { }

impl terrars::ToListMappable for ApiShieldOperation_Schemas_Learned {
    type O = terrars::BlockAssignable<ApiShieldOperation_Schemas_Learned>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Schemas_Learned {}

impl BuildApiShieldOperation_Schemas_Learned {
    pub fn build(self) -> ApiShieldOperation_Schemas_Learned {
        ApiShieldOperation_Schemas_Learned {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Schemas_LearnedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Schemas_LearnedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiShieldOperation_Schemas_LearnedRef {
        ApiShieldOperation_Schemas_LearnedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Schemas_LearnedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nOpenAPI parameter objects describing path, query, header, or cookie parameters."]
    pub fn parameters(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_body` after provisioning.\nOpenAPI request body object describing the expected request payload."]
    pub fn request_body(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_body", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Schemas_Uploaded {}

impl ApiShieldOperation_Schemas_Uploaded { }

impl terrars::ToListMappable for ApiShieldOperation_Schemas_Uploaded {
    type O = terrars::BlockAssignable<ApiShieldOperation_Schemas_Uploaded>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Schemas_Uploaded {}

impl BuildApiShieldOperation_Schemas_Uploaded {
    pub fn build(self) -> ApiShieldOperation_Schemas_Uploaded {
        ApiShieldOperation_Schemas_Uploaded {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_Schemas_UploadedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_Schemas_UploadedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiShieldOperation_Schemas_UploadedRef {
        ApiShieldOperation_Schemas_UploadedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_Schemas_UploadedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nOpenAPI parameter objects describing path, query, header, or cookie parameters."]
    pub fn parameters(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_body` after provisioning.\nOpenAPI request body object describing the expected request payload."]
    pub fn request_body(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_body", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldOperation_Schemas {}

impl ApiShieldOperation_Schemas { }

impl terrars::ToListMappable for ApiShieldOperation_Schemas {
    type O = terrars::BlockAssignable<ApiShieldOperation_Schemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldOperation_Schemas {}

impl BuildApiShieldOperation_Schemas {
    pub fn build(self) -> ApiShieldOperation_Schemas {
        ApiShieldOperation_Schemas {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldOperation_SchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldOperation_SchemasRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiShieldOperation_SchemasRef {
        ApiShieldOperation_SchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldOperation_SchemasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `learned` after provisioning.\nAn OpenAPI operation object fragment containing schema information for an operation. May include parameter definitions, request body specifications, and a component schema extension."]
    pub fn learned(&self) -> ApiShieldOperation_Schemas_LearnedRef {
        ApiShieldOperation_Schemas_LearnedRef::new(self.shared().clone(), format!("{}.learned", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uploaded` after provisioning.\nAn OpenAPI operation object fragment containing schema information for an operation. May include parameter definitions, request body specifications, and a component schema extension."]
    pub fn uploaded(&self) -> ApiShieldOperation_Schemas_UploadedRef {
        ApiShieldOperation_Schemas_UploadedRef::new(self.shared().clone(), format!("{}.uploaded", self.base))
    }
}
