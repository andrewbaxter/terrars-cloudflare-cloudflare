#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct R2BucketSippyData_ {
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
    destination: std::option::Option<R2BucketSippy_Destination>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jurisdiction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<R2BucketSippy_Source>,
}

struct R2BucketSippy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<R2BucketSippyData_>,
}

#[derive(Clone)]
pub struct R2BucketSippy(std::rc::Rc<R2BucketSippy_>);

impl R2BucketSippy {
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

    #[doc = "Set the field `destination`.\nR2 bucket to copy objects to."]
    pub fn set_destination(self, v: impl std::convert::Into<R2BucketSippy_Destination>) -> Self {
        self.0.data.borrow_mut().destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jurisdiction`.\nJurisdiction of the bucket"]
    pub fn set_jurisdiction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().jurisdiction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\nAWS S3 bucket to copy objects from."]
    pub fn set_source(self, v: impl std::convert::Into<R2BucketSippy_Source>) -> Self {
        self.0.data.borrow_mut().source = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `destination` after provisioning.\nR2 bucket to copy objects to."]
    pub fn destination(&self) -> R2BucketSippy_DestinationRef {
        R2BucketSippy_DestinationRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nState of Sippy for this bucket."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jurisdiction` after provisioning.\nJurisdiction of the bucket"]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\nAWS S3 bucket to copy objects from."]
    pub fn source(&self) -> R2BucketSippy_SourceRef {
        R2BucketSippy_SourceRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl terrars::Referable for R2BucketSippy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for R2BucketSippy { }

impl terrars::ToListMappable for R2BucketSippy {
    type O = terrars::ListRef<R2BucketSippyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for R2BucketSippy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_r2_bucket_sippy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildR2BucketSippy {
    pub tf_id: std::string::String,
    #[doc = "Cloudflare account ID that owns the R2 resource."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildR2BucketSippy {
    pub fn build(self, stack: &mut terrars::Stack) -> R2BucketSippy {
        let out = R2BucketSippy(std::rc::Rc::new(R2BucketSippy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(R2BucketSippyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                bucket_name: self.bucket_name,
                destination: core::default::Default::default(),
                jurisdiction: core::default::Default::default(),
                source: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct R2BucketSippyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketSippyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl R2BucketSippyRef {
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

    #[doc = "Get a reference to the value of field `destination` after provisioning.\nR2 bucket to copy objects to."]
    pub fn destination(&self) -> R2BucketSippy_DestinationRef {
        R2BucketSippy_DestinationRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nState of Sippy for this bucket."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jurisdiction` after provisioning.\nJurisdiction of the bucket"]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\nAWS S3 bucket to copy objects from."]
    pub fn source(&self) -> R2BucketSippy_SourceRef {
        R2BucketSippy_SourceRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct R2BucketSippy_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl R2BucketSippy_Destination {
    #[doc =
        "Set the field `access_key_id`.\nID of a Cloudflare API token.\nThis is the value labelled \"Access Key ID\" when creating an API.\ntoken from the [R2 dashboard](https://dash.cloudflare.com/?to=/:account/r2/api-tokens).\n\nSippy will use this token when writing objects to R2, so it is\nbest to scope this token to the bucket you're enabling Sippy for."]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_provider`.\nAvailable values: \"r2\"."]
    pub fn set_cloud_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cloud_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_access_key`.\nValue of a Cloudflare API token.\nThis is the value labelled \"Secret Access Key\" when creating an API.\ntoken from the [R2 dashboard](https://dash.cloudflare.com/?to=/:account/r2/api-tokens).\n\nSippy will use this token when writing objects to R2, so it is\nbest to scope this token to the bucket you're enabling Sippy for."]
    pub fn set_secret_access_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_access_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for R2BucketSippy_Destination {
    type O = terrars::BlockAssignable<R2BucketSippy_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketSippy_Destination {}

impl BuildR2BucketSippy_Destination {
    pub fn build(self) -> R2BucketSippy_Destination {
        R2BucketSippy_Destination {
            access_key_id: core::default::Default::default(),
            cloud_provider: core::default::Default::default(),
            secret_access_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct R2BucketSippy_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketSippy_DestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> R2BucketSippy_DestinationRef {
        R2BucketSippy_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketSippy_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nID of a Cloudflare API token.\nThis is the value labelled \"Access Key ID\" when creating an API.\ntoken from the [R2 dashboard](https://dash.cloudflare.com/?to=/:account/r2/api-tokens).\n\nSippy will use this token when writing objects to R2, so it is\nbest to scope this token to the bucket you're enabling Sippy for."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_provider` after provisioning.\nAvailable values: \"r2\"."]
    pub fn cloud_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_provider", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key` after provisioning.\nValue of a Cloudflare API token.\nThis is the value labelled \"Secret Access Key\" when creating an API.\ntoken from the [R2 dashboard](https://dash.cloudflare.com/?to=/:account/r2/api-tokens).\n\nSippy will use this token when writing objects to R2, so it is\nbest to scope this token to the bucket you're enabling Sippy for."]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }
}

#[derive(Serialize)]
pub struct R2BucketSippy_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sas_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl R2BucketSippy_Source {
    #[doc =
        "Set the field `access_key_id`.\nAccess Key ID of an IAM credential (ideally scoped to a single S3 bucket)."]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `account_key`.\nAccess key for the Azure Storage account. Mutually exclusive with sasToken."]
    pub fn set_account_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `account_name`.\nName of the Azure Storage account."]
    pub fn set_account_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket`.\nName of the AWS S3 bucket."]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_url`.\nURL to the S3-compatible API of the bucket."]
    pub fn set_bucket_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_email`.\nClient email of an IAM credential (ideally scoped to a single GCS bucket)."]
    pub fn set_client_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_provider`.\nAvailable values: \"aws\", \"gcs\", \"s3\", \"azure\"."]
    pub fn set_cloud_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cloud_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container`.\nName of the Azure Blob Storage container."]
    pub fn set_container(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\nPrivate Key of an IAM credential (ideally scoped to a single GCS bucket)."]
    pub fn set_private_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nAWS region containing the source S3 bucket."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sas_token`.\nShared Access Signature token for the Azure Storage account. Mutually exclusive with accountKey."]
    pub fn set_sas_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sas_token = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_access_key`.\nSecret Access Key of an IAM credential (ideally scoped to a single S3 bucket)."]
    pub fn set_secret_access_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_access_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for R2BucketSippy_Source {
    type O = terrars::BlockAssignable<R2BucketSippy_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildR2BucketSippy_Source {}

impl BuildR2BucketSippy_Source {
    pub fn build(self) -> R2BucketSippy_Source {
        R2BucketSippy_Source {
            access_key_id: core::default::Default::default(),
            account_key: core::default::Default::default(),
            account_name: core::default::Default::default(),
            bucket: core::default::Default::default(),
            bucket_url: core::default::Default::default(),
            client_email: core::default::Default::default(),
            cloud_provider: core::default::Default::default(),
            container: core::default::Default::default(),
            private_key: core::default::Default::default(),
            region: core::default::Default::default(),
            sas_token: core::default::Default::default(),
            secret_access_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct R2BucketSippy_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for R2BucketSippy_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> R2BucketSippy_SourceRef {
        R2BucketSippy_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl R2BucketSippy_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nAccess Key ID of an IAM credential (ideally scoped to a single S3 bucket)."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `account_key` after provisioning.\nAccess key for the Azure Storage account. Mutually exclusive with sasToken."]
    pub fn account_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `account_name` after provisioning.\nName of the Azure Storage account."]
    pub fn account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nName of the AWS S3 bucket."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bucket_url` after provisioning.\nURL to the S3-compatible API of the bucket."]
    pub fn bucket_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_email` after provisioning.\nClient email of an IAM credential (ideally scoped to a single GCS bucket)."]
    pub fn client_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cloud_provider` after provisioning.\nAvailable values: \"aws\", \"gcs\", \"s3\", \"azure\"."]
    pub fn cloud_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_provider", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container` after provisioning.\nName of the Azure Blob Storage container."]
    pub fn container(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key` after provisioning.\nPrivate Key of an IAM credential (ideally scoped to a single GCS bucket)."]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nAWS region containing the source S3 bucket."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sas_token` after provisioning.\nShared Access Signature token for the Azure Storage account. Mutually exclusive with accountKey."]
    pub fn sas_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sas_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key` after provisioning.\nSecret Access Key of an IAM credential (ideally scoped to a single S3 bucket)."]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }
}
