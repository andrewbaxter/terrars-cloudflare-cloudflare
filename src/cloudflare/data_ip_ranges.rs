use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct DataIpRangesData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataIpRanges_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIpRangesData>,
}

#[derive(Clone)]
pub struct DataIpRanges(std::rc::Rc<DataIpRanges_>);

impl DataIpRanges {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `china_ipv4_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv4 China CIDR blocks."]
    pub fn china_ipv4_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.china_ipv4_cidr_blocks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `china_ipv6_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv6 China CIDR blocks."]
    pub fn china_ipv6_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.china_ipv6_cidr_blocks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cidr_blocks` after provisioning.\nThe lexically ordered list of all non-China CIDR blocks."]
    pub fn cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv4 CIDR blocks."]
    pub fn ipv4_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4_cidr_blocks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv6 CIDR blocks."]
    pub fn ipv6_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr_blocks", self.extract_ref()))
    }
}

impl terrars::Referable for DataIpRanges {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIpRanges { }

impl terrars::ToListMappable for DataIpRanges {
    type O = terrars::ListRef<DataIpRangesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIpRanges_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "cloudflare_ip_ranges".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIpRanges {
    pub tf_id: std::string::String,
}

impl BuildDataIpRanges {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIpRanges {
        let out = DataIpRanges(std::rc::Rc::new(DataIpRanges_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIpRangesData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIpRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIpRangesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIpRangesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `china_ipv4_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv4 China CIDR blocks."]
    pub fn china_ipv4_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.china_ipv4_cidr_blocks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `china_ipv6_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv6 China CIDR blocks."]
    pub fn china_ipv6_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.china_ipv6_cidr_blocks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cidr_blocks` after provisioning.\nThe lexically ordered list of all non-China CIDR blocks."]
    pub fn cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv4 CIDR blocks."]
    pub fn ipv4_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4_cidr_blocks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_cidr_blocks` after provisioning.\nThe lexically ordered list of only the IPv6 CIDR blocks."]
    pub fn ipv6_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr_blocks", self.extract_ref()))
    }
}
