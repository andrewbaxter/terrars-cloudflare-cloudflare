#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataR2BucketSippyData_ {
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

struct DataR2BucketSippy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataR2BucketSippyData_>,
}

#[derive(Clone)]
pub struct DataR2BucketSippy(std::rc::Rc<DataR2BucketSippy_>);

impl DataR2BucketSippy {
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

    #[doc =
        "Get a reference to the value of field `destination` after provisioning.\nDetails about the configured destination bucket."]
    pub fn destination(&self) -> DataR2BucketSippy_DestinationRef {
        DataR2BucketSippy_DestinationRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nState of Sippy for this bucket."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nDetails about the configured source bucket."]
    pub fn source(&self) -> DataR2BucketSippy_SourceRef {
        DataR2BucketSippy_SourceRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl terrars::Referable for DataR2BucketSippy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataR2BucketSippy { }

impl terrars::ToListMappable for DataR2BucketSippy {
    type O = terrars::ListRef<DataR2BucketSippyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataR2BucketSippy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_r2_bucket_sippy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataR2BucketSippy {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID that owns the R2 resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildDataR2BucketSippy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataR2BucketSippy {
        let out = DataR2BucketSippy(std::rc::Rc::new(DataR2BucketSippy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataR2BucketSippyData_ {
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

pub struct DataR2BucketSippyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketSippyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataR2BucketSippyRef {
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

    #[doc =
        "Get a reference to the value of field `destination` after provisioning.\nDetails about the configured destination bucket."]
    pub fn destination(&self) -> DataR2BucketSippy_DestinationRef {
        DataR2BucketSippy_DestinationRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nState of Sippy for this bucket."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nDetails about the configured source bucket."]
    pub fn source(&self) -> DataR2BucketSippy_SourceRef {
        DataR2BucketSippy_SourceRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataR2BucketSippy_Destination {}

impl DataR2BucketSippy_Destination { }

impl terrars::ToListMappable for DataR2BucketSippy_Destination {
    type O = terrars::BlockAssignable<DataR2BucketSippy_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketSippy_Destination {}

impl BuildDataR2BucketSippy_Destination {
    pub fn build(self) -> DataR2BucketSippy_Destination {
        DataR2BucketSippy_Destination {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketSippy_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketSippy_DestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataR2BucketSippy_DestinationRef {
        DataR2BucketSippy_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketSippy_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nID of the Cloudflare API token used when writing objects to this\nbucket."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nName of the bucket on the provider."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `r2_bucket_sippy_provider` after provisioning.\nAvailable values: \"r2\"."]
    pub fn r2_bucket_sippy_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.r2_bucket_sippy_provider", self.base))
    }
}

#[derive(Serialize)]
pub struct DataR2BucketSippy_Source {}

impl DataR2BucketSippy_Source { }

impl terrars::ToListMappable for DataR2BucketSippy_Source {
    type O = terrars::BlockAssignable<DataR2BucketSippy_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2BucketSippy_Source {}

impl BuildDataR2BucketSippy_Source {
    pub fn build(self) -> DataR2BucketSippy_Source {
        DataR2BucketSippy_Source {}
    }
}

#[allow(dead_code)]
pub struct DataR2BucketSippy_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2BucketSippy_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataR2BucketSippy_SourceRef {
        DataR2BucketSippy_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2BucketSippy_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nName of the bucket on the provider (AWS, GCS only)."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bucket_url` after provisioning.\nS3-compatible URL (Generic S3-compatible providers only)."]
    pub fn bucket_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container` after provisioning.\nName of the Azure Blob Storage container (Azure only)."]
    pub fn container(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container", self.base))
    }

    #[doc =
        "Get a reference to the value of field `r2_bucket_sippy_provider` after provisioning.\nAvailable values: \"aws\", \"gcs\", \"s3\", \"azure\"."]
    pub fn r2_bucket_sippy_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.r2_bucket_sippy_provider", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where the bucket resides (AWS only)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
