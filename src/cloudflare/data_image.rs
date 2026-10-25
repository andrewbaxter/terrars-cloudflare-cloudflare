#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataImageData_ {
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
    image_id: terrars::PrimField<std::string::String>,
}

struct DataImage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataImageData_>,
}

#[derive(Clone)]
pub struct DataImage(std::rc::Rc<DataImage_>);

impl DataImage {
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

    #[doc = "Set the field `account_id`.\nAccount identifier tag."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nCan set the creator field with an internal user ID."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filename` after provisioning.\nImage file name."]
    pub fn filename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nImage unique identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\nImage unique identifier."]
    pub fn image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nUser modifiable key-value store. Can be used for keeping references to another system of record for managing images. Metadata must not exceed 1024 bytes."]
    pub fn meta(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_signed_urls` after provisioning.\nIndicates whether the image can be a accessed only using it's UID. If set to true, a signed token needs to be generated with a signing key to view the image."]
    pub fn require_signed_urls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_signed_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uploaded` after provisioning.\nWhen the media item was uploaded."]
    pub fn uploaded(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uploaded", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `variants` after provisioning.\nObject specifying available variants for an image."]
    pub fn variants(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variants", self.extract_ref()))
    }
}

impl terrars::Referable for DataImage {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataImage { }

impl terrars::ToListMappable for DataImage {
    type O = terrars::ListRef<DataImageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataImage_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_image".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImage {
    pub tf_id: std::string::String,
    #[doc = "Image unique identifier."]
    pub image_id: terrars::PrimField<std::string::String>,
}

impl BuildDataImage {
    pub fn build(self, stack: &mut terrars::Stack) -> DataImage {
        let out = DataImage(std::rc::Rc::new(DataImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataImageData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                image_id: self.image_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImageRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nCan set the creator field with an internal user ID."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filename` after provisioning.\nImage file name."]
    pub fn filename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nImage unique identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\nImage unique identifier."]
    pub fn image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meta` after provisioning.\nUser modifiable key-value store. Can be used for keeping references to another system of record for managing images. Metadata must not exceed 1024 bytes."]
    pub fn meta(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_signed_urls` after provisioning.\nIndicates whether the image can be a accessed only using it's UID. If set to true, a signed token needs to be generated with a signing key to view the image."]
    pub fn require_signed_urls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_signed_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uploaded` after provisioning.\nWhen the media item was uploaded."]
    pub fn uploaded(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uploaded", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `variants` after provisioning.\nObject specifying available variants for an image."]
    pub fn variants(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variants", self.extract_ref()))
    }
}
