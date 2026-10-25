#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustDlpDatasetData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    case_sensitive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding_version: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<bool>>,
}

struct ZeroTrustDlpDataset_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustDlpDatasetData_>,
}

#[derive(Clone)]
pub struct ZeroTrustDlpDataset(std::rc::Rc<ZeroTrustDlpDataset_>);

impl ZeroTrustDlpDataset {
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
        "Set the field `case_sensitive`.\nOnly applies to custom word lists.\nDetermines if the words should be matched in a case-sensitive manner\nCannot be set to false if `secret` is true or undefined"]
    pub fn set_case_sensitive(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().case_sensitive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the dataset."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `encoding_version`.\nDataset encoding version\n\nNon-secret custom word lists with no header are always version 1.\nSecret EDM lists with no header are version 1.\nMulticolumn CSV with headers are version 2.\nOmitting this field provides the default value 0, which is interpreted\nthe same as 1."]
    pub fn set_encoding_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().encoding_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret`.\nGenerate a secret dataset.\n\nIf true, the response will include a secret to use with the EDM encoder.\nIf false, the response has no secret and the dataset is uploaded in plaintext."]
    pub fn set_secret(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `case_sensitive` after provisioning.\nOnly applies to custom word lists.\nDetermines if the words should be matched in a case-sensitive manner\nCannot be set to false if `secret` is true or undefined"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<ZeroTrustDlpDataset_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> ZeroTrustDlpDataset_DatasetRef {
        ZeroTrustDlpDataset_DatasetRef::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the dataset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encoding_version` after provisioning.\nDataset encoding version\n\nNon-secret custom word lists with no header are always version 1.\nSecret EDM lists with no header are version 1.\nMulticolumn CSV with headers are version 2.\nOmitting this field provides the default value 0, which is interpreted\nthe same as 1."]
    pub fn encoding_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_cells` after provisioning.\n"]
    pub fn max_cells(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cells", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_cells` after provisioning.\n"]
    pub fn num_cells(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_cells", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nGenerate a secret dataset.\n\nIf true, the response will include a secret to use with the EDM encoder.\nIf false, the response has no secret and the dataset is uploaded in plaintext."]
    pub fn secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"empty\", \"uploading\", \"pending\", \"processing\", \"failed\", \"complete\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nStores when the dataset was last updated.\n\nThis includes name or description changes as well as uploads."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uploads` after provisioning.\n"]
    pub fn uploads(&self) -> terrars::ListRef<ZeroTrustDlpDataset_UploadsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uploads", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe version to use when uploading the dataset."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustDlpDataset {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustDlpDataset { }

impl terrars::ToListMappable for ZeroTrustDlpDataset {
    type O = terrars::ListRef<ZeroTrustDlpDatasetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustDlpDataset_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_dlp_dataset".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustDlpDataset {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustDlpDataset {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustDlpDataset {
        let out = ZeroTrustDlpDataset(std::rc::Rc::new(ZeroTrustDlpDataset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustDlpDatasetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                case_sensitive: core::default::Default::default(),
                dataset_id: core::default::Default::default(),
                description: core::default::Default::default(),
                encoding_version: core::default::Default::default(),
                name: self.name,
                secret: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustDlpDatasetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpDatasetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustDlpDatasetRef {
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

    #[doc =
        "Get a reference to the value of field `case_sensitive` after provisioning.\nOnly applies to custom word lists.\nDetermines if the words should be matched in a case-sensitive manner\nCannot be set to false if `secret` is true or undefined"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<ZeroTrustDlpDataset_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> ZeroTrustDlpDataset_DatasetRef {
        ZeroTrustDlpDataset_DatasetRef::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the dataset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encoding_version` after provisioning.\nDataset encoding version\n\nNon-secret custom word lists with no header are always version 1.\nSecret EDM lists with no header are version 1.\nMulticolumn CSV with headers are version 2.\nOmitting this field provides the default value 0, which is interpreted\nthe same as 1."]
    pub fn encoding_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_cells` after provisioning.\n"]
    pub fn max_cells(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cells", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `num_cells` after provisioning.\n"]
    pub fn num_cells(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_cells", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nGenerate a secret dataset.\n\nIf true, the response will include a secret to use with the EDM encoder.\nIf false, the response has no secret and the dataset is uploaded in plaintext."]
    pub fn secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"empty\", \"uploading\", \"pending\", \"processing\", \"failed\", \"complete\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nStores when the dataset was last updated.\n\nThis includes name or description changes as well as uploads."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uploads` after provisioning.\n"]
    pub fn uploads(&self) -> terrars::ListRef<ZeroTrustDlpDataset_UploadsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uploads", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe version to use when uploading the dataset."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpDataset_Columns {}

