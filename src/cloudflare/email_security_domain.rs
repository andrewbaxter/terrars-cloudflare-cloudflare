#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct EmailSecurityDomainData_ {
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
    allowed_delivery_modes: terrars::SetField<terrars::PrimField<std::string::String>>,
    domain: terrars::PrimField<std::string::String>,
    drop_dispositions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    folder: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_id: std::option::Option<terrars::PrimField<std::string::String>>,
    ip_restrictions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lookback_hops: std::option::Option<terrars::PrimField<f64>>,
    regions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_tls_inbound: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_tls_outbound: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transport: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct EmailSecurityDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EmailSecurityDomainData_>,
}

#[derive(Clone)]
pub struct EmailSecurityDomain(std::rc::Rc<EmailSecurityDomain_>);

impl EmailSecurityDomain {
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
        "Set the field `folder`.\nThe mailbox folder to scan, for API-scanning domains.\nAvailable values: \"AllItems\", \"Inbox\"."]
    pub fn set_folder(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().folder = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `integration_id`.\nIdentifier of the CASB integration that authorizes this domain. The integration also enables API scanning, post-delivery actions, and directory sync."]
    pub fn set_integration_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().integration_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lookback_hops`.\nNumber of hops to trace back through received headers when reconstructing the original message (1-20)."]
    pub fn set_lookback_hops(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().lookback_hops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_tls_inbound`.\nRequire TLS on inbound connections."]
    pub fn set_require_tls_inbound(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_tls_inbound = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_tls_outbound`.\nRequire TLS on outbound connections."]
    pub fn set_require_tls_outbound(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_tls_outbound = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transport`.\nThe mail transport hostname for MX/Inline delivery — the MX record Cloudflare delivers email to (e.g. `mx.example.com`)."]
    pub fn set_transport(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().transport = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed_delivery_modes` after provisioning.\nDelivery modes to onboard the domain through."]
    pub fn allowed_delivery_modes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_delivery_modes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> EmailSecurityDomain_AuthorizationRef {
        EmailSecurityDomain_AuthorizationRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dmarc_status` after provisioning.\nAvailable values: \"none\", \"good\", \"invalid\"."]
    pub fn dmarc_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dmarc_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe email domain to protect."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `drop_dispositions` after provisioning.\nDispositions to drop instead of delivering, e.g. `[\"MALICIOUS\", \"SPAM\"]`."]
    pub fn drop_dispositions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.drop_dispositions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails_processed` after provisioning.\n"]
    pub fn emails_processed(&self) -> EmailSecurityDomain_EmailsProcessedRef {
        EmailSecurityDomain_EmailsProcessedRef::new(
            self.shared().clone(),
            format!("{}.emails_processed", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `folder` after provisioning.\nThe mailbox folder to scan, for API-scanning domains.\nAvailable values: \"AllItems\", \"Inbox\"."]
    pub fn folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nDomain identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inbox_provider` after provisioning.\nAvailable values: \"Microsoft\", \"Google\"."]
    pub fn inbox_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbox_provider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integration_id` after provisioning.\nIdentifier of the CASB integration that authorizes this domain. The integration also enables API scanning, post-delivery actions, and directory sync."]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_restrictions` after provisioning.\nSource IP ranges mail is accepted from. Any other source is rejected."]
    pub fn ip_restrictions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_restrictions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lookback_hops` after provisioning.\nNumber of hops to trace back through received headers when reconstructing the original message (1-20)."]
    pub fn lookback_hops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookback_hops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `o365_tenant_id` after provisioning.\n"]
    pub fn o365_tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.o365_tenant_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `regions` after provisioning.\nRegions that process messages for this domain, e.g. `[\"GLOBAL\"]` or `[\"US\"]`."]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_tls_inbound` after provisioning.\nRequire TLS on inbound connections."]
    pub fn require_tls_inbound(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls_inbound", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_tls_outbound` after provisioning.\nRequire TLS on outbound connections."]
    pub fn require_tls_outbound(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls_outbound", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `spf_status` after provisioning.\nAvailable values: \"none\", \"good\", \"neutral\", \"open\", \"invalid\"."]
    pub fn spf_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spf_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"PENDING\", \"ACTIVE\", \"FAILED\", \"TIMEOUT\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transport` after provisioning.\nThe mail transport hostname for MX/Inline delivery — the MX record Cloudflare delivers email to (e.g. `mx.example.com`)."]
    pub fn transport(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport", self.extract_ref()))
    }
}

impl terrars::Referable for EmailSecurityDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EmailSecurityDomain { }

