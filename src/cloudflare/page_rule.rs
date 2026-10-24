use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct PageRuleData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<std::string::String>,
    zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<PageRuleActionsEl>>,
    dynamic: PageRuleDynamic,
}

struct PageRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PageRuleData>,
}

#[derive(Clone)]
pub struct PageRule(std::rc::Rc<PageRule_>);

impl PageRule {
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

    #[doc = "Set the field `priority`.\nDefaults to `1`."]
    pub fn set_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\nDefaults to `active`."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(self, v: impl std::convert::Into<terrars::BlockAssignable<PageRuleActionsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\nDefaults to `1`."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nDefaults to `active`."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<PageRuleActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }
}

impl terrars::Referable for PageRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PageRule { }

impl terrars::ToListMappable for PageRule {
    type O = terrars::ListRef<PageRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PageRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_page_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPageRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
    #[doc =
        "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildPageRule {
    pub fn build(self, stack: &mut terrars::Stack) -> PageRule {
        let out = PageRule(std::rc::Rc::new(PageRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PageRuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                priority: core::default::Default::default(),
                status: core::default::Default::default(),
                target: self.target,
                zone_id: self.zone_id,
                actions: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PageRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PageRuleRef {
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

    #[doc = "Get a reference to the value of field `priority` after provisioning.\nDefaults to `1`."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nDefaults to `active`."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<PageRuleActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsElCookieEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_presence: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl PageRuleActionsElCacheKeyFieldsElCookieEl {
    #[doc = "Set the field `check_presence`.\n"]
    pub fn set_check_presence(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.check_presence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\n"]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PageRuleActionsElCacheKeyFieldsElCookieEl {
    type O = terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsElCookieEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElCacheKeyFieldsElCookieEl {}

impl BuildPageRuleActionsElCacheKeyFieldsElCookieEl {
    pub fn build(self) -> PageRuleActionsElCacheKeyFieldsElCookieEl {
        PageRuleActionsElCacheKeyFieldsElCookieEl {
            check_presence: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

pub struct PageRuleActionsElCacheKeyFieldsElCookieElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleActionsElCacheKeyFieldsElCookieElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PageRuleActionsElCacheKeyFieldsElCookieElRef {
        PageRuleActionsElCacheKeyFieldsElCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElCookieElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `check_presence` after provisioning.\n"]
    pub fn check_presence(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsElHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_presence: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl PageRuleActionsElCacheKeyFieldsElHeaderEl {
    #[doc = "Set the field `check_presence`.\n"]
    pub fn set_check_presence(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.check_presence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude`.\n"]
    pub fn set_exclude(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\n"]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PageRuleActionsElCacheKeyFieldsElHeaderEl {
    type O = terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsElHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElCacheKeyFieldsElHeaderEl {}

impl BuildPageRuleActionsElCacheKeyFieldsElHeaderEl {
    pub fn build(self) -> PageRuleActionsElCacheKeyFieldsElHeaderEl {
        PageRuleActionsElCacheKeyFieldsElHeaderEl {
            check_presence: core::default::Default::default(),
            exclude: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

pub struct PageRuleActionsElCacheKeyFieldsElHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleActionsElCacheKeyFieldsElHeaderElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PageRuleActionsElCacheKeyFieldsElHeaderElRef {
        PageRuleActionsElCacheKeyFieldsElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `check_presence` after provisioning.\n"]
    pub fn check_presence(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsElHostEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolved: std::option::Option<terrars::PrimField<bool>>,
}

impl PageRuleActionsElCacheKeyFieldsElHostEl {
    #[doc = "Set the field `resolved`.\nDefaults to `false`."]
    pub fn set_resolved(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.resolved = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PageRuleActionsElCacheKeyFieldsElHostEl {
    type O = terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsElHostEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElCacheKeyFieldsElHostEl {}

impl BuildPageRuleActionsElCacheKeyFieldsElHostEl {
    pub fn build(self) -> PageRuleActionsElCacheKeyFieldsElHostEl {
        PageRuleActionsElCacheKeyFieldsElHostEl { resolved: core::default::Default::default() }
    }
}

pub struct PageRuleActionsElCacheKeyFieldsElHostElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleActionsElCacheKeyFieldsElHostElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PageRuleActionsElCacheKeyFieldsElHostElRef {
        PageRuleActionsElCacheKeyFieldsElHostElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElHostElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolved` after provisioning.\nDefaults to `false`."]
    pub fn resolved(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolved", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsElQueryStringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl PageRuleActionsElCacheKeyFieldsElQueryStringEl {
    #[doc = "Set the field `exclude`.\n"]
    pub fn set_exclude(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore`.\n"]
    pub fn set_ignore(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\n"]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PageRuleActionsElCacheKeyFieldsElQueryStringEl {
    type O = terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsElQueryStringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElCacheKeyFieldsElQueryStringEl {}

impl BuildPageRuleActionsElCacheKeyFieldsElQueryStringEl {
    pub fn build(self) -> PageRuleActionsElCacheKeyFieldsElQueryStringEl {
        PageRuleActionsElCacheKeyFieldsElQueryStringEl {
            exclude: core::default::Default::default(),
            ignore: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

pub struct PageRuleActionsElCacheKeyFieldsElQueryStringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleActionsElCacheKeyFieldsElQueryStringElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PageRuleActionsElCacheKeyFieldsElQueryStringElRef {
        PageRuleActionsElCacheKeyFieldsElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElQueryStringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore` after provisioning.\n"]
    pub fn ignore(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore", self.base))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsElUserEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lang: std::option::Option<terrars::PrimField<bool>>,
}

impl PageRuleActionsElCacheKeyFieldsElUserEl {
    #[doc = "Set the field `device_type`.\n"]
    pub fn set_device_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.device_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.geo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lang`.\n"]
    pub fn set_lang(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.lang = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PageRuleActionsElCacheKeyFieldsElUserEl {
    type O = terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsElUserEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElCacheKeyFieldsElUserEl {}

impl BuildPageRuleActionsElCacheKeyFieldsElUserEl {
    pub fn build(self) -> PageRuleActionsElCacheKeyFieldsElUserEl {
        PageRuleActionsElCacheKeyFieldsElUserEl {
            device_type: core::default::Default::default(),
            geo: core::default::Default::default(),
            lang: core::default::Default::default(),
        }
    }
}

pub struct PageRuleActionsElCacheKeyFieldsElUserElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleActionsElCacheKeyFieldsElUserElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PageRuleActionsElCacheKeyFieldsElUserElRef {
        PageRuleActionsElCacheKeyFieldsElUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElUserElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_type` after provisioning.\n"]
    pub fn device_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }

    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc = "Get a reference to the value of field `lang` after provisioning.\n"]
    pub fn lang(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lang", self.base))
    }
}

#[derive(Serialize, Default)]
struct PageRuleActionsElCacheKeyFieldsElDynamic {
    cookie: std::option::Option<terrars::DynamicBlock<PageRuleActionsElCacheKeyFieldsElCookieEl>>,
    header: std::option::Option<terrars::DynamicBlock<PageRuleActionsElCacheKeyFieldsElHeaderEl>>,
    host: std::option::Option<terrars::DynamicBlock<PageRuleActionsElCacheKeyFieldsElHostEl>>,
    query_string: std::option::Option<terrars::DynamicBlock<PageRuleActionsElCacheKeyFieldsElQueryStringEl>>,
    user: std::option::Option<terrars::DynamicBlock<PageRuleActionsElCacheKeyFieldsElUserEl>>,
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie: std::option::Option<std::vec::Vec<PageRuleActionsElCacheKeyFieldsElCookieEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<PageRuleActionsElCacheKeyFieldsElHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<std::vec::Vec<PageRuleActionsElCacheKeyFieldsElHostEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<std::vec::Vec<PageRuleActionsElCacheKeyFieldsElQueryStringEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<std::vec::Vec<PageRuleActionsElCacheKeyFieldsElUserEl>>,
    dynamic: PageRuleActionsElCacheKeyFieldsElDynamic,
}

impl PageRuleActionsElCacheKeyFieldsEl {
    #[doc = "Set the field `cookie`.\n"]
    pub fn set_cookie(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsElCookieEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cookie = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cookie = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsElHeaderEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `host`.\n"]
    pub fn set_host(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsElHostEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.host = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.host = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsElQueryStringEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsElUserEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PageRuleActionsElCacheKeyFieldsEl {
    type O = terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElCacheKeyFieldsEl {}

impl BuildPageRuleActionsElCacheKeyFieldsEl {
    pub fn build(self) -> PageRuleActionsElCacheKeyFieldsEl {
        PageRuleActionsElCacheKeyFieldsEl {
            cookie: core::default::Default::default(),
            header: core::default::Default::default(),
            host: core::default::Default::default(),
            query_string: core::default::Default::default(),
            user: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PageRuleActionsElCacheKeyFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleActionsElCacheKeyFieldsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PageRuleActionsElCacheKeyFieldsElRef {
        PageRuleActionsElCacheKeyFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie` after provisioning.\n"]
    pub fn cookie(&self) -> terrars::ListRef<PageRuleActionsElCacheKeyFieldsElCookieElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::ListRef<PageRuleActionsElCacheKeyFieldsElHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::ListRef<PageRuleActionsElCacheKeyFieldsElHostElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> terrars::ListRef<PageRuleActionsElCacheKeyFieldsElQueryStringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::ListRef<PageRuleActionsElCacheKeyFieldsElUserElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheTtlByStatusEl {
    codes: terrars::PrimField<std::string::String>,
    ttl: terrars::PrimField<f64>,
}

impl PageRuleActionsElCacheTtlByStatusEl { }

impl terrars::ToListMappable for PageRuleActionsElCacheTtlByStatusEl {
    type O = terrars::BlockAssignable<PageRuleActionsElCacheTtlByStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElCacheTtlByStatusEl {
    #[doc = ""]
    pub codes: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub ttl: terrars::PrimField<f64>,
}

impl BuildPageRuleActionsElCacheTtlByStatusEl {
    pub fn build(self) -> PageRuleActionsElCacheTtlByStatusEl {
        PageRuleActionsElCacheTtlByStatusEl {
            codes: self.codes,
            ttl: self.ttl,
        }
    }
}

pub struct PageRuleActionsElCacheTtlByStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleActionsElCacheTtlByStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PageRuleActionsElCacheTtlByStatusElRef {
        PageRuleActionsElCacheTtlByStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheTtlByStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `codes` after provisioning.\n"]
    pub fn codes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.codes", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElForwardingUrlEl {
    status_code: terrars::PrimField<f64>,
    url: terrars::PrimField<std::string::String>,
}

impl PageRuleActionsElForwardingUrlEl { }

impl terrars::ToListMappable for PageRuleActionsElForwardingUrlEl {
    type O = terrars::BlockAssignable<PageRuleActionsElForwardingUrlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElForwardingUrlEl {
    #[doc = ""]
    pub status_code: terrars::PrimField<f64>,
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildPageRuleActionsElForwardingUrlEl {
    pub fn build(self) -> PageRuleActionsElForwardingUrlEl {
        PageRuleActionsElForwardingUrlEl {
            status_code: self.status_code,
            url: self.url,
        }
    }
}

pub struct PageRuleActionsElForwardingUrlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleActionsElForwardingUrlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PageRuleActionsElForwardingUrlElRef {
        PageRuleActionsElForwardingUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElForwardingUrlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElMinifyEl {
    css: terrars::PrimField<std::string::String>,
    html: terrars::PrimField<std::string::String>,
    js: terrars::PrimField<std::string::String>,
}

impl PageRuleActionsElMinifyEl { }

impl terrars::ToListMappable for PageRuleActionsElMinifyEl {
    type O = terrars::BlockAssignable<PageRuleActionsElMinifyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElMinifyEl {
    #[doc = ""]
    pub css: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub html: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub js: terrars::PrimField<std::string::String>,
}

impl BuildPageRuleActionsElMinifyEl {
    pub fn build(self) -> PageRuleActionsElMinifyEl {
        PageRuleActionsElMinifyEl {
            css: self.css,
            html: self.html,
            js: self.js,
        }
    }
}

pub struct PageRuleActionsElMinifyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleActionsElMinifyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PageRuleActionsElMinifyElRef {
        PageRuleActionsElMinifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElMinifyElRef {
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

#[derive(Serialize, Default)]
struct PageRuleActionsElDynamic {
    cache_key_fields: std::option::Option<terrars::DynamicBlock<PageRuleActionsElCacheKeyFieldsEl>>,
    cache_ttl_by_status: std::option::Option<terrars::DynamicBlock<PageRuleActionsElCacheTtlByStatusEl>>,
    forwarding_url: std::option::Option<terrars::DynamicBlock<PageRuleActionsElForwardingUrlEl>>,
    minify: std::option::Option<terrars::DynamicBlock<PageRuleActionsElMinifyEl>>,
}

#[derive(Serialize)]
pub struct PageRuleActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    always_use_https: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_https_rewrites: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_cache_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bypass_cache_on_cookie: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_by_device_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_deception_armor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_on_cookie: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_apps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_performance: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_railgun: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_security: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_zaraz: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_cache_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_obfuscation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    explicit_cache_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_header_override: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_geolocation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mirage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opportunistic_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_error_page_pass_thru: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    polish: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolve_override: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respect_strong_etag: std::option::Option<terrars::PrimField<std::string::String>>,
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
    true_client_ip_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    waf: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key_fields: std::option::Option<std::vec::Vec<PageRuleActionsElCacheKeyFieldsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_ttl_by_status: std::option::Option<std::vec::Vec<PageRuleActionsElCacheTtlByStatusEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_url: std::option::Option<std::vec::Vec<PageRuleActionsElForwardingUrlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minify: std::option::Option<std::vec::Vec<PageRuleActionsElMinifyEl>>,
    dynamic: PageRuleActionsElDynamic,
}

impl PageRuleActionsEl {
    #[doc = "Set the field `always_use_https`.\nDefaults to `false`."]
    pub fn set_always_use_https(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
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

    #[doc = "Set the field `browser_cache_ttl`.\n"]
    pub fn set_browser_cache_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.browser_cache_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `browser_check`.\n"]
    pub fn set_browser_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.browser_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bypass_cache_on_cookie`.\n"]
    pub fn set_bypass_cache_on_cookie(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bypass_cache_on_cookie = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_by_device_type`.\n"]
    pub fn set_cache_by_device_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cache_by_device_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_deception_armor`.\n"]
    pub fn set_cache_deception_armor(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cache_deception_armor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_level`.\n"]
    pub fn set_cache_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_on_cookie`.\n"]
    pub fn set_cache_on_cookie(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_on_cookie = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_apps`.\nDefaults to `false`."]
    pub fn set_disable_apps(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_apps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_performance`.\nDefaults to `false`."]
    pub fn set_disable_performance(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_performance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_railgun`.\nDefaults to `false`."]
    pub fn set_disable_railgun(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_railgun = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_security`.\nDefaults to `false`."]
    pub fn set_disable_security(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_security = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_zaraz`.\nDefaults to `false`."]
    pub fn set_disable_zaraz(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_zaraz = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edge_cache_ttl`.\n"]
    pub fn set_edge_cache_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.edge_cache_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_obfuscation`.\n"]
    pub fn set_email_obfuscation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_obfuscation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `explicit_cache_control`.\n"]
    pub fn set_explicit_cache_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.explicit_cache_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_header_override`.\n"]
    pub fn set_host_header_override(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.host_header_override = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_geolocation`.\n"]
    pub fn set_ip_geolocation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_geolocation = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `origin_error_page_pass_thru`.\n"]
    pub fn set_origin_error_page_pass_thru(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_error_page_pass_thru = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `polish`.\n"]
    pub fn set_polish(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.polish = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resolve_override`.\n"]
    pub fn set_resolve_override(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resolve_override = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `respect_strong_etag`.\n"]
    pub fn set_respect_strong_etag(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.respect_strong_etag = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `true_client_ip_header`.\n"]
    pub fn set_true_client_ip_header(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.true_client_ip_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `waf`.\n"]
    pub fn set_waf(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.waf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_key_fields`.\n"]
    pub fn set_cache_key_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PageRuleActionsElCacheKeyFieldsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_key_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_key_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cache_ttl_by_status`.\n"]
    pub fn set_cache_ttl_by_status(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PageRuleActionsElCacheTtlByStatusEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_ttl_by_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_ttl_by_status = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forwarding_url`.\n"]
    pub fn set_forwarding_url(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PageRuleActionsElForwardingUrlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forwarding_url = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarding_url = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `minify`.\n"]
    pub fn set_minify(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PageRuleActionsElMinifyEl>>,
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
}

impl terrars::ToListMappable for PageRuleActionsEl {
    type O = terrars::BlockAssignable<PageRuleActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsEl {}

impl BuildPageRuleActionsEl {
    pub fn build(self) -> PageRuleActionsEl {
        PageRuleActionsEl {
            always_use_https: core::default::Default::default(),
            automatic_https_rewrites: core::default::Default::default(),
            browser_cache_ttl: core::default::Default::default(),
            browser_check: core::default::Default::default(),
            bypass_cache_on_cookie: core::default::Default::default(),
            cache_by_device_type: core::default::Default::default(),
            cache_deception_armor: core::default::Default::default(),
            cache_level: core::default::Default::default(),
            cache_on_cookie: core::default::Default::default(),
            disable_apps: core::default::Default::default(),
            disable_performance: core::default::Default::default(),
            disable_railgun: core::default::Default::default(),
            disable_security: core::default::Default::default(),
            disable_zaraz: core::default::Default::default(),
            edge_cache_ttl: core::default::Default::default(),
            email_obfuscation: core::default::Default::default(),
            explicit_cache_control: core::default::Default::default(),
            host_header_override: core::default::Default::default(),
            ip_geolocation: core::default::Default::default(),
            mirage: core::default::Default::default(),
            opportunistic_encryption: core::default::Default::default(),
            origin_error_page_pass_thru: core::default::Default::default(),
            polish: core::default::Default::default(),
            resolve_override: core::default::Default::default(),
            respect_strong_etag: core::default::Default::default(),
            response_buffering: core::default::Default::default(),
            rocket_loader: core::default::Default::default(),
            security_level: core::default::Default::default(),
            server_side_exclude: core::default::Default::default(),
            sort_query_string_for_cache: core::default::Default::default(),
            ssl: core::default::Default::default(),
            true_client_ip_header: core::default::Default::default(),
            waf: core::default::Default::default(),
            cache_key_fields: core::default::Default::default(),
            cache_ttl_by_status: core::default::Default::default(),
            forwarding_url: core::default::Default::default(),
            minify: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PageRuleActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PageRuleActionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PageRuleActionsElRef {
        PageRuleActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `always_use_https` after provisioning.\nDefaults to `false`."]
    pub fn always_use_https(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.always_use_https", self.base))
    }

    #[doc = "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\n"]
    pub fn automatic_https_rewrites(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc = "Get a reference to the value of field `browser_cache_ttl` after provisioning.\n"]
    pub fn browser_cache_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_cache_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `browser_check` after provisioning.\n"]
    pub fn browser_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_check", self.base))
    }

    #[doc = "Get a reference to the value of field `bypass_cache_on_cookie` after provisioning.\n"]
    pub fn bypass_cache_on_cookie(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bypass_cache_on_cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_by_device_type` after provisioning.\n"]
    pub fn cache_by_device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_by_device_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_deception_armor` after provisioning.\n"]
    pub fn cache_deception_armor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_deception_armor", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_level` after provisioning.\n"]
    pub fn cache_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_level", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_on_cookie` after provisioning.\n"]
    pub fn cache_on_cookie(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_on_cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_apps` after provisioning.\nDefaults to `false`."]
    pub fn disable_apps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_apps", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_performance` after provisioning.\nDefaults to `false`."]
    pub fn disable_performance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_performance", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_railgun` after provisioning.\nDefaults to `false`."]
    pub fn disable_railgun(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_railgun", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_security` after provisioning.\nDefaults to `false`."]
    pub fn disable_security(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_security", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_zaraz` after provisioning.\nDefaults to `false`."]
    pub fn disable_zaraz(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_zaraz", self.base))
    }

    #[doc = "Get a reference to the value of field `edge_cache_ttl` after provisioning.\n"]
    pub fn edge_cache_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_cache_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `email_obfuscation` after provisioning.\n"]
    pub fn email_obfuscation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc = "Get a reference to the value of field `explicit_cache_control` after provisioning.\n"]
    pub fn explicit_cache_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.explicit_cache_control", self.base))
    }

    #[doc = "Get a reference to the value of field `host_header_override` after provisioning.\n"]
    pub fn host_header_override(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_header_override", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_geolocation` after provisioning.\n"]
    pub fn ip_geolocation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_geolocation", self.base))
    }

    #[doc = "Get a reference to the value of field `mirage` after provisioning.\n"]
    pub fn mirage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc = "Get a reference to the value of field `opportunistic_encryption` after provisioning.\n"]
    pub fn opportunistic_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_error_page_pass_thru` after provisioning.\n"]
    pub fn origin_error_page_pass_thru(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_pass_thru", self.base))
    }

    #[doc = "Get a reference to the value of field `polish` after provisioning.\n"]
    pub fn polish(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc = "Get a reference to the value of field `resolve_override` after provisioning.\n"]
    pub fn resolve_override(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolve_override", self.base))
    }

    #[doc = "Get a reference to the value of field `respect_strong_etag` after provisioning.\n"]
    pub fn respect_strong_etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.respect_strong_etag", self.base))
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

    #[doc = "Get a reference to the value of field `true_client_ip_header` after provisioning.\n"]
    pub fn true_client_ip_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.true_client_ip_header", self.base))
    }

    #[doc = "Get a reference to the value of field `waf` after provisioning.\n"]
    pub fn waf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waf", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_key_fields` after provisioning.\n"]
    pub fn cache_key_fields(&self) -> terrars::ListRef<PageRuleActionsElCacheKeyFieldsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_key_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarding_url` after provisioning.\n"]
    pub fn forwarding_url(&self) -> terrars::ListRef<PageRuleActionsElForwardingUrlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarding_url", self.base))
    }

    #[doc = "Get a reference to the value of field `minify` after provisioning.\n"]
    pub fn minify(&self) -> terrars::ListRef<PageRuleActionsElMinifyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.minify", self.base))
    }
}

#[derive(Serialize, Default)]
struct PageRuleDynamic {
    actions: std::option::Option<terrars::DynamicBlock<PageRuleActionsEl>>,
}
