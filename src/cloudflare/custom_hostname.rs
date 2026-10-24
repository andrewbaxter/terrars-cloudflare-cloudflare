use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct CustomHostnameData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_ssl_pending_validation: std::option::Option<terrars::PrimField<bool>>,
    zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<std::vec::Vec<CustomHostnameSslEl>>,
    dynamic: CustomHostnameDynamic,
}

struct CustomHostname_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CustomHostnameData>,
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
        "Set the field `custom_metadata`.\nCustom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly."]
    pub fn set_custom_metadata(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_origin_server`.\nThe custom origin server used for certificates."]
    pub fn set_custom_origin_server(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_origin_server = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_origin_sni`.\nThe [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates."]
    pub fn set_custom_origin_sni(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_origin_sni = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait_for_ssl_pending_validation`.\nWhether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`."]
    pub fn set_wait_for_ssl_pending_validation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_ssl_pending_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(self, v: impl std::convert::Into<terrars::BlockAssignable<CustomHostnameSslEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `custom_metadata` after provisioning.\nCustom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly."]
    pub fn custom_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_server` after provisioning.\nThe custom origin server used for certificates."]
    pub fn custom_origin_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_server", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_sni` after provisioning.\nThe [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates."]
    pub fn custom_origin_sni(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_sni", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ownership_verification` after provisioning.\n"]
    pub fn ownership_verification(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ownership_verification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ownership_verification_http` after provisioning.\n"]
    pub fn ownership_verification_http(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ownership_verification_http", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the certificate."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_ssl_pending_validation` after provisioning.\nWhether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`."]
    pub fn wait_for_ssl_pending_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_ssl_pending_validation", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> terrars::ListRef<CustomHostnameSslElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
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
    #[doc =
        "Hostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**"]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc =
        "The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildCustomHostname {
    pub fn build(self, stack: &mut terrars::Stack) -> CustomHostname {
        let out = CustomHostname(std::rc::Rc::new(CustomHostname_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CustomHostnameData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                custom_metadata: core::default::Default::default(),
                custom_origin_server: core::default::Default::default(),
                custom_origin_sni: core::default::Default::default(),
                hostname: self.hostname,
                id: core::default::Default::default(),
                wait_for_ssl_pending_validation: core::default::Default::default(),
                zone_id: self.zone_id,
                ssl: core::default::Default::default(),
                dynamic: core::default::Default::default(),
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
        "Get a reference to the value of field `custom_metadata` after provisioning.\nCustom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly."]
    pub fn custom_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_server` after provisioning.\nThe custom origin server used for certificates."]
    pub fn custom_origin_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_server", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_origin_sni` after provisioning.\nThe [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates."]
    pub fn custom_origin_sni(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_origin_sni", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ownership_verification` after provisioning.\n"]
    pub fn ownership_verification(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ownership_verification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ownership_verification_http` after provisioning.\n"]
    pub fn ownership_verification_http(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ownership_verification_http", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the certificate."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_ssl_pending_validation` after provisioning.\nWhether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`."]
    pub fn wait_for_ssl_pending_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_ssl_pending_validation", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `zone_id` after provisioning.\nThe zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> terrars::ListRef<CustomHostnameSslElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CustomHostnameSslElValidationErrorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomHostnameSslElValidationErrorsEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomHostnameSslElValidationErrorsEl {
    type O = terrars::BlockAssignable<CustomHostnameSslElValidationErrorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostnameSslElValidationErrorsEl {}

impl BuildCustomHostnameSslElValidationErrorsEl {
    pub fn build(self) -> CustomHostnameSslElValidationErrorsEl {
        CustomHostnameSslElValidationErrorsEl { message: core::default::Default::default() }
    }
}

pub struct CustomHostnameSslElValidationErrorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomHostnameSslElValidationErrorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomHostnameSslElValidationErrorsElRef {
        CustomHostnameSslElValidationErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostnameSslElValidationErrorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomHostnameSslElValidationRecordsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cname_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cname_target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    txt_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    txt_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomHostnameSslElValidationRecordsEl {
    #[doc = "Set the field `cname_name`.\n"]
    pub fn set_cname_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cname_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cname_target`.\n"]
    pub fn set_cname_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cname_target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `emails`.\n"]
    pub fn set_emails(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.emails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_body`.\n"]
    pub fn set_http_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_body = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_url`.\n"]
    pub fn set_http_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `txt_name`.\n"]
    pub fn set_txt_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.txt_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `txt_value`.\n"]
    pub fn set_txt_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.txt_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomHostnameSslElValidationRecordsEl {
    type O = terrars::BlockAssignable<CustomHostnameSslElValidationRecordsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostnameSslElValidationRecordsEl {}

impl BuildCustomHostnameSslElValidationRecordsEl {
    pub fn build(self) -> CustomHostnameSslElValidationRecordsEl {
        CustomHostnameSslElValidationRecordsEl {
            cname_name: core::default::Default::default(),
            cname_target: core::default::Default::default(),
            emails: core::default::Default::default(),
            http_body: core::default::Default::default(),
            http_url: core::default::Default::default(),
            txt_name: core::default::Default::default(),
            txt_value: core::default::Default::default(),
        }
    }
}

pub struct CustomHostnameSslElValidationRecordsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomHostnameSslElValidationRecordsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomHostnameSslElValidationRecordsElRef {
        CustomHostnameSslElValidationRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostnameSslElValidationRecordsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cname_name` after provisioning.\n"]
    pub fn cname_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname_name", self.base))
    }

    #[doc = "Get a reference to the value of field `cname_target` after provisioning.\n"]
    pub fn cname_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cname_target", self.base))
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc = "Get a reference to the value of field `http_body` after provisioning.\n"]
    pub fn http_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_body", self.base))
    }

    #[doc = "Get a reference to the value of field `http_url` after provisioning.\n"]
    pub fn http_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_url", self.base))
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
pub struct CustomHostnameSslElSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ciphers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    early_hints: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_tls_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls13: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomHostnameSslElSettingsEl {
    #[doc = "Set the field `ciphers`.\nList of SSL/TLS ciphers to associate with this certificate."]
    pub fn set_ciphers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ciphers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `early_hints`.\nWhether early hints should be supported. Available values: `on`, `off`."]
    pub fn set_early_hints(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.early_hints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http2`.\nWhether HTTP2 should be supported. Available values: `on`, `off`."]
    pub fn set_http2(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http2 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_tls_version`.\nLowest version of TLS this certificate should support. Available values: `1.0`, `1.1`, `1.2`, `1.3`."]
    pub fn set_min_tls_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_tls_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls13`.\nWhether TLSv1.3 should be supported. Available values: `on`, `off`."]
    pub fn set_tls13(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls13 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomHostnameSslElSettingsEl {
    type O = terrars::BlockAssignable<CustomHostnameSslElSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostnameSslElSettingsEl {}

impl BuildCustomHostnameSslElSettingsEl {
    pub fn build(self) -> CustomHostnameSslElSettingsEl {
        CustomHostnameSslElSettingsEl {
            ciphers: core::default::Default::default(),
            early_hints: core::default::Default::default(),
            http2: core::default::Default::default(),
            min_tls_version: core::default::Default::default(),
            tls13: core::default::Default::default(),
        }
    }
}

pub struct CustomHostnameSslElSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomHostnameSslElSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomHostnameSslElSettingsElRef {
        CustomHostnameSslElSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostnameSslElSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ciphers` after provisioning.\nList of SSL/TLS ciphers to associate with this certificate."]
    pub fn ciphers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ciphers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `early_hints` after provisioning.\nWhether early hints should be supported. Available values: `on`, `off`."]
    pub fn early_hints(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.early_hints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http2` after provisioning.\nWhether HTTP2 should be supported. Available values: `on`, `off`."]
    pub fn http2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_tls_version` after provisioning.\nLowest version of TLS this certificate should support. Available values: `1.0`, `1.1`, `1.2`, `1.3`."]
    pub fn min_tls_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_tls_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls13` after provisioning.\nWhether TLSv1.3 should be supported. Available values: `on`, `off`."]
    pub fn tls13(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls13", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomHostnameSslElDynamic {
    settings: std::option::Option<terrars::DynamicBlock<CustomHostnameSslElSettingsEl>>,
}

#[derive(Serialize)]
pub struct CustomHostnameSslEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<CustomHostnameSslElSettingsEl>>,
    dynamic: CustomHostnameSslElDynamic,
}

impl CustomHostnameSslEl {
    #[doc = "Set the field `certificate_authority`.\n"]
    pub fn set_certificate_authority(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.certificate_authority = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `custom_key`.\nThe key for a custom uploaded certificate."]
    pub fn set_custom_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `method`.\nDomain control validation (DCV) method used for this hostname. Available values: `http`, `txt`, `email`."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nLevel of validation to be used for this hostname. Available values: `dv`. Defaults to `dv`."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wildcard`.\nIndicates whether the certificate covers a wildcard."]
    pub fn set_wildcard(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.wildcard = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomHostnameSslElSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CustomHostnameSslEl {
    type O = terrars::BlockAssignable<CustomHostnameSslEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomHostnameSslEl {}

impl BuildCustomHostnameSslEl {
    pub fn build(self) -> CustomHostnameSslEl {
        CustomHostnameSslEl {
            certificate_authority: core::default::Default::default(),
            custom_certificate: core::default::Default::default(),
            custom_key: core::default::Default::default(),
            method: core::default::Default::default(),
            type_: core::default::Default::default(),
            wildcard: core::default::Default::default(),
            settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CustomHostnameSslElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomHostnameSslElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomHostnameSslElRef {
        CustomHostnameSslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomHostnameSslElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.base))
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

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nDomain control validation (DCV) method used for this hostname. Available values: `http`, `txt`, `email`."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nLevel of validation to be used for this hostname. Available values: `dv`. Defaults to `dv`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `validation_errors` after provisioning.\n"]
    pub fn validation_errors(&self) -> terrars::ListRef<CustomHostnameSslElValidationErrorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `validation_records` after provisioning.\n"]
    pub fn validation_records(&self) -> terrars::ListRef<CustomHostnameSslElValidationRecordsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_records", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wildcard` after provisioning.\nIndicates whether the certificate covers a wildcard."]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<CustomHostnameSslElSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomHostnameDynamic {
    ssl: std::option::Option<terrars::DynamicBlock<CustomHostnameSslEl>>,
}
