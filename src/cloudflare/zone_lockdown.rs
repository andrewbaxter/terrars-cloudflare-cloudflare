use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZoneLockdownData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paused: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    urls: SetField<PrimField<String>>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configurations: Option<Vec<ZoneLockdownConfigurationsEl>>,
    dynamic: ZoneLockdownDynamic,
}

struct ZoneLockdown_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ZoneLockdownData>,
}

#[derive(Clone)]
pub struct ZoneLockdown(Rc<ZoneLockdown_>);

impl ZoneLockdown {
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

    #[doc= "Set the field `description`.\nA description about the lockdown entry. Typically used as a reminder or explanation for the lockdown."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `paused`.\nBoolean of whether this zone lockdown is currently paused. Defaults to `false`."]
    pub fn set_paused(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().paused = Some(v.into());
        self
    }

    #[doc= "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }

    #[doc= "Set the field `configurations`.\n"]
    pub fn set_configurations(self, v: impl Into<BlockAssignable<ZoneLockdownConfigurationsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configurations = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configurations = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description about the lockdown entry. Typically used as a reminder or explanation for the lockdown."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nBoolean of whether this zone lockdown is currently paused. Defaults to `false`."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urls` after provisioning.\nA list of simple wildcard patterns to match requests against. The order of the urls is unimportant."]
    pub fn urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl Referable for ZoneLockdown {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ZoneLockdown { }

impl ToListMappable for ZoneLockdown {
    type O = ListRef<ZoneLockdownRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ZoneLockdown_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_zone_lockdown".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZoneLockdown {
    pub tf_id: String,
    #[doc= "A list of simple wildcard patterns to match requests against. The order of the urls is unimportant."]
    pub urls: SetField<PrimField<String>>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildZoneLockdown {
    pub fn build(self, stack: &mut Stack) -> ZoneLockdown {
        let out = ZoneLockdown(Rc::new(ZoneLockdown_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ZoneLockdownData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                paused: core::default::Default::default(),
                priority: core::default::Default::default(),
                urls: self.urls,
                zone_id: self.zone_id,
                configurations: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZoneLockdownRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneLockdownRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZoneLockdownRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description about the lockdown entry. Typically used as a reminder or explanation for the lockdown."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nBoolean of whether this zone lockdown is currently paused. Defaults to `false`."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urls` after provisioning.\nA list of simple wildcard patterns to match requests against. The order of the urls is unimportant."]
    pub fn urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ZoneLockdownConfigurationsEl {
    target: PrimField<String>,
    value: PrimField<String>,
}

impl ZoneLockdownConfigurationsEl { }

impl ToListMappable for ZoneLockdownConfigurationsEl {
    type O = BlockAssignable<ZoneLockdownConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildZoneLockdownConfigurationsEl {
    #[doc= "The request property to target. Available values: `ip`, `ip_range`."]
    pub target: PrimField<String>,
    #[doc= "The value to target. Depends on target's type. IP addresses should just be standard IPv4/IPv6 notation i.e. `192.0.2.1` or `2001:db8::/32` and IP ranges in CIDR format i.e. `192.0.2.0/24`."]
    pub value: PrimField<String>,
}

impl BuildZoneLockdownConfigurationsEl {
    pub fn build(self) -> ZoneLockdownConfigurationsEl {
        ZoneLockdownConfigurationsEl {
            target: self.target,
            value: self.value,
        }
    }
}

pub struct ZoneLockdownConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ZoneLockdownConfigurationsElRef {
    fn new(shared: StackShared, base: String) -> ZoneLockdownConfigurationsElRef {
        ZoneLockdownConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ZoneLockdownConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nThe request property to target. Available values: `ip`, `ip_range`."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value to target. Depends on target's type. IP addresses should just be standard IPv4/IPv6 notation i.e. `192.0.2.1` or `2001:db8::/32` and IP ranges in CIDR format i.e. `192.0.2.0/24`."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ZoneLockdownDynamic {
    configurations: Option<DynamicBlock<ZoneLockdownConfigurationsEl>>,
}
