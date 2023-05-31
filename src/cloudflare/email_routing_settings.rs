use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct EmailRoutingSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_wizard: Option<PrimField<bool>>,
    zone_id: PrimField<String>,
}

struct EmailRoutingSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmailRoutingSettingsData>,
}

#[derive(Clone)]
pub struct EmailRoutingSettings(Rc<EmailRoutingSettings_>);

impl EmailRoutingSettings {
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

    #[doc= "Set the field `skip_wizard`.\nFlag to check if the user skipped the configuration wizard."]
    pub fn set_skip_wizard(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_wizard = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nThe date and time the settings have been created."]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nState of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified` after provisioning.\nThe date and time the settings have been modified."]
    pub fn modified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nDomain of your zone."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_wizard` after provisioning.\nFlag to check if the user skipped the configuration wizard."]
    pub fn skip_wizard(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_wizard", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nShow the state of your account, and the type or configuration error."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nEmail Routing settings identifier."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for EmailRoutingSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EmailRoutingSettings { }

impl ToListMappable for EmailRoutingSettings {
    type O = ListRef<EmailRoutingSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmailRoutingSettings_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_email_routing_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmailRoutingSettings {
    pub tf_id: String,
    #[doc= "State of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**"]
    pub enabled: PrimField<bool>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildEmailRoutingSettings {
    pub fn build(self, stack: &mut Stack) -> EmailRoutingSettings {
        let out = EmailRoutingSettings(Rc::new(EmailRoutingSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmailRoutingSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enabled: self.enabled,
                id: core::default::Default::default(),
                skip_wizard: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmailRoutingSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmailRoutingSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmailRoutingSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nThe date and time the settings have been created."]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nState of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified` after provisioning.\nThe date and time the settings have been modified."]
    pub fn modified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nDomain of your zone."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_wizard` after provisioning.\nFlag to check if the user skipped the configuration wizard."]
    pub fn skip_wizard(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_wizard", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nShow the state of your account, and the type or configuration error."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nEmail Routing settings identifier."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}
