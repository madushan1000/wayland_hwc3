/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/composer3/Buffer.aidl aidl/android/hardware/graphics/composer3/Capability.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionLayer.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionTypes.aidl aidl/android/hardware/graphics/composer3/ClientTarget.aidl aidl/android/hardware/graphics/composer3/ClientTargetProperty.aidl aidl/android/hardware/graphics/composer3/ClientTargetPropertyWithBrightness.aidl aidl/android/hardware/graphics/composer3/ClockMonotonicTimestamp.aidl aidl/android/hardware/graphics/composer3/Color.aidl aidl/android/hardware/graphics/composer3/ColorMode.aidl aidl/android/hardware/graphics/composer3/CommandError.aidl aidl/android/hardware/graphics/composer3/CommandResultPayload.aidl aidl/android/hardware/graphics/composer3/Composition.aidl aidl/android/hardware/graphics/composer3/ContentType.aidl aidl/android/hardware/graphics/composer3/DimmingStage.aidl aidl/android/hardware/graphics/composer3/DisplayAttribute.aidl aidl/android/hardware/graphics/composer3/DisplayBrightness.aidl aidl/android/hardware/graphics/composer3/DisplayCapability.aidl aidl/android/hardware/graphics/composer3/DisplayCommand.aidl aidl/android/hardware/graphics/composer3/DisplayConnectionType.aidl aidl/android/hardware/graphics/composer3/DisplayContentSample.aidl aidl/android/hardware/graphics/composer3/DisplayContentSamplingAttributes.aidl aidl/android/hardware/graphics/composer3/DisplayIdentification.aidl aidl/android/hardware/graphics/composer3/DisplayRequest.aidl aidl/android/hardware/graphics/composer3/FormatColorComponent.aidl aidl/android/hardware/graphics/composer3/HdrCapabilities.aidl aidl/android/hardware/graphics/composer3/IComposer.aidl aidl/android/hardware/graphics/composer3/IComposerCallback.aidl aidl/android/hardware/graphics/composer3/IComposerClient.aidl aidl/android/hardware/graphics/composer3/LayerBrightness.aidl aidl/android/hardware/graphics/composer3/LayerCommand.aidl aidl/android/hardware/graphics/composer3/OverlayProperties.aidl aidl/android/hardware/graphics/composer3/ParcelableBlendMode.aidl aidl/android/hardware/graphics/composer3/ParcelableComposition.aidl aidl/android/hardware/graphics/composer3/ParcelableDataspace.aidl aidl/android/hardware/graphics/composer3/ParcelableTransform.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadata.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataBlob.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataKey.aidl aidl/android/hardware/graphics/composer3/PlaneAlpha.aidl aidl/android/hardware/graphics/composer3/PowerMode.aidl aidl/android/hardware/graphics/composer3/PresentFence.aidl aidl/android/hardware/graphics/composer3/PresentOrValidate.aidl aidl/android/hardware/graphics/composer3/ReadbackBufferAttributes.aidl aidl/android/hardware/graphics/composer3/RefreshRateChangedDebugData.aidl aidl/android/hardware/graphics/composer3/ReleaseFences.aidl aidl/android/hardware/graphics/composer3/RenderIntent.aidl aidl/android/hardware/graphics/composer3/VirtualDisplay.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeConstraints.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeTimeline.aidl aidl/android/hardware/graphics/composer3/ZOrder.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_composer3/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IComposer["android.hardware.graphics.composer3.IComposer"] {
    native: BnComposer(on_transact),
    proxy: BpComposer {
    },
    async: IComposerAsync,
    stability: binder::binder_impl::Stability::Vintf,
  }
}
pub trait IComposer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposer" }
  fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>>;
  fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>>;
  fn getDefaultImpl() -> IComposerDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IComposerDefaultRef) -> IComposerDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IComposerAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposer" }
  fn r#createClient<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>>>;
  fn r#getCapabilities<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>>>;
}
#[::async_trait::async_trait]
pub trait IComposerAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposer" }
  async fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>>;
  async fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>>;
}
impl BnComposer {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IComposer>
  where
    T: IComposerAsyncServer + binder::Interface + Send + Sync + 'static,
    R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
  {
    struct Wrapper<T, R> {
      _inner: T,
      _rt: R,
    }
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
    }
    impl<T, R> IComposer for Wrapper<T, R>
    where
      T: IComposerAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>> {
        self._rt.block_on(self._inner.r#createClient())
      }
      fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>> {
        self._rt.block_on(self._inner.r#getCapabilities())
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IComposerDefault: Send + Sync {
  fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#createClient: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#getCapabilities: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
}
pub type IComposerDefaultRef = Option<std::sync::Arc<dyn IComposerDefault>>;
static DEFAULT_IMPL: std::sync::Mutex<IComposerDefaultRef> = std::sync::Mutex::new(None);
pub const r#EX_NO_RESOURCES: i32 = 6;
impl BpComposer {
  fn build_parcel_createClient(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_createClient(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposer>::getDefaultImpl() {
        return _aidl_default_impl.r#createClient();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getCapabilities(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getCapabilities(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposer>::getDefaultImpl() {
        return _aidl_default_impl.r#getCapabilities();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
}
impl IComposer for BpComposer {
  fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>> {
    let _aidl_data = self.build_parcel_createClient()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#createClient, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_createClient(_aidl_reply)
  }
  fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>> {
    let _aidl_data = self.build_parcel_getCapabilities()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getCapabilities(_aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IComposerAsync<P> for BpComposer {
  fn r#createClient<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>>> {
    let _aidl_data = match self.build_parcel_createClient() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#createClient, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_createClient(_aidl_reply)
      }
    )
  }
  fn r#getCapabilities<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>>> {
    let _aidl_data = match self.build_parcel_getCapabilities() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getCapabilities(_aidl_reply)
      }
    )
  }
}
impl IComposer for binder::binder_impl::Binder<BnComposer> {
  fn r#createClient(&self) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient>> { self.0.r#createClient() }
  fn r#getCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_Capability>> { self.0.r#getCapabilities() }
}
fn on_transact(_aidl_service: &dyn IComposer, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#createClient => {
      let _aidl_return = _aidl_service.r#createClient();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getCapabilities => {
      let _aidl_return = _aidl_service.r#getCapabilities();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IComposer as _7_android_8_hardware_8_graphics_9_composer3_9_IComposer;
}
