#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to scout_msgs__msg__ScoutActuatorState
/// define DRIVER_STATE_INPUT_VOLTAGE_LOW_MASK ((uint8_t)0x01)
/// define DRIVER_STATE_MOTOR_OVERHEAT_MASK ((uint8_t)0x02)
/// define DRIVER_STATE_DRIVER_OVERLOAD_MASK ((uint8_t)0x04)
/// define DRIVER_STATE_DRIVER_OVERHEAT_MASK ((uint8_t)0x08)
/// define DRIVER_STATE_SENSOR_FAULT_MASK ((uint8_t)0x10)
/// define DRIVER_STATE_DRIVER_FAULT_MASK ((uint8_t)0x20)
/// define DRIVER_STATE_DRIVER_ENABLED_MASK ((uint8_t)0x40)
/// define DRIVER_STATE_DRIVER_RESET_MASK ((uint8_t)0x80)

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ScoutActuatorState::default())
  }
}

impl rosidl_runtime_rs::Message for ScoutActuatorState {
  type RmwMsg = super::msg::rmw::ScoutActuatorState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rpm: msg.rpm,
        current: msg.current,
        driver_voltage: msg.driver_voltage,
        driver_temperature: msg.driver_temperature,
        motor_temperature: msg.motor_temperature,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      rpm: msg.rpm,
      current: msg.current,
      driver_voltage: msg.driver_voltage,
      driver_temperature: msg.driver_temperature,
      motor_temperature: msg.motor_temperature,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      rpm: msg.rpm,
      current: msg.current,
      driver_voltage: msg.driver_voltage,
      driver_temperature: msg.driver_temperature,
      motor_temperature: msg.motor_temperature,
    }
  }
}


// Corresponds to scout_msgs__msg__ScoutLightCmd

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ScoutLightCmd::default())
  }
}

impl rosidl_runtime_rs::Message for ScoutLightCmd {
  type RmwMsg = super::msg::rmw::ScoutLightCmd;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cmd_ctrl_allowed: msg.cmd_ctrl_allowed,
        front_mode: msg.front_mode,
        front_custom_value: msg.front_custom_value,
        rear_mode: msg.rear_mode,
        rear_custom_value: msg.rear_custom_value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      cmd_ctrl_allowed: msg.cmd_ctrl_allowed,
      front_mode: msg.front_mode,
      front_custom_value: msg.front_custom_value,
      rear_mode: msg.rear_mode,
      rear_custom_value: msg.rear_custom_value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cmd_ctrl_allowed: msg.cmd_ctrl_allowed,
      front_mode: msg.front_mode,
      front_custom_value: msg.front_custom_value,
      rear_mode: msg.rear_mode,
      rear_custom_value: msg.rear_custom_value,
    }
  }
}


// Corresponds to scout_msgs__msg__ScoutLightState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ScoutLightState::default())
  }
}

impl rosidl_runtime_rs::Message for ScoutLightState {
  type RmwMsg = super::msg::rmw::ScoutLightState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        custom_value: msg.custom_value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      custom_value: msg.custom_value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      custom_value: msg.custom_value,
    }
  }
}


// Corresponds to scout_msgs__msg__ScoutRCState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ScoutRCState::default())
  }
}

impl rosidl_runtime_rs::Message for ScoutRCState {
  type RmwMsg = super::msg::rmw::ScoutRCState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        swa: msg.swa,
        swb: msg.swb,
        swc: msg.swc,
        swd: msg.swd,
        stick_right_v: msg.stick_right_v,
        stick_right_h: msg.stick_right_h,
        stick_left_v: msg.stick_left_v,
        stick_left_h: msg.stick_left_h,
        var_a: msg.var_a,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      swa: msg.swa,
      swb: msg.swb,
      swc: msg.swc,
      swd: msg.swd,
      stick_right_v: msg.stick_right_v,
      stick_right_h: msg.stick_right_h,
      stick_left_v: msg.stick_left_v,
      stick_left_h: msg.stick_left_h,
      var_a: msg.var_a,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      swa: msg.swa,
      swb: msg.swb,
      swc: msg.swc,
      swd: msg.swd,
      stick_right_v: msg.stick_right_v,
      stick_right_h: msg.stick_right_h,
      stick_left_v: msg.stick_left_v,
      stick_left_h: msg.stick_left_h,
      var_a: msg.var_a,
    }
  }
}


// Corresponds to scout_msgs__msg__ScoutStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ScoutStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

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
    pub actuator_states: [super::msg::ScoutActuatorState; 4],

    /// light state
    pub light_control_enabled: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_light_state: super::msg::ScoutLightState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_light_state: super::msg::ScoutLightState,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ScoutStatus::default())
  }
}

impl rosidl_runtime_rs::Message for ScoutStatus {
  type RmwMsg = super::msg::rmw::ScoutStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        linear_velocity: msg.linear_velocity,
        angular_velocity: msg.angular_velocity,
        vehicle_state: msg.vehicle_state,
        control_mode: msg.control_mode,
        error_code: msg.error_code,
        battery_voltage: msg.battery_voltage,
        actuator_states: msg.actuator_states
          .map(|elem| super::msg::ScoutActuatorState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
        light_control_enabled: msg.light_control_enabled,
        front_light_state: super::msg::ScoutLightState::into_rmw_message(std::borrow::Cow::Owned(msg.front_light_state)).into_owned(),
        rear_light_state: super::msg::ScoutLightState::into_rmw_message(std::borrow::Cow::Owned(msg.rear_light_state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      linear_velocity: msg.linear_velocity,
      angular_velocity: msg.angular_velocity,
      vehicle_state: msg.vehicle_state,
      control_mode: msg.control_mode,
      error_code: msg.error_code,
      battery_voltage: msg.battery_voltage,
        actuator_states: msg.actuator_states
          .iter()
          .map(|elem| super::msg::ScoutActuatorState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
      light_control_enabled: msg.light_control_enabled,
        front_light_state: super::msg::ScoutLightState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.front_light_state)).into_owned(),
        rear_light_state: super::msg::ScoutLightState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.rear_light_state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      linear_velocity: msg.linear_velocity,
      angular_velocity: msg.angular_velocity,
      vehicle_state: msg.vehicle_state,
      control_mode: msg.control_mode,
      error_code: msg.error_code,
      battery_voltage: msg.battery_voltage,
      actuator_states: msg.actuator_states
        .map(super::msg::ScoutActuatorState::from_rmw_message),
      light_control_enabled: msg.light_control_enabled,
      front_light_state: super::msg::ScoutLightState::from_rmw_message(msg.front_light_state),
      rear_light_state: super::msg::ScoutLightState::from_rmw_message(msg.rear_light_state),
    }
  }
}


