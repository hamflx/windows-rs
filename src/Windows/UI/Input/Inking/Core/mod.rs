#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Input_Inking_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreIncrementalInkStroke(pub ::windows::runtime::IInspectable);
impl CoreIncrementalInkStroke {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`, `Foundation_Collections`*"]
    pub fn AppendInkPoints<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::InkPoint>>>(&self, inkpoints: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpoints.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking_Core`*"]
    pub fn CreateInkStroke(&self) -> ::windows::runtime::Result<super::InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkStroke>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking_Core`*"]
    pub fn DrawingAttributes(&self) -> ::windows::runtime::Result<super::InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkDrawingAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation_Numerics`*"]
    pub fn PointTransform(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation_Numerics`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::InkDrawingAttributes>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Numerics::Matrix3x2>>(drawingattributes: Param0, pointtransform: Param1) -> ::windows::runtime::Result<CoreIncrementalInkStroke> {
        Self::ICoreIncrementalInkStrokeFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), drawingattributes.into_param().abi(), pointtransform.into_param().abi(), &mut result__).from_abi::<CoreIncrementalInkStroke>(result__)
        })
    }
    pub fn ICoreIncrementalInkStrokeFactory<R, F: FnOnce(&ICoreIncrementalInkStrokeFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreIncrementalInkStroke, ICoreIncrementalInkStrokeFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreIncrementalInkStroke {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke;{fda015d3-9d66-4f7d-a57f-cc70b9cfaa76})");
}
unsafe impl ::windows::runtime::Interface for CoreIncrementalInkStroke {
    type Vtable = ICoreIncrementalInkStroke_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4255126995, 40294, 20349, [165, 127, 204, 112, 185, 207, 170, 118]);
}
impl ::windows::runtime::RuntimeName for CoreIncrementalInkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke";
}
impl ::core::convert::From<CoreIncrementalInkStroke> for ::windows::runtime::IUnknown {
    fn from(value: CoreIncrementalInkStroke) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreIncrementalInkStroke> for ::windows::runtime::IUnknown {
    fn from(value: &CoreIncrementalInkStroke) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreIncrementalInkStroke> for ::windows::runtime::IInspectable {
    fn from(value: CoreIncrementalInkStroke) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreIncrementalInkStroke> for ::windows::runtime::IInspectable {
    fn from(value: &CoreIncrementalInkStroke) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreIncrementalInkStroke {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreIncrementalInkStroke {}
unsafe impl ::core::marker::Sync for CoreIncrementalInkStroke {}
#[doc = "*Required features: `UI_Input_Inking_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInkIndependentInputSource(pub ::windows::runtime::IInspectable);
impl CoreInkIndependentInputSource {
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`, `UI_Core`*"]
    pub fn PointerEntering<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemovePointerEntering<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`, `UI_Core`*"]
    pub fn PointerHovering<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemovePointerHovering<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`, `UI_Core`*"]
    pub fn PointerExiting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemovePointerExiting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`, `UI_Core`*"]
    pub fn PointerPressing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemovePointerPressing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`, `UI_Core`*"]
    pub fn PointerMoving<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemovePointerMoving<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`, `UI_Core`*"]
    pub fn PointerReleasing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemovePointerReleasing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`, `UI_Core`*"]
    pub fn PointerLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemovePointerLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking_Core`*"]
    pub fn InkPresenter(&self) -> ::windows::runtime::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkPresenter>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking_Core`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::InkPresenter>>(inkpresenter: Param0) -> ::windows::runtime::Result<CoreInkIndependentInputSource> {
        Self::ICoreInkIndependentInputSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<CoreInkIndependentInputSource>(result__)
        })
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `UI_Core`*"]
    pub fn PointerCursor(&self) -> ::windows::runtime::Result<super::super::super::Core::CoreCursor> {
        let this = &::windows::runtime::Interface::cast::<ICoreInkIndependentInputSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Core::CoreCursor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `UI_Core`*"]
    pub fn SetPointerCursor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Core::CoreCursor>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreInkIndependentInputSource2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ICoreInkIndependentInputSourceStatics<R, F: FnOnce(&ICoreInkIndependentInputSourceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreInkIndependentInputSource, ICoreInkIndependentInputSourceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreInkIndependentInputSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource;{39b38da9-7639-4499-a5b5-191d00e35b16})");
}
unsafe impl ::windows::runtime::Interface for CoreInkIndependentInputSource {
    type Vtable = ICoreInkIndependentInputSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(968068521, 30265, 17561, [165, 181, 25, 29, 0, 227, 91, 22]);
}
impl ::windows::runtime::RuntimeName for CoreInkIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource";
}
impl ::core::convert::From<CoreInkIndependentInputSource> for ::windows::runtime::IUnknown {
    fn from(value: CoreInkIndependentInputSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInkIndependentInputSource> for ::windows::runtime::IUnknown {
    fn from(value: &CoreInkIndependentInputSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInkIndependentInputSource> for ::windows::runtime::IInspectable {
    fn from(value: CoreInkIndependentInputSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInkIndependentInputSource> for ::windows::runtime::IInspectable {
    fn from(value: &CoreInkIndependentInputSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreInkIndependentInputSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInkIndependentInputSource {}
unsafe impl ::core::marker::Sync for CoreInkIndependentInputSource {}
#[doc = "*Required features: `UI_Input_Inking_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreInkPresenterHost(pub ::windows::runtime::IInspectable);
impl CoreInkPresenterHost {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreInkPresenterHost, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Input_Inking_Core`*"]
    pub fn InkPresenter(&self) -> ::windows::runtime::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkPresenter>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `UI_Composition`*"]
    pub fn RootVisual(&self) -> ::windows::runtime::Result<super::super::super::Composition::ContainerVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Composition::ContainerVisual>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `UI_Composition`*"]
    pub fn SetRootVisual<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::ContainerVisual>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreInkPresenterHost {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreInkPresenterHost;{396e89e6-7d55-4617-9e58-68c70c9169b9})");
}
unsafe impl ::windows::runtime::Interface for CoreInkPresenterHost {
    type Vtable = ICoreInkPresenterHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(963545574, 32085, 17943, [158, 88, 104, 199, 12, 145, 105, 185]);
}
impl ::windows::runtime::RuntimeName for CoreInkPresenterHost {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreInkPresenterHost";
}
impl ::core::convert::From<CoreInkPresenterHost> for ::windows::runtime::IUnknown {
    fn from(value: CoreInkPresenterHost) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreInkPresenterHost> for ::windows::runtime::IUnknown {
    fn from(value: &CoreInkPresenterHost) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreInkPresenterHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreInkPresenterHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreInkPresenterHost> for ::windows::runtime::IInspectable {
    fn from(value: CoreInkPresenterHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreInkPresenterHost> for ::windows::runtime::IInspectable {
    fn from(value: &CoreInkPresenterHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreInkPresenterHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreInkPresenterHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreInkPresenterHost {}
unsafe impl ::core::marker::Sync for CoreInkPresenterHost {}
#[doc = "*Required features: `UI_Input_Inking_Core`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreWetStrokeDisposition(pub i32);
impl CoreWetStrokeDisposition {
    pub const Inking: CoreWetStrokeDisposition = CoreWetStrokeDisposition(0i32);
    pub const Completed: CoreWetStrokeDisposition = CoreWetStrokeDisposition(1i32);
    pub const Canceled: CoreWetStrokeDisposition = CoreWetStrokeDisposition(2i32);
}
impl ::core::convert::From<i32> for CoreWetStrokeDisposition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreWetStrokeDisposition {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreWetStrokeDisposition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Core.CoreWetStrokeDisposition;i4)");
}
impl ::windows::runtime::DefaultType for CoreWetStrokeDisposition {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Input_Inking_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreWetStrokeUpdateEventArgs(pub ::windows::runtime::IInspectable);
impl CoreWetStrokeUpdateEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation_Collections`*"]
    pub fn NewInkPoints(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVector<super::InkPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<super::InkPoint>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking_Core`*"]
    pub fn PointerId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking_Core`*"]
    pub fn Disposition(&self) -> ::windows::runtime::Result<CoreWetStrokeDisposition> {
        let this = self;
        unsafe {
            let mut result__: CoreWetStrokeDisposition = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreWetStrokeDisposition>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking_Core`*"]
    pub fn SetDisposition(&self, value: CoreWetStrokeDisposition) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWetStrokeUpdateEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs;{fb07d14c-3380-457a-a987-991357896c1b})");
}
unsafe impl ::windows::runtime::Interface for CoreWetStrokeUpdateEventArgs {
    type Vtable = ICoreWetStrokeUpdateEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4211593548, 13184, 17786, [169, 135, 153, 19, 87, 137, 108, 27]);
}
impl ::windows::runtime::RuntimeName for CoreWetStrokeUpdateEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs";
}
impl ::core::convert::From<CoreWetStrokeUpdateEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreWetStrokeUpdateEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWetStrokeUpdateEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWetStrokeUpdateEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreWetStrokeUpdateEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWetStrokeUpdateEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreWetStrokeUpdateEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWetStrokeUpdateEventArgs {}
unsafe impl ::core::marker::Sync for CoreWetStrokeUpdateEventArgs {}
#[doc = "*Required features: `UI_Input_Inking_Core`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreWetStrokeUpdateSource(pub ::windows::runtime::IInspectable);
impl CoreWetStrokeUpdateSource {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn WetStrokeStarting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemoveWetStrokeStarting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn WetStrokeContinuing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemoveWetStrokeContinuing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn WetStrokeStopping<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemoveWetStrokeStopping<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn WetStrokeCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemoveWetStrokeCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn WetStrokeCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking_Core`, `Foundation`*"]
    pub fn RemoveWetStrokeCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking_Core`*"]
    pub fn InkPresenter(&self) -> ::windows::runtime::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::InkPresenter>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking_Core`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::InkPresenter>>(inkpresenter: Param0) -> ::windows::runtime::Result<CoreWetStrokeUpdateSource> {
        Self::ICoreWetStrokeUpdateSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<CoreWetStrokeUpdateSource>(result__)
        })
    }
    pub fn ICoreWetStrokeUpdateSourceStatics<R, F: FnOnce(&ICoreWetStrokeUpdateSourceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreWetStrokeUpdateSource, ICoreWetStrokeUpdateSourceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreWetStrokeUpdateSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource;{1f718e22-ee52-4e00-8209-4c3e5b21a3cc})");
}
unsafe impl ::windows::runtime::Interface for CoreWetStrokeUpdateSource {
    type Vtable = ICoreWetStrokeUpdateSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(527535650, 61010, 19968, [130, 9, 76, 62, 91, 33, 163, 204]);
}
impl ::windows::runtime::RuntimeName for CoreWetStrokeUpdateSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource";
}
impl ::core::convert::From<CoreWetStrokeUpdateSource> for ::windows::runtime::IUnknown {
    fn from(value: CoreWetStrokeUpdateSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateSource> for ::windows::runtime::IUnknown {
    fn from(value: &CoreWetStrokeUpdateSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWetStrokeUpdateSource> for ::windows::runtime::IInspectable {
    fn from(value: CoreWetStrokeUpdateSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWetStrokeUpdateSource> for ::windows::runtime::IInspectable {
    fn from(value: &CoreWetStrokeUpdateSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreWetStrokeUpdateSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWetStrokeUpdateSource {}
unsafe impl ::core::marker::Sync for CoreWetStrokeUpdateSource {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStroke(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreIncrementalInkStroke {
    type Vtable = ICoreIncrementalInkStroke_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4255126995, 40294, 20349, [165, 127, 204, 112, 185, 207, 170, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStroke_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkpoints: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStrokeFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreIncrementalInkStrokeFactory {
    type Vtable = ICoreIncrementalInkStrokeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3620052806, 36264, 20336, [151, 81, 229, 59, 182, 223, 69, 150]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIncrementalInkStrokeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, drawingattributes: ::windows::runtime::RawPtr, pointtransform: super::super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreInkIndependentInputSource {
    type Vtable = ICoreInkIndependentInputSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(968068521, 30265, 17561, [165, 181, 25, 29, 0, 227, 91, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreInkIndependentInputSource2 {
    type Vtable = ICoreInkIndependentInputSource2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(675721234, 2905, 23481, [163, 197, 190, 203, 124, 240, 58, 51]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSource2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSourceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreInkIndependentInputSourceStatics {
    type Vtable = ICoreInkIndependentInputSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1944453403, 32960, 19963, [155, 102, 16, 186, 127, 63, 156, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkIndependentInputSourceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkpresenter: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreInkPresenterHost(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreInkPresenterHost {
    type Vtable = ICoreInkPresenterHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(963545574, 32085, 17943, [158, 88, 104, 199, 12, 145, 105, 185]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInkPresenterHost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWetStrokeUpdateEventArgs {
    type Vtable = ICoreWetStrokeUpdateEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4211593548, 13184, 17786, [169, 135, 153, 19, 87, 137, 108, 27]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreWetStrokeDisposition) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CoreWetStrokeDisposition) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWetStrokeUpdateSource {
    type Vtable = ICoreWetStrokeUpdateSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(527535650, 61010, 19968, [130, 9, 76, 62, 91, 33, 163, 204]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSourceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreWetStrokeUpdateSourceStatics {
    type Vtable = ICoreWetStrokeUpdateSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1034788026, 7485, 18094, [171, 157, 134, 71, 72, 108, 111, 144]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWetStrokeUpdateSourceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkpresenter: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);