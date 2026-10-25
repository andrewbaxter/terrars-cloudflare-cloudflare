#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ManagedTransformsData_ {
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
    managed_request_headers: std::option::Option<std::vec::Vec<ManagedTransforms_ManagedRequestHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_response_headers: std::option::Option<std::vec::Vec<ManagedTransforms_ManagedResponseHeaders>>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct ManagedTransforms_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ManagedTransformsData_>,
}

#[derive(Clone)]
pub struct ManagedTransforms(std::rc::Rc<ManagedTransforms_>);

impl ManagedTransforms {
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

    #[doc = "Set the field `managed_request_headers`.\nThe list of Managed Request Transforms."]
    pub fn set_managed_request_headers(
        self,
        v: impl std::convert::Into<std::vec::Vec<ManagedTransforms_ManagedRequestHeaders>>,
    ) -> Self {
        self.0.data.borrow_mut().managed_request_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_response_headers`.\nThe list of Managed Response Transforms."]
    pub fn set_managed_response_headers(
        self,
        v: impl std::convert::Into<std::vec::Vec<ManagedTransforms_ManagedResponseHeaders>>,
    ) -> Self {
        self.0.data.borrow_mut().managed_response_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the zone."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_request_headers` after provisioning.\nThe list of Managed Request Transforms."]
    pub fn managed_request_headers(&self) -> terrars::SetRef<ManagedTransforms_ManagedRequestHeadersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.managed_request_headers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_response_headers` after provisioning.\nThe list of Managed Response Transforms."]
    pub fn managed_response_headers(&self) -> terrars::SetRef<ManagedTransforms_ManagedResponseHeadersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.managed_response_headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nThe unique ID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for ManagedTransforms {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ManagedTransforms { }

impl terrars::ToListMappable for ManagedTransforms {
    type O = terrars::ListRef<ManagedTransformsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ManagedTransforms_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_managed_transforms".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildManagedTransforms {
    pub tf_id: std::string::String,
    #[doc = "The unique ID of the zone."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildManagedTransforms {
    pub fn build(self, stack: &mut terrars::Stack) -> ManagedTransforms {
        let out = ManagedTransforms(std::rc::Rc::new(ManagedTransforms_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ManagedTransformsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                managed_request_headers: core::default::Default::default(),
                managed_response_headers: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ManagedTransformsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedTransformsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ManagedTransformsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the zone."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_request_headers` after provisioning.\nThe list of Managed Request Transforms."]
    pub fn managed_request_headers(&self) -> terrars::SetRef<ManagedTransforms_ManagedRequestHeadersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.managed_request_headers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_response_headers` after provisioning.\nThe list of Managed Response Transforms."]
    pub fn managed_response_headers(&self) -> terrars::SetRef<ManagedTransforms_ManagedResponseHeadersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.managed_response_headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nThe unique ID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ManagedTransforms_ManagedRequestHeaders {
    enabled: terrars::PrimField<bool>,
    id: terrars::PrimField<std::string::String>,
}

impl ManagedTransforms_ManagedRequestHeaders { }

impl terrars::ToListMappable for ManagedTransforms_ManagedRequestHeaders {
    type O = terrars::BlockAssignable<ManagedTransforms_ManagedRequestHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedTransforms_ManagedRequestHeaders {
    #[doc = "Whether the Managed Transform is enabled."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The human-readable identifier of the Managed Transform."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildManagedTransforms_ManagedRequestHeaders {
    pub fn build(self) -> ManagedTransforms_ManagedRequestHeaders {
        ManagedTransforms_ManagedRequestHeaders {
            enabled: self.enabled,
            id: self.id,
        }
    }
}

#[allow(dead_code)]
pub struct ManagedTransforms_ManagedRequestHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedTransforms_ManagedRequestHeadersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedTransforms_ManagedRequestHeadersRef {
        ManagedTransforms_ManagedRequestHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedTransforms_ManagedRequestHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the Managed Transform is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe human-readable identifier of the Managed Transform."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ManagedTransforms_ManagedResponseHeaders {
    enabled: terrars::PrimField<bool>,
    id: terrars::PrimField<std::string::String>,
}

impl ManagedTransforms_ManagedResponseHeaders { }

impl terrars::ToListMappable for ManagedTransforms_ManagedResponseHeaders {
    type O = terrars::BlockAssignable<ManagedTransforms_ManagedResponseHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedTransforms_ManagedResponseHeaders {
    #[doc = "Whether the Managed Transform is enabled."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The human-readable identifier of the Managed Transform."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildManagedTransforms_ManagedResponseHeaders {
    pub fn build(self) -> ManagedTransforms_ManagedResponseHeaders {
        ManagedTransforms_ManagedResponseHeaders {
            enabled: self.enabled,
            id: self.id,
        }
    }
}

#[allow(dead_code)]
pub struct ManagedTransforms_ManagedResponseHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedTransforms_ManagedResponseHeadersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedTransforms_ManagedResponseHeadersRef {
        ManagedTransforms_ManagedResponseHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedTransforms_ManagedResponseHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the Managed Transform is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe human-readable identifier of the Managed Transform."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