impl ZeroTrustDlpDataset_Columns { }

impl terrars::ToListMappable for ZeroTrustDlpDataset_Columns {
    type O = terrars::BlockAssignable<ZeroTrustDlpDataset_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpDataset_Columns {}

impl BuildZeroTrustDlpDataset_Columns {
    pub fn build(self) -> ZeroTrustDlpDataset_Columns {
        ZeroTrustDlpDataset_Columns {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpDataset_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpDataset_ColumnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpDataset_ColumnsRef {
        ZeroTrustDlpDataset_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpDataset_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\n"]
    pub fn entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_id", self.base))
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `num_cells` after provisioning.\n"]
    pub fn num_cells(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_cells", self.base))
    }

    #[doc =
        "Get a reference to the value of field `upload_status` after provisioning.\nAvailable values: \"empty\", \"uploading\", \"pending\", \"processing\", \"failed\", \"complete\"."]
    pub fn upload_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_status", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpDataset_Uploads {}

impl ZeroTrustDlpDataset_Uploads { }

impl terrars::ToListMappable for ZeroTrustDlpDataset_Uploads {
    type O = terrars::BlockAssignable<ZeroTrustDlpDataset_Uploads>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpDataset_Uploads {}

impl BuildZeroTrustDlpDataset_Uploads {
    pub fn build(self) -> ZeroTrustDlpDataset_Uploads {
        ZeroTrustDlpDataset_Uploads {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpDataset_UploadsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpDataset_UploadsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpDataset_UploadsRef {
        ZeroTrustDlpDataset_UploadsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpDataset_UploadsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `num_cells` after provisioning.\n"]
    pub fn num_cells(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_cells", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"empty\", \"uploading\", \"pending\", \"processing\", \"failed\", \"complete\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct ZeroTrustDlpDataset_Dataset {}

impl ZeroTrustDlpDataset_Dataset { }

impl terrars::ToListMappable for ZeroTrustDlpDataset_Dataset {
    type O = terrars::BlockAssignable<ZeroTrustDlpDataset_Dataset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZeroTrustDlpDataset_Dataset {}

impl BuildZeroTrustDlpDataset_Dataset {
    pub fn build(self) -> ZeroTrustDlpDataset_Dataset {
        ZeroTrustDlpDataset_Dataset {}
    }
}

#[allow(dead_code)]
pub struct ZeroTrustDlpDataset_DatasetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustDlpDataset_DatasetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZeroTrustDlpDataset_DatasetRef {
        ZeroTrustDlpDataset_DatasetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZeroTrustDlpDataset_DatasetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `case_sensitive` after provisioning.\n"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.base))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<ZeroTrustDlpDataset_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the dataset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `encoding_version` after provisioning.\n"]
    pub fn encoding_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding_version", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `num_cells` after provisioning.\n"]
    pub fn num_cells(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_cells", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"empty\", \"uploading\", \"pending\", \"processing\", \"failed\", \"complete\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nStores when the dataset was last updated.\n\nThis includes name or description changes as well as uploads."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `uploads` after provisioning.\n"]
    pub fn uploads(&self) -> terrars::ListRef<ZeroTrustDlpDataset_UploadsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uploads", self.base))
    }
}
