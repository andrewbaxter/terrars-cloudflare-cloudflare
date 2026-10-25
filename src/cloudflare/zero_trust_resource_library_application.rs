#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ZeroTrustResourceLibraryApplicationData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    category_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostnames: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_protocols: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_domains: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

struct ZeroTrustResourceLibraryApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ZeroTrustResourceLibraryApplicationData_>,
}

#[derive(Clone)]
pub struct ZeroTrustResourceLibraryApplication(std::rc::Rc<ZeroTrustResourceLibraryApplication_>);

impl ZeroTrustResourceLibraryApplication {
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

    #[doc = "Set the field `category_id`.\nNumeric identifier for an application category."]
    pub fn set_category_id(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().category_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostnames`.\nHostnames matched by the application."]
    pub fn set_hostnames(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().hostnames = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `human_id`.\nHuman-readable identifier for the application."]
    pub fn set_human_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().human_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_subnets`.\nIP subnets for this application. Custom application create and update requests accept IPv4 prefix lengths /8 through /32 and IPv6 prefix lengths /32 through /128."]
    pub fn set_ip_subnets(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ip_subnets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nApplication name."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_protocols`.\nPorts matched by the application, in `protocol/port` or inclusive `protocol/start-end` format (for example, `tcp/443` or `udp/10000-20000`). TCP and UDP ports must be between 0 and 65535."]
    pub fn set_port_protocols(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().port_protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `support_domains`.\nSupport domains matched by the application."]
    pub fn set_support_domains(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().support_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_confidence_score` after provisioning.\nConfidence score for the application. Returns -1 when no score is available."]
    pub fn application_confidence_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_confidence_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_score_composition` after provisioning.\nReturns the score composition breakdown for the application."]
    pub fn application_score_composition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_score_composition", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `application_source` after provisioning.\nReturns the application source."]
    pub fn application_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_type` after provisioning.\nReturns the application type."]
    pub fn application_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_type_description` after provisioning.\nReturns the application type description."]
    pub fn application_type_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `category_id` after provisioning.\nNumeric identifier for an application category."]
    pub fn category_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nReturns the application creation time."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gen_ai_score` after provisioning.\nGenAI score for the application. Returns -1 when no score is available."]
    pub fn gen_ai_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gen_ai_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nHostnames matched by the application."]
    pub fn hostnames(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `human_id` after provisioning.\nHuman-readable identifier for the application."]
    pub fn human_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.human_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nNumeric identifier for the application."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_subnets` after provisioning.\nIP subnets for this application. Custom application create and update requests accept IPv4 prefix lengths /8 through /32 and IPv6 prefix lengths /32 through /128."]
    pub fn ip_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nApplication name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port_protocols` after provisioning.\nPorts matched by the application, in `protocol/port` or inclusive `protocol/start-end` format (for example, `tcp/443` or `udp/10000-20000`). TCP and UDP ports must be between 0 and 65535."]
    pub fn port_protocols(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.port_protocols", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `support_domains` after provisioning.\nSupport domains matched by the application."]
    pub fn support_domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.support_domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported` after provisioning.\nCloudflare products that support this application."]
    pub fn supported(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nReturns the application update time."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nReturns the application version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for ZeroTrustResourceLibraryApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ZeroTrustResourceLibraryApplication { }

impl terrars::ToListMappable for ZeroTrustResourceLibraryApplication {
    type O = terrars::ListRef<ZeroTrustResourceLibraryApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ZeroTrustResourceLibraryApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_resource_library_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildZeroTrustResourceLibraryApplication {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildZeroTrustResourceLibraryApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> ZeroTrustResourceLibraryApplication {
        let out = ZeroTrustResourceLibraryApplication(std::rc::Rc::new(ZeroTrustResourceLibraryApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ZeroTrustResourceLibraryApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                category_id: core::default::Default::default(),
                hostnames: core::default::Default::default(),
                human_id: core::default::Default::default(),
                ip_subnets: core::default::Default::default(),
                name: core::default::Default::default(),
                port_protocols: core::default::Default::default(),
                support_domains: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ZeroTrustResourceLibraryApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ZeroTrustResourceLibraryApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ZeroTrustResourceLibraryApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_confidence_score` after provisioning.\nConfidence score for the application. Returns -1 when no score is available."]
    pub fn application_confidence_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_confidence_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_score_composition` after provisioning.\nReturns the score composition breakdown for the application."]
    pub fn application_score_composition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_score_composition", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `application_source` after provisioning.\nReturns the application source."]
    pub fn application_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_type` after provisioning.\nReturns the application type."]
    pub fn application_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_type_description` after provisioning.\nReturns the application type description."]
    pub fn application_type_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_type_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `category_id` after provisioning.\nNumeric identifier for an application category."]
    pub fn category_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nReturns the application creation time."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gen_ai_score` after provisioning.\nGenAI score for the application. Returns -1 when no score is available."]
    pub fn gen_ai_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gen_ai_score", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostnames` after provisioning.\nHostnames matched by the application."]
    pub fn hostnames(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hostnames", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `human_id` after provisioning.\nHuman-readable identifier for the application."]
    pub fn human_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.human_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nNumeric identifier for the application."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_subnets` after provisioning.\nIP subnets for this application. Custom application create and update requests accept IPv4 prefix lengths /8 through /32 and IPv6 prefix lengths /32 through /128."]
    pub fn ip_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nApplication name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port_protocols` after provisioning.\nPorts matched by the application, in `protocol/port` or inclusive `protocol/start-end` format (for example, `tcp/443` or `udp/10000-20000`). TCP and UDP ports must be between 0 and 65535."]
    pub fn port_protocols(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.port_protocols", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `support_domains` after provisioning.\nSupport domains matched by the application."]
    pub fn support_domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.support_domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported` after provisioning.\nCloudflare products that support this application."]
    pub fn supported(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nReturns the application update time."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nReturns the application version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}
