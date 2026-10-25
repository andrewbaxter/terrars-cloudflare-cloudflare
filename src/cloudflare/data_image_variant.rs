#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataImageVariantData_ {
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
    variant_id: terrars::PrimField<std::string::String>,
}

struct DataImageVariant_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataImageVariantData_>,
}

#[derive(Clone)]
pub struct DataImageVariant(std::rc::Rc<DataImageVariant_>);

impl DataImageVariant {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variant` after provisioning.\n"]
    pub fn variant(&self) -> DataImageVariant_VariantRef {
        DataImageVariant_VariantRef::new(self.shared().clone(), format!("{}.variant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variant_id` after provisioning.\n"]
    pub fn variant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.variant_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataImageVariant {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataImageVariant { }

impl terrars::ToListMappable for DataImageVariant {
    type O = terrars::ListRef<DataImageVariantRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataImageVariant_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_image_variant".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImageVariant {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub variant_id: terrars::PrimField<std::string::String>,
}

impl BuildDataImageVariant {
    pub fn build(self, stack: &mut terrars::Stack) -> DataImageVariant {
        let out = DataImageVariant(std::rc::Rc::new(DataImageVariant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataImageVariantData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                variant_id: self.variant_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImageVariantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImageVariantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImageVariantRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variant` after provisioning.\n"]
    pub fn variant(&self) -> DataImageVariant_VariantRef {
        DataImageVariant_VariantRef::new(self.shared().clone(), format!("{}.variant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variant_id` after provisioning.\n"]
    pub fn variant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.variant_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImageVariant_Variant_Options {}

impl DataImageVariant_Variant_Options { }

impl terrars::ToListMappable for DataImageVariant_Variant_Options {
    type O = terrars::BlockAssignable<DataImageVariant_Variant_Options>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImageVariant_Variant_Options {}

impl BuildDataImageVariant_Variant_Options {
    pub fn build(self) -> DataImageVariant_Variant_Options {
        DataImageVariant_Variant_Options {}
    }
}

#[allow(dead_code)]
pub struct DataImageVariant_Variant_OptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImageVariant_Variant_OptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataImageVariant_Variant_OptionsRef {
        DataImageVariant_Variant_OptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImageVariant_Variant_OptionsRef {
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

#[derive(Serialize)]
pub struct DataImageVariant_Variant {}

impl DataImageVariant_Variant { }

impl terrars::ToListMappable for DataImageVariant_Variant {
    type O = terrars::BlockAssignable<DataImageVariant_Variant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImageVariant_Variant {}

impl BuildDataImageVariant_Variant {
    pub fn build(self) -> DataImageVariant_Variant {
        DataImageVariant_Variant {}
    }
}

#[allow(dead_code)]
pub struct DataImageVariant_VariantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImageVariant_VariantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataImageVariant_VariantRef {
        DataImageVariant_VariantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImageVariant_VariantRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `never_require_signed_urls` after provisioning.\nIndicates whether the variant can access an image without a signature, regardless of image access control."]
    pub fn never_require_signed_urls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.never_require_signed_urls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `options` after provisioning.\nAllows you to define image resizing sizes for different use cases."]
    pub fn options(&self) -> DataImageVariant_Variant_OptionsRef {
        DataImageVariant_Variant_OptionsRef::new(self.shared().clone(), format!("{}.options", self.base))
    }
}
