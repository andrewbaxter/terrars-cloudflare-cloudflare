use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AccessGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<Vec<AccessGroupExcludeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<Vec<AccessGroupIncludeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require: Option<Vec<AccessGroupRequireEl>>,
    dynamic: AccessGroupDynamic,
}

struct AccessGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessGroupData>,
}

#[derive(Clone)]
pub struct AccessGroup(Rc<AccessGroup_>);

impl AccessGroup {
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

    #[doc= "Set the field `account_id`.\nThe account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `zone_id`.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn set_zone_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `exclude`.\n"]
    pub fn set_exclude(self, v: impl Into<BlockAssignable<AccessGroupExcludeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `include`.\n"]
    pub fn set_include(self, v: impl Into<BlockAssignable<AccessGroupIncludeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `require`.\n"]
    pub fn set_require(self, v: impl Into<BlockAssignable<AccessGroupRequireEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().require = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.require = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> ListRef<AccessGroupExcludeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> ListRef<AccessGroupIncludeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require` after provisioning.\n"]
    pub fn require(&self) -> ListRef<AccessGroupRequireElRef> {
        ListRef::new(self.shared().clone(), format!("{}.require", self.extract_ref()))
    }
}

impl Referable for AccessGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AccessGroup { }

impl ToListMappable for AccessGroup {
    type O = ListRef<AccessGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AccessGroup_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_access_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessGroup {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildAccessGroup {
    pub fn build(self, stack: &mut Stack) -> AccessGroup {
        let out = AccessGroup(Rc::new(AccessGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccessGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                zone_id: core::default::Default::default(),
                exclude: core::default::Default::default(),
                include: core::default::Default::default(),
                require: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. Conflicts with `account_id`."]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> ListRef<AccessGroupExcludeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclude", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> ListRef<AccessGroupIncludeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.include", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require` after provisioning.\n"]
    pub fn require(&self) -> ListRef<AccessGroupRequireElRef> {
        ListRef::new(self.shared().clone(), format!("{}.require", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AccessGroupExcludeElAzureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessGroupExcludeElAzureEl {
    #[doc= "Set the field `id`.\nThe ID of the Azure group or user."]
    pub fn set_id(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\nThe ID of the Azure Identity provider."]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupExcludeElAzureEl {
    type O = BlockAssignable<AccessGroupExcludeElAzureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupExcludeElAzureEl {}

impl BuildAccessGroupExcludeElAzureEl {
    pub fn build(self) -> AccessGroupExcludeElAzureEl {
        AccessGroupExcludeElAzureEl {
            id: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupExcludeElAzureElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupExcludeElAzureElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupExcludeElAzureElRef {
        AccessGroupExcludeElAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupExcludeElAzureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the Azure group or user."]
    pub fn id(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of the Azure Identity provider."]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupExcludeElExternalEvaluationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keys_url: Option<PrimField<String>>,
}

impl AccessGroupExcludeElExternalEvaluationEl {
    #[doc= "Set the field `evaluate_url`.\n"]
    pub fn set_evaluate_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluate_url = Some(v.into());
        self
    }

    #[doc= "Set the field `keys_url`.\n"]
    pub fn set_keys_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keys_url = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupExcludeElExternalEvaluationEl {
    type O = BlockAssignable<AccessGroupExcludeElExternalEvaluationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupExcludeElExternalEvaluationEl {}

impl BuildAccessGroupExcludeElExternalEvaluationEl {
    pub fn build(self) -> AccessGroupExcludeElExternalEvaluationEl {
        AccessGroupExcludeElExternalEvaluationEl {
            evaluate_url: core::default::Default::default(),
            keys_url: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupExcludeElExternalEvaluationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupExcludeElExternalEvaluationElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupExcludeElExternalEvaluationElRef {
        AccessGroupExcludeElExternalEvaluationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupExcludeElExternalEvaluationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `evaluate_url` after provisioning.\n"]
    pub fn evaluate_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc= "Get a reference to the value of field `keys_url` after provisioning.\n"]
    pub fn keys_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupExcludeElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<ListField<PrimField<String>>>,
}

impl AccessGroupExcludeElGithubEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `teams`.\n"]
    pub fn set_teams(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.teams = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupExcludeElGithubEl {
    type O = BlockAssignable<AccessGroupExcludeElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupExcludeElGithubEl {}

impl BuildAccessGroupExcludeElGithubEl {
    pub fn build(self) -> AccessGroupExcludeElGithubEl {
        AccessGroupExcludeElGithubEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
            teams: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupExcludeElGithubElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupExcludeElGithubElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupExcludeElGithubElRef {
        AccessGroupExcludeElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupExcludeElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupExcludeElGsuiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessGroupExcludeElGsuiteEl {
    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupExcludeElGsuiteEl {
    type O = BlockAssignable<AccessGroupExcludeElGsuiteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupExcludeElGsuiteEl {}

impl BuildAccessGroupExcludeElGsuiteEl {
    pub fn build(self) -> AccessGroupExcludeElGsuiteEl {
        AccessGroupExcludeElGsuiteEl {
            email: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupExcludeElGsuiteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupExcludeElGsuiteElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupExcludeElGsuiteElRef {
        AccessGroupExcludeElGsuiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupExcludeElGsuiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupExcludeElOktaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<ListField<PrimField<String>>>,
}

impl AccessGroupExcludeElOktaEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupExcludeElOktaEl {
    type O = BlockAssignable<AccessGroupExcludeElOktaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupExcludeElOktaEl {}

impl BuildAccessGroupExcludeElOktaEl {
    pub fn build(self) -> AccessGroupExcludeElOktaEl {
        AccessGroupExcludeElOktaEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupExcludeElOktaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupExcludeElOktaElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupExcludeElOktaElRef {
        AccessGroupExcludeElOktaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupExcludeElOktaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupExcludeElSamlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessGroupExcludeElSamlEl {
    #[doc= "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_name = Some(v.into());
        self
    }

    #[doc= "Set the field `attribute_value`.\n"]
    pub fn set_attribute_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_value = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupExcludeElSamlEl {
    type O = BlockAssignable<AccessGroupExcludeElSamlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupExcludeElSamlEl {}

impl BuildAccessGroupExcludeElSamlEl {
    pub fn build(self) -> AccessGroupExcludeElSamlEl {
        AccessGroupExcludeElSamlEl {
            attribute_name: core::default::Default::default(),
            attribute_value: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupExcludeElSamlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupExcludeElSamlElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupExcludeElSamlElRef {
        AccessGroupExcludeElSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupExcludeElSamlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc= "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessGroupExcludeElDynamic {
    azure: Option<DynamicBlock<AccessGroupExcludeElAzureEl>>,
    external_evaluation: Option<DynamicBlock<AccessGroupExcludeElExternalEvaluationEl>>,
    github: Option<DynamicBlock<AccessGroupExcludeElGithubEl>>,
    gsuite: Option<DynamicBlock<AccessGroupExcludeElGsuiteEl>>,
    okta: Option<DynamicBlock<AccessGroupExcludeElOktaEl>>,
    saml: Option<DynamicBlock<AccessGroupExcludeElSamlEl>>,
}

#[derive(Serialize)]
pub struct AccessGroupExcludeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    any_valid_service_token: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_posture: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_domain: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    everyone: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_method: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_token: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure: Option<Vec<AccessGroupExcludeElAzureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_evaluation: Option<Vec<AccessGroupExcludeElExternalEvaluationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AccessGroupExcludeElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gsuite: Option<Vec<AccessGroupExcludeElGsuiteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    okta: Option<Vec<AccessGroupExcludeElOktaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml: Option<Vec<AccessGroupExcludeElSamlEl>>,
    dynamic: AccessGroupExcludeElDynamic,
}

impl AccessGroupExcludeEl {
    #[doc= "Set the field `any_valid_service_token`.\n"]
    pub fn set_any_valid_service_token(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.any_valid_service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_method = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }

    #[doc= "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.device_posture = Some(v.into());
        self
    }

    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `everyone`.\n"]
    pub fn set_everyone(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.everyone = Some(v.into());
        self
    }

    #[doc= "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.geo = Some(v.into());
        self
    }

    #[doc= "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.group = Some(v.into());
        self
    }

    #[doc= "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_list = Some(v.into());
        self
    }

    #[doc= "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.login_method = Some(v.into());
        self
    }

    #[doc= "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `azure`.\n"]
    pub fn set_azure(mut self, v: impl Into<BlockAssignable<AccessGroupExcludeElAzureEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl Into<BlockAssignable<AccessGroupExcludeElExternalEvaluationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_evaluation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_evaluation = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AccessGroupExcludeElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl Into<BlockAssignable<AccessGroupExcludeElGsuiteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gsuite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gsuite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl Into<BlockAssignable<AccessGroupExcludeElOktaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.okta = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.okta = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl Into<BlockAssignable<AccessGroupExcludeElSamlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.saml = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.saml = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessGroupExcludeEl {
    type O = BlockAssignable<AccessGroupExcludeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupExcludeEl {}

impl BuildAccessGroupExcludeEl {
    pub fn build(self) -> AccessGroupExcludeEl {
        AccessGroupExcludeEl {
            any_valid_service_token: core::default::Default::default(),
            auth_method: core::default::Default::default(),
            certificate: core::default::Default::default(),
            common_name: core::default::Default::default(),
            device_posture: core::default::Default::default(),
            email: core::default::Default::default(),
            email_domain: core::default::Default::default(),
            everyone: core::default::Default::default(),
            geo: core::default::Default::default(),
            group: core::default::Default::default(),
            ip: core::default::Default::default(),
            ip_list: core::default::Default::default(),
            login_method: core::default::Default::default(),
            service_token: core::default::Default::default(),
            azure: core::default::Default::default(),
            external_evaluation: core::default::Default::default(),
            github: core::default::Default::default(),
            gsuite: core::default::Default::default(),
            okta: core::default::Default::default(),
            saml: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AccessGroupExcludeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupExcludeElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupExcludeElRef {
        AccessGroupExcludeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupExcludeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `any_valid_service_token` after provisioning.\n"]
    pub fn any_valid_service_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_valid_service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc= "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.device_posture", self.base))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email_domain", self.base))
    }

    #[doc= "Get a reference to the value of field `everyone` after provisioning.\n"]
    pub fn everyone(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc= "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc= "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.login_method", self.base))
    }

    #[doc= "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> ListRef<AccessGroupExcludeElAzureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.azure", self.base))
    }

    #[doc= "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ListRef<AccessGroupExcludeElExternalEvaluationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_evaluation", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AccessGroupExcludeElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ListRef<AccessGroupExcludeElGsuiteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc= "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ListRef<AccessGroupExcludeElOktaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc= "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ListRef<AccessGroupExcludeElSamlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupIncludeElAzureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessGroupIncludeElAzureEl {
    #[doc= "Set the field `id`.\nThe ID of the Azure group or user."]
    pub fn set_id(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\nThe ID of the Azure Identity provider."]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupIncludeElAzureEl {
    type O = BlockAssignable<AccessGroupIncludeElAzureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupIncludeElAzureEl {}

impl BuildAccessGroupIncludeElAzureEl {
    pub fn build(self) -> AccessGroupIncludeElAzureEl {
        AccessGroupIncludeElAzureEl {
            id: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupIncludeElAzureElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupIncludeElAzureElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupIncludeElAzureElRef {
        AccessGroupIncludeElAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupIncludeElAzureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the Azure group or user."]
    pub fn id(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of the Azure Identity provider."]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupIncludeElExternalEvaluationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keys_url: Option<PrimField<String>>,
}

impl AccessGroupIncludeElExternalEvaluationEl {
    #[doc= "Set the field `evaluate_url`.\n"]
    pub fn set_evaluate_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluate_url = Some(v.into());
        self
    }

    #[doc= "Set the field `keys_url`.\n"]
    pub fn set_keys_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keys_url = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupIncludeElExternalEvaluationEl {
    type O = BlockAssignable<AccessGroupIncludeElExternalEvaluationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupIncludeElExternalEvaluationEl {}

impl BuildAccessGroupIncludeElExternalEvaluationEl {
    pub fn build(self) -> AccessGroupIncludeElExternalEvaluationEl {
        AccessGroupIncludeElExternalEvaluationEl {
            evaluate_url: core::default::Default::default(),
            keys_url: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupIncludeElExternalEvaluationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupIncludeElExternalEvaluationElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupIncludeElExternalEvaluationElRef {
        AccessGroupIncludeElExternalEvaluationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupIncludeElExternalEvaluationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `evaluate_url` after provisioning.\n"]
    pub fn evaluate_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc= "Get a reference to the value of field `keys_url` after provisioning.\n"]
    pub fn keys_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupIncludeElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<ListField<PrimField<String>>>,
}

impl AccessGroupIncludeElGithubEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `teams`.\n"]
    pub fn set_teams(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.teams = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupIncludeElGithubEl {
    type O = BlockAssignable<AccessGroupIncludeElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupIncludeElGithubEl {}

impl BuildAccessGroupIncludeElGithubEl {
    pub fn build(self) -> AccessGroupIncludeElGithubEl {
        AccessGroupIncludeElGithubEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
            teams: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupIncludeElGithubElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupIncludeElGithubElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupIncludeElGithubElRef {
        AccessGroupIncludeElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupIncludeElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupIncludeElGsuiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessGroupIncludeElGsuiteEl {
    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupIncludeElGsuiteEl {
    type O = BlockAssignable<AccessGroupIncludeElGsuiteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupIncludeElGsuiteEl {}

impl BuildAccessGroupIncludeElGsuiteEl {
    pub fn build(self) -> AccessGroupIncludeElGsuiteEl {
        AccessGroupIncludeElGsuiteEl {
            email: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupIncludeElGsuiteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupIncludeElGsuiteElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupIncludeElGsuiteElRef {
        AccessGroupIncludeElGsuiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupIncludeElGsuiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupIncludeElOktaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<ListField<PrimField<String>>>,
}

impl AccessGroupIncludeElOktaEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupIncludeElOktaEl {
    type O = BlockAssignable<AccessGroupIncludeElOktaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupIncludeElOktaEl {}

impl BuildAccessGroupIncludeElOktaEl {
    pub fn build(self) -> AccessGroupIncludeElOktaEl {
        AccessGroupIncludeElOktaEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupIncludeElOktaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupIncludeElOktaElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupIncludeElOktaElRef {
        AccessGroupIncludeElOktaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupIncludeElOktaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupIncludeElSamlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessGroupIncludeElSamlEl {
    #[doc= "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_name = Some(v.into());
        self
    }

    #[doc= "Set the field `attribute_value`.\n"]
    pub fn set_attribute_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_value = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupIncludeElSamlEl {
    type O = BlockAssignable<AccessGroupIncludeElSamlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupIncludeElSamlEl {}

impl BuildAccessGroupIncludeElSamlEl {
    pub fn build(self) -> AccessGroupIncludeElSamlEl {
        AccessGroupIncludeElSamlEl {
            attribute_name: core::default::Default::default(),
            attribute_value: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupIncludeElSamlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupIncludeElSamlElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupIncludeElSamlElRef {
        AccessGroupIncludeElSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupIncludeElSamlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc= "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessGroupIncludeElDynamic {
    azure: Option<DynamicBlock<AccessGroupIncludeElAzureEl>>,
    external_evaluation: Option<DynamicBlock<AccessGroupIncludeElExternalEvaluationEl>>,
    github: Option<DynamicBlock<AccessGroupIncludeElGithubEl>>,
    gsuite: Option<DynamicBlock<AccessGroupIncludeElGsuiteEl>>,
    okta: Option<DynamicBlock<AccessGroupIncludeElOktaEl>>,
    saml: Option<DynamicBlock<AccessGroupIncludeElSamlEl>>,
}

#[derive(Serialize)]
pub struct AccessGroupIncludeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    any_valid_service_token: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_posture: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_domain: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    everyone: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_method: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_token: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure: Option<Vec<AccessGroupIncludeElAzureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_evaluation: Option<Vec<AccessGroupIncludeElExternalEvaluationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AccessGroupIncludeElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gsuite: Option<Vec<AccessGroupIncludeElGsuiteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    okta: Option<Vec<AccessGroupIncludeElOktaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml: Option<Vec<AccessGroupIncludeElSamlEl>>,
    dynamic: AccessGroupIncludeElDynamic,
}

impl AccessGroupIncludeEl {
    #[doc= "Set the field `any_valid_service_token`.\n"]
    pub fn set_any_valid_service_token(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.any_valid_service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_method = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }

    #[doc= "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.device_posture = Some(v.into());
        self
    }

    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `everyone`.\n"]
    pub fn set_everyone(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.everyone = Some(v.into());
        self
    }

    #[doc= "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.geo = Some(v.into());
        self
    }

    #[doc= "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.group = Some(v.into());
        self
    }

    #[doc= "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_list = Some(v.into());
        self
    }

    #[doc= "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.login_method = Some(v.into());
        self
    }

    #[doc= "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `azure`.\n"]
    pub fn set_azure(mut self, v: impl Into<BlockAssignable<AccessGroupIncludeElAzureEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl Into<BlockAssignable<AccessGroupIncludeElExternalEvaluationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_evaluation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_evaluation = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AccessGroupIncludeElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl Into<BlockAssignable<AccessGroupIncludeElGsuiteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gsuite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gsuite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl Into<BlockAssignable<AccessGroupIncludeElOktaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.okta = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.okta = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl Into<BlockAssignable<AccessGroupIncludeElSamlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.saml = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.saml = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessGroupIncludeEl {
    type O = BlockAssignable<AccessGroupIncludeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupIncludeEl {}

impl BuildAccessGroupIncludeEl {
    pub fn build(self) -> AccessGroupIncludeEl {
        AccessGroupIncludeEl {
            any_valid_service_token: core::default::Default::default(),
            auth_method: core::default::Default::default(),
            certificate: core::default::Default::default(),
            common_name: core::default::Default::default(),
            device_posture: core::default::Default::default(),
            email: core::default::Default::default(),
            email_domain: core::default::Default::default(),
            everyone: core::default::Default::default(),
            geo: core::default::Default::default(),
            group: core::default::Default::default(),
            ip: core::default::Default::default(),
            ip_list: core::default::Default::default(),
            login_method: core::default::Default::default(),
            service_token: core::default::Default::default(),
            azure: core::default::Default::default(),
            external_evaluation: core::default::Default::default(),
            github: core::default::Default::default(),
            gsuite: core::default::Default::default(),
            okta: core::default::Default::default(),
            saml: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AccessGroupIncludeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupIncludeElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupIncludeElRef {
        AccessGroupIncludeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupIncludeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `any_valid_service_token` after provisioning.\n"]
    pub fn any_valid_service_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_valid_service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc= "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.device_posture", self.base))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email_domain", self.base))
    }

    #[doc= "Get a reference to the value of field `everyone` after provisioning.\n"]
    pub fn everyone(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc= "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc= "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.login_method", self.base))
    }

    #[doc= "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> ListRef<AccessGroupIncludeElAzureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.azure", self.base))
    }

    #[doc= "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ListRef<AccessGroupIncludeElExternalEvaluationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_evaluation", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AccessGroupIncludeElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ListRef<AccessGroupIncludeElGsuiteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc= "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ListRef<AccessGroupIncludeElOktaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc= "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ListRef<AccessGroupIncludeElSamlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupRequireElAzureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessGroupRequireElAzureEl {
    #[doc= "Set the field `id`.\nThe ID of the Azure group or user."]
    pub fn set_id(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\nThe ID of the Azure Identity provider."]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupRequireElAzureEl {
    type O = BlockAssignable<AccessGroupRequireElAzureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupRequireElAzureEl {}

impl BuildAccessGroupRequireElAzureEl {
    pub fn build(self) -> AccessGroupRequireElAzureEl {
        AccessGroupRequireElAzureEl {
            id: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupRequireElAzureElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupRequireElAzureElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupRequireElAzureElRef {
        AccessGroupRequireElAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupRequireElAzureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the Azure group or user."]
    pub fn id(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\nThe ID of the Azure Identity provider."]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupRequireElExternalEvaluationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keys_url: Option<PrimField<String>>,
}

impl AccessGroupRequireElExternalEvaluationEl {
    #[doc= "Set the field `evaluate_url`.\n"]
    pub fn set_evaluate_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluate_url = Some(v.into());
        self
    }

    #[doc= "Set the field `keys_url`.\n"]
    pub fn set_keys_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keys_url = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupRequireElExternalEvaluationEl {
    type O = BlockAssignable<AccessGroupRequireElExternalEvaluationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupRequireElExternalEvaluationEl {}

impl BuildAccessGroupRequireElExternalEvaluationEl {
    pub fn build(self) -> AccessGroupRequireElExternalEvaluationEl {
        AccessGroupRequireElExternalEvaluationEl {
            evaluate_url: core::default::Default::default(),
            keys_url: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupRequireElExternalEvaluationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupRequireElExternalEvaluationElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupRequireElExternalEvaluationElRef {
        AccessGroupRequireElExternalEvaluationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupRequireElExternalEvaluationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `evaluate_url` after provisioning.\n"]
    pub fn evaluate_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.evaluate_url", self.base))
    }

    #[doc= "Get a reference to the value of field `keys_url` after provisioning.\n"]
    pub fn keys_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keys_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupRequireElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<ListField<PrimField<String>>>,
}

impl AccessGroupRequireElGithubEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `teams`.\n"]
    pub fn set_teams(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.teams = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupRequireElGithubEl {
    type O = BlockAssignable<AccessGroupRequireElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupRequireElGithubEl {}

impl BuildAccessGroupRequireElGithubEl {
    pub fn build(self) -> AccessGroupRequireElGithubEl {
        AccessGroupRequireElGithubEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
            teams: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupRequireElGithubElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupRequireElGithubElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupRequireElGithubElRef {
        AccessGroupRequireElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupRequireElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupRequireElGsuiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessGroupRequireElGsuiteEl {
    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupRequireElGsuiteEl {
    type O = BlockAssignable<AccessGroupRequireElGsuiteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupRequireElGsuiteEl {}

impl BuildAccessGroupRequireElGsuiteEl {
    pub fn build(self) -> AccessGroupRequireElGsuiteEl {
        AccessGroupRequireElGsuiteEl {
            email: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupRequireElGsuiteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupRequireElGsuiteElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupRequireElGsuiteElRef {
        AccessGroupRequireElGsuiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupRequireElGsuiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupRequireElOktaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<ListField<PrimField<String>>>,
}

impl AccessGroupRequireElOktaEl {
    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupRequireElOktaEl {
    type O = BlockAssignable<AccessGroupRequireElOktaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupRequireElOktaEl {}

impl BuildAccessGroupRequireElOktaEl {
    pub fn build(self) -> AccessGroupRequireElOktaEl {
        AccessGroupRequireElOktaEl {
            identity_provider_id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupRequireElOktaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupRequireElOktaElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupRequireElOktaElRef {
        AccessGroupRequireElOktaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupRequireElOktaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessGroupRequireElSamlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_id: Option<PrimField<String>>,
}

impl AccessGroupRequireElSamlEl {
    #[doc= "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_name = Some(v.into());
        self
    }

    #[doc= "Set the field `attribute_value`.\n"]
    pub fn set_attribute_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_value = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_id`.\n"]
    pub fn set_identity_provider_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider_id = Some(v.into());
        self
    }
}

impl ToListMappable for AccessGroupRequireElSamlEl {
    type O = BlockAssignable<AccessGroupRequireElSamlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupRequireElSamlEl {}

impl BuildAccessGroupRequireElSamlEl {
    pub fn build(self) -> AccessGroupRequireElSamlEl {
        AccessGroupRequireElSamlEl {
            attribute_name: core::default::Default::default(),
            attribute_value: core::default::Default::default(),
            identity_provider_id: core::default::Default::default(),
        }
    }
}

pub struct AccessGroupRequireElSamlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupRequireElSamlElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupRequireElSamlElRef {
        AccessGroupRequireElSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupRequireElSamlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc= "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_provider_id` after provisioning.\n"]
    pub fn identity_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessGroupRequireElDynamic {
    azure: Option<DynamicBlock<AccessGroupRequireElAzureEl>>,
    external_evaluation: Option<DynamicBlock<AccessGroupRequireElExternalEvaluationEl>>,
    github: Option<DynamicBlock<AccessGroupRequireElGithubEl>>,
    gsuite: Option<DynamicBlock<AccessGroupRequireElGsuiteEl>>,
    okta: Option<DynamicBlock<AccessGroupRequireElOktaEl>>,
    saml: Option<DynamicBlock<AccessGroupRequireElSamlEl>>,
}

#[derive(Serialize)]
pub struct AccessGroupRequireEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    any_valid_service_token: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_posture: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_domain: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    everyone: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_method: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_token: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure: Option<Vec<AccessGroupRequireElAzureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_evaluation: Option<Vec<AccessGroupRequireElExternalEvaluationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AccessGroupRequireElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gsuite: Option<Vec<AccessGroupRequireElGsuiteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    okta: Option<Vec<AccessGroupRequireElOktaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml: Option<Vec<AccessGroupRequireElSamlEl>>,
    dynamic: AccessGroupRequireElDynamic,
}

impl AccessGroupRequireEl {
    #[doc= "Set the field `any_valid_service_token`.\n"]
    pub fn set_any_valid_service_token(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.any_valid_service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_method`.\n"]
    pub fn set_auth_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth_method = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.common_name = Some(v.into());
        self
    }

    #[doc= "Set the field `device_posture`.\n"]
    pub fn set_device_posture(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.device_posture = Some(v.into());
        self
    }

    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `email_domain`.\n"]
    pub fn set_email_domain(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `everyone`.\n"]
    pub fn set_everyone(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.everyone = Some(v.into());
        self
    }

    #[doc= "Set the field `geo`.\n"]
    pub fn set_geo(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.geo = Some(v.into());
        self
    }

    #[doc= "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.group = Some(v.into());
        self
    }

    #[doc= "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_list`.\n"]
    pub fn set_ip_list(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_list = Some(v.into());
        self
    }

    #[doc= "Set the field `login_method`.\n"]
    pub fn set_login_method(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.login_method = Some(v.into());
        self
    }

    #[doc= "Set the field `service_token`.\n"]
    pub fn set_service_token(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `azure`.\n"]
    pub fn set_azure(mut self, v: impl Into<BlockAssignable<AccessGroupRequireElAzureEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.azure = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.azure = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `external_evaluation`.\n"]
    pub fn set_external_evaluation(
        mut self,
        v: impl Into<BlockAssignable<AccessGroupRequireElExternalEvaluationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_evaluation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_evaluation = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AccessGroupRequireElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gsuite`.\n"]
    pub fn set_gsuite(mut self, v: impl Into<BlockAssignable<AccessGroupRequireElGsuiteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gsuite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gsuite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `okta`.\n"]
    pub fn set_okta(mut self, v: impl Into<BlockAssignable<AccessGroupRequireElOktaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.okta = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.okta = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `saml`.\n"]
    pub fn set_saml(mut self, v: impl Into<BlockAssignable<AccessGroupRequireElSamlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.saml = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.saml = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessGroupRequireEl {
    type O = BlockAssignable<AccessGroupRequireEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessGroupRequireEl {}

impl BuildAccessGroupRequireEl {
    pub fn build(self) -> AccessGroupRequireEl {
        AccessGroupRequireEl {
            any_valid_service_token: core::default::Default::default(),
            auth_method: core::default::Default::default(),
            certificate: core::default::Default::default(),
            common_name: core::default::Default::default(),
            device_posture: core::default::Default::default(),
            email: core::default::Default::default(),
            email_domain: core::default::Default::default(),
            everyone: core::default::Default::default(),
            geo: core::default::Default::default(),
            group: core::default::Default::default(),
            ip: core::default::Default::default(),
            ip_list: core::default::Default::default(),
            login_method: core::default::Default::default(),
            service_token: core::default::Default::default(),
            azure: core::default::Default::default(),
            external_evaluation: core::default::Default::default(),
            github: core::default::Default::default(),
            gsuite: core::default::Default::default(),
            okta: core::default::Default::default(),
            saml: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AccessGroupRequireElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessGroupRequireElRef {
    fn new(shared: StackShared, base: String) -> AccessGroupRequireElRef {
        AccessGroupRequireElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessGroupRequireElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `any_valid_service_token` after provisioning.\n"]
    pub fn any_valid_service_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_valid_service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_method` after provisioning.\n"]
    pub fn auth_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_method", self.base))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc= "Get a reference to the value of field `device_posture` after provisioning.\n"]
    pub fn device_posture(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.device_posture", self.base))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `email_domain` after provisioning.\n"]
    pub fn email_domain(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email_domain", self.base))
    }

    #[doc= "Get a reference to the value of field `everyone` after provisioning.\n"]
    pub fn everyone(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.everyone", self.base))
    }

    #[doc= "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_list` after provisioning.\n"]
    pub fn ip_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_list", self.base))
    }

    #[doc= "Get a reference to the value of field `login_method` after provisioning.\n"]
    pub fn login_method(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.login_method", self.base))
    }

    #[doc= "Get a reference to the value of field `service_token` after provisioning.\n"]
    pub fn service_token(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.service_token", self.base))
    }

    #[doc= "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> ListRef<AccessGroupRequireElAzureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.azure", self.base))
    }

    #[doc= "Get a reference to the value of field `external_evaluation` after provisioning.\n"]
    pub fn external_evaluation(&self) -> ListRef<AccessGroupRequireElExternalEvaluationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_evaluation", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AccessGroupRequireElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gsuite` after provisioning.\n"]
    pub fn gsuite(&self) -> ListRef<AccessGroupRequireElGsuiteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gsuite", self.base))
    }

    #[doc= "Get a reference to the value of field `okta` after provisioning.\n"]
    pub fn okta(&self) -> ListRef<AccessGroupRequireElOktaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.okta", self.base))
    }

    #[doc= "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ListRef<AccessGroupRequireElSamlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.saml", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessGroupDynamic {
    exclude: Option<DynamicBlock<AccessGroupExcludeEl>>,
    include: Option<DynamicBlock<AccessGroupIncludeEl>>,
    require: Option<DynamicBlock<AccessGroupRequireEl>>,
}
