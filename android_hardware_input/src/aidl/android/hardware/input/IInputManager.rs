/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/input/IInputManager.aidl -I aidl/ --structured -o android_hardware_input/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IInputManager["android.hardware.input.IInputManager"] {
    native: BnInputManager(on_transact),
    proxy: BpInputManager {
    },
    async: IInputManagerAsync,
  }
}
pub trait IInputManager: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.input.IInputManager" }
  fn r#getVelocityTrackerStrategy(&self) -> binder::Result<String>;
  fn r#getInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()>;
  fn r#getInputDeviceIds(&self) -> binder::Result<Vec<i32>>;
  fn r#isInputDeviceEnabled(&self, _arg_deviceId: i32) -> binder::Result<bool>;
  fn r#enableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()>;
  fn r#disableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()>;
  fn r#hasKeys(&self, _arg_deviceId: i32, _arg_sourceMask: i32, _arg_keyCodes: &[i32], _arg_keyExists: &mut Vec<bool>) -> binder::Result<bool>;
  fn r#getKeyCodeForKeyLocation(&self, _arg_deviceId: i32, _arg_locationKeyCode: i32) -> binder::Result<i32>;
  fn r#tryPointerSpeed(&self, _arg_speed: i32) -> binder::Result<()>;
  fn r#injectInputEvent(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32) -> binder::Result<bool>;
  fn r#injectInputEventToTarget(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32, _arg_targetUid: i32) -> binder::Result<bool>;
  fn getDefaultImpl() -> IInputManagerDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IInputManagerDefaultRef) -> IInputManagerDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IInputManagerAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.input.IInputManager" }
  fn r#getVelocityTrackerStrategy<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getInputDevice<'a>(&'a self, _arg_deviceId: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#getInputDeviceIds<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<i32>>>;
  fn r#isInputDeviceEnabled<'a>(&'a self, _arg_deviceId: i32) -> binder::BoxFuture<'a, binder::Result<bool>>;
  fn r#enableInputDevice<'a>(&'a self, _arg_deviceId: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#disableInputDevice<'a>(&'a self, _arg_deviceId: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#hasKeys<'a>(&'a self, _arg_deviceId: i32, _arg_sourceMask: i32, _arg_keyCodes: &'a [i32], _arg_keyExists: &'a mut Vec<bool>) -> binder::BoxFuture<'a, binder::Result<bool>>;
  fn r#getKeyCodeForKeyLocation<'a>(&'a self, _arg_deviceId: i32, _arg_locationKeyCode: i32) -> binder::BoxFuture<'a, binder::Result<i32>>;
  fn r#tryPointerSpeed<'a>(&'a self, _arg_speed: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#injectInputEvent<'a>(&'a self, _arg_ev: &'a input_manager::InputEvent, _arg_mode: i32) -> binder::BoxFuture<'a, binder::Result<bool>>;
  fn r#injectInputEventToTarget<'a>(&'a self, _arg_ev: &'a input_manager::InputEvent, _arg_mode: i32, _arg_targetUid: i32) -> binder::BoxFuture<'a, binder::Result<bool>>;
}
#[::async_trait::async_trait]
pub trait IInputManagerAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.input.IInputManager" }
  async fn r#getVelocityTrackerStrategy(&self) -> binder::Result<String>;
  async fn r#getInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()>;
  async fn r#getInputDeviceIds(&self) -> binder::Result<Vec<i32>>;
  async fn r#isInputDeviceEnabled(&self, _arg_deviceId: i32) -> binder::Result<bool>;
  async fn r#enableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()>;
  async fn r#disableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()>;
  async fn r#hasKeys(&self, _arg_deviceId: i32, _arg_sourceMask: i32, _arg_keyCodes: &[i32], _arg_keyExists: &mut Vec<bool>) -> binder::Result<bool>;
  async fn r#getKeyCodeForKeyLocation(&self, _arg_deviceId: i32, _arg_locationKeyCode: i32) -> binder::Result<i32>;
  async fn r#tryPointerSpeed(&self, _arg_speed: i32) -> binder::Result<()>;
  async fn r#injectInputEvent(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32) -> binder::Result<bool>;
  async fn r#injectInputEventToTarget(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32, _arg_targetUid: i32) -> binder::Result<bool>;
}
impl BnInputManager {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IInputManager>
  where
    T: IInputManagerAsyncServer + binder::Interface + Send + Sync + 'static,
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
    impl<T, R> IInputManager for Wrapper<T, R>
    where
      T: IInputManagerAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#getVelocityTrackerStrategy(&self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getVelocityTrackerStrategy())
      }
      fn r#getInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#getInputDevice(_arg_deviceId))
      }
      fn r#getInputDeviceIds(&self) -> binder::Result<Vec<i32>> {
        self._rt.block_on(self._inner.r#getInputDeviceIds())
      }
      fn r#isInputDeviceEnabled(&self, _arg_deviceId: i32) -> binder::Result<bool> {
        self._rt.block_on(self._inner.r#isInputDeviceEnabled(_arg_deviceId))
      }
      fn r#enableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#enableInputDevice(_arg_deviceId))
      }
      fn r#disableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#disableInputDevice(_arg_deviceId))
      }
      fn r#hasKeys(&self, _arg_deviceId: i32, _arg_sourceMask: i32, _arg_keyCodes: &[i32], _arg_keyExists: &mut Vec<bool>) -> binder::Result<bool> {
        self._rt.block_on(self._inner.r#hasKeys(_arg_deviceId, _arg_sourceMask, _arg_keyCodes, _arg_keyExists))
      }
      fn r#getKeyCodeForKeyLocation(&self, _arg_deviceId: i32, _arg_locationKeyCode: i32) -> binder::Result<i32> {
        self._rt.block_on(self._inner.r#getKeyCodeForKeyLocation(_arg_deviceId, _arg_locationKeyCode))
      }
      fn r#tryPointerSpeed(&self, _arg_speed: i32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#tryPointerSpeed(_arg_speed))
      }
      fn r#injectInputEvent(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32) -> binder::Result<bool> {
        self._rt.block_on(self._inner.r#injectInputEvent(_arg_ev, _arg_mode))
      }
      fn r#injectInputEventToTarget(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32, _arg_targetUid: i32) -> binder::Result<bool> {
        self._rt.block_on(self._inner.r#injectInputEventToTarget(_arg_ev, _arg_mode, _arg_targetUid))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IInputManagerDefault: Send + Sync {
  fn r#getVelocityTrackerStrategy(&self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getInputDeviceIds(&self) -> binder::Result<Vec<i32>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#isInputDeviceEnabled(&self, _arg_deviceId: i32) -> binder::Result<bool> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#enableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#disableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#hasKeys(&self, _arg_deviceId: i32, _arg_sourceMask: i32, _arg_keyCodes: &[i32], _arg_keyExists: &mut Vec<bool>) -> binder::Result<bool> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getKeyCodeForKeyLocation(&self, _arg_deviceId: i32, _arg_locationKeyCode: i32) -> binder::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#tryPointerSpeed(&self, _arg_speed: i32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#injectInputEvent(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32) -> binder::Result<bool> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#injectInputEventToTarget(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32, _arg_targetUid: i32) -> binder::Result<bool> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#getVelocityTrackerStrategy: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#getInputDevice: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
  pub const r#getInputDeviceIds: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
  pub const r#isInputDeviceEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
  pub const r#enableInputDevice: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
  pub const r#disableInputDevice: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
  pub const r#hasKeys: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
  pub const r#getKeyCodeForKeyLocation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
  pub const r#tryPointerSpeed: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
  pub const r#injectInputEvent: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
  pub const r#injectInputEventToTarget: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
}
pub type IInputManagerDefaultRef = Option<std::sync::Arc<dyn IInputManagerDefault>>;
static DEFAULT_IMPL: std::sync::Mutex<IInputManagerDefaultRef> = std::sync::Mutex::new(None);
impl BpInputManager {
  fn build_parcel_getVelocityTrackerStrategy(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getVelocityTrackerStrategy(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#getVelocityTrackerStrategy();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getInputDevice(&self, _arg_deviceId: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_deviceId)?;
    Ok(aidl_data)
  }
  fn read_response_getInputDevice(&self, _arg_deviceId: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#getInputDevice(_arg_deviceId);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_getInputDeviceIds(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getInputDeviceIds(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<i32>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#getInputDeviceIds();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<i32> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_isInputDeviceEnabled(&self, _arg_deviceId: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_deviceId)?;
    Ok(aidl_data)
  }
  fn read_response_isInputDeviceEnabled(&self, _arg_deviceId: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#isInputDeviceEnabled(_arg_deviceId);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: bool = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_enableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_deviceId)?;
    Ok(aidl_data)
  }
  fn read_response_enableInputDevice(&self, _arg_deviceId: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#enableInputDevice(_arg_deviceId);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_disableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_deviceId)?;
    Ok(aidl_data)
  }
  fn read_response_disableInputDevice(&self, _arg_deviceId: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#disableInputDevice(_arg_deviceId);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_hasKeys(&self, _arg_deviceId: i32, _arg_sourceMask: i32, _arg_keyCodes: &[i32], _arg_keyExists: &mut Vec<bool>) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_deviceId)?;
    aidl_data.write(&_arg_sourceMask)?;
    aidl_data.write(_arg_keyCodes)?;
    aidl_data.write_slice_size(Some(_arg_keyExists))?;
    Ok(aidl_data)
  }
  fn read_response_hasKeys(&self, _arg_deviceId: i32, _arg_sourceMask: i32, _arg_keyCodes: &[i32], _arg_keyExists: &mut Vec<bool>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#hasKeys(_arg_deviceId, _arg_sourceMask, _arg_keyCodes, _arg_keyExists);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: bool = _aidl_reply.read()?;
    _aidl_reply.read_onto(_arg_keyExists)?;
    Ok(_aidl_return)
  }
  fn build_parcel_getKeyCodeForKeyLocation(&self, _arg_deviceId: i32, _arg_locationKeyCode: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_deviceId)?;
    aidl_data.write(&_arg_locationKeyCode)?;
    Ok(aidl_data)
  }
  fn read_response_getKeyCodeForKeyLocation(&self, _arg_deviceId: i32, _arg_locationKeyCode: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#getKeyCodeForKeyLocation(_arg_deviceId, _arg_locationKeyCode);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_tryPointerSpeed(&self, _arg_speed: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_speed)?;
    Ok(aidl_data)
  }
  fn read_response_tryPointerSpeed(&self, _arg_speed: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#tryPointerSpeed(_arg_speed);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_injectInputEvent(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_ev)?;
    aidl_data.write(&_arg_mode)?;
    Ok(aidl_data)
  }
  fn read_response_injectInputEvent(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#injectInputEvent(_arg_ev, _arg_mode);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: bool = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_injectInputEventToTarget(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32, _arg_targetUid: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_ev)?;
    aidl_data.write(&_arg_mode)?;
    aidl_data.write(&_arg_targetUid)?;
    Ok(aidl_data)
  }
  fn read_response_injectInputEventToTarget(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32, _arg_targetUid: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IInputManager>::getDefaultImpl() {
        return _aidl_default_impl.r#injectInputEventToTarget(_arg_ev, _arg_mode, _arg_targetUid);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: bool = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
}
impl IInputManager for BpInputManager {
  fn r#getVelocityTrackerStrategy(&self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getVelocityTrackerStrategy()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getVelocityTrackerStrategy, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getVelocityTrackerStrategy(_aidl_reply)
  }
  fn r#getInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_getInputDevice(_arg_deviceId)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getInputDevice, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getInputDevice(_arg_deviceId, _aidl_reply)
  }
  fn r#getInputDeviceIds(&self) -> binder::Result<Vec<i32>> {
    let _aidl_data = self.build_parcel_getInputDeviceIds()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getInputDeviceIds, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getInputDeviceIds(_aidl_reply)
  }
  fn r#isInputDeviceEnabled(&self, _arg_deviceId: i32) -> binder::Result<bool> {
    let _aidl_data = self.build_parcel_isInputDeviceEnabled(_arg_deviceId)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#isInputDeviceEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_isInputDeviceEnabled(_arg_deviceId, _aidl_reply)
  }
  fn r#enableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_enableInputDevice(_arg_deviceId)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#enableInputDevice, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_enableInputDevice(_arg_deviceId, _aidl_reply)
  }
  fn r#disableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_disableInputDevice(_arg_deviceId)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#disableInputDevice, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_disableInputDevice(_arg_deviceId, _aidl_reply)
  }
  fn r#hasKeys(&self, _arg_deviceId: i32, _arg_sourceMask: i32, _arg_keyCodes: &[i32], _arg_keyExists: &mut Vec<bool>) -> binder::Result<bool> {
    let _aidl_data = self.build_parcel_hasKeys(_arg_deviceId, _arg_sourceMask, _arg_keyCodes, _arg_keyExists)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#hasKeys, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_hasKeys(_arg_deviceId, _arg_sourceMask, _arg_keyCodes, _arg_keyExists, _aidl_reply)
  }
  fn r#getKeyCodeForKeyLocation(&self, _arg_deviceId: i32, _arg_locationKeyCode: i32) -> binder::Result<i32> {
    let _aidl_data = self.build_parcel_getKeyCodeForKeyLocation(_arg_deviceId, _arg_locationKeyCode)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getKeyCodeForKeyLocation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getKeyCodeForKeyLocation(_arg_deviceId, _arg_locationKeyCode, _aidl_reply)
  }
  fn r#tryPointerSpeed(&self, _arg_speed: i32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_tryPointerSpeed(_arg_speed)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#tryPointerSpeed, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_tryPointerSpeed(_arg_speed, _aidl_reply)
  }
  fn r#injectInputEvent(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32) -> binder::Result<bool> {
    let _aidl_data = self.build_parcel_injectInputEvent(_arg_ev, _arg_mode)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#injectInputEvent, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_injectInputEvent(_arg_ev, _arg_mode, _aidl_reply)
  }
  fn r#injectInputEventToTarget(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32, _arg_targetUid: i32) -> binder::Result<bool> {
    let _aidl_data = self.build_parcel_injectInputEventToTarget(_arg_ev, _arg_mode, _arg_targetUid)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#injectInputEventToTarget, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_injectInputEventToTarget(_arg_ev, _arg_mode, _arg_targetUid, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IInputManagerAsync<P> for BpInputManager {
  fn r#getVelocityTrackerStrategy<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getVelocityTrackerStrategy() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getVelocityTrackerStrategy, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getVelocityTrackerStrategy(_aidl_reply)
      }
    )
  }
  fn r#getInputDevice<'a>(&'a self, _arg_deviceId: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_getInputDevice(_arg_deviceId) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getInputDevice, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getInputDevice(_arg_deviceId, _aidl_reply)
      }
    )
  }
  fn r#getInputDeviceIds<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<i32>>> {
    let _aidl_data = match self.build_parcel_getInputDeviceIds() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getInputDeviceIds, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getInputDeviceIds(_aidl_reply)
      }
    )
  }
  fn r#isInputDeviceEnabled<'a>(&'a self, _arg_deviceId: i32) -> binder::BoxFuture<'a, binder::Result<bool>> {
    let _aidl_data = match self.build_parcel_isInputDeviceEnabled(_arg_deviceId) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#isInputDeviceEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_isInputDeviceEnabled(_arg_deviceId, _aidl_reply)
      }
    )
  }
  fn r#enableInputDevice<'a>(&'a self, _arg_deviceId: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_enableInputDevice(_arg_deviceId) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#enableInputDevice, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_enableInputDevice(_arg_deviceId, _aidl_reply)
      }
    )
  }
  fn r#disableInputDevice<'a>(&'a self, _arg_deviceId: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_disableInputDevice(_arg_deviceId) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#disableInputDevice, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_disableInputDevice(_arg_deviceId, _aidl_reply)
      }
    )
  }
  fn r#hasKeys<'a>(&'a self, _arg_deviceId: i32, _arg_sourceMask: i32, _arg_keyCodes: &'a [i32], _arg_keyExists: &'a mut Vec<bool>) -> binder::BoxFuture<'a, binder::Result<bool>> {
    let _aidl_data = match self.build_parcel_hasKeys(_arg_deviceId, _arg_sourceMask, _arg_keyCodes, _arg_keyExists) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#hasKeys, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_hasKeys(_arg_deviceId, _arg_sourceMask, _arg_keyCodes, _arg_keyExists, _aidl_reply)
      }
    )
  }
  fn r#getKeyCodeForKeyLocation<'a>(&'a self, _arg_deviceId: i32, _arg_locationKeyCode: i32) -> binder::BoxFuture<'a, binder::Result<i32>> {
    let _aidl_data = match self.build_parcel_getKeyCodeForKeyLocation(_arg_deviceId, _arg_locationKeyCode) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getKeyCodeForKeyLocation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getKeyCodeForKeyLocation(_arg_deviceId, _arg_locationKeyCode, _aidl_reply)
      }
    )
  }
  fn r#tryPointerSpeed<'a>(&'a self, _arg_speed: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_tryPointerSpeed(_arg_speed) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#tryPointerSpeed, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_tryPointerSpeed(_arg_speed, _aidl_reply)
      }
    )
  }
  fn r#injectInputEvent<'a>(&'a self, _arg_ev: &'a input_manager::InputEvent, _arg_mode: i32) -> binder::BoxFuture<'a, binder::Result<bool>> {
    let _aidl_data = match self.build_parcel_injectInputEvent(_arg_ev, _arg_mode) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#injectInputEvent, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_injectInputEvent(_arg_ev, _arg_mode, _aidl_reply)
      }
    )
  }
  fn r#injectInputEventToTarget<'a>(&'a self, _arg_ev: &'a input_manager::InputEvent, _arg_mode: i32, _arg_targetUid: i32) -> binder::BoxFuture<'a, binder::Result<bool>> {
    let _aidl_data = match self.build_parcel_injectInputEventToTarget(_arg_ev, _arg_mode, _arg_targetUid) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#injectInputEventToTarget, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_injectInputEventToTarget(_arg_ev, _arg_mode, _arg_targetUid, _aidl_reply)
      }
    )
  }
}
impl IInputManager for binder::binder_impl::Binder<BnInputManager> {
  fn r#getVelocityTrackerStrategy(&self) -> binder::Result<String> { self.0.r#getVelocityTrackerStrategy() }
  fn r#getInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> { self.0.r#getInputDevice(_arg_deviceId) }
  fn r#getInputDeviceIds(&self) -> binder::Result<Vec<i32>> { self.0.r#getInputDeviceIds() }
  fn r#isInputDeviceEnabled(&self, _arg_deviceId: i32) -> binder::Result<bool> { self.0.r#isInputDeviceEnabled(_arg_deviceId) }
  fn r#enableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> { self.0.r#enableInputDevice(_arg_deviceId) }
  fn r#disableInputDevice(&self, _arg_deviceId: i32) -> binder::Result<()> { self.0.r#disableInputDevice(_arg_deviceId) }
  fn r#hasKeys(&self, _arg_deviceId: i32, _arg_sourceMask: i32, _arg_keyCodes: &[i32], _arg_keyExists: &mut Vec<bool>) -> binder::Result<bool> { self.0.r#hasKeys(_arg_deviceId, _arg_sourceMask, _arg_keyCodes, _arg_keyExists) }
  fn r#getKeyCodeForKeyLocation(&self, _arg_deviceId: i32, _arg_locationKeyCode: i32) -> binder::Result<i32> { self.0.r#getKeyCodeForKeyLocation(_arg_deviceId, _arg_locationKeyCode) }
  fn r#tryPointerSpeed(&self, _arg_speed: i32) -> binder::Result<()> { self.0.r#tryPointerSpeed(_arg_speed) }
  fn r#injectInputEvent(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32) -> binder::Result<bool> { self.0.r#injectInputEvent(_arg_ev, _arg_mode) }
  fn r#injectInputEventToTarget(&self, _arg_ev: &input_manager::InputEvent, _arg_mode: i32, _arg_targetUid: i32) -> binder::Result<bool> { self.0.r#injectInputEventToTarget(_arg_ev, _arg_mode, _arg_targetUid) }
}
fn on_transact(_aidl_service: &dyn IInputManager, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#getVelocityTrackerStrategy => {
      let _aidl_return = _aidl_service.r#getVelocityTrackerStrategy();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getInputDevice => {
      let _arg_deviceId: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getInputDevice(_arg_deviceId);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getInputDeviceIds => {
      let _aidl_return = _aidl_service.r#getInputDeviceIds();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#isInputDeviceEnabled => {
      let _arg_deviceId: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#isInputDeviceEnabled(_arg_deviceId);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#enableInputDevice => {
      let _arg_deviceId: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#enableInputDevice(_arg_deviceId);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#disableInputDevice => {
      let _arg_deviceId: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#disableInputDevice(_arg_deviceId);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#hasKeys => {
      let _arg_deviceId: i32 = _aidl_data.read()?;
      let _arg_sourceMask: i32 = _aidl_data.read()?;
      let _arg_keyCodes: Vec<i32> = _aidl_data.read()?;
      let mut _arg_keyExists: Vec<bool> = Default::default();
      _aidl_data.resize_out_vec(&mut _arg_keyExists)?;
      let _aidl_return = _aidl_service.r#hasKeys(_arg_deviceId, _arg_sourceMask, &_arg_keyCodes, &mut _arg_keyExists);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
          _aidl_reply.write(&_arg_keyExists)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getKeyCodeForKeyLocation => {
      let _arg_deviceId: i32 = _aidl_data.read()?;
      let _arg_locationKeyCode: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getKeyCodeForKeyLocation(_arg_deviceId, _arg_locationKeyCode);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#tryPointerSpeed => {
      let _arg_speed: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#tryPointerSpeed(_arg_speed);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#injectInputEvent => {
      let _arg_ev: input_manager::InputEvent = _aidl_data.read()?;
      let _arg_mode: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#injectInputEvent(&_arg_ev, _arg_mode);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#injectInputEventToTarget => {
      let _arg_ev: input_manager::InputEvent = _aidl_data.read()?;
      let _arg_mode: i32 = _aidl_data.read()?;
      let _arg_targetUid: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#injectInputEventToTarget(&_arg_ev, _arg_mode, _arg_targetUid);
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
 pub use super::r#IInputManager as _7_android_8_hardware_5_input_13_IInputManager;
}
