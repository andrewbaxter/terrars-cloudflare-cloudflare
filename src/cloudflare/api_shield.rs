use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ApiShieldData {
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
    zone_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_id_characteristics: Option<Vec<ApiShieldAuthIdCharacteristicsEl>>,
    dynamic: ApiShieldDynamic,
}

struct ApiShield_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiShieldData>,
}

#[derive(Clone)]
pub struct ApiShield(Rc<ApiShield_>);

impl ApiShield {
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

    #[doc= "Set the field `auth_id_characteristics`.\n"]
    pub fn set_auth_id_characteristics(self, v: impl Into<BlockAssignable<ApiShieldAuthIdCharacteristicsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth_id_characteristics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth_id_characteristics = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auth_id_characteristics` after provisioning.\n"]
    pub fn auth_id_characteristics(&self) -> ListRef<ApiShieldAuthIdCharacteristicsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auth_id_characteristics", self.extract_ref()))
    }
}

impl Referable for ApiShield {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApiShield { }

impl ToListMappable for ApiShield {
    type O = ListRef<ApiShieldRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApiShield_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_api_shield".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiShield {
    pub tf_id: String,
    #[doc= "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: PrimField<String>,
}

impl BuildApiShield {
    pub fn build(self, stack: &mut Stack) -> ApiShield {
        let out = ApiShield(Rc::new(ApiShield_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiShieldData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                zone_id: self.zone_id,
                auth_id_characteristics: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiShieldRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiShieldRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiShieldRef {
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

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auth_id_characteristics` after provisioning.\n"]
    pub fn auth_id_characteristics(&self) -> ListRef<ApiShieldAuthIdCharacteristicsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auth_id_characteristics", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiShieldAuthIdCharacteristicsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl ApiShieldAuthIdCharacteristicsEl {
    #[doc= "Set the field `name`.\nThe name of the characteristic."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type of characteristic. Available values: `header`, `cookie`."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for ApiShieldAuthIdCharacteristicsEl {
    type O = BlockAssignable<ApiShieldAuthIdCharacteristicsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldAuthIdCharacteristicsEl {}

impl BuildApiShieldAuthIdCharacteristicsEl {
    pub fn build(self) -> ApiShieldAuthIdCharacteristicsEl {
        ApiShieldAuthIdCharacteristicsEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct ApiShieldAuthIdCharacteristicsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiShieldAuthIdCharacteristicsElRef {
    fn new(shared: StackShared, base: String) -> ApiShieldAuthIdCharacteristicsElRef {
        ApiShieldAuthIdCharacteristicsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldAuthIdCharacteristicsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the characteristic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of characteristic. Available values: `header`, `cookie`."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiShieldDynamic {
    auth_id_characteristics: Option<DynamicBlock<ApiShieldAuthIdCharacteristicsEl>>,
}
