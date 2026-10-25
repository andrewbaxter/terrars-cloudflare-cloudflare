#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct ApiShieldSchemaData_ {
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
    file: terrars::PrimField<std::string::String>,
    kind: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    omit_source: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_enabled: std::option::Option<terrars::PrimField<std::string::String>>,
    zone_id: terrars::PrimField<std::string::String>,
}

struct ApiShieldSchema_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApiShieldSchemaData_>,
}

#[derive(Clone)]
pub struct ApiShieldSchema(std::rc::Rc<ApiShieldSchema_>);

impl ApiShieldSchema {
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

    #[doc = "Set the field `name`.\nName of the schema"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `omit_source`.\nOmit the source-files of schemas and only retrieve their meta-data."]
    pub fn set_omit_source(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().omit_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_id`.\n"]
    pub fn set_schema_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schema_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `validation_enabled`.\nFlag whether schema is enabled for validation.\nAvailable values: \"true\", \"false\"."]
    pub fn set_validation_enabled(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().validation_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\nSchema file bytes"]
    pub fn file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nKind of schema\nAvailable values: \"openapi_v3\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the schema"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `omit_source` after provisioning.\nOmit the source-files of schemas and only retrieve their meta-data."]
    pub fn omit_source(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.omit_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ApiShieldSchema_SchemaRef {
        ApiShieldSchema_SchemaRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_id` after provisioning.\n"]
    pub fn schema_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\nSource of the schema"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upload_details` after provisioning.\n"]
    pub fn upload_details(&self) -> ApiShieldSchema_UploadDetailsRef {
        ApiShieldSchema_UploadDetailsRef::new(
            self.shared().clone(),
            format!("{}.upload_details", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `validation_enabled` after provisioning.\nFlag whether schema is enabled for validation.\nAvailable values: \"true\", \"false\"."]
    pub fn validation_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for ApiShieldSchema {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApiShieldSchema { }

impl terrars::ToListMappable for ApiShieldSchema {
    type O = terrars::ListRef<ApiShieldSchemaRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApiShieldSchema_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_api_shield_schema".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiShieldSchema {
    pub tf_id: std::string::String,
    #[doc = "Schema file bytes"]
    pub file: terrars::PrimField<std::string::String>,
    #[doc = "Kind of schema\nAvailable values: \"openapi_v3\"."]
    pub kind: terrars::PrimField<std::string::String>,
    #[doc = "Identifier."]
    pub zone_id: terrars::PrimField<std::string::String>,
}

impl BuildApiShieldSchema {
    pub fn build(self, stack: &mut terrars::Stack) -> ApiShieldSchema {
        let out = ApiShieldSchema(std::rc::Rc::new(ApiShieldSchema_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApiShieldSchemaData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                file: self.file,
                kind: self.kind,
                name: core::default::Default::default(),
                omit_source: core::default::Default::default(),
                schema_id: core::default::Default::default(),
                validation_enabled: core::default::Default::default(),
                zone_id: self.zone_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiShieldSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldSchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiShieldSchemaRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\nSchema file bytes"]
    pub fn file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nKind of schema\nAvailable values: \"openapi_v3\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the schema"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `omit_source` after provisioning.\nOmit the source-files of schemas and only retrieve their meta-data."]
    pub fn omit_source(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.omit_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ApiShieldSchema_SchemaRef {
        ApiShieldSchema_SchemaRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_id` after provisioning.\n"]
    pub fn schema_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\nSource of the schema"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upload_details` after provisioning.\n"]
    pub fn upload_details(&self) -> ApiShieldSchema_UploadDetailsRef {
        ApiShieldSchema_UploadDetailsRef::new(
            self.shared().clone(),
            format!("{}.upload_details", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `validation_enabled` after provisioning.\nFlag whether schema is enabled for validation.\nAvailable values: \"true\", \"false\"."]
    pub fn validation_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\nIdentifier."]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiShieldSchema_Schema {}

impl ApiShieldSchema_Schema { }

impl terrars::ToListMappable for ApiShieldSchema_Schema {
    type O = terrars::BlockAssignable<ApiShieldSchema_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldSchema_Schema {}

impl BuildApiShieldSchema_Schema {
    pub fn build(self) -> ApiShieldSchema_Schema {
        ApiShieldSchema_Schema {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldSchema_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldSchema_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiShieldSchema_SchemaRef {
        ApiShieldSchema_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldSchema_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nKind of schema\nAvailable values: \"openapi_v3\"."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the schema"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_id` after provisioning.\nUUID."]
    pub fn schema_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_id", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\nSource of the schema"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc =
        "Get a reference to the value of field `validation_enabled` after provisioning.\nFlag whether schema is enabled for validation."]
    pub fn validation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldSchema_UploadDetails_Warnings {}

impl ApiShieldSchema_UploadDetails_Warnings { }

impl terrars::ToListMappable for ApiShieldSchema_UploadDetails_Warnings {
    type O = terrars::BlockAssignable<ApiShieldSchema_UploadDetails_Warnings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldSchema_UploadDetails_Warnings {}

impl BuildApiShieldSchema_UploadDetails_Warnings {
    pub fn build(self) -> ApiShieldSchema_UploadDetails_Warnings {
        ApiShieldSchema_UploadDetails_Warnings {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldSchema_UploadDetails_WarningsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldSchema_UploadDetails_WarningsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiShieldSchema_UploadDetails_WarningsRef {
        ApiShieldSchema_UploadDetails_WarningsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldSchema_UploadDetails_WarningsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nCode that identifies the event that occurred."]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `locations` after provisioning.\nJSONPath location(s) in the schema where these events were encountered.  See [https://goessner.net/articles/JsonPath/](https://goessner.net/articles/JsonPath/) for JSONPath specification."]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `message` after provisioning.\nDiagnostic message that describes the event."]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiShieldSchema_UploadDetails {}

impl ApiShieldSchema_UploadDetails { }

impl terrars::ToListMappable for ApiShieldSchema_UploadDetails {
    type O = terrars::BlockAssignable<ApiShieldSchema_UploadDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiShieldSchema_UploadDetails {}

impl BuildApiShieldSchema_UploadDetails {
    pub fn build(self) -> ApiShieldSchema_UploadDetails {
        ApiShieldSchema_UploadDetails {}
    }
}

#[allow(dead_code)]
pub struct ApiShieldSchema_UploadDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiShieldSchema_UploadDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiShieldSchema_UploadDetailsRef {
        ApiShieldSchema_UploadDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiShieldSchema_UploadDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `warnings` after provisioning.\nDiagnostic warning events that occurred during processing. These events are non-critical errors found within the schema."]
    pub fn warnings(&self) -> terrars::ListRef<ApiShieldSchema_UploadDetails_WarningsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warnings", self.base))
    }
}
