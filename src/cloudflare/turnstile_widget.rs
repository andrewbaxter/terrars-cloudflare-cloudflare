#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TurnstileWidgetData_ {
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
    bot_fight_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clearance_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    domains: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_id: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    mode: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offlabel: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_page: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct TurnstileWidget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TurnstileWidgetData_>,
}

#[derive(Clone)]
pub struct TurnstileWidget(std::rc::Rc<TurnstileWidget_>);

impl TurnstileWidget {
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
        "Set the field `bot_fight_mode`.\nIf bot_fight_mode is set to `true`, Cloudflare issues computationally\nexpensive challenges in response to malicious bots (ENT only)."]
    pub fn set_bot_fight_mode(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bot_fight_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `clearance_level`.\nIf Turnstile is embedded on a Cloudflare site and the widget should grant challenge clearance,\nthis setting can determine the clearance level to be set\nAvailable values: \"no_clearance\", \"jschallenge\", \"managed\", \"interactive\"."]
    pub fn set_clearance_level(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().clearance_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\nDirection to order widgets.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ephemeral_id`.\nReturn the Ephemeral ID in /siteverify (ENT only)."]
    pub fn set_ephemeral_id(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ephemeral_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nFilter widgets by field. The `name` field uses case-insensitive\nsubstring matching; `sitekey` uses exact matching.\nFormat: `field:value`\n\nSupported fields:\n- `name` - Filter by widget name (e.g., `filter=name:login-form`)\n- `sitekey` - Filter by sitekey (e.g., `filter=sitekey:0x4AAA`)\n\nReturns 400 Bad Request if the field is unsupported or format is invalid.\nAn empty filter value returns all results."]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `offlabel`.\nDo not show any Cloudflare branding on the widget (ENT only)."]
    pub fn set_offlabel(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().offlabel = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nField to order widgets by.\nAvailable values: \"id\", \"sitekey\", \"name\", \"created_on\", \"modified_on\"."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `page`.\nPage number of paginated results."]
    pub fn set_page(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_page`.\nNumber of items per page."]
    pub fn set_per_page(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().per_page = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this widget can be used. This cannot be changed after creation.\nAvailable values: \"world\", \"china\"."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bot_fight_mode` after provisioning.\nIf bot_fight_mode is set to `true`, Cloudflare issues computationally\nexpensive challenges in response to malicious bots (ENT only)."]
    pub fn bot_fight_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_fight_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `clearance_level` after provisioning.\nIf Turnstile is embedded on a Cloudflare site and the widget should grant challenge clearance,\nthis setting can determine the clearance level to be set\nAvailable values: \"no_clearance\", \"jschallenge\", \"managed\", \"interactive\"."]
    pub fn clearance_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clearance_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the widget was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_via` after provisioning.\nOrigin that created this widget, recorded at creation time and\nimmutable afterward. Server-derived from the create request; not\nclient-settable. Omitted from the response for widgets created\nbefore this field existed.\nAvailable values: \"wrangler\", \"dashboard\", \"spin\", \"api\", \"unknown\"."]
    pub fn deployed_via(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_via", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order widgets.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ephemeral_id` after provisioning.\nReturn the Ephemeral ID in /siteverify (ENT only)."]
    pub fn ephemeral_id(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ephemeral_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter widgets by field. The `name` field uses case-insensitive\nsubstring matching; `sitekey` uses exact matching.\nFormat: `field:value`\n\nSupported fields:\n- `name` - Filter by widget name (e.g., `filter=name:login-form`)\n- `sitekey` - Filter by sitekey (e.g., `filter=sitekey:0x4AAA`)\n\nReturns 400 Bad Request if the field is unsupported or format is invalid.\nAn empty filter value returns all results."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nWidget item identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_via` after provisioning.\nOrigin of the most recent mutation (create, update, delete, or\nsecret rotation). Server-derived; not client-settable. Omitted for\nwidgets last mutated before this field existed.\nAvailable values: \"wrangler\", \"dashboard\", \"spin\", \"api\", \"unknown\"."]
    pub fn last_modified_via(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_via", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nWidget Mode\nAvailable values: \"non-interactive\", \"invisible\", \"managed\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\nWhen the widget was modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nHuman readable widget name. Not unique. Cloudflare suggests that you\nset this to a meaningful string to make it easier to identify your\nwidget, and where it is used."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `offlabel` after provisioning.\nDo not show any Cloudflare branding on the widget (ENT only)."]
    pub fn offlabel(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offlabel", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order widgets by.\nAvailable values: \"id\", \"sitekey\", \"name\", \"created_on\", \"modified_on\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `page` after provisioning.\nPage number of paginated results."]
    pub fn page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nNumber of items per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this widget can be used. This cannot be changed after creation.\nAvailable values: \"world\", \"china\"."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\nSecret key for this widget."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sitekey` after provisioning.\nWidget item identifier tag."]
    pub fn sitekey(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sitekey", self.extract_ref()))
    }
}

