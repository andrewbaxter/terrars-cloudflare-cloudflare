#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustAccessAiControlsMcpServersData_ {
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
    search: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataZeroTrustAccessAiControlsMcpServers_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustAccessAiControlsMcpServersData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustAccessAiControlsMcpServers(std::rc::Rc<DataZeroTrustAccessAiControlsMcpServers_>);

impl DataZeroTrustAccessAiControlsMcpServers {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nSearch by id, name"]
    pub fn set_search(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustAccessAiControlsMcpServers_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\nSearch by id, name"]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustAccessAiControlsMcpServers {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustAccessAiControlsMcpServers { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServers {
    type O = terrars::ListRef<DataZeroTrustAccessAiControlsMcpServersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustAccessAiControlsMcpServers_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_access_ai_controls_mcp_servers".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServers {
    pub tf_id: std::string::String,
}

impl BuildDataZeroTrustAccessAiControlsMcpServers {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustAccessAiControlsMcpServers {
        let out = DataZeroTrustAccessAiControlsMcpServers(std::rc::Rc::new(DataZeroTrustAccessAiControlsMcpServers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustAccessAiControlsMcpServersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                max_items: core::default::Default::default(),
                search: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustAccessAiControlsMcpServersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataZeroTrustAccessAiControlsMcpServers_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\nSearch by id, name"]
    pub fn search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_Config {}

impl DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_Config { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_Config {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_Config {}

impl BuildDataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_Config {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_Config {
        DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_Config {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_ConfigRef {
        DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `revocation_endpoint` after provisioning.\n"]
    pub fn revocation_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revocation_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfo {}

impl DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfo { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfo {
    type O =
        terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfo {}

impl BuildDataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfo {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfo {
        DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfo {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfoRef {
        DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_uris` after provisioning.\n"]
    pub fn redirect_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_uris", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint_auth_method` after provisioning.\n"]
    pub fn token_endpoint_auth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint_auth_method", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary {}

impl DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary {}

impl BuildDataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary {
        DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummaryRef {
        DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_mode` after provisioning.\nAvailable values: \"dcr\", \"manual\"."]
    pub fn auth_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_version` after provisioning.\n"]
    pub fn client_secret_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_ConfigRef {
        DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_ConfigRef::new(
            self.shared().clone(),
            format!("{}.config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `has_client_secret` after provisioning.\n"]
    pub fn has_client_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `registration_info` after provisioning.\n"]
    pub fn registration_info(
        &self,
    ) -> DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfoRef {
        DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummary_RegistrationInfoRef::new(
            self.shared().clone(),
            format!("{}.registration_info", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetails {}

impl DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetails { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetails {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetails {}

impl BuildDataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetails {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetails {
        DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetails {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetailsRef {
        DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cause` after provisioning.\nUnderlying error message"]
    pub fn cause(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cause", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_upstream` after provisioning.\nTrue = MCP server returned an error. False = couldn't reach the server"]
    pub fn is_upstream(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_upstream", self.base))
    }

    #[doc = "Get a reference to the value of field `mcp_code` after provisioning.\nMCP protocol error code"]
    pub fn mcp_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mcp_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retryable` after provisioning.\nWhether the error is transient and worth retrying"]
    pub fn retryable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retryable", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\nHTTP status code from the server"]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPrompts {}

impl DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPrompts { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPrompts {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPrompts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPrompts {}

impl BuildDataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPrompts {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPrompts {
        DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPrompts {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPromptsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPromptsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPromptsRef {
        DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPromptsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPromptsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alias` after provisioning.\nCustom name exposed for the capability."]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nCustom description exposed for the capability."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the capability is available through the MCP server."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the tool or prompt capability to override."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedTools {}

impl DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedTools { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedTools {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedTools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServers_Result_UpdatedTools {}

impl BuildDataZeroTrustAccessAiControlsMcpServers_Result_UpdatedTools {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedTools {
        DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedTools {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedToolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedToolsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedToolsRef {
        DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedToolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedToolsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alias` after provisioning.\nCustom name exposed for the capability."]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nCustom description exposed for the capability."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the capability is available through the MCP server."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the tool or prompt capability to override."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataZeroTrustAccessAiControlsMcpServers_Result {}

impl DataZeroTrustAccessAiControlsMcpServers_Result { }

impl terrars::ToListMappable for DataZeroTrustAccessAiControlsMcpServers_Result {
    type O = terrars::BlockAssignable<DataZeroTrustAccessAiControlsMcpServers_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataZeroTrustAccessAiControlsMcpServers_Result {}

impl BuildDataZeroTrustAccessAiControlsMcpServers_Result {
    pub fn build(self) -> DataZeroTrustAccessAiControlsMcpServers_Result {
        DataZeroTrustAccessAiControlsMcpServers_Result {}
    }
}

#[allow(dead_code)]
pub struct DataZeroTrustAccessAiControlsMcpServers_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustAccessAiControlsMcpServers_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataZeroTrustAccessAiControlsMcpServers_ResultRef {
        DataZeroTrustAccessAiControlsMcpServers_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataZeroTrustAccessAiControlsMcpServers_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_config_summary` after provisioning.\nSafe subset of auth_credentials surfaced to the dashboard. Includes auth_mode (dcr|manual), has_client_secret, client_secret_version, and the OAuth endpoints + client_id for manual servers. Never includes the secret value."]
    pub fn auth_config_summary(&self) -> DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummaryRef {
        DataZeroTrustAccessAiControlsMcpServers_Result_AuthConfigSummaryRef::new(
            self.shared().clone(),
            format!("{}.auth_config_summary", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `auth_type` after provisioning.\nAuthentication method used to connect to the upstream MCP server.\nAvailable values: \"oauth\", \"bearer\", \"unauthenticated\"."]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `authentication_status` after provisioning.\nWhether administrative authentication is required before capabilities can be synced. Manual OAuth is user-managed and has no administrative authentication flow.\nAvailable values: \"not_required\", \"required\", \"connected\", \"stale\", \"manual\"."]
    pub fn authentication_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_status", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional description of the MCP server."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(&self) -> DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetailsRef {
        DataZeroTrustAccessAiControlsMcpServers_Result_ErrorDetailsRef::new(
            self.shared().clone(),
            format!("{}.error_details", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nURL of the upstream MCP endpoint."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the MCP server."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_shared_oauth_callback_enabled` after provisioning.\nWhen true, the gateway worker uses the shared Cloudflare-owned OAuth callback endpoint as the redirect_uri for upstream on-behalf OAuth, instead of the customer portal hostname. New public server creates default to true; existing servers default to false from migration until explicitly updated. Effective behavior is gated by the gateway worker's per-env rollout mode KV key."]
    pub fn is_shared_oauth_callback_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_shared_oauth_callback_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `last_successful_sync` after provisioning.\n"]
    pub fn last_successful_sync(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_successful_sync", self.base))
    }

    #[doc = "Get a reference to the value of field `last_synced` after provisioning.\n"]
    pub fn last_synced(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_synced", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_at` after provisioning.\n"]
    pub fn modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_at", self.base))
    }

    #[doc = "Get a reference to the value of field `modified_by` after provisioning.\n"]
    pub fn modified_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modified_by", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDisplay name for the MCP server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `prompts` after provisioning.\n"]
    pub fn prompts(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prompts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secure_web_gateway` after provisioning.\nRoute outbound traffic to this MCP server through Zero Trust Secure Web Gateway."]
    pub fn secure_web_gateway(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure_web_gateway", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nCurrent sync state of the server\nAvailable values: \"waiting\", \"ready\", \"stale\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `tools` after provisioning.\n"]
    pub fn tools(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_prompts` after provisioning.\nServer-wide prompt capability overrides."]
    pub fn updated_prompts(&self) -> terrars::ListRef<DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedPromptsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_prompts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_tools` after provisioning.\nServer-wide tool capability overrides."]
    pub fn updated_tools(&self) -> terrars::ListRef<DataZeroTrustAccessAiControlsMcpServers_Result_UpdatedToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.updated_tools", self.base))
    }
}
