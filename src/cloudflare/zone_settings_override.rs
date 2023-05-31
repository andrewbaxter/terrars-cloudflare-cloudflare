use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneSettingsOverrideData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<Vec<ZoneSettingsOverrideSettingsEl>>,
    dynamic: ZoneSettingsOverrideDynamic,
}

struct ZoneSettingsOverride_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ZoneSettingsOverrideData>,
}

#[derive(Clone)]
pub struct ZoneSettingsOverride(Rc<ZoneSettingsOverride_>);

impl ZoneSettingsOverride {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `settings`.\n"]
    pub fn set_settings(self, v: impl Into<BlockAssignable<ZoneSettingsOverrideSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `initial_settings` after provisioning.\n"]
    pub fn initial_settings(&self) -> ListRef<ZoneSettingsOverrideInitialSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.initial_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `initial_settings_read_at` after provisioning.\n"]
    pub fn initial_settings_read_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_settings_read_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `readonly_settings` after provisioning.\n"]
    pub fn readonly_settings(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.readonly_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_status` after provisioning.\n"]
    pub fn zone_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_type` after provisioning.\n"]
    pub fn zone_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<ZoneSettingsOverrideSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl Referable for ZoneSettingsOverride {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ZoneSettingsOverride { }

impl ToListMappable for ZoneSettingsOverride {
    type O = ListRef<ZoneSettingsOverrideRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ZoneSettingsOverride_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_zone_settings_override".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneSettingsOverride {
    pub tf_id: String,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildZoneSettingsOverride {
    pub fn build(self, stack: &mut Stack) -> ZoneSettingsOverride {
        let out = ZoneSettingsOverride(Rc::new(ZoneSettingsOverride_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ZoneSettingsOverrideData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                zone_id: self.zone_id,
                settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneSettingsOverrideRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneSettingsOverrideRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneSettingsOverrideRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `initial_settings` after provisioning.\n"]
    pub fn initial_settings(&self) -> ListRef<ZoneSettingsOverrideInitialSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.initial_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `initial_settings_read_at` after provisioning.\n"]
    pub fn initial_settings_read_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_settings_read_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `readonly_settings` after provisioning.\n"]
    pub fn readonly_settings(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.readonly_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_status` after provisioning.\n"]
    pub fn zone_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_type` after provisioning.\n"]
    pub fn zone_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<ZoneSettingsOverrideSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideInitialSettingsElMinifyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    css: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    js: Option<PrimField<String>>,
}

impl ZoneSettingsOverrideInitialSettingsElMinifyEl {
    #[doc= "Set the field `css`.\n"]
    pub fn set_css(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.css = Some(v.into());
        self
    }

    #[doc= "Set the field `html`.\n"]
    pub fn set_html(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.html = Some(v.into());
        self
    }

    #[doc= "Set the field `js`.\n"]
    pub fn set_js(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.js = Some(v.into());
        self
    }
}

impl ToListMappable for ZoneSettingsOverrideInitialSettingsElMinifyEl {
    type O = BlockAssignable<ZoneSettingsOverrideInitialSettingsElMinifyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSettingsOverrideInitialSettingsElMinifyEl {}

impl BuildZoneSettingsOverrideInitialSettingsElMinifyEl {
    pub fn build(self) -> ZoneSettingsOverrideInitialSettingsElMinifyEl {
        ZoneSettingsOverrideInitialSettingsElMinifyEl {
            css: core::default::Default::default(),
            html: core::default::Default::default(),
            js: core::default::Default::default(),
        }
    }
}

pub struct ZoneSettingsOverrideInitialSettingsElMinifyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneSettingsOverrideInitialSettingsElMinifyElRef {
    fn new(shared: StackShared, base: String) -> ZoneSettingsOverrideInitialSettingsElMinifyElRef {
        ZoneSettingsOverrideInitialSettingsElMinifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideInitialSettingsElMinifyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `css` after provisioning.\n"]
    pub fn css(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.css", self.base))
    }

    #[doc= "Get a reference to the value of field `html` after provisioning.\n"]
    pub fn html(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }

    #[doc= "Get a reference to the value of field `js` after provisioning.\n"]
    pub fn js(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.js", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideInitialSettingsElMobileRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile_subdomain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strip_uri: Option<PrimField<bool>>,
}

impl ZoneSettingsOverrideInitialSettingsElMobileRedirectEl {
    #[doc= "Set the field `mobile_subdomain`.\n"]
    pub fn set_mobile_subdomain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mobile_subdomain = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc= "Set the field `strip_uri`.\n"]
    pub fn set_strip_uri(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strip_uri = Some(v.into());
        self
    }
}

impl ToListMappable for ZoneSettingsOverrideInitialSettingsElMobileRedirectEl {
    type O = BlockAssignable<ZoneSettingsOverrideInitialSettingsElMobileRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSettingsOverrideInitialSettingsElMobileRedirectEl {}

impl BuildZoneSettingsOverrideInitialSettingsElMobileRedirectEl {
    pub fn build(self) -> ZoneSettingsOverrideInitialSettingsElMobileRedirectEl {
        ZoneSettingsOverrideInitialSettingsElMobileRedirectEl {
            mobile_subdomain: core::default::Default::default(),
            status: core::default::Default::default(),
            strip_uri: core::default::Default::default(),
        }
    }
}

pub struct ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef {
    fn new(shared: StackShared, base: String) -> ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef {
        ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mobile_subdomain` after provisioning.\n"]
    pub fn mobile_subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mobile_subdomain", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc= "Get a reference to the value of field `strip_uri` after provisioning.\n"]
    pub fn strip_uri(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_subdomains: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nosniff: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preload: Option<PrimField<bool>>,
}

impl ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `include_subdomains`.\n"]
    pub fn set_include_subdomains(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_subdomains = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }

    #[doc= "Set the field `nosniff`.\n"]
    pub fn set_nosniff(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nosniff = Some(v.into());
        self
    }

    #[doc= "Set the field `preload`.\n"]
    pub fn set_preload(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preload = Some(v.into());
        self
    }
}

impl ToListMappable for ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl {
    type O = BlockAssignable<ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSettingsOverrideInitialSettingsElSecurityHeaderEl {}

impl BuildZoneSettingsOverrideInitialSettingsElSecurityHeaderEl {
    pub fn build(self) -> ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl {
        ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl {
            enabled: core::default::Default::default(),
            include_subdomains: core::default::Default::default(),
            max_age: core::default::Default::default(),
            nosniff: core::default::Default::default(),
            preload: core::default::Default::default(),
        }
    }
}

pub struct ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef {
    fn new(shared: StackShared, base: String) -> ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef {
        ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `include_subdomains` after provisioning.\n"]
    pub fn include_subdomains(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc= "Get a reference to the value of field `nosniff` after provisioning.\n"]
    pub fn nosniff(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nosniff", self.base))
    }

    #[doc= "Get a reference to the value of field `preload` after provisioning.\n"]
    pub fn preload(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preload", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideInitialSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    always_online: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    always_use_https: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_https_rewrites: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_ast: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brotli: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_cache_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_check: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    challenge_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ciphers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cname_flattening: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    development_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    early_hints: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_obfuscation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_logs_to_cloudflare: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    h2_prioritization: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hotlink_protection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http3: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_resizing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_geolocation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_to_cloudflare: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_upload: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_tls_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minify: Option<ListField<ZoneSettingsOverrideInitialSettingsElMinifyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile_redirect: Option<ListField<ZoneSettingsOverrideInitialSettingsElMobileRedirectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opportunistic_encryption: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opportunistic_onion: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orange_to_orange: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_error_page_pass_thru: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_max_http_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polish: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefetch_preload: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy_pass: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_read_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudo_ipv4: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_buffering: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rocket_loader: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_header: Option<ListField<ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_exclude: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_query_string_for_cache: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_1_2_only: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_1_3: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_client_auth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    true_client_ip_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    universal_ssl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visitor_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    waf: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    websockets: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_rtt: Option<PrimField<String>>,
}

impl ZoneSettingsOverrideInitialSettingsEl {
    #[doc= "Set the field `always_online`.\n"]
    pub fn set_always_online(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.always_online = Some(v.into());
        self
    }

    #[doc= "Set the field `always_use_https`.\n"]
    pub fn set_always_use_https(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.always_use_https = Some(v.into());
        self
    }

    #[doc= "Set the field `automatic_https_rewrites`.\n"]
    pub fn set_automatic_https_rewrites(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.automatic_https_rewrites = Some(v.into());
        self
    }

    #[doc= "Set the field `binary_ast`.\n"]
    pub fn set_binary_ast(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.binary_ast = Some(v.into());
        self
    }

    #[doc= "Set the field `brotli`.\n"]
    pub fn set_brotli(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.brotli = Some(v.into());
        self
    }

    #[doc= "Set the field `browser_cache_ttl`.\n"]
    pub fn set_browser_cache_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.browser_cache_ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `browser_check`.\n"]
    pub fn set_browser_check(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.browser_check = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_level`.\n"]
    pub fn set_cache_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_level = Some(v.into());
        self
    }

    #[doc= "Set the field `challenge_ttl`.\n"]
    pub fn set_challenge_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.challenge_ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `ciphers`.\n"]
    pub fn set_ciphers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ciphers = Some(v.into());
        self
    }

    #[doc= "Set the field `cname_flattening`.\n"]
    pub fn set_cname_flattening(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cname_flattening = Some(v.into());
        self
    }

    #[doc= "Set the field `development_mode`.\n"]
    pub fn set_development_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.development_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `early_hints`.\n"]
    pub fn set_early_hints(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.early_hints = Some(v.into());
        self
    }

    #[doc= "Set the field `email_obfuscation`.\n"]
    pub fn set_email_obfuscation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_obfuscation = Some(v.into());
        self
    }

    #[doc= "Set the field `filter_logs_to_cloudflare`.\n"]
    pub fn set_filter_logs_to_cloudflare(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_logs_to_cloudflare = Some(v.into());
        self
    }

    #[doc= "Set the field `h2_prioritization`.\n"]
    pub fn set_h2_prioritization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.h2_prioritization = Some(v.into());
        self
    }

    #[doc= "Set the field `hotlink_protection`.\n"]
    pub fn set_hotlink_protection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hotlink_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `http2`.\n"]
    pub fn set_http2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http2 = Some(v.into());
        self
    }

    #[doc= "Set the field `http3`.\n"]
    pub fn set_http3(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http3 = Some(v.into());
        self
    }

    #[doc= "Set the field `image_resizing`.\n"]
    pub fn set_image_resizing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_resizing = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_geolocation`.\n"]
    pub fn set_ip_geolocation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_geolocation = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6`.\n"]
    pub fn set_ipv6(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6 = Some(v.into());
        self
    }

    #[doc= "Set the field `log_to_cloudflare`.\n"]
    pub fn set_log_to_cloudflare(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_to_cloudflare = Some(v.into());
        self
    }

    #[doc= "Set the field `max_upload`.\n"]
    pub fn set_max_upload(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_upload = Some(v.into());
        self
    }

    #[doc= "Set the field `min_tls_version`.\n"]
    pub fn set_min_tls_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_tls_version = Some(v.into());
        self
    }

    #[doc= "Set the field `minify`.\n"]
    pub fn set_minify(mut self, v: impl Into<ListField<ZoneSettingsOverrideInitialSettingsElMinifyEl>>) -> Self {
        self.minify = Some(v.into());
        self
    }

    #[doc= "Set the field `mirage`.\n"]
    pub fn set_mirage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mirage = Some(v.into());
        self
    }

    #[doc= "Set the field `mobile_redirect`.\n"]
    pub fn set_mobile_redirect(
        mut self,
        v: impl Into<ListField<ZoneSettingsOverrideInitialSettingsElMobileRedirectEl>>,
    ) -> Self {
        self.mobile_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `opportunistic_encryption`.\n"]
    pub fn set_opportunistic_encryption(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.opportunistic_encryption = Some(v.into());
        self
    }

    #[doc= "Set the field `opportunistic_onion`.\n"]
    pub fn set_opportunistic_onion(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.opportunistic_onion = Some(v.into());
        self
    }

    #[doc= "Set the field `orange_to_orange`.\n"]
    pub fn set_orange_to_orange(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.orange_to_orange = Some(v.into());
        self
    }

    #[doc= "Set the field `origin_error_page_pass_thru`.\n"]
    pub fn set_origin_error_page_pass_thru(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_error_page_pass_thru = Some(v.into());
        self
    }

    #[doc= "Set the field `origin_max_http_version`.\n"]
    pub fn set_origin_max_http_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_max_http_version = Some(v.into());
        self
    }

    #[doc= "Set the field `polish`.\n"]
    pub fn set_polish(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.polish = Some(v.into());
        self
    }

    #[doc= "Set the field `prefetch_preload`.\n"]
    pub fn set_prefetch_preload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefetch_preload = Some(v.into());
        self
    }

    #[doc= "Set the field `privacy_pass`.\n"]
    pub fn set_privacy_pass(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.privacy_pass = Some(v.into());
        self
    }

    #[doc= "Set the field `proxy_read_timeout`.\n"]
    pub fn set_proxy_read_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_read_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `pseudo_ipv4`.\n"]
    pub fn set_pseudo_ipv4(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pseudo_ipv4 = Some(v.into());
        self
    }

    #[doc= "Set the field `response_buffering`.\n"]
    pub fn set_response_buffering(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_buffering = Some(v.into());
        self
    }

    #[doc= "Set the field `rocket_loader`.\n"]
    pub fn set_rocket_loader(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rocket_loader = Some(v.into());
        self
    }

    #[doc= "Set the field `security_header`.\n"]
    pub fn set_security_header(
        mut self,
        v: impl Into<ListField<ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl>>,
    ) -> Self {
        self.security_header = Some(v.into());
        self
    }

    #[doc= "Set the field `security_level`.\n"]
    pub fn set_security_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_level = Some(v.into());
        self
    }

    #[doc= "Set the field `server_side_exclude`.\n"]
    pub fn set_server_side_exclude(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_side_exclude = Some(v.into());
        self
    }

    #[doc= "Set the field `sort_query_string_for_cache`.\n"]
    pub fn set_sort_query_string_for_cache(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sort_query_string_for_cache = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl`.\n"]
    pub fn set_ssl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl = Some(v.into());
        self
    }

    #[doc= "Set the field `tls_1_2_only`.\n"]
    pub fn set_tls_1_2_only(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_1_2_only = Some(v.into());
        self
    }

    #[doc= "Set the field `tls_1_3`.\n"]
    pub fn set_tls_1_3(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_1_3 = Some(v.into());
        self
    }

    #[doc= "Set the field `tls_client_auth`.\n"]
    pub fn set_tls_client_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_client_auth = Some(v.into());
        self
    }

    #[doc= "Set the field `true_client_ip_header`.\n"]
    pub fn set_true_client_ip_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.true_client_ip_header = Some(v.into());
        self
    }

    #[doc= "Set the field `universal_ssl`.\n"]
    pub fn set_universal_ssl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.universal_ssl = Some(v.into());
        self
    }

    #[doc= "Set the field `visitor_ip`.\n"]
    pub fn set_visitor_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visitor_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `waf`.\n"]
    pub fn set_waf(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.waf = Some(v.into());
        self
    }

    #[doc= "Set the field `webp`.\n"]
    pub fn set_webp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webp = Some(v.into());
        self
    }

    #[doc= "Set the field `websockets`.\n"]
    pub fn set_websockets(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.websockets = Some(v.into());
        self
    }

    #[doc= "Set the field `zero_rtt`.\n"]
    pub fn set_zero_rtt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zero_rtt = Some(v.into());
        self
    }
}

impl ToListMappable for ZoneSettingsOverrideInitialSettingsEl {
    type O = BlockAssignable<ZoneSettingsOverrideInitialSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSettingsOverrideInitialSettingsEl {}

impl BuildZoneSettingsOverrideInitialSettingsEl {
    pub fn build(self) -> ZoneSettingsOverrideInitialSettingsEl {
        ZoneSettingsOverrideInitialSettingsEl {
            always_online: core::default::Default::default(),
            always_use_https: core::default::Default::default(),
            automatic_https_rewrites: core::default::Default::default(),
            binary_ast: core::default::Default::default(),
            brotli: core::default::Default::default(),
            browser_cache_ttl: core::default::Default::default(),
            browser_check: core::default::Default::default(),
            cache_level: core::default::Default::default(),
            challenge_ttl: core::default::Default::default(),
            ciphers: core::default::Default::default(),
            cname_flattening: core::default::Default::default(),
            development_mode: core::default::Default::default(),
            early_hints: core::default::Default::default(),
            email_obfuscation: core::default::Default::default(),
            filter_logs_to_cloudflare: core::default::Default::default(),
            h2_prioritization: core::default::Default::default(),
            hotlink_protection: core::default::Default::default(),
            http2: core::default::Default::default(),
            http3: core::default::Default::default(),
            image_resizing: core::default::Default::default(),
            ip_geolocation: core::default::Default::default(),
            ipv6: core::default::Default::default(),
            log_to_cloudflare: core::default::Default::default(),
            max_upload: core::default::Default::default(),
            min_tls_version: core::default::Default::default(),
            minify: core::default::Default::default(),
            mirage: core::default::Default::default(),
            mobile_redirect: core::default::Default::default(),
            opportunistic_encryption: core::default::Default::default(),
            opportunistic_onion: core::default::Default::default(),
            orange_to_orange: core::default::Default::default(),
            origin_error_page_pass_thru: core::default::Default::default(),
            origin_max_http_version: core::default::Default::default(),
            polish: core::default::Default::default(),
            prefetch_preload: core::default::Default::default(),
            privacy_pass: core::default::Default::default(),
            proxy_read_timeout: core::default::Default::default(),
            pseudo_ipv4: core::default::Default::default(),
            response_buffering: core::default::Default::default(),
            rocket_loader: core::default::Default::default(),
            security_header: core::default::Default::default(),
            security_level: core::default::Default::default(),
            server_side_exclude: core::default::Default::default(),
            sort_query_string_for_cache: core::default::Default::default(),
            ssl: core::default::Default::default(),
            tls_1_2_only: core::default::Default::default(),
            tls_1_3: core::default::Default::default(),
            tls_client_auth: core::default::Default::default(),
            true_client_ip_header: core::default::Default::default(),
            universal_ssl: core::default::Default::default(),
            visitor_ip: core::default::Default::default(),
            waf: core::default::Default::default(),
            webp: core::default::Default::default(),
            websockets: core::default::Default::default(),
            zero_rtt: core::default::Default::default(),
        }
    }
}

pub struct ZoneSettingsOverrideInitialSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneSettingsOverrideInitialSettingsElRef {
    fn new(shared: StackShared, base: String) -> ZoneSettingsOverrideInitialSettingsElRef {
        ZoneSettingsOverrideInitialSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideInitialSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `always_online` after provisioning.\n"]
    pub fn always_online(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.always_online", self.base))
    }

    #[doc= "Get a reference to the value of field `always_use_https` after provisioning.\n"]
    pub fn always_use_https(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.always_use_https", self.base))
    }

    #[doc= "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\n"]
    pub fn automatic_https_rewrites(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc= "Get a reference to the value of field `binary_ast` after provisioning.\n"]
    pub fn binary_ast(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.binary_ast", self.base))
    }

    #[doc= "Get a reference to the value of field `brotli` after provisioning.\n"]
    pub fn brotli(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.brotli", self.base))
    }

    #[doc= "Get a reference to the value of field `browser_cache_ttl` after provisioning.\n"]
    pub fn browser_cache_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_cache_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `browser_check` after provisioning.\n"]
    pub fn browser_check(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_check", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_level` after provisioning.\n"]
    pub fn cache_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_level", self.base))
    }

    #[doc= "Get a reference to the value of field `challenge_ttl` after provisioning.\n"]
    pub fn challenge_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.challenge_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `ciphers` after provisioning.\n"]
    pub fn ciphers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ciphers", self.base))
    }

    #[doc= "Get a reference to the value of field `cname_flattening` after provisioning.\n"]
    pub fn cname_flattening(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cname_flattening", self.base))
    }

    #[doc= "Get a reference to the value of field `development_mode` after provisioning.\n"]
    pub fn development_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.development_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `early_hints` after provisioning.\n"]
    pub fn early_hints(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.early_hints", self.base))
    }

    #[doc= "Get a reference to the value of field `email_obfuscation` after provisioning.\n"]
    pub fn email_obfuscation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc= "Get a reference to the value of field `filter_logs_to_cloudflare` after provisioning.\n"]
    pub fn filter_logs_to_cloudflare(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_logs_to_cloudflare", self.base))
    }

    #[doc= "Get a reference to the value of field `h2_prioritization` after provisioning.\n"]
    pub fn h2_prioritization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.h2_prioritization", self.base))
    }

    #[doc= "Get a reference to the value of field `hotlink_protection` after provisioning.\n"]
    pub fn hotlink_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hotlink_protection", self.base))
    }

