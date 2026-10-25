#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataTurnstileWidgetData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataTurnstileWidget_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sitekey: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataTurnstileWidget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataTurnstileWidgetData_>,
}

#[derive(Clone)]
pub struct DataTurnstileWidget(std::rc::Rc<DataTurnstileWidget_>);

impl DataTurnstileWidget {
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

    #[doc = "Set the field `account_id`.\nIdentifier"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataTurnstileWidget_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sitekey`.\nUnique identifier for a Turnstile widget."]
    pub fn set_sitekey(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sitekey = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ephemeral_id` after provisioning.\nReturn the Ephemeral ID in /siteverify (ENT only)."]
    pub fn ephemeral_id(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ephemeral_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataTurnstileWidget_FilterRef {
        DataTurnstileWidget_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for a Turnstile widget."]
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
        "Get a reference to the value of field `region` after provisioning.\nRegion where this widget can be used. This cannot be changed after creation.\nAvailable values: \"world\", \"china\"."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\nSecret key for this widget."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sitekey` after provisioning.\nUnique identifier for a Turnstile widget."]
    pub fn sitekey(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sitekey", self.extract_ref()))
    }
}

impl terrars::Referable for DataTurnstileWidget {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataTurnstileWidget { }

impl terrars::ToListMappable for DataTurnstileWidget {
    type O = terrars::ListRef<DataTurnstileWidgetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataTurnstileWidget_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_turnstile_widget".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTurnstileWidget {
    pub tf_id: std::string::String,
}

impl BuildDataTurnstileWidget {
    pub fn build(self, stack: &mut terrars::Stack) -> DataTurnstileWidget {
        let out = DataTurnstileWidget(std::rc::Rc::new(DataTurnstileWidget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataTurnstileWidgetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                sitekey: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTurnstileWidgetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTurnstileWidgetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataTurnstileWidgetRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ephemeral_id` after provisioning.\nReturn the Ephemeral ID in /siteverify (ENT only)."]
    pub fn ephemeral_id(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ephemeral_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataTurnstileWidget_FilterRef {
        DataTurnstileWidget_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for a Turnstile widget."]
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
        "Get a reference to the value of field `region` after provisioning.\nRegion where this widget can be used. This cannot be changed after creation.\nAvailable values: \"world\", \"china\"."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\nSecret key for this widget."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sitekey` after provisioning.\nUnique identifier for a Turnstile widget."]
    pub fn sitekey(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sitekey", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataTurnstileWidget_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataTurnstileWidget_Filter {
    #[doc = "Set the field `direction`.\nDirection to order widgets.\nAvailable values: \"asc\", \"desc\"."]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nFilter widgets by field. The `name` field uses case-insensitive\nsubstring matching; `sitekey` uses exact matching.\nFormat: `field:value`\n\nSupported fields:\n- `name` - Filter by widget name (e.g., `filter=name:login-form`)\n- `sitekey` - Filter by sitekey (e.g., `filter=sitekey:0x4AAA`)\n\nReturns 400 Bad Request if the field is unsupported or format is invalid.\nAn empty filter value returns all results."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nField to order widgets by.\nAvailable values: \"id\", \"sitekey\", \"name\", \"created_on\", \"modified_on\"."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataTurnstileWidget_Filter {
    type O = terrars::BlockAssignable<DataTurnstileWidget_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTurnstileWidget_Filter {}

impl BuildDataTurnstileWidget_Filter {
    pub fn build(self) -> DataTurnstileWidget_Filter {
        DataTurnstileWidget_Filter {
            direction: core::default::Default::default(),
            filter: core::default::Default::default(),
            order: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataTurnstileWidget_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTurnstileWidget_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataTurnstileWidget_FilterRef {
        DataTurnstileWidget_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTurnstileWidget_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nDirection to order widgets.\nAvailable values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter widgets by field. The `name` field uses case-insensitive\nsubstring matching; `sitekey` uses exact matching.\nFormat: `field:value`\n\nSupported fields:\n- `name` - Filter by widget name (e.g., `filter=name:login-form`)\n- `sitekey` - Filter by sitekey (e.g., `filter=sitekey:0x4AAA`)\n\nReturns 400 Bad Request if the field is unsupported or format is invalid.\nAn empty filter value returns all results."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nField to order widgets by.\nAvailable values: \"id\", \"sitekey\", \"name\", \"created_on\", \"modified_on\"."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
}
