#![forbid(unsafe_code)]
#![rustfmt::skip]
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
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _file: &std::fs::File, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_file, _args) }
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
use lazy_static::lazy_static;
lazy_static! {
  static ref DEFAULT_IMPL: std::sync::Mutex<IComposerDefaultRef> = std::sync::Mutex::new(None);
}
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