    #[doc= "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc= "Get a reference to the value of field `http3` after provisioning.\n"]
    pub fn http3(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http3", self.base))
    }

    #[doc= "Get a reference to the value of field `image_resizing` after provisioning.\n"]
    pub fn image_resizing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_resizing", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_geolocation` after provisioning.\n"]
    pub fn ip_geolocation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_geolocation", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }

    #[doc= "Get a reference to the value of field `log_to_cloudflare` after provisioning.\n"]
    pub fn log_to_cloudflare(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_to_cloudflare", self.base))
    }

    #[doc= "Get a reference to the value of field `max_upload` after provisioning.\n"]
    pub fn max_upload(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_upload", self.base))
    }

    #[doc= "Get a reference to the value of field `min_tls_version` after provisioning.\n"]
    pub fn min_tls_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_tls_version", self.base))
    }

    #[doc= "Get a reference to the value of field `minify` after provisioning.\n"]
    pub fn minify(&self) -> ListRef<ZoneSettingsOverrideInitialSettingsElMinifyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.minify", self.base))
    }

    #[doc= "Get a reference to the value of field `mirage` after provisioning.\n"]
    pub fn mirage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc= "Get a reference to the value of field `mobile_redirect` after provisioning.\n"]
    pub fn mobile_redirect(&self) -> ListRef<ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mobile_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `opportunistic_encryption` after provisioning.\n"]
    pub fn opportunistic_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc= "Get a reference to the value of field `opportunistic_onion` after provisioning.\n"]
    pub fn opportunistic_onion(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_onion", self.base))
    }

    #[doc= "Get a reference to the value of field `orange_to_orange` after provisioning.\n"]
    pub fn orange_to_orange(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.orange_to_orange", self.base))
    }

    #[doc= "Get a reference to the value of field `origin_error_page_pass_thru` after provisioning.\n"]
    pub fn origin_error_page_pass_thru(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_pass_thru", self.base))
    }

    #[doc= "Get a reference to the value of field `origin_max_http_version` after provisioning.\n"]
    pub fn origin_max_http_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_max_http_version", self.base))
    }

    #[doc= "Get a reference to the value of field `polish` after provisioning.\n"]
    pub fn polish(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc= "Get a reference to the value of field `prefetch_preload` after provisioning.\n"]
    pub fn prefetch_preload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefetch_preload", self.base))
    }

    #[doc= "Get a reference to the value of field `privacy_pass` after provisioning.\n"]
    pub fn privacy_pass(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.privacy_pass", self.base))
    }

    #[doc= "Get a reference to the value of field `proxy_read_timeout` after provisioning.\n"]
    pub fn proxy_read_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_read_timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `pseudo_ipv4` after provisioning.\n"]
    pub fn pseudo_ipv4(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pseudo_ipv4", self.base))
    }

    #[doc= "Get a reference to the value of field `response_buffering` after provisioning.\n"]
    pub fn response_buffering(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response_buffering", self.base))
    }

    #[doc= "Get a reference to the value of field `rocket_loader` after provisioning.\n"]
    pub fn rocket_loader(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rocket_loader", self.base))
    }

    #[doc= "Get a reference to the value of field `security_header` after provisioning.\n"]
    pub fn security_header(&self) -> ListRef<ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_header", self.base))
    }

    #[doc= "Get a reference to the value of field `security_level` after provisioning.\n"]
    pub fn security_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc= "Get a reference to the value of field `server_side_exclude` after provisioning.\n"]
    pub fn server_side_exclude(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_side_exclude", self.base))
    }

    #[doc= "Get a reference to the value of field `sort_query_string_for_cache` after provisioning.\n"]
    pub fn sort_query_string_for_cache(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_query_string_for_cache", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc= "Get a reference to the value of field `tls_1_2_only` after provisioning.\n"]
    pub fn tls_1_2_only(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_1_2_only", self.base))
    }

    #[doc= "Get a reference to the value of field `tls_1_3` after provisioning.\n"]
    pub fn tls_1_3(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_1_3", self.base))
    }

    #[doc= "Get a reference to the value of field `tls_client_auth` after provisioning.\n"]
    pub fn tls_client_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_client_auth", self.base))
    }

    #[doc= "Get a reference to the value of field `true_client_ip_header` after provisioning.\n"]
    pub fn true_client_ip_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.true_client_ip_header", self.base))
    }

    #[doc= "Get a reference to the value of field `universal_ssl` after provisioning.\n"]
    pub fn universal_ssl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.universal_ssl", self.base))
    }

    #[doc= "Get a reference to the value of field `visitor_ip` after provisioning.\n"]
    pub fn visitor_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visitor_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `waf` after provisioning.\n"]
    pub fn waf(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.waf", self.base))
    }

    #[doc= "Get a reference to the value of field `webp` after provisioning.\n"]
    pub fn webp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webp", self.base))
    }

    #[doc= "Get a reference to the value of field `websockets` after provisioning.\n"]
    pub fn websockets(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.websockets", self.base))
    }

    #[doc= "Get a reference to the value of field `zero_rtt` after provisioning.\n"]
    pub fn zero_rtt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zero_rtt", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideSettingsElMinifyEl {
    css: PrimField<String>,
    html: PrimField<String>,
    js: PrimField<String>,
}

