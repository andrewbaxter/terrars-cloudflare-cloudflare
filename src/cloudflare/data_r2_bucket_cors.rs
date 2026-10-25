#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataR2BucketCorsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    bucket_name: terrars::PrimField<std::string::String>,
}

struct DataR2BucketCors_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataR2BucketCorsData_>,
}

#[derive(Clone)]
pub struct DataR2BucketCors(std::rc::Rc<DataR2BucketCors_>);

impl DataR2BucketCors {
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

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nCloudflare account ID that owns the R2 resource."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nName of the bucket."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataR2BucketCors_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl terrars::Referable for DataR2BucketCors {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataR2BucketCors { }

impl terrars::ToListMappable for DataR2BucketCors {
    type O = terrars::ListRef<DataR2BucketCorsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataR2BucketCors_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_r2_bucket_cors".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataR2BucketCors {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID that owns the R2 resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildDataR2BucketCors {
    pub fn build(self, stack: &mut terrars::Stack) -> DataR2BucketCors {
        let out = DataR2BucketCors(std::rc::Rc::new(DataR2BucketCors_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataR2BucketCorsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                bucket_name: self.bucket_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataR2BucketCorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketCorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataR2BucketCorsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataR2BucketCors_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataR2BucketCors_Rules_Allowed {}

impl DataR2BucketCors_Rules_Allowed { }

impl terrars::ToListMappable for DataR2BucketCors_Rules_Allowed {
    type O = terrars::BlockAssignable<DataR2BucketCors_Rules_Allowed>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketCors_Rules_Allowed {}

impl BuildDataR2BucketCors_Rules_Allowed {
    pub fn build(self) -> DataR2BucketCors_Rules_Allowed {
        DataR2BucketCors_Rules_Allowed {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketCors_Rules_AllowedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketCors_Rules_AllowedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataR2BucketCors_Rules_AllowedRef {
        DataR2BucketCors_Rules_AllowedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketCors_Rules_AllowedRef {
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
pub struct DataR2BucketCors_Rules {}

impl DataR2BucketCors_Rules { }

impl terrars::ToListMappable for DataR2BucketCors_Rules {
    type O = terrars::BlockAssignable<DataR2BucketCors_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketCors_Rules {}

impl BuildDataR2BucketCors_Rules {
    pub fn build(self) -> DataR2BucketCors_Rules {
        DataR2BucketCors_Rules {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketCors_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketCors_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataR2BucketCors_RulesRef {
        DataR2BucketCors_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketCors_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed` after provisioning.\nObject specifying allowed origins, methods and headers for this CORS rule."]
    pub fn allowed(&self) -> DataR2BucketCors_Rules_AllowedRef {
        DataR2BucketCors_Rules_AllowedRef::new(self.shared().clone(), format!("{}.allowed", self.base))
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
