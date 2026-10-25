#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataUserData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
}

struct DataUser_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataUserData_>,
}

#[derive(Clone)]
pub struct DataUser(std::rc::Rc<DataUser_>);

impl DataUser {
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
        "Get a reference to the value of field `betas` after provisioning.\nLists the betas that the user is participating in."]
    pub fn betas(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.betas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\nThe country in which the user lives."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nCurrent email address of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\nUser's first name"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_business_zones` after provisioning.\nIndicates whether user has any business zones"]
    pub fn has_business_zones(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_business_zones", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_enterprise_zones` after provisioning.\nIndicates whether user has any enterprise zones"]
    pub fn has_enterprise_zones(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_enterprise_zones", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_pro_zones` after provisioning.\nIndicates whether user has any pro zones"]
    pub fn has_pro_zones(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_pro_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the user."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\nUser's last name"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizations` after provisioning.\n"]
    pub fn organizations(&self) -> terrars::ListRef<DataUser_OrganizationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.organizations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nIndicates whether user has been suspended"]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `telephone` after provisioning.\nUser's telephone number"]
    pub fn telephone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.telephone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `two_factor_authentication_enabled` after provisioning.\nIndicates whether two-factor authentication is enabled for the user account. Does not apply to API authentication."]
    pub fn two_factor_authentication_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.two_factor_authentication_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `two_factor_authentication_locked` after provisioning.\nIndicates whether two-factor authentication is required by one of the accounts that the user is a member of."]
    pub fn two_factor_authentication_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.two_factor_authentication_locked", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `zipcode` after provisioning.\nThe zipcode or postal code where the user lives."]
    pub fn zipcode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zipcode", self.extract_ref()))
    }
}

impl terrars::Referable for DataUser {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataUser { }

impl terrars::ToListMappable for DataUser {
    type O = terrars::ListRef<DataUserRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataUser_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_user".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUser {
    pub tf_id: std::string::String,
}

impl BuildDataUser {
    pub fn build(self, stack: &mut terrars::Stack) -> DataUser {
        let out = DataUser(std::rc::Rc::new(DataUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataUserData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUserRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `betas` after provisioning.\nLists the betas that the user is participating in."]
    pub fn betas(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.betas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\nThe country in which the user lives."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nCurrent email address of the user."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\nUser's first name"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_business_zones` after provisioning.\nIndicates whether user has any business zones"]
    pub fn has_business_zones(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_business_zones", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_enterprise_zones` after provisioning.\nIndicates whether user has any enterprise zones"]
    pub fn has_enterprise_zones(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_enterprise_zones", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_pro_zones` after provisioning.\nIndicates whether user has any pro zones"]
    pub fn has_pro_zones(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_pro_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier of the user."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\nUser's last name"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizations` after provisioning.\n"]
    pub fn organizations(&self) -> terrars::ListRef<DataUser_OrganizationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.organizations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nIndicates whether user has been suspended"]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `telephone` after provisioning.\nUser's telephone number"]
    pub fn telephone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.telephone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `two_factor_authentication_enabled` after provisioning.\nIndicates whether two-factor authentication is enabled for the user account. Does not apply to API authentication."]
    pub fn two_factor_authentication_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.two_factor_authentication_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `two_factor_authentication_locked` after provisioning.\nIndicates whether two-factor authentication is required by one of the accounts that the user is a member of."]
    pub fn two_factor_authentication_locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.two_factor_authentication_locked", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `zipcode` after provisioning.\nThe zipcode or postal code where the user lives."]
    pub fn zipcode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zipcode", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataUser_Organizations {}

impl DataUser_Organizations { }

impl terrars::ToListMappable for DataUser_Organizations {
    type O = terrars::BlockAssignable<DataUser_Organizations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUser_Organizations {}

impl BuildDataUser_Organizations {
    pub fn build(self) -> DataUser_Organizations {
        DataUser_Organizations {}
    }
}

#[allow(dead_code)]
pub struct DataUser_OrganizationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataUser_OrganizationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataUser_OrganizationsRef {
        DataUser_OrganizationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUser_OrganizationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nOrganization name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nAccess permissions for this User."]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `roles` after provisioning.\nList of roles that a user has within an organization."]
    pub fn roles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nWhether the user is a member of the organization or has an invitation pending.\nAvailable values: \"member\", \"invited\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
