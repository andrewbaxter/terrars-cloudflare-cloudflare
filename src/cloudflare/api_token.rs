use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ApiTokenData {
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
    expires_on: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_before: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<ApiTokenConditionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<std::vec::Vec<ApiTokenPolicyEl>>,
    dynamic: ApiTokenDynamic,
}

struct ApiToken_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApiTokenData>,
}

#[derive(Clone)]
pub struct ApiToken(std::rc::Rc<ApiToken_>);

impl ApiToken {
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

    #[doc =
        "Set the field `expires_on`.\nThe expiration time on or after which the token MUST NOT be accepted for processing."]
    pub fn set_expires_on(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expires_on = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_before`.\nThe time before which the token MUST NOT be accepted for processing."]
    pub fn set_not_before(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().not_before = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(self, v: impl std::convert::Into<terrars::BlockAssignable<ApiTokenConditionEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(self, v: impl std::convert::Into<terrars::BlockAssignable<ApiTokenPolicyEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nThe expiration time on or after which the token MUST NOT be accepted for processing."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issued_on` after provisioning.\nTimestamp of when the token was issued."]
    pub fn issued_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issued_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nTimestamp of when the token was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the API Token."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `not_before` after provisioning.\nThe time before which the token MUST NOT be accepted for processing."]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the API Token."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<ApiTokenConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }
}

impl terrars::Referable for ApiToken {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApiToken { }

impl terrars::ToListMappable for ApiToken {
    type O = terrars::ListRef<ApiTokenRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApiToken_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_api_token".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiToken {
    pub tf_id: std::string::String,
    #[doc = "Name of the API Token."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApiToken {
    pub fn build(self, stack: &mut terrars::Stack) -> ApiToken {
        let out = ApiToken(std::rc::Rc::new(ApiToken_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApiTokenData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                expires_on: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                not_before: core::default::Default::default(),
                condition: core::default::Default::default(),
                policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiTokenRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiTokenRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expires_on` after provisioning.\nThe expiration time on or after which the token MUST NOT be accepted for processing."]
    pub fn expires_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issued_on` after provisioning.\nTimestamp of when the token was issued."]
    pub fn issued_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issued_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nTimestamp of when the token was last modified."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the API Token."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `not_before` after provisioning.\nThe time before which the token MUST NOT be accepted for processing."]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the API Token."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<ApiTokenConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiTokenConditionElRequestIpEl {
    #[serde(rename = "in", skip_serializing_if = "std::option::Option::is_none")]
    in_: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_in: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ApiTokenConditionElRequestIpEl {
    #[doc =
        "Set the field `in_`.\nList of IP addresses or CIDR notation where the token may be used from. If not specified, the token will be valid for all IP addresses."]
    pub fn set_in(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.in_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_in`.\nList of IP addresses or CIDR notation where the token should not be used from."]
    pub fn set_not_in(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_in = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApiTokenConditionElRequestIpEl {
    type O = terrars::BlockAssignable<ApiTokenConditionElRequestIpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiTokenConditionElRequestIpElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiTokenConditionElRequestIpElRef {
        ApiTokenConditionElRequestIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiTokenConditionElRequestIpElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `in_` after provisioning.\nList of IP addresses or CIDR notation where the token may be used from. If not specified, the token will be valid for all IP addresses."]
    pub fn in_(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.in", self.base))
    }

    #[doc =
        "Get a reference to the value of field `not_in` after provisioning.\nList of IP addresses or CIDR notation where the token should not be used from."]
    pub fn not_in(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.not_in", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiTokenConditionElDynamic {
    request_ip: std::option::Option<terrars::DynamicBlock<ApiTokenConditionElRequestIpEl>>,
}

#[derive(Serialize)]
pub struct ApiTokenConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_ip: std::option::Option<std::vec::Vec<ApiTokenConditionElRequestIpEl>>,
    dynamic: ApiTokenConditionElDynamic,
}

impl ApiTokenConditionEl {
    #[doc = "Set the field `request_ip`.\n"]
    pub fn set_request_ip(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApiTokenConditionElRequestIpEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_ip = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_ip = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApiTokenConditionEl {
    type O = terrars::BlockAssignable<ApiTokenConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApiTokenConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiTokenConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiTokenConditionElRef {
        ApiTokenConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiTokenConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_ip` after provisioning.\n"]
    pub fn request_ip(&self) -> terrars::ListRef<ApiTokenConditionElRequestIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiTokenPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    permission_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    resources: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl ApiTokenPolicyEl {
    #[doc = "Set the field `effect`.\nEffect of the policy. Available values: `allow`, `deny`. Defaults to `allow`."]
    pub fn set_effect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effect = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApiTokenPolicyEl {
    type O = terrars::BlockAssignable<ApiTokenPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiTokenPolicyEl {
    #[doc =
        "List of permissions groups IDs. See [documentation](https://developers.cloudflare.com/api/tokens/create/permissions) for more information."]
    pub permission_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = "Describes what operations against which resources are allowed or denied."]
    pub resources: terrars::RecField<terrars::PrimField<std::string::String>>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiTokenPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiTokenPolicyElRef {
        ApiTokenPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiTokenPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effect` after provisioning.\nEffect of the policy. Available values: `allow`, `deny`. Defaults to `allow`."]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission_groups` after provisioning.\nList of permissions groups IDs. See [documentation](https://developers.cloudflare.com/api/tokens/create/permissions) for more information."]
    pub fn permission_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permission_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nDescribes what operations against which resources are allowed or denied."]
    pub fn resources(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiTokenDynamic {
    condition: std::option::Option<terrars::DynamicBlock<ApiTokenConditionEl>>,
    policy: std::option::Option<terrars::DynamicBlock<ApiTokenPolicyEl>>,
}
