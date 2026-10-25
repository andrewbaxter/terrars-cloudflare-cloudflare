#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct R2BucketCorsData_ {
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
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jurisdiction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<R2BucketCors_Rules>>,
}

struct R2BucketCors_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<R2BucketCorsData_>,
}

#[derive(Clone)]
pub struct R2BucketCors(std::rc::Rc<R2BucketCors_>);

impl R2BucketCors {
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

    #[doc = "Set the field `jurisdiction`.\nJurisdiction of the bucket"]
    pub fn set_jurisdiction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().jurisdiction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl std::convert::Into<std::vec::Vec<R2BucketCors_Rules>>) -> Self {
        self.0.data.borrow_mut().rules = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID that owns the R2 resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nName of the bucket."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jurisdiction` after provisioning.\nJurisdiction of the bucket"]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<R2BucketCors_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl terrars::Referable for R2BucketCors {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for R2BucketCors { }

impl terrars::ToListMappable for R2BucketCors {
    type O = terrars::ListRef<R2BucketCorsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for R2BucketCors_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_r2_bucket_cors".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildR2BucketCors {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID that owns the R2 resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildR2BucketCors {
    pub fn build(self, stack: &mut terrars::Stack) -> R2BucketCors {
        let out = R2BucketCors(std::rc::Rc::new(R2BucketCors_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(R2BucketCorsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                bucket_name: self.bucket_name,
                jurisdiction: core::default::Default::default(),
                rules: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct R2BucketCorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketCorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl R2BucketCorsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID that owns the R2 resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nName of the bucket."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jurisdiction` after provisioning.\nJurisdiction of the bucket"]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<R2BucketCors_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct R2BucketCors_Rules_Allowed {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    methods: terrars::ListField<terrars::PrimField<std::string::String>>,
    origins: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl R2BucketCors_Rules_Allowed {
    #[doc =
        "Set the field `headers`.\nSpecifies the value for the Access-Control-Allow-Headers header R2 sets when requesting objects in this bucket from a browser. Cross-origin requests that include custom headers (e.g. x-user-id) should specify these headers as AllowedHeaders."]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for R2BucketCors_Rules_Allowed {
    type O = terrars::BlockAssignable<R2BucketCors_Rules_Allowed>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketCors_Rules_Allowed {
    #[doc =
        "Specifies the value for the Access-Control-Allow-Methods header R2 sets when requesting objects in a bucket from a browser."]
    pub methods: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "Specifies the value for the Access-Control-Allow-Origin header R2 sets when requesting objects in a bucket from a browser."]
    pub origins: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildR2BucketCors_Rules_Allowed {
    pub fn build(self) -> R2BucketCors_Rules_Allowed {
        R2BucketCors_Rules_Allowed {
            headers: core::default::Default::default(),
            methods: self.methods,
            origins: self.origins,
        }
    }
}

#[allow(dead_code)]
pub struct R2BucketCors_Rules_AllowedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketCors_Rules_AllowedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> R2BucketCors_Rules_AllowedRef {
        R2BucketCors_Rules_AllowedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketCors_Rules_AllowedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nSpecifies the value for the Access-Control-Allow-Headers header R2 sets when requesting objects in this bucket from a browser. Cross-origin requests that include custom headers (e.g. x-user-id) should specify these headers as AllowedHeaders."]
    pub fn headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `methods` after provisioning.\nSpecifies the value for the Access-Control-Allow-Methods header R2 sets when requesting objects in a bucket from a browser."]
    pub fn methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origins` after provisioning.\nSpecifies the value for the Access-Control-Allow-Origin header R2 sets when requesting objects in a bucket from a browser."]
    pub fn origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origins", self.base))
    }
}

#[derive(Serialize)]
pub struct R2BucketCors_Rules {
    allowed: R2BucketCors_Rules_Allowed,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl R2BucketCors_Rules {
    #[doc =
        "Set the field `expose_headers`.\nSpecifies the headers that can be exposed back, and accessed by, the JavaScript making the cross-origin request. If you need to access headers beyond the safelisted response headers, such as Content-Encoding or cf-cache-status, you must specify it here."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nIdentifier for this rule."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_age_seconds`.\nSpecifies the amount of time (in seconds) browsers are allowed to cache CORS preflight responses. Browsers may limit this to 2 hours or less, even if the maximum value (86400) is specified."]
    pub fn set_max_age_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for R2BucketCors_Rules {
    type O = terrars::BlockAssignable<R2BucketCors_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketCors_Rules {
    #[doc = "Object specifying allowed origins, methods and headers for this CORS rule."]
    pub allowed: R2BucketCors_Rules_Allowed,
}

impl BuildR2BucketCors_Rules {
    pub fn build(self) -> R2BucketCors_Rules {
        R2BucketCors_Rules {
            allowed: self.allowed,
            expose_headers: core::default::Default::default(),
            id: core::default::Default::default(),
            max_age_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct R2BucketCors_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketCors_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> R2BucketCors_RulesRef {
        R2BucketCors_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketCors_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed` after provisioning.\nObject specifying allowed origins, methods and headers for this CORS rule."]
    pub fn allowed(&self) -> R2BucketCors_Rules_AllowedRef {
        R2BucketCors_Rules_AllowedRef::new(self.shared().clone(), format!("{}.allowed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the headers that can be exposed back, and accessed by, the JavaScript making the cross-origin request. If you need to access headers beyond the safelisted response headers, such as Content-Encoding or cf-cache-status, you must specify it here."]
    pub fn expose_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier for this rule."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age_seconds` after provisioning.\nSpecifies the amount of time (in seconds) browsers are allowed to cache CORS preflight responses. Browsers may limit this to 2 hours or less, even if the maximum value (86400) is specified."]
    pub fn max_age_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }
}
