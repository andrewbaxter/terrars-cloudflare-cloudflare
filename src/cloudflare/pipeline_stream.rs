#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct PipelineStreamData_ {
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
    format: std::option::Option<PipelineStream_Format>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<PipelineStream_Http>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<PipelineStream_Schema>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_binding: std::option::Option<PipelineStream_WorkerBinding>,
}

struct PipelineStream_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PipelineStreamData_>,
}

#[derive(Clone)]
pub struct PipelineStream(std::rc::Rc<PipelineStream_>);

impl PipelineStream {
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

    #[doc = "Set the field `format`.\nDefines the data format of the events."]
    pub fn set_format(self, v: impl std::convert::Into<PipelineStream_Format>) -> Self {
        self.0.data.borrow_mut().format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(self, v: impl std::convert::Into<PipelineStream_Http>) -> Self {
        self.0.data.borrow_mut().http = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\nDefines the schema of the events in the data stream."]
    pub fn set_schema(self, v: impl std::convert::Into<PipelineStream_Schema>) -> Self {
        self.0.data.borrow_mut().schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_binding`.\n"]
    pub fn set_worker_binding(self, v: impl std::convert::Into<PipelineStream_WorkerBinding>) -> Self {
        self.0.data.borrow_mut().worker_binding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecifies the public ID of the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nIndicates the endpoint URL of this stream."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nDefines the data format of the events."]
    pub fn format(&self) -> PipelineStream_FormatRef {
        PipelineStream_FormatRef::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> PipelineStream_HttpRef {
        PipelineStream_HttpRef::new(self.shared().clone(), format!("{}.http", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIndicates a unique identifier for this stream."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecifies the name of the Stream."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nDefines the schema of the events in the data stream."]
    pub fn schema(&self) -> PipelineStream_SchemaRef {
        PipelineStream_SchemaRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIndicates the current version of this stream."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_binding` after provisioning.\n"]
    pub fn worker_binding(&self) -> PipelineStream_WorkerBindingRef {
        PipelineStream_WorkerBindingRef::new(self.shared().clone(), format!("{}.worker_binding", self.extract_ref()))
    }
}

impl terrars::Referable for PipelineStream {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PipelineStream { }

impl terrars::ToListMappable for PipelineStream {
    type O = terrars::ListRef<PipelineStreamRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PipelineStream_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_pipeline_stream".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPipelineStream {
    pub tf_id: std::string::String,
    #[doc = "Specifies the public ID of the account."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Specifies the name of the Stream."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildPipelineStream {
    pub fn build(self, stack: &mut terrars::Stack) -> PipelineStream {
        let out = PipelineStream(std::rc::Rc::new(PipelineStream_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PipelineStreamData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                format: core::default::Default::default(),
                http: core::default::Default::default(),
                name: self.name,
                schema: core::default::Default::default(),
                worker_binding: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PipelineStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineStreamRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PipelineStreamRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecifies the public ID of the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nIndicates the endpoint URL of this stream."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nDefines the data format of the events."]
    pub fn format(&self) -> PipelineStream_FormatRef {
        PipelineStream_FormatRef::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> PipelineStream_HttpRef {
        PipelineStream_HttpRef::new(self.shared().clone(), format!("{}.http", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIndicates a unique identifier for this stream."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecifies the name of the Stream."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nDefines the schema of the events in the data stream."]
    pub fn schema(&self) -> PipelineStream_SchemaRef {
        PipelineStream_SchemaRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIndicates the current version of this stream."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_binding` after provisioning.\n"]
    pub fn worker_binding(&self) -> PipelineStream_WorkerBindingRef {
        PipelineStream_WorkerBindingRef::new(self.shared().clone(), format!("{}.worker_binding", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PipelineStream_Format {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decimal_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_group_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unstructured: std::option::Option<terrars::PrimField<bool>>,
}

impl PipelineStream_Format {
    #[doc =
        "Set the field `compression`.\nAvailable values: \"uncompressed\", \"snappy\", \"gzip\", \"zstd\", \"lz4\"."]
    pub fn set_compression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `decimal_encoding`.\nAvailable values: \"number\", \"string\", \"bytes\"."]
    pub fn set_decimal_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.decimal_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `row_group_bytes`.\n"]
    pub fn set_row_group_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.row_group_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp_format`.\nAvailable values: \"rfc3339\", \"unix_millis\"."]
    pub fn set_timestamp_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timestamp_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unstructured`.\n"]
    pub fn set_unstructured(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unstructured = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipelineStream_Format {
    type O = terrars::BlockAssignable<PipelineStream_Format>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineStream_Format {
    #[doc = "Available values: \"json\", \"parquet\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildPipelineStream_Format {
    pub fn build(self) -> PipelineStream_Format {
        PipelineStream_Format {
            compression: core::default::Default::default(),
            decimal_encoding: core::default::Default::default(),
            row_group_bytes: core::default::Default::default(),
            timestamp_format: core::default::Default::default(),
            type_: self.type_,
            unstructured: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipelineStream_FormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineStream_FormatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineStream_FormatRef {
        PipelineStream_FormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineStream_FormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compression` after provisioning.\nAvailable values: \"uncompressed\", \"snappy\", \"gzip\", \"zstd\", \"lz4\"."]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `decimal_encoding` after provisioning.\nAvailable values: \"number\", \"string\", \"bytes\"."]
    pub fn decimal_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decimal_encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `row_group_bytes` after provisioning.\n"]
    pub fn row_group_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_group_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timestamp_format` after provisioning.\nAvailable values: \"rfc3339\", \"unix_millis\"."]
    pub fn timestamp_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"json\", \"parquet\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `unstructured` after provisioning.\n"]
    pub fn unstructured(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unstructured", self.base))
    }
}

#[derive(Serialize)]
pub struct PipelineStream_Http_Cors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl PipelineStream_Http_Cors {
    #[doc = "Set the field `origins`.\n"]
    pub fn set_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.origins = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipelineStream_Http_Cors {
    type O = terrars::BlockAssignable<PipelineStream_Http_Cors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineStream_Http_Cors {}

impl BuildPipelineStream_Http_Cors {
    pub fn build(self) -> PipelineStream_Http_Cors {
        PipelineStream_Http_Cors { origins: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct PipelineStream_Http_CorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineStream_Http_CorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineStream_Http_CorsRef {
        PipelineStream_Http_CorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineStream_Http_CorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `origins` after provisioning.\n"]
    pub fn origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origins", self.base))
    }
}

#[derive(Serialize)]
pub struct PipelineStream_Http {
    authentication: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<PipelineStream_Http_Cors>,
    enabled: terrars::PrimField<bool>,
}

impl PipelineStream_Http {
    #[doc = "Set the field `cors`.\nSpecifies the CORS options for the HTTP endpoint."]
    pub fn set_cors(mut self, v: impl std::convert::Into<PipelineStream_Http_Cors>) -> Self {
        self.cors = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipelineStream_Http {
    type O = terrars::BlockAssignable<PipelineStream_Http>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineStream_Http {
    #[doc = "Indicates that authentication is required for the HTTP endpoint."]
    pub authentication: terrars::PrimField<bool>,
    #[doc = "Indicates that the HTTP endpoint is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildPipelineStream_Http {
    pub fn build(self) -> PipelineStream_Http {
        PipelineStream_Http {
            authentication: self.authentication,
            cors: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct PipelineStream_HttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineStream_HttpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineStream_HttpRef {
        PipelineStream_HttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineStream_HttpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication` after provisioning.\nIndicates that authentication is required for the HTTP endpoint."]
    pub fn authentication(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cors` after provisioning.\nSpecifies the CORS options for the HTTP endpoint."]
    pub fn cors(&self) -> PipelineStream_Http_CorsRef {
        PipelineStream_Http_CorsRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicates that the HTTP endpoint is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct PipelineStream_Schema_Fields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipelineStream_Schema_Fields {
    #[doc = "Set the field `metadata_key`.\n"]
    pub fn set_metadata_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sql_name`.\n"]
    pub fn set_sql_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\nAvailable values: \"second\", \"millisecond\", \"microsecond\", \"nanosecond\"."]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipelineStream_Schema_Fields {
    type O = terrars::BlockAssignable<PipelineStream_Schema_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineStream_Schema_Fields {
    #[doc =
        "Available values: \"int32\", \"int64\", \"float32\", \"float64\", \"bool\", \"string\", \"binary\", \"timestamp\", \"json\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildPipelineStream_Schema_Fields {
    pub fn build(self) -> PipelineStream_Schema_Fields {
        PipelineStream_Schema_Fields {
            metadata_key: core::default::Default::default(),
            name: core::default::Default::default(),
            required: core::default::Default::default(),
            sql_name: core::default::Default::default(),
            type_: self.type_,
            unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipelineStream_Schema_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineStream_Schema_FieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineStream_Schema_FieldsRef {
        PipelineStream_Schema_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineStream_Schema_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_key` after provisioning.\n"]
    pub fn metadata_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_name` after provisioning.\n"]
    pub fn sql_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nAvailable values: \"int32\", \"int64\", \"float32\", \"float64\", \"bool\", \"string\", \"binary\", \"timestamp\", \"json\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unit` after provisioning.\nAvailable values: \"second\", \"millisecond\", \"microsecond\", \"nanosecond\"."]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct PipelineStream_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<std::vec::Vec<PipelineStream_Schema_Fields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inferred: std::option::Option<terrars::PrimField<bool>>,
}

impl PipelineStream_Schema {
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(mut self, v: impl std::convert::Into<std::vec::Vec<PipelineStream_Schema_Fields>>) -> Self {
        self.fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inferred`.\n"]
    pub fn set_inferred(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inferred = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipelineStream_Schema {
    type O = terrars::BlockAssignable<PipelineStream_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineStream_Schema {}

impl BuildPipelineStream_Schema {
    pub fn build(self) -> PipelineStream_Schema {
        PipelineStream_Schema {
            fields: core::default::Default::default(),
            inferred: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipelineStream_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineStream_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineStream_SchemaRef {
        PipelineStream_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineStream_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<PipelineStream_Schema_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }

    #[doc = "Get a reference to the value of field `inferred` after provisioning.\n"]
    pub fn inferred(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inferred", self.base))
    }
}

#[derive(Serialize)]
pub struct PipelineStream_WorkerBinding {
    enabled: terrars::PrimField<bool>,
}

impl PipelineStream_WorkerBinding { }

impl terrars::ToListMappable for PipelineStream_WorkerBinding {
    type O = terrars::BlockAssignable<PipelineStream_WorkerBinding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineStream_WorkerBinding {
    #[doc = "Indicates that the worker binding is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildPipelineStream_WorkerBinding {
    pub fn build(self) -> PipelineStream_WorkerBinding {
        PipelineStream_WorkerBinding { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct PipelineStream_WorkerBindingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineStream_WorkerBindingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineStream_WorkerBindingRef {
        PipelineStream_WorkerBindingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineStream_WorkerBindingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicates that the worker binding is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
