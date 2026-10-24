use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataRulesetsData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_rules: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataRulesetsFilterEl>>,
    dynamic: DataRulesetsDynamic,
}

struct DataRulesets_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRulesetsData>,
}

#[derive(Clone)]
pub struct DataRulesets(std::rc::Rc<DataRulesets_>);

impl DataRulesets {
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

    #[doc =
        "Set the field `account_id`.\nThe account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_rules`.\nInclude rule data in response."]
    pub fn set_include_rules(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone_id`.\nThe zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataRulesetsFilterEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_rules` after provisioning.\nInclude rule data in response."]
    pub fn include_rules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rulesets` after provisioning.\n"]
    pub fn rulesets(&self) -> terrars::ListRef<DataRulesetsRulesetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rulesets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataRulesetsFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

impl terrars::Referable for DataRulesets {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRulesets { }

impl terrars::ToListMappable for DataRulesets {
    type O = terrars::ListRef<DataRulesetsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRulesets_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_rulesets".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRulesets {
    pub tf_id: std::string::String,
}

impl BuildDataRulesets {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRulesets {
        let out = DataRulesets(std::rc::Rc::new(DataRulesets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRulesetsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                include_rules: core::default::Default::default(),
                zone_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRulesetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRulesetsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_rules` after provisioning.\nInclude rule data in response."]
    pub fn include_rules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rulesets` after provisioning.\n"]
    pub fn rulesets(&self) -> terrars::ListRef<DataRulesetsRulesetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rulesets", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataRulesetsFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    css: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    html: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    js: std::option::Option<terrars::PrimField<bool>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl {
    #[doc = "Set the field `css`.\n"]
    pub fn set_css(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.css = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `html`.\n"]
    pub fn set_html(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.html = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `js`.\n"]
    pub fn set_js(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.js = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef {
        DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `css` after provisioning.\n"]
    pub fn css(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.css", self.base))
    }

    #[doc = "Get a reference to the value of field `html` after provisioning.\n"]
    pub fn html(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }

    #[doc = "Get a reference to the value of field `js` after provisioning.\n"]
    pub fn js(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.js", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl {
    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef {
        DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_presence: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    #[doc = "Set the field `check_presence`.\n"]
    pub fn set_check_presence(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.check_presence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\n"]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `check_presence` after provisioning.\n"]
    pub fn check_presence(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_presence: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    #[doc = "Set the field `check_presence`.\n"]
    pub fn set_check_presence(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.check_presence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_origin`.\n"]
    pub fn set_exclude_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\n"]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `check_presence` after provisioning.\n"]
    pub fn check_presence(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_presence", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_origin` after provisioning.\n"]
    pub fn exclude_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolved: std::option::Option<terrars::PrimField<bool>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    #[doc = "Set the field `resolved`.\n"]
    pub fn set_resolved(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.resolved = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolved` after provisioning.\n"]
    pub fn resolved(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolved", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    #[doc = "Set the field `exclude`.\n"]
    pub fn set_exclude(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include`.\n"]
    pub fn set_include(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl {
    type O =
        terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lang: std::option::Option<terrars::PrimField<bool>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
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

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef {
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

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie: std
    ::option
    ::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std
    ::option
    ::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std
    ::option
    ::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std
    ::option
    ::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std
    ::option
    ::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl {
    #[doc = "Set the field `cookie`.\n"]
    pub fn set_cookie(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieEl>,
                    >,
    ) -> Self {
        self.cookie = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderEl>,
                    >,
    ) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\n"]
    pub fn set_host(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostEl>,
                    >,
    ) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringEl>,
                    >,
    ) -> Self {
        self.query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserEl>,
                    >,
    ) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie` after provisioning.\n"]
    pub fn cookie(
        &self,
    ) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElCookieElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(
        &self,
    ) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElHostElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElQueryStringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(
        &self,
    ) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElUserElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_by_device_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_deception_armor: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_key: std
    ::option
    ::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_query_strings_order: std::option::Option<terrars::PrimField<bool>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl {
    #[doc = "Set the field `cache_by_device_type`.\n"]
    pub fn set_cache_by_device_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cache_by_device_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_deception_armor`.\n"]
    pub fn set_cache_deception_armor(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cache_deception_armor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_key`.\n"]
    pub fn set_custom_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyEl>>,
    ) -> Self {
        self.custom_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_query_strings_order`.\n"]
    pub fn set_ignore_query_strings_order(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_query_strings_order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef {
        DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_by_device_type` after provisioning.\n"]
    pub fn cache_by_device_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_by_device_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_deception_armor` after provisioning.\n"]
    pub fn cache_deception_armor(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_deception_armor", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_key` after provisioning.\n"]
    pub fn custom_key(
        &self,
    ) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElCustomKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_key", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_query_strings_order` after provisioning.\n"]
    pub fn ignore_query_strings_order(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_query_strings_order", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl {
    type O =
        terrars
        ::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
        DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code_range: std
    ::option
    ::Option<
        terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code_range`.\n"]
    pub fn set_status_code_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeEl,
                        >,
                    >,
    ) -> Self {
        self.status_code_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
        DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code_range` after provisioning.\n"]
    pub fn status_code_range(
        &self,
    ) -> terrars
    ::ListRef<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElStatusCodeRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status_code_range", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code_ttl: std
    ::option
    ::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl {
    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code_ttl`.\n"]
    pub fn set_status_code_ttl(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlEl>,
                    >,
    ) -> Self {
        self.status_code_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef {
        DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code_ttl` after provisioning.\n"]
    pub fn status_code_ttl(
        &self,
    ) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElStatusCodeTtlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status_code_ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElFromListEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromListEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElFromListEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElFromListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElFromListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElFromListElRef {
        DataRulesetsRulesetsElRulesElActionParametersElFromListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromListElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef {
        DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElFromValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_query_string: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_url: std
    ::option
    ::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromValueEl {
    #[doc = "Set the field `preserve_query_string`.\n"]
    pub fn set_preserve_query_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_url`.\n"]
    pub fn set_target_url(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlEl>>,
    ) -> Self {
        self.target_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElFromValueEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElFromValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef {
        DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `preserve_query_string` after provisioning.\n"]
    pub fn preserve_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc = "Get a reference to the value of field `target_url` after provisioning.\n"]
    pub fn target_url(
        &self,
    ) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElFromValueElTargetUrlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElHeadersEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElHeadersEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef {
        DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl {
    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef {
        DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElOriginEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElOriginEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElOriginEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElOriginEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElOriginElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElOriginElRef {
        DataRulesetsRulesetsElRulesElActionParametersElOriginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElOriginElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    category: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef {
        DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `score_threshold`.\n"]
    pub fn set_score_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.score_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_level`.\n"]
    pub fn set_sensitivity_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensitivity_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef {
        DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `score_threshold` after provisioning.\n"]
    pub fn score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_level` after provisioning.\n"]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElOverridesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    categories: std
    ::option
    ::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `categories`.\n"]
    pub fn set_categories(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesEl>,
                    >,
    ) -> Self {
        self.categories = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesEl>>,
    ) -> Self {
        self.rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_level`.\n"]
    pub fn set_sensitivity_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensitivity_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElOverridesEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElOverridesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef {
        DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `categories` after provisioning.\n"]
    pub fn categories(
        &self,
    ) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElOverridesElCategoriesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categories", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElOverridesElRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_level` after provisioning.\n"]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElResponseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElResponseEl {
    #[doc = "Set the field `content`.\n"]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElResponseEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElResponseElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElResponseElRef {
        DataRulesetsRulesetsElRulesElActionParametersElResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElResponseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_stale_while_updating: std::option::Option<terrars::PrimField<bool>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl {
    #[doc = "Set the field `disable_stale_while_updating`.\n"]
    pub fn set_disable_stale_while_updating(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_stale_while_updating = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef {
        DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_stale_while_updating` after provisioning.\n"]
    pub fn disable_stale_while_updating(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_stale_while_updating", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElSniEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElSniEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElSniEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElSniEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElSniElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElSniElRef {
        DataRulesetsRulesetsElRulesElActionParametersElSniElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElSniElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef {
        DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef {
        DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersElUriEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriEl {
    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElUriElPathEl>>,
    ) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query`.\n"]
    pub fn set_query(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElUriElQueryEl>>,
    ) -> Self {
        self.query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersElUriEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersElUriEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElUriElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElUriElRef {
        DataRulesetsRulesetsElRulesElActionParametersElUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElUriElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElUriElPathElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElUriElQueryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElActionParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_https_rewrites: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autominify: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_ttl: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_fields: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_apps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_railgun: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_zaraz: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_ttl: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_obfuscation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_list: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElFromListEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_value: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElFromValueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElHeadersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotlink_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    increment: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matched_data: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mirage: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opportunistic_encryption: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElOriginEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_error_page_passthru: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overrides: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phases: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    polish: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    products: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_fields: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respect_strong_etags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElResponseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_fields: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rocket_loader: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ruleset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rulesets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serve_stale: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_excludes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sni: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElSniEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sxg: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElUriEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElActionParametersEl {
    #[doc = "Set the field `automatic_https_rewrites`.\n"]
    pub fn set_automatic_https_rewrites(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.automatic_https_rewrites = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autominify`.\n"]
    pub fn set_autominify(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElAutominifyEl>>,
    ) -> Self {
        self.autominify = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bic`.\n"]
    pub fn set_bic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `browser_ttl`.\n"]
    pub fn set_browser_ttl(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlEl>>,
    ) -> Self {
        self.browser_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache`.\n"]
    pub fn set_cache(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cache = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_key`.\n"]
    pub fn set_cache_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyEl>>,
    ) -> Self {
        self.cache_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content`.\n"]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cookie_fields`.\n"]
    pub fn set_cookie_fields(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cookie_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_apps`.\n"]
    pub fn set_disable_apps(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_apps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_railgun`.\n"]
    pub fn set_disable_railgun(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_railgun = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_zaraz`.\n"]
    pub fn set_disable_zaraz(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_zaraz = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edge_ttl`.\n"]
    pub fn set_edge_ttl(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlEl>>,
    ) -> Self {
        self.edge_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_obfuscation`.\n"]
    pub fn set_email_obfuscation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.email_obfuscation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_list`.\n"]
    pub fn set_from_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElFromListEl>>,
    ) -> Self {
        self.from_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_value`.\n"]
    pub fn set_from_value(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElFromValueEl>>,
    ) -> Self {
        self.from_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElHeadersEl>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_header`.\n"]
    pub fn set_host_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hotlink_protection`.\n"]
    pub fn set_hotlink_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.hotlink_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `increment`.\n"]
    pub fn set_increment(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.increment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matched_data`.\n"]
    pub fn set_matched_data(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElMatchedDataEl>>,
    ) -> Self {
        self.matched_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mirage`.\n"]
    pub fn set_mirage(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mirage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `opportunistic_encryption`.\n"]
    pub fn set_opportunistic_encryption(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.opportunistic_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElOriginEl>>,
    ) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_error_page_passthru`.\n"]
    pub fn set_origin_error_page_passthru(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.origin_error_page_passthru = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElOverridesEl>>,
    ) -> Self {
        self.overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phases`.\n"]
    pub fn set_phases(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.phases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `polish`.\n"]
    pub fn set_polish(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.polish = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `products`.\n"]
    pub fn set_products(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.products = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_fields`.\n"]
    pub fn set_request_fields(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.request_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `respect_strong_etags`.\n"]
    pub fn set_respect_strong_etags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.respect_strong_etags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElResponseEl>>,
    ) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_fields`.\n"]
    pub fn set_response_fields(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.response_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rocket_loader`.\n"]
    pub fn set_rocket_loader(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.rocket_loader = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ruleset`.\n"]
    pub fn set_ruleset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ruleset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rulesets`.\n"]
    pub fn set_rulesets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rulesets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_level`.\n"]
    pub fn set_security_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serve_stale`.\n"]
    pub fn set_serve_stale(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElServeStaleEl>>,
    ) -> Self {
        self.serve_stale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_excludes`.\n"]
    pub fn set_server_side_excludes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.server_side_excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sni`.\n"]
    pub fn set_sni(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElSniEl>>,
    ) -> Self {
        self.sni = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sxg`.\n"]
    pub fn set_sxg(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sxg = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersElUriEl>>,
    ) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElActionParametersEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElActionParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElActionParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElActionParametersElRef {
        DataRulesetsRulesetsElRulesElActionParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElActionParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automatic_https_rewrites` after provisioning.\n"]
    pub fn automatic_https_rewrites(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_https_rewrites", self.base))
    }

    #[doc = "Get a reference to the value of field `autominify` after provisioning.\n"]
    pub fn autominify(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElAutominifyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autominify", self.base))
    }

    #[doc = "Get a reference to the value of field `bic` after provisioning.\n"]
    pub fn bic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bic", self.base))
    }

    #[doc = "Get a reference to the value of field `browser_ttl` after provisioning.\n"]
    pub fn browser_ttl(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElBrowserTtlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.browser_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_key` after provisioning.\n"]
    pub fn cache_key(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElCacheKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_key", self.base))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cookie_fields` after provisioning.\n"]
    pub fn cookie_fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cookie_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_apps` after provisioning.\n"]
    pub fn disable_apps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_apps", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_railgun` after provisioning.\n"]
    pub fn disable_railgun(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_railgun", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_zaraz` after provisioning.\n"]
    pub fn disable_zaraz(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_zaraz", self.base))
    }

    #[doc = "Get a reference to the value of field `edge_ttl` after provisioning.\n"]
    pub fn edge_ttl(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElEdgeTtlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edge_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `email_obfuscation` after provisioning.\n"]
    pub fn email_obfuscation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_obfuscation", self.base))
    }

    #[doc = "Get a reference to the value of field `from_list` after provisioning.\n"]
    pub fn from_list(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElFromListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.from_list", self.base))
    }

    #[doc = "Get a reference to the value of field `from_value` after provisioning.\n"]
    pub fn from_value(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElFromValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.from_value", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `host_header` after provisioning.\n"]
    pub fn host_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc = "Get a reference to the value of field `hotlink_protection` after provisioning.\n"]
    pub fn hotlink_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hotlink_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `increment` after provisioning.\n"]
    pub fn increment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.increment", self.base))
    }

    #[doc = "Get a reference to the value of field `matched_data` after provisioning.\n"]
    pub fn matched_data(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElMatchedDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matched_data", self.base))
    }

    #[doc = "Get a reference to the value of field `mirage` after provisioning.\n"]
    pub fn mirage(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirage", self.base))
    }

    #[doc = "Get a reference to the value of field `opportunistic_encryption` after provisioning.\n"]
    pub fn opportunistic_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opportunistic_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElOriginElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_error_page_passthru` after provisioning.\n"]
    pub fn origin_error_page_passthru(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_error_page_passthru", self.base))
    }

    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElOverridesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `phases` after provisioning.\n"]
    pub fn phases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.phases", self.base))
    }

    #[doc = "Get a reference to the value of field `polish` after provisioning.\n"]
    pub fn polish(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.polish", self.base))
    }

    #[doc = "Get a reference to the value of field `products` after provisioning.\n"]
    pub fn products(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.products", self.base))
    }

    #[doc = "Get a reference to the value of field `request_fields` after provisioning.\n"]
    pub fn request_fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.request_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `respect_strong_etags` after provisioning.\n"]
    pub fn respect_strong_etags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.respect_strong_etags", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }

    #[doc = "Get a reference to the value of field `response_fields` after provisioning.\n"]
    pub fn response_fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.response_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `rocket_loader` after provisioning.\n"]
    pub fn rocket_loader(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rocket_loader", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc = "Get a reference to the value of field `ruleset` after provisioning.\n"]
    pub fn ruleset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ruleset", self.base))
    }

    #[doc = "Get a reference to the value of field `rulesets` after provisioning.\n"]
    pub fn rulesets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rulesets", self.base))
    }

    #[doc = "Get a reference to the value of field `security_level` after provisioning.\n"]
    pub fn security_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc = "Get a reference to the value of field `serve_stale` after provisioning.\n"]
    pub fn serve_stale(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElServeStaleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serve_stale", self.base))
    }

    #[doc = "Get a reference to the value of field `server_side_excludes` after provisioning.\n"]
    pub fn server_side_excludes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_side_excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `sni` after provisioning.\n"]
    pub fn sni(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElSniElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sni", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }

    #[doc = "Get a reference to the value of field `sxg` after provisioning.\n"]
    pub fn sxg(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sxg", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElUriElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElExposedCredentialCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElExposedCredentialCheckEl {
    #[doc = "Set the field `password_expression`.\n"]
    pub fn set_password_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username_expression`.\n"]
    pub fn set_username_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.username_expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElExposedCredentialCheckEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElExposedCredentialCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef {
        DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password_expression` after provisioning.\n"]
    pub fn password_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `username_expression` after provisioning.\n"]
    pub fn username_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElLoggingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElLoggingEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElLoggingEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElLoggingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElLoggingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRulesetsRulesetsElRulesElLoggingElRef {
        DataRulesetsRulesetsElRulesElLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElLoggingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesElRatelimitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    characteristics: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    counting_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mitigation_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests_per_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests_to_origin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_per_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score_response_header_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesElRatelimitEl {
    #[doc = "Set the field `characteristics`.\n"]
    pub fn set_characteristics(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.characteristics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `counting_expression`.\n"]
    pub fn set_counting_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.counting_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mitigation_timeout`.\n"]
    pub fn set_mitigation_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mitigation_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period`.\n"]
    pub fn set_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requests_per_period`.\n"]
    pub fn set_requests_per_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.requests_per_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requests_to_origin`.\n"]
    pub fn set_requests_to_origin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.requests_to_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `score_per_period`.\n"]
    pub fn set_score_per_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.score_per_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `score_response_header_name`.\n"]
    pub fn set_score_response_header_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.score_response_header_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesElRatelimitEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesElRatelimitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElRatelimitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRulesetsRulesetsElRulesElRatelimitElRef {
        DataRulesetsRulesetsElRulesElRatelimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElRatelimitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `characteristics` after provisioning.\n"]
    pub fn characteristics(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.characteristics", self.base))
    }

    #[doc = "Get a reference to the value of field `counting_expression` after provisioning.\n"]
    pub fn counting_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.counting_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `mitigation_timeout` after provisioning.\n"]
    pub fn mitigation_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mitigation_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }

    #[doc = "Get a reference to the value of field `requests_per_period` after provisioning.\n"]
    pub fn requests_per_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests_per_period", self.base))
    }

    #[doc = "Get a reference to the value of field `requests_to_origin` after provisioning.\n"]
    pub fn requests_to_origin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requests_to_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `score_per_period` after provisioning.\n"]
    pub fn score_per_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_per_period", self.base))
    }

    #[doc = "Get a reference to the value of field `score_response_header_name` after provisioning.\n"]
    pub fn score_response_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_response_header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsElRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_parameters: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exposed_credential_check: std
    ::option
    ::Option<terrars::ListField<DataRulesetsRulesetsElRulesElExposedCredentialCheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_updated: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElLoggingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ratelimit: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesElRatelimitEl>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsElRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action_parameters`.\n"]
    pub fn set_action_parameters(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElActionParametersEl>>,
    ) -> Self {
        self.action_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exposed_credential_check`.\n"]
    pub fn set_exposed_credential_check(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElExposedCredentialCheckEl>>,
    ) -> Self {
        self.exposed_credential_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_updated`.\n"]
    pub fn set_last_updated(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_updated = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElLoggingEl>>,
    ) -> Self {
        self.logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ratelimit`.\n"]
    pub fn set_ratelimit(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesElRatelimitEl>>,
    ) -> Self {
        self.ratelimit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ref_`.\n"]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsElRulesEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsElRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRulesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRulesetsRulesetsElRulesElRef {
        DataRulesetsRulesetsElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `action_parameters` after provisioning.\n"]
    pub fn action_parameters(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElActionParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `exposed_credential_check` after provisioning.\n"]
    pub fn exposed_credential_check(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElExposedCredentialCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exposed_credential_check", self.base))
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc = "Get a reference to the value of field `ratelimit` after provisioning.\n"]
    pub fn ratelimit(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElRatelimitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ratelimit", self.base))
    }

    #[doc = "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsRulesetsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<terrars::ListField<DataRulesetsRulesetsElRulesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsRulesetsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kind`.\n"]
    pub fn set_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kind = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phase`.\n"]
    pub fn set_phase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(mut self, v: impl std::convert::Into<terrars::ListField<DataRulesetsRulesetsElRulesEl>>) -> Self {
        self.rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsRulesetsEl {
    type O = terrars::BlockAssignable<DataRulesetsRulesetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsRulesetsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRulesetsRulesetsElRef {
        DataRulesetsRulesetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsRulesetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `phase` after provisioning.\n"]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataRulesetsRulesetsElRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRulesetsFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRulesetsFilterEl {
    #[doc = "Set the field `id`.\nThe ID of the Ruleset to target."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kind`.\nType of Ruleset to create. Available values: `custom`, `managed`, `root`, `schema`, `zone`."]
    pub fn set_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kind = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the ruleset."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `phase`.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn set_phase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nVersion of the ruleset to filter on."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRulesetsFilterEl {
    type O = terrars::BlockAssignable<DataRulesetsFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRulesetsFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRulesetsFilterElRef {
        DataRulesetsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRulesetsFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the Ruleset to target."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of Ruleset to create. Available values: `custom`, `managed`, `root`, `schema`, `zone`."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the ruleset."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phase` after provisioning.\nPoint in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_custom_errors`, `http_log_custom_fields`, `http_request_cache_settings`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_late_transform_managed`, `http_request_main`, `http_request_origin`, `http_request_dynamic_redirect`, `http_request_redirect`, `http_request_sanitize`, `http_request_transform`, `http_response_firewall_managed`, `http_response_headers_transform`, `http_response_headers_transform_managed`, `http_response_compression`, `magic_transit`, `http_ratelimit`, `http_request_sbfm`, `http_config_settings`."]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the ruleset to filter on."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataRulesetsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataRulesetsFilterEl>>,
}
