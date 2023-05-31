use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataRulesetsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_rules: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataRulesetsFilterEl>>,
    dynamic: DataRulesetsDynamic,
}

struct DataRulesets_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRulesetsData>,
}

#[derive(Clone)]
pub struct DataRulesets(Rc<DataRulesets_>);

impl DataRulesets {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderCloudflare) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `include_rules`.\nInclude rule data in response."]
    pub fn set_include_rules(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_rules = Some(v.into());
        self
    }

    #[doc= "Set the field `zone_id`.\nThe zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn set_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataRulesetsFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_rules` after provisioning.\nInclude rule data in response."]
    pub fn include_rules(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rulesets` after provisioning.\n"]
    pub fn rulesets(&self) -> ListRef<DataRulesetsRulesetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rulesets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataRulesetsFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

impl Referable for DataRulesets {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRulesets { }

impl ToListMappable for DataRulesets {
    type O = ListRef<DataRulesetsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRulesets_ {
    fn extract_datasource_type(&self) -> String {
        "cloudflare_rulesets".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRulesets {
    pub tf_id: String,
}

impl BuildDataRulesets {
    pub fn build(self, stack: &mut Stack) -> DataRulesets {
        let out = DataRulesets(Rc::new(DataRulesets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRulesetsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                include_rules: core::default::Default::default(),
                zone_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRulesetsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRulesetsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_rules` after provisioning.\nInclude rule data in response."]
    pub fn include_rules(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rulesets` after provisioning.\n"]
    pub fn rulesets(&self) -> ListRef<DataRulesetsRulesetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rulesets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataRulesetsFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    css: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    js: Option<PrimField<bool>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl {
    #[doc= "Set the field `css`.\n"]
    pub fn set_css(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.css = Some(v.into());
        self
    }

    #[doc= "Set the field `html`.\n"]
    pub fn set_html(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.html = Some(v.into());
        self
    }

    #[doc= "Set the field `js`.\n"]
    pub fn set_js(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.js = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElAutominifyEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElAutominifyEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl {
        DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl {
            css: core::default::Default::default(),
            html: core::default::Default::default(),
            js: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef {
        DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `css` after provisioning.\n"]
    pub fn css(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.css", self.base))
    }

    #[doc= "Get a reference to the value of field `html` after provisioning.\n"]
    pub fn html(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }

    #[doc= "Get a reference to the value of field `js` after provisioning.\n"]
    pub fn js(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.js", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl {
    #[doc= "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default = Some(v.into());
        self
    }

    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl {
        DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl {
            default: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef {
        DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_presence: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<ListField<PrimField<String>>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    #[doc= "Set the field `check_presence`.\n"]
    pub fn set_check_presence(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.check_presence = Some(v.into());
        self
    }

    #[doc= "Set the field `include`.\n"]
    pub fn set_include(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
            check_presence: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `check_presence` after provisioning.\n"]
    pub fn check_presence(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_presence: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_origin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<ListField<PrimField<String>>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    #[doc= "Set the field `check_presence`.\n"]
    pub fn set_check_presence(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.check_presence = Some(v.into());
        self
    }

    #[doc= "Set the field `exclude_origin`.\n"]
    pub fn set_exclude_origin(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.exclude_origin = Some(v.into());
        self
    }

    #[doc= "Set the field `include`.\n"]
    pub fn set_include(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
            check_presence: core::default::Default::default(),
            exclude_origin: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `check_presence` after provisioning.\n"]
    pub fn check_presence(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc= "Get a reference to the value of field `exclude_origin` after provisioning.\n"]
    pub fn exclude_origin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_origin", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resolved: Option<PrimField<bool>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    #[doc= "Set the field `resolved`.\n"]
    pub fn set_resolved(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.resolved = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
            resolved: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `resolved` after provisioning.\n"]
    pub fn resolved(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolved", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<ListField<PrimField<String>>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    #[doc= "Set the field `exclude`.\n"]
    pub fn set_exclude(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude = Some(v.into());
        self
    }

    #[doc= "Set the field `include`.\n"]
    pub fn set_include(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
            exclude: core::default::Default::default(),
            include: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<PrimField<bool>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
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

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
            device_type: core::default::Default::default(),
            geo: core::default::Default::default(),
            lang: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
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

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl {
    #[doc= "Set the field `cookie`.\n"]
    pub fn set_cookie(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>>,
    ) -> Self {
        self.cookie = Some(v.into());
        self
    }

    #[doc= "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>>,
    ) -> Self {
        self.header = Some(v.into());
        self
    }

    #[doc= "Set the field `host`.\n"]
    pub fn set_host(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl>>,
    ) -> Self {
        self.host = Some(v.into());
        self
    }

    #[doc= "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>>,
    ) -> Self {
        self.query_string = Some(v.into());
        self
    }

    #[doc= "Set the field `user`.\n"]
    pub fn set_user(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl>>,
    ) -> Self {
        self.user = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl {
            cookie: core::default::Default::default(),
            header: core::default::Default::default(),
            host: core::default::Default::default(),
            query_string: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cookie` after provisioning.\n"]
    pub fn cookie(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cookie", self.base))
    }

    #[doc= "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_by_device_type: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_deception_armor: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_key: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_query_strings_order: Option<PrimField<bool>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl {
    #[doc= "Set the field `cache_by_device_type`.\n"]
    pub fn set_cache_by_device_type(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cache_by_device_type = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_deception_armor`.\n"]
    pub fn set_cache_deception_armor(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cache_deception_armor = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_key`.\n"]
    pub fn set_custom_key(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl>>,
    ) -> Self {
        self.custom_key = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore_query_strings_order`.\n"]
    pub fn set_ignore_query_strings_order(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_query_strings_order = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl {
            cache_by_device_type: core::default::Default::default(),
            cache_deception_armor: core::default::Default::default(),
            custom_key: core::default::Default::default(),
            ignore_query_strings_order: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cache_by_device_type` after provisioning.\n"]
    pub fn cache_by_device_type(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_by_device_type", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_deception_armor` after provisioning.\n"]
    pub fn cache_deception_armor(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_deception_armor", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_key` after provisioning.\n"]
    pub fn custom_key(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_key", self.base))
    }

    #[doc= "Get a reference to the value of field `ignore_query_strings_order` after provisioning.\n"]
    pub fn ignore_query_strings_order(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_query_strings_order", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    #[doc= "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc= "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    type O =
        BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
        DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
        DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc= "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code_range: Option<
        ListField<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    #[doc= "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code_range`.\n"]
    pub fn set_status_code_range(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl,
                        >,
                    >,
    ) -> Self {
        self.status_code_range = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
        DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
            status_code: core::default::Default::default(),
            status_code_range: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
        DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code_range` after provisioning.\n"]
    pub fn status_code_range(
        &self,
    ) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status_code_range", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code_ttl: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl {
    #[doc= "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default = Some(v.into());
        self
    }

    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code_ttl`.\n"]
    pub fn set_status_code_ttl(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>>,
    ) -> Self {
        self.status_code_ttl = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl {
        DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl {
            default: core::default::Default::default(),
            mode: core::default::Default::default(),
            status_code_ttl: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef {
        DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code_ttl` after provisioning.\n"]
    pub fn status_code_ttl(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.status_code_ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElFromListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromListEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElFromListEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElFromListEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElFromListEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElFromListEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElFromListEl {
        DataRulesetsRulesetsElRulesElActionParametersElFromListEl {
            key: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElFromListElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElFromListElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElFromListElRef {
        DataRulesetsRulesetsElRulesElActionParametersElFromListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl {
    #[doc= "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl {
        DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl {
            expression: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef {
        DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElFromValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_query_string: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_url: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromValueEl {
    #[doc= "Set the field `preserve_query_string`.\n"]
    pub fn set_preserve_query_string(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preserve_query_string = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }

    #[doc= "Set the field `target_url`.\n"]
    pub fn set_target_url(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl>>,
    ) -> Self {
        self.target_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElFromValueEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElFromValueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElFromValueEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElFromValueEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElFromValueEl {
        DataRulesetsRulesetsElRulesElActionParametersElFromValueEl {
            preserve_query_string: core::default::Default::default(),
            status_code: core::default::Default::default(),
            target_url: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef {
        DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `preserve_query_string` after provisioning.\n"]
    pub fn preserve_query_string(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_query_string", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc= "Get a reference to the value of field `target_url` after provisioning.\n"]
    pub fn target_url(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElHeadersEl {
    #[doc= "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `operation`.\n"]
    pub fn set_operation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operation = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElHeadersEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElHeadersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElHeadersEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElHeadersEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElHeadersEl {
        DataRulesetsRulesetsElRulesElActionParametersElHeadersEl {
            expression: core::default::Default::default(),
            name: core::default::Default::default(),
            operation: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef {
        DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl {
    #[doc= "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl {
        DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl {
            public_key: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef {
        DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElOriginEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElOriginEl {
    #[doc= "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElOriginEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElOriginEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElOriginEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElOriginEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElOriginEl {
        DataRulesetsRulesetsElRulesElActionParametersElOriginEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElOriginElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElOriginElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElOriginElRef {
        DataRulesetsRulesetsElRulesElActionParametersElOriginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElOriginElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl {
    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc= "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.category = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl {
        DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl {
            action: core::default::Default::default(),
            category: core::default::Default::default(),
            enabled: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef {
        DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitivity_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl {
    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `score_threshold`.\n"]
    pub fn set_score_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.score_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `sensitivity_level`.\n"]
    pub fn set_sensitivity_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sensitivity_level = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl {
        DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl {
            action: core::default::Default::default(),
            enabled: core::default::Default::default(),
            id: core::default::Default::default(),
            score_threshold: core::default::Default::default(),
            sensitivity_level: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef {
        DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `score_threshold` after provisioning.\n"]
    pub fn score_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `sensitivity_level` after provisioning.\n"]
    pub fn sensitivity_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitivity_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesEl {
    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc= "Set the field `categories`.\n"]
    pub fn set_categories(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl>>,
    ) -> Self {
        self.categories = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl>>,
    ) -> Self {
        self.rules = Some(v.into());
        self
    }

    #[doc= "Set the field `sensitivity_level`.\n"]
    pub fn set_sensitivity_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sensitivity_level = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElOverridesEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElOverridesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElOverridesEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElOverridesEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElOverridesEl {
        DataRulesetsRulesetsElRulesElActionParametersElOverridesEl {
            action: core::default::Default::default(),
            categories: core::default::Default::default(),
            enabled: core::default::Default::default(),
            rules: core::default::Default::default(),
            sensitivity_level: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef {
        DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `categories` after provisioning.\n"]
    pub fn categories(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.categories", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc= "Get a reference to the value of field `sensitivity_level` after provisioning.\n"]
    pub fn sensitivity_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElResponseEl {
    #[doc= "Set the field `content`.\n"]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }

    #[doc= "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElResponseEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElResponseEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElResponseEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElResponseEl {
        DataRulesetsRulesetsElRulesElActionParametersElResponseEl {
            content: core::default::Default::default(),
            content_type: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElResponseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElResponseElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElResponseElRef {
        DataRulesetsRulesetsElRulesElActionParametersElResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_stale_while_updating: Option<PrimField<bool>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl {
    #[doc= "Set the field `disable_stale_while_updating`.\n"]
    pub fn set_disable_stale_while_updating(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_stale_while_updating = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElServeStaleEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElServeStaleEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl {
        DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl {
            disable_stale_while_updating: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef {
        DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disable_stale_while_updating` after provisioning.\n"]
    pub fn disable_stale_while_updating(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_stale_while_updating", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElSniEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElSniEl {
    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElSniEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElSniEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElSniEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElSniEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElSniEl {
        DataRulesetsRulesetsElRulesElActionParametersElSniEl { value: core::default::Default::default() }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElSniElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElSniElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElSniElRef {
        DataRulesetsRulesetsElRulesElActionParametersElSniElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElSniElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl {
    #[doc= "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElUriElPathEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElUriElPathEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl {
        DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl {
            expression: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef {
        DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl {
    #[doc= "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl {
        DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl {
            expression: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef {
        DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElUriEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriEl {
    #[doc= "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.origin = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl>>,
    ) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `query`.\n"]
    pub fn set_query(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl>>,
    ) -> Self {
        self.query = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElUriEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElUriEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersElUriEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersElUriEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersElUriEl {
        DataRulesetsRulesetsElRulesElActionParametersElUriEl {
            origin: core::default::Default::default(),
            path: core::default::Default::default(),
            query: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElUriElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElUriElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElUriElRef {
        DataRulesetsRulesetsElRulesElActionParametersElUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_https_rewrites: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autominify: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bic: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_ttl: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_key: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_fields: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_apps: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_railgun: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_zaraz: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_ttl: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_obfuscation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_list: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElFromListEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_value: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElFromValueEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hotlink_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    increment: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matched_data: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirage: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opportunistic_encryption: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElOriginEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_error_page_passthru: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phases: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polish: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    products: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_fields: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    respect_strong_etags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_fields: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rocket_loader: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ruleset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rulesets: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serve_stale: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_excludes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sni: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElSniEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sxg: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersElUriEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersEl {
    #[doc= "Set the field `automatic_https_rewrites`.\n"]
    pub fn set_automatic_https_rewrites(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.automatic_https_rewrites = Some(v.into());
        self
    }

    #[doc= "Set the field `autominify`.\n"]
    pub fn set_autominify(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl>>,
    ) -> Self {
        self.autominify = Some(v.into());
        self
    }

    #[doc= "Set the field `bic`.\n"]
    pub fn set_bic(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bic = Some(v.into());
        self
    }

    #[doc= "Set the field `browser_ttl`.\n"]
    pub fn set_browser_ttl(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl>>,
    ) -> Self {
        self.browser_ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `cache`.\n"]
    pub fn set_cache(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cache = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_key`.\n"]
    pub fn set_cache_key(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl>>,
    ) -> Self {
        self.cache_key = Some(v.into());
        self
    }

    #[doc= "Set the field `content`.\n"]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }

    #[doc= "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `cookie_fields`.\n"]
    pub fn set_cookie_fields(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cookie_fields = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_apps`.\n"]
    pub fn set_disable_apps(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_apps = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_railgun`.\n"]
    pub fn set_disable_railgun(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_railgun = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_zaraz`.\n"]
    pub fn set_disable_zaraz(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_zaraz = Some(v.into());
        self
    }

    #[doc= "Set the field `edge_ttl`.\n"]
    pub fn set_edge_ttl(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl>>,
    ) -> Self {
        self.edge_ttl = Some(v.into());
        self
    }

    #[doc= "Set the field `email_obfuscation`.\n"]
    pub fn set_email_obfuscation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.email_obfuscation = Some(v.into());
        self
    }

    #[doc= "Set the field `from_list`.\n"]
    pub fn set_from_list(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElFromListEl>>,
    ) -> Self {
        self.from_list = Some(v.into());
        self
    }

    #[doc= "Set the field `from_value`.\n"]
    pub fn set_from_value(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElFromValueEl>>,
    ) -> Self {
        self.from_value = Some(v.into());
        self
    }

    #[doc= "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElHeadersEl>>,
    ) -> Self {
        self.headers = Some(v.into());
        self
    }

    #[doc= "Set the field `host_header`.\n"]
    pub fn set_host_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_header = Some(v.into());
        self
    }

    #[doc= "Set the field `hotlink_protection`.\n"]
    pub fn set_hotlink_protection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.hotlink_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `increment`.\n"]
    pub fn set_increment(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.increment = Some(v.into());
        self
    }

    #[doc= "Set the field `matched_data`.\n"]
    pub fn set_matched_data(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl>>,
    ) -> Self {
        self.matched_data = Some(v.into());
        self
    }

    #[doc= "Set the field `mirage`.\n"]
    pub fn set_mirage(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mirage = Some(v.into());
        self
    }

    #[doc= "Set the field `opportunistic_encryption`.\n"]
    pub fn set_opportunistic_encryption(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.opportunistic_encryption = Some(v.into());
        self
    }

    #[doc= "Set the field `origin`.\n"]
    pub fn set_origin(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElOriginEl>>,
    ) -> Self {
        self.origin = Some(v.into());
        self
    }

    #[doc= "Set the field `origin_error_page_passthru`.\n"]
    pub fn set_origin_error_page_passthru(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.origin_error_page_passthru = Some(v.into());
        self
    }

    #[doc= "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesEl>>,
    ) -> Self {
        self.overrides = Some(v.into());
        self
    }

    #[doc= "Set the field `phases`.\n"]
    pub fn set_phases(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.phases = Some(v.into());
        self
    }

    #[doc= "Set the field `polish`.\n"]
    pub fn set_polish(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.polish = Some(v.into());
        self
    }

    #[doc= "Set the field `products`.\n"]
    pub fn set_products(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.products = Some(v.into());
        self
    }

    #[doc= "Set the field `request_fields`.\n"]
    pub fn set_request_fields(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.request_fields = Some(v.into());
        self
    }

    #[doc= "Set the field `respect_strong_etags`.\n"]
    pub fn set_respect_strong_etags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.respect_strong_etags = Some(v.into());
        self
    }

    #[doc= "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElResponseEl>>,
    ) -> Self {
        self.response = Some(v.into());
        self
    }

    #[doc= "Set the field `response_fields`.\n"]
    pub fn set_response_fields(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.response_fields = Some(v.into());
        self
    }

    #[doc= "Set the field `rocket_loader`.\n"]
    pub fn set_rocket_loader(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.rocket_loader = Some(v.into());
        self
    }

    #[doc= "Set the field `rules`.\n"]
    pub fn set_rules(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.rules = Some(v.into());
        self
    }

    #[doc= "Set the field `ruleset`.\n"]
    pub fn set_ruleset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ruleset = Some(v.into());
        self
    }

    #[doc= "Set the field `rulesets`.\n"]
    pub fn set_rulesets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.rulesets = Some(v.into());
        self
    }

    #[doc= "Set the field `security_level`.\n"]
    pub fn set_security_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_level = Some(v.into());
        self
    }

    #[doc= "Set the field `serve_stale`.\n"]
    pub fn set_serve_stale(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl>>,
    ) -> Self {
        self.serve_stale = Some(v.into());
        self
    }

    #[doc= "Set the field `server_side_excludes`.\n"]
    pub fn set_server_side_excludes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.server_side_excludes = Some(v.into());
        self
    }

    #[doc= "Set the field `sni`.\n"]
    pub fn set_sni(mut self, v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElSniEl>>) -> Self {
        self.sni = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl`.\n"]
    pub fn set_ssl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }

    #[doc= "Set the field `sxg`.\n"]
    pub fn set_sxg(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sxg = Some(v.into());
        self
    }

    #[doc= "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersElUriEl>>) -> Self {
        self.uri = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElActionParametersEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElActionParametersEl {}

impl BuildDataRulesetsRulesetsElRulesElActionParametersEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElActionParametersEl {
        DataRulesetsRulesetsElRulesElActionParametersEl {
            automatic_https_rewrites: core::default::Default::default(),
            autominify: core::default::Default::default(),
            bic: core::default::Default::default(),
            browser_ttl: core::default::Default::default(),
            cache: core::default::Default::default(),
            cache_key: core::default::Default::default(),
            content: core::default::Default::default(),
            content_type: core::default::Default::default(),
            cookie_fields: core::default::Default::default(),
            disable_apps: core::default::Default::default(),
            disable_railgun: core::default::Default::default(),
            disable_zaraz: core::default::Default::default(),
            edge_ttl: core::default::Default::default(),
            email_obfuscation: core::default::Default::default(),
            from_list: core::default::Default::default(),
            from_value: core::default::Default::default(),
            headers: core::default::Default::default(),
            host_header: core::default::Default::default(),
            hotlink_protection: core::default::Default::default(),
            id: core::default::Default::default(),
            increment: core::default::Default::default(),
            matched_data: core::default::Default::default(),
            mirage: core::default::Default::default(),
            opportunistic_encryption: core::default::Default::default(),
            origin: core::default::Default::default(),
            origin_error_page_passthru: core::default::Default::default(),
            overrides: core::default::Default::default(),
            phases: core::default::Default::default(),
            polish: core::default::Default::default(),
            products: core::default::Default::default(),
            request_fields: core::default::Default::default(),
            respect_strong_etags: core::default::Default::default(),
            response: core::default::Default::default(),
            response_fields: core::default::Default::default(),
            rocket_loader: core::default::Default::default(),
            rules: core::default::Default::default(),
            ruleset: core::default::Default::default(),
            rulesets: core::default::Default::default(),
            security_level: core::default::Default::default(),
            serve_stale: core::default::Default::default(),
            server_side_excludes: core::default::Default::default(),
            sni: core::default::Default::default(),
            ssl: core::default::Default::default(),
            status_code: core::default::Default::default(),
            sxg: core::default::Default::default(),
            uri: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElActionParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElActionParametersElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElActionParametersElRef {
        DataRulesetsRulesetsElRulesElActionParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\n"]
    pub fn automatic_https_rewrites(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc= "Get a reference to the value of field `autominify` after provisioning.\n"]
    pub fn autominify(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autominify", self.base))
    }

    #[doc= "Get a reference to the value of field `bic` after provisioning.\n"]
    pub fn bic(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bic", self.base))
    }

    #[doc= "Get a reference to the value of field `browser_ttl` after provisioning.\n"]
    pub fn browser_ttl(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.browser_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_key` after provisioning.\n"]
    pub fn cache_key(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cache_key", self.base))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc= "Get a reference to the value of field `cookie_fields` after provisioning.\n"]
    pub fn cookie_fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cookie_fields", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_apps` after provisioning.\n"]
    pub fn disable_apps(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_apps", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_railgun` after provisioning.\n"]
    pub fn disable_railgun(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_railgun", self.base))
    }

    #[doc= "Get a reference to the value of field `disable_zaraz` after provisioning.\n"]
    pub fn disable_zaraz(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_zaraz", self.base))
    }

    #[doc= "Get a reference to the value of field `edge_ttl` after provisioning.\n"]
    pub fn edge_ttl(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.edge_ttl", self.base))
    }

    #[doc= "Get a reference to the value of field `email_obfuscation` after provisioning.\n"]
    pub fn email_obfuscation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc= "Get a reference to the value of field `from_list` after provisioning.\n"]
    pub fn from_list(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElFromListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.from_list", self.base))
    }

    #[doc= "Get a reference to the value of field `from_value` after provisioning.\n"]
    pub fn from_value(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef> {
        ListRef::new(self.shared().clone(), format!("{}.from_value", self.base))
    }

    #[doc= "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc= "Get a reference to the value of field `host_header` after provisioning.\n"]
    pub fn host_header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc= "Get a reference to the value of field `hotlink_protection` after provisioning.\n"]
    pub fn hotlink_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hotlink_protection", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `increment` after provisioning.\n"]
    pub fn increment(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.increment", self.base))
    }

    #[doc= "Get a reference to the value of field `matched_data` after provisioning.\n"]
    pub fn matched_data(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.matched_data", self.base))
    }

    #[doc= "Get a reference to the value of field `mirage` after provisioning.\n"]
    pub fn mirage(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc= "Get a reference to the value of field `opportunistic_encryption` after provisioning.\n"]
    pub fn opportunistic_encryption(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc= "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElOriginElRef> {
        ListRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc= "Get a reference to the value of field `origin_error_page_passthru` after provisioning.\n"]
    pub fn origin_error_page_passthru(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_passthru", self.base))
    }

    #[doc= "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }

    #[doc= "Get a reference to the value of field `phases` after provisioning.\n"]
    pub fn phases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.phases", self.base))
    }

    #[doc= "Get a reference to the value of field `polish` after provisioning.\n"]
    pub fn polish(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc= "Get a reference to the value of field `products` after provisioning.\n"]
    pub fn products(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.products", self.base))
    }

    #[doc= "Get a reference to the value of field `request_fields` after provisioning.\n"]
    pub fn request_fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.request_fields", self.base))
    }

    #[doc= "Get a reference to the value of field `respect_strong_etags` after provisioning.\n"]
    pub fn respect_strong_etags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.respect_strong_etags", self.base))
    }

    #[doc= "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }

    #[doc= "Get a reference to the value of field `response_fields` after provisioning.\n"]
    pub fn response_fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.response_fields", self.base))
    }

    #[doc= "Get a reference to the value of field `rocket_loader` after provisioning.\n"]
    pub fn rocket_loader(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rocket_loader", self.base))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc= "Get a reference to the value of field `ruleset` after provisioning.\n"]
    pub fn ruleset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ruleset", self.base))
    }

    #[doc= "Get a reference to the value of field `rulesets` after provisioning.\n"]
    pub fn rulesets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.rulesets", self.base))
    }

    #[doc= "Get a reference to the value of field `security_level` after provisioning.\n"]
    pub fn security_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc= "Get a reference to the value of field `serve_stale` after provisioning.\n"]
    pub fn serve_stale(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.serve_stale", self.base))
    }

    #[doc= "Get a reference to the value of field `server_side_excludes` after provisioning.\n"]
    pub fn server_side_excludes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_side_excludes", self.base))
    }

    #[doc= "Get a reference to the value of field `sni` after provisioning.\n"]
    pub fn sni(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElSniElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sni", self.base))
    }

    #[doc= "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc= "Get a reference to the value of field `sxg` after provisioning.\n"]
    pub fn sxg(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sxg", self.base))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElUriElRef> {
        ListRef::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElExposedCredentialCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username_expression: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElExposedCredentialCheckEl {
    #[doc= "Set the field `password_expression`.\n"]
    pub fn set_password_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_expression = Some(v.into());
        self
    }

    #[doc= "Set the field `username_expression`.\n"]
    pub fn set_username_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username_expression = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElExposedCredentialCheckEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElExposedCredentialCheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElExposedCredentialCheckEl {}

impl BuildDataRulesetsRulesetsElRulesElExposedCredentialCheckEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElExposedCredentialCheckEl {
        DataRulesetsRulesetsElRulesElExposedCredentialCheckEl {
            password_expression: core::default::Default::default(),
            username_expression: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef {
        DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `password_expression` after provisioning.\n"]
    pub fn password_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_expression", self.base))
    }

    #[doc= "Get a reference to the value of field `username_expression` after provisioning.\n"]
    pub fn username_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElLoggingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElLoggingEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElLoggingEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElLoggingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElLoggingEl {}

impl BuildDataRulesetsRulesetsElRulesElLoggingEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElLoggingEl {
        DataRulesetsRulesetsElRulesElLoggingEl {
            enabled: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElLoggingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElLoggingElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElLoggingElRef {
        DataRulesetsRulesetsElRulesElLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElRatelimitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    characteristics: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counting_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mitigation_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requests_per_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requests_to_origin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_per_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_response_header_name: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesElRatelimitEl {
    #[doc= "Set the field `characteristics`.\n"]
    pub fn set_characteristics(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.characteristics = Some(v.into());
        self
    }

    #[doc= "Set the field `counting_expression`.\n"]
    pub fn set_counting_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.counting_expression = Some(v.into());
        self
    }

    #[doc= "Set the field `mitigation_timeout`.\n"]
    pub fn set_mitigation_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mitigation_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `period`.\n"]
    pub fn set_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period = Some(v.into());
        self
    }

    #[doc= "Set the field `requests_per_period`.\n"]
    pub fn set_requests_per_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.requests_per_period = Some(v.into());
        self
    }

    #[doc= "Set the field `requests_to_origin`.\n"]
    pub fn set_requests_to_origin(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.requests_to_origin = Some(v.into());
        self
    }

    #[doc= "Set the field `score_per_period`.\n"]
    pub fn set_score_per_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.score_per_period = Some(v.into());
        self
    }

    #[doc= "Set the field `score_response_header_name`.\n"]
    pub fn set_score_response_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.score_response_header_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesElRatelimitEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesElRatelimitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesElRatelimitEl {}

impl BuildDataRulesetsRulesetsElRulesElRatelimitEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesElRatelimitEl {
        DataRulesetsRulesetsElRulesElRatelimitEl {
            characteristics: core::default::Default::default(),
            counting_expression: core::default::Default::default(),
            mitigation_timeout: core::default::Default::default(),
            period: core::default::Default::default(),
            requests_per_period: core::default::Default::default(),
            requests_to_origin: core::default::Default::default(),
            score_per_period: core::default::Default::default(),
            score_response_header_name: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElRatelimitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElRatelimitElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElRatelimitElRef {
        DataRulesetsRulesetsElRulesElRatelimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElRatelimitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `characteristics` after provisioning.\n"]
    pub fn characteristics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.characteristics", self.base))
    }

    #[doc= "Get a reference to the value of field `counting_expression` after provisioning.\n"]
    pub fn counting_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.counting_expression", self.base))
    }

    #[doc= "Get a reference to the value of field `mitigation_timeout` after provisioning.\n"]
    pub fn mitigation_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mitigation_timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc= "Get a reference to the value of field `requests_per_period` after provisioning.\n"]
    pub fn requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.requests_per_period", self.base))
    }

    #[doc= "Get a reference to the value of field `requests_to_origin` after provisioning.\n"]
    pub fn requests_to_origin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.requests_to_origin", self.base))
    }

    #[doc= "Get a reference to the value of field `score_per_period` after provisioning.\n"]
    pub fn score_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.score_per_period", self.base))
    }

    #[doc= "Get a reference to the value of field `score_response_header_name` after provisioning.\n"]
    pub fn score_response_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.score_response_header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_parameters: Option<ListField<DataRulesetsRulesetsElRulesElActionParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exposed_credential_check: Option<ListField<DataRulesetsRulesetsElRulesElExposedCredentialCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_updated: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<ListField<DataRulesetsRulesetsElRulesElLoggingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ratelimit: Option<ListField<DataRulesetsRulesetsElRulesElRatelimitEl>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsElRulesEl {
    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc= "Set the field `action_parameters`.\n"]
    pub fn set_action_parameters(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElActionParametersEl>>,
    ) -> Self {
        self.action_parameters = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `exposed_credential_check`.\n"]
    pub fn set_exposed_credential_check(
        mut self,
        v: impl Into<ListField<DataRulesetsRulesetsElRulesElExposedCredentialCheckEl>>,
    ) -> Self {
        self.exposed_credential_check = Some(v.into());
        self
    }

    #[doc= "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `last_updated`.\n"]
    pub fn set_last_updated(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_updated = Some(v.into());
        self
    }

    #[doc= "Set the field `logging`.\n"]
    pub fn set_logging(mut self, v: impl Into<ListField<DataRulesetsRulesetsElRulesElLoggingEl>>) -> Self {
        self.logging = Some(v.into());
        self
    }

    #[doc= "Set the field `ratelimit`.\n"]
    pub fn set_ratelimit(mut self, v: impl Into<ListField<DataRulesetsRulesetsElRulesElRatelimitEl>>) -> Self {
        self.ratelimit = Some(v.into());
        self
    }

    #[doc= "Set the field `ref_`.\n"]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsElRulesEl {
    type O = BlockAssignable<DataRulesetsRulesetsElRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsElRulesEl {}

impl BuildDataRulesetsRulesetsElRulesEl {
    pub fn build(self) -> DataRulesetsRulesetsElRulesEl {
        DataRulesetsRulesetsElRulesEl {
            action: core::default::Default::default(),
            action_parameters: core::default::Default::default(),
            description: core::default::Default::default(),
            enabled: core::default::Default::default(),
            exposed_credential_check: core::default::Default::default(),
            expression: core::default::Default::default(),
            id: core::default::Default::default(),
            last_updated: core::default::Default::default(),
            logging: core::default::Default::default(),
            ratelimit: core::default::Default::default(),
            ref_: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRulesElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRulesElRef {
        DataRulesetsRulesetsElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `action_parameters` after provisioning.\n"]
    pub fn action_parameters(&self) -> ListRef<DataRulesetsRulesetsElRulesElActionParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action_parameters", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `exposed_credential_check` after provisioning.\n"]
    pub fn exposed_credential_check(&self) -> ListRef<DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exposed_credential_check", self.base))
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc= "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<DataRulesetsRulesetsElRulesElLoggingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc= "Get a reference to the value of field `ratelimit` after provisioning.\n"]
    pub fn ratelimit(&self) -> ListRef<DataRulesetsRulesetsElRulesElRatelimitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ratelimit", self.base))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phase: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<ListField<DataRulesetsRulesetsElRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataRulesetsRulesetsEl {
    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `kind`.\n"]
    pub fn set_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kind = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `phase`.\n"]
    pub fn set_phase(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.phase = Some(v.into());
        self
    }

    #[doc= "Set the field `rules`.\n"]
    pub fn set_rules(mut self, v: impl Into<ListField<DataRulesetsRulesetsElRulesEl>>) -> Self {
        self.rules = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsRulesetsEl {
    type O = BlockAssignable<DataRulesetsRulesetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsRulesetsEl {}

impl BuildDataRulesetsRulesetsEl {
    pub fn build(self) -> DataRulesetsRulesetsEl {
        DataRulesetsRulesetsEl {
            description: core::default::Default::default(),
            id: core::default::Default::default(),
            kind: core::default::Default::default(),
            name: core::default::Default::default(),
            phase: core::default::Default::default(),
            rules: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsRulesetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsRulesetsElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsRulesetsElRef {
        DataRulesetsRulesetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `phase` after provisioning.\n"]
    pub fn phase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phase", self.base))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<DataRulesetsRulesetsElRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phase: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataRulesetsFilterEl {
    #[doc= "Set the field `id`.\nThe ID of the Ruleset to target."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `kind`.\nType of Ruleset to create. Available values: `custom`, `managed`, `root`, `schema`, `zone`."]
    pub fn set_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kind = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nName of the ruleset."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `phase`.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn set_phase(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.phase = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\nVersion of the ruleset to filter on."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataRulesetsFilterEl {
    type O = BlockAssignable<DataRulesetsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRulesetsFilterEl {}

impl BuildDataRulesetsFilterEl {
    pub fn build(self) -> DataRulesetsFilterEl {
        DataRulesetsFilterEl {
            id: core::default::Default::default(),
            kind: core::default::Default::default(),
            name: core::default::Default::default(),
            phase: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataRulesetsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRulesetsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataRulesetsFilterElRef {
        DataRulesetsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the Ruleset to target."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nType of Ruleset to create. Available values: `custom`, `managed`, `root`, `schema`, `zone`."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the ruleset."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `phase` after provisioning.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn phase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phase", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nVersion of the ruleset to filter on."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataRulesetsDynamic {
    filter: Option<DynamicBlock<DataRulesetsFilterEl>>,
}
