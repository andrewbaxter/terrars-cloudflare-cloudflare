#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneCacheVariantsData_ {
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
    value: ZoneCacheVariants_Value,
    zone_id: terrars::PrimField<std::string::String>,
}

struct ZoneCacheVariants_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZoneCacheVariantsData_>,
}

#[derive(Clone)]
pub struct ZoneCacheVariants(std::rc::Rc<ZoneCacheVariants_>);

impl ZoneCacheVariants {
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
    pub fn value(&self) -> ZoneCacheVariants_ValueRef {
        ZoneCacheVariants_ValueRef::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for ZoneCacheVariants {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZoneCacheVariants { }

impl terrars::ToListMappable for ZoneCacheVariants {
    type O = terrars::ListRef<ZoneCacheVariantsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZoneCacheVariants_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zone_cache_variants".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneCacheVariants {
    pub tf_id: std::string::String,
    #[doc = "Value of the zone setting."]
    pub value: ZoneCacheVariants_Value,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildZoneCacheVariants {
    pub fn build(self, stack: &mut terrars::Stack) -> ZoneCacheVariants {
        let out = ZoneCacheVariants(std::rc::Rc::new(ZoneCacheVariants_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZoneCacheVariantsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                value: self.value,
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneCacheVariantsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneCacheVariantsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneCacheVariantsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn value(&self) -> ZoneCacheVariants_ValueRef {
        ZoneCacheVariants_ValueRef::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZoneCacheVariants_Value {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avif: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bmp: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gif: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jp2: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jpeg: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jpg: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jpg2: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    png: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tif: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiff: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webp: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ZoneCacheVariants_Value {
    #[doc =
        "Set the field `avif`.\nList of strings with the MIME types of all the variants that should be served for avif."]
    pub fn set_avif(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.avif = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bmp`.\nList of strings with the MIME types of all the variants that should be served for bmp."]
    pub fn set_bmp(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.bmp = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gif`.\nList of strings with the MIME types of all the variants that should be served for gif."]
    pub fn set_gif(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.gif = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jp2`.\nList of strings with the MIME types of all the variants that should be served for jp2."]
    pub fn set_jp2(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jp2 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jpeg`.\nList of strings with the MIME types of all the variants that should be served for jpeg."]
    pub fn set_jpeg(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jpeg = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jpg`.\nList of strings with the MIME types of all the variants that should be served for jpg."]
    pub fn set_jpg(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jpg = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jpg2`.\nList of strings with the MIME types of all the variants that should be served for jpg2."]
    pub fn set_jpg2(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jpg2 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `png`.\nList of strings with the MIME types of all the variants that should be served for png."]
    pub fn set_png(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.png = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tif`.\nList of strings with the MIME types of all the variants that should be served for tif."]
    pub fn set_tif(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tif = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tiff`.\nList of strings with the MIME types of all the variants that should be served for tiff."]
    pub fn set_tiff(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tiff = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `webp`.\nList of strings with the MIME types of all the variants that should be served for webp."]
    pub fn set_webp(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.webp = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZoneCacheVariants_Value {
    type O = terrars::BlockAssignable<ZoneCacheVariants_Value>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneCacheVariants_Value {}

impl BuildZoneCacheVariants_Value {
    pub fn build(self) -> ZoneCacheVariants_Value {
        ZoneCacheVariants_Value {
            avif: core::default::Default::default(),
            bmp: core::default::Default::default(),
            gif: core::default::Default::default(),
            jp2: core::default::Default::default(),
            jpeg: core::default::Default::default(),
            jpg: core::default::Default::default(),
            jpg2: core::default::Default::default(),
            png: core::default::Default::default(),
            tif: core::default::Default::default(),
            tiff: core::default::Default::default(),
            webp: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ZoneCacheVariants_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneCacheVariants_ValueRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZoneCacheVariants_ValueRef {
        ZoneCacheVariants_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneCacheVariants_ValueRef {
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
