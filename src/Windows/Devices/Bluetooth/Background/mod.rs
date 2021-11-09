#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothEventTriggeringMode(pub i32);
impl BluetoothEventTriggeringMode {
    pub const Serial: BluetoothEventTriggeringMode = BluetoothEventTriggeringMode(0i32);
    pub const Batch: BluetoothEventTriggeringMode = BluetoothEventTriggeringMode(1i32);
    pub const KeepLatest: BluetoothEventTriggeringMode = BluetoothEventTriggeringMode(2i32);
}
impl ::core::convert::From<i32> for BluetoothEventTriggeringMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothEventTriggeringMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothEventTriggeringMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Background.BluetoothEventTriggeringMode;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothEventTriggeringMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementPublisherTriggerDetails(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementPublisherTriggerDetails {
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Advertisement`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::Advertisement::BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: super::Advertisement::BluetoothLEAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Advertisement::BluetoothLEAdvertisementPublisherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Error(&self) -> ::windows::runtime::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Foundation`*"]
    pub fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementPublisherTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i16>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementPublisherTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails;{610eca86-3480-41c9-a918-7ddadf207e00})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementPublisherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1628359302, 13440, 16841, [169, 24, 125, 218, 223, 32, 126, 0]);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementPublisherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementPublisherTriggerDetails";
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisementPublisherTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisementPublisherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherTriggerDetails {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherTriggerDetails {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementWatcherTriggerDetails(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementWatcherTriggerDetails {
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Error(&self) -> ::windows::runtime::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Advertisement`, `Foundation_Collections`*"]
    pub fn Advertisements(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn SignalStrengthFilter(&self) -> ::windows::runtime::Result<super::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothSignalStrengthFilter>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementWatcherTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails;{a7db5ad7-2257-4e69-9784-fee645c1dce0})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementWatcherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2816170711, 8791, 20073, [151, 132, 254, 230, 69, 193, 220, 224]);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.BluetoothLEAdvertisementWatcherTriggerDetails";
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisementWatcherTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisementWatcherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherTriggerDetails {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherTriggerDetails {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattCharacteristicNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
impl GattCharacteristicNotificationTriggerDetails {
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Characteristic(&self) -> ::windows::runtime::Result<super::GenericAttributeProfile::GattCharacteristic> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::GenericAttributeProfile::GattCharacteristic>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Storage_Streams`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Error(&self) -> ::windows::runtime::Result<super::BluetoothError> {
        let this = &::windows::runtime::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn EventTriggeringMode(&self) -> ::windows::runtime::Result<BluetoothEventTriggeringMode> {
        let this = &::windows::runtime::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: BluetoothEventTriggeringMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothEventTriggeringMode>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn ValueChangedEvents(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>> {
        let this = &::windows::runtime::Interface::cast::<IGattCharacteristicNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GattCharacteristicNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails;{9ba03b18-0fec-436a-93b1-f46c697532a2})");
}
unsafe impl ::windows::runtime::Interface for GattCharacteristicNotificationTriggerDetails {
    type Vtable = IGattCharacteristicNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2610969368, 4076, 17258, [147, 177, 244, 108, 105, 117, 50, 162]);
}
impl ::windows::runtime::RuntimeName for GattCharacteristicNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattCharacteristicNotificationTriggerDetails";
}
impl ::core::convert::From<GattCharacteristicNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: GattCharacteristicNotificationTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattCharacteristicNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &GattCharacteristicNotificationTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattCharacteristicNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: GattCharacteristicNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattCharacteristicNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &GattCharacteristicNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GattCharacteristicNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattCharacteristicNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for GattCharacteristicNotificationTriggerDetails {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattServiceProviderConnection(pub ::windows::runtime::IInspectable);
impl GattServiceProviderConnection {
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn TriggerId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Service(&self) -> ::windows::runtime::Result<super::GenericAttributeProfile::GattLocalService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::GenericAttributeProfile::GattLocalService>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Foundation_Collections`*"]
    pub fn AllServices() -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, GattServiceProviderConnection>> {
        Self::IGattServiceProviderConnectionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, GattServiceProviderConnection>>(result__)
        })
    }
    pub fn IGattServiceProviderConnectionStatics<R, F: FnOnce(&IGattServiceProviderConnectionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GattServiceProviderConnection, IGattServiceProviderConnectionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GattServiceProviderConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattServiceProviderConnection;{7fa1b9b9-2f13-40b5-9582-8eb78e98ef13})");
}
unsafe impl ::windows::runtime::Interface for GattServiceProviderConnection {
    type Vtable = IGattServiceProviderConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2141305273, 12051, 16565, [149, 130, 142, 183, 142, 152, 239, 19]);
}
impl ::windows::runtime::RuntimeName for GattServiceProviderConnection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattServiceProviderConnection";
}
impl ::core::convert::From<GattServiceProviderConnection> for ::windows::runtime::IUnknown {
    fn from(value: GattServiceProviderConnection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattServiceProviderConnection> for ::windows::runtime::IUnknown {
    fn from(value: &GattServiceProviderConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GattServiceProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GattServiceProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattServiceProviderConnection> for ::windows::runtime::IInspectable {
    fn from(value: GattServiceProviderConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattServiceProviderConnection> for ::windows::runtime::IInspectable {
    fn from(value: &GattServiceProviderConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GattServiceProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GattServiceProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderConnection {}
unsafe impl ::core::marker::Sync for GattServiceProviderConnection {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattServiceProviderTriggerDetails(pub ::windows::runtime::IInspectable);
impl GattServiceProviderTriggerDetails {
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Connection(&self) -> ::windows::runtime::Result<GattServiceProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattServiceProviderConnection>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GattServiceProviderTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails;{ae8c0625-05ff-4afb-b16a-de95f3cf0158})");
}
unsafe impl ::windows::runtime::Interface for GattServiceProviderTriggerDetails {
    type Vtable = IGattServiceProviderTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2928412197, 1535, 19195, [177, 106, 222, 149, 243, 207, 1, 88]);
}
impl ::windows::runtime::RuntimeName for GattServiceProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.GattServiceProviderTriggerDetails";
}
impl ::core::convert::From<GattServiceProviderTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: GattServiceProviderTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattServiceProviderTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &GattServiceProviderTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattServiceProviderTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: GattServiceProviderTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattServiceProviderTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &GattServiceProviderTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GattServiceProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for GattServiceProviderTriggerDetails {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementPublisherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1628359302, 13440, 16841, [169, 24, 125, 218, 223, 32, 126, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Advertisement::BluetoothLEAdvertisementPublisherStatus) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::BluetoothError) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    type Vtable = IBluetoothLEAdvertisementPublisherTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3567505445, 50689, 17110, [152, 41, 76, 203, 63, 92, 215, 127]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementWatcherTriggerDetails {
    type Vtable = IBluetoothLEAdvertisementWatcherTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2816170711, 8791, 20073, [151, 132, 254, 230, 69, 193, 220, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::BluetoothError) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattCharacteristicNotificationTriggerDetails {
    type Vtable = IGattCharacteristicNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2610969368, 4076, 17258, [147, 177, 244, 108, 105, 117, 50, 162]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattCharacteristicNotificationTriggerDetails2 {
    type Vtable = IGattCharacteristicNotificationTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1920618716, 38045, 17738, [177, 146, 152, 52, 103, 227, 213, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::BluetoothError) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothEventTriggeringMode) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderConnection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattServiceProviderConnection {
    type Vtable = IGattServiceProviderConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2141305273, 12051, 16565, [149, 130, 142, 183, 142, 152, 239, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderConnectionStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattServiceProviderConnectionStatics {
    type Vtable = IGattServiceProviderConnectionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1028693835, 2830, 17510, [184, 205, 110, 189, 218, 31, 161, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderConnectionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGattServiceProviderTriggerDetails {
    type Vtable = IGattServiceProviderTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2928412197, 1535, 19195, [177, 106, 222, 149, 243, 207, 1, 88]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRfcommConnectionTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommConnectionTriggerDetails {
    type Vtable = IRfcommConnectionTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4179784525, 11836, 20220, [171, 89, 252, 92, 249, 111, 151, 227]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommConnectionTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRfcommInboundConnectionInformation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommInboundConnectionInformation {
    type Vtable = IRfcommInboundConnectionInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1832809896, 21545, 16473, [146, 227, 30, 139, 101, 82, 135, 7]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommInboundConnectionInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::BluetoothServiceCapabilities) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::BluetoothServiceCapabilities) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRfcommOutboundConnectionInformation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommOutboundConnectionInformation {
    type Vtable = IRfcommOutboundConnectionInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2962301563, 62516, 19632, [153, 177, 74, 184, 206, 218, 237, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommOutboundConnectionInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))] usize,
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Rfcomm"))] usize,
);
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RfcommConnectionTriggerDetails(pub ::windows::runtime::IInspectable);
impl RfcommConnectionTriggerDetails {
    #[cfg(feature = "Networking_Sockets")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Networking_Sockets`*"]
    pub fn Socket(&self) -> ::windows::runtime::Result<super::super::super::Networking::Sockets::StreamSocket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Networking::Sockets::StreamSocket>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn Incoming(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn RemoteDevice(&self) -> ::windows::runtime::Result<super::BluetoothDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothDevice>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RfcommConnectionTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails;{f922734d-2e3c-4efc-ab59-fc5cf96f97e3})");
}
unsafe impl ::windows::runtime::Interface for RfcommConnectionTriggerDetails {
    type Vtable = IRfcommConnectionTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4179784525, 11836, 20220, [171, 89, 252, 92, 249, 111, 151, 227]);
}
impl ::windows::runtime::RuntimeName for RfcommConnectionTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommConnectionTriggerDetails";
}
impl ::core::convert::From<RfcommConnectionTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: RfcommConnectionTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RfcommConnectionTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &RfcommConnectionTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RfcommConnectionTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: RfcommConnectionTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RfcommConnectionTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &RfcommConnectionTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RfcommConnectionTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RfcommConnectionTriggerDetails {}
unsafe impl ::core::marker::Sync for RfcommConnectionTriggerDetails {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RfcommInboundConnectionInformation(pub ::windows::runtime::IInspectable);
impl RfcommInboundConnectionInformation {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Storage_Streams`*"]
    pub fn SdpRecord(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Storage_Streams`*"]
    pub fn SetSdpRecord<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Rfcomm`*"]
    pub fn LocalServiceId(&self) -> ::windows::runtime::Result<super::Rfcomm::RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Rfcomm::RfcommServiceId>(result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Rfcomm`*"]
    pub fn SetLocalServiceId<'a, Param0: ::windows::runtime::IntoParam<'a, super::Rfcomm::RfcommServiceId>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn ServiceCapabilities(&self) -> ::windows::runtime::Result<super::BluetoothServiceCapabilities> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothServiceCapabilities = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothServiceCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Background`*"]
    pub fn SetServiceCapabilities(&self, value: super::BluetoothServiceCapabilities) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RfcommInboundConnectionInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation;{6d3e75a8-5429-4059-92e3-1e8b65528707})");
}
unsafe impl ::windows::runtime::Interface for RfcommInboundConnectionInformation {
    type Vtable = IRfcommInboundConnectionInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1832809896, 21545, 16473, [146, 227, 30, 139, 101, 82, 135, 7]);
}
impl ::windows::runtime::RuntimeName for RfcommInboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommInboundConnectionInformation";
}
impl ::core::convert::From<RfcommInboundConnectionInformation> for ::windows::runtime::IUnknown {
    fn from(value: RfcommInboundConnectionInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RfcommInboundConnectionInformation> for ::windows::runtime::IUnknown {
    fn from(value: &RfcommInboundConnectionInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RfcommInboundConnectionInformation> for ::windows::runtime::IInspectable {
    fn from(value: RfcommInboundConnectionInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RfcommInboundConnectionInformation> for ::windows::runtime::IInspectable {
    fn from(value: &RfcommInboundConnectionInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RfcommInboundConnectionInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RfcommInboundConnectionInformation {}
unsafe impl ::core::marker::Sync for RfcommInboundConnectionInformation {}
#[doc = "*Required features: `Devices_Bluetooth_Background`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RfcommOutboundConnectionInformation(pub ::windows::runtime::IInspectable);
impl RfcommOutboundConnectionInformation {
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Rfcomm`*"]
    pub fn RemoteServiceId(&self) -> ::windows::runtime::Result<super::Rfcomm::RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Rfcomm::RfcommServiceId>(result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Rfcomm")]
    #[doc = "*Required features: `Devices_Bluetooth_Background`, `Devices_Bluetooth_Rfcomm`*"]
    pub fn SetRemoteServiceId<'a, Param0: ::windows::runtime::IntoParam<'a, super::Rfcomm::RfcommServiceId>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RfcommOutboundConnectionInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation;{b091227b-f434-4cb0-99b1-4ab8cedaedd7})");
}
unsafe impl ::windows::runtime::Interface for RfcommOutboundConnectionInformation {
    type Vtable = IRfcommOutboundConnectionInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2962301563, 62516, 19632, [153, 177, 74, 184, 206, 218, 237, 215]);
}
impl ::windows::runtime::RuntimeName for RfcommOutboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.RfcommOutboundConnectionInformation";
}
impl ::core::convert::From<RfcommOutboundConnectionInformation> for ::windows::runtime::IUnknown {
    fn from(value: RfcommOutboundConnectionInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RfcommOutboundConnectionInformation> for ::windows::runtime::IUnknown {
    fn from(value: &RfcommOutboundConnectionInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RfcommOutboundConnectionInformation> for ::windows::runtime::IInspectable {
    fn from(value: RfcommOutboundConnectionInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RfcommOutboundConnectionInformation> for ::windows::runtime::IInspectable {
    fn from(value: &RfcommOutboundConnectionInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RfcommOutboundConnectionInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RfcommOutboundConnectionInformation {}
unsafe impl ::core::marker::Sync for RfcommOutboundConnectionInformation {}