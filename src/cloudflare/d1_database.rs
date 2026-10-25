#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct D1DatabaseData_ {
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
    fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jurisdiction: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_location_hint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_replication: std::option::Option<D1Database_ReadReplication>,
}

struct D1Database_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<D1DatabaseData_>,
}

#[derive(Clone)]
pub struct D1Database(std::rc::Rc<D1Database_>);

impl D1Database {
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
        "Set the field `fields`.\nComma-separated list of fields to include in the response. When omitted,\nall fields are returned."]
    pub fn set_fields(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jurisdiction`.\nSpecify the location to restrict the D1 database to run and store data. If this option is present, the location hint is ignored.\nAvailable values: \"eu\", \"fedramp\", \"us\"."]
    pub fn set_jurisdiction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().jurisdiction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `primary_location_hint`.\nSpecify the region to create the D1 primary, if available. If this option is omitted, the D1 will be created as close as possible to the current user.\nAvailable values: \"wnam\", \"enam\", \"weur\", \"eeur\", \"apac\", \"oc\"."]
    pub fn set_primary_location_hint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().primary_location_hint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_replication`.\nConfiguration for D1 read replication."]
    pub fn set_read_replication(self, v: impl std::convert::Into<D1Database_ReadReplication>) -> Self {
        self.0.data.borrow_mut().read_replication = std::option::Option::Some(v.into());
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

    #[doc =
        "Get a reference to the value of field `fields` after provisioning.\nComma-separated list of fields to include in the response. When omitted,\nall fields are returned."]
    pub fn fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_size` after provisioning.\nThe D1 database's size, in bytes."]
    pub fn file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_size", self.extract_ref()))
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
        "Get a reference to the value of field `primary_location_hint` after provisioning.\nSpecify the region to create the D1 primary, if available. If this option is omitted, the D1 will be created as close as possible to the current user.\nAvailable values: \"wnam\", \"enam\", \"weur\", \"eeur\", \"apac\", \"oc\"."]
    pub fn primary_location_hint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_location_hint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_replication` after provisioning.\nConfiguration for D1 read replication."]
    pub fn read_replication(&self) -> D1Database_ReadReplicationRef {
        D1Database_ReadReplicationRef::new(self.shared().clone(), format!("{}.read_replication", self.extract_ref()))
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

impl terrars::Referable for D1Database {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for D1Database { }

impl terrars::ToListMappable for D1Database {
    type O = terrars::ListRef<D1DatabaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for D1Database_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_d1_database".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildD1Database {
    pub tf_id: std::string::String,
    #[doc = "Account identifier tag."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "D1 database name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildD1Database {
    pub fn build(self, stack: &mut terrars::Stack) -> D1Database {
        let out = D1Database(std::rc::Rc::new(D1Database_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(D1DatabaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                fields: core::default::Default::default(),
                jurisdiction: core::default::Default::default(),
                name: self.name,
                primary_location_hint: core::default::Default::default(),
                read_replication: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct D1DatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for D1DatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl D1DatabaseRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc =
        "Get a reference to the value of field `fields` after provisioning.\nComma-separated list of fields to include in the response. When omitted,\nall fields are returned."]
    pub fn fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_size` after provisioning.\nThe D1 database's size, in bytes."]
    pub fn file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_size", self.extract_ref()))
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
        "Get a reference to the value of field `primary_location_hint` after provisioning.\nSpecify the region to create the D1 primary, if available. If this option is omitted, the D1 will be created as close as possible to the current user.\nAvailable values: \"wnam\", \"enam\", \"weur\", \"eeur\", \"apac\", \"oc\"."]
    pub fn primary_location_hint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_location_hint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_replication` after provisioning.\nConfiguration for D1 read replication."]
    pub fn read_replication(&self) -> D1Database_ReadReplicationRef {
        D1Database_ReadReplicationRef::new(self.shared().clone(), format!("{}.read_replication", self.extract_ref()))
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
pub struct D1Database_ReadReplication {
    mode: terrars::PrimField<std::string::String>,
}

impl D1Database_ReadReplication { }

impl terrars::ToListMappable for D1Database_ReadReplication {
    type O = terrars::BlockAssignable<D1Database_ReadReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildD1Database_ReadReplication {
    #[doc =
        "The read replication mode for the database. Use 'auto' to create replicas and allow D1 automatically place them around the world, or 'disabled' to not use any database replicas (it can take a few hours for all replicas to be deleted).\nAvailable values: \"auto\", \"disabled\"."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildD1Database_ReadReplication {
    pub fn build(self) -> D1Database_ReadReplication {
        D1Database_ReadReplication { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct D1Database_ReadReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for D1Database_ReadReplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> D1Database_ReadReplicationRef {
        D1Database_ReadReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl D1Database_ReadReplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe read replication mode for the database. Use 'auto' to create replicas and allow D1 automatically place them around the world, or 'disabled' to not use any database replicas (it can take a few hours for all replicas to be deleted).\nAvailable values: \"auto\", \"disabled\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
