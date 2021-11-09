#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub struct ACT_AUTHORIZATION_STATE {
    pub ulState: u32,
}
impl ACT_AUTHORIZATION_STATE {}
impl ::core::default::Default for ACT_AUTHORIZATION_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ACT_AUTHORIZATION_STATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ACT_AUTHORIZATION_STATE").field("ulState", &self.ulState).finish()
    }
}
impl ::core::cmp::PartialEq for ACT_AUTHORIZATION_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.ulState == other.ulState
    }
}
impl ::core::cmp::Eq for ACT_AUTHORIZATION_STATE {}
unsafe impl ::windows::runtime::Abi for ACT_AUTHORIZATION_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ACT_AUTHORIZATION_STATE_VALUE(pub i32);
pub const ACT_UNAUTHORIZED: ACT_AUTHORIZATION_STATE_VALUE = ACT_AUTHORIZATION_STATE_VALUE(0i32);
pub const ACT_AUTHORIZED: ACT_AUTHORIZATION_STATE_VALUE = ACT_AUTHORIZATION_STATE_VALUE(1i32);
impl ::core::convert::From<i32> for ACT_AUTHORIZATION_STATE_VALUE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACT_AUTHORIZATION_STATE_VALUE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ACT_AUTHORIZE_ON_RESUME: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ACT_AUTHORIZE_ON_SESSION_UNLOCK: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ACT_UNAUTHORIZE_ON_SESSION_LOCK: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ACT_UNAUTHORIZE_ON_SUSPEND: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const APPUSERMODEL_STARTPINOPTION_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const APPUSERMODEL_STARTPINOPTION_NOPINONINSTALL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const APPUSERMODEL_STARTPINOPTION_USERPINNED: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const AUDIO_CHANNELCOUNT_MONO: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const AUDIO_CHANNELCOUNT_STEREO: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const BLUETOOTH_ADDRESS_TYPE_PUBLIC: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const BLUETOOTH_ADDRESS_TYPE_RANDOM: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const BLUETOOTH_CACHED_MODE_UNCACHED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const BLUETOOTH_CACHE_MODE_CACHED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_CAPABILITY_CERTIFICATE_SUPPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_CAPABILITY_HASH_ALG: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_CAPABILITY_OPTIONAL_FEATURES: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_CAPABILITY_SIGNATURE_ALG: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_MAX_CAPABILITY: u32 = 255u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_ASCh: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_ASCm: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_EMPTY: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_HCh: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_PCp: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_SIGNER: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_VALIDATION_POLICY_BASIC: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_VALIDATION_POLICY_EXTENDED: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_VALIDATION_POLICY_NONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_VALIDATION_POLICY_RESERVED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CREATOROPENWITHUIOPTION_HIDDEN: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CREATOROPENWITHUIOPTION_VISIBLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_AUTHENTICATED: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_AUTHENTICATION_DENIED: u32 = 2147483649u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_DEVICE_ERROR: u32 = 2147483650u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_NOT_AUTHENTICATED: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_NO_AUTHENTICATION_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_UNKNOWN: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
pub struct ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    pub CurrentAdminFailures: u8,
    pub CurrentUserFailures: u8,
    pub TotalUserAuthenticationCount: u32,
    pub TotalAdminAuthenticationCount: u32,
    pub FipsCompliant: super::super::Foundation::BOOL,
    pub SecurityIDAvailable: super::super::Foundation::BOOL,
    pub InitializeInProgress: super::super::Foundation::BOOL,
    pub ITMSArmed: super::super::Foundation::BOOL,
    pub ITMSArmable: super::super::Foundation::BOOL,
    pub UserCreated: super::super::Foundation::BOOL,
    pub ResetOnPORDefault: super::super::Foundation::BOOL,
    pub ResetOnPORCurrent: super::super::Foundation::BOOL,
    pub MaxAdminFailures: u8,
    pub MaxUserFailures: u8,
    pub TimeToCompleteInitialization: u32,
    pub TimeRemainingToCompleteInitialization: u32,
    pub MinTimeToAuthenticate: u32,
    pub MaxAdminPasswordSize: u8,
    pub MinAdminPasswordSize: u8,
    pub MaxAdminHintSize: u8,
    pub MaxUserPasswordSize: u8,
    pub MinUserPasswordSize: u8,
    pub MaxUserHintSize: u8,
    pub MaxUserNameSize: u8,
    pub MaxSiloNameSize: u8,
    pub MaxChallengeSize: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION")
            .field("CurrentAdminFailures", &self.CurrentAdminFailures)
            .field("CurrentUserFailures", &self.CurrentUserFailures)
            .field("TotalUserAuthenticationCount", &self.TotalUserAuthenticationCount)
            .field("TotalAdminAuthenticationCount", &self.TotalAdminAuthenticationCount)
            .field("FipsCompliant", &self.FipsCompliant)
            .field("SecurityIDAvailable", &self.SecurityIDAvailable)
            .field("InitializeInProgress", &self.InitializeInProgress)
            .field("ITMSArmed", &self.ITMSArmed)
            .field("ITMSArmable", &self.ITMSArmable)
            .field("UserCreated", &self.UserCreated)
            .field("ResetOnPORDefault", &self.ResetOnPORDefault)
            .field("ResetOnPORCurrent", &self.ResetOnPORCurrent)
            .field("MaxAdminFailures", &self.MaxAdminFailures)
            .field("MaxUserFailures", &self.MaxUserFailures)
            .field("TimeToCompleteInitialization", &self.TimeToCompleteInitialization)
            .field("TimeRemainingToCompleteInitialization", &self.TimeRemainingToCompleteInitialization)
            .field("MinTimeToAuthenticate", &self.MinTimeToAuthenticate)
            .field("MaxAdminPasswordSize", &self.MaxAdminPasswordSize)
            .field("MinAdminPasswordSize", &self.MinAdminPasswordSize)
            .field("MaxAdminHintSize", &self.MaxAdminHintSize)
            .field("MaxUserPasswordSize", &self.MaxUserPasswordSize)
            .field("MinUserPasswordSize", &self.MinUserPasswordSize)
            .field("MaxUserHintSize", &self.MaxUserHintSize)
            .field("MaxUserNameSize", &self.MaxUserNameSize)
            .field("MaxSiloNameSize", &self.MaxSiloNameSize)
            .field("MaxChallengeSize", &self.MaxChallengeSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentAdminFailures == other.CurrentAdminFailures
            && self.CurrentUserFailures == other.CurrentUserFailures
            && self.TotalUserAuthenticationCount == other.TotalUserAuthenticationCount
            && self.TotalAdminAuthenticationCount == other.TotalAdminAuthenticationCount
            && self.FipsCompliant == other.FipsCompliant
            && self.SecurityIDAvailable == other.SecurityIDAvailable
            && self.InitializeInProgress == other.InitializeInProgress
            && self.ITMSArmed == other.ITMSArmed
            && self.ITMSArmable == other.ITMSArmable
            && self.UserCreated == other.UserCreated
            && self.ResetOnPORDefault == other.ResetOnPORDefault
            && self.ResetOnPORCurrent == other.ResetOnPORCurrent
            && self.MaxAdminFailures == other.MaxAdminFailures
            && self.MaxUserFailures == other.MaxUserFailures
            && self.TimeToCompleteInitialization == other.TimeToCompleteInitialization
            && self.TimeRemainingToCompleteInitialization == other.TimeRemainingToCompleteInitialization
            && self.MinTimeToAuthenticate == other.MinTimeToAuthenticate
            && self.MaxAdminPasswordSize == other.MaxAdminPasswordSize
            && self.MinAdminPasswordSize == other.MinAdminPasswordSize
            && self.MaxAdminHintSize == other.MaxAdminHintSize
            && self.MaxUserPasswordSize == other.MaxUserPasswordSize
            && self.MinUserPasswordSize == other.MinUserPasswordSize
            && self.MaxUserHintSize == other.MaxUserHintSize
            && self.MaxUserNameSize == other.MaxUserNameSize
            && self.MaxSiloNameSize == other.MaxSiloNameSize
            && self.MaxChallengeSize == other.MaxChallengeSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_AUTHN_ERROR_END: u32 = 1279u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_AUTHN_ERROR_START: u32 = 1024u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_GENERAL_ERROR_END: u32 = 1023u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_GENERAL_ERROR_START: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_PW_SILO_ERROR_END: u32 = 4607u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_PW_SILO_ERROR_START: u32 = 4352u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_COM_ERROR_END: u32 = 511u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_COM_ERROR_START: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_SILO_ERROR_END: u32 = 4095u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_SILO_ERROR_START: u32 = 1280u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_SILO_SPECIFIC_ERROR_END: u32 = 49151u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_SILO_SPECIFIC_ERROR_START: u32 = 4608u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_VENDOR_ERROR_END: u32 = 65535u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_VENDOR_ERROR_START: u32 = 49152u32;
pub const EnhancedStorageACT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2936498709, 11982, 19156, [187, 33, 41, 240, 64, 225, 118, 216]);
pub const EnhancedStorageSilo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3408208396, 30407, 20462, [132, 43, 243, 56, 60, 208, 34, 188]);
pub const EnhancedStorageSiloAction: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2288855517, 46342, 18027, [159, 191, 180, 79, 243, 131, 251, 63]);
pub const EnumEnhancedStorageACT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4270068883, 33628, 20387, [182, 204, 180, 178, 212, 113, 152, 72]);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FACILITY_ENHANCED_STORAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_COMPLETE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_COMPLETE_PINNED: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_EXCLUDED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_FOLDER_EMPTY: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_NOTAVAILABLEOFFLINE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_PARTIAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FLAGSTATUS_COMPLETED: i32 = 1i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FLAGSTATUS_FOLLOWUP: i32 = 2i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FLAGSTATUS_NOTFLAGGED: i32 = 0i32;
pub const GUID_DEVINTERFACE_ENHANCED_STORAGE_SILO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(949483172, 64821, 19400, [160, 183, 93, 187, 163, 106, 218, 250]);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnhancedStorageACT(pub ::windows::runtime::IUnknown);
impl IEnhancedStorageACT {
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Unauthorize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetAuthorizationState(&self) -> ::windows::runtime::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__: <ACT_AUTHORIZATION_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ACT_AUTHORIZATION_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetMatchingVolume(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppienhancedstoragesilos), ::core::mem::transmute(pcenhancedstoragesilos)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnhancedStorageACT {
    type Vtable = IEnhancedStorageACT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1853325812, 57586, 16953, [185, 118, 160, 26, 186, 181, 41, 48]);
}
impl ::core::convert::From<IEnhancedStorageACT> for ::windows::runtime::IUnknown {
    fn from(value: IEnhancedStorageACT) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnhancedStorageACT> for ::windows::runtime::IUnknown {
    fn from(value: &IEnhancedStorageACT) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnhancedStorageACT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnhancedStorageACT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageACT_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszvolume: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszidentity: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppienhancedstoragesilos: *mut *mut ::windows::runtime::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnhancedStorageACT2(pub ::windows::runtime::IUnknown);
impl IEnhancedStorageACT2 {
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Unauthorize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetAuthorizationState(&self) -> ::windows::runtime::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__: <ACT_AUTHORIZATION_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ACT_AUTHORIZATION_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetMatchingVolume(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppienhancedstoragesilos), ::core::mem::transmute(pcenhancedstoragesilos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn IsDeviceRemovable(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnhancedStorageACT2 {
    type Vtable = IEnhancedStorageACT2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1302691118, 36531, 16886, [160, 126, 152, 181, 43, 136, 36, 43]);
}
impl ::core::convert::From<IEnhancedStorageACT2> for ::windows::runtime::IUnknown {
    fn from(value: IEnhancedStorageACT2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnhancedStorageACT2> for ::windows::runtime::IUnknown {
    fn from(value: &IEnhancedStorageACT2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IEnhancedStorageACT2> for IEnhancedStorageACT {
    fn from(value: IEnhancedStorageACT2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT2> for IEnhancedStorageACT {
    fn from(value: &IEnhancedStorageACT2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT> for IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT> for &IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageACT2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszvolume: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszidentity: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppienhancedstoragesilos: *mut *mut ::windows::runtime::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszdevicename: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnhancedStorageACT3(pub ::windows::runtime::IUnknown);
impl IEnhancedStorageACT3 {
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Unauthorize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetAuthorizationState(&self) -> ::windows::runtime::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__: <ACT_AUTHORIZATION_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ACT_AUTHORIZATION_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetMatchingVolume(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppienhancedstoragesilos), ::core::mem::transmute(pcenhancedstoragesilos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn IsDeviceRemovable(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn UnauthorizeEx(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn IsQueueFrozen(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetShellExtSupport(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnhancedStorageACT3 {
    type Vtable = IEnhancedStorageACT3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(35737761, 4413, 4575, [187, 97, 0, 26, 160, 27, 188, 88]);
}
impl ::core::convert::From<IEnhancedStorageACT3> for ::windows::runtime::IUnknown {
    fn from(value: IEnhancedStorageACT3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnhancedStorageACT3> for ::windows::runtime::IUnknown {
    fn from(value: &IEnhancedStorageACT3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IEnhancedStorageACT3> for IEnhancedStorageACT2 {
    fn from(value: IEnhancedStorageACT3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT3> for IEnhancedStorageACT2 {
    fn from(value: &IEnhancedStorageACT3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT2> for IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT2> for &IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEnhancedStorageACT3> for IEnhancedStorageACT {
    fn from(value: IEnhancedStorageACT3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT3> for IEnhancedStorageACT {
    fn from(value: &IEnhancedStorageACT3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT> for IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT> for &IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageACT3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszvolume: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszidentity: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppienhancedstoragesilos: *mut *mut ::windows::runtime::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszdevicename: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisqueuefrozen: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pshellextsupport: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnhancedStorageSilo(pub ::windows::runtime::IUnknown);
impl IEnhancedStorageSilo {
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetInfo(&self) -> ::windows::runtime::Result<SILO_INFO> {
        let mut result__: <SILO_INFO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<SILO_INFO>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetActions(&self, pppienhancedstoragesiloactions: *mut *mut ::core::option::Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppienhancedstoragesiloactions), ::core::mem::transmute(pcenhancedstoragesiloactions)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn SendCommand(&self, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(command), ::core::mem::transmute(pbcommandbuffer), ::core::mem::transmute(cbcommandbuffer), ::core::mem::transmute(pbresponsebuffer), ::core::mem::transmute(pcbresponsebuffer)).ok()
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetPortableDevice(&self) -> ::windows::runtime::Result<super::super::Devices::PortableDevices::IPortableDevice> {
        let mut result__: <super::super::Devices::PortableDevices::IPortableDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Devices::PortableDevices::IPortableDevice>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetDevicePath(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnhancedStorageSilo {
    type Vtable = IEnhancedStorageSilo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1525643462, 8770, 18179, [191, 73, 68, 178, 147, 87, 163, 89]);
}
impl ::core::convert::From<IEnhancedStorageSilo> for ::windows::runtime::IUnknown {
    fn from(value: IEnhancedStorageSilo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnhancedStorageSilo> for ::windows::runtime::IUnknown {
    fn from(value: &IEnhancedStorageSilo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnhancedStorageSilo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnhancedStorageSilo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageSilo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psiloinfo: *mut SILO_INFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppienhancedstoragesiloactions: *mut *mut ::windows::runtime::RawPtr, pcenhancedstoragesiloactions: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Devices_PortableDevices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiportabledevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszsilodevicepath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnhancedStorageSiloAction(pub ::windows::runtime::IUnknown);
impl IEnhancedStorageSiloAction {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Invoke(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnhancedStorageSiloAction {
    type Vtable = IEnhancedStorageSiloAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3069702929, 8303, 20472, [156, 75, 39, 239, 238, 119, 168, 111]);
}
impl ::core::convert::From<IEnhancedStorageSiloAction> for ::windows::runtime::IUnknown {
    fn from(value: IEnhancedStorageSiloAction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnhancedStorageSiloAction> for ::windows::runtime::IUnknown {
    fn from(value: &IEnhancedStorageSiloAction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnhancedStorageSiloAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnhancedStorageSiloAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageSiloAction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszactionname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszactiondescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumEnhancedStorageACT(pub ::windows::runtime::IUnknown);
impl IEnumEnhancedStorageACT {
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetACTs(&self, pppienhancedstorageacts: *mut *mut ::core::option::Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppienhancedstorageacts), ::core::mem::transmute(pcenhancedstorageacts)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetMatchingACT<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, szvolume: Param0) -> ::windows::runtime::Result<IEnhancedStorageACT> {
        let mut result__: <IEnhancedStorageACT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), szvolume.into_param().abi(), &mut result__).from_abi::<IEnhancedStorageACT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumEnhancedStorageACT {
    type Vtable = IEnumEnhancedStorageACT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(162669757, 4917, 17969, [167, 255, 207, 211, 169, 38, 70, 215]);
}
impl ::core::convert::From<IEnumEnhancedStorageACT> for ::windows::runtime::IUnknown {
    fn from(value: IEnumEnhancedStorageACT) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumEnhancedStorageACT> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumEnhancedStorageACT) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumEnhancedStorageACT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumEnhancedStorageACT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumEnhancedStorageACT_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppienhancedstorageacts: *mut *mut ::windows::runtime::RawPtr, pcenhancedstorageacts: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szvolume: super::super::Foundation::PWSTR, ppienhancedstorageact: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_HIGH_MAX: i32 = 5i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_HIGH_MIN: i32 = 5i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_HIGH_SET: i32 = 5i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_LOW_MAX: i32 = 1i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_LOW_MIN: i32 = 0i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_LOW_SET: i32 = 1i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_NORMAL_MAX: i32 = 4i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_NORMAL_MIN: i32 = 2i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_NORMAL_SET: i32 = 3i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ISDEFAULTSAVE_BOTH: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ISDEFAULTSAVE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ISDEFAULTSAVE_NONOWNER: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ISDEFAULTSAVE_OWNER: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const LINK_STATUS_BROKEN: i32 = 2i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const LINK_STATUS_RESOLVED: i32 = 1i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINEAVAILABILITY_ALWAYS_AVAILABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINEAVAILABILITY_AVAILABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINEAVAILABILITY_NOT_AVAILABLE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE_ERROR: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE_FORCED: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE_ITEM_VERSION_CONFLICT: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE_SLOW: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE_SUSPENDED: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_ONLINE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_CONTRAST_HARD: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_CONTRAST_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_CONTRAST_SOFT: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_ACTION: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_APERTURE: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_CREATIVE: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_LANDSCAPE: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_MANUAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_NORMAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_PORTRAIT: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_SHUTTER: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO: u32 = 25u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO_NORETURNLIGHT: u32 = 29u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE: u32 = 89u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE_NORETURNLIGHT: u32 = 93u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE_RETURNLIGHT: u32 = 95u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO_RETURNLIGHT: u32 = 31u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY: u32 = 9u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY_NORETURNLIGHT: u32 = 13u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE: u32 = 73u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE_NORETURNLIGHT: u32 = 77u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE_RETURNLIGHT: u32 = 79u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY_RETURNLIGHT: u32 = 15u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_REDEYE: u32 = 65u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_REDEYE_NORETURNLIGHT: u32 = 69u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_REDEYE_RETURNLIGHT: u32 = 71u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_NOFUNCTION: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_NONE_AUTO: u32 = 24u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_NONE_COMPULSORY: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_WITHOUTSTROBE: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_WITHSTROBE: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_GAINCONTROL_HIGHGAINDOWN: f64 = 4f64;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_GAINCONTROL_HIGHGAINUP: f64 = 2f64;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_GAINCONTROL_LOWGAINDOWN: f64 = 3f64;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_GAINCONTROL_LOWGAINUP: f64 = 1f64;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_GAINCONTROL_NONE: f64 = 0f64;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_D55: u32 = 20u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_D65: u32 = 21u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_D75: u32 = 22u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_DAYLIGHT: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_FLUORESCENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_STANDARD_A: u32 = 17u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_STANDARD_B: u32 = 18u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_STANDARD_C: u32 = 19u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_TUNGSTEN: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_ACTION: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_APERTURE: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_CREATIVE: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_LANDSCAPE: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_MANUAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_NORMAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_NOTDEFINED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_PORTRAIT: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_SHUTTER: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SATURATION_HIGH: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SATURATION_LOW: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SATURATION_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SHARPNESS_HARD: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SHARPNESS_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SHARPNESS_SOFT: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_WHITEBALANCE_AUTO: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_WHITEBALANCE_MANUAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_NOMEDIA: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_PAUSED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_PLAYING: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_RECORDING: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_RECORDINGPAUSED: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_STOPPED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_TRANSITIONING: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FIVE_STARS_MAX: u32 = 99u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FIVE_STARS_MIN: u32 = 88u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FIVE_STARS_SET: u32 = 99u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FOUR_STARS_MAX: u32 = 87u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FOUR_STARS_MIN: u32 = 63u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FOUR_STARS_SET: u32 = 75u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_ONE_STAR_MAX: u32 = 12u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_ONE_STAR_MIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_ONE_STAR_SET: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_THREE_STARS_MAX: u32 = 62u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_THREE_STARS_MIN: u32 = 38u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_THREE_STARS_SET: u32 = 50u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_TWO_STARS_MAX: u32 = 37u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_TWO_STARS_MIN: u32 = 13u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_TWO_STARS_SET: u32 = 25u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SHARINGSTATUS_NOTSHARED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SHARINGSTATUS_PRIVATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SHARINGSTATUS_SHARED: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub struct SILO_INFO {
    pub ulSTID: u32,
    pub SpecificationMajor: u8,
    pub SpecificationMinor: u8,
    pub ImplementationMajor: u8,
    pub ImplementationMinor: u8,
    pub r#type: u8,
    pub capabilities: u8,
}
impl SILO_INFO {}
impl ::core::default::Default for SILO_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SILO_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SILO_INFO")
            .field("ulSTID", &self.ulSTID)
            .field("SpecificationMajor", &self.SpecificationMajor)
            .field("SpecificationMinor", &self.SpecificationMinor)
            .field("ImplementationMajor", &self.ImplementationMajor)
            .field("ImplementationMinor", &self.ImplementationMinor)
            .field("r#type", &self.r#type)
            .field("capabilities", &self.capabilities)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SILO_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulSTID == other.ulSTID && self.SpecificationMajor == other.SpecificationMajor && self.SpecificationMinor == other.SpecificationMinor && self.ImplementationMajor == other.ImplementationMajor && self.ImplementationMinor == other.ImplementationMinor && self.r#type == other.r#type && self.capabilities == other.capabilities
    }
}
impl ::core::cmp::Eq for SILO_INFO {}
unsafe impl ::windows::runtime::Abi for SILO_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_NOTSHARED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_PRIVATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC_COOWNED: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC_OWNED: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED_COOWNED: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED_OWNED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_COMPUTERS: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_DEVICES: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_FOLDERS: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_OTHER: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_PROGRAMS: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_WEBSERVICES: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_ERROR: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_IDLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_NOTSETUP: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_PENDING: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_SYNCING: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_SYNCNOTRUN: u32 = 1u32;
pub const WPD_CATEGORY_ENHANCED_STORAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]);