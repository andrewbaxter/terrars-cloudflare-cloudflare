#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ImageVariantData_ {
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
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    never_require_signed_urls: std::option::Option<terrars::PrimField<bool>>,
    options: ImageVariant_Options,
}

struct ImageVariant_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ImageVariantData_>,
}

#[derive(Clone)]
pub struct ImageVariant(std::rc::Rc<ImageVariant_>);

impl ImageVariant {
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
        "Set the field `never_require_signed_urls`.\nIndicates whether the variant can access an image without a signature, regardless of image access control."]
    pub fn set_never_require_signed_urls(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().never_require_signed_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `never_require_signed_urls` after provisioning.\nIndicates whether the variant can access an image without a signature, regardless of image access control."]
    pub fn never_require_signed_urls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.never_require_signed_urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `options` after provisioning.\nAllows you to define image resizing sizes for different use cases."]
    pub fn options(&self) -> ImageVariant_OptionsRef {
        ImageVariant_OptionsRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }
}

impl terrars::Referable for ImageVariant {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ImageVariant { }

impl terrars::ToListMappable for ImageVariant {
    type O = terrars::ListRef<ImageVariantRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ImageVariant_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_image_variant".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImageVariant {
    pub tf_id: std::string::String,
    #[doc = "Account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "Allows you to define image resizing sizes for different use cases."]
    pub options: ImageVariant_Options,
}

impl BuildImageVariant {
    pub fn build(self, stack: &mut terrars::Stack) -> ImageVariant {
        let out = ImageVariant(std::rc::Rc::new(ImageVariant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ImageVariantData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                id: self.id,
                never_require_signed_urls: core::default::Default::default(),
                options: self.options,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImageVariantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImageVariantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImageVariantRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `never_require_signed_urls` after provisioning.\nIndicates whether the variant can access an image without a signature, regardless of image access control."]
    pub fn never_require_signed_urls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.never_require_signed_urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `options` after provisioning.\nAllows you to define image resizing sizes for different use cases."]
    pub fn options(&self) -> ImageVariant_OptionsRef {
        ImageVariant_OptionsRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ImageVariant_Options {
    fit: terrars::PrimField<std::string::String>,
    height: terrars::PrimField<f64>,
    metadata: terrars::PrimField<std::string::String>,
    width: terrars::PrimField<f64>,
}

impl ImageVariant_Options { }

impl terrars::ToListMappable for ImageVariant_Options {
    type O = terrars::BlockAssignable<ImageVariant_Options>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImageVariant_Options {
    #[doc =
        "The fit property describes how the width and height dimensions should be interpreted.\nAvailable values: \"scale-down\", \"contain\", \"cover\", \"crop\", \"pad\"."]
    pub fit: terrars::PrimField<std::string::String>,
    #[doc = "Maximum height in image pixels."]
    pub height: terrars::PrimField<f64>,
    #[doc =
        "What EXIF data should be preserved in the output image.\nAvailable values: \"keep\", \"copyright\", \"none\"."]
    pub metadata: terrars::PrimField<std::string::String>,
    #[doc = "Maximum width in image pixels."]
    pub width: terrars::PrimField<f64>,
}

impl BuildImageVariant_Options {
    pub fn build(self) -> ImageVariant_Options {
        ImageVariant_Options {
            fit: self.fit,
            height: self.height,
            metadata: self.metadata,
            width: self.width,
        }
    }
}

#[allow(dead_code)]
pub struct ImageVariant_OptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImageVariant_OptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImageVariant_OptionsRef {
        ImageVariant_OptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImageVariant_OptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fit` after provisioning.\nThe fit property describes how the width and height dimensions should be interpreted.\nAvailable values: \"scale-down\", \"contain\", \"cover\", \"crop\", \"pad\"."]
    pub fn fit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fit", self.base))
    }

    #[doc = "Get a reference to the value of field `height` after provisioning.\nMaximum height in image pixels."]
    pub fn height(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.height", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nWhat EXIF data should be preserved in the output image.\nAvailable values: \"keep\", \"copyright\", \"none\"."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `width` after provisioning.\nMaximum width in image pixels."]
    pub fn width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.width", self.base))
    }
}
