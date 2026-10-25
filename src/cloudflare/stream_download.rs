#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct StreamDownloadData_ {
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
    identifier: terrars::PrimField<std::string::String>,
}

struct StreamDownload_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StreamDownloadData_>,
}

#[derive(Clone)]
pub struct StreamDownload(std::rc::Rc<StreamDownload_>);

impl StreamDownload {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `audio` after provisioning.\nThe audio-only download. Only present if this download type has been created."]
    pub fn audio(&self) -> StreamDownload_AudioRef {
        StreamDownload_AudioRef::new(self.shared().clone(), format!("{}.audio", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nThe default video download. Only present if this download type has been created."]
    pub fn default(&self) -> StreamDownload_DefaultRef {
        StreamDownload_DefaultRef::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }
}

impl terrars::Referable for StreamDownload {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StreamDownload { }

impl terrars::ToListMappable for StreamDownload {
    type O = terrars::ListRef<StreamDownloadRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StreamDownload_ {
    fn extract_resource_type(&self) -> std::string::String {
        "cloudflare_stream_download".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStreamDownload {
    pub tf_id: std::string::String,
    #[doc = "Identifier."]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = "A Cloudflare-generated unique identifier for a media item."]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildStreamDownload {
    pub fn build(self, stack: &mut terrars::Stack) -> StreamDownload {
        let out = StreamDownload(std::rc::Rc::new(StreamDownload_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StreamDownloadData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: self.account_id,
                identifier: self.identifier,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StreamDownloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamDownloadRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StreamDownloadRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nIdentifier."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `audio` after provisioning.\nThe audio-only download. Only present if this download type has been created."]
    pub fn audio(&self) -> StreamDownload_AudioRef {
        StreamDownload_AudioRef::new(self.shared().clone(), format!("{}.audio", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nThe default video download. Only present if this download type has been created."]
    pub fn default(&self) -> StreamDownload_DefaultRef {
        StreamDownload_DefaultRef::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nA Cloudflare-generated unique identifier for a media item."]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct StreamDownload_Audio {}

impl StreamDownload_Audio { }

impl terrars::ToListMappable for StreamDownload_Audio {
    type O = terrars::BlockAssignable<StreamDownload_Audio>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamDownload_Audio {}

impl BuildStreamDownload_Audio {
    pub fn build(self) -> StreamDownload_Audio {
        StreamDownload_Audio {}
    }
}

#[allow(dead_code)]
pub struct StreamDownload_AudioRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamDownload_AudioRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamDownload_AudioRef {
        StreamDownload_AudioRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamDownload_AudioRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percent_complete` after provisioning.\nIndicates the progress as a percentage between 0 and 100."]
    pub fn percent_complete(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_complete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of a generated download.\nAvailable values: \"ready\", \"inprogress\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe URL to access the generated download."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct StreamDownload_Default {}

impl StreamDownload_Default { }

impl terrars::ToListMappable for StreamDownload_Default {
    type O = terrars::BlockAssignable<StreamDownload_Default>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStreamDownload_Default {}

impl BuildStreamDownload_Default {
    pub fn build(self) -> StreamDownload_Default {
        StreamDownload_Default {}
    }
}

#[allow(dead_code)]
pub struct StreamDownload_DefaultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StreamDownload_DefaultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StreamDownload_DefaultRef {
        StreamDownload_DefaultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StreamDownload_DefaultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percent_complete` after provisioning.\nIndicates the progress as a percentage between 0 and 100."]
    pub fn percent_complete(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_complete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of a generated download.\nAvailable values: \"ready\", \"inprogress\", \"error\"."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe URL to access the generated download."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
