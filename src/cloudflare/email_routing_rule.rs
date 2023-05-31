use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct EmailRoutingRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<EmailRoutingRuleActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matcher: Option<Vec<EmailRoutingRuleMatcherEl>>,
    dynamic: EmailRoutingRuleDynamic,
}

struct EmailRoutingRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmailRoutingRuleData>,
}

#[derive(Clone)]
pub struct EmailRoutingRule(Rc<EmailRoutingRule_>);

impl EmailRoutingRule {
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

    #[doc= "Set the field `enabled`.\nRouting rule status."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `priority`.\nPriority of the routing rule."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }

    #[doc= "Set the field `action`.\n"]
    pub fn set_action(self, v: impl Into<BlockAssignable<EmailRoutingRuleActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `matcher`.\n"]
    pub fn set_matcher(self, v: impl Into<BlockAssignable<EmailRoutingRuleMatcherEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().matcher = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.matcher = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nRouting rule status."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nRouting rule name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nPriority of the routing rule."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nRouting rule identifier."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for EmailRoutingRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EmailRoutingRule { }

impl ToListMappable for EmailRoutingRule {
    type O = ListRef<EmailRoutingRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmailRoutingRule_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_email_routing_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmailRoutingRule {
    pub tf_id: String,
    #[doc= "Routing rule name."]
    pub name: PrimField<String>,
    #[doc= "The zone identifier to target for the resource."]
    pub zone_id: PrimField<String>,
}

impl BuildEmailRoutingRule {
    pub fn build(self, stack: &mut Stack) -> EmailRoutingRule {
        let out = EmailRoutingRule(Rc::new(EmailRoutingRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmailRoutingRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                priority: core::default::Default::default(),
                zone_id: self.zone_id,
                action: core::default::Default::default(),
                matcher: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmailRoutingRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmailRoutingRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmailRoutingRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nRouting rule status."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nRouting rule name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nPriority of the routing rule."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nRouting rule identifier."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EmailRoutingRuleActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: ListField<PrimField<String>>,
}

impl EmailRoutingRuleActionEl { }

impl ToListMappable for EmailRoutingRuleActionEl {
    type O = BlockAssignable<EmailRoutingRuleActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmailRoutingRuleActionEl {
    #[doc= "Type of supported action."]
    pub type_: PrimField<String>,
    #[doc= "An array with items in the following form."]
    pub value: ListField<PrimField<String>>,
}

impl BuildEmailRoutingRuleActionEl {
    pub fn build(self) -> EmailRoutingRuleActionEl {
        EmailRoutingRuleActionEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct EmailRoutingRuleActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmailRoutingRuleActionElRef {
    fn new(shared: StackShared, base: String) -> EmailRoutingRuleActionElRef {
        EmailRoutingRuleActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmailRoutingRuleActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nType of supported action."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nAn array with items in the following form."]
    pub fn value(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct EmailRoutingRuleMatcherEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl EmailRoutingRuleMatcherEl {
    #[doc= "Set the field `field`.\nField for type matcher."]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nValue for matcher."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for EmailRoutingRuleMatcherEl {
    type O = BlockAssignable<EmailRoutingRuleMatcherEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmailRoutingRuleMatcherEl {
    #[doc= "Type of matcher."]
    pub type_: PrimField<String>,
}

impl BuildEmailRoutingRuleMatcherEl {
    pub fn build(self) -> EmailRoutingRuleMatcherEl {
        EmailRoutingRuleMatcherEl {
            field: core::default::Default::default(),
            type_: self.type_,
            value: core::default::Default::default(),
        }
    }
}

pub struct EmailRoutingRuleMatcherElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmailRoutingRuleMatcherElRef {
    fn new(shared: StackShared, base: String) -> EmailRoutingRuleMatcherElRef {
        EmailRoutingRuleMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmailRoutingRuleMatcherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `field` after provisioning.\nField for type matcher."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nType of matcher."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue for matcher."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmailRoutingRuleDynamic {
    action: Option<DynamicBlock<EmailRoutingRuleActionEl>>,
    matcher: Option<DynamicBlock<EmailRoutingRuleMatcherEl>>,
}
