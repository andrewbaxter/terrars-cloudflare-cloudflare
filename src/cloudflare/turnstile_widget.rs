use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct TurnstileWidgetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_fight_mode: Option<PrimField<bool>>,
    domains: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    mode: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offlabel: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct TurnstileWidget_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TurnstileWidgetData>,
}

#[derive(Clone)]
pub struct TurnstileWidget(Rc<TurnstileWidget_>);

impl TurnstileWidget {
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

    #[doc= "Set the field `bot_fight_mode`.\nIf bot_fight_mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only)."]
    pub fn set_bot_fight_mode(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bot_fight_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\nThe identifier of this resource. This is the site key value."]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `offlabel`.\nDo not show any Cloudflare branding on the widget (Enterprise only)."]
    pub fn set_offlabel(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().offlabel = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nRegion where this widget can be used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bot_fight_mode` after provisioning.\nIf bot_fight_mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only)."]
    pub fn bot_fight_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bot_fight_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domains` after provisioning.\nDomains where the widget is deployed"]
    pub fn domains(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe identifier of this resource. This is the site key value."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nWidget Mode. Available values: `non-interactive`, `invisible`, `managed`"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nHuman readable widget name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `offlabel` after provisioning.\nDo not show any Cloudflare branding on the widget (Enterprise only)."]
    pub fn offlabel(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.offlabel", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nRegion where this widget can be used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\nSecret key for this widget."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }
}

impl Referable for TurnstileWidget {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TurnstileWidget { }

impl ToListMappable for TurnstileWidget {
    type O = ListRef<TurnstileWidgetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TurnstileWidget_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_turnstile_widget".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTurnstileWidget {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "Domains where the widget is deployed"]
    pub domains: SetField<PrimField<String>>,
    #[doc= "Widget Mode. Available values: `non-interactive`, `invisible`, `managed`"]
    pub mode: PrimField<String>,
    #[doc= "Human readable widget name."]
    pub name: PrimField<String>,
}

impl BuildTurnstileWidget {
    pub fn build(self, stack: &mut Stack) -> TurnstileWidget {
        let out = TurnstileWidget(Rc::new(TurnstileWidget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TurnstileWidgetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                bot_fight_mode: core::default::Default::default(),
                domains: self.domains,
                id: core::default::Default::default(),
                mode: self.mode,
                name: self.name,
                offlabel: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TurnstileWidgetRef {
    shared: StackShared,
    base: String,
}

impl Ref for TurnstileWidgetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TurnstileWidgetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bot_fight_mode` after provisioning.\nIf bot_fight_mode is set to true, Cloudflare issues computationally expensive challenges in response to malicious bots (Enterprise only)."]
    pub fn bot_fight_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bot_fight_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domains` after provisioning.\nDomains where the widget is deployed"]
    pub fn domains(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe identifier of this resource. This is the site key value."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nWidget Mode. Available values: `non-interactive`, `invisible`, `managed`"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nHuman readable widget name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `offlabel` after provisioning.\nDo not show any Cloudflare branding on the widget (Enterprise only)."]
    pub fn offlabel(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.offlabel", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nRegion where this widget can be used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\nSecret key for this widget."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }
}
