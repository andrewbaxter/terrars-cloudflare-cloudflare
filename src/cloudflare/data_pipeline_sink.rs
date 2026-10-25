#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataPipelineSinkData_ {
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
    filter: std::option::Option<DataPipelineSink_Filter>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sink_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataPipelineSink_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPipelineSinkData_>,
}

#[derive(Clone)]
pub struct DataPipelineSink(std::rc::Rc<DataPipelineSink_>);

impl DataPipelineSink {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataPipelineSink_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sink_id`.\nSpecifies the publid ID of the sink."]
    pub fn set_sink_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sink_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecifies the public ID of the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nDefines the configuration of the R2 Sink."]
    pub fn config(&self) -> DataPipelineSink_ConfigRef {
        DataPipelineSink_ConfigRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataPipelineSink_FilterRef {
        DataPipelineSink_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nDefines the output data format of a sink."]
    pub fn format(&self) -> DataPipelineSink_FormatRef {
        DataPipelineSink_FormatRef::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSpecifies the publid ID of the sink."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDefines the name of the Sink."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nDefines the schema of the events in the data stream."]
    pub fn schema(&self) -> DataPipelineSink_SchemaRef {
        DataPipelineSink_SchemaRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sink_id` after provisioning.\nSpecifies the publid ID of the sink."]
    pub fn sink_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sink_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSpecifies the type of sink.\nAvailable values: \"r2\", \"r2_data_catalog\", \"basin_catalog\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataPipelineSink {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPipelineSink { }

impl terrars::ToListMappable for DataPipelineSink {
    type O = terrars::ListRef<DataPipelineSinkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPipelineSink_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_pipeline_sink".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPipelineSink {
    pub tf_id: std::string::String,
}

impl BuildDataPipelineSink {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPipelineSink {
        let out = DataPipelineSink(std::rc::Rc::new(DataPipelineSink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPipelineSinkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                sink_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPipelineSinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSinkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPipelineSinkRef {
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

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nDefines the configuration of the R2 Sink."]
    pub fn config(&self) -> DataPipelineSink_ConfigRef {
        DataPipelineSink_ConfigRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataPipelineSink_FilterRef {
        DataPipelineSink_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nDefines the output data format of a sink."]
    pub fn format(&self) -> DataPipelineSink_FormatRef {
        DataPipelineSink_FormatRef::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSpecifies the publid ID of the sink."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDefines the name of the Sink."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nDefines the schema of the events in the data stream."]
    pub fn schema(&self) -> DataPipelineSink_SchemaRef {
        DataPipelineSink_SchemaRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sink_id` after provisioning.\nSpecifies the publid ID of the sink."]
    pub fn sink_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sink_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSpecifies the type of sink.\nAvailable values: \"r2\", \"r2_data_catalog\", \"basin_catalog\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPipelineSink_Config_FileNaming {}

impl DataPipelineSink_Config_FileNaming { }

impl terrars::ToListMappable for DataPipelineSink_Config_FileNaming {
    type O = terrars::BlockAssignable<DataPipelineSink_Config_FileNaming>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSink_Config_FileNaming {}

impl BuildDataPipelineSink_Config_FileNaming {
    pub fn build(self) -> DataPipelineSink_Config_FileNaming {
        DataPipelineSink_Config_FileNaming {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSink_Config_FileNamingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSink_Config_FileNamingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSink_Config_FileNamingRef {
        DataPipelineSink_Config_FileNamingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSink_Config_FileNamingRef {
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
pub struct DataPipelineSink_Config_Partitioning {}

impl DataPipelineSink_Config_Partitioning { }

impl terrars::ToListMappable for DataPipelineSink_Config_Partitioning {
    type O = terrars::BlockAssignable<DataPipelineSink_Config_Partitioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSink_Config_Partitioning {}

impl BuildDataPipelineSink_Config_Partitioning {
    pub fn build(self) -> DataPipelineSink_Config_Partitioning {
        DataPipelineSink_Config_Partitioning {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSink_Config_PartitioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSink_Config_PartitioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSink_Config_PartitioningRef {
        DataPipelineSink_Config_PartitioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSink_Config_PartitioningRef {
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
pub struct DataPipelineSink_Config_RollingPolicy {}

impl DataPipelineSink_Config_RollingPolicy { }

impl terrars::ToListMappable for DataPipelineSink_Config_RollingPolicy {
    type O = terrars::BlockAssignable<DataPipelineSink_Config_RollingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSink_Config_RollingPolicy {}

impl BuildDataPipelineSink_Config_RollingPolicy {
    pub fn build(self) -> DataPipelineSink_Config_RollingPolicy {
        DataPipelineSink_Config_RollingPolicy {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSink_Config_RollingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSink_Config_RollingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSink_Config_RollingPolicyRef {
        DataPipelineSink_Config_RollingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSink_Config_RollingPolicyRef {
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
pub struct DataPipelineSink_Config {}

impl DataPipelineSink_Config { }

impl terrars::ToListMappable for DataPipelineSink_Config {
    type O = terrars::BlockAssignable<DataPipelineSink_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSink_Config {}

impl BuildDataPipelineSink_Config {
    pub fn build(self) -> DataPipelineSink_Config {
        DataPipelineSink_Config {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSink_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSink_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSink_ConfigRef {
        DataPipelineSink_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSink_ConfigRef {
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
    pub fn file_naming(&self) -> DataPipelineSink_Config_FileNamingRef {
        DataPipelineSink_Config_FileNamingRef::new(self.shared().clone(), format!("{}.file_naming", self.base))
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
    pub fn partitioning(&self) -> DataPipelineSink_Config_PartitioningRef {
        DataPipelineSink_Config_PartitioningRef::new(self.shared().clone(), format!("{}.partitioning", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nSubpath within the bucket to write to"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rolling_policy` after provisioning.\nRolling policy for file sinks (when & why to close a file and open a new one)."]
    pub fn rolling_policy(&self) -> DataPipelineSink_Config_RollingPolicyRef {
        DataPipelineSink_Config_RollingPolicyRef::new(self.shared().clone(), format!("{}.rolling_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\nTable name"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelineSink_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPipelineSink_Filter {
    #[doc = "Set the field `name`.\nFilters sinks by name (case-insensitive substring)."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_id`.\n"]
    pub fn set_pipeline_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pipeline_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPipelineSink_Filter {
    type O = terrars::BlockAssignable<DataPipelineSink_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSink_Filter {}

impl BuildDataPipelineSink_Filter {
    pub fn build(self) -> DataPipelineSink_Filter {
        DataPipelineSink_Filter {
            name: core::default::Default::default(),
            pipeline_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPipelineSink_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSink_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSink_FilterRef {
        DataPipelineSink_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSink_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFilters sinks by name (case-insensitive substring)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelineSink_Format {}

impl DataPipelineSink_Format { }

impl terrars::ToListMappable for DataPipelineSink_Format {
    type O = terrars::BlockAssignable<DataPipelineSink_Format>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSink_Format {}

impl BuildDataPipelineSink_Format {
    pub fn build(self) -> DataPipelineSink_Format {
        DataPipelineSink_Format {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSink_FormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSink_FormatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSink_FormatRef {
        DataPipelineSink_FormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSink_FormatRef {
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
pub struct DataPipelineSink_Schema_Fields {}

impl DataPipelineSink_Schema_Fields { }

impl terrars::ToListMappable for DataPipelineSink_Schema_Fields {
    type O = terrars::BlockAssignable<DataPipelineSink_Schema_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSink_Schema_Fields {}

impl BuildDataPipelineSink_Schema_Fields {
    pub fn build(self) -> DataPipelineSink_Schema_Fields {
        DataPipelineSink_Schema_Fields {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSink_Schema_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSink_Schema_FieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSink_Schema_FieldsRef {
        DataPipelineSink_Schema_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSink_Schema_FieldsRef {
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
pub struct DataPipelineSink_Schema {}

impl DataPipelineSink_Schema { }

impl terrars::ToListMappable for DataPipelineSink_Schema {
    type O = terrars::BlockAssignable<DataPipelineSink_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelineSink_Schema {}

impl BuildDataPipelineSink_Schema {
    pub fn build(self) -> DataPipelineSink_Schema {
        DataPipelineSink_Schema {}
    }
}

#[allow(dead_code)]
pub struct DataPipelineSink_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelineSink_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelineSink_SchemaRef {
        DataPipelineSink_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelineSink_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<DataPipelineSink_Schema_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }

    #[doc = "Get a reference to the value of field `inferred` after provisioning.\n"]
    pub fn inferred(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inferred", self.base))
    }
}
