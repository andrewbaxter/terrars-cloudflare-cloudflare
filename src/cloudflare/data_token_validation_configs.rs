#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataTokenValidationConfigsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_items: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataTokenValidationConfigs_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataTokenValidationConfigsData_>,
}

#[derive(Clone)]
pub struct DataTokenValidationConfigs(std::rc::Rc<DataTokenValidationConfigs_>);

impl DataTokenValidationConfigs {
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

    #[doc = "Set the field `max_items`.\nMax items to fetch, default: 1000"]
    pub fn set_max_items(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone_id`.\nIdentifier."]
    pub fn set_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataTokenValidationConfigs_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataTokenValidationConfigs {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataTokenValidationConfigs { }

impl terrars::ToListMappable for DataTokenValidationConfigs {
    type O = terrars::ListRef<DataTokenValidationConfigsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataTokenValidationConfigs_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_token_validation_configs".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTokenValidationConfigs {
    pub tf_id: std::string::String,
}

impl BuildDataTokenValidationConfigs {
    pub fn build(self, stack: &mut terrars::Stack) -> DataTokenValidationConfigs {
        let out = DataTokenValidationConfigs(std::rc::Rc::new(DataTokenValidationConfigs_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataTokenValidationConfigsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                max_items: core::default::Default::default(),
                zone_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTokenValidationConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTokenValidationConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataTokenValidationConfigsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMax items to fetch, default: 1000"]
    pub fn max_items(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\nThe items returned by the data source"]
    pub fn result(&self) -> terrars::ListRef<DataTokenValidationConfigs_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataTokenValidationConfigs_Result_Credentials_Keys {}

impl DataTokenValidationConfigs_Result_Credentials_Keys { }

impl terrars::ToListMappable for DataTokenValidationConfigs_Result_Credentials_Keys {
    type O = terrars::BlockAssignable<DataTokenValidationConfigs_Result_Credentials_Keys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTokenValidationConfigs_Result_Credentials_Keys {}

impl BuildDataTokenValidationConfigs_Result_Credentials_Keys {
    pub fn build(self) -> DataTokenValidationConfigs_Result_Credentials_Keys {
        DataTokenValidationConfigs_Result_Credentials_Keys {}
    }
}

#[allow(dead_code)]
pub struct DataTokenValidationConfigs_Result_Credentials_KeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTokenValidationConfigs_Result_Credentials_KeysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTokenValidationConfigs_Result_Credentials_KeysRef {
        DataTokenValidationConfigs_Result_Credentials_KeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTokenValidationConfigs_Result_Credentials_KeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alg` after provisioning.\nAlgorithm\nAvailable values: \"RS256\", \"RS384\", \"RS512\", \"PS256\", \"PS384\", \"PS512\", \"ES256\", \"ES384\", \"HS256\", \"HS384\", \"HS512\"."]
    pub fn alg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alg", self.base))
    }

    #[doc =
        "Get a reference to the value of field `crv` after provisioning.\nCurve\nAvailable values: \"P-256\", \"P-384\"."]
    pub fn crv(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crv", self.base))
    }

    #[doc = "Get a reference to the value of field `e` after provisioning.\nRSA exponent"]
    pub fn e(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.e", self.base))
    }

    #[doc = "Get a reference to the value of field `kid` after provisioning.\nKey ID"]
    pub fn kid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kty` after provisioning.\nKey Type\nAvailable values: \"RSA\", \"EC\", \"oct\"."]
    pub fn kty(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kty", self.base))
    }

    #[doc = "Get a reference to the value of field `n` after provisioning.\nRSA modulus"]
    pub fn n(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.n", self.base))
    }

    #[doc = "Get a reference to the value of field `x` after provisioning.\nX EC coordinate"]
    pub fn x(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x", self.base))
    }

    #[doc = "Get a reference to the value of field `y` after provisioning.\nY EC coordinate"]
    pub fn y(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.y", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTokenValidationConfigs_Result_Credentials {}

impl DataTokenValidationConfigs_Result_Credentials { }

impl terrars::ToListMappable for DataTokenValidationConfigs_Result_Credentials {
    type O = terrars::BlockAssignable<DataTokenValidationConfigs_Result_Credentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTokenValidationConfigs_Result_Credentials {}

impl BuildDataTokenValidationConfigs_Result_Credentials {
    pub fn build(self) -> DataTokenValidationConfigs_Result_Credentials {
        DataTokenValidationConfigs_Result_Credentials {}
    }
}

#[allow(dead_code)]
pub struct DataTokenValidationConfigs_Result_CredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTokenValidationConfigs_Result_CredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTokenValidationConfigs_Result_CredentialsRef {
        DataTokenValidationConfigs_Result_CredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTokenValidationConfigs_Result_CredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> terrars::ListRef<DataTokenValidationConfigs_Result_Credentials_KeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keys", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTokenValidationConfigs_Result {}

impl DataTokenValidationConfigs_Result { }

impl terrars::ToListMappable for DataTokenValidationConfigs_Result {
    type O = terrars::BlockAssignable<DataTokenValidationConfigs_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTokenValidationConfigs_Result {}

impl BuildDataTokenValidationConfigs_Result {
    pub fn build(self) -> DataTokenValidationConfigs_Result {
        DataTokenValidationConfigs_Result {}
    }
}

#[allow(dead_code)]
pub struct DataTokenValidationConfigs_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTokenValidationConfigs_ResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataTokenValidationConfigs_ResultRef {
        DataTokenValidationConfigs_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTokenValidationConfigs_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> DataTokenValidationConfigs_Result_CredentialsRef {
        DataTokenValidationConfigs_Result_CredentialsRef::new(
            self.shared().clone(),
            format!("{}.credentials", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUUID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc = "Get a reference to the value of field `token_sources` after provisioning.\n"]
    pub fn token_sources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `token_type` after provisioning.\nAvailable values: \"JWT\"."]
    pub fn token_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_type", self.base))
    }
}