impl ZoneSettingsOverrideSettingsElMinifyEl { }

impl ToListMappable for ZoneSettingsOverrideSettingsElMinifyEl {
    type O = BlockAssignable<ZoneSettingsOverrideSettingsElMinifyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSettingsOverrideSettingsElMinifyEl {
    #[doc= ""]
    pub css: PrimField<String>,
    #[doc= ""]
    pub html: PrimField<String>,
    #[doc= ""]
    pub js: PrimField<String>,
}

impl BuildZoneSettingsOverrideSettingsElMinifyEl {
    pub fn build(self) -> ZoneSettingsOverrideSettingsElMinifyEl {
        ZoneSettingsOverrideSettingsElMinifyEl {
            css: self.css,
            html: self.html,
            js: self.js,
        }
    }
}

pub struct ZoneSettingsOverrideSettingsElMinifyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneSettingsOverrideSettingsElMinifyElRef {
    fn new(shared: StackShared, base: String) -> ZoneSettingsOverrideSettingsElMinifyElRef {
        ZoneSettingsOverrideSettingsElMinifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideSettingsElMinifyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `css` after provisioning.\n"]
    pub fn css(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.css", self.base))
    }

    #[doc= "Get a reference to the value of field `html` after provisioning.\n"]
    pub fn html(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }

    #[doc= "Get a reference to the value of field `js` after provisioning.\n"]
    pub fn js(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.js", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideSettingsElMobileRedirectEl {
    mobile_subdomain: PrimField<String>,
    status: PrimField<String>,
    strip_uri: PrimField<bool>,
}

impl ZoneSettingsOverrideSettingsElMobileRedirectEl { }

impl ToListMappable for ZoneSettingsOverrideSettingsElMobileRedirectEl {
    type O = BlockAssignable<ZoneSettingsOverrideSettingsElMobileRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSettingsOverrideSettingsElMobileRedirectEl {
    #[doc= ""]
    pub mobile_subdomain: PrimField<String>,
    #[doc= ""]
    pub status: PrimField<String>,
    #[doc= ""]
    pub strip_uri: PrimField<bool>,
}

impl BuildZoneSettingsOverrideSettingsElMobileRedirectEl {
    pub fn build(self) -> ZoneSettingsOverrideSettingsElMobileRedirectEl {
        ZoneSettingsOverrideSettingsElMobileRedirectEl {
            mobile_subdomain: self.mobile_subdomain,
            status: self.status,
            strip_uri: self.strip_uri,
        }
    }
}

pub struct ZoneSettingsOverrideSettingsElMobileRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneSettingsOverrideSettingsElMobileRedirectElRef {
    fn new(shared: StackShared, base: String) -> ZoneSettingsOverrideSettingsElMobileRedirectElRef {
        ZoneSettingsOverrideSettingsElMobileRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideSettingsElMobileRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mobile_subdomain` after provisioning.\n"]
    pub fn mobile_subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mobile_subdomain", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc= "Get a reference to the value of field `strip_uri` after provisioning.\n"]
    pub fn strip_uri(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideSettingsElSecurityHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_subdomains: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nosniff: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preload: Option<PrimField<bool>>,
}

impl ZoneSettingsOverrideSettingsElSecurityHeaderEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `include_subdomains`.\n"]
    pub fn set_include_subdomains(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_subdomains = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }

    #[doc= "Set the field `nosniff`.\n"]
    pub fn set_nosniff(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nosniff = Some(v.into());
        self
    }

    #[doc= "Set the field `preload`.\n"]
    pub fn set_preload(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preload = Some(v.into());
        self
    }
}

impl ToListMappable for ZoneSettingsOverrideSettingsElSecurityHeaderEl {
    type O = BlockAssignable<ZoneSettingsOverrideSettingsElSecurityHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSettingsOverrideSettingsElSecurityHeaderEl {}

impl BuildZoneSettingsOverrideSettingsElSecurityHeaderEl {
    pub fn build(self) -> ZoneSettingsOverrideSettingsElSecurityHeaderEl {
        ZoneSettingsOverrideSettingsElSecurityHeaderEl {
            enabled: core::default::Default::default(),
            include_subdomains: core::default::Default::default(),
            max_age: core::default::Default::default(),
            nosniff: core::default::Default::default(),
            preload: core::default::Default::default(),
        }
    }
}

pub struct ZoneSettingsOverrideSettingsElSecurityHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneSettingsOverrideSettingsElSecurityHeaderElRef {
    fn new(shared: StackShared, base: String) -> ZoneSettingsOverrideSettingsElSecurityHeaderElRef {
        ZoneSettingsOverrideSettingsElSecurityHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideSettingsElSecurityHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `include_subdomains` after provisioning.\n"]
    pub fn include_subdomains(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc= "Get a reference to the value of field `nosniff` after provisioning.\n"]
    pub fn nosniff(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nosniff", self.base))
    }

    #[doc= "Get a reference to the value of field `preload` after provisioning.\n"]
    pub fn preload(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preload", self.base))
    }
}

#[derive(Serialize, Default)]
struct ZoneSettingsOverrideSettingsElDynamic {
    minify: Option<DynamicBlock<ZoneSettingsOverrideSettingsElMinifyEl>>,
    mobile_redirect: Option<DynamicBlock<ZoneSettingsOverrideSettingsElMobileRedirectEl>>,
    security_header: Option<DynamicBlock<ZoneSettingsOverrideSettingsElSecurityHeaderEl>>,
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    always_online: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    always_use_https: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_https_rewrites: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_ast: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brotli: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_cache_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_check: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    challenge_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ciphers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cname_flattening: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    development_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    early_hints: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_obfuscation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_logs_to_cloudflare: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    h2_prioritization: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hotlink_protection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http3: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_resizing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_geolocation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_to_cloudflare: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_upload: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_tls_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opportunistic_encryption: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opportunistic_onion: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orange_to_orange: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_error_page_pass_thru: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_max_http_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polish: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefetch_preload: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy_pass: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_read_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pseudo_ipv4: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_buffering: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rocket_loader: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_exclude: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_query_string_for_cache: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_1_2_only: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_1_3: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_client_auth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    true_client_ip_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    universal_ssl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visitor_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    waf: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    websockets: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zero_rtt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minify: Option<Vec<ZoneSettingsOverrideSettingsElMinifyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile_redirect: Option<Vec<ZoneSettingsOverrideSettingsElMobileRedirectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_header: Option<Vec<ZoneSettingsOverrideSettingsElSecurityHeaderEl>>,
    dynamic: ZoneSettingsOverrideSettingsElDynamic,
}

impl ZoneSettingsOverrideSettingsEl {
    #[doc= "Set the field `always_online`.\n"]
    pub fn set_always_online(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.always_online = Some(v.into());
        self
    }

    #[doc= "Set the field `always_use_https`.\n"]
    pub fn set_always_use_https(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.always_use_https = Some(v.into());
        self
    }

    #[doc= "Set the field `automatic_https_rewrites`.\n"]
    pub fn set_automatic_https_rewrites(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.automatic_https_rewrites = Some(v.into());
        self
    }

    #[doc= "Set the field `binary_ast`.\n"]
    pub fn set_binary_ast(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.binary_ast = Some(v.into());
        self
    }

    #[doc= "Set the field `brotli`.\n"]
    pub fn set_brotli(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.brotli = Some(v.into());
        self
    }

    #[doc= "Set the field `browser_cache_ttl`.\n"]
    pub fn set_browser_cache_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.browser_cache_ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `browser_check`.\n"]
    pub fn set_browser_check(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.browser_check = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_level`.\n"]
    pub fn set_cache_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_level = Some(v.into());
        self
    }

    #[doc= "Set the field `challenge_ttl`.\n"]
    pub fn set_challenge_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.challenge_ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `ciphers`.\n"]
    pub fn set_ciphers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ciphers = Some(v.into());
        self
    }

    #[doc= "Set the field `cname_flattening`.\n"]
    pub fn set_cname_flattening(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cname_flattening = Some(v.into());
        self
    }

    #[doc= "Set the field `development_mode`.\n"]
    pub fn set_development_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.development_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `early_hints`.\n"]
    pub fn set_early_hints(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.early_hints = Some(v.into());
        self
    }

    #[doc= "Set the field `email_obfuscation`.\n"]
    pub fn set_email_obfuscation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_obfuscation = Some(v.into());
        self
    }

    #[doc= "Set the field `filter_logs_to_cloudflare`.\n"]
    pub fn set_filter_logs_to_cloudflare(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_logs_to_cloudflare = Some(v.into());
        self
    }

    #[doc= "Set the field `h2_prioritization`.\n"]
    pub fn set_h2_prioritization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.h2_prioritization = Some(v.into());
        self
    }

