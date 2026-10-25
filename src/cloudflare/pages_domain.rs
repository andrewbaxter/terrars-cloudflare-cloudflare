#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct PagesDomainData_ {
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
    name: terrars::PrimField<std::string::String>,
    project_name: terrars::PrimField<std::string::String>,
}

struct PagesDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PagesDomainData_>,
}

#[derive(Clone)]
pub struct PagesDomain(std::rc::Rc<PagesDomain_>);

impl PagesDomain {
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
    pub fn validation_data(&self) -> PagesDomain_ValidationDataRef {
        PagesDomain_ValidationDataRef::new(self.shared().clone(), format!("{}.validation_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_data` after provisioning.\n"]
    pub fn verification_data(&self) -> PagesDomain_VerificationDataRef {
        PagesDomain_VerificationDataRef::new(
            self.shared().clone(),
            format!("{}.verification_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zone_tag` after provisioning.\n"]
    pub fn zone_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_tag", self.extract_ref()))
    }
}

impl terrars::Referable for PagesDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PagesDomain { }

impl terrars::ToListMappable for PagesDomain {
    type O = terrars::ListRef<PagesDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PagesDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_pages_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPagesDomain {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Fully qualified domain name for the Pages project, such as `example.com`."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the Pages project. Must begin with a lowercase letter or digit and contain only lowercase letters, digits, and hyphens."]
    pub project_name: terrars::PrimField<std::string::String>,
}

impl BuildPagesDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> PagesDomain {
        let out = PagesDomain(std::rc::Rc::new(PagesDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PagesDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                name: self.name,
                project_name: self.project_name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PagesDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PagesDomainRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn validation_data(&self) -> PagesDomain_ValidationDataRef {
        PagesDomain_ValidationDataRef::new(self.shared().clone(), format!("{}.validation_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_data` after provisioning.\n"]
    pub fn verification_data(&self) -> PagesDomain_VerificationDataRef {
        PagesDomain_VerificationDataRef::new(
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
pub struct PagesDomain_ValidationData {}

impl PagesDomain_ValidationData { }

impl terrars::ToListMappable for PagesDomain_ValidationData {
    type O = terrars::BlockAssignable<PagesDomain_ValidationData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesDomain_ValidationData {}

impl BuildPagesDomain_ValidationData {
    pub fn build(self) -> PagesDomain_ValidationData {
        PagesDomain_ValidationData {}
    }
}

#[allow(dead_code)]
pub struct PagesDomain_ValidationDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesDomain_ValidationDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesDomain_ValidationDataRef {
        PagesDomain_ValidationDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesDomain_ValidationDataRef {
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
pub struct PagesDomain_VerificationData {}

impl PagesDomain_VerificationData { }

impl terrars::ToListMappable for PagesDomain_VerificationData {
    type O = terrars::BlockAssignable<PagesDomain_VerificationData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPagesDomain_VerificationData {}

impl BuildPagesDomain_VerificationData {
    pub fn build(self) -> PagesDomain_VerificationData {
        PagesDomain_VerificationData {}
    }
}

#[allow(dead_code)]
pub struct PagesDomain_VerificationDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PagesDomain_VerificationDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PagesDomain_VerificationDataRef {
        PagesDomain_VerificationDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PagesDomain_VerificationDataRef {
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
