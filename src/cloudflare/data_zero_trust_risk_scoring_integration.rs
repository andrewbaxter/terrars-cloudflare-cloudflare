#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataZeroTrustRiskScoringIntegrationData_ {
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
    integration_id: terrars::PrimField<std::string::String>,
}

struct DataZeroTrustRiskScoringIntegration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataZeroTrustRiskScoringIntegrationData_>,
}

#[derive(Clone)]
pub struct DataZeroTrustRiskScoringIntegration(std::rc::Rc<DataZeroTrustRiskScoringIntegration_>);

impl DataZeroTrustRiskScoringIntegration {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_tag` after provisioning.\nThe Cloudflare account tag."]
    pub fn account_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `active` after provisioning.\nWhether this integration is enabled and should export changes in risk score."]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nWhen the integration was created in RFC3339 format."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\n"]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_type` after provisioning.\nAvailable values: \"Okta\"."]
    pub fn integration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reference_id` after provisioning.\nA reference ID defined by the client.\nShould be set to the Access-Okta IDP integration ID.\nUseful when the risk-score integration needs to be associated with a secondary asset and recalled using that ID."]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant_url` after provisioning.\nThe base URL for the tenant. E.g. \"https://tenant.okta.com\"."]
    pub fn tenant_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `well_known_url` after provisioning.\nThe URL for the Shared Signals Framework configuration, e.g. \"/.well-known/sse-configuration/{integration_uuid}/\". https://openid.net/specs/openid-sse-framework-1_0.html#rfc.section.6.2.1."]
    pub fn well_known_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.well_known_url", self.extract_ref()))
    }
}

impl terrars::Referable for DataZeroTrustRiskScoringIntegration {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataZeroTrustRiskScoringIntegration { }

impl terrars::ToListMappable for DataZeroTrustRiskScoringIntegration {
    type O = terrars::ListRef<DataZeroTrustRiskScoringIntegrationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataZeroTrustRiskScoringIntegration_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_zero_trust_risk_scoring_integration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataZeroTrustRiskScoringIntegration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub integration_id: terrars::PrimField<std::string::String>,
}

impl BuildDataZeroTrustRiskScoringIntegration {
    pub fn build(self, stack: &mut terrars::Stack) -> DataZeroTrustRiskScoringIntegration {
        let out = DataZeroTrustRiskScoringIntegration(std::rc::Rc::new(DataZeroTrustRiskScoringIntegration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataZeroTrustRiskScoringIntegrationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                integration_id: self.integration_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataZeroTrustRiskScoringIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataZeroTrustRiskScoringIntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataZeroTrustRiskScoringIntegrationRef {
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

    #[doc = "Get a reference to the value of field `account_tag` after provisioning.\nThe Cloudflare account tag."]
    pub fn account_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `active` after provisioning.\nWhether this integration is enabled and should export changes in risk score."]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nWhen the integration was created in RFC3339 format."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_id` after provisioning.\n"]
    pub fn integration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_type` after provisioning.\nAvailable values: \"Okta\"."]
    pub fn integration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reference_id` after provisioning.\nA reference ID defined by the client.\nShould be set to the Access-Okta IDP integration ID.\nUseful when the risk-score integration needs to be associated with a secondary asset and recalled using that ID."]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant_url` after provisioning.\nThe base URL for the tenant. E.g. \"https://tenant.okta.com\"."]
    pub fn tenant_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `well_known_url` after provisioning.\nThe URL for the Shared Signals Framework configuration, e.g. \"/.well-known/sse-configuration/{integration_uuid}/\". https://openid.net/specs/openid-sse-framework-1_0.html#rfc.section.6.2.1."]
    pub fn well_known_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.well_known_url", self.extract_ref()))
    }
}
