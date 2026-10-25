#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataOrganizationsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containing: std::option::Option<DataOrganizations_Containing>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<DataOrganizations_Name>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<DataOrganizations_Parent>,
}

struct DataOrganizations_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOrganizationsData_>,
}

#[derive(Clone)]
pub struct DataOrganizations(std::rc::Rc<DataOrganizations_>);

impl DataOrganizations {
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

    #[doc = "Set the field `containing`.\n"]
    pub fn set_containing(self, v: impl std::convert::Into<DataOrganizations_Containing>) -> Self {
        self.0.data.borrow_mut().containing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `id`.\nOnly return organizations with the specified IDs (ex. id=foo&id=bar). Send multiple elements\nby repeating the query value."]
    pub fn set_id(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<DataOrganizations_Name>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `page_size`.\nThe amount of items to return. Defaults to 10."]
    pub fn set_page_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().page_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `page_token`.\nAn opaque token returned from the last list response that when\nprovided will retrieve the next page.\n\nParameters used to filter the retrieved list must remain in subsequent\nrequests with a page token."]
    pub fn set_page_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().page_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parent`.\n"]
    pub fn set_parent(self, v: impl std::convert::Into<DataOrganizations_Parent>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `containing` after provisioning.\n"]
    pub fn containing(&self) -> DataOrganizations_ContainingRef {
        DataOrganizations_ContainingRef::new(self.shared().clone(), format!("{}.containing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nOnly return organizations with the specified IDs (ex. id=foo&id=bar). Send multiple elements\nby repeating the query value."]
    pub fn id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> DataOrganizations_NameRef {
        DataOrganizations_NameRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page_size` after provisioning.\nThe amount of items to return. Defaults to 10."]
    pub fn page_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page_token` after provisioning.\nAn opaque token returned from the last list response that when\nprovided will retrieve the next page.\n\nParameters used to filter the retrieved list must remain in subsequent\nrequests with a page token."]
    pub fn page_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> DataOrganizations_ParentRef {
        DataOrganizations_ParentRef::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataOrganizations_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataOrganizations {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOrganizations { }

impl terrars::ToListMappable for DataOrganizations {
    type O = terrars::ListRef<DataOrganizationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOrganizations_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_organizations".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizations {
    pub tf_id: std::string::String,
}

impl BuildDataOrganizations {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOrganizations {
        let out = DataOrganizations(std::rc::Rc::new(DataOrganizations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOrganizationsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                containing: core::default::Default::default(),
                id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                name: core::default::Default::default(),
                page_size: core::default::Default::default(),
                page_token: core::default::Default::default(),
                parent: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `containing` after provisioning.\n"]
    pub fn containing(&self) -> DataOrganizations_ContainingRef {
        DataOrganizations_ContainingRef::new(self.shared().clone(), format!("{}.containing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nOnly return organizations with the specified IDs (ex. id=foo&id=bar). Send multiple elements\nby repeating the query value."]
    pub fn id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> DataOrganizations_NameRef {
        DataOrganizations_NameRef::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page_size` after provisioning.\nThe amount of items to return. Defaults to 10."]
    pub fn page_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page_token` after provisioning.\nAn opaque token returned from the last list response that when\nprovided will retrieve the next page.\n\nParameters used to filter the retrieved list must remain in subsequent\nrequests with a page token."]
    pub fn page_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> DataOrganizations_ParentRef {
        DataOrganizations_ParentRef::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataOrganizations_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizations_Containing {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizations_Containing {
    #[doc =
        "Set the field `account`.\nFilter the list of organizations to the ones that contain this particular\naccount."]
    pub fn set_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `organization`.\nFilter the list of organizations to the ones that contain this particular\norganization."]
    pub fn set_organization(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.organization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user`.\nFilter the list of organizations to the ones that contain this particular\nuser.\n\nIMPORTANT: Just because an organization \"contains\" a user is not a\nrepresentation of any authorization or privilege to manage any resources\ntherein. An organization \"containing\" a user simply means the user is managed by\nthat organization."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizations_Containing {
    type O = terrars::BlockAssignable<DataOrganizations_Containing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizations_Containing {}

impl BuildDataOrganizations_Containing {
    pub fn build(self) -> DataOrganizations_Containing {
        DataOrganizations_Containing {
            account: core::default::Default::default(),
            organization: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOrganizations_ContainingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizations_ContainingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizations_ContainingRef {
        DataOrganizations_ContainingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizations_ContainingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account` after provisioning.\nFilter the list of organizations to the ones that contain this particular\naccount."]
    pub fn account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nFilter the list of organizations to the ones that contain this particular\norganization."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nFilter the list of organizations to the ones that contain this particular\nuser.\n\nIMPORTANT: Just because an organization \"contains\" a user is not a\nrepresentation of any authorization or privilege to manage any resources\ntherein. An organization \"containing\" a user simply means the user is managed by\nthat organization."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizations_Name {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ends_with: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starts_with: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizations_Name {
    #[doc =
        "Set the field `contains`.\n(case-insensitive) Filter the list of organizations to where the name contains a particular\nstring."]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ends_with`.\n(case-insensitive) Filter the list of organizations to where the name ends with a particular\nstring."]
    pub fn set_ends_with(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ends_with = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `starts_with`.\n(case-insensitive) Filter the list of organizations to where the name starts with a\nparticular string."]
    pub fn set_starts_with(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.starts_with = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizations_Name {
    type O = terrars::BlockAssignable<DataOrganizations_Name>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizations_Name {}

impl BuildDataOrganizations_Name {
    pub fn build(self) -> DataOrganizations_Name {
        DataOrganizations_Name {
            contains: core::default::Default::default(),
            ends_with: core::default::Default::default(),
            starts_with: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOrganizations_NameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizations_NameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizations_NameRef {
        DataOrganizations_NameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizations_NameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\n(case-insensitive) Filter the list of organizations to where the name contains a particular\nstring."]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ends_with` after provisioning.\n(case-insensitive) Filter the list of organizations to where the name ends with a particular\nstring."]
    pub fn ends_with(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ends_with", self.base))
    }

    #[doc =
        "Get a reference to the value of field `starts_with` after provisioning.\n(case-insensitive) Filter the list of organizations to where the name starts with a\nparticular string."]
    pub fn starts_with(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starts_with", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizations_Parent {}

impl DataOrganizations_Parent { }

impl terrars::ToListMappable for DataOrganizations_Parent {
    type O = terrars::BlockAssignable<DataOrganizations_Parent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizations_Parent {}

impl BuildDataOrganizations_Parent {
    pub fn build(self) -> DataOrganizations_Parent {
        DataOrganizations_Parent {}
    }
}

#[allow(dead_code)]
pub struct DataOrganizations_ParentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizations_ParentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizations_ParentRef {
        DataOrganizations_ParentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizations_ParentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nFilter the list of organizations to the ones that are a sub-organization\nof the specified organization.\n\n\"null\" is a valid value to provide for this parameter. It means \"where\nan organization has no parent (i.e. it is a 'root' organization).\""]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizations_Result_Meta_TenantFlags {}

impl DataOrganizations_Result_Meta_TenantFlags { }

impl terrars::ToListMappable for DataOrganizations_Result_Meta_TenantFlags {
    type O = terrars::BlockAssignable<DataOrganizations_Result_Meta_TenantFlags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizations_Result_Meta_TenantFlags {}

impl BuildDataOrganizations_Result_Meta_TenantFlags {
    pub fn build(self) -> DataOrganizations_Result_Meta_TenantFlags {
        DataOrganizations_Result_Meta_TenantFlags {}
    }
}

#[allow(dead_code)]
pub struct DataOrganizations_Result_Meta_TenantFlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizations_Result_Meta_TenantFlagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizations_Result_Meta_TenantFlagsRef {
        DataOrganizations_Result_Meta_TenantFlagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizations_Result_Meta_TenantFlagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_creation` after provisioning.\n"]
    pub fn account_creation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_creation", self.base))
    }

    #[doc = "Get a reference to the value of field `account_creation_applies_tenant_defaults` after provisioning.\n"]
    pub fn account_creation_applies_tenant_defaults(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_creation_applies_tenant_defaults", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `account_deletion` after provisioning.\n"]
    pub fn account_deletion(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_deletion", self.base))
    }

    #[doc = "Get a reference to the value of field `account_migration` after provisioning.\n"]
    pub fn account_migration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_migration", self.base))
    }

    #[doc = "Get a reference to the value of field `account_mobility` after provisioning.\n"]
    pub fn account_mobility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_mobility", self.base))
    }

    #[doc = "Get a reference to the value of field `enterprise_capability` after provisioning.\n"]
    pub fn enterprise_capability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enterprise_capability", self.base))
    }

    #[doc = "Get a reference to the value of field `member_management` after provisioning.\n"]
    pub fn member_management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.member_management", self.base))
    }

    #[doc = "Get a reference to the value of field `sub_org_creation` after provisioning.\n"]
    pub fn sub_org_creation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sub_org_creation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizations_Result_Meta {}

impl DataOrganizations_Result_Meta { }

impl terrars::ToListMappable for DataOrganizations_Result_Meta {
    type O = terrars::BlockAssignable<DataOrganizations_Result_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizations_Result_Meta {}

impl BuildDataOrganizations_Result_Meta {
    pub fn build(self) -> DataOrganizations_Result_Meta {
        DataOrganizations_Result_Meta {}
    }
}

#[allow(dead_code)]
pub struct DataOrganizations_Result_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizations_Result_MetaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizations_Result_MetaRef {
        DataOrganizations_Result_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizations_Result_MetaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hierarchy_tags` after provisioning.\nOrdered chain of organization tags from the root organization down to\n(and including) this organization itself. Root organizations return a\nsingle-element array containing their own tag; sub-organizations return\n`[rootTag, ...intermediateTags, parentTag, selfTag]`. Useful for\nconstructing authorization scopes that need to cover every ancestor\nin the hierarchy."]
    pub fn hierarchy_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\n"]
    pub fn managed_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_flags` after provisioning.\nEnable features for Organizations."]
    pub fn tenant_flags(&self) -> DataOrganizations_Result_Meta_TenantFlagsRef {
        DataOrganizations_Result_Meta_TenantFlagsRef::new(
            self.shared().clone(),
            format!("{}.tenant_flags", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataOrganizations_Result_Parent {}

impl DataOrganizations_Result_Parent { }

impl terrars::ToListMappable for DataOrganizations_Result_Parent {
    type O = terrars::BlockAssignable<DataOrganizations_Result_Parent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizations_Result_Parent {}

impl BuildDataOrganizations_Result_Parent {
    pub fn build(self) -> DataOrganizations_Result_Parent {
        DataOrganizations_Result_Parent {}
    }
}

#[allow(dead_code)]
pub struct DataOrganizations_Result_ParentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizations_Result_ParentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizations_Result_ParentRef {
        DataOrganizations_Result_ParentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizations_Result_ParentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizations_Result_Profile {}

impl DataOrganizations_Result_Profile { }

impl terrars::ToListMappable for DataOrganizations_Result_Profile {
    type O = terrars::BlockAssignable<DataOrganizations_Result_Profile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizations_Result_Profile {}

impl BuildDataOrganizations_Result_Profile {
    pub fn build(self) -> DataOrganizations_Result_Profile {
        DataOrganizations_Result_Profile {}
    }
}

#[allow(dead_code)]
pub struct DataOrganizations_Result_ProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizations_Result_ProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizations_Result_ProfileRef {
        DataOrganizations_Result_ProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizations_Result_ProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `business_address` after provisioning.\n"]
    pub fn business_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_address", self.base))
    }

    #[doc = "Get a reference to the value of field `business_email` after provisioning.\n"]
    pub fn business_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_email", self.base))
    }

    #[doc = "Get a reference to the value of field `business_name` after provisioning.\n"]
    pub fn business_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_name", self.base))
    }

    #[doc = "Get a reference to the value of field `business_phone` after provisioning.\n"]
    pub fn business_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_phone", self.base))
    }

    #[doc = "Get a reference to the value of field `external_metadata` after provisioning.\n"]
    pub fn external_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizations_Result {}

impl DataOrganizations_Result { }

impl terrars::ToListMappable for DataOrganizations_Result {
    type O = terrars::BlockAssignable<DataOrganizations_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizations_Result {}

impl BuildDataOrganizations_Result {
    pub fn build(self) -> DataOrganizations_Result {
        DataOrganizations_Result {}
    }
}

#[allow(dead_code)]
pub struct DataOrganizations_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizations_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOrganizations_ResultRef {
        DataOrganizations_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizations_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\n"]
    pub fn meta(&self) -> DataOrganizations_Result_MetaRef {
        DataOrganizations_Result_MetaRef::new(self.shared().clone(), format!("{}.meta", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> DataOrganizations_Result_ParentRef {
        DataOrganizations_Result_ParentRef::new(self.shared().clone(), format!("{}.parent", self.base))
    }

    #[doc = "Get a reference to the value of field `profile` after provisioning.\n"]
    pub fn profile(&self) -> DataOrganizations_Result_ProfileRef {
        DataOrganizations_Result_ProfileRef::new(self.shared().clone(), format!("{}.profile", self.base))
    }
}
