#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataD1DatabaseData_ {
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
    database_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<DataD1Database_Filter>,
}

struct DataD1Database_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataD1DatabaseData_>,
}

#[derive(Clone)]
pub struct DataD1Database(std::rc::Rc<DataD1Database_>);

impl DataD1Database {
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

    #[doc = "Set the field `account_id`.\nAccount identifier tag."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_id`.\nD1 database identifier (UUID)."]
    pub fn set_database_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fields`.\nComma-separated list of fields to include in the response. When omitted,\nall fields are returned."]
    pub fn set_fields(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<DataD1Database_Filter>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nSpecifies the timestamp the resource was created as an ISO8601 string."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_id` after provisioning.\nD1 database identifier (UUID)."]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fields` after provisioning.\nComma-separated list of fields to include in the response. When omitted,\nall fields are returned."]
    pub fn fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_size` after provisioning.\nThe D1 database's size, in bytes."]
    pub fn file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataD1Database_FilterRef {
        DataD1Database_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nD1 database identifier (UUID)."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jurisdiction` after provisioning.\nSpecify the location to restrict the D1 database to run and store data. If this option is present, the location hint is ignored.\nAvailable values: \"eu\", \"fedramp\", \"us\"."]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nD1 database name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_tables` after provisioning.\nThe number of tables in the D1 database. This count is no longer accurate and should not be relied upon."]
    pub fn num_tables(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_tables", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_replication` after provisioning.\nConfiguration for D1 read replication."]
    pub fn read_replication(&self) -> DataD1Database_ReadReplicationRef {
        DataD1Database_ReadReplicationRef::new(
            self.shared().clone(),
            format!("{}.read_replication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nD1 database identifier (UUID)."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for DataD1Database {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataD1Database { }

impl terrars::ToListMappable for DataD1Database {
    type O = terrars::ListRef<DataD1DatabaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataD1Database_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_d1_database".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataD1Database {
    pub tf_id: std::string::String,
}

impl BuildDataD1Database {
    pub fn build(self, stack: &mut terrars::Stack) -> DataD1Database {
        let out = DataD1Database(std::rc::Rc::new(DataD1Database_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataD1DatabaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                database_id: core::default::Default::default(),
                fields: core::default::Default::default(),
                filter: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataD1DatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataD1DatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataD1DatabaseRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAccount identifier tag."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nSpecifies the timestamp the resource was created as an ISO8601 string."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_id` after provisioning.\nD1 database identifier (UUID)."]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fields` after provisioning.\nComma-separated list of fields to include in the response. When omitted,\nall fields are returned."]
    pub fn fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_size` after provisioning.\nThe D1 database's size, in bytes."]
    pub fn file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> DataD1Database_FilterRef {
        DataD1Database_FilterRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nD1 database identifier (UUID)."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jurisdiction` after provisioning.\nSpecify the location to restrict the D1 database to run and store data. If this option is present, the location hint is ignored.\nAvailable values: \"eu\", \"fedramp\", \"us\"."]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nD1 database name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_tables` after provisioning.\nThe number of tables in the D1 database. This count is no longer accurate and should not be relied upon."]
    pub fn num_tables(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_tables", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_replication` after provisioning.\nConfiguration for D1 read replication."]
    pub fn read_replication(&self) -> DataD1Database_ReadReplicationRef {
        DataD1Database_ReadReplicationRef::new(
            self.shared().clone(),
            format!("{}.read_replication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nD1 database identifier (UUID)."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataD1Database_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataD1Database_Filter {
    #[doc = "Set the field `name`.\na database name to search for."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataD1Database_Filter {
    type O = terrars::BlockAssignable<DataD1Database_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataD1Database_Filter {}

impl BuildDataD1Database_Filter {
    pub fn build(self) -> DataD1Database_Filter {
        DataD1Database_Filter { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataD1Database_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataD1Database_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataD1Database_FilterRef {
        DataD1Database_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataD1Database_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\na database name to search for."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataD1Database_ReadReplication {}

impl DataD1Database_ReadReplication { }

impl terrars::ToListMappable for DataD1Database_ReadReplication {
    type O = terrars::BlockAssignable<DataD1Database_ReadReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataD1Database_ReadReplication {}

impl BuildDataD1Database_ReadReplication {
    pub fn build(self) -> DataD1Database_ReadReplication {
        DataD1Database_ReadReplication {}
    }
}

#[allow(dead_code)]
pub struct DataD1Database_ReadReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataD1Database_ReadReplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataD1Database_ReadReplicationRef {
        DataD1Database_ReadReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataD1Database_ReadReplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe read replication mode for the database. Mode 'auto' denotes that D1 creates replicas and automatically places them around the world. Mode 'disabled' denotes that no database replicas are used.\nAvailable values: \"auto\", \"disabled\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
