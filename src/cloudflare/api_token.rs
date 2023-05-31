use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ApiTokenData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_on: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_before: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<ApiTokenConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<Vec<ApiTokenPolicyEl>>,
    dynamic: ApiTokenDynamic,
}

struct ApiToken_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiTokenData>,
}

#[derive(Clone)]
pub struct ApiToken(Rc<ApiToken_>);

impl ApiToken {
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

    #[doc= "Set the field `expires_on`.\nThe expiration time on or after which the token MUST NOT be accepted for processing."]
    pub fn set_expires_on(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expires_on = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `not_before`.\nThe time before which the token MUST NOT be accepted for processing."]
    pub fn set_not_before(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().not_before = Some(v.into());
        self
    }

    #[doc= "Set the field `condition`.\n"]
    pub fn set_condition(self, v: impl Into<BlockAssignable<ApiTokenConditionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `policy`.\n"]
    pub fn set_policy(self, v: impl Into<BlockAssignable<ApiTokenPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `expires_on` after provisioning.\nThe expiration time on or after which the token MUST NOT be accepted for processing."]
    pub fn expires_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issued_on` after provisioning.\nTimestamp of when the token was issued."]
    pub fn issued_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issued_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nTimestamp of when the token was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the API Token."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_before` after provisioning.\nThe time before which the token MUST NOT be accepted for processing."]
    pub fn not_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the API Token."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<ApiTokenConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }
}

impl Referable for ApiToken {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApiToken { }

impl ToListMappable for ApiToken {
    type O = ListRef<ApiTokenRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApiToken_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_api_token".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiToken {
    pub tf_id: String,
    #[doc= "Name of the API Token."]
    pub name: PrimField<String>,
}

impl BuildApiToken {
    pub fn build(self, stack: &mut Stack) -> ApiToken {
        let out = ApiToken(Rc::new(ApiToken_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiTokenData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                expires_on: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                not_before: core::default::Default::default(),
                condition: core::default::Default::default(),
                policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiTokenRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiTokenRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiTokenRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expires_on` after provisioning.\nThe expiration time on or after which the token MUST NOT be accepted for processing."]
    pub fn expires_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issued_on` after provisioning.\nTimestamp of when the token was issued."]
    pub fn issued_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issued_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `modified_on` after provisioning.\nTimestamp of when the token was last modified."]
    pub fn modified_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the API Token."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_before` after provisioning.\nThe time before which the token MUST NOT be accepted for processing."]
    pub fn not_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the API Token."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<ApiTokenConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiTokenConditionElRequestIpEl {
    #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
    in_: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_in: Option<SetField<PrimField<String>>>,
}

impl ApiTokenConditionElRequestIpEl {
    #[doc= "Set the field `in_`.\nList of IP addresses or CIDR notation where the token may be used from. If not specified, the token will be valid for all IP addresses."]
    pub fn set_in(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.in_ = Some(v.into());
        self
    }

    #[doc= "Set the field `not_in`.\nList of IP addresses or CIDR notation where the token should not be used from."]
    pub fn set_not_in(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.not_in = Some(v.into());
        self
    }
}

impl ToListMappable for ApiTokenConditionElRequestIpEl {
    type O = BlockAssignable<ApiTokenConditionElRequestIpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiTokenConditionElRequestIpEl {}

impl BuildApiTokenConditionElRequestIpEl {
    pub fn build(self) -> ApiTokenConditionElRequestIpEl {
        ApiTokenConditionElRequestIpEl {
            in_: core::default::Default::default(),
            not_in: core::default::Default::default(),
        }
    }
}

pub struct ApiTokenConditionElRequestIpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiTokenConditionElRequestIpElRef {
    fn new(shared: StackShared, base: String) -> ApiTokenConditionElRequestIpElRef {
        ApiTokenConditionElRequestIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiTokenConditionElRequestIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `in_` after provisioning.\nList of IP addresses or CIDR notation where the token may be used from. If not specified, the token will be valid for all IP addresses."]
    pub fn in_(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.in", self.base))
    }

    #[doc= "Get a reference to the value of field `not_in` after provisioning.\nList of IP addresses or CIDR notation where the token should not be used from."]
    pub fn not_in(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.not_in", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiTokenConditionElDynamic {
    request_ip: Option<DynamicBlock<ApiTokenConditionElRequestIpEl>>,
}

#[derive(Serialize)]
pub struct ApiTokenConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_ip: Option<Vec<ApiTokenConditionElRequestIpEl>>,
    dynamic: ApiTokenConditionElDynamic,
}

impl ApiTokenConditionEl {
    #[doc= "Set the field `request_ip`.\n"]
    pub fn set_request_ip(mut self, v: impl Into<BlockAssignable<ApiTokenConditionElRequestIpEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_ip = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_ip = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ApiTokenConditionEl {
    type O = BlockAssignable<ApiTokenConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiTokenConditionEl {}

impl BuildApiTokenConditionEl {
    pub fn build(self) -> ApiTokenConditionEl {
        ApiTokenConditionEl {
            request_ip: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ApiTokenConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiTokenConditionElRef {
    fn new(shared: StackShared, base: String) -> ApiTokenConditionElRef {
        ApiTokenConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiTokenConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `request_ip` after provisioning.\n"]
    pub fn request_ip(&self) -> ListRef<ApiTokenConditionElRequestIpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiTokenPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    permission_groups: SetField<PrimField<String>>,
    resources: RecField<PrimField<String>>,
}

impl ApiTokenPolicyEl {
    #[doc= "Set the field `effect`.\nEffect of the policy. Available values: `allow`, `deny`. Defaults to `allow`."]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }
}

impl ToListMappable for ApiTokenPolicyEl {
    type O = BlockAssignable<ApiTokenPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiTokenPolicyEl {
    #[doc= "List of permissions groups IDs. See [documentation](https://developers.cloudflare.com/api/tokens/create/permissions) for more information."]
    pub permission_groups: SetField<PrimField<String>>,
    #[doc= "Describes what operations against which resources are allowed or denied."]
    pub resources: RecField<PrimField<String>>,
}

impl BuildApiTokenPolicyEl {
    pub fn build(self) -> ApiTokenPolicyEl {
        ApiTokenPolicyEl {
            effect: core::default::Default::default(),
            permission_groups: self.permission_groups,
            resources: self.resources,
        }
    }
}

pub struct ApiTokenPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiTokenPolicyElRef {
    fn new(shared: StackShared, base: String) -> ApiTokenPolicyElRef {
        ApiTokenPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiTokenPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `effect` after provisioning.\nEffect of the policy. Available values: `allow`, `deny`. Defaults to `allow`."]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc= "Get a reference to the value of field `permission_groups` after provisioning.\nList of permissions groups IDs. See [documentation](https://developers.cloudflare.com/api/tokens/create/permissions) for more information."]
    pub fn permission_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.permission_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `resources` after provisioning.\nDescribes what operations against which resources are allowed or denied."]
    pub fn resources(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiTokenDynamic {
    condition: Option<DynamicBlock<ApiTokenConditionEl>>,
    policy: Option<DynamicBlock<ApiTokenPolicyEl>>,
}
