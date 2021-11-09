#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_Contacts_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AddContactResult(pub i32);
impl AddContactResult {
    pub const Added: AddContactResult = AddContactResult(0i32);
    pub const AlreadyAdded: AddContactResult = AddContactResult(1i32);
    pub const Unavailable: AddContactResult = AddContactResult(2i32);
}
impl ::core::convert::From<i32> for AddContactResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AddContactResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AddContactResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.Provider.AddContactResult;i4)");
}
impl ::windows::runtime::DefaultType for AddContactResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactPickerUI(pub ::windows::runtime::IInspectable);
impl ContactPickerUI {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Contacts_Provider`*"]
    pub fn AddContact<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Contact>>(&self, id: Param0, contact: Param1) -> ::windows::runtime::Result<AddContactResult> {
        let this = self;
        unsafe {
            let mut result__: AddContactResult = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), contact.into_param().abi(), &mut result__).from_abi::<AddContactResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts_Provider`*"]
    pub fn RemoveContact<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, id: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts_Provider`*"]
    pub fn ContainsContact<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, id: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts_Provider`, `Foundation_Collections`*"]
    pub fn DesiredFields(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts_Provider`*"]
    pub fn SelectionMode(&self) -> ::windows::runtime::Result<super::ContactSelectionMode> {
        let this = self;
        unsafe {
            let mut result__: super::ContactSelectionMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ContactSelectionMode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_Provider`, `Foundation`*"]
    pub fn ContactRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_Provider`, `Foundation`*"]
    pub fn RemoveContactRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts_Provider`*"]
    pub fn AddContact2<'a, Param0: ::windows::runtime::IntoParam<'a, super::Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<AddContactResult> {
        let this = &::windows::runtime::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__: AddContactResult = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<AddContactResult>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts_Provider`, `Foundation_Collections`*"]
    pub fn DesiredFieldsWithContactFieldType(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>> {
        let this = &::windows::runtime::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactPickerUI {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Provider.ContactPickerUI;{e2cc1366-cf66-43c4-a96a-a5a112db4746})");
}
unsafe impl ::windows::runtime::Interface for ContactPickerUI {
    type Vtable = IContactPickerUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3805025126, 53094, 17348, [169, 106, 165, 161, 18, 219, 71, 70]);
}
impl ::windows::runtime::RuntimeName for ContactPickerUI {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactPickerUI";
}
impl ::core::convert::From<ContactPickerUI> for ::windows::runtime::IUnknown {
    fn from(value: ContactPickerUI) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactPickerUI> for ::windows::runtime::IUnknown {
    fn from(value: &ContactPickerUI) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactPickerUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactPickerUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactPickerUI> for ::windows::runtime::IInspectable {
    fn from(value: ContactPickerUI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactPickerUI> for ::windows::runtime::IInspectable {
    fn from(value: &ContactPickerUI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactPickerUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactPickerUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `ApplicationModel_Contacts_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactRemovedEventArgs(pub ::windows::runtime::IInspectable);
impl ContactRemovedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Contacts_Provider`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactRemovedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs;{6f354338-3302-4d13-ad8d-adcc0ff9e47c})");
}
unsafe impl ::windows::runtime::Interface for ContactRemovedEventArgs {
    type Vtable = IContactRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1865761592, 13058, 19731, [173, 141, 173, 204, 15, 249, 228, 124]);
}
impl ::windows::runtime::RuntimeName for ContactRemovedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs";
}
impl ::core::convert::From<ContactRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPickerUI(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPickerUI {
    type Vtable = IContactPickerUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3805025126, 53094, 17348, [169, 106, 165, 161, 18, 219, 71, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, contact: ::windows::runtime::RawPtr, result__: *mut AddContactResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::ContactSelectionMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPickerUI2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPickerUI2 {
    type Vtable = IContactPickerUI2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1849990696, 31525, 18841, [155, 11, 135, 84, 0, 161, 232, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut AddContactResult) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactRemovedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactRemovedEventArgs {
    type Vtable = IContactRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1865761592, 13058, 19731, [173, 141, 173, 204, 15, 249, 228, 124]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);