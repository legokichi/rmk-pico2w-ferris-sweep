//! Register IDs and bit definitions for IQS5xx.

pub const PRODUCT_NUMBER_0: u16 = 0x0000;
pub const GESTURE_EVENTS_0: u16 = 0x000D;
pub const GESTURE_EVENTS_1: u16 = 0x000E;
pub const SYSTEM_INFO_0: u16 = 0x000F;
pub const SYSTEM_INFO_1: u16 = 0x0010;
pub const NUM_FINGERS: u16 = 0x0011;
pub const REL_X: u16 = 0x0012;
pub const REL_Y: u16 = 0x0014;
pub const ABS_X: u16 = 0x0016;
pub const ABS_Y: u16 = 0x0018;
pub const TOUCH_STRENGTH: u16 = 0x001A;
pub const TOUCH_AREA: u16 = 0x001C;

pub const SYSTEM_CONTROL_0: u16 = 0x0431;

pub const SYSTEM_CONFIG_0: u16 = 0x058E;
pub const SYSTEM_CONFIG_1: u16 = 0x058F;

pub const FILTER_SETTINGS: u16 = 0x0632;
pub const BOTTOM_BETA: u16 = 0x0637;
pub const XY_CONFIG_0: u16 = 0x0669;
pub const STATIONARY_THRESHOLD: u16 = 0x0672;

pub const SINGLE_FINGER_GESTURES_CONF: u16 = 0x06B7;
pub const MULTI_FINGER_GESTURES_CONF: u16 = 0x06B8;
pub const HOLD_TIME: u16 = 0x06BD;

pub const END_WINDOW: u16 = 0xEEEE;

// SYSTEM_CONTROL_0 bits
pub const SYSTEM_CONTROL_0_ACK_RESET: u8 = 1 << 7;

// SYSTEM_CONFIG_0 bits
pub const SYSTEM_CONFIG_0_SETUP_COMPLETE: u8 = 1 << 6;
pub const SYSTEM_CONFIG_0_WDT: u8 = 1 << 5;

// SYSTEM_CONFIG_1 bits
pub const SYSTEM_CONFIG_1_EVENT_MODE: u8 = 1 << 0;
pub const SYSTEM_CONFIG_1_GESTURE_EVENT: u8 = 1 << 1;
pub const SYSTEM_CONFIG_1_TP_EVENT: u8 = 1 << 2;

// FILTER_SETTINGS bits
pub const FILTER_IIR: u8 = 1 << 0;
pub const FILTER_MAV: u8 = 1 << 1;
pub const FILTER_IIR_SELECT: u8 = 1 << 2;
pub const FILTER_ALP_COUNT: u8 = 1 << 3;

// SYSTEM_INFO_0 bits
pub const SYSTEM_INFO_0_SHOW_RESET: u8 = 1 << 7;

// SYSTEM_INFO_1 bits
pub const SYSTEM_INFO_1_TP_MOVEMENT: u8 = 1 << 0;

// GESTURE events (single finger)
pub const GESTURE_SINGLE_TAP: u8 = 1 << 0;
pub const GESTURE_PRESS_HOLD: u8 = 1 << 1;
pub const GESTURE_SWIPE_LEFT: u8 = 1 << 2;
pub const GESTURE_SWIPE_RIGHT: u8 = 1 << 3;
pub const GESTURE_SWIPE_UP: u8 = 1 << 4;
pub const GESTURE_SWIPE_DOWN: u8 = 1 << 5;

// GESTURE events (multi finger)
pub const GESTURE_TWO_FINGER_TAP: u8 = 1 << 0;
pub const GESTURE_SCROLL: u8 = 1 << 1;
pub const GESTURE_ZOOM: u8 = 1 << 2;

// XY_CONFIG_0 bits
pub const XY_CONFIG_FLIP_X: u8 = 1 << 0;
pub const XY_CONFIG_FLIP_Y: u8 = 1 << 1;
pub const XY_CONFIG_SWITCH_XY: u8 = 1 << 2;
