#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IComposerCallback["android.hardware.graphics.composer3.IComposerCallback"] {
    native: BnComposerCallback(on_transact),
    proxy: BpComposerCallback {
    },
    async: IComposerCallbackAsync,
    stability: binder::binder_impl::Stability::Vintf,
  }
}
pub trait IComposerCallback: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerCallback" }
  fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()>;
  fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()>;
  fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()>;
  fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()>;
  fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()>;
  fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()>;
  fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()>;
  fn getDefaultImpl() -> IComposerCallbackDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IComposerCallbackDefaultRef) -> IComposerCallbackDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IComposerCallbackAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerCallback" }
  fn r#onHotplug<'a>(&'a self, _arg_display: i64, _arg_connected: bool) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#onRefresh(&self, _arg_display: i64) -> std::future::Ready<binder::Result<()>>;
  fn r#onSeamlessPossible(&self, _arg_display: i64) -> std::future::Ready<binder::Result<()>>;
  fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> std::future::Ready<binder::Result<()>>;
  fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> std::future::Ready<binder::Result<()>>;
  fn r#onVsyncIdle(&self, _arg_display: i64) -> std::future::Ready<binder::Result<()>>;
  fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> std::future::Ready<binder::Result<()>>;
}
#[::async_trait::async_trait]
pub trait IComposerCallbackAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerCallback" }
  async fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()>;
  async fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()>;
  async fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()>;
  async fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()>;
  async fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()>;
  async fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()>;
  async fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()>;
}
impl BnComposerCallback {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IComposerCallback>
  where
    T: IComposerCallbackAsyncServer + binder::Interface + Send + Sync + 'static,
    R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
  {
    struct Wrapper<T, R> {
      _inner: T,
      _rt: R,
    }
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _file: &std::fs::File, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_file, _args) }
    }
    impl<T, R> IComposerCallback for Wrapper<T, R>
    where
      T: IComposerCallbackAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onHotplug(_arg_display, _arg_connected))
      }
      fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onRefresh(_arg_display))
      }
      fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onSeamlessPossible(_arg_display))
      }
      fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos))
      }
      fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline))
      }
      fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onVsyncIdle(_arg_display))
      }
      fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onRefreshRateChangedDebug(_arg_data))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IComposerCallbackDefault: Send + Sync {
  fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#onHotplug: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#onRefresh: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
  pub const r#onSeamlessPossible: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
  pub const r#onVsync: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
  pub const r#onVsyncPeriodTimingChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
  pub const r#onVsyncIdle: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
  pub const r#onRefreshRateChangedDebug: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
}
pub type IComposerCallbackDefaultRef = Option<std::sync::Arc<dyn IComposerCallbackDefault>>;
use lazy_static::lazy_static;
lazy_static! {
  static ref DEFAULT_IMPL: std::sync::Mutex<IComposerCallbackDefaultRef> = std::sync::Mutex::new(None);
}
impl BpComposerCallback {
  fn build_parcel_onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_connected)?;
    Ok(aidl_data)
  }
  fn read_response_onHotplug(&self, _arg_display: i64, _arg_connected: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
        return _aidl_default_impl.r#onHotplug(_arg_display, _arg_connected);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_onRefresh(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_onRefresh(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
        return _aidl_default_impl.r#onRefresh(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
  fn build_parcel_onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_onSeamlessPossible(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
        return _aidl_default_impl.r#onSeamlessPossible(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
  fn build_parcel_onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_timestamp)?;
    aidl_data.write(&_arg_vsyncPeriodNanos)?;
    Ok(aidl_data)
  }
  fn read_response_onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
        return _aidl_default_impl.r#onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos);
      }
    }
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
  fn build_parcel_onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(_arg_updatedTimeline)?;
    Ok(aidl_data)
  }
  fn read_response_onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
        return _aidl_default_impl.r#onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline);
      }
    }
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
  fn build_parcel_onVsyncIdle(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_onVsyncIdle(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
        return _aidl_default_impl.r#onVsyncIdle(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
  fn build_parcel_onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_data)?;
    Ok(aidl_data)
  }
  fn read_response_onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerCallback>::getDefaultImpl() {
        return _aidl_default_impl.r#onRefreshRateChangedDebug(_arg_data);
      }
    }
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
}
impl IComposerCallback for BpComposerCallback {
  fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onHotplug(_arg_display, _arg_connected)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onHotplug, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onHotplug(_arg_display, _arg_connected, _aidl_reply)
  }
  fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onRefresh(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onRefresh, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onRefresh(_arg_display, _aidl_reply)
  }
  fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onSeamlessPossible(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onSeamlessPossible, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onSeamlessPossible(_arg_display, _aidl_reply)
  }
  fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onVsync, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos, _aidl_reply)
  }
  fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onVsyncPeriodTimingChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline, _aidl_reply)
  }
  fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onVsyncIdle(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onVsyncIdle, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onVsyncIdle(_arg_display, _aidl_reply)
  }
  fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onRefreshRateChangedDebug(_arg_data)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onRefreshRateChangedDebug, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onRefreshRateChangedDebug(_arg_data, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IComposerCallbackAsync<P> for BpComposerCallback {
  fn r#onHotplug<'a>(&'a self, _arg_display: i64, _arg_connected: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onHotplug(_arg_display, _arg_connected) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#onHotplug, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_onHotplug(_arg_display, _arg_connected, _aidl_reply)
      }
    )
  }
  fn r#onRefresh(&self, _arg_display: i64) -> std::future::Ready<binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onRefresh(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return std::future::ready(Err(err)),
    };
    let _aidl_reply = self.binder.submit_transact(transactions::r#onRefresh, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    std::future::ready(self.read_response_onRefresh(_arg_display, _aidl_reply))
  }
  fn r#onSeamlessPossible(&self, _arg_display: i64) -> std::future::Ready<binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onSeamlessPossible(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return std::future::ready(Err(err)),
    };
    let _aidl_reply = self.binder.submit_transact(transactions::r#onSeamlessPossible, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    std::future::ready(self.read_response_onSeamlessPossible(_arg_display, _aidl_reply))
  }
  fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> std::future::Ready<binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return std::future::ready(Err(err)),
    };
    let _aidl_reply = self.binder.submit_transact(transactions::r#onVsync, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    std::future::ready(self.read_response_onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos, _aidl_reply))
  }
  fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> std::future::Ready<binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return std::future::ready(Err(err)),
    };
    let _aidl_reply = self.binder.submit_transact(transactions::r#onVsyncPeriodTimingChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    std::future::ready(self.read_response_onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline, _aidl_reply))
  }
  fn r#onVsyncIdle(&self, _arg_display: i64) -> std::future::Ready<binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onVsyncIdle(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return std::future::ready(Err(err)),
    };
    let _aidl_reply = self.binder.submit_transact(transactions::r#onVsyncIdle, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    std::future::ready(self.read_response_onVsyncIdle(_arg_display, _aidl_reply))
  }
  fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> std::future::Ready<binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onRefreshRateChangedDebug(_arg_data) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return std::future::ready(Err(err)),
    };
    let _aidl_reply = self.binder.submit_transact(transactions::r#onRefreshRateChangedDebug, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    std::future::ready(self.read_response_onRefreshRateChangedDebug(_arg_data, _aidl_reply))
  }
}
impl IComposerCallback for binder::binder_impl::Binder<BnComposerCallback> {
  fn r#onHotplug(&self, _arg_display: i64, _arg_connected: bool) -> binder::Result<()> { self.0.r#onHotplug(_arg_display, _arg_connected) }
  fn r#onRefresh(&self, _arg_display: i64) -> binder::Result<()> { self.0.r#onRefresh(_arg_display) }
  fn r#onSeamlessPossible(&self, _arg_display: i64) -> binder::Result<()> { self.0.r#onSeamlessPossible(_arg_display) }
  fn r#onVsync(&self, _arg_display: i64, _arg_timestamp: i64, _arg_vsyncPeriodNanos: i32) -> binder::Result<()> { self.0.r#onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos) }
  fn r#onVsyncPeriodTimingChanged(&self, _arg_display: i64, _arg_updatedTimeline: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline) -> binder::Result<()> { self.0.r#onVsyncPeriodTimingChanged(_arg_display, _arg_updatedTimeline) }
  fn r#onVsyncIdle(&self, _arg_display: i64) -> binder::Result<()> { self.0.r#onVsyncIdle(_arg_display) }
  fn r#onRefreshRateChangedDebug(&self, _arg_data: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData) -> binder::Result<()> { self.0.r#onRefreshRateChangedDebug(_arg_data) }
}
fn on_transact(_aidl_service: &dyn IComposerCallback, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#onHotplug => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_connected: bool = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onHotplug(_arg_display, _arg_connected);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#onRefresh => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onRefresh(_arg_display);
      Ok(())
    }
    transactions::r#onSeamlessPossible => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onSeamlessPossible(_arg_display);
      Ok(())
    }
    transactions::r#onVsync => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_timestamp: i64 = _aidl_data.read()?;
      let _arg_vsyncPeriodNanos: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onVsync(_arg_display, _arg_timestamp, _arg_vsyncPeriodNanos);
      Ok(())
    }
    transactions::r#onVsyncPeriodTimingChanged => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_updatedTimeline: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onVsyncPeriodTimingChanged(_arg_display, &_arg_updatedTimeline);
      Ok(())
    }
    transactions::r#onVsyncIdle => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onVsyncIdle(_arg_display);
      Ok(())
    }
    transactions::r#onRefreshRateChangedDebug => {
      let _arg_data: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onRefreshRateChangedDebug(&_arg_data);
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IComposerCallback as _7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback;
}