    #[doc= "Set the field `hotlink_protection`.\n"]
    pub fn set_hotlink_protection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hotlink_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `http2`.\n"]
    pub fn set_http2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http2 = Some(v.into());
        self
    }

    #[doc= "Set the field `http3`.\n"]
    pub fn set_http3(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http3 = Some(v.into());
        self
    }

    #[doc= "Set the field `image_resizing`.\n"]
    pub fn set_image_resizing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_resizing = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_geolocation`.\n"]
    pub fn set_ip_geolocation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_geolocation = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6`.\n"]
    pub fn set_ipv6(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6 = Some(v.into());
        self
    }

    #[doc= "Set the field `log_to_cloudflare`.\n"]
    pub fn set_log_to_cloudflare(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_to_cloudflare = Some(v.into());
        self
    }

    #[doc= "Set the field `max_upload`.\n"]
    pub fn set_max_upload(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_upload = Some(v.into());
        self
    }

    #[doc= "Set the field `min_tls_version`.\n"]
    pub fn set_min_tls_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_tls_version = Some(v.into());
        self
    }

    #[doc= "Set the field `mirage`.\n"]
    pub fn set_mirage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mirage = Some(v.into());
        self
    }

    #[doc= "Set the field `opportunistic_encryption`.\n"]
    pub fn set_opportunistic_encryption(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.opportunistic_encryption = Some(v.into());
        self
    }

    #[doc= "Set the field `opportunistic_onion`.\n"]
    pub fn set_opportunistic_onion(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.opportunistic_onion = Some(v.into());
        self
    }

    #[doc= "Set the field `orange_to_orange`.\n"]
    pub fn set_orange_to_orange(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.orange_to_orange = Some(v.into());
        self
    }

    #[doc= "Set the field `origin_error_page_pass_thru`.\n"]
    pub fn set_origin_error_page_pass_thru(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_error_page_pass_thru = Some(v.into());
        self
    }

    #[doc= "Set the field `origin_max_http_version`.\n"]
    pub fn set_origin_max_http_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_max_http_version = Some(v.into());
        self
    }

    #[doc= "Set the field `polish`.\n"]
    pub fn set_polish(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.polish = Some(v.into());
        self
    }

    #[doc= "Set the field `prefetch_preload`.\n"]
    pub fn set_prefetch_preload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefetch_preload = Some(v.into());
        self
    }

    #[doc= "Set the field `privacy_pass`.\n"]
    pub fn set_privacy_pass(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.privacy_pass = Some(v.into());
        self
    }

    #[doc= "Set the field `proxy_read_timeout`.\n"]
    pub fn set_proxy_read_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.proxy_read_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `pseudo_ipv4`.\n"]
    pub fn set_pseudo_ipv4(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pseudo_ipv4 = Some(v.into());
        self
    }

    #[doc= "Set the field `response_buffering`.\n"]
    pub fn set_response_buffering(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_buffering = Some(v.into());
        self
    }

    #[doc= "Set the field `rocket_loader`.\n"]
    pub fn set_rocket_loader(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rocket_loader = Some(v.into());
        self
    }

    #[doc= "Set the field `security_level`.\n"]
    pub fn set_security_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_level = Some(v.into());
        self
    }

    #[doc= "Set the field `server_side_exclude`.\n"]
    pub fn set_server_side_exclude(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_side_exclude = Some(v.into());
        self
    }

    #[doc= "Set the field `sort_query_string_for_cache`.\n"]
    pub fn set_sort_query_string_for_cache(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sort_query_string_for_cache = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl`.\n"]
    pub fn set_ssl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl = Some(v.into());
        self
    }

    #[doc= "Set the field `tls_1_2_only`.\n"]
    pub fn set_tls_1_2_only(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_1_2_only = Some(v.into());
        self
    }

    #[doc= "Set the field `tls_1_3`.\n"]
    pub fn set_tls_1_3(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_1_3 = Some(v.into());
        self
    }

    #[doc= "Set the field `tls_client_auth`.\n"]
    pub fn set_tls_client_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_client_auth = Some(v.into());
        self
    }

    #[doc= "Set the field `true_client_ip_header`.\n"]
    pub fn set_true_client_ip_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.true_client_ip_header = Some(v.into());
        self
    }

    #[doc= "Set the field `universal_ssl`.\n"]
    pub fn set_universal_ssl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.universal_ssl = Some(v.into());
        self
    }

    #[doc= "Set the field `visitor_ip`.\n"]
    pub fn set_visitor_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visitor_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `waf`.\n"]
    pub fn set_waf(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.waf = Some(v.into());
        self
    }

    #[doc= "Set the field `webp`.\n"]
    pub fn set_webp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.webp = Some(v.into());
        self
    }

    #[doc= "Set the field `websockets`.\n"]
    pub fn set_websockets(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.websockets = Some(v.into());
        self
    }

    #[doc= "Set the field `zero_rtt`.\n"]
    pub fn set_zero_rtt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zero_rtt = Some(v.into());
        self
    }

    #[doc= "Set the field `minify`.\n"]
    pub fn set_minify(mut self, v: impl Into<BlockAssignable<ZoneSettingsOverrideSettingsElMinifyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.minify = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.minify = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `mobile_redirect`.\n"]
    pub fn set_mobile_redirect(
        mut self,
        v: impl Into<BlockAssignable<ZoneSettingsOverrideSettingsElMobileRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mobile_redirect = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mobile_redirect = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `security_header`.\n"]
    pub fn set_security_header(
        mut self,
        v: impl Into<BlockAssignable<ZoneSettingsOverrideSettingsElSecurityHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_header = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_header = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ZoneSettingsOverrideSettingsEl {
    type O = BlockAssignable<ZoneSettingsOverrideSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSettingsOverrideSettingsEl {}

impl BuildZoneSettingsOverrideSettingsEl {
    pub fn build(self) -> ZoneSettingsOverrideSettingsEl {
        ZoneSettingsOverrideSettingsEl {
            always_online: core::default::Default::default(),
            always_use_https: core::default::Default::default(),
            automatic_https_rewrites: core::default::Default::default(),
            binary_ast: core::default::Default::default(),
            brotli: core::default::Default::default(),
            browser_cache_ttl: core::default::Default::default(),
            browser_check: core::default::Default::default(),
            cache_level: core::default::Default::default(),
            challenge_ttl: core::default::Default::default(),
            ciphers: core::default::Default::default(),
            cname_flattening: core::default::Default::default(),
            development_mode: core::default::Default::default(),
            early_hints: core::default::Default::default(),
            email_obfuscation: core::default::Default::default(),
            filter_logs_to_cloudflare: core::default::Default::default(),
            h2_prioritization: core::default::Default::default(),
            hotlink_protection: core::default::Default::default(),
            http2: core::default::Default::default(),
            http3: core::default::Default::default(),
            image_resizing: core::default::Default::default(),
            ip_geolocation: core::default::Default::default(),
            ipv6: core::default::Default::default(),
            log_to_cloudflare: core::default::Default::default(),
            max_upload: core::default::Default::default(),
            min_tls_version: core::default::Default::default(),
            mirage: core::default::Default::default(),
            opportunistic_encryption: core::default::Default::default(),
            opportunistic_onion: core::default::Default::default(),
            orange_to_orange: core::default::Default::default(),
            origin_error_page_pass_thru: core::default::Default::default(),
            origin_max_http_version: core::default::Default::default(),
            polish: core::default::Default::default(),
            prefetch_preload: core::default::Default::default(),
            privacy_pass: core::default::Default::default(),
            proxy_read_timeout: core::default::Default::default(),
            pseudo_ipv4: core::default::Default::default(),
            response_buffering: core::default::Default::default(),
            rocket_loader: core::default::Default::default(),
            security_level: core::default::Default::default(),
            server_side_exclude: core::default::Default::default(),
            sort_query_string_for_cache: core::default::Default::default(),
            ssl: core::default::Default::default(),
            tls_1_2_only: core::default::Default::default(),
            tls_1_3: core::default::Default::default(),
            tls_client_auth: core::default::Default::default(),
            true_client_ip_header: core::default::Default::default(),
            universal_ssl: core::default::Default::default(),
            visitor_ip: core::default::Default::default(),
            waf: core::default::Default::default(),
            webp: core::default::Default::default(),
            websockets: core::default::Default::default(),
            zero_rtt: core::default::Default::default(),
            minify: core::default::Default::default(),
            mobile_redirect: core::default::Default::default(),
            security_header: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ZoneSettingsOverrideSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneSettingsOverrideSettingsElRef {
    fn new(shared: StackShared, base: String) -> ZoneSettingsOverrideSettingsElRef {
        ZoneSettingsOverrideSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `always_online` after provisioning.\n"]
    pub fn always_online(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.always_online", self.base))
    }

    #[doc= "Get a reference to the value of field `always_use_https` after provisioning.\n"]
    pub fn always_use_https(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.always_use_https", self.base))
    }

    #[doc= "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\n"]
    pub fn automatic_https_rewrites(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc= "Get a reference to the value of field `binary_ast` after provisioning.\n"]
    pub fn binary_ast(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.binary_ast", self.base))
    }

    #[doc= "Get a reference to the value of field `brotli` after provisioning.\n"]
    pub fn brotli(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.brotli", self.base))
    }

    #[doc= "Get a reference to the value of field `browser_cache_ttl` after provisioning.\n"]
    pub fn browser_cache_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_cache_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `browser_check` after provisioning.\n"]
    pub fn browser_check(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_check", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_level` after provisioning.\n"]
    pub fn cache_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_level", self.base))
    }

    #[doc= "Get a reference to the value of field `challenge_ttl` after provisioning.\n"]
    pub fn challenge_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.challenge_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `ciphers` after provisioning.\n"]
    pub fn ciphers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ciphers", self.base))
    }

    #[doc= "Get a reference to the value of field `cname_flattening` after provisioning.\n"]
    pub fn cname_flattening(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cname_flattening", self.base))
    }

    #[doc= "Get a reference to the value of field `development_mode` after provisioning.\n"]
    pub fn development_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.development_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `early_hints` after provisioning.\n"]
    pub fn early_hints(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.early_hints", self.base))
    }

    #[doc= "Get a reference to the value of field `email_obfuscation` after provisioning.\n"]
    pub fn email_obfuscation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc= "Get a reference to the value of field `filter_logs_to_cloudflare` after provisioning.\n"]
    pub fn filter_logs_to_cloudflare(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_logs_to_cloudflare", self.base))
    }

    #[doc= "Get a reference to the value of field `h2_prioritization` after provisioning.\n"]
    pub fn h2_prioritization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.h2_prioritization", self.base))
    }

    #[doc= "Get a reference to the value of field `hotlink_protection` after provisioning.\n"]
    pub fn hotlink_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hotlink_protection", self.base))
    }

    #[doc= "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc= "Get a reference to the value of field `http3` after provisioning.\n"]
    pub fn http3(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http3", self.base))
    }

    #[doc= "Get a reference to the value of field `image_resizing` after provisioning.\n"]
    pub fn image_resizing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_resizing", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_geolocation` after provisioning.\n"]
    pub fn ip_geolocation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_geolocation", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }

    #[doc= "Get a reference to the value of field `log_to_cloudflare` after provisioning.\n"]
    pub fn log_to_cloudflare(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_to_cloudflare", self.base))
    }

    #[doc= "Get a reference to the value of field `max_upload` after provisioning.\n"]
    pub fn max_upload(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_upload", self.base))
    }

    #[doc= "Get a reference to the value of field `min_tls_version` after provisioning.\n"]
    pub fn min_tls_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_tls_version", self.base))
    }

    #[doc= "Get a reference to the value of field `mirage` after provisioning.\n"]
    pub fn mirage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc= "Get a reference to the value of field `opportunistic_encryption` after provisioning.\n"]
    pub fn opportunistic_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc= "Get a reference to the value of field `opportunistic_onion` after provisioning.\n"]
    pub fn opportunistic_onion(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_onion", self.base))
    }

    #[doc= "Get a reference to the value of field `orange_to_orange` after provisioning.\n"]
    pub fn orange_to_orange(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.orange_to_orange", self.base))
    }

    #[doc= "Get a reference to the value of field `origin_error_page_pass_thru` after provisioning.\n"]
    pub fn origin_error_page_pass_thru(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_pass_thru", self.base))
    }

    #[doc= "Get a reference to the value of field `origin_max_http_version` after provisioning.\n"]
    pub fn origin_max_http_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_max_http_version", self.base))
    }

    #[doc= "Get a reference to the value of field `polish` after provisioning.\n"]
    pub fn polish(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc= "Get a reference to the value of field `prefetch_preload` after provisioning.\n"]
    pub fn prefetch_preload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefetch_preload", self.base))
    }

    #[doc= "Get a reference to the value of field `privacy_pass` after provisioning.\n"]
    pub fn privacy_pass(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.privacy_pass", self.base))
    }

    #[doc= "Get a reference to the value of field `proxy_read_timeout` after provisioning.\n"]
    pub fn proxy_read_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_read_timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `pseudo_ipv4` after provisioning.\n"]
    pub fn pseudo_ipv4(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pseudo_ipv4", self.base))
    }

    #[doc= "Get a reference to the value of field `response_buffering` after provisioning.\n"]
    pub fn response_buffering(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response_buffering", self.base))
    }

    #[doc= "Get a reference to the value of field `rocket_loader` after provisioning.\n"]
    pub fn rocket_loader(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rocket_loader", self.base))
    }

    #[doc= "Get a reference to the value of field `security_level` after provisioning.\n"]
    pub fn security_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc= "Get a reference to the value of field `server_side_exclude` after provisioning.\n"]
    pub fn server_side_exclude(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_side_exclude", self.base))
    }

    #[doc= "Get a reference to the value of field `sort_query_string_for_cache` after provisioning.\n"]
    pub fn sort_query_string_for_cache(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_query_string_for_cache", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc= "Get a reference to the value of field `tls_1_2_only` after provisioning.\n"]
    pub fn tls_1_2_only(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_1_2_only", self.base))
    }

    #[doc= "Get a reference to the value of field `tls_1_3` after provisioning.\n"]
    pub fn tls_1_3(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_1_3", self.base))
    }

    #[doc= "Get a reference to the value of field `tls_client_auth` after provisioning.\n"]
    pub fn tls_client_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_client_auth", self.base))
    }

    #[doc= "Get a reference to the value of field `true_client_ip_header` after provisioning.\n"]
    pub fn true_client_ip_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.true_client_ip_header", self.base))
    }

    #[doc= "Get a reference to the value of field `universal_ssl` after provisioning.\n"]
    pub fn universal_ssl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.universal_ssl", self.base))
    }

    #[doc= "Get a reference to the value of field `visitor_ip` after provisioning.\n"]
    pub fn visitor_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visitor_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `waf` after provisioning.\n"]
    pub fn waf(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.waf", self.base))
    }

    #[doc= "Get a reference to the value of field `webp` after provisioning.\n"]
    pub fn webp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webp", self.base))
    }

    #[doc= "Get a reference to the value of field `websockets` after provisioning.\n"]
    pub fn websockets(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.websockets", self.base))
    }

    #[doc= "Get a reference to the value of field `zero_rtt` after provisioning.\n"]
    pub fn zero_rtt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zero_rtt", self.base))
    }

    #[doc= "Get a reference to the value of field `minify` after provisioning.\n"]
    pub fn minify(&self) -> ListRef<ZoneSettingsOverrideSettingsElMinifyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.minify", self.base))
    }

    #[doc= "Get a reference to the value of field `mobile_redirect` after provisioning.\n"]
    pub fn mobile_redirect(&self) -> ListRef<ZoneSettingsOverrideSettingsElMobileRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mobile_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `security_header` after provisioning.\n"]
    pub fn security_header(&self) -> ListRef<ZoneSettingsOverrideSettingsElSecurityHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_header", self.base))
    }
}

#[derive(Serialize, Default)]
struct ZoneSettingsOverrideDynamic {
    settings: Option<DynamicBlock<ZoneSettingsOverrideSettingsEl>>,
}