impl terrars::ToListMappable for EmailSecurityDomain {
    type O = terrars::ListRef<EmailSecurityDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EmailSecurityDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_email_security_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmailSecurityDomain {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Delivery modes to onboard the domain through."]
    pub allowed_delivery_modes: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = "The email domain to protect."]
    pub domain: terrars::PrimField<std::string::String>,
    #[doc = "Dispositions to drop instead of delivering, e.g. `[\"MALICIOUS\", \"SPAM\"]`."]
    pub drop_dispositions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = "Source IP ranges mail is accepted from. Any other source is rejected."]
    pub ip_restrictions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = "Regions that process messages for this domain, e.g. `[\"GLOBAL\"]` or `[\"US\"]`."]
    pub regions: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEmailSecurityDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> EmailSecurityDomain {
        let out = EmailSecurityDomain(std::rc::Rc::new(EmailSecurityDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EmailSecurityDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                allowed_delivery_modes: self.allowed_delivery_modes,
                domain: self.domain,
                drop_dispositions: self.drop_dispositions,
                folder: core::default::Default::default(),
                integration_id: core::default::Default::default(),
                ip_restrictions: self.ip_restrictions,
                lookback_hops: core::default::Default::default(),
                regions: self.regions,
                require_tls_inbound: core::default::Default::default(),
                require_tls_outbound: core::default::Default::default(),
                transport: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmailSecurityDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmailSecurityDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmailSecurityDomainRef {
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
        "Get a reference to the value of field `allowed_delivery_modes` after provisioning.\nDelivery modes to onboard the domain through."]
    pub fn allowed_delivery_modes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_delivery_modes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> EmailSecurityDomain_AuthorizationRef {
        EmailSecurityDomain_AuthorizationRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dmarc_status` after provisioning.\nAvailable values: \"none\", \"good\", \"invalid\"."]
    pub fn dmarc_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dmarc_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe email domain to protect."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `drop_dispositions` after provisioning.\nDispositions to drop instead of delivering, e.g. `[\"MALICIOUS\", \"SPAM\"]`."]
    pub fn drop_dispositions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.drop_dispositions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `emails_processed` after provisioning.\n"]
    pub fn emails_processed(&self) -> EmailSecurityDomain_EmailsProcessedRef {
        EmailSecurityDomain_EmailsProcessedRef::new(
            self.shared().clone(),
            format!("{}.emails_processed", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `folder` after provisioning.\nThe mailbox folder to scan, for API-scanning domains.\nAvailable values: \"AllItems\", \"Inbox\"."]
    pub fn folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nDomain identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inbox_provider` after provisioning.\nAvailable values: \"Microsoft\", \"Google\"."]
    pub fn inbox_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbox_provider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integration_id` after provisioning.\nIdentifier of the CASB integration that authorizes this domain. The integration also enables API scanning, post-delivery actions, and directory sync."]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_restrictions` after provisioning.\nSource IP ranges mail is accepted from. Any other source is rejected."]
    pub fn ip_restrictions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_restrictions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified` after provisioning.\nDeprecated, use `modified_at` instead. End of life: November 1, 2026."]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lookback_hops` after provisioning.\nNumber of hops to trace back through received headers when reconstructing the original message (1-20)."]
    pub fn lookback_hops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookback_hops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `o365_tenant_id` after provisioning.\n"]
    pub fn o365_tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.o365_tenant_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `regions` after provisioning.\nRegions that process messages for this domain, e.g. `[\"GLOBAL\"]` or `[\"US\"]`."]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_tls_inbound` after provisioning.\nRequire TLS on inbound connections."]
    pub fn require_tls_inbound(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls_inbound", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_tls_outbound` after provisioning.\nRequire TLS on outbound connections."]
    pub fn require_tls_outbound(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls_outbound", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `spf_status` after provisioning.\nAvailable values: \"none\", \"good\", \"neutral\", \"open\", \"invalid\"."]
    pub fn spf_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spf_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nAvailable values: \"PENDING\", \"ACTIVE\", \"FAILED\", \"TIMEOUT\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transport` after provisioning.\nThe mail transport hostname for MX/Inline delivery — the MX record Cloudflare delivers email to (e.g. `mx.example.com`)."]
    pub fn transport(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EmailSecurityDomain_Authorization {}

impl EmailSecurityDomain_Authorization { }

impl terrars::ToListMappable for EmailSecurityDomain_Authorization {
    type O = terrars::BlockAssignable<EmailSecurityDomain_Authorization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmailSecurityDomain_Authorization {}

impl BuildEmailSecurityDomain_Authorization {
    pub fn build(self) -> EmailSecurityDomain_Authorization {
        EmailSecurityDomain_Authorization {}
    }
}

#[allow(dead_code)]
pub struct EmailSecurityDomain_AuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmailSecurityDomain_AuthorizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmailSecurityDomain_AuthorizationRef {
        EmailSecurityDomain_AuthorizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmailSecurityDomain_AuthorizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorized` after provisioning.\n"]
    pub fn authorized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorized", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}

#[derive(Serialize)]
pub struct EmailSecurityDomain_EmailsProcessed {}

impl EmailSecurityDomain_EmailsProcessed { }

impl terrars::ToListMappable for EmailSecurityDomain_EmailsProcessed {
    type O = terrars::BlockAssignable<EmailSecurityDomain_EmailsProcessed>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmailSecurityDomain_EmailsProcessed {}

impl BuildEmailSecurityDomain_EmailsProcessed {
    pub fn build(self) -> EmailSecurityDomain_EmailsProcessed {
        EmailSecurityDomain_EmailsProcessed {}
    }
}

#[allow(dead_code)]
pub struct EmailSecurityDomain_EmailsProcessedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmailSecurityDomain_EmailsProcessedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmailSecurityDomain_EmailsProcessedRef {
        EmailSecurityDomain_EmailsProcessedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmailSecurityDomain_EmailsProcessedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `total_emails_processed` after provisioning.\n"]
    pub fn total_emails_processed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_emails_processed", self.base))
    }

    #[doc = "Get a reference to the value of field `total_emails_processed_previous` after provisioning.\n"]
    pub fn total_emails_processed_previous(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_emails_processed_previous", self.base))
    }
}
