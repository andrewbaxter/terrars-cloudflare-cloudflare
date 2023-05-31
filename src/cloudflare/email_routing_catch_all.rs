use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct EmailRoutingCatchAllData {
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
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<EmailRoutingCatchAllActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matcher: Option<Vec<EmailRoutingCatchAllMatcherEl>>,
    dynamic: EmailRoutingCatchAllDynamic,
}

struct EmailRoutingCatchAll_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmailRoutingCatchAllData>,
}

#[derive(Clone)]
pub struct EmailRoutingCatchAll(Rc<EmailRoutingCatchAll_>);

impl EmailRoutingCatchAll {
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

    #[doc= "Set the field `action`.\n"]
    pub fn set_action(self, v: impl Into<BlockAssignable<EmailRoutingCatchAllActionEl>>) -> Self {
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
    pub fn set_matcher(self, v: impl Into<BlockAssignable<EmailRoutingCatchAllMatcherEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nRouting rule identifier."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for EmailRoutingCatchAll {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EmailRoutingCatchAll { }

impl ToListMappable for EmailRoutingCatchAll {
    type O = ListRef<EmailRoutingCatchAllRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmailRoutingCatchAll_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_email_routing_catch_all".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmailRoutingCatchAll {
    pub tf_id: String,
    #[doc= "Routing rule name."]
    pub name: PrimField<String>,
    #[doc= "The zone identifier to target for the resource."]
    pub zone_id: PrimField<String>,
}

impl BuildEmailRoutingCatchAll {
    pub fn build(self, stack: &mut Stack) -> EmailRoutingCatchAll {
        let out = EmailRoutingCatchAll(Rc::new(EmailRoutingCatchAll_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmailRoutingCatchAllData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
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

pub struct EmailRoutingCatchAllRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmailRoutingCatchAllRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmailRoutingCatchAllRef {
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
pub struct EmailRoutingCatchAllActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: ListField<PrimField<String>>,
}

impl EmailRoutingCatchAllActionEl { }

impl ToListMappable for EmailRoutingCatchAllActionEl {
    type O = BlockAssignable<EmailRoutingCatchAllActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmailRoutingCatchAllActionEl {
    #[doc= "Type of supported action. Available values: `drop`, `forward`, `worker`."]
    pub type_: PrimField<String>,
    #[doc= "A list with items in the following form."]
    pub value: ListField<PrimField<String>>,
}

impl BuildEmailRoutingCatchAllActionEl {
    pub fn build(self) -> EmailRoutingCatchAllActionEl {
        EmailRoutingCatchAllActionEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct EmailRoutingCatchAllActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmailRoutingCatchAllActionElRef {
    fn new(shared: StackShared, base: String) -> EmailRoutingCatchAllActionElRef {
        EmailRoutingCatchAllActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmailRoutingCatchAllActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nType of supported action. Available values: `drop`, `forward`, `worker`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nA list with items in the following form."]
    pub fn value(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct EmailRoutingCatchAllMatcherEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl EmailRoutingCatchAllMatcherEl { }

impl ToListMappable for EmailRoutingCatchAllMatcherEl {
    type O = BlockAssignable<EmailRoutingCatchAllMatcherEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmailRoutingCatchAllMatcherEl {
    #[doc= "Type of matcher. Available values: `all`."]
    pub type_: PrimField<String>,
}

impl BuildEmailRoutingCatchAllMatcherEl {
    pub fn build(self) -> EmailRoutingCatchAllMatcherEl {
        EmailRoutingCatchAllMatcherEl { type_: self.type_ }
    }
}

pub struct EmailRoutingCatchAllMatcherElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmailRoutingCatchAllMatcherElRef {
    fn new(shared: StackShared, base: String) -> EmailRoutingCatchAllMatcherElRef {
        EmailRoutingCatchAllMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmailRoutingCatchAllMatcherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nType of matcher. Available values: `all`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmailRoutingCatchAllDynamic {
    action: Option<DynamicBlock<EmailRoutingCatchAllActionEl>>,
    matcher: Option<DynamicBlock<EmailRoutingCatchAllMatcherEl>>,
}
