use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataAccountRolesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataAccountRoles_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAccountRolesData>,
}

#[derive(Clone)]
pub struct DataAccountRoles(Rc<DataAccountRoles_>);

impl DataAccountRoles {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `roles` after provisioning.\nA list of roles object."]
    pub fn roles(&self) -> ListRef<DataAccountRolesRolesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.roles", self.extract_ref()))
    }
}

impl Referable for DataAccountRoles {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAccountRoles { }

impl ToListMappable for DataAccountRoles {
    type O = ListRef<DataAccountRolesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAccountRoles_ {
    fn extract_datasource_type(&self) -> String {
        "cloudflare_account_roles".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountRoles {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
}

impl BuildDataAccountRoles {
    pub fn build(self, stack: &mut Stack) -> DataAccountRoles {
        let out = DataAccountRoles(Rc::new(DataAccountRoles_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAccountRolesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                account_id: self.account_id,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountRolesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAccountRolesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountRolesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `roles` after provisioning.\nA list of roles object."]
    pub fn roles(&self) -> ListRef<DataAccountRolesRolesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.roles", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountRolesRolesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataAccountRolesRolesEl {
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

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAccountRolesRolesEl {
    type O = BlockAssignable<DataAccountRolesRolesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRolesRolesEl {}

impl BuildDataAccountRolesRolesEl {
    pub fn build(self) -> DataAccountRolesRolesEl {
        DataAccountRolesRolesEl {
            description: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataAccountRolesRolesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAccountRolesRolesElRef {
    fn new(shared: StackShared, base: String) -> DataAccountRolesRolesElRef {
        DataAccountRolesRolesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRolesRolesElRef {
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
