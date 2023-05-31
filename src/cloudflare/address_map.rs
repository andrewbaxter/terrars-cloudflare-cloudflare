use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderCloudflare;

#[derive(Serialize)]
struct AddressMapData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_sni: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ips: Option<Vec<AddressMapIpsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memberships: Option<Vec<AddressMapMembershipsEl>>,
    dynamic: AddressMapDynamic,
}

struct AddressMap_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AddressMapData>,
}

#[derive(Clone)]
pub struct AddressMap(Rc<AddressMap_>);

impl AddressMap {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderCloudflare) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `default_sni`.\nIf you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map."]
    pub fn set_default_sni(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_sni = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nDescription of the address map."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ips`.\n"]
    pub fn set_ips(self, v: impl Into<BlockAssignable<AddressMapIpsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ips = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ips = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `memberships`.\n"]
    pub fn set_memberships(self, v: impl Into<BlockAssignable<AddressMapMembershipsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().memberships = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.memberships = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `can_delete` after provisioning.\nIf set to false, then the Address Map cannot be deleted via API. This is true for Cloudflare-managed maps."]
    pub fn can_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_delete", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `can_modify_ips` after provisioning.\nIf set to false, then the IPs on the Address Map cannot be modified via the API. This is true for Cloudflare-managed maps."]
    pub fn can_modify_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_modify_ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_sni` after provisioning.\nIf you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map."]
    pub fn default_sni(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_sni", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the address map."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether the Address Map is enabled or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl Referable for AddressMap {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AddressMap { }

impl ToListMappable for AddressMap {
    type O = ListRef<AddressMapRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AddressMap_ {
    fn extract_resource_type(&self) -> String {
        "cloudflare_address_map".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAddressMap {
    pub tf_id: String,
    #[doc= "The account identifier to target for the resource."]
    pub account_id: PrimField<String>,
    #[doc= "Whether the Address Map is enabled or not."]
    pub enabled: PrimField<bool>,
}

impl BuildAddressMap {
    pub fn build(self, stack: &mut Stack) -> AddressMap {
        let out = AddressMap(Rc::new(AddressMap_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AddressMapData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                default_sni: core::default::Default::default(),
                description: core::default::Default::default(),
                enabled: self.enabled,
                id: core::default::Default::default(),
                ips: core::default::Default::default(),
                memberships: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AddressMapRef {
    shared: StackShared,
    base: String,
}

impl Ref for AddressMapRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AddressMapRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe account identifier to target for the resource."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `can_delete` after provisioning.\nIf set to false, then the Address Map cannot be deleted via API. This is true for Cloudflare-managed maps."]
    pub fn can_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_delete", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `can_modify_ips` after provisioning.\nIf set to false, then the IPs on the Address Map cannot be modified via the API. This is true for Cloudflare-managed maps."]
    pub fn can_modify_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_modify_ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_sni` after provisioning.\nIf you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map."]
    pub fn default_sni(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_sni", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the address map."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether the Address Map is enabled or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AddressMapIpsEl {
    ip: PrimField<String>,
}

impl AddressMapIpsEl { }

impl ToListMappable for AddressMapIpsEl {
    type O = BlockAssignable<AddressMapIpsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAddressMapIpsEl {
    #[doc= "An IPv4 or IPv6 address."]
    pub ip: PrimField<String>,
}

impl BuildAddressMapIpsEl {
    pub fn build(self) -> AddressMapIpsEl {
        AddressMapIpsEl { ip: self.ip }
    }
}

pub struct AddressMapIpsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AddressMapIpsElRef {
    fn new(shared: StackShared, base: String) -> AddressMapIpsElRef {
        AddressMapIpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AddressMapIpsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\nAn IPv4 or IPv6 address."]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct AddressMapMembershipsEl {
    identifier: PrimField<String>,
    kind: PrimField<String>,
}

impl AddressMapMembershipsEl { }

impl ToListMappable for AddressMapMembershipsEl {
    type O = BlockAssignable<AddressMapMembershipsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAddressMapMembershipsEl {
    #[doc= "Identifier of the account or zone."]
    pub identifier: PrimField<String>,
    #[doc= "The type of the membership."]
    pub kind: PrimField<String>,
}

impl BuildAddressMapMembershipsEl {
    pub fn build(self) -> AddressMapMembershipsEl {
        AddressMapMembershipsEl {
            identifier: self.identifier,
            kind: self.kind,
        }
    }
}

pub struct AddressMapMembershipsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AddressMapMembershipsElRef {
    fn new(shared: StackShared, base: String) -> AddressMapMembershipsElRef {
        AddressMapMembershipsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AddressMapMembershipsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `can_delete` after provisioning.\nControls whether the membership can be deleted via the API or not."]
    pub fn can_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_delete", self.base))
    }

    #[doc= "Get a reference to the value of field `identifier` after provisioning.\nIdentifier of the account or zone."]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nThe type of the membership."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }
}

#[derive(Serialize, Default)]
struct AddressMapDynamic {
    ips: Option<DynamicBlock<AddressMapIpsEl>>,
    memberships: Option<DynamicBlock<AddressMapMembershipsEl>>,
}
