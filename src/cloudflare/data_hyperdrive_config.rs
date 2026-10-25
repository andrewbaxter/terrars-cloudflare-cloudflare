#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataHyperdriveConfigData_ {
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
    hyperdrive_id: terrars::PrimField<std::string::String>,
}

struct DataHyperdriveConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataHyperdriveConfigData_>,
}

#[derive(Clone)]
pub struct DataHyperdriveConfig(std::rc::Rc<DataHyperdriveConfig_>);

impl DataHyperdriveConfig {
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

    #[doc = "Set the field `account_id`.\nDefine configurations using a unique string identifier."]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nDefine configurations using a unique string identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `caching` after provisioning.\n"]
    pub fn caching(&self) -> DataHyperdriveConfig_CachingRef {
        DataHyperdriveConfig_CachingRef::new(self.shared().clone(), format!("{}.caching", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nDefines the creation time of the Hyperdrive configuration."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hyperdrive_id` after provisioning.\nDefine configurations using a unique string identifier."]
    pub fn hyperdrive_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hyperdrive_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nDefine configurations using a unique string identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integration` after provisioning.\nSelects a PlanetScale database and provisions credentials managed by Cloudflare when creating a Hyperdrive configuration. The Cloudflare account must already be linked to PlanetScale in the Hyperdrive dashboard. The selection is returned as read-only metadata and cannot be changed with replace or update operations."]
    pub fn integration(&self) -> DataHyperdriveConfig_IntegrationRef {
        DataHyperdriveConfig_IntegrationRef::new(
            self.shared().clone(),
            format!("{}.integration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nDefines the last modified time of the Hyperdrive configuration."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mtls` after provisioning.\nmTLS configuration for the origin connection. Cannot be used with VPC Service origins; TLS must be managed on the VPC Service."]
    pub fn mtls(&self) -> DataHyperdriveConfig_MtlsRef {
        DataHyperdriveConfig_MtlsRef::new(self.shared().clone(), format!("{}.mtls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Hyperdrive configuration. Used to identify the configuration in the Cloudflare dashboard and API."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nCombines database connection fields with exactly one supported network location."]
    pub fn origin(&self) -> DataHyperdriveConfig_OriginRef {
        DataHyperdriveConfig_OriginRef::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_connection_limit` after provisioning.\nThe (soft) maximum number of connections the Hyperdrive is allowed to make to the origin database.\n\nMaximum allowed: 20 for free tier accounts, 100 for paid tier accounts.\nIf not specified, defaults to 20 for free tier and 60 for paid tier.\nCertain Cloudflare-managed origins may be permitted a higher limit.\nContact Cloudflare if you need a higher limit."]
    pub fn origin_connection_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_connection_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restarted_on` after provisioning.\nDefines the last time the Hyperdrive connection pool was explicitly restarted via the restart endpoint. Omitted if the pool has never been explicitly restarted."]
    pub fn restarted_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restarted_on", self.extract_ref()))
    }
}

impl terrars::Referable for DataHyperdriveConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataHyperdriveConfig { }

impl terrars::ToListMappable for DataHyperdriveConfig {
    type O = terrars::ListRef<DataHyperdriveConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataHyperdriveConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_hyperdrive_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataHyperdriveConfig {
    pub tf_id: std::string::String,
    #[doc = "Define configurations using a unique string identifier."]
    pub hyperdrive_id: terrars::PrimField<std::string::String>,
}

impl BuildDataHyperdriveConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataHyperdriveConfig {
        let out = DataHyperdriveConfig(std::rc::Rc::new(DataHyperdriveConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataHyperdriveConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                hyperdrive_id: self.hyperdrive_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataHyperdriveConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataHyperdriveConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataHyperdriveConfigRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nDefine configurations using a unique string identifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `caching` after provisioning.\n"]
    pub fn caching(&self) -> DataHyperdriveConfig_CachingRef {
        DataHyperdriveConfig_CachingRef::new(self.shared().clone(), format!("{}.caching", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_on` after provisioning.\nDefines the creation time of the Hyperdrive configuration."]
    pub fn created_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hyperdrive_id` after provisioning.\nDefine configurations using a unique string identifier."]
    pub fn hyperdrive_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hyperdrive_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nDefine configurations using a unique string identifier."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integration` after provisioning.\nSelects a PlanetScale database and provisions credentials managed by Cloudflare when creating a Hyperdrive configuration. The Cloudflare account must already be linked to PlanetScale in the Hyperdrive dashboard. The selection is returned as read-only metadata and cannot be changed with replace or update operations."]
    pub fn integration(&self) -> DataHyperdriveConfig_IntegrationRef {
        DataHyperdriveConfig_IntegrationRef::new(
            self.shared().clone(),
            format!("{}.integration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `modified_on` after provisioning.\nDefines the last modified time of the Hyperdrive configuration."]
    pub fn modified_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_on", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mtls` after provisioning.\nmTLS configuration for the origin connection. Cannot be used with VPC Service origins; TLS must be managed on the VPC Service."]
    pub fn mtls(&self) -> DataHyperdriveConfig_MtlsRef {
        DataHyperdriveConfig_MtlsRef::new(self.shared().clone(), format!("{}.mtls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Hyperdrive configuration. Used to identify the configuration in the Cloudflare dashboard and API."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin` after provisioning.\nCombines database connection fields with exactly one supported network location."]
    pub fn origin(&self) -> DataHyperdriveConfig_OriginRef {
        DataHyperdriveConfig_OriginRef::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_connection_limit` after provisioning.\nThe (soft) maximum number of connections the Hyperdrive is allowed to make to the origin database.\n\nMaximum allowed: 20 for free tier accounts, 100 for paid tier accounts.\nIf not specified, defaults to 20 for free tier and 60 for paid tier.\nCertain Cloudflare-managed origins may be permitted a higher limit.\nContact Cloudflare if you need a higher limit."]
    pub fn origin_connection_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_connection_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restarted_on` after provisioning.\nDefines the last time the Hyperdrive connection pool was explicitly restarted via the restart endpoint. Omitted if the pool has never been explicitly restarted."]
    pub fn restarted_on(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restarted_on", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataHyperdriveConfig_Caching {}

impl DataHyperdriveConfig_Caching { }

impl terrars::ToListMappable for DataHyperdriveConfig_Caching {
    type O = terrars::BlockAssignable<DataHyperdriveConfig_Caching>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataHyperdriveConfig_Caching {}

impl BuildDataHyperdriveConfig_Caching {
    pub fn build(self) -> DataHyperdriveConfig_Caching {
        DataHyperdriveConfig_Caching {}
    }
}

#[allow(dead_code)]
pub struct DataHyperdriveConfig_CachingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataHyperdriveConfig_CachingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataHyperdriveConfig_CachingRef {
        DataHyperdriveConfig_CachingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataHyperdriveConfig_CachingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDefines whether caching is disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nDefines the maximum duration (in seconds) items persist in the cache."]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stale_while_revalidate` after provisioning.\nDefines the number of seconds the cache may serve a stale response."]
    pub fn stale_while_revalidate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stale_while_revalidate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataHyperdriveConfig_Integration {}

impl DataHyperdriveConfig_Integration { }

impl terrars::ToListMappable for DataHyperdriveConfig_Integration {
    type O = terrars::BlockAssignable<DataHyperdriveConfig_Integration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataHyperdriveConfig_Integration {}

impl BuildDataHyperdriveConfig_Integration {
    pub fn build(self) -> DataHyperdriveConfig_Integration {
        DataHyperdriveConfig_Integration {}
    }
}

#[allow(dead_code)]
pub struct DataHyperdriveConfig_IntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataHyperdriveConfig_IntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataHyperdriveConfig_IntegrationRef {
        DataHyperdriveConfig_IntegrationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataHyperdriveConfig_IntegrationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `custom_database_name` after provisioning.\nThe database name to use when connecting. Defaults to `postgres` for PostgreSQL and `mysql` for MySQL."]
    pub fn custom_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_database_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_branch_name` after provisioning.\nThe name of the PlanetScale database branch."]
    pub fn database_branch_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_branch_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_name` after provisioning.\nThe name of the PlanetScale database."]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hyperdrive_config_provider` after provisioning.\nThe database integration provider used by this operation.\nAvailable values: \"planetscale\"."]
    pub fn hyperdrive_config_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hyperdrive_config_provider", self.base))
    }

    #[doc =
        "Get a reference to the value of field `organization_name` after provisioning.\nThe name of the PlanetScale organization."]
    pub fn organization_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheme` after provisioning.\nSpecifies the URL scheme used to connect to your origin database.\nAvailable values: \"postgres\", \"postgresql\", \"mysql\"."]
    pub fn scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
}

#[derive(Serialize)]
pub struct DataHyperdriveConfig_Mtls {}

impl DataHyperdriveConfig_Mtls { }

impl terrars::ToListMappable for DataHyperdriveConfig_Mtls {
    type O = terrars::BlockAssignable<DataHyperdriveConfig_Mtls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataHyperdriveConfig_Mtls {}

impl BuildDataHyperdriveConfig_Mtls {
    pub fn build(self) -> DataHyperdriveConfig_Mtls {
        DataHyperdriveConfig_Mtls {}
    }
}

#[allow(dead_code)]
pub struct DataHyperdriveConfig_MtlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataHyperdriveConfig_MtlsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataHyperdriveConfig_MtlsRef {
        DataHyperdriveConfig_MtlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataHyperdriveConfig_MtlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate_id` after provisioning.\nDefine CA certificate ID obtained after uploading CA cert."]
    pub fn ca_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mtls_certificate_id` after provisioning.\nDefine mTLS certificate ID obtained after uploading client cert."]
    pub fn mtls_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtls_certificate_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sslmode` after provisioning.\nPostgreSQL accepts `require`, `verify-ca`, and `verify-full`. MySQL accepts `REQUIRED`, `VERIFY_CA`, and `VERIFY_IDENTITY`. The verify modes require a CA certificate; the require modes cannot be used with a CA certificate."]
    pub fn sslmode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sslmode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataHyperdriveConfig_Origin {}

impl DataHyperdriveConfig_Origin { }

impl terrars::ToListMappable for DataHyperdriveConfig_Origin {
    type O = terrars::BlockAssignable<DataHyperdriveConfig_Origin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataHyperdriveConfig_Origin {}

impl BuildDataHyperdriveConfig_Origin {
    pub fn build(self) -> DataHyperdriveConfig_Origin {
        DataHyperdriveConfig_Origin {}
    }
}

#[allow(dead_code)]
pub struct DataHyperdriveConfig_OriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataHyperdriveConfig_OriginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataHyperdriveConfig_OriginRef {
        DataHyperdriveConfig_OriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataHyperdriveConfig_OriginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_client_id` after provisioning.\nDefines the Client ID of the Access token to use when connecting to the origin database."]
    pub fn access_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `access_client_secret` after provisioning.\nDefines the Client Secret of the Access Token to use when connecting to the origin database. The API never returns this write-only value."]
    pub fn access_client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nSet the name of your origin database."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nDefines the publicly reachable hostname or IP of your origin database. Private, loopback, and link-local IP addresses are not allowed."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nSet the password needed to access your origin database. The API never returns this write-only value."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nDefines the port of your origin database. Defaults to 5432 for PostgreSQL or 3306 for MySQL if not specified."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheme` after provisioning.\nSpecifies the URL scheme used to connect to your origin database.\nAvailable values: \"postgres\", \"postgresql\", \"mysql\"."]
    pub fn scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_id` after provisioning.\nThe identifier of the Workers VPC Service to connect through. Hyperdrive will egress through the specified VPC Service to reach the origin database."]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nSet the user of your origin database."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}
