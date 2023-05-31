use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct PageRuleData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    target: PrimField<String>,
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<PageRuleActionsEl>>,
    dynamic: PageRuleDynamic,
}

struct PageRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PageRuleData>,
}

#[derive(Clone)]
pub struct PageRule(Rc<PageRule_>);

impl PageRule {
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

    #[doc= "Set the field `priority`.\nDefaults to `1`."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\nDefaults to `active`."]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }

    #[doc= "Set the field `actions`.\n"]
    pub fn set_actions(self, v: impl Into<BlockAssignable<PageRuleActionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nDefaults to `1`."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nDefaults to `active`."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<PageRuleActionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }
}

impl Referable for PageRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for PageRule { }

impl ToListMappable for PageRule {
    type O = ListRef<PageRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PageRule_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_page_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPageRule {
    pub tf_id: String,
    #[doc= ""]
    pub target: PrimField<String>,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildPageRule {
    pub fn build(self, stack: &mut Stack) -> PageRule {
        let out = PageRule(Rc::new(PageRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PageRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                priority: core::default::Default::default(),
                status: core::default::Default::default(),
                target: self.target,
                zone_id: self.zone_id,
                actions: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PageRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PageRuleRef {
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

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nDefaults to `1`."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nDefaults to `active`."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<PageRuleActionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsElCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_presence: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<SetField<PrimField<String>>>,
}

impl PageRuleActionsElCacheKeyFieldsElCookieEl {
    #[doc= "Set the field `check_presence`.\n"]
    pub fn set_check_presence(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.check_presence = Some(v.into());
        self
    }

    #[doc= "Set the field `include`.\n"]
    pub fn set_include(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.include = Some(v.into());
        self
    }
}

impl ToListMappable for PageRuleActionsElCacheKeyFieldsElCookieEl {
    type O = BlockAssignable<PageRuleActionsElCacheKeyFieldsElCookieEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleActionsElCacheKeyFieldsElCookieElRef {
    fn new(shared: StackShared, base: String) -> PageRuleActionsElCacheKeyFieldsElCookieElRef {
        PageRuleActionsElCacheKeyFieldsElCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `check_presence` after provisioning.\n"]
    pub fn check_presence(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_presence: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<SetField<PrimField<String>>>,
}

impl PageRuleActionsElCacheKeyFieldsElHeaderEl {
    #[doc= "Set the field `check_presence`.\n"]
    pub fn set_check_presence(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.check_presence = Some(v.into());
        self
    }

    #[doc= "Set the field `exclude`.\n"]
    pub fn set_exclude(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclude = Some(v.into());
        self
    }

    #[doc= "Set the field `include`.\n"]
    pub fn set_include(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.include = Some(v.into());
        self
    }
}

impl ToListMappable for PageRuleActionsElCacheKeyFieldsElHeaderEl {
    type O = BlockAssignable<PageRuleActionsElCacheKeyFieldsElHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleActionsElCacheKeyFieldsElHeaderElRef {
    fn new(shared: StackShared, base: String) -> PageRuleActionsElCacheKeyFieldsElHeaderElRef {
        PageRuleActionsElCacheKeyFieldsElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `check_presence` after provisioning.\n"]
    pub fn check_presence(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc= "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsElHostEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resolved: Option<PrimField<bool>>,
}

impl PageRuleActionsElCacheKeyFieldsElHostEl {
    #[doc= "Set the field `resolved`.\nDefaults to `false`."]
    pub fn set_resolved(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.resolved = Some(v.into());
        self
    }
}

impl ToListMappable for PageRuleActionsElCacheKeyFieldsElHostEl {
    type O = BlockAssignable<PageRuleActionsElCacheKeyFieldsElHostEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleActionsElCacheKeyFieldsElHostElRef {
    fn new(shared: StackShared, base: String) -> PageRuleActionsElCacheKeyFieldsElHostElRef {
        PageRuleActionsElCacheKeyFieldsElHostElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElHostElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `resolved` after provisioning.\nDefaults to `false`."]
    pub fn resolved(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolved", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsElQueryStringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<SetField<PrimField<String>>>,
}

impl PageRuleActionsElCacheKeyFieldsElQueryStringEl {
    #[doc= "Set the field `exclude`.\n"]
    pub fn set_exclude(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclude = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore`.\n"]
    pub fn set_ignore(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore = Some(v.into());
        self
    }

    #[doc= "Set the field `include`.\n"]
    pub fn set_include(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.include = Some(v.into());
        self
    }
}

impl ToListMappable for PageRuleActionsElCacheKeyFieldsElQueryStringEl {
    type O = BlockAssignable<PageRuleActionsElCacheKeyFieldsElQueryStringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleActionsElCacheKeyFieldsElQueryStringElRef {
    fn new(shared: StackShared, base: String) -> PageRuleActionsElCacheKeyFieldsElQueryStringElRef {
        PageRuleActionsElCacheKeyFieldsElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElQueryStringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc= "Get a reference to the value of field `ignore` after provisioning.\n"]
    pub fn ignore(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsElUserEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<PrimField<bool>>,
}

impl PageRuleActionsElCacheKeyFieldsElUserEl {
    #[doc= "Set the field `device_type`.\n"]
    pub fn set_device_type(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.device_type = Some(v.into());
        self
    }

    #[doc= "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.geo = Some(v.into());
        self
    }

    #[doc= "Set the field `lang`.\n"]
    pub fn set_lang(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.lang = Some(v.into());
        self
    }
}

impl ToListMappable for PageRuleActionsElCacheKeyFieldsElUserEl {
    type O = BlockAssignable<PageRuleActionsElCacheKeyFieldsElUserEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleActionsElCacheKeyFieldsElUserElRef {
    fn new(shared: StackShared, base: String) -> PageRuleActionsElCacheKeyFieldsElUserElRef {
        PageRuleActionsElCacheKeyFieldsElUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElUserElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `device_type` after provisioning.\n"]
    pub fn device_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }

    #[doc= "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc= "Get a reference to the value of field `lang` after provisioning.\n"]
    pub fn lang(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lang", self.base))
    }
}

#[derive(Serialize, Default)]
struct PageRuleActionsElCacheKeyFieldsElDynamic {
    cookie: Option<DynamicBlock<PageRuleActionsElCacheKeyFieldsElCookieEl>>,
    header: Option<DynamicBlock<PageRuleActionsElCacheKeyFieldsElHeaderEl>>,
    host: Option<DynamicBlock<PageRuleActionsElCacheKeyFieldsElHostEl>>,
    query_string: Option<DynamicBlock<PageRuleActionsElCacheKeyFieldsElQueryStringEl>>,
    user: Option<DynamicBlock<PageRuleActionsElCacheKeyFieldsElUserEl>>,
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheKeyFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie: Option<Vec<PageRuleActionsElCacheKeyFieldsElCookieEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<PageRuleActionsElCacheKeyFieldsElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<Vec<PageRuleActionsElCacheKeyFieldsElHostEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string: Option<Vec<PageRuleActionsElCacheKeyFieldsElQueryStringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<Vec<PageRuleActionsElCacheKeyFieldsElUserEl>>,
    dynamic: PageRuleActionsElCacheKeyFieldsElDynamic,
}

impl PageRuleActionsElCacheKeyFieldsEl {
    #[doc= "Set the field `cookie`.\n"]
    pub fn set_cookie(mut self, v: impl Into<BlockAssignable<PageRuleActionsElCacheKeyFieldsElCookieEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cookie = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cookie = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<BlockAssignable<PageRuleActionsElCacheKeyFieldsElHeaderEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<BlockAssignable<PageRuleActionsElCacheKeyFieldsElHostEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.host = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.host = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v: impl Into<BlockAssignable<PageRuleActionsElCacheKeyFieldsElQueryStringEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_string = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl Into<BlockAssignable<PageRuleActionsElCacheKeyFieldsElUserEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for PageRuleActionsElCacheKeyFieldsEl {
    type O = BlockAssignable<PageRuleActionsElCacheKeyFieldsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct PageRuleActionsElCacheKeyFieldsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleActionsElCacheKeyFieldsElRef {
    fn new(shared: StackShared, base: String) -> PageRuleActionsElCacheKeyFieldsElRef {
        PageRuleActionsElCacheKeyFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheKeyFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cookie` after provisioning.\n"]
    pub fn cookie(&self) -> ListRef<PageRuleActionsElCacheKeyFieldsElCookieElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cookie", self.base))
    }

    #[doc= "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> ListRef<PageRuleActionsElCacheKeyFieldsElHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> ListRef<PageRuleActionsElCacheKeyFieldsElHostElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> ListRef<PageRuleActionsElCacheKeyFieldsElQueryStringElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> ListRef<PageRuleActionsElCacheKeyFieldsElUserElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElCacheTtlByStatusEl {
    codes: PrimField<String>,
    ttl: PrimField<f64>,
}

impl PageRuleActionsElCacheTtlByStatusEl { }

impl ToListMappable for PageRuleActionsElCacheTtlByStatusEl {
    type O = BlockAssignable<PageRuleActionsElCacheTtlByStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElCacheTtlByStatusEl {
    #[doc= ""]
    pub codes: PrimField<String>,
    #[doc= ""]
    pub ttl: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleActionsElCacheTtlByStatusElRef {
    fn new(shared: StackShared, base: String) -> PageRuleActionsElCacheTtlByStatusElRef {
        PageRuleActionsElCacheTtlByStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElCacheTtlByStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `codes` after provisioning.\n"]
    pub fn codes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.codes", self.base))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElForwardingUrlEl {
    status_code: PrimField<f64>,
    url: PrimField<String>,
}

impl PageRuleActionsElForwardingUrlEl { }

impl ToListMappable for PageRuleActionsElForwardingUrlEl {
    type O = BlockAssignable<PageRuleActionsElForwardingUrlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElForwardingUrlEl {
    #[doc= ""]
    pub status_code: PrimField<f64>,
    #[doc= ""]
    pub url: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleActionsElForwardingUrlElRef {
    fn new(shared: StackShared, base: String) -> PageRuleActionsElForwardingUrlElRef {
        PageRuleActionsElForwardingUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElForwardingUrlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct PageRuleActionsElMinifyEl {
    css: PrimField<String>,
    html: PrimField<String>,
    js: PrimField<String>,
}

impl PageRuleActionsElMinifyEl { }

impl ToListMappable for PageRuleActionsElMinifyEl {
    type O = BlockAssignable<PageRuleActionsElMinifyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPageRuleActionsElMinifyEl {
    #[doc= ""]
    pub css: PrimField<String>,
    #[doc= ""]
    pub html: PrimField<String>,
    #[doc= ""]
    pub js: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleActionsElMinifyElRef {
    fn new(shared: StackShared, base: String) -> PageRuleActionsElMinifyElRef {
        PageRuleActionsElMinifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElMinifyElRef {
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

#[derive(Serialize, Default)]
struct PageRuleActionsElDynamic {
    cache_key_fields: Option<DynamicBlock<PageRuleActionsElCacheKeyFieldsEl>>,
    cache_ttl_by_status: Option<DynamicBlock<PageRuleActionsElCacheTtlByStatusEl>>,
    forwarding_url: Option<DynamicBlock<PageRuleActionsElForwardingUrlEl>>,
    minify: Option<DynamicBlock<PageRuleActionsElMinifyEl>>,
}

#[derive(Serialize)]
pub struct PageRuleActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    always_use_https: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_https_rewrites: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_cache_ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_check: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_cache_on_cookie: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_by_device_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_deception_armor: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_on_cookie: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_apps: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_performance: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_railgun: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_security: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_zaraz: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_cache_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_obfuscation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explicit_cache_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_header_override: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_geolocation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opportunistic_encryption: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_error_page_pass_thru: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polish: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolve_override: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    respect_strong_etag: Option<PrimField<String>>,
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
    true_client_ip_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    waf: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_key_fields: Option<Vec<PageRuleActionsElCacheKeyFieldsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_ttl_by_status: Option<Vec<PageRuleActionsElCacheTtlByStatusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_url: Option<Vec<PageRuleActionsElForwardingUrlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minify: Option<Vec<PageRuleActionsElMinifyEl>>,
    dynamic: PageRuleActionsElDynamic,
}

impl PageRuleActionsEl {
    #[doc= "Set the field `always_use_https`.\nDefaults to `false`."]
    pub fn set_always_use_https(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.always_use_https = Some(v.into());
        self
    }

    #[doc= "Set the field `automatic_https_rewrites`.\n"]
    pub fn set_automatic_https_rewrites(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.automatic_https_rewrites = Some(v.into());
        self
    }

    #[doc= "Set the field `browser_cache_ttl`.\n"]
    pub fn set_browser_cache_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.browser_cache_ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `browser_check`.\n"]
    pub fn set_browser_check(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.browser_check = Some(v.into());
        self
    }

    #[doc= "Set the field `bypass_cache_on_cookie`.\n"]
    pub fn set_bypass_cache_on_cookie(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bypass_cache_on_cookie = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_by_device_type`.\n"]
    pub fn set_cache_by_device_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_by_device_type = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_deception_armor`.\n"]
    pub fn set_cache_deception_armor(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_deception_armor = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_level`.\n"]
    pub fn set_cache_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_level = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_on_cookie`.\n"]
    pub fn set_cache_on_cookie(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_on_cookie = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_apps`.\nDefaults to `false`."]
    pub fn set_disable_apps(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_apps = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_performance`.\nDefaults to `false`."]
    pub fn set_disable_performance(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_performance = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_railgun`.\nDefaults to `false`."]
    pub fn set_disable_railgun(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_railgun = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_security`.\nDefaults to `false`."]
    pub fn set_disable_security(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_security = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_zaraz`.\nDefaults to `false`."]
    pub fn set_disable_zaraz(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_zaraz = Some(v.into());
        self
    }

    #[doc= "Set the field `edge_cache_ttl`.\n"]
    pub fn set_edge_cache_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.edge_cache_ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `email_obfuscation`.\n"]
    pub fn set_email_obfuscation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email_obfuscation = Some(v.into());
        self
    }

    #[doc= "Set the field `explicit_cache_control`.\n"]
    pub fn set_explicit_cache_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.explicit_cache_control = Some(v.into());
        self
    }

    #[doc= "Set the field `host_header_override`.\n"]
    pub fn set_host_header_override(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_header_override = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_geolocation`.\n"]
    pub fn set_ip_geolocation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_geolocation = Some(v.into());
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

    #[doc= "Set the field `origin_error_page_pass_thru`.\n"]
    pub fn set_origin_error_page_pass_thru(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.origin_error_page_pass_thru = Some(v.into());
        self
    }

    #[doc= "Set the field `polish`.\n"]
    pub fn set_polish(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.polish = Some(v.into());
        self
    }

    #[doc= "Set the field `resolve_override`.\n"]
    pub fn set_resolve_override(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resolve_override = Some(v.into());
        self
    }

    #[doc= "Set the field `respect_strong_etag`.\n"]
    pub fn set_respect_strong_etag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.respect_strong_etag = Some(v.into());
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

    #[doc= "Set the field `true_client_ip_header`.\n"]
    pub fn set_true_client_ip_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.true_client_ip_header = Some(v.into());
        self
    }

    #[doc= "Set the field `waf`.\n"]
    pub fn set_waf(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.waf = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_key_fields`.\n"]
    pub fn set_cache_key_fields(mut self, v: impl Into<BlockAssignable<PageRuleActionsElCacheKeyFieldsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_key_fields = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_key_fields = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `cache_ttl_by_status`.\n"]
    pub fn set_cache_ttl_by_status(
        mut self,
        v: impl Into<BlockAssignable<PageRuleActionsElCacheTtlByStatusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_ttl_by_status = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_ttl_by_status = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `forwarding_url`.\n"]
    pub fn set_forwarding_url(mut self, v: impl Into<BlockAssignable<PageRuleActionsElForwardingUrlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forwarding_url = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarding_url = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `minify`.\n"]
    pub fn set_minify(mut self, v: impl Into<BlockAssignable<PageRuleActionsElMinifyEl>>) -> Self {
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
}

impl ToListMappable for PageRuleActionsEl {
    type O = BlockAssignable<PageRuleActionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct PageRuleActionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PageRuleActionsElRef {
    fn new(shared: StackShared, base: String) -> PageRuleActionsElRef {
        PageRuleActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PageRuleActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `always_use_https` after provisioning.\nDefaults to `false`."]
    pub fn always_use_https(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.always_use_https", self.base))
    }

    #[doc= "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\n"]
    pub fn automatic_https_rewrites(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc= "Get a reference to the value of field `browser_cache_ttl` after provisioning.\n"]
    pub fn browser_cache_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_cache_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `browser_check` after provisioning.\n"]
    pub fn browser_check(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_check", self.base))
    }

    #[doc= "Get a reference to the value of field `bypass_cache_on_cookie` after provisioning.\n"]
    pub fn bypass_cache_on_cookie(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bypass_cache_on_cookie", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_by_device_type` after provisioning.\n"]
    pub fn cache_by_device_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_by_device_type", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_deception_armor` after provisioning.\n"]
    pub fn cache_deception_armor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_deception_armor", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_level` after provisioning.\n"]
    pub fn cache_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_level", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_on_cookie` after provisioning.\n"]
    pub fn cache_on_cookie(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_on_cookie", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_apps` after provisioning.\nDefaults to `false`."]
    pub fn disable_apps(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_apps", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_performance` after provisioning.\nDefaults to `false`."]
    pub fn disable_performance(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_performance", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_railgun` after provisioning.\nDefaults to `false`."]
    pub fn disable_railgun(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_railgun", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_security` after provisioning.\nDefaults to `false`."]
    pub fn disable_security(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_security", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_zaraz` after provisioning.\nDefaults to `false`."]
    pub fn disable_zaraz(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_zaraz", self.base))
    }

    #[doc= "Get a reference to the value of field `edge_cache_ttl` after provisioning.\n"]
    pub fn edge_cache_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.edge_cache_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `email_obfuscation` after provisioning.\n"]
    pub fn email_obfuscation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc= "Get a reference to the value of field `explicit_cache_control` after provisioning.\n"]
    pub fn explicit_cache_control(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.explicit_cache_control", self.base))
    }

    #[doc= "Get a reference to the value of field `host_header_override` after provisioning.\n"]
    pub fn host_header_override(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_header_override", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_geolocation` after provisioning.\n"]
    pub fn ip_geolocation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_geolocation", self.base))
    }

    #[doc= "Get a reference to the value of field `mirage` after provisioning.\n"]
    pub fn mirage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc= "Get a reference to the value of field `opportunistic_encryption` after provisioning.\n"]
    pub fn opportunistic_encryption(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc= "Get a reference to the value of field `origin_error_page_pass_thru` after provisioning.\n"]
    pub fn origin_error_page_pass_thru(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_pass_thru", self.base))
    }

    #[doc= "Get a reference to the value of field `polish` after provisioning.\n"]
    pub fn polish(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc= "Get a reference to the value of field `resolve_override` after provisioning.\n"]
    pub fn resolve_override(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolve_override", self.base))
    }

    #[doc= "Get a reference to the value of field `respect_strong_etag` after provisioning.\n"]
    pub fn respect_strong_etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.respect_strong_etag", self.base))
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

    #[doc= "Get a reference to the value of field `true_client_ip_header` after provisioning.\n"]
    pub fn true_client_ip_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.true_client_ip_header", self.base))
    }

    #[doc= "Get a reference to the value of field `waf` after provisioning.\n"]
    pub fn waf(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.waf", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_key_fields` after provisioning.\n"]
    pub fn cache_key_fields(&self) -> ListRef<PageRuleActionsElCacheKeyFieldsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_key_fields", self.base))
    }

    #[doc= "Get a reference to the value of field `forwarding_url` after provisioning.\n"]
    pub fn forwarding_url(&self) -> ListRef<PageRuleActionsElForwardingUrlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forwarding_url", self.base))
    }

    #[doc= "Get a reference to the value of field `minify` after provisioning.\n"]
    pub fn minify(&self) -> ListRef<PageRuleActionsElMinifyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.minify", self.base))
    }
}

#[derive(Serialize, Default)]
struct PageRuleDynamic {
    actions: Option<DynamicBlock<PageRuleActionsEl>>,
}
