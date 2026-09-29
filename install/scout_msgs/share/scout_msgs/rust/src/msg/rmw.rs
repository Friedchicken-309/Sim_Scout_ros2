#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "scout_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__scout_msgs__msg__ScoutActuatorState() -> *const std::ffi::c_void;
}

#[link(name = "scout_msgs__rosidl_generator_c")]
extern "C" {
    fn scout_msgs__msg__ScoutActuatorState__init(msg: *mut ScoutActuatorState) -> bool;
    fn scout_msgs__msg__ScoutActuatorState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ScoutActuatorState>, size: usize) -> bool;
    fn scout_msgs__msg__ScoutActuatorState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ScoutActuatorState>);
    fn scout_msgs__msg__ScoutActuatorState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ScoutActuatorState>, out_seq: *mut rosidl_runtime_rs::Sequence<ScoutActuatorState>) -> bool;
}

// Corresponds to scout_msgs__msg__ScoutActuatorState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// define DRIVER_STATE_INPUT_VOLTAGE_LOW_MASK ((uint8_t)0x01)
/// define DRIVER_STATE_MOTOR_OVERHEAT_MASK ((uint8_t)0x02)
/// define DRIVER_STATE_DRIVER_OVERLOAD_MASK ((uint8_t)0x04)
/// define DRIVER_STATE_DRIVER_OVERHEAT_MASK ((uint8_t)0x08)
/// define DRIVER_STATE_SENSOR_FAULT_MASK ((uint8_t)0x10)
/// define DRIVER_STATE_DRIVER_FAULT_MASK ((uint8_t)0x20)
/// define DRIVER_STATE_DRIVER_ENABLED_MASK ((uint8_t)0x40)
/// define DRIVER_STATE_DRIVER_RESET_MASK ((uint8_t)0x80)

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ScoutActuatorState {
    /// uint8 motor_id
    pub rpm: i16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current: f64,

    /// int32 pulse_count
    pub driver_voltage: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub driver_temperature: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_temperature: i8,

}



impl Default for ScoutActuatorState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !scout_msgs__msg__ScoutActuatorState__init(&mut msg as *mut _) {
        panic!("Call to scout_msgs__msg__ScoutActuatorState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ScoutActuatorState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutActuatorState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutActuatorState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutActuatorState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ScoutActuatorState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ScoutActuatorState where Self: Sized {
  const TYPE_NAME: &'static str = "scout_msgs/msg/ScoutActuatorState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__scout_msgs__msg__ScoutActuatorState() }
  }
}


#[link(name = "scout_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__scout_msgs__msg__ScoutLightCmd() -> *const std::ffi::c_void;
}

#[link(name = "scout_msgs__rosidl_generator_c")]
extern "C" {
    fn scout_msgs__msg__ScoutLightCmd__init(msg: *mut ScoutLightCmd) -> bool;
    fn scout_msgs__msg__ScoutLightCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ScoutLightCmd>, size: usize) -> bool;
    fn scout_msgs__msg__ScoutLightCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ScoutLightCmd>);
    fn scout_msgs__msg__ScoutLightCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ScoutLightCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<ScoutLightCmd>) -> bool;
}

// Corresponds to scout_msgs__msg__ScoutLightCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ScoutLightCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cmd_ctrl_allowed: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_custom_value: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_custom_value: u8,

}

impl ScoutLightCmd {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_CONST_OFF: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_CONST_ON: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_BREATH: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_CUSTOM: u8 = 3;

}


impl Default for ScoutLightCmd {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !scout_msgs__msg__ScoutLightCmd__init(&mut msg as *mut _) {
        panic!("Call to scout_msgs__msg__ScoutLightCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ScoutLightCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutLightCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutLightCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutLightCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ScoutLightCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ScoutLightCmd where Self: Sized {
  const TYPE_NAME: &'static str = "scout_msgs/msg/ScoutLightCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__scout_msgs__msg__ScoutLightCmd() }
  }
}


#[link(name = "scout_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__scout_msgs__msg__ScoutLightState() -> *const std::ffi::c_void;
}

