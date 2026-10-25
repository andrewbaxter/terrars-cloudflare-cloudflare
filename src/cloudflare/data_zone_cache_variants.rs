#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZoneCacheVariantsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZoneCacheVariants_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZoneCacheVariantsData_>,
}

#[derive(Clone)]
pub struct DataZoneCacheVariants(std::rc::Rc<DataZoneCacheVariants_>);

impl DataZoneCacheVariants {
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

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `editable` after provisioning.\nWhether the setting is editable."]
    pub fn editable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.editable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nLast time this setting was modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the zone setting."]
    pub fn value(&self) -> DataZoneCacheVariants_ValueRef {
        DataZoneCacheVariants_ValueRef::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataZoneCacheVariants {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZoneCacheVariants { }

impl terrars::ToListMappable for DataZoneCacheVariants {
    type O = terrars::ListRef<DataZoneCacheVariantsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZoneCacheVariants_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zone_cache_variants".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZoneCacheVariants {
    pub tf_id: std::string::String,
}

impl BuildDataZoneCacheVariants {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZoneCacheVariants {
        let out = DataZoneCacheVariants(std::rc::Rc::new(DataZoneCacheVariants_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZoneCacheVariantsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZoneCacheVariantsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneCacheVariantsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZoneCacheVariantsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `editable` after provisioning.\nWhether the setting is editable."]
    pub fn editable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.editable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nLast time this setting was modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the zone setting."]
    pub fn value(&self) -> DataZoneCacheVariants_ValueRef {
        DataZoneCacheVariants_ValueRef::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZoneCacheVariants_Value {}

impl DataZoneCacheVariants_Value { }

impl terrars::ToListMappable for DataZoneCacheVariants_Value {
    type O = terrars::BlockAssignable<DataZoneCacheVariants_Value>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZoneCacheVariants_Value {}

impl BuildDataZoneCacheVariants_Value {
    pub fn build(self) -> DataZoneCacheVariants_Value {
        DataZoneCacheVariants_Value {}
    }
}

#[allow(dead_code)]
pub struct DataZoneCacheVariants_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZoneCacheVariants_ValueRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataZoneCacheVariants_ValueRef {
        DataZoneCacheVariants_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZoneCacheVariants_ValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `avif` after provisioning.\nList of strings with the MIME types of all the variants that should be served for avif."]
    pub fn avif(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avif", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bmp` after provisioning.\nList of strings with the MIME types of all the variants that should be served for bmp."]
    pub fn bmp(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bmp", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gif` after provisioning.\nList of strings with the MIME types of all the variants that should be served for gif."]
    pub fn gif(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gif", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jp2` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jp2."]
    pub fn jp2(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jp2", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jpeg` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jpeg."]
    pub fn jpeg(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jpeg", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jpg` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jpg."]
    pub fn jpg(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jpg", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jpg2` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jpg2."]
    pub fn jpg2(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jpg2", self.base))
    }

    #[doc =
        "Get a reference to the value of field `png` after provisioning.\nList of strings with the MIME types of all the variants that should be served for png."]
    pub fn png(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.png", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tif` after provisioning.\nList of strings with the MIME types of all the variants that should be served for tif."]
    pub fn tif(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tif", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tiff` after provisioning.\nList of strings with the MIME types of all the variants that should be served for tiff."]
    pub fn tiff(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiff", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webp` after provisioning.\nList of strings with the MIME types of all the variants that should be served for webp."]
    pub fn webp(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webp", self.base))
    }
}
