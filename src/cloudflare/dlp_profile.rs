use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DlpProfileData {
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
    account_id: terrars::PrimField<std::string::String>,
    allowed_match_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry: std::option::Option<std::vec::Vec<DlpProfileEntryEl>>,
    dynamic: DlpProfileDynamic,
}

struct DlpProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DlpProfileData>,
}

#[derive(Clone)]
pub struct DlpProfile(std::rc::Rc<DlpProfile_>);

impl DlpProfile {
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

    #[doc = "Set the field `description`.\nBrief summary of the profile and its intended use."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entry`.\n"]
    pub fn set_entry(self, v: impl std::convert::Into<terrars::BlockAssignable<DlpProfileEntryEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entry = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_match_count` after provisioning.\nRelated DLP policies will trigger when the match count exceeds the number set."]
    pub fn allowed_match_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_match_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nBrief summary of the profile and its intended use."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the profile. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DlpProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DlpProfile { }

impl terrars::ToListMappable for DlpProfile {
    type O = terrars::ListRef<DlpProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DlpProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_dlp_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDlpProfile {
    pub tf_id: std::string::String,
    #[doc =
        "The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Related DLP policies will trigger when the match count exceeds the number set."]
    pub allowed_match_count: terrars::PrimField<f64>,
    #[doc = "Name of the profile. **Modifying this attribute will force creation of a new resource.**"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDlpProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> DlpProfile {
        let out = DlpProfile(std::rc::Rc::new(DlpProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DlpProfileData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                allowed_match_count: self.allowed_match_count,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                type_: self.type_,
                entry: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DlpProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlpProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DlpProfileRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_match_count` after provisioning.\nRelated DLP policies will trigger when the match count exceeds the number set."]
    pub fn allowed_match_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_match_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nBrief summary of the profile and its intended use."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the profile. **Modifying this attribute will force creation of a new resource.**"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the profile. Available values: `custom`, `predefined`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DlpProfileEntryElPatternEl {
    regex: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DlpProfileEntryElPatternEl {
    #[doc = "Set the field `validation`.\nThe validation algorithm to apply with this pattern."]
    pub fn set_validation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.validation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlpProfileEntryElPatternEl {
    type O = terrars::BlockAssignable<DlpProfileEntryElPatternEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlpProfileEntryElPatternEl {
    #[doc = "The regex that defines the pattern."]
    pub regex: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlpProfileEntryElPatternElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DlpProfileEntryElPatternElRef {
        DlpProfileEntryElPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlpProfileEntryElPatternElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nThe regex that defines the pattern."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `validation` after provisioning.\nThe validation algorithm to apply with this pattern."]
    pub fn validation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlpProfileEntryElDynamic {
    pattern: std::option::Option<terrars::DynamicBlock<DlpProfileEntryElPatternEl>>,
}

#[derive(Serialize)]
pub struct DlpProfileEntryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pattern: std::option::Option<std::vec::Vec<DlpProfileEntryElPatternEl>>,
    dynamic: DlpProfileEntryElDynamic,
}

impl DlpProfileEntryEl {
    #[doc = "Set the field `enabled`.\nWhether the entry is active. Defaults to `false`."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nUnique entry identifier."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pattern`.\n"]
    pub fn set_pattern(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlpProfileEntryElPatternEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pattern = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pattern = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DlpProfileEntryEl {
    type O = terrars::BlockAssignable<DlpProfileEntryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlpProfileEntryEl {
    #[doc = "Name of the entry to deploy."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDlpProfileEntryEl {
    pub fn build(self) -> DlpProfileEntryEl {
        DlpProfileEntryEl {
            enabled: core::default::Default::default(),
            id: core::default::Default::default(),
            name: self.name,
            pattern: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DlpProfileEntryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlpProfileEntryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DlpProfileEntryElRef {
        DlpProfileEntryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlpProfileEntryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the entry is active. Defaults to `false`."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique entry identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the entry to deploy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> terrars::ListRef<DlpProfileEntryElPatternElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlpProfileDynamic {
    entry: std::option::Option<terrars::DynamicBlock<DlpProfileEntryEl>>,
}
