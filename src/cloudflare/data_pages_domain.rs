#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataPagesDomainData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    domain_name: terrars::PrimField<std::string::String>,
    project_name: terrars::PrimField<std::string::String>,
}

struct DataPagesDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPagesDomainData_>,
}

#[derive(Clone)]
pub struct DataPagesDomain(std::rc::Rc<DataPagesDomain_>);

impl DataPagesDomain {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nAvailable values: \"google\", \"lets_encrypt\"."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_name` after provisioning.\nFully qualified domain name for the Pages project, such as `example.com`."]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nFully qualified domain name for the Pages project, such as `example.com`."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFully qualified domain name for the Pages project, such as `example.com`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_name` after provisioning.\nName of the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"initializing\", \"pending\", \"active\", \"deactivated\", \"blocked\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_data` after provisioning.\n"]
    pub fn validation_data(&self) -> DataPagesDomain_ValidationDataRef {
        DataPagesDomain_ValidationDataRef::new(
            self.shared().clone(),
            format!("{}.validation_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `verification_data` after provisioning.\n"]
    pub fn verification_data(&self) -> DataPagesDomain_VerificationDataRef {
        DataPagesDomain_VerificationDataRef::new(
            self.shared().clone(),
            format!("{}.verification_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zone_tag` after provisioning.\n"]
    pub fn zone_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_tag", self.extract_ref()))
    }
}

impl terrars::Referable for DataPagesDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPagesDomain { }

impl terrars::ToListMappable for DataPagesDomain {
    type O = terrars::ListRef<DataPagesDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPagesDomain_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_pages_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPagesDomain {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Fully qualified domain name for the Pages project, such as `example.com`."]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub project_name: terrars::PrimField<std::string::String>,
}

impl BuildDataPagesDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPagesDomain {
        let out = DataPagesDomain(std::rc::Rc::new(DataPagesDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPagesDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                domain_name: self.domain_name,
                project_name: self.project_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPagesDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPagesDomainRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nAvailable values: \"google\", \"lets_encrypt\"."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_on` after provisioning.\n"]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_name` after provisioning.\nFully qualified domain name for the Pages project, such as `example.com`."]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nFully qualified domain name for the Pages project, such as `example.com`."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFully qualified domain name for the Pages project, such as `example.com`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_name` after provisioning.\nName of the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"initializing\", \"pending\", \"active\", \"deactivated\", \"blocked\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_data` after provisioning.\n"]
    pub fn validation_data(&self) -> DataPagesDomain_ValidationDataRef {
        DataPagesDomain_ValidationDataRef::new(
            self.shared().clone(),
            format!("{}.validation_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `verification_data` after provisioning.\n"]
    pub fn verification_data(&self) -> DataPagesDomain_VerificationDataRef {
        DataPagesDomain_VerificationDataRef::new(
            self.shared().clone(),
            format!("{}.verification_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zone_tag` after provisioning.\n"]
    pub fn zone_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_tag", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPagesDomain_ValidationData {}

impl DataPagesDomain_ValidationData { }

impl terrars::ToListMappable for DataPagesDomain_ValidationData {
    type O = terrars::BlockAssignable<DataPagesDomain_ValidationData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesDomain_ValidationData {}

impl BuildDataPagesDomain_ValidationData {
    pub fn build(self) -> DataPagesDomain_ValidationData {
        DataPagesDomain_ValidationData {}
    }
}

#[allow(dead_code)]
pub struct DataPagesDomain_ValidationDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesDomain_ValidationDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPagesDomain_ValidationDataRef {
        DataPagesDomain_ValidationDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesDomain_ValidationDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\nAvailable values: \"http\", \"txt\"."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"initializing\", \"pending\", \"active\", \"deactivated\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `txt_name` after provisioning.\n"]
    pub fn txt_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.txt_name", self.base))
    }

    #[doc = "Get a reference to the value of field `txt_value` after provisioning.\n"]
    pub fn txt_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.txt_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPagesDomain_VerificationData {}

impl DataPagesDomain_VerificationData { }

impl terrars::ToListMappable for DataPagesDomain_VerificationData {
    type O = terrars::BlockAssignable<DataPagesDomain_VerificationData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPagesDomain_VerificationData {}

impl BuildDataPagesDomain_VerificationData {
    pub fn build(self) -> DataPagesDomain_VerificationData {
        DataPagesDomain_VerificationData {}
    }
}

#[allow(dead_code)]
pub struct DataPagesDomain_VerificationDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPagesDomain_VerificationDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPagesDomain_VerificationDataRef {
        DataPagesDomain_VerificationDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPagesDomain_VerificationDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"pending\", \"active\", \"deactivated\", \"blocked\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
