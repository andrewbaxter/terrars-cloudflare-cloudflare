use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneSettingsOverrideData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<ZoneSettingsOverrideSettingsEl>>,
    dynamic: ZoneSettingsOverrideDynamic,
}

struct ZoneSettingsOverride_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZoneSettingsOverrideData>,
}

#[derive(Clone)]
pub struct ZoneSettingsOverride(std::rc::Rc<ZoneSettingsOverride_>);

impl ZoneSettingsOverride {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ZoneSettingsOverrideSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_settings` after provisioning.\n"]
    pub fn initial_settings(&self) -> terrars::ListRef<ZoneSettingsOverrideInitialSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_settings_read_at` after provisioning.\n"]
    pub fn initial_settings_read_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_settings_read_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `readonly_settings` after provisioning.\n"]
    pub fn readonly_settings(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.readonly_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_status` after provisioning.\n"]
    pub fn zone_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_type` after provisioning.\n"]
    pub fn zone_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<ZoneSettingsOverrideSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl terrars::Referable for ZoneSettingsOverride {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZoneSettingsOverride { }

impl terrars::ToListMappable for ZoneSettingsOverride {
    type O = terrars::ListRef<ZoneSettingsOverrideRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZoneSettingsOverride_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zone_settings_override".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneSettingsOverride {
    pub tf_id: std::string::String,
    #[doc =
        "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildZoneSettingsOverride {
    pub fn build(self, stack: &mut terrars::Stack) -> ZoneSettingsOverride {
        let out = ZoneSettingsOverride(std::rc::Rc::new(ZoneSettingsOverride_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZoneSettingsOverrideData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                zone_id: self.zone_id,
                settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneSettingsOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSettingsOverrideRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneSettingsOverrideRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_settings` after provisioning.\n"]
    pub fn initial_settings(&self) -> terrars::ListRef<ZoneSettingsOverrideInitialSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_settings_read_at` after provisioning.\n"]
    pub fn initial_settings_read_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_settings_read_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `readonly_settings` after provisioning.\n"]
    pub fn readonly_settings(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.readonly_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_status` after provisioning.\n"]
    pub fn zone_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_type` after provisioning.\n"]
    pub fn zone_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<ZoneSettingsOverrideSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideInitialSettingsElMinifyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    css: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    html: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    js: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZoneSettingsOverrideInitialSettingsElMinifyEl {
    #[doc = "Set the field `css`.\n"]
    pub fn set_css(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.css = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `html`.\n"]
    pub fn set_html(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.html = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `js`.\n"]
    pub fn set_js(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.js = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZoneSettingsOverrideInitialSettingsElMinifyEl {
    type O = terrars::BlockAssignable<ZoneSettingsOverrideInitialSettingsElMinifyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSettingsOverrideInitialSettingsElMinifyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZoneSettingsOverrideInitialSettingsElMinifyElRef {
        ZoneSettingsOverrideInitialSettingsElMinifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideInitialSettingsElMinifyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `css` after provisioning.\n"]
    pub fn css(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.css", self.base))
    }

    #[doc = "Get a reference to the value of field `html` after provisioning.\n"]
    pub fn html(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }

    #[doc = "Get a reference to the value of field `js` after provisioning.\n"]
    pub fn js(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.js", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideInitialSettingsElMobileRedirectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mobile_subdomain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strip_uri: std::option::Option<terrars::PrimField<bool>>,
}

impl ZoneSettingsOverrideInitialSettingsElMobileRedirectEl {
    #[doc = "Set the field `mobile_subdomain`.\n"]
    pub fn set_mobile_subdomain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mobile_subdomain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strip_uri`.\n"]
    pub fn set_strip_uri(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strip_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZoneSettingsOverrideInitialSettingsElMobileRedirectEl {
    type O = terrars::BlockAssignable<ZoneSettingsOverrideInitialSettingsElMobileRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef {
        ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mobile_subdomain` after provisioning.\n"]
    pub fn mobile_subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mobile_subdomain", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `strip_uri` after provisioning.\n"]
    pub fn strip_uri(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_subdomains: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nosniff: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preload: std::option::Option<terrars::PrimField<bool>>,
}

impl ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_subdomains`.\n"]
    pub fn set_include_subdomains(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_subdomains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nosniff`.\n"]
    pub fn set_nosniff(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nosniff = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preload`.\n"]
    pub fn set_preload(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preload = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl {
    type O = terrars::BlockAssignable<ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef {
        ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `include_subdomains` after provisioning.\n"]
    pub fn include_subdomains(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `nosniff` after provisioning.\n"]
    pub fn nosniff(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nosniff", self.base))
    }

    #[doc = "Get a reference to the value of field `preload` after provisioning.\n"]
    pub fn preload(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preload", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideInitialSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    always_online: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    always_use_https: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_https_rewrites: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_ast: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    brotli: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_cache_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ciphers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cname_flattening: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    development_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    early_hints: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_obfuscation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_logs_to_cloudflare: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    h2_prioritization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotlink_protection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_resizing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_geolocation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_to_cloudflare: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_upload: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_tls_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minify: std::option::Option<terrars::ListField<ZoneSettingsOverrideInitialSettingsElMinifyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mirage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mobile_redirect: std::option::Option<terrars::ListField<ZoneSettingsOverrideInitialSettingsElMobileRedirectEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opportunistic_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opportunistic_onion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orange_to_orange: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_error_page_pass_thru: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_max_http_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    polish: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefetch_preload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privacy_pass: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_read_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pseudo_ipv4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_buffering: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rocket_loader: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_header: std::option::Option<terrars::ListField<ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_exclude: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_query_string_for_cache: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_1_2_only: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_1_3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_client_auth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    true_client_ip_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    universal_ssl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visitor_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    waf: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    websockets: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zero_rtt: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ZoneSettingsOverrideInitialSettingsEl {
    #[doc = "Set the field `always_online`.\n"]
    pub fn set_always_online(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.always_online = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `always_use_https`.\n"]
    pub fn set_always_use_https(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.always_use_https = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_https_rewrites`.\n"]
    pub fn set_automatic_https_rewrites(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.automatic_https_rewrites = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `binary_ast`.\n"]
    pub fn set_binary_ast(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.binary_ast = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `brotli`.\n"]
    pub fn set_brotli(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.brotli = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `browser_cache_ttl`.\n"]
    pub fn set_browser_cache_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.browser_cache_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `browser_check`.\n"]
    pub fn set_browser_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.browser_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_level`.\n"]
    pub fn set_cache_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `challenge_ttl`.\n"]
    pub fn set_challenge_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.challenge_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ciphers`.\n"]
    pub fn set_ciphers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ciphers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cname_flattening`.\n"]
    pub fn set_cname_flattening(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cname_flattening = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `development_mode`.\n"]
    pub fn set_development_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.development_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `early_hints`.\n"]
    pub fn set_early_hints(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.early_hints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_obfuscation`.\n"]
    pub fn set_email_obfuscation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_obfuscation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_logs_to_cloudflare`.\n"]
    pub fn set_filter_logs_to_cloudflare(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filter_logs_to_cloudflare = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `h2_prioritization`.\n"]
    pub fn set_h2_prioritization(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.h2_prioritization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hotlink_protection`.\n"]
    pub fn set_hotlink_protection(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.hotlink_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http3`.\n"]
    pub fn set_http3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_resizing`.\n"]
    pub fn set_image_resizing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_resizing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_geolocation`.\n"]
    pub fn set_ip_geolocation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_geolocation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6`.\n"]
    pub fn set_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_to_cloudflare`.\n"]
    pub fn set_log_to_cloudflare(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_to_cloudflare = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_upload`.\n"]
    pub fn set_max_upload(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_upload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_tls_version`.\n"]
    pub fn set_min_tls_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_tls_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minify`.\n"]
    pub fn set_minify(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ZoneSettingsOverrideInitialSettingsElMinifyEl>>,
    ) -> Self {
        self.minify = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mirage`.\n"]
    pub fn set_mirage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mirage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mobile_redirect`.\n"]
    pub fn set_mobile_redirect(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ZoneSettingsOverrideInitialSettingsElMobileRedirectEl>>,
    ) -> Self {
        self.mobile_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `opportunistic_encryption`.\n"]
    pub fn set_opportunistic_encryption(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.opportunistic_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `opportunistic_onion`.\n"]
    pub fn set_opportunistic_onion(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.opportunistic_onion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `orange_to_orange`.\n"]
    pub fn set_orange_to_orange(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.orange_to_orange = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_error_page_pass_thru`.\n"]
    pub fn set_origin_error_page_pass_thru(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_error_page_pass_thru = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_max_http_version`.\n"]
    pub fn set_origin_max_http_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_max_http_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `polish`.\n"]
    pub fn set_polish(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.polish = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefetch_preload`.\n"]
    pub fn set_prefetch_preload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefetch_preload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `privacy_pass`.\n"]
    pub fn set_privacy_pass(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.privacy_pass = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_read_timeout`.\n"]
    pub fn set_proxy_read_timeout(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.proxy_read_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pseudo_ipv4`.\n"]
    pub fn set_pseudo_ipv4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pseudo_ipv4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_buffering`.\n"]
    pub fn set_response_buffering(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.response_buffering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rocket_loader`.\n"]
    pub fn set_rocket_loader(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rocket_loader = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_header`.\n"]
    pub fn set_security_header(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ZoneSettingsOverrideInitialSettingsElSecurityHeaderEl>>,
    ) -> Self {
        self.security_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_level`.\n"]
    pub fn set_security_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_exclude`.\n"]
    pub fn set_server_side_exclude(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.server_side_exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_query_string_for_cache`.\n"]
    pub fn set_sort_query_string_for_cache(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sort_query_string_for_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_1_2_only`.\n"]
    pub fn set_tls_1_2_only(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_1_2_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_1_3`.\n"]
    pub fn set_tls_1_3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_1_3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_client_auth`.\n"]
    pub fn set_tls_client_auth(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_client_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `true_client_ip_header`.\n"]
    pub fn set_true_client_ip_header(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.true_client_ip_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `universal_ssl`.\n"]
    pub fn set_universal_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.universal_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visitor_ip`.\n"]
    pub fn set_visitor_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.visitor_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `waf`.\n"]
    pub fn set_waf(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.waf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `webp`.\n"]
    pub fn set_webp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.webp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `websockets`.\n"]
    pub fn set_websockets(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.websockets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zero_rtt`.\n"]
    pub fn set_zero_rtt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zero_rtt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZoneSettingsOverrideInitialSettingsEl {
    type O = terrars::BlockAssignable<ZoneSettingsOverrideInitialSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSettingsOverrideInitialSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZoneSettingsOverrideInitialSettingsElRef {
        ZoneSettingsOverrideInitialSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideInitialSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `always_online` after provisioning.\n"]
    pub fn always_online(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.always_online", self.base))
    }

    #[doc = "Get a reference to the value of field `always_use_https` after provisioning.\n"]
    pub fn always_use_https(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.always_use_https", self.base))
    }

    #[doc = "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\n"]
    pub fn automatic_https_rewrites(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc = "Get a reference to the value of field `binary_ast` after provisioning.\n"]
    pub fn binary_ast(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.binary_ast", self.base))
    }

    #[doc = "Get a reference to the value of field `brotli` after provisioning.\n"]
    pub fn brotli(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.brotli", self.base))
    }

    #[doc = "Get a reference to the value of field `browser_cache_ttl` after provisioning.\n"]
    pub fn browser_cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_cache_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `browser_check` after provisioning.\n"]
    pub fn browser_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_check", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_level` after provisioning.\n"]
    pub fn cache_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_level", self.base))
    }

    #[doc = "Get a reference to the value of field `challenge_ttl` after provisioning.\n"]
    pub fn challenge_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.challenge_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `ciphers` after provisioning.\n"]
    pub fn ciphers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ciphers", self.base))
    }

    #[doc = "Get a reference to the value of field `cname_flattening` after provisioning.\n"]
    pub fn cname_flattening(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname_flattening", self.base))
    }

    #[doc = "Get a reference to the value of field `development_mode` after provisioning.\n"]
    pub fn development_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.development_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `early_hints` after provisioning.\n"]
    pub fn early_hints(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.early_hints", self.base))
    }

    #[doc = "Get a reference to the value of field `email_obfuscation` after provisioning.\n"]
    pub fn email_obfuscation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_logs_to_cloudflare` after provisioning.\n"]
    pub fn filter_logs_to_cloudflare(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_logs_to_cloudflare", self.base))
    }

    #[doc = "Get a reference to the value of field `h2_prioritization` after provisioning.\n"]
    pub fn h2_prioritization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.h2_prioritization", self.base))
    }

    #[doc = "Get a reference to the value of field `hotlink_protection` after provisioning.\n"]
    pub fn hotlink_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hotlink_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc = "Get a reference to the value of field `http3` after provisioning.\n"]
    pub fn http3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http3", self.base))
    }

    #[doc = "Get a reference to the value of field `image_resizing` after provisioning.\n"]
    pub fn image_resizing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_resizing", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_geolocation` after provisioning.\n"]
    pub fn ip_geolocation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_geolocation", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `log_to_cloudflare` after provisioning.\n"]
    pub fn log_to_cloudflare(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_to_cloudflare", self.base))
    }

    #[doc = "Get a reference to the value of field `max_upload` after provisioning.\n"]
    pub fn max_upload(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_upload", self.base))
    }

    #[doc = "Get a reference to the value of field `min_tls_version` after provisioning.\n"]
    pub fn min_tls_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_tls_version", self.base))
    }

    #[doc = "Get a reference to the value of field `minify` after provisioning.\n"]
    pub fn minify(&self) -> terrars::ListRef<ZoneSettingsOverrideInitialSettingsElMinifyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.minify", self.base))
    }

    #[doc = "Get a reference to the value of field `mirage` after provisioning.\n"]
    pub fn mirage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc = "Get a reference to the value of field `mobile_redirect` after provisioning.\n"]
    pub fn mobile_redirect(&self) -> terrars::ListRef<ZoneSettingsOverrideInitialSettingsElMobileRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mobile_redirect", self.base))
    }

    #[doc = "Get a reference to the value of field `opportunistic_encryption` after provisioning.\n"]
    pub fn opportunistic_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `opportunistic_onion` after provisioning.\n"]
    pub fn opportunistic_onion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_onion", self.base))
    }

    #[doc = "Get a reference to the value of field `orange_to_orange` after provisioning.\n"]
    pub fn orange_to_orange(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.orange_to_orange", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_error_page_pass_thru` after provisioning.\n"]
    pub fn origin_error_page_pass_thru(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_pass_thru", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_max_http_version` after provisioning.\n"]
    pub fn origin_max_http_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_max_http_version", self.base))
    }

    #[doc = "Get a reference to the value of field `polish` after provisioning.\n"]
    pub fn polish(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc = "Get a reference to the value of field `prefetch_preload` after provisioning.\n"]
    pub fn prefetch_preload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefetch_preload", self.base))
    }

    #[doc = "Get a reference to the value of field `privacy_pass` after provisioning.\n"]
    pub fn privacy_pass(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.privacy_pass", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_read_timeout` after provisioning.\n"]
    pub fn proxy_read_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_read_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `pseudo_ipv4` after provisioning.\n"]
    pub fn pseudo_ipv4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pseudo_ipv4", self.base))
    }

    #[doc = "Get a reference to the value of field `response_buffering` after provisioning.\n"]
    pub fn response_buffering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_buffering", self.base))
    }

    #[doc = "Get a reference to the value of field `rocket_loader` after provisioning.\n"]
    pub fn rocket_loader(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rocket_loader", self.base))
    }

    #[doc = "Get a reference to the value of field `security_header` after provisioning.\n"]
    pub fn security_header(&self) -> terrars::ListRef<ZoneSettingsOverrideInitialSettingsElSecurityHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_header", self.base))
    }

    #[doc = "Get a reference to the value of field `security_level` after provisioning.\n"]
    pub fn security_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc = "Get a reference to the value of field `server_side_exclude` after provisioning.\n"]
    pub fn server_side_exclude(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_side_exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_query_string_for_cache` after provisioning.\n"]
    pub fn sort_query_string_for_cache(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_query_string_for_cache", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_1_2_only` after provisioning.\n"]
    pub fn tls_1_2_only(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_1_2_only", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_1_3` after provisioning.\n"]
    pub fn tls_1_3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_1_3", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_client_auth` after provisioning.\n"]
    pub fn tls_client_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_client_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `true_client_ip_header` after provisioning.\n"]
    pub fn true_client_ip_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.true_client_ip_header", self.base))
    }

    #[doc = "Get a reference to the value of field `universal_ssl` after provisioning.\n"]
    pub fn universal_ssl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.universal_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `visitor_ip` after provisioning.\n"]
    pub fn visitor_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visitor_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `waf` after provisioning.\n"]
    pub fn waf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waf", self.base))
    }

    #[doc = "Get a reference to the value of field `webp` after provisioning.\n"]
    pub fn webp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webp", self.base))
    }

    #[doc = "Get a reference to the value of field `websockets` after provisioning.\n"]
    pub fn websockets(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.websockets", self.base))
    }

    #[doc = "Get a reference to the value of field `zero_rtt` after provisioning.\n"]
    pub fn zero_rtt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zero_rtt", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideSettingsElMinifyEl {
    css: terrars::PrimField<std::string::String>,
    html: terrars::PrimField<std::string::String>,
    js: terrars::PrimField<std::string::String>,
}

impl ZoneSettingsOverrideSettingsElMinifyEl { }

impl terrars::ToListMappable for ZoneSettingsOverrideSettingsElMinifyEl {
    type O = terrars::BlockAssignable<ZoneSettingsOverrideSettingsElMinifyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSettingsOverrideSettingsElMinifyEl {
    #[doc = ""]
    pub css: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub html: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub js: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSettingsOverrideSettingsElMinifyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZoneSettingsOverrideSettingsElMinifyElRef {
        ZoneSettingsOverrideSettingsElMinifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideSettingsElMinifyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `css` after provisioning.\n"]
    pub fn css(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.css", self.base))
    }

    #[doc = "Get a reference to the value of field `html` after provisioning.\n"]
    pub fn html(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }

    #[doc = "Get a reference to the value of field `js` after provisioning.\n"]
    pub fn js(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.js", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideSettingsElMobileRedirectEl {
    mobile_subdomain: terrars::PrimField<std::string::String>,
    status: terrars::PrimField<std::string::String>,
    strip_uri: terrars::PrimField<bool>,
}

impl ZoneSettingsOverrideSettingsElMobileRedirectEl { }

impl terrars::ToListMappable for ZoneSettingsOverrideSettingsElMobileRedirectEl {
    type O = terrars::BlockAssignable<ZoneSettingsOverrideSettingsElMobileRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneSettingsOverrideSettingsElMobileRedirectEl {
    #[doc = ""]
    pub mobile_subdomain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub strip_uri: terrars::PrimField<bool>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSettingsOverrideSettingsElMobileRedirectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZoneSettingsOverrideSettingsElMobileRedirectElRef {
        ZoneSettingsOverrideSettingsElMobileRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideSettingsElMobileRedirectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mobile_subdomain` after provisioning.\n"]
    pub fn mobile_subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mobile_subdomain", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `strip_uri` after provisioning.\n"]
    pub fn strip_uri(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideSettingsElSecurityHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_subdomains: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nosniff: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preload: std::option::Option<terrars::PrimField<bool>>,
}

impl ZoneSettingsOverrideSettingsElSecurityHeaderEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_subdomains`.\n"]
    pub fn set_include_subdomains(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_subdomains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nosniff`.\n"]
    pub fn set_nosniff(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nosniff = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preload`.\n"]
    pub fn set_preload(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preload = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ZoneSettingsOverrideSettingsElSecurityHeaderEl {
    type O = terrars::BlockAssignable<ZoneSettingsOverrideSettingsElSecurityHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSettingsOverrideSettingsElSecurityHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ZoneSettingsOverrideSettingsElSecurityHeaderElRef {
        ZoneSettingsOverrideSettingsElSecurityHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideSettingsElSecurityHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `include_subdomains` after provisioning.\n"]
    pub fn include_subdomains(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `nosniff` after provisioning.\n"]
    pub fn nosniff(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nosniff", self.base))
    }

    #[doc = "Get a reference to the value of field `preload` after provisioning.\n"]
    pub fn preload(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preload", self.base))
    }
}

#[derive(Serialize, Default)]
struct ZoneSettingsOverrideSettingsElDynamic {
    minify: std::option::Option<terrars::DynamicBlock<ZoneSettingsOverrideSettingsElMinifyEl>>,
    mobile_redirect: std::option::Option<terrars::DynamicBlock<ZoneSettingsOverrideSettingsElMobileRedirectEl>>,
    security_header: std::option::Option<terrars::DynamicBlock<ZoneSettingsOverrideSettingsElSecurityHeaderEl>>,
}

#[derive(Serialize)]
pub struct ZoneSettingsOverrideSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    always_online: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    always_use_https: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_https_rewrites: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_ast: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    brotli: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_cache_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ciphers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cname_flattening: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    development_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    early_hints: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_obfuscation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_logs_to_cloudflare: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    h2_prioritization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotlink_protection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_resizing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_geolocation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_to_cloudflare: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_upload: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_tls_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mirage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opportunistic_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opportunistic_onion: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orange_to_orange: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_error_page_pass_thru: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_max_http_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    polish: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefetch_preload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privacy_pass: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_read_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pseudo_ipv4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_buffering: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rocket_loader: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_exclude: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_query_string_for_cache: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_1_2_only: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_1_3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_client_auth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    true_client_ip_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    universal_ssl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visitor_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    waf: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    websockets: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zero_rtt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minify: std::option::Option<std::vec::Vec<ZoneSettingsOverrideSettingsElMinifyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mobile_redirect: std::option::Option<std::vec::Vec<ZoneSettingsOverrideSettingsElMobileRedirectEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_header: std::option::Option<std::vec::Vec<ZoneSettingsOverrideSettingsElSecurityHeaderEl>>,
    dynamic: ZoneSettingsOverrideSettingsElDynamic,
}

impl ZoneSettingsOverrideSettingsEl {
    #[doc = "Set the field `always_online`.\n"]
    pub fn set_always_online(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.always_online = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `always_use_https`.\n"]
    pub fn set_always_use_https(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.always_use_https = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_https_rewrites`.\n"]
    pub fn set_automatic_https_rewrites(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.automatic_https_rewrites = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `binary_ast`.\n"]
    pub fn set_binary_ast(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.binary_ast = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `brotli`.\n"]
    pub fn set_brotli(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.brotli = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `browser_cache_ttl`.\n"]
    pub fn set_browser_cache_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.browser_cache_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `browser_check`.\n"]
    pub fn set_browser_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.browser_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_level`.\n"]
    pub fn set_cache_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `challenge_ttl`.\n"]
    pub fn set_challenge_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.challenge_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ciphers`.\n"]
    pub fn set_ciphers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ciphers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cname_flattening`.\n"]
    pub fn set_cname_flattening(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cname_flattening = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `development_mode`.\n"]
    pub fn set_development_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.development_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `early_hints`.\n"]
    pub fn set_early_hints(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.early_hints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_obfuscation`.\n"]
    pub fn set_email_obfuscation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_obfuscation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_logs_to_cloudflare`.\n"]
    pub fn set_filter_logs_to_cloudflare(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filter_logs_to_cloudflare = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `h2_prioritization`.\n"]
    pub fn set_h2_prioritization(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.h2_prioritization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hotlink_protection`.\n"]
    pub fn set_hotlink_protection(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.hotlink_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http3`.\n"]
    pub fn set_http3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_resizing`.\n"]
    pub fn set_image_resizing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_resizing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_geolocation`.\n"]
    pub fn set_ip_geolocation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_geolocation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6`.\n"]
    pub fn set_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_to_cloudflare`.\n"]
    pub fn set_log_to_cloudflare(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_to_cloudflare = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_upload`.\n"]
    pub fn set_max_upload(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_upload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_tls_version`.\n"]
    pub fn set_min_tls_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_tls_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mirage`.\n"]
    pub fn set_mirage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mirage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `opportunistic_encryption`.\n"]
    pub fn set_opportunistic_encryption(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.opportunistic_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `opportunistic_onion`.\n"]
    pub fn set_opportunistic_onion(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.opportunistic_onion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `orange_to_orange`.\n"]
    pub fn set_orange_to_orange(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.orange_to_orange = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_error_page_pass_thru`.\n"]
    pub fn set_origin_error_page_pass_thru(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_error_page_pass_thru = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_max_http_version`.\n"]
    pub fn set_origin_max_http_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_max_http_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `polish`.\n"]
    pub fn set_polish(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.polish = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefetch_preload`.\n"]
    pub fn set_prefetch_preload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefetch_preload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `privacy_pass`.\n"]
    pub fn set_privacy_pass(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.privacy_pass = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_read_timeout`.\n"]
    pub fn set_proxy_read_timeout(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.proxy_read_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pseudo_ipv4`.\n"]
    pub fn set_pseudo_ipv4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pseudo_ipv4 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_buffering`.\n"]
    pub fn set_response_buffering(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.response_buffering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rocket_loader`.\n"]
    pub fn set_rocket_loader(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rocket_loader = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_level`.\n"]
    pub fn set_security_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_exclude`.\n"]
    pub fn set_server_side_exclude(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.server_side_exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_query_string_for_cache`.\n"]
    pub fn set_sort_query_string_for_cache(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sort_query_string_for_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_1_2_only`.\n"]
    pub fn set_tls_1_2_only(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_1_2_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_1_3`.\n"]
    pub fn set_tls_1_3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_1_3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_client_auth`.\n"]
    pub fn set_tls_client_auth(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_client_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `true_client_ip_header`.\n"]
    pub fn set_true_client_ip_header(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.true_client_ip_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `universal_ssl`.\n"]
    pub fn set_universal_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.universal_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visitor_ip`.\n"]
    pub fn set_visitor_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.visitor_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `waf`.\n"]
    pub fn set_waf(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.waf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `webp`.\n"]
    pub fn set_webp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.webp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `websockets`.\n"]
    pub fn set_websockets(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.websockets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zero_rtt`.\n"]
    pub fn set_zero_rtt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zero_rtt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minify`.\n"]
    pub fn set_minify(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ZoneSettingsOverrideSettingsElMinifyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.minify = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.minify = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mobile_redirect`.\n"]
    pub fn set_mobile_redirect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ZoneSettingsOverrideSettingsElMobileRedirectEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mobile_redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mobile_redirect = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_header`.\n"]
    pub fn set_security_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ZoneSettingsOverrideSettingsElSecurityHeaderEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ZoneSettingsOverrideSettingsEl {
    type O = terrars::BlockAssignable<ZoneSettingsOverrideSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ZoneSettingsOverrideSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZoneSettingsOverrideSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ZoneSettingsOverrideSettingsElRef {
        ZoneSettingsOverrideSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneSettingsOverrideSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `always_online` after provisioning.\n"]
    pub fn always_online(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.always_online", self.base))
    }

    #[doc = "Get a reference to the value of field `always_use_https` after provisioning.\n"]
    pub fn always_use_https(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.always_use_https", self.base))
    }

    #[doc = "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\n"]
    pub fn automatic_https_rewrites(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc = "Get a reference to the value of field `binary_ast` after provisioning.\n"]
    pub fn binary_ast(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.binary_ast", self.base))
    }

    #[doc = "Get a reference to the value of field `brotli` after provisioning.\n"]
    pub fn brotli(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.brotli", self.base))
    }

    #[doc = "Get a reference to the value of field `browser_cache_ttl` after provisioning.\n"]
    pub fn browser_cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_cache_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `browser_check` after provisioning.\n"]
    pub fn browser_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_check", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_level` after provisioning.\n"]
    pub fn cache_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_level", self.base))
    }

    #[doc = "Get a reference to the value of field `challenge_ttl` after provisioning.\n"]
    pub fn challenge_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.challenge_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `ciphers` after provisioning.\n"]
    pub fn ciphers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ciphers", self.base))
    }

    #[doc = "Get a reference to the value of field `cname_flattening` after provisioning.\n"]
    pub fn cname_flattening(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname_flattening", self.base))
    }

    #[doc = "Get a reference to the value of field `development_mode` after provisioning.\n"]
    pub fn development_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.development_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `early_hints` after provisioning.\n"]
    pub fn early_hints(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.early_hints", self.base))
    }

    #[doc = "Get a reference to the value of field `email_obfuscation` after provisioning.\n"]
    pub fn email_obfuscation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_logs_to_cloudflare` after provisioning.\n"]
    pub fn filter_logs_to_cloudflare(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_logs_to_cloudflare", self.base))
    }

    #[doc = "Get a reference to the value of field `h2_prioritization` after provisioning.\n"]
    pub fn h2_prioritization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.h2_prioritization", self.base))
    }

    #[doc = "Get a reference to the value of field `hotlink_protection` after provisioning.\n"]
    pub fn hotlink_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hotlink_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc = "Get a reference to the value of field `http3` after provisioning.\n"]
    pub fn http3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http3", self.base))
    }

    #[doc = "Get a reference to the value of field `image_resizing` after provisioning.\n"]
    pub fn image_resizing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_resizing", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_geolocation` after provisioning.\n"]
    pub fn ip_geolocation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_geolocation", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `log_to_cloudflare` after provisioning.\n"]
    pub fn log_to_cloudflare(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_to_cloudflare", self.base))
    }

    #[doc = "Get a reference to the value of field `max_upload` after provisioning.\n"]
    pub fn max_upload(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_upload", self.base))
    }

    #[doc = "Get a reference to the value of field `min_tls_version` after provisioning.\n"]
    pub fn min_tls_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_tls_version", self.base))
    }

    #[doc = "Get a reference to the value of field `mirage` after provisioning.\n"]
    pub fn mirage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc = "Get a reference to the value of field `opportunistic_encryption` after provisioning.\n"]
    pub fn opportunistic_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `opportunistic_onion` after provisioning.\n"]
    pub fn opportunistic_onion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_onion", self.base))
    }

    #[doc = "Get a reference to the value of field `orange_to_orange` after provisioning.\n"]
    pub fn orange_to_orange(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.orange_to_orange", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_error_page_pass_thru` after provisioning.\n"]
    pub fn origin_error_page_pass_thru(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_pass_thru", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_max_http_version` after provisioning.\n"]
    pub fn origin_max_http_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_max_http_version", self.base))
    }

    #[doc = "Get a reference to the value of field `polish` after provisioning.\n"]
    pub fn polish(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc = "Get a reference to the value of field `prefetch_preload` after provisioning.\n"]
    pub fn prefetch_preload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefetch_preload", self.base))
    }

    #[doc = "Get a reference to the value of field `privacy_pass` after provisioning.\n"]
    pub fn privacy_pass(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.privacy_pass", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_read_timeout` after provisioning.\n"]
    pub fn proxy_read_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_read_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `pseudo_ipv4` after provisioning.\n"]
    pub fn pseudo_ipv4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pseudo_ipv4", self.base))
    }

    #[doc = "Get a reference to the value of field `response_buffering` after provisioning.\n"]
    pub fn response_buffering(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_buffering", self.base))
    }

    #[doc = "Get a reference to the value of field `rocket_loader` after provisioning.\n"]
    pub fn rocket_loader(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rocket_loader", self.base))
    }

    #[doc = "Get a reference to the value of field `security_level` after provisioning.\n"]
    pub fn security_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc = "Get a reference to the value of field `server_side_exclude` after provisioning.\n"]
    pub fn server_side_exclude(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_side_exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_query_string_for_cache` after provisioning.\n"]
    pub fn sort_query_string_for_cache(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_query_string_for_cache", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_1_2_only` after provisioning.\n"]
    pub fn tls_1_2_only(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_1_2_only", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_1_3` after provisioning.\n"]
    pub fn tls_1_3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_1_3", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_client_auth` after provisioning.\n"]
    pub fn tls_client_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_client_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `true_client_ip_header` after provisioning.\n"]
    pub fn true_client_ip_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.true_client_ip_header", self.base))
    }

    #[doc = "Get a reference to the value of field `universal_ssl` after provisioning.\n"]
    pub fn universal_ssl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.universal_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `visitor_ip` after provisioning.\n"]
    pub fn visitor_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visitor_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `waf` after provisioning.\n"]
    pub fn waf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waf", self.base))
    }

    #[doc = "Get a reference to the value of field `webp` after provisioning.\n"]
    pub fn webp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webp", self.base))
    }

    #[doc = "Get a reference to the value of field `websockets` after provisioning.\n"]
    pub fn websockets(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.websockets", self.base))
    }

    #[doc = "Get a reference to the value of field `zero_rtt` after provisioning.\n"]
    pub fn zero_rtt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zero_rtt", self.base))
    }

    #[doc = "Get a reference to the value of field `minify` after provisioning.\n"]
    pub fn minify(&self) -> terrars::ListRef<ZoneSettingsOverrideSettingsElMinifyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.minify", self.base))
    }

    #[doc = "Get a reference to the value of field `mobile_redirect` after provisioning.\n"]
    pub fn mobile_redirect(&self) -> terrars::ListRef<ZoneSettingsOverrideSettingsElMobileRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mobile_redirect", self.base))
    }

    #[doc = "Get a reference to the value of field `security_header` after provisioning.\n"]
    pub fn security_header(&self) -> terrars::ListRef<ZoneSettingsOverrideSettingsElSecurityHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_header", self.base))
    }
}

#[derive(Serialize, Default)]
struct ZoneSettingsOverrideDynamic {
    settings: std::option::Option<terrars::DynamicBlock<ZoneSettingsOverrideSettingsEl>>,
}
