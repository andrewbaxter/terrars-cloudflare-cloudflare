#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataR2DataCatalogData_ {
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
    bucket_name: terrars::PrimField<std::string::String>,
}

struct DataR2DataCatalog_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataR2DataCatalogData_>,
}

#[derive(Clone)]
pub struct DataR2DataCatalog(std::rc::Rc<DataR2DataCatalog_>);

impl DataR2DataCatalog {
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

    #[doc = "Set the field `account_id`.\nUse this to identify the account."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nUse this to identify the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nSpecifies the associated R2 bucket name."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nSpecifies the R2 bucket name."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `credential_status` after provisioning.\nShows the credential configuration status.\nAvailable values: \"present\", \"absent\"."]
    pub fn credential_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSpecifies the R2 bucket name."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_config` after provisioning.\nConfigures maintenance for the catalog."]
    pub fn maintenance_config(&self) -> DataR2DataCatalog_MaintenanceConfigRef {
        DataR2DataCatalog_MaintenanceConfigRef::new(
            self.shared().clone(),
            format!("{}.maintenance_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSpecifies the catalog name (generated from account and bucket name)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nIndicates the status of the catalog.\nAvailable values: \"active\", \"inactive\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl terrars::Referable for DataR2DataCatalog {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataR2DataCatalog { }

impl terrars::ToListMappable for DataR2DataCatalog {
    type O = terrars::ListRef<DataR2DataCatalogRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataR2DataCatalog_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_r2_data_catalog".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataR2DataCatalog {
    pub tf_id: std::string::String,
    #[doc = "Specifies the R2 bucket name."]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildDataR2DataCatalog {
    pub fn build(self, stack: &mut terrars::Stack) -> DataR2DataCatalog {
        let out = DataR2DataCatalog(std::rc::Rc::new(DataR2DataCatalog_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataR2DataCatalogData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                bucket_name: self.bucket_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataR2DataCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2DataCatalogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataR2DataCatalogRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nUse this to identify the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nSpecifies the associated R2 bucket name."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nSpecifies the R2 bucket name."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `credential_status` after provisioning.\nShows the credential configuration status.\nAvailable values: \"present\", \"absent\"."]
    pub fn credential_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nSpecifies the R2 bucket name."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_config` after provisioning.\nConfigures maintenance for the catalog."]
    pub fn maintenance_config(&self) -> DataR2DataCatalog_MaintenanceConfigRef {
        DataR2DataCatalog_MaintenanceConfigRef::new(
            self.shared().clone(),
            format!("{}.maintenance_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSpecifies the catalog name (generated from account and bucket name)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nIndicates the status of the catalog.\nAvailable values: \"active\", \"inactive\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataR2DataCatalog_MaintenanceConfig_Compaction {}

impl DataR2DataCatalog_MaintenanceConfig_Compaction { }

impl terrars::ToListMappable for DataR2DataCatalog_MaintenanceConfig_Compaction {
    type O = terrars::BlockAssignable<DataR2DataCatalog_MaintenanceConfig_Compaction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2DataCatalog_MaintenanceConfig_Compaction {}

impl BuildDataR2DataCatalog_MaintenanceConfig_Compaction {
    pub fn build(self) -> DataR2DataCatalog_MaintenanceConfig_Compaction {
        DataR2DataCatalog_MaintenanceConfig_Compaction {}
    }
}

#[allow(dead_code)]
pub struct DataR2DataCatalog_MaintenanceConfig_CompactionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2DataCatalog_MaintenanceConfig_CompactionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataR2DataCatalog_MaintenanceConfig_CompactionRef {
        DataR2DataCatalog_MaintenanceConfig_CompactionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2DataCatalog_MaintenanceConfig_CompactionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nSpecifies the state of maintenance operations.\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_size_mb` after provisioning.\nSets the target file size for compaction in megabytes. Defaults to \"128\".\nAvailable values: \"64\", \"128\", \"256\", \"512\"."]
    pub fn target_size_mb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_size_mb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataR2DataCatalog_MaintenanceConfig_SnapshotExpiration {}

impl DataR2DataCatalog_MaintenanceConfig_SnapshotExpiration { }

impl terrars::ToListMappable for DataR2DataCatalog_MaintenanceConfig_SnapshotExpiration {
    type O = terrars::BlockAssignable<DataR2DataCatalog_MaintenanceConfig_SnapshotExpiration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2DataCatalog_MaintenanceConfig_SnapshotExpiration {}

impl BuildDataR2DataCatalog_MaintenanceConfig_SnapshotExpiration {
    pub fn build(self) -> DataR2DataCatalog_MaintenanceConfig_SnapshotExpiration {
        DataR2DataCatalog_MaintenanceConfig_SnapshotExpiration {}
    }
}

#[allow(dead_code)]
pub struct DataR2DataCatalog_MaintenanceConfig_SnapshotExpirationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2DataCatalog_MaintenanceConfig_SnapshotExpirationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataR2DataCatalog_MaintenanceConfig_SnapshotExpirationRef {
        DataR2DataCatalog_MaintenanceConfig_SnapshotExpirationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2DataCatalog_MaintenanceConfig_SnapshotExpirationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_snapshot_age` after provisioning.\nSpecifies the maximum age for snapshots. The system deletes snapshots older than this age.\nFormat: <number><unit> where unit is d (days), h (hours), m (minutes), or s (seconds).\nExamples: \"7d\" (7 days), \"48h\" (48 hours), \"2880m\" (2,880 minutes).\nDefaults to \"7d\"."]
    pub fn max_snapshot_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_snapshot_age", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_snapshots_to_keep` after provisioning.\nSpecifies the minimum number of snapshots to retain. Defaults to 100."]
    pub fn min_snapshots_to_keep(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_snapshots_to_keep", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nSpecifies the state of maintenance operations.\nAvailable values: \"enabled\", \"disabled\"."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataR2DataCatalog_MaintenanceConfig {}

impl DataR2DataCatalog_MaintenanceConfig { }

impl terrars::ToListMappable for DataR2DataCatalog_MaintenanceConfig {
    type O = terrars::BlockAssignable<DataR2DataCatalog_MaintenanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataR2DataCatalog_MaintenanceConfig {}

impl BuildDataR2DataCatalog_MaintenanceConfig {
    pub fn build(self) -> DataR2DataCatalog_MaintenanceConfig {
        DataR2DataCatalog_MaintenanceConfig {}
    }
}

#[allow(dead_code)]
pub struct DataR2DataCatalog_MaintenanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataR2DataCatalog_MaintenanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataR2DataCatalog_MaintenanceConfigRef {
        DataR2DataCatalog_MaintenanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataR2DataCatalog_MaintenanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compaction` after provisioning.\nConfigures compaction for catalog maintenance."]
    pub fn compaction(&self) -> DataR2DataCatalog_MaintenanceConfig_CompactionRef {
        DataR2DataCatalog_MaintenanceConfig_CompactionRef::new(
            self.shared().clone(),
            format!("{}.compaction", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nScheduling interval between normal table maintenance runs."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snapshot_expiration` after provisioning.\nConfigures snapshot expiration settings."]
    pub fn snapshot_expiration(&self) -> DataR2DataCatalog_MaintenanceConfig_SnapshotExpirationRef {
        DataR2DataCatalog_MaintenanceConfig_SnapshotExpirationRef::new(
            self.shared().clone(),
            format!("{}.snapshot_expiration", self.base),
        )
    }
}
