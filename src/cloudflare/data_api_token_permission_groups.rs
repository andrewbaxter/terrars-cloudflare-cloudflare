use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataApiTokenPermissionGroupsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataApiTokenPermissionGroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataApiTokenPermissionGroupsData>,
}

#[derive(Clone)]
pub struct DataApiTokenPermissionGroups(Rc<DataApiTokenPermissionGroups_>);

impl DataApiTokenPermissionGroups {
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

    #[doc= "Get a reference to the value of field `account` after provisioning.\nMap of permissions for account level resources."]
    pub fn account(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.account", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `permissions` after provisioning.\nMap of all permissions available. Should not be used as some permissions will overlap resource scope. Instead, use resource level specific attributes."]
    pub fn permissions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\nMap of permissions for user level resources."]
    pub fn user(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nMap of permissions for zone level resources."]
    pub fn zone(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

impl Referable for DataApiTokenPermissionGroups {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataApiTokenPermissionGroups { }

impl ToListMappable for DataApiTokenPermissionGroups {
    type O = ListRef<DataApiTokenPermissionGroupsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataApiTokenPermissionGroups_ {
    fn extract_datasource_type(&self) -> String {
        "cloudflare_api_token_permission_groups".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApiTokenPermissionGroups {
    pub tf_id: String,
}

impl BuildDataApiTokenPermissionGroups {
    pub fn build(self, stack: &mut Stack) -> DataApiTokenPermissionGroups {
        let out = DataApiTokenPermissionGroups(Rc::new(DataApiTokenPermissionGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataApiTokenPermissionGroupsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataApiTokenPermissionGroupsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataApiTokenPermissionGroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataApiTokenPermissionGroupsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `account` after provisioning.\nMap of permissions for account level resources."]
    pub fn account(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.account", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `permissions` after provisioning.\nMap of all permissions available. Should not be used as some permissions will overlap resource scope. Instead, use resource level specific attributes."]
    pub fn permissions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\nMap of permissions for user level resources."]
    pub fn user(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nMap of permissions for zone level resources."]
    pub fn zone(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}
