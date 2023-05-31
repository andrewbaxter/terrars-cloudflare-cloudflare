use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DlpProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    allowed_match_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry: Option<Vec<DlpProfileEntryEl>>,
    dynamic: DlpProfileDynamic,
}

struct DlpProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DlpProfileData>,
}

#[derive(Clone)]
pub struct DlpProfile(Rc<DlpProfile_>);

impl DlpProfile {
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

    #[doc= "Set the field `description`.\nBrief summary of the profile and its intended use."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `entry`.\n"]
    pub fn set_entry(self, v: impl Into<BlockAssignable<DlpProfileEntryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entry = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entry = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allowed_match_count` after provisioning.\nRelated DLP policies will trigger when the match count exceeds the number set."]
    pub fn allowed_match_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allowed_match_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nBrief summary of the profile and its intended use."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the profile. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl Referable for DlpProfile {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DlpProfile { }

impl ToListMappable for DlpProfile {
    type O = ListRef<DlpProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DlpProfile_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_dlp_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDlpProfile {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub account_id: PrimField<String>,
    #[doc= "Related DLP policies will trigger when the match count exceeds the number set."]
    pub allowed_match_count: PrimField<f64>,
    #[doc= "Name of the profile. **Modifying this attribute will force creation of a new resource.**"]
    pub name: PrimField<String>,
    #[doc= "The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**"]
    pub type_: PrimField<String>,
}

impl BuildDlpProfile {
    pub fn build(self, stack: &mut Stack) -> DlpProfile {
        let out = DlpProfile(Rc::new(DlpProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DlpProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                allowed_match_count: self.allowed_match_count,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                type_: self.type_,
                entry: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DlpProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlpProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DlpProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allowed_match_count` after provisioning.\nRelated DLP policies will trigger when the match count exceeds the number set."]
    pub fn allowed_match_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allowed_match_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nBrief summary of the profile and its intended use."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the profile. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DlpProfileEntryElPatternEl {
    regex: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<PrimField<String>>,
}

impl DlpProfileEntryElPatternEl {
    #[doc= "Set the field `validation`.\nThe validation algorithm to apply with this pattern."]
    pub fn set_validation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.validation = Some(v.into());
        self
    }
}

impl ToListMappable for DlpProfileEntryElPatternEl {
    type O = BlockAssignable<DlpProfileEntryElPatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlpProfileEntryElPatternEl {
    #[doc= "The regex that defines the pattern."]
    pub regex: PrimField<String>,
}

impl BuildDlpProfileEntryElPatternEl {
    pub fn build(self) -> DlpProfileEntryElPatternEl {
        DlpProfileEntryElPatternEl {
            regex: self.regex,
            validation: core::default::Default::default(),
        }
    }
}

pub struct DlpProfileEntryElPatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlpProfileEntryElPatternElRef {
    fn new(shared: StackShared, base: String) -> DlpProfileEntryElPatternElRef {
        DlpProfileEntryElPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlpProfileEntryElPatternElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `regex` after provisioning.\nThe regex that defines the pattern."]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc= "Get a reference to the value of field `validation` after provisioning.\nThe validation algorithm to apply with this pattern."]
    pub fn validation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlpProfileEntryElDynamic {
    pattern: Option<DynamicBlock<DlpProfileEntryElPatternEl>>,
}

#[derive(Serialize)]
pub struct DlpProfileEntryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<Vec<DlpProfileEntryElPatternEl>>,
    dynamic: DlpProfileEntryElDynamic,
}

impl DlpProfileEntryEl {
    #[doc= "Set the field `enabled`.\nWhether the entry is active. Defaults to `false`."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\nUnique entry identifier."]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `pattern`.\n"]
    pub fn set_pattern(mut self, v: impl Into<BlockAssignable<DlpProfileEntryElPatternEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pattern = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pattern = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DlpProfileEntryEl {
    type O = BlockAssignable<DlpProfileEntryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlpProfileEntryEl {
    #[doc= "Name of the entry to deploy."]
    pub name: PrimField<String>,
}

impl BuildDlpProfileEntryEl {
    pub fn build(self) -> DlpProfileEntryEl {
        DlpProfileEntryEl {
            enabled: core::default::Default::default(),
            id: core::default::Default::default(),
            name: self.name,
            pattern: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DlpProfileEntryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DlpProfileEntryElRef {
    fn new(shared: StackShared, base: String) -> DlpProfileEntryElRef {
        DlpProfileEntryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlpProfileEntryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether the entry is active. Defaults to `false`."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique entry identifier."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the entry to deploy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> ListRef<DlpProfileEntryElPatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlpProfileDynamic {
    entry: Option<DynamicBlock<DlpProfileEntryEl>>,
}
