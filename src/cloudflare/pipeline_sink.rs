#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct PipelineSinkData_ {
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
    config: std::option::Option<PipelineSink_Config>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<PipelineSink_Format>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<PipelineSink_Schema>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

struct PipelineSink_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PipelineSinkData_>,
}

#[derive(Clone)]
pub struct PipelineSink(std::rc::Rc<PipelineSink_>);

impl PipelineSink {
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

    #[doc = "Set the field `config`.\nDefines the configuration of the R2 Sink."]
    pub fn set_config(self, v: impl std::convert::Into<PipelineSink_Config>) -> Self {
        self.0.data.borrow_mut().config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `format`.\nDefines the output data format of a sink."]
    pub fn set_format(self, v: impl std::convert::Into<PipelineSink_Format>) -> Self {
        self.0.data.borrow_mut().format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\nDefines the schema of the events in the data stream."]
    pub fn set_schema(self, v: impl std::convert::Into<PipelineSink_Schema>) -> Self {
        self.0.data.borrow_mut().schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nSpecifies the public ID of the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nDefines the configuration of the R2 Sink."]
    pub fn config(&self) -> PipelineSink_ConfigRef {
        PipelineSink_ConfigRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nDefines the output data format of a sink."]
    pub fn format(&self) -> PipelineSink_FormatRef {
        PipelineSink_FormatRef::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIndicates a unique identifier for this sink."]
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
    pub fn schema(&self) -> PipelineSink_SchemaRef {
        PipelineSink_SchemaRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSpecifies the type of sink.\nAvailable values: \"r2\", \"r2_data_catalog\", \"basin_catalog\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for PipelineSink {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PipelineSink { }

impl terrars::ToListMappable for PipelineSink {
    type O = terrars::ListRef<PipelineSinkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PipelineSink_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_pipeline_sink".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPipelineSink {
    pub tf_id: std::string::String,
    #[doc = "Specifies the public ID of the account."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "Defines the name of the Sink."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Specifies the type of sink.\nAvailable values: \"r2\", \"r2_data_catalog\", \"basin_catalog\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildPipelineSink {
    pub fn build(self, stack: &mut terrars::Stack) -> PipelineSink {
        let out = PipelineSink(std::rc::Rc::new(PipelineSink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PipelineSinkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                config: core::default::Default::default(),
                format: core::default::Default::default(),
                name: self.name,
                schema: core::default::Default::default(),
                type_: self.type_,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PipelineSinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineSinkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PipelineSinkRef {
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

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nDefines the configuration of the R2 Sink."]
    pub fn config(&self) -> PipelineSink_ConfigRef {
        PipelineSink_ConfigRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nDefines the output data format of a sink."]
    pub fn format(&self) -> PipelineSink_FormatRef {
        PipelineSink_FormatRef::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIndicates a unique identifier for this sink."]
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
    pub fn schema(&self) -> PipelineSink_SchemaRef {
        PipelineSink_SchemaRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nSpecifies the type of sink.\nAvailable values: \"r2\", \"r2_data_catalog\", \"basin_catalog\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PipelineSink_Config_Credentials {
    access_key_id: terrars::PrimField<std::string::String>,
    secret_access_key: terrars::PrimField<std::string::String>,
}

impl PipelineSink_Config_Credentials { }

impl terrars::ToListMappable for PipelineSink_Config_Credentials {
    type O = terrars::BlockAssignable<PipelineSink_Config_Credentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineSink_Config_Credentials {
    #[doc = "Cloudflare Account ID for the bucket"]
    pub access_key_id: terrars::PrimField<std::string::String>,
    #[doc = "Cloudflare Account ID for the bucket"]
    pub secret_access_key: terrars::PrimField<std::string::String>,
}

impl BuildPipelineSink_Config_Credentials {
    pub fn build(self) -> PipelineSink_Config_Credentials {
        PipelineSink_Config_Credentials {
            access_key_id: self.access_key_id,
            secret_access_key: self.secret_access_key,
        }
    }
}

#[allow(dead_code)]
pub struct PipelineSink_Config_CredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineSink_Config_CredentialsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineSink_Config_CredentialsRef {
        PipelineSink_Config_CredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineSink_Config_CredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nCloudflare Account ID for the bucket"]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key` after provisioning.\nCloudflare Account ID for the bucket"]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }
}

#[derive(Serialize)]
pub struct PipelineSink_Config_FileNaming {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipelineSink_Config_FileNaming {
    #[doc = "Set the field `prefix`.\nThe prefix to use in file name. i.e prefix-<uuid>.parquet"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strategy`.\nFilename generation strategy.\nAvailable values: \"serial\", \"uuid\", \"uuid_v7\", \"ulid\"."]
    pub fn set_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suffix`.\nThis will overwrite the default file suffix. i.e .parquet, use with caution"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipelineSink_Config_FileNaming {
    type O = terrars::BlockAssignable<PipelineSink_Config_FileNaming>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineSink_Config_FileNaming {}

impl BuildPipelineSink_Config_FileNaming {
    pub fn build(self) -> PipelineSink_Config_FileNaming {
        PipelineSink_Config_FileNaming {
            prefix: core::default::Default::default(),
            strategy: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipelineSink_Config_FileNamingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineSink_Config_FileNamingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineSink_Config_FileNamingRef {
        PipelineSink_Config_FileNamingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineSink_Config_FileNamingRef {
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
pub struct PipelineSink_Config_Partitioning {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_pattern: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipelineSink_Config_Partitioning {
    #[doc = "Set the field `time_pattern`.\nThe pattern of the date string"]
    pub fn set_time_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_pattern = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipelineSink_Config_Partitioning {
    type O = terrars::BlockAssignable<PipelineSink_Config_Partitioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineSink_Config_Partitioning {}

impl BuildPipelineSink_Config_Partitioning {
    pub fn build(self) -> PipelineSink_Config_Partitioning {
        PipelineSink_Config_Partitioning { time_pattern: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct PipelineSink_Config_PartitioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineSink_Config_PartitioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineSink_Config_PartitioningRef {
        PipelineSink_Config_PartitioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineSink_Config_PartitioningRef {
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
pub struct PipelineSink_Config_RollingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_size_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inactivity_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl PipelineSink_Config_RollingPolicy {
    #[doc = "Set the field `file_size_bytes`.\nFiles will be rolled after reaching this number of bytes"]
    pub fn set_file_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.file_size_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `inactivity_seconds`.\nNumber of seconds of inactivity to wait before rolling over to a new file"]
    pub fn set_inactivity_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.inactivity_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_seconds`.\nNumber of seconds to wait before rolling over to a new file"]
    pub fn set_interval_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipelineSink_Config_RollingPolicy {
    type O = terrars::BlockAssignable<PipelineSink_Config_RollingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineSink_Config_RollingPolicy {}

impl BuildPipelineSink_Config_RollingPolicy {
    pub fn build(self) -> PipelineSink_Config_RollingPolicy {
        PipelineSink_Config_RollingPolicy {
            file_size_bytes: core::default::Default::default(),
            inactivity_seconds: core::default::Default::default(),
            interval_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipelineSink_Config_RollingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineSink_Config_RollingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineSink_Config_RollingPolicyRef {
        PipelineSink_Config_RollingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineSink_Config_RollingPolicyRef {
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
pub struct PipelineSink_Config {
    account_id: terrars::PrimField<std::string::String>,
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std::option::Option<PipelineSink_Config_Credentials>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_naming: std::option::Option<PipelineSink_Config_FileNaming>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jurisdiction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partitioning: std::option::Option<PipelineSink_Config_Partitioning>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rolling_policy: std::option::Option<PipelineSink_Config_RollingPolicy>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipelineSink_Config {
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(mut self, v: impl std::convert::Into<PipelineSink_Config_Credentials>) -> Self {
        self.credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_naming`.\nControls filename prefix/suffix and strategy."]
    pub fn set_file_naming(mut self, v: impl std::convert::Into<PipelineSink_Config_FileNaming>) -> Self {
        self.file_naming = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jurisdiction`.\nJurisdiction this bucket is hosted in"]
    pub fn set_jurisdiction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jurisdiction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\nTable namespace"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partitioning`.\nData-layout partitioning for sinks."]
    pub fn set_partitioning(mut self, v: impl std::convert::Into<PipelineSink_Config_Partitioning>) -> Self {
        self.partitioning = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nSubpath within the bucket to write to"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rolling_policy`.\nRolling policy for file sinks (when & why to close a file and open a new one)."]
    pub fn set_rolling_policy(mut self, v: impl std::convert::Into<PipelineSink_Config_RollingPolicy>) -> Self {
        self.rolling_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_name`.\nTable name"]
    pub fn set_table_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token`.\nAuthentication token"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipelineSink_Config {
    type O = terrars::BlockAssignable<PipelineSink_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineSink_Config {
    #[doc = "Cloudflare Account ID for the bucket"]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "R2 Bucket to write to"]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildPipelineSink_Config {
    pub fn build(self) -> PipelineSink_Config {
        PipelineSink_Config {
            account_id: self.account_id,
            bucket: self.bucket,
            credentials: core::default::Default::default(),
            file_naming: core::default::Default::default(),
            jurisdiction: core::default::Default::default(),
            namespace: core::default::Default::default(),
            partitioning: core::default::Default::default(),
            path: core::default::Default::default(),
            rolling_policy: core::default::Default::default(),
            table_name: core::default::Default::default(),
            token: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipelineSink_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineSink_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineSink_ConfigRef {
        PipelineSink_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineSink_ConfigRef {
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

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> PipelineSink_Config_CredentialsRef {
        PipelineSink_Config_CredentialsRef::new(self.shared().clone(), format!("{}.credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_naming` after provisioning.\nControls filename prefix/suffix and strategy."]
    pub fn file_naming(&self) -> PipelineSink_Config_FileNamingRef {
        PipelineSink_Config_FileNamingRef::new(self.shared().clone(), format!("{}.file_naming", self.base))
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
    pub fn partitioning(&self) -> PipelineSink_Config_PartitioningRef {
        PipelineSink_Config_PartitioningRef::new(self.shared().clone(), format!("{}.partitioning", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nSubpath within the bucket to write to"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rolling_policy` after provisioning.\nRolling policy for file sinks (when & why to close a file and open a new one)."]
    pub fn rolling_policy(&self) -> PipelineSink_Config_RollingPolicyRef {
        PipelineSink_Config_RollingPolicyRef::new(self.shared().clone(), format!("{}.rolling_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\nTable name"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\nAuthentication token"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct PipelineSink_Format {
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

impl PipelineSink_Format {
    #[doc =
        "Set the field `compression`.\nSpecifies the compression applied to JSON sink output.\nAvailable values: \"uncompressed\", \"gzip\", \"snappy\", \"zstd\", \"lz4\"."]
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

impl terrars::ToListMappable for PipelineSink_Format {
    type O = terrars::BlockAssignable<PipelineSink_Format>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineSink_Format {
    #[doc = "Available values: \"json\", \"parquet\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildPipelineSink_Format {
    pub fn build(self) -> PipelineSink_Format {
        PipelineSink_Format {
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
pub struct PipelineSink_FormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineSink_FormatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineSink_FormatRef {
        PipelineSink_FormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineSink_FormatRef {
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
pub struct PipelineSink_Schema_Fields {
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

impl PipelineSink_Schema_Fields {
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

impl terrars::ToListMappable for PipelineSink_Schema_Fields {
    type O = terrars::BlockAssignable<PipelineSink_Schema_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineSink_Schema_Fields {
    #[doc =
        "Available values: \"int32\", \"int64\", \"float32\", \"float64\", \"bool\", \"string\", \"binary\", \"timestamp\", \"json\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildPipelineSink_Schema_Fields {
    pub fn build(self) -> PipelineSink_Schema_Fields {
        PipelineSink_Schema_Fields {
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
pub struct PipelineSink_Schema_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineSink_Schema_FieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineSink_Schema_FieldsRef {
        PipelineSink_Schema_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineSink_Schema_FieldsRef {
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
pub struct PipelineSink_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<std::vec::Vec<PipelineSink_Schema_Fields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inferred: std::option::Option<terrars::PrimField<bool>>,
}

impl PipelineSink_Schema {
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(mut self, v: impl std::convert::Into<std::vec::Vec<PipelineSink_Schema_Fields>>) -> Self {
        self.fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inferred`.\n"]
    pub fn set_inferred(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inferred = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipelineSink_Schema {
    type O = terrars::BlockAssignable<PipelineSink_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipelineSink_Schema {}

impl BuildPipelineSink_Schema {
    pub fn build(self) -> PipelineSink_Schema {
        PipelineSink_Schema {
            fields: core::default::Default::default(),
            inferred: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipelineSink_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipelineSink_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipelineSink_SchemaRef {
        PipelineSink_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipelineSink_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<PipelineSink_Schema_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }

    #[doc = "Get a reference to the value of field `inferred` after provisioning.\n"]
    pub fn inferred(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inferred", self.base))
    }
}