#[link(name = "scout_msgs__rosidl_generator_c")]
extern "C" {
    fn scout_msgs__msg__ScoutLightState__init(msg: *mut ScoutLightState) -> bool;
    fn scout_msgs__msg__ScoutLightState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ScoutLightState>, size: usize) -> bool;
    fn scout_msgs__msg__ScoutLightState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ScoutLightState>);
    fn scout_msgs__msg__ScoutLightState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ScoutLightState>, out_seq: *mut rosidl_runtime_rs::Sequence<ScoutLightState>) -> bool;
}

// Corresponds to scout_msgs__msg__ScoutLightState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ScoutLightState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub custom_value: u8,

}



impl Default for ScoutLightState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !scout_msgs__msg__ScoutLightState__init(&mut msg as *mut _) {
        panic!("Call to scout_msgs__msg__ScoutLightState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ScoutLightState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutLightState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutLightState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutLightState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ScoutLightState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ScoutLightState where Self: Sized {
  const TYPE_NAME: &'static str = "scout_msgs/msg/ScoutLightState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__scout_msgs__msg__ScoutLightState() }
  }
}


#[link(name = "scout_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__scout_msgs__msg__ScoutRCState() -> *const std::ffi::c_void;
}

#[link(name = "scout_msgs__rosidl_generator_c")]
extern "C" {
    fn scout_msgs__msg__ScoutRCState__init(msg: *mut ScoutRCState) -> bool;
    fn scout_msgs__msg__ScoutRCState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ScoutRCState>, size: usize) -> bool;
    fn scout_msgs__msg__ScoutRCState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ScoutRCState>);
    fn scout_msgs__msg__ScoutRCState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ScoutRCState>, out_seq: *mut rosidl_runtime_rs::Sequence<ScoutRCState>) -> bool;
}

// Corresponds to scout_msgs__msg__ScoutRCState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ScoutRCState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub swa: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swb: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swc: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swd: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stick_right_v: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stick_right_h: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stick_left_v: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stick_left_h: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub var_a: i8,

}



impl Default for ScoutRCState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !scout_msgs__msg__ScoutRCState__init(&mut msg as *mut _) {
        panic!("Call to scout_msgs__msg__ScoutRCState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ScoutRCState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutRCState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutRCState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutRCState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ScoutRCState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ScoutRCState where Self: Sized {
  const TYPE_NAME: &'static str = "scout_msgs/msg/ScoutRCState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__scout_msgs__msg__ScoutRCState() }
  }
}


#[link(name = "scout_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__scout_msgs__msg__ScoutStatus() -> *const std::ffi::c_void;
}

#[link(name = "scout_msgs__rosidl_generator_c")]
extern "C" {
    fn scout_msgs__msg__ScoutStatus__init(msg: *mut ScoutStatus) -> bool;
    fn scout_msgs__msg__ScoutStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ScoutStatus>, size: usize) -> bool;
    fn scout_msgs__msg__ScoutStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ScoutStatus>);
    fn scout_msgs__msg__ScoutStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ScoutStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<ScoutStatus>) -> bool;
}

// Corresponds to scout_msgs__msg__ScoutStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ScoutStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// motion state
    pub linear_velocity: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub angular_velocity: f64,

    /// base state
    pub vehicle_state: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub control_mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_code: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub battery_voltage: f64,

    /// motor state
    pub actuator_states: [super::super::msg::rmw::ScoutActuatorState; 4],

    /// light state
    pub light_control_enabled: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_light_state: super::super::msg::rmw::ScoutLightState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_light_state: super::super::msg::rmw::ScoutLightState,

}

impl ScoutStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_ID_FRONT_RIGHT: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_ID_FRONT_LEFT: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_ID_REAR_RIGHT: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_ID_REAR_LEFT: i8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_ID_FRONT: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIGHT_ID_REAR: i8 = 1;

}


impl Default for ScoutStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !scout_msgs__msg__ScoutStatus__init(&mut msg as *mut _) {
        panic!("Call to scout_msgs__msg__ScoutStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ScoutStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { scout_msgs__msg__ScoutStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ScoutStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ScoutStatus where Self: Sized {
  const TYPE_NAME: &'static str = "scout_msgs/msg/ScoutStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__scout_msgs__msg__ScoutStatus() }
  }
}


