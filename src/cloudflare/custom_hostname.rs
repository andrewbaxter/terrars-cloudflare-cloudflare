#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct CustomHostnameData_ {
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
    custom_metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_origin_server: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_origin_sni: std::option::Option<terrars::PrimField<std::string::String>>,
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<CustomHostname_Ssl>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct CustomHostname_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CustomHostnameData_>,
}

#[derive(Clone)]
pub struct CustomHostname(std::rc::Rc<CustomHostname_>);

impl CustomHostname {
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
        "Set the field `custom_metadata`.\nUnique key/value metadata for this hostname. These are per-hostname (customer) settings."]
    pub fn set_custom_metadata(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_origin_server`.\na valid hostname that’s been added to your DNS zone as an A, AAAA, or CNAME record."]
    pub fn set_custom_origin_server(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_origin_server = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_origin_sni`.\nA hostname that will be sent to your custom origin server as SNI for TLS handshake. This can be a valid subdomain of the zone or custom origin server name or the string ':request_host_header:' which will cause the host header in the request to be used as SNI. Not configurable with default/fallback origin server."]
    pub fn set_custom_origin_sni(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_origin_sni = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl`.\nSSL properties used when creating the custom hostname."]
    pub fn set_ssl(self, v: impl std::convert::Into<CustomHostname_Ssl>) -> Self {
        self.0.data.borrow_mut().ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThis is the time the hostname was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_metadata` after provisioning.\nUnique key/value metadata for this hostname. These are per-hostname (customer) settings."]
    pub fn custom_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_server` after provisioning.\na valid hostname that’s been added to your DNS zone as an A, AAAA, or CNAME record."]
    pub fn custom_origin_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_server", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_sni` after provisioning.\nA hostname that will be sent to your custom origin server as SNI for TLS handshake. This can be a valid subdomain of the zone or custom origin server name or the string ':request_host_header:' which will cause the host header in the request to be used as SNI. Not configurable with default/fallback origin server."]
    pub fn custom_origin_sni(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_sni", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe custom hostname that will point to your hostname via CNAME."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nCustom hostname identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_verification` after provisioning.\nThis is a record which can be placed to activate a hostname."]
    pub fn ownership_verification(&self) -> CustomHostname_OwnershipVerificationRef {
        CustomHostname_OwnershipVerificationRef::new(
            self.shared().clone(),
            format!("{}.ownership_verification", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `ownership_verification_http` after provisioning.\nThis presents the token to be served by the given http url to activate a hostname."]
    pub fn ownership_verification_http(&self) -> CustomHostname_OwnershipVerificationHttpRef {
        CustomHostname_OwnershipVerificationHttpRef::new(
            self.shared().clone(),
            format!("{}.ownership_verification_http", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `ssl` after provisioning.\nSSL properties used when creating the custom hostname."]
    pub fn ssl(&self) -> CustomHostname_SslRef {
        CustomHostname_SslRef::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the hostname's activation.\nAvailable values: \"active\", \"pending\", \"active_redeploying\", \"moved\", \"pending_deletion\", \"deleted\", \"pending_blocked\", \"pending_migration\", \"pending_provisioned\", \"test_pending\", \"test_active\", \"test_active_apex\", \"test_blocked\", \"test_failed\", \"provisioned\", \"blocked\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `verification_errors` after provisioning.\nThese are errors that were encountered while trying to activate a hostname."]
    pub fn verification_errors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verification_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for CustomHostname {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CustomHostname { }

impl terrars::ToListMappable for CustomHostname {
    type O = terrars::ListRef<CustomHostnameRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CustomHostname_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_custom_hostname".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCustomHostname {
    pub tf_id: std::string::String,
    #[doc = "The custom hostname that will point to your hostname via CNAME."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildCustomHostname {
    pub fn build(self, stack: &mut terrars::Stack) -> CustomHostname {
        let out = CustomHostname(std::rc::Rc::new(CustomHostname_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CustomHostnameData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                custom_metadata: core::default::Default::default(),
                custom_origin_server: core::default::Default::default(),
                custom_origin_sni: core::default::Default::default(),
                hostname: self.hostname,
                ssl: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CustomHostnameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomHostnameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CustomHostnameRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThis is the time the hostname was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_metadata` after provisioning.\nUnique key/value metadata for this hostname. These are per-hostname (customer) settings."]
    pub fn custom_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_server` after provisioning.\na valid hostname that’s been added to your DNS zone as an A, AAAA, or CNAME record."]
    pub fn custom_origin_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_server", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_sni` after provisioning.\nA hostname that will be sent to your custom origin server as SNI for TLS handshake. This can be a valid subdomain of the zone or custom origin server name or the string ':request_host_header:' which will cause the host header in the request to be used as SNI. Not configurable with default/fallback origin server."]
    pub fn custom_origin_sni(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_sni", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe custom hostname that will point to your hostname via CNAME."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nCustom hostname identifier tag."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_verification` after provisioning.\nThis is a record which can be placed to activate a hostname."]
    pub fn ownership_verification(&self) -> CustomHostname_OwnershipVerificationRef {
        CustomHostname_OwnershipVerificationRef::new(
            self.shared().clone(),
            format!("{}.ownership_verification", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `ownership_verification_http` after provisioning.\nThis presents the token to be served by the given http url to activate a hostname."]
    pub fn ownership_verification_http(&self) -> CustomHostname_OwnershipVerificationHttpRef {
        CustomHostname_OwnershipVerificationHttpRef::new(
            self.shared().clone(),
            format!("{}.ownership_verification_http", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `ssl` after provisioning.\nSSL properties used when creating the custom hostname."]
    pub fn ssl(&self) -> CustomHostname_SslRef {
        CustomHostname_SslRef::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the hostname's activation.\nAvailable values: \"active\", \"pending\", \"active_redeploying\", \"moved\", \"pending_deletion\", \"deleted\", \"pending_blocked\", \"pending_migration\", \"pending_provisioned\", \"test_pending\", \"test_active\", \"test_active_apex\", \"test_blocked\", \"test_failed\", \"provisioned\", \"blocked\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `verification_errors` after provisioning.\nThese are errors that were encountered while trying to activate a hostname."]
    pub fn verification_errors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verification_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CustomHostname_OwnershipVerification {}

impl CustomHostname_OwnershipVerification { }

impl terrars::ToListMappable for CustomHostname_OwnershipVerification {
    type O = terrars::BlockAssignable<CustomHostname_OwnershipVerification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostname_OwnershipVerification {}

impl BuildCustomHostname_OwnershipVerification {
    pub fn build(self) -> CustomHostname_OwnershipVerification {
        CustomHostname_OwnershipVerification {}
    }
}

#[allow(dead_code)]
pub struct CustomHostname_OwnershipVerificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomHostname_OwnershipVerificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomHostname_OwnershipVerificationRef {
        CustomHostname_OwnershipVerificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostname_OwnershipVerificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDNS Name for record."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nDNS Record type.\nAvailable values: \"txt\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nContent for the record."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomHostname_OwnershipVerificationHttp {}

impl CustomHostname_OwnershipVerificationHttp { }

impl terrars::ToListMappable for CustomHostname_OwnershipVerificationHttp {
    type O = terrars::BlockAssignable<CustomHostname_OwnershipVerificationHttp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostname_OwnershipVerificationHttp {}

impl BuildCustomHostname_OwnershipVerificationHttp {
    pub fn build(self) -> CustomHostname_OwnershipVerificationHttp {
        CustomHostname_OwnershipVerificationHttp {}
    }
}

#[allow(dead_code)]
pub struct CustomHostname_OwnershipVerificationHttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomHostname_OwnershipVerificationHttpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomHostname_OwnershipVerificationHttpRef {
        CustomHostname_OwnershipVerificationHttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostname_OwnershipVerificationHttpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_body` after provisioning.\nToken to be served."]
    pub fn http_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_url` after provisioning.\nThe HTTP URL that will be checked during custom hostname verification and where the customer should host the token."]
    pub fn http_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_url", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomHostname_Ssl_CustomCertBundle {
    custom_certificate: terrars::PrimField<std::string::String>,
    custom_key: terrars::PrimField<std::string::String>,
}

impl CustomHostname_Ssl_CustomCertBundle { }

impl terrars::ToListMappable for CustomHostname_Ssl_CustomCertBundle {
    type O = terrars::BlockAssignable<CustomHostname_Ssl_CustomCertBundle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostname_Ssl_CustomCertBundle {
    #[doc = "If a custom uploaded certificate is used."]
    pub custom_certificate: terrars::PrimField<std::string::String>,
    #[doc = "The key for a custom uploaded certificate."]
    pub custom_key: terrars::PrimField<std::string::String>,
}

impl BuildCustomHostname_Ssl_CustomCertBundle {
    pub fn build(self) -> CustomHostname_Ssl_CustomCertBundle {
        CustomHostname_Ssl_CustomCertBundle {
            custom_certificate: self.custom_certificate,
            custom_key: self.custom_key,
        }
    }
}

#[allow(dead_code)]
pub struct CustomHostname_Ssl_CustomCertBundleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomHostname_Ssl_CustomCertBundleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomHostname_Ssl_CustomCertBundleRef {
        CustomHostname_Ssl_CustomCertBundleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostname_Ssl_CustomCertBundleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `custom_certificate` after provisioning.\nIf a custom uploaded certificate is used."]
    pub fn custom_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_key` after provisioning.\nThe key for a custom uploaded certificate."]
    pub fn custom_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_key", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomHostname_Ssl_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ciphers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    early_hints: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_tls_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_1_3: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomHostname_Ssl_Settings {
    #[doc =
        "Set the field `ciphers`.\nAn allowlist of ciphers for TLS termination. These ciphers must be in the BoringSSL format."]
    pub fn set_ciphers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ciphers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `early_hints`.\nWhether or not Early Hints is enabled.\nAvailable values: \"on\", \"off\"."]
    pub fn set_early_hints(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.early_hints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http2`.\nWhether or not HTTP2 is enabled.\nAvailable values: \"on\", \"off\"."]
    pub fn set_http2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http2 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_tls_version`.\nThe minimum TLS version supported.\nAvailable values: \"1.0\", \"1.1\", \"1.2\", \"1.3\"."]
    pub fn set_min_tls_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_tls_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_1_3`.\nWhether or not TLS 1.3 is enabled.\nAvailable values: \"on\", \"off\"."]
    pub fn set_tls_1_3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_1_3 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomHostname_Ssl_Settings {
    type O = terrars::BlockAssignable<CustomHostname_Ssl_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostname_Ssl_Settings {}

impl BuildCustomHostname_Ssl_Settings {
    pub fn build(self) -> CustomHostname_Ssl_Settings {
        CustomHostname_Ssl_Settings {
            ciphers: core::default::Default::default(),
            early_hints: core::default::Default::default(),
            http2: core::default::Default::default(),
            min_tls_version: core::default::Default::default(),
            tls_1_3: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomHostname_Ssl_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomHostname_Ssl_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomHostname_Ssl_SettingsRef {
        CustomHostname_Ssl_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostname_Ssl_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ciphers` after provisioning.\nAn allowlist of ciphers for TLS termination. These ciphers must be in the BoringSSL format."]
    pub fn ciphers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ciphers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `early_hints` after provisioning.\nWhether or not Early Hints is enabled.\nAvailable values: \"on\", \"off\"."]
    pub fn early_hints(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.early_hints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http2` after provisioning.\nWhether or not HTTP2 is enabled.\nAvailable values: \"on\", \"off\"."]
    pub fn http2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_tls_version` after provisioning.\nThe minimum TLS version supported.\nAvailable values: \"1.0\", \"1.1\", \"1.2\", \"1.3\"."]
    pub fn min_tls_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_tls_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_1_3` after provisioning.\nWhether or not TLS 1.3 is enabled.\nAvailable values: \"on\", \"off\"."]
    pub fn tls_1_3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_1_3", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomHostname_Ssl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bundle_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudflare_branding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_cert_bundle: std::option::Option<std::vec::Vec<CustomHostname_Ssl_CustomCertBundle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_csr_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<CustomHostname_Ssl_Settings>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
}

impl CustomHostname_Ssl {
    #[doc =
        "Set the field `bundle_method`.\nA ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it.\nAvailable values: \"ubiquitous\", \"optimal\", \"force\"."]
    pub fn set_bundle_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bundle_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `certificate_authority`.\nThe Certificate Authority that will issue the certificate.\nAvailable values: \"digicert\", \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub fn set_certificate_authority(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.certificate_authority = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cloudflare_branding`.\nWhether or not to add Cloudflare Branding for the order.  This will add a subdomain of sni.cloudflaressl.com as the Common Name if set to true."]
    pub fn set_cloudflare_branding(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudflare_branding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_cert_bundle`.\nArray of custom certificate and key pairs (1 or 2 pairs allowed)."]
    pub fn set_custom_cert_bundle(
        mut self,
        v: impl std::convert::Into<std::vec::Vec<CustomHostname_Ssl_CustomCertBundle>>,
    ) -> Self {
        self.custom_cert_bundle = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_certificate`.\nIf a custom uploaded certificate is used."]
    pub fn set_custom_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_csr_id`.\nThe identifier for the Custom CSR that was used."]
    pub fn set_custom_csr_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_csr_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_key`.\nThe key for a custom uploaded certificate."]
    pub fn set_custom_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `method`.\nDomain control validation (DCV) method used for this hostname.\nAvailable values: \"http\", \"txt\", \"email\"."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\nSSL specific settings."]
    pub fn set_settings(mut self, v: impl std::convert::Into<CustomHostname_Ssl_Settings>) -> Self {
        self.settings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nLevel of validation to be used for this hostname. Domain validation (dv) must be used.\nAvailable values: \"dv\"."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wildcard`.\nIndicates whether the certificate covers a wildcard."]
    pub fn set_wildcard(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.wildcard = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomHostname_Ssl {
    type O = terrars::BlockAssignable<CustomHostname_Ssl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostname_Ssl {}

impl BuildCustomHostname_Ssl {
    pub fn build(self) -> CustomHostname_Ssl {
        CustomHostname_Ssl {
            bundle_method: core::default::Default::default(),
            certificate_authority: core::default::Default::default(),
            cloudflare_branding: core::default::Default::default(),
            custom_cert_bundle: core::default::Default::default(),
            custom_certificate: core::default::Default::default(),
            custom_csr_id: core::default::Default::default(),
            custom_key: core::default::Default::default(),
            method: core::default::Default::default(),
            settings: core::default::Default::default(),
            type_: core::default::Default::default(),
            wildcard: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomHostname_SslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomHostname_SslRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomHostname_SslRef {
        CustomHostname_SslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostname_SslRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bundle_method` after provisioning.\nA ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it.\nAvailable values: \"ubiquitous\", \"optimal\", \"force\"."]
    pub fn bundle_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nThe Certificate Authority that will issue the certificate.\nAvailable values: \"digicert\", \"google\", \"lets_encrypt\", \"ssl_com\"."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cloudflare_branding` after provisioning.\nWhether or not to add Cloudflare Branding for the order.  This will add a subdomain of sni.cloudflaressl.com as the Common Name if set to true."]
    pub fn cloudflare_branding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudflare_branding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_cert_bundle` after provisioning.\nArray of custom certificate and key pairs (1 or 2 pairs allowed)."]
    pub fn custom_cert_bundle(&self) -> terrars::ListRef<CustomHostname_Ssl_CustomCertBundleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_cert_bundle", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_certificate` after provisioning.\nIf a custom uploaded certificate is used."]
    pub fn custom_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_csr_id` after provisioning.\nThe identifier for the Custom CSR that was used."]
    pub fn custom_csr_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_csr_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_key` after provisioning.\nThe key for a custom uploaded certificate."]
    pub fn custom_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nDomain control validation (DCV) method used for this hostname.\nAvailable values: \"http\", \"txt\", \"email\"."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\nSSL specific settings."]
    pub fn settings(&self) -> CustomHostname_Ssl_SettingsRef {
        CustomHostname_Ssl_SettingsRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nLevel of validation to be used for this hostname. Domain validation (dv) must be used.\nAvailable values: \"dv\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wildcard` after provisioning.\nIndicates whether the certificate covers a wildcard."]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }
}
