#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataPipelineSinksData_ {
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

struct DataPipelineSinks_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPipelineSinksData_>,
}

#[derive(Clone)]
pub struct DataPipelineSinks(std::rc::Rc<DataPipelineSinks_>);

impl DataPipelineSinks {
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

    #[doc = "Set the field `name`.\nFilters sinks by name (case-insensitive substring)."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_id`.\n"]
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
        "Get a reference to the value of field `name` after provisioning.\nFilters sinks by name (case-insensitive substring)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataPipelineSinks_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

impl terrars::Referable for DataPipelineSinks {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPipelineSinks { }

impl terrars::ToListMappable for DataPipelineSinks {
    type O = terrars::ListRef<DataPipelineSinksRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPipelineSinks_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_pipeline_sinks".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPipelineSinks {
    pub tf_id: std::string::String,
}

impl BuildDataPipelineSinks {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPipelineSinks {
        let out = DataPipelineSinks(std::rc::Rc::new(DataPipelineSinks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPipelineSinksData_ {
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

pub struct DataPipelineSinksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSinksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPipelineSinksRef {
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
        "Get a reference to the value of field `name` after provisioning.\nFilters sinks by name (case-insensitive substring)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataPipelineSinks_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPipelineSinks_Result_Config_FileNaming {}

impl DataPipelineSinks_Result_Config_FileNaming { }

impl terrars::ToListMappable for DataPipelineSinks_Result_Config_FileNaming {
    type O = terrars::BlockAssignable<DataPipelineSinks_Result_Config_FileNaming>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSinks_Result_Config_FileNaming {}

impl BuildDataPipelineSinks_Result_Config_FileNaming {
    pub fn build(self) -> DataPipelineSinks_Result_Config_FileNaming {
        DataPipelineSinks_Result_Config_FileNaming {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSinks_Result_Config_FileNamingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSinks_Result_Config_FileNamingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSinks_Result_Config_FileNamingRef {
        DataPipelineSinks_Result_Config_FileNamingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSinks_Result_Config_FileNamingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nThe prefix to use in file name. i.e prefix-<uuid>.parquet"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strategy` after provisioning.\nFilename generation strategy.\nAvailable values: \"serial\", \"uuid\", \"uuid_v7\", \"ulid\"."]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThis will overwrite the default file suffix. i.e .parquet, use with caution"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelineSinks_Result_Config_Partitioning {}

impl DataPipelineSinks_Result_Config_Partitioning { }

impl terrars::ToListMappable for DataPipelineSinks_Result_Config_Partitioning {
    type O = terrars::BlockAssignable<DataPipelineSinks_Result_Config_Partitioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSinks_Result_Config_Partitioning {}

impl BuildDataPipelineSinks_Result_Config_Partitioning {
    pub fn build(self) -> DataPipelineSinks_Result_Config_Partitioning {
        DataPipelineSinks_Result_Config_Partitioning {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSinks_Result_Config_PartitioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSinks_Result_Config_PartitioningRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPipelineSinks_Result_Config_PartitioningRef {
        DataPipelineSinks_Result_Config_PartitioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSinks_Result_Config_PartitioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `time_pattern` after provisioning.\nThe pattern of the date string"]
    pub fn time_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelineSinks_Result_Config_RollingPolicy {}

impl DataPipelineSinks_Result_Config_RollingPolicy { }

impl terrars::ToListMappable for DataPipelineSinks_Result_Config_RollingPolicy {
    type O = terrars::BlockAssignable<DataPipelineSinks_Result_Config_RollingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSinks_Result_Config_RollingPolicy {}

impl BuildDataPipelineSinks_Result_Config_RollingPolicy {
    pub fn build(self) -> DataPipelineSinks_Result_Config_RollingPolicy {
        DataPipelineSinks_Result_Config_RollingPolicy {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSinks_Result_Config_RollingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSinks_Result_Config_RollingPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPipelineSinks_Result_Config_RollingPolicyRef {
        DataPipelineSinks_Result_Config_RollingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSinks_Result_Config_RollingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_size_bytes` after provisioning.\nFiles will be rolled after reaching this number of bytes"]
    pub fn file_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_size_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inactivity_seconds` after provisioning.\nNumber of seconds of inactivity to wait before rolling over to a new file"]
    pub fn inactivity_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inactivity_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interval_seconds` after provisioning.\nNumber of seconds to wait before rolling over to a new file"]
    pub fn interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelineSinks_Result_Config {}

impl DataPipelineSinks_Result_Config { }

impl terrars::ToListMappable for DataPipelineSinks_Result_Config {
    type O = terrars::BlockAssignable<DataPipelineSinks_Result_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSinks_Result_Config {}

impl BuildDataPipelineSinks_Result_Config {
    pub fn build(self) -> DataPipelineSinks_Result_Config {
        DataPipelineSinks_Result_Config {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSinks_Result_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSinks_Result_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSinks_Result_ConfigRef {
        DataPipelineSinks_Result_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSinks_Result_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nCloudflare Account ID for the bucket"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nR2 Bucket to write to"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_naming` after provisioning.\nControls filename prefix/suffix and strategy."]
    pub fn file_naming(&self) -> DataPipelineSinks_Result_Config_FileNamingRef {
        DataPipelineSinks_Result_Config_FileNamingRef::new(
            self.shared().clone(),
            format!("{}.file_naming", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `jurisdiction` after provisioning.\nJurisdiction this bucket is hosted in"]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nTable namespace"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `partitioning` after provisioning.\nData-layout partitioning for sinks."]
    pub fn partitioning(&self) -> DataPipelineSinks_Result_Config_PartitioningRef {
        DataPipelineSinks_Result_Config_PartitioningRef::new(
            self.shared().clone(),
            format!("{}.partitioning", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nSubpath within the bucket to write to"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rolling_policy` after provisioning.\nRolling policy for file sinks (when & why to close a file and open a new one)."]
    pub fn rolling_policy(&self) -> DataPipelineSinks_Result_Config_RollingPolicyRef {
        DataPipelineSinks_Result_Config_RollingPolicyRef::new(
            self.shared().clone(),
            format!("{}.rolling_policy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\nTable name"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelineSinks_Result_Format {}

impl DataPipelineSinks_Result_Format { }

impl terrars::ToListMappable for DataPipelineSinks_Result_Format {
    type O = terrars::BlockAssignable<DataPipelineSinks_Result_Format>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSinks_Result_Format {}

impl BuildDataPipelineSinks_Result_Format {
    pub fn build(self) -> DataPipelineSinks_Result_Format {
        DataPipelineSinks_Result_Format {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSinks_Result_FormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSinks_Result_FormatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSinks_Result_FormatRef {
        DataPipelineSinks_Result_FormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSinks_Result_FormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compression` after provisioning.\nSpecifies the compression applied to JSON sink output.\nAvailable values: \"uncompressed\", \"gzip\", \"snappy\", \"zstd\", \"lz4\"."]
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
pub struct DataPipelineSinks_Result_Schema_Fields {}

impl DataPipelineSinks_Result_Schema_Fields { }

impl terrars::ToListMappable for DataPipelineSinks_Result_Schema_Fields {
    type O = terrars::BlockAssignable<DataPipelineSinks_Result_Schema_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSinks_Result_Schema_Fields {}

impl BuildDataPipelineSinks_Result_Schema_Fields {
    pub fn build(self) -> DataPipelineSinks_Result_Schema_Fields {
        DataPipelineSinks_Result_Schema_Fields {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSinks_Result_Schema_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSinks_Result_Schema_FieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSinks_Result_Schema_FieldsRef {
        DataPipelineSinks_Result_Schema_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSinks_Result_Schema_FieldsRef {
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
pub struct DataPipelineSinks_Result_Schema {}

impl DataPipelineSinks_Result_Schema { }

impl terrars::ToListMappable for DataPipelineSinks_Result_Schema {
    type O = terrars::BlockAssignable<DataPipelineSinks_Result_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSinks_Result_Schema {}

impl BuildDataPipelineSinks_Result_Schema {
    pub fn build(self) -> DataPipelineSinks_Result_Schema {
        DataPipelineSinks_Result_Schema {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSinks_Result_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSinks_Result_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSinks_Result_SchemaRef {
        DataPipelineSinks_Result_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSinks_Result_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<DataPipelineSinks_Result_Schema_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }

    #[doc = "Get a reference to the value of field `inferred` after provisioning.\n"]
    pub fn inferred(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inferred", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelineSinks_Result {}

impl DataPipelineSinks_Result { }

impl terrars::ToListMappable for DataPipelineSinks_Result {
    type O = terrars::BlockAssignable<DataPipelineSinks_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSinks_Result {}

impl BuildDataPipelineSinks_Result {
    pub fn build(self) -> DataPipelineSinks_Result {
        DataPipelineSinks_Result {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSinks_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSinks_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSinks_ResultRef {
        DataPipelineSinks_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSinks_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nDefines the configuration of the R2 Sink."]
    pub fn config(&self) -> DataPipelineSinks_Result_ConfigRef {
        DataPipelineSinks_Result_ConfigRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nDefines the output data format of a sink."]
    pub fn format(&self) -> DataPipelineSinks_Result_FormatRef {
        DataPipelineSinks_Result_FormatRef::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIndicates a unique identifier for this sink."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDefines the name of the Sink."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nDefines the schema of the events in the data stream."]
    pub fn schema(&self) -> DataPipelineSinks_Result_SchemaRef {
        DataPipelineSinks_Result_SchemaRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSpecifies the type of sink.\nAvailable values: \"r2\", \"r2_data_catalog\", \"basin_catalog\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
