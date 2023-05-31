use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneCacheVariantsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avif: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bmp: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jp2: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jpeg: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jpg: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jpg2: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    png: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tif: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiff: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webp: Option<SetField<PrimField<String>>>,
    zone_id: PrimField<String>,
}

struct ZoneCacheVariants_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ZoneCacheVariantsData>,
}

#[derive(Clone)]
pub struct ZoneCacheVariants(Rc<ZoneCacheVariants_>);

impl ZoneCacheVariants {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `avif`.\nList of strings with the MIME types of all the variants that should be served for avif."]
    pub fn set_avif(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().avif = Some(v.into());
        self
    }

    #[doc= "Set the field `bmp`.\nList of strings with the MIME types of all the variants that should be served for bmp."]
    pub fn set_bmp(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().bmp = Some(v.into());
        self
    }

    #[doc= "Set the field `gif`.\nList of strings with the MIME types of all the variants that should be served for gif."]
    pub fn set_gif(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().gif = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `jp2`.\nList of strings with the MIME types of all the variants that should be served for jp2."]
    pub fn set_jp2(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().jp2 = Some(v.into());
        self
    }

    #[doc= "Set the field `jpeg`.\nList of strings with the MIME types of all the variants that should be served for jpeg."]
    pub fn set_jpeg(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().jpeg = Some(v.into());
        self
    }

    #[doc= "Set the field `jpg`.\nList of strings with the MIME types of all the variants that should be served for jpg."]
    pub fn set_jpg(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().jpg = Some(v.into());
        self
    }

    #[doc= "Set the field `jpg2`.\nList of strings with the MIME types of all the variants that should be served for jpg2."]
    pub fn set_jpg2(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().jpg2 = Some(v.into());
        self
    }

    #[doc= "Set the field `png`.\nList of strings with the MIME types of all the variants that should be served for png."]
    pub fn set_png(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().png = Some(v.into());
        self
    }

    #[doc= "Set the field `tif`.\nList of strings with the MIME types of all the variants that should be served for tif."]
    pub fn set_tif(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tif = Some(v.into());
        self
    }

    #[doc= "Set the field `tiff`.\nList of strings with the MIME types of all the variants that should be served for tiff."]
    pub fn set_tiff(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tiff = Some(v.into());
        self
    }

    #[doc= "Set the field `webp`.\nList of strings with the MIME types of all the variants that should be served for webp."]
    pub fn set_webp(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().webp = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `avif` after provisioning.\nList of strings with the MIME types of all the variants that should be served for avif."]
    pub fn avif(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.avif", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bmp` after provisioning.\nList of strings with the MIME types of all the variants that should be served for bmp."]
    pub fn bmp(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.bmp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gif` after provisioning.\nList of strings with the MIME types of all the variants that should be served for gif."]
    pub fn gif(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.gif", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jp2` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jp2."]
    pub fn jp2(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.jp2", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jpeg` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jpeg."]
    pub fn jpeg(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.jpeg", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jpg` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jpg."]
    pub fn jpg(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.jpg", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jpg2` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jpg2."]
    pub fn jpg2(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.jpg2", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `png` after provisioning.\nList of strings with the MIME types of all the variants that should be served for png."]
    pub fn png(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.png", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tif` after provisioning.\nList of strings with the MIME types of all the variants that should be served for tif."]
    pub fn tif(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tif", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiff` after provisioning.\nList of strings with the MIME types of all the variants that should be served for tiff."]
    pub fn tiff(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tiff", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `webp` after provisioning.\nList of strings with the MIME types of all the variants that should be served for webp."]
    pub fn webp(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.webp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for ZoneCacheVariants {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ZoneCacheVariants { }

impl ToListMappable for ZoneCacheVariants {
    type O = ListRef<ZoneCacheVariantsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ZoneCacheVariants_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_zone_cache_variants".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneCacheVariants {
    pub tf_id: String,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildZoneCacheVariants {
    pub fn build(self, stack: &mut Stack) -> ZoneCacheVariants {
        let out = ZoneCacheVariants(Rc::new(ZoneCacheVariants_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ZoneCacheVariantsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                avif: core::default::Default::default(),
                bmp: core::default::Default::default(),
                gif: core::default::Default::default(),
                id: core::default::Default::default(),
                jp2: core::default::Default::default(),
                jpeg: core::default::Default::default(),
                jpg: core::default::Default::default(),
                jpg2: core::default::Default::default(),
                png: core::default::Default::default(),
                tif: core::default::Default::default(),
                tiff: core::default::Default::default(),
                webp: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneCacheVariantsRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneCacheVariantsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneCacheVariantsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `avif` after provisioning.\nList of strings with the MIME types of all the variants that should be served for avif."]
    pub fn avif(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.avif", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bmp` after provisioning.\nList of strings with the MIME types of all the variants that should be served for bmp."]
    pub fn bmp(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.bmp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gif` after provisioning.\nList of strings with the MIME types of all the variants that should be served for gif."]
    pub fn gif(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.gif", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jp2` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jp2."]
    pub fn jp2(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.jp2", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jpeg` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jpeg."]
    pub fn jpeg(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.jpeg", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jpg` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jpg."]
    pub fn jpg(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.jpg", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jpg2` after provisioning.\nList of strings with the MIME types of all the variants that should be served for jpg2."]
    pub fn jpg2(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.jpg2", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `png` after provisioning.\nList of strings with the MIME types of all the variants that should be served for png."]
    pub fn png(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.png", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tif` after provisioning.\nList of strings with the MIME types of all the variants that should be served for tif."]
    pub fn tif(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tif", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiff` after provisioning.\nList of strings with the MIME types of all the variants that should be served for tiff."]
    pub fn tiff(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tiff", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `webp` after provisioning.\nList of strings with the MIME types of all the variants that should be served for webp."]
    pub fn webp(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.webp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