impl terrars::Referable for TurnstileWidget {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TurnstileWidget { }

impl terrars::ToListMappable for TurnstileWidget {
    type O = terrars::ListRef<TurnstileWidgetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TurnstileWidget_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_turnstile_widget".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTurnstileWidget {
    pub tf_id: std::string::String,
    #[doc = "Identifier"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub domains: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Widget Mode\nAvailable values: \"non-interactive\", \"invisible\", \"managed\"."]
    pub mode: terrars::PrimField<std::string::String>,
    #[doc =
        "Human readable widget name. Not unique. Cloudflare suggests that you\nset this to a meaningful string to make it easier to identify your\nwidget, and where it is used."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildTurnstileWidget {
    pub fn build(self, stack: &mut terrars::Stack) -> TurnstileWidget {
        let out = TurnstileWidget(std::rc::Rc::new(TurnstileWidget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TurnstileWidgetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                bot_fight_mode: core::default::Default::default(),
                clearance_level: core::default::Default::default(),
                direction: core::default::Default::default(),
                domains: self.domains,
                ephemeral_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                mode: self.mode,
                name: self.name,
                offlabel: core::default::Default::default(),
                order: core::default::Default::default(),
                page: core::default::Default::default(),
                per_page: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TurnstileWidgetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TurnstileWidgetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TurnstileWidgetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bot_fight_mode` after provisioning.\nIf bot_fight_mode is set to `true`, Cloudflare issues computationally\nexpensive challenges in response to malicious bots (ENT only)."]
    pub fn bot_fight_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_fight_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `clearance_level` after provisioning.\nIf Turnstile is embedded on a Cloudflare site and the widget should grant challenge clearance,\nthis setting can determine the clearance level to be set\nAvailable values: \"no_clearance\", \"jschallenge\", \"managed\", \"interactive\"."]
    pub fn clearance_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clearance_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\nWhen the widget was created."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_via` after provisioning.\nOrigin that created this widget, recorded at creation time and\nimmutable afterward. Server-derived from the create request; not\nclient-settable. Omitted from the response for widgets created\nbefore this field existed.\nAvailable values: \"wrangler\", \"dashboard\", \"spin\", \"api\", \"unknown\"."]
    pub fn deployed_via(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_via", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order widgets.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ephemeral_id` after provisioning.\nReturn the Ephemeral ID in /siteverify (ENT only)."]
    pub fn ephemeral_id(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ephemeral_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter widgets by field. The `name` field uses case-insensitive\nsubstring matching; `sitekey` uses exact matching.\nFormat: `field:value`\n\nSupported fields:\n- `name` - Filter by widget name (e.g., `filter=name:login-form`)\n- `sitekey` - Filter by sitekey (e.g., `filter=sitekey:0x4AAA`)\n\nReturns 400 Bad Request if the field is unsupported or format is invalid.\nAn empty filter value returns all results."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nWidget item identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_via` after provisioning.\nOrigin of the most recent mutation (create, update, delete, or\nsecret rotation). Server-derived; not client-settable. Omitted for\nwidgets last mutated before this field existed.\nAvailable values: \"wrangler\", \"dashboard\", \"spin\", \"api\", \"unknown\"."]
    pub fn last_modified_via(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_via", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nWidget Mode\nAvailable values: \"non-interactive\", \"invisible\", \"managed\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_on` after provisioning.\nWhen the widget was modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nHuman readable widget name. Not unique. Cloudflare suggests that you\nset this to a meaningful string to make it easier to identify your\nwidget, and where it is used."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `offlabel` after provisioning.\nDo not show any Cloudflare branding on the widget (ENT only)."]
    pub fn offlabel(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offlabel", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order widgets by.\nAvailable values: \"id\", \"sitekey\", \"name\", \"created_on\", \"modified_on\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `page` after provisioning.\nPage number of paginated results."]
    pub fn page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `per_page` after provisioning.\nNumber of items per page."]
    pub fn per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this widget can be used. This cannot be changed after creation.\nAvailable values: \"world\", \"china\"."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\nSecret key for this widget."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sitekey` after provisioning.\nWidget item identifier tag."]
    pub fn sitekey(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sitekey", self.extract_ref()))
    }
}
