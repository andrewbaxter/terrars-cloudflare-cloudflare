#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataPipelineStreamsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataPipelineStreams_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPipelineStreamsData_>,
}

#[derive(Clone)]
pub struct DataPipelineStreams(std::rc::Rc<DataPipelineStreams_>);

impl DataPipelineStreams {
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

    #[doc = "Set the field `account_id`.\nSpecifies the public ID of the account."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nFilters streams by name (case-insensitive substring)."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_id`.\nSpecifies the public ID of the pipeline."]
    pub fn set_pipeline_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pipeline_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecifies the public ID of the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFilters streams by name (case-insensitive substring)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pipeline_id` after provisioning.\nSpecifies the public ID of the pipeline."]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataPipelineStreams_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataPipelineStreams {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPipelineStreams { }

impl terrars::ToListMappable for DataPipelineStreams {
    type O = terrars::ListRef<DataPipelineStreamsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPipelineStreams_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_pipeline_streams".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPipelineStreams {
    pub tf_id: std::string::String,
}

impl BuildDataPipelineStreams {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPipelineStreams {
        let out = DataPipelineStreams(std::rc::Rc::new(DataPipelineStreams_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPipelineStreamsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                name: core::default::Default::default(),
                pipeline_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPipelineStreamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineStreamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPipelineStreamsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecifies the public ID of the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFilters streams by name (case-insensitive substring)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pipeline_id` after provisioning.\nSpecifies the public ID of the pipeline."]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataPipelineStreams_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPipelineStreams_Result_Format {}

impl DataPipelineStreams_Result_Format { }

impl terrars::ToListMappable for DataPipelineStreams_Result_Format {
    type O = terrars::BlockAssignable<DataPipelineStreams_Result_Format>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineStreams_Result_Format {}

impl BuildDataPipelineStreams_Result_Format {
    pub fn build(self) -> DataPipelineStreams_Result_Format {
        DataPipelineStreams_Result_Format {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineStreams_Result_FormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineStreams_Result_FormatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineStreams_Result_FormatRef {
        DataPipelineStreams_Result_FormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineStreams_Result_FormatRef {
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
pub struct DataPipelineStreams_Result_Http_Cors {}

impl DataPipelineStreams_Result_Http_Cors { }

impl terrars::ToListMappable for DataPipelineStreams_Result_Http_Cors {
    type O = terrars::BlockAssignable<DataPipelineStreams_Result_Http_Cors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineStreams_Result_Http_Cors {}

impl BuildDataPipelineStreams_Result_Http_Cors {
    pub fn build(self) -> DataPipelineStreams_Result_Http_Cors {
        DataPipelineStreams_Result_Http_Cors {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineStreams_Result_Http_CorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineStreams_Result_Http_CorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineStreams_Result_Http_CorsRef {
        DataPipelineStreams_Result_Http_CorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineStreams_Result_Http_CorsRef {
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
pub struct DataPipelineStreams_Result_Http {}

impl DataPipelineStreams_Result_Http { }

impl terrars::ToListMappable for DataPipelineStreams_Result_Http {
    type O = terrars::BlockAssignable<DataPipelineStreams_Result_Http>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineStreams_Result_Http {}

impl BuildDataPipelineStreams_Result_Http {
    pub fn build(self) -> DataPipelineStreams_Result_Http {
        DataPipelineStreams_Result_Http {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineStreams_Result_HttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineStreams_Result_HttpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineStreams_Result_HttpRef {
        DataPipelineStreams_Result_HttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineStreams_Result_HttpRef {
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
    pub fn cors(&self) -> DataPipelineStreams_Result_Http_CorsRef {
        DataPipelineStreams_Result_Http_CorsRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicates that the HTTP endpoint is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelineStreams_Result_Schema_Fields {}

impl DataPipelineStreams_Result_Schema_Fields { }

impl terrars::ToListMappable for DataPipelineStreams_Result_Schema_Fields {
    type O = terrars::BlockAssignable<DataPipelineStreams_Result_Schema_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineStreams_Result_Schema_Fields {}

impl BuildDataPipelineStreams_Result_Schema_Fields {
    pub fn build(self) -> DataPipelineStreams_Result_Schema_Fields {
        DataPipelineStreams_Result_Schema_Fields {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineStreams_Result_Schema_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineStreams_Result_Schema_FieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineStreams_Result_Schema_FieldsRef {
        DataPipelineStreams_Result_Schema_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineStreams_Result_Schema_FieldsRef {
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
pub struct DataPipelineStreams_Result_Schema {}

impl DataPipelineStreams_Result_Schema { }

impl terrars::ToListMappable for DataPipelineStreams_Result_Schema {
    type O = terrars::BlockAssignable<DataPipelineStreams_Result_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineStreams_Result_Schema {}

impl BuildDataPipelineStreams_Result_Schema {
    pub fn build(self) -> DataPipelineStreams_Result_Schema {
        DataPipelineStreams_Result_Schema {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineStreams_Result_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineStreams_Result_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineStreams_Result_SchemaRef {
        DataPipelineStreams_Result_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineStreams_Result_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<DataPipelineStreams_Result_Schema_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }

    #[doc = "Get a reference to the value of field `inferred` after provisioning.\n"]
    pub fn inferred(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inferred", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelineStreams_Result_WorkerBinding {}

impl DataPipelineStreams_Result_WorkerBinding { }

impl terrars::ToListMappable for DataPipelineStreams_Result_WorkerBinding {
    type O = terrars::BlockAssignable<DataPipelineStreams_Result_WorkerBinding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineStreams_Result_WorkerBinding {}

impl BuildDataPipelineStreams_Result_WorkerBinding {
    pub fn build(self) -> DataPipelineStreams_Result_WorkerBinding {
        DataPipelineStreams_Result_WorkerBinding {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineStreams_Result_WorkerBindingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineStreams_Result_WorkerBindingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineStreams_Result_WorkerBindingRef {
        DataPipelineStreams_Result_WorkerBindingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineStreams_Result_WorkerBindingRef {
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

#[derive(Serialize)]
pub struct DataPipelineStreams_Result {}

impl DataPipelineStreams_Result { }

impl terrars::ToListMappable for DataPipelineStreams_Result {
    type O = terrars::BlockAssignable<DataPipelineStreams_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineStreams_Result {}

impl BuildDataPipelineStreams_Result {
    pub fn build(self) -> DataPipelineStreams_Result {
        DataPipelineStreams_Result {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineStreams_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineStreams_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineStreams_ResultRef {
        DataPipelineStreams_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineStreams_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nIndicates the endpoint URL of this stream."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nDefines the data format of the events."]
    pub fn format(&self) -> DataPipelineStreams_Result_FormatRef {
        DataPipelineStreams_Result_FormatRef::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> DataPipelineStreams_Result_HttpRef {
        DataPipelineStreams_Result_HttpRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIndicates a unique identifier for this stream."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIndicates the name of the Stream."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nDefines the schema of the events in the data stream."]
    pub fn schema(&self) -> DataPipelineStreams_Result_SchemaRef {
        DataPipelineStreams_Result_SchemaRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIndicates the current version of this stream."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_binding` after provisioning.\n"]
    pub fn worker_binding(&self) -> DataPipelineStreams_Result_WorkerBindingRef {
        DataPipelineStreams_Result_WorkerBindingRef::new(
            self.shared().clone(),
            format!("{}.worker_binding", self.base),
        )
    }
}
