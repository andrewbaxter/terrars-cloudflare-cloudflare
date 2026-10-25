#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataManagedTransformsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataManagedTransforms_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataManagedTransformsData_>,
}

#[derive(Clone)]
pub struct DataManagedTransforms(std::rc::Rc<DataManagedTransforms_>);

impl DataManagedTransforms {
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

    #[doc = "Set the field `zone_id`.\nThe unique ID of the zone."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the zone."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_request_headers` after provisioning.\nThe list of Managed Request Transforms."]
    pub fn managed_request_headers(&self) -> terrars::ListRef<DataManagedTransforms_ManagedRequestHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_request_headers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_response_headers` after provisioning.\nThe list of Managed Response Transforms."]
    pub fn managed_response_headers(&self) -> terrars::ListRef<DataManagedTransforms_ManagedResponseHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_response_headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nThe unique ID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataManagedTransforms {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataManagedTransforms { }

impl terrars::ToListMappable for DataManagedTransforms {
    type O = terrars::ListRef<DataManagedTransformsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataManagedTransforms_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_managed_transforms".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataManagedTransforms {
    pub tf_id: std::string::String,
}

impl BuildDataManagedTransforms {
    pub fn build(self, stack: &mut terrars::Stack) -> DataManagedTransforms {
        let out = DataManagedTransforms(std::rc::Rc::new(DataManagedTransforms_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataManagedTransformsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataManagedTransformsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataManagedTransformsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataManagedTransformsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the zone."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_request_headers` after provisioning.\nThe list of Managed Request Transforms."]
    pub fn managed_request_headers(&self) -> terrars::ListRef<DataManagedTransforms_ManagedRequestHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_request_headers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_response_headers` after provisioning.\nThe list of Managed Response Transforms."]
    pub fn managed_response_headers(&self) -> terrars::ListRef<DataManagedTransforms_ManagedResponseHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_response_headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nThe unique ID of the zone."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataManagedTransforms_ManagedRequestHeaders {}

impl DataManagedTransforms_ManagedRequestHeaders { }

impl terrars::ToListMappable for DataManagedTransforms_ManagedRequestHeaders {
    type O = terrars::BlockAssignable<DataManagedTransforms_ManagedRequestHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataManagedTransforms_ManagedRequestHeaders {}

impl BuildDataManagedTransforms_ManagedRequestHeaders {
    pub fn build(self) -> DataManagedTransforms_ManagedRequestHeaders {
        DataManagedTransforms_ManagedRequestHeaders {}
    }
}

#[allow(dead_code)]
pub struct DataManagedTransforms_ManagedRequestHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataManagedTransforms_ManagedRequestHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataManagedTransforms_ManagedRequestHeadersRef {
        DataManagedTransforms_ManagedRequestHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataManagedTransforms_ManagedRequestHeadersRef {
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
pub struct DataManagedTransforms_ManagedResponseHeaders {}

impl DataManagedTransforms_ManagedResponseHeaders { }

impl terrars::ToListMappable for DataManagedTransforms_ManagedResponseHeaders {
    type O = terrars::BlockAssignable<DataManagedTransforms_ManagedResponseHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataManagedTransforms_ManagedResponseHeaders {}

impl BuildDataManagedTransforms_ManagedResponseHeaders {
    pub fn build(self) -> DataManagedTransforms_ManagedResponseHeaders {
        DataManagedTransforms_ManagedResponseHeaders {}
    }
}

#[allow(dead_code)]
pub struct DataManagedTransforms_ManagedResponseHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataManagedTransforms_ManagedResponseHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataManagedTransforms_ManagedResponseHeadersRef {
        DataManagedTransforms_ManagedResponseHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataManagedTransforms_ManagedResponseHeadersRef {
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
