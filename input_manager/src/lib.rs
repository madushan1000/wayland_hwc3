use binder::binder_impl::{BorrowedParcel, UnstructuredParcelable};
use binder::{
    impl_deserialize_for_unstructured_parcelable, impl_serialize_for_unstructured_parcelable,
    StatusCode,
};

const PARCEL_TOKEN_MOTION_EVENT: i32 = 1;
const PARCEL_TOKEN_KEY_EVENT: i32 = 2;

pub const INJECT_INPUT_EVENT_MODE_ASYNC: i32 = 0;
pub const INJECT_INPUT_EVENT_MODE_WAIT_FOR_RESULT: i32 = 1;
pub const INJECT_INPUT_EVENT_MODE_WAIT_FOR_FINISH: i32 = 2;

pub enum InputEvent {
    MotionEvent(MotionEvent),
    KeyEvent(KeyEvent),
}

impl UnstructuredParcelable for InputEvent {
    fn write_to_parcel(&self, parcel: &mut BorrowedParcel<'_>) -> Result<(), StatusCode> {
        match self {
            InputEvent::MotionEvent(me) => me.write_to_parcel(parcel),
            InputEvent::KeyEvent(ke) => ke.write_to_parcel(parcel),
        }
    }

    fn from_parcel(parcel: &BorrowedParcel<'_>) -> Result<Self, StatusCode> {
        let token = parcel.read()?;
        match token {
            PARCEL_TOKEN_MOTION_EVENT => {
                Ok(InputEvent::MotionEvent(MotionEvent::from_parcel(parcel)?))
            }
            PARCEL_TOKEN_KEY_EVENT => Ok(InputEvent::KeyEvent(KeyEvent::from_parcel(parcel)?)),
            _ => panic!("shouldn't get these"),
        }
    }
}

impl_serialize_for_unstructured_parcelable!(InputEvent);
impl_deserialize_for_unstructured_parcelable!(InputEvent);

pub struct KeyEvent {
    pub id: i32,
    pub device_id: i32,
    pub source: i32,
    pub display_id: i32,
    pub hmac: Option<Vec<u8>>,
    pub action: i32,
    pub keycode: i32,
    pub repeat_count: i32,
    pub mata_state: i32,
    pub scancode: i32,
    pub flags: i32,
    pub down_time: i64,
    pub event_time: i64,
    pub characters: Option<String>,
}

impl_serialize_for_unstructured_parcelable!(KeyEvent);
impl_deserialize_for_unstructured_parcelable!(KeyEvent);

impl UnstructuredParcelable for KeyEvent {
    fn write_to_parcel(&self, parcel: &mut BorrowedParcel<'_>) -> Result<(), StatusCode> {
        parcel.write(&PARCEL_TOKEN_KEY_EVENT)?;
        parcel.write(&self.id)?;
        parcel.write(&self.device_id)?;
        parcel.write(&self.source)?;
        parcel.write(&self.display_id)?;
        parcel.write(&self.hmac)?;
        parcel.write(&self.action)?;
        parcel.write(&self.keycode)?;
        parcel.write(&self.repeat_count)?;
        parcel.write(&self.mata_state)?;
        parcel.write(&self.scancode)?;
        parcel.write(&self.flags)?;
        parcel.write(&self.down_time)?;
        parcel.write(&self.event_time)?;
        parcel.write(&self.characters)?;
        Ok(())
    }

    fn from_parcel(parcel: &BorrowedParcel<'_>) -> Result<Self, StatusCode> {
        Ok(Self {
            id: parcel.read()?,
            device_id: parcel.read()?,
            source: parcel.read()?,
            display_id: parcel.read()?,
            hmac: parcel.read()?,
            action: parcel.read()?,
            keycode: parcel.read()?,
            repeat_count: parcel.read()?,
            mata_state: parcel.read()?,
            scancode: parcel.read()?,
            flags: parcel.read()?,
            down_time: parcel.read()?,
            event_time: parcel.read()?,
            characters: parcel.read()?,
        })
    }
}

pub struct MotionEvent {}

impl_serialize_for_unstructured_parcelable!(MotionEvent);
impl_deserialize_for_unstructured_parcelable!(MotionEvent);

impl UnstructuredParcelable for MotionEvent {
    fn write_to_parcel(&self, parcel: &mut BorrowedParcel<'_>) -> Result<(), StatusCode> {
        todo!()
    }

    fn from_parcel(parcel: &BorrowedParcel<'_>) -> Result<Self, StatusCode> {
        todo!()
    }
}
