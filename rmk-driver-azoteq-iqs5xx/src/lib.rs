#![no_std]

use embassy_time::{Duration, Timer};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::i2c::I2c;

pub mod registers;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    I2c(E),
    Gpio,
    Timeout,
}

pub type Result<T, E> = core::result::Result<T, Error<E>>;

#[derive(Debug, Clone, Copy, Default)]
pub struct Touch {
    pub abs_x: u16,
    pub abs_y: u16,
    pub strength: u16,
    pub size: u8,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Report {
    pub events0: u8,
    pub events1: u8,
    pub sys_info0: u8,
    pub sys_info1: u8,
    pub num_fingers: u8,
    pub rel_x: i16,
    pub rel_y: i16,
    pub touches: [Touch; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Event {
    None,
    Move { x: i16, y: i16 },
    SingleTap { x: u16, y: u16 },
    PressHold { x: i16, y: i16 },
    SwipeX(i16),
    SwipeY(i16),
    TwoFingerTap,
    Scroll { x: i16, y: i16 },
    Zoom(i16),
    Invalid,
}

impl Event {
    pub fn from_report(report: &Report) -> Self {
        match (report.events0, report.events1) {
            (0, 0) if report.rel_x == 0 && report.rel_y == 0 => Self::None,
            (0, 0) => Self::Move {
                x: report.rel_x,
                y: report.rel_y,
            },
            (registers::GESTURE_SINGLE_TAP, 0) => Self::SingleTap {
                x: report.touches[0].abs_x,
                y: report.touches[0].abs_y,
            },
            (registers::GESTURE_PRESS_HOLD, 0) => Self::PressHold {
                x: report.rel_x,
                y: report.rel_y,
            },
            (registers::GESTURE_SWIPE_RIGHT, 0) => Self::SwipeX(report.rel_x),
            (registers::GESTURE_SWIPE_LEFT, 0) => Self::SwipeX(report.rel_x),
            (registers::GESTURE_SWIPE_DOWN, 0) => Self::SwipeY(report.rel_y),
            (registers::GESTURE_SWIPE_UP, 0) => Self::SwipeY(report.rel_y),
            (0, registers::GESTURE_TWO_FINGER_TAP) => Self::TwoFingerTap,
            (0, registers::GESTURE_SCROLL) => Self::Scroll {
                x: report.rel_x,
                y: report.rel_y,
            },
            (0, registers::GESTURE_ZOOM) => Self::Zoom(report.rel_x),
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Iqs5xxConfig {
    pub addr: u8,
    pub ready_timeout_ms: u32,
    pub reset_low_ms: u16,
    pub reset_high_ms: u16,
    pub enable_single_tap: bool,
    pub enable_press_and_hold: bool,
    pub press_and_hold_time_ms: u16,
    pub enable_two_finger_tap: bool,
    pub enable_scroll: bool,
    pub invert_x: bool,
    pub invert_y: bool,
    pub swap_xy: bool,
    pub bottom_beta: u8,
    pub stationary_threshold: u8,
}

impl Default for Iqs5xxConfig {
    fn default() -> Self {
        Self {
            addr: 0x74,
            ready_timeout_ms: 100,
            reset_low_ms: 10,
            reset_high_ms: 10,
            enable_single_tap: true,
            enable_press_and_hold: true,
            press_and_hold_time_ms: 250,
            enable_two_finger_tap: true,
            enable_scroll: true,
            invert_x: false,
            invert_y: false,
            swap_xy: false,
            bottom_beta: 5,
            stationary_threshold: 5,
        }
    }
}

pub struct Iqs5xx<I2C, RDY, RST> {
    i2c: I2C,
    rdy_pin: Option<RDY>,
    rst_pin: Option<RST>,
    config: Iqs5xxConfig,
}

impl<I2C, RDY, RST> Iqs5xx<I2C, RDY, RST> {
    pub fn new(i2c: I2C, rdy_pin: Option<RDY>, rst_pin: Option<RST>, config: Iqs5xxConfig) -> Self {
        Self {
            i2c,
            rdy_pin,
            rst_pin,
            config,
        }
    }

    pub fn into_inner(self) -> (I2C, Option<RDY>, Option<RST>) {
        (self.i2c, self.rdy_pin, self.rst_pin)
    }

    pub fn config(&self) -> &Iqs5xxConfig {
        &self.config
    }
}

impl<I2C, RDY, RST> Iqs5xx<I2C, RDY, RST>
where
    I2C: I2c,
{
    async fn write_reg_u8(&mut self, reg: u16, value: u8) -> Result<(), I2C::Error> {
        let mut buf = [0u8; 3];
        buf[0..2].copy_from_slice(&reg.to_be_bytes());
        buf[2] = value;
        self.i2c
            .write(self.config.addr, &buf)
            .await
            .map_err(Error::I2c)
    }

    async fn write_reg_u16(&mut self, reg: u16, value: u16) -> Result<(), I2C::Error> {
        let mut buf = [0u8; 4];
        buf[0..2].copy_from_slice(&reg.to_be_bytes());
        buf[2..4].copy_from_slice(&value.to_be_bytes());
        self.i2c
            .write(self.config.addr, &buf)
            .await
            .map_err(Error::I2c)
    }

    #[allow(dead_code)]
    async fn read_reg_u8(&mut self, reg: u16) -> Result<u8, I2C::Error> {
        let mut rd_buf = [0u8; 1];
        self.i2c
            .write_read(self.config.addr, &reg.to_be_bytes(), &mut rd_buf)
            .await
            .map_err(Error::I2c)?;
        Ok(rd_buf[0])
    }

    #[allow(dead_code)]
    async fn read_reg_u16(&mut self, reg: u16) -> Result<u16, I2C::Error> {
        let mut rd_buf = [0u8; 2];
        self.i2c
            .write_read(self.config.addr, &reg.to_be_bytes(), &mut rd_buf)
            .await
            .map_err(Error::I2c)?;
        Ok(u16::from_be_bytes(rd_buf))
    }

    pub async fn end_session(&mut self) -> Result<(), I2C::Error> {
        let mut buf = [0u8; 3];
        buf[0..2].copy_from_slice(&registers::END_WINDOW.to_be_bytes());
        self.i2c
            .write(self.config.addr, &buf)
            .await
            .map_err(Error::I2c)
    }

    async fn read_report_now(&mut self) -> Result<Report, I2C::Error> {
        let mut rd_buf = [0u8; 44];
        self.i2c
            .write_read(
                self.config.addr,
                &registers::GESTURE_EVENTS_0.to_be_bytes(),
                &mut rd_buf,
            )
            .await
            .map_err(Error::I2c)?;
        let mut r = rd_buf.iter();
        Ok(Report {
            events0: *r.next().unwrap_or(&0),
            events1: *r.next().unwrap_or(&0),
            sys_info0: *r.next().unwrap_or(&0),
            sys_info1: *r.next().unwrap_or(&0),
            num_fingers: *r.next().unwrap_or(&0),
            rel_x: i16_be_from_iter(&mut r),
            rel_y: i16_be_from_iter(&mut r),
            touches: [
                Touch::from_iter(&mut r),
                Touch::from_iter(&mut r),
                Touch::from_iter(&mut r),
                Touch::from_iter(&mut r),
                Touch::from_iter(&mut r),
            ],
        })
    }
}

impl<I2C, RDY, RST> Iqs5xx<I2C, RDY, RST>
where
    I2C: I2c,
    RDY: InputPin,
{
    pub async fn read_report(&mut self) -> Result<Report, I2C::Error> {
        self.wait_ready().await?;
        let report = self.read_report_now().await?;
        self.end_session().await?;
        self.wait_ready_low().await?;
        Ok(report)
    }

    pub async fn try_read_report(&mut self) -> Result<Option<Report>, I2C::Error> {
        if !self.is_ready()? {
            return Ok(None);
        }
        let report = self.read_report_now().await?;
        self.end_session().await?;
        Ok(Some(report))
    }

    pub fn is_ready(&mut self) -> Result<bool, I2C::Error> {
        match &mut self.rdy_pin {
            Some(pin) => pin.is_high().map_err(|_| Error::Gpio),
            None => Ok(true),
        }
    }

    pub async fn wait_ready(&mut self) -> Result<(), I2C::Error> {
        let mut waited_ms = 0u32;
        loop {
            if self.is_ready()? {
                return Ok(());
            }
            if waited_ms >= self.config.ready_timeout_ms {
                return Err(Error::Timeout);
            }
            Timer::after(Duration::from_millis(1)).await;
            waited_ms += 1;
        }
    }

    pub async fn wait_ready_low(&mut self) -> Result<(), I2C::Error> {
        let mut waited_ms = 0u32;
        loop {
            let is_low = match &mut self.rdy_pin {
                Some(pin) => pin.is_low().map_err(|_| Error::Gpio)?,
                None => true,
            };
            if is_low {
                return Ok(());
            }
            if waited_ms >= self.config.ready_timeout_ms {
                return Err(Error::Timeout);
            }
            Timer::after(Duration::from_millis(1)).await;
            waited_ms += 1;
        }
    }
}

impl<I2C, RDY, RST> Iqs5xx<I2C, RDY, RST>
where
    I2C: I2c,
    RST: OutputPin,
{
    pub async fn reset(&mut self) -> Result<(), I2C::Error> {
        if let Some(pin) = &mut self.rst_pin {
            pin.set_low().map_err(|_| Error::Gpio)?;
            Timer::after(Duration::from_millis(self.config.reset_low_ms as u64)).await;
            pin.set_high().map_err(|_| Error::Gpio)?;
            Timer::after(Duration::from_millis(self.config.reset_high_ms as u64)).await;
        }
        Ok(())
    }
}

impl<I2C, RDY, RST> Iqs5xx<I2C, RDY, RST>
where
    I2C: I2c,
    RDY: InputPin,
    RST: OutputPin,
{
    pub async fn init(&mut self) -> Result<(), I2C::Error> {
        self.reset().await?;

        self.wait_ready().await?;
        self.write_reg_u8(
            registers::SYSTEM_CONTROL_0,
            registers::SYSTEM_CONTROL_0_ACK_RESET,
        )
        .await?;

        self.write_reg_u8(
            registers::SYSTEM_CONFIG_1,
            registers::SYSTEM_CONFIG_1_EVENT_MODE
                | registers::SYSTEM_CONFIG_1_TP_EVENT
                | registers::SYSTEM_CONFIG_1_GESTURE_EVENT,
        )
        .await?;

        self.write_reg_u8(registers::BOTTOM_BETA, self.config.bottom_beta)
            .await?;
        self.write_reg_u8(
            registers::STATIONARY_THRESHOLD,
            self.config.stationary_threshold,
        )
        .await?;

        let filter_settings =
            registers::FILTER_IIR | registers::FILTER_MAV | registers::FILTER_ALP_COUNT;
        self.write_reg_u8(registers::FILTER_SETTINGS, filter_settings)
            .await?;

        let mut single = 0u8;
        if self.config.enable_single_tap {
            single |= registers::GESTURE_SINGLE_TAP;
        }
        if self.config.enable_press_and_hold {
            single |= registers::GESTURE_PRESS_HOLD;
        }
        self.write_reg_u8(registers::SINGLE_FINGER_GESTURES_CONF, single)
            .await?;
        self.write_reg_u16(registers::HOLD_TIME, self.config.press_and_hold_time_ms)
            .await?;

        let mut multi = 0u8;
        if self.config.enable_two_finger_tap {
            multi |= registers::GESTURE_TWO_FINGER_TAP;
        }
        if self.config.enable_scroll {
            multi |= registers::GESTURE_SCROLL;
        }
        self.write_reg_u8(registers::MULTI_FINGER_GESTURES_CONF, multi)
            .await?;

        let mut xy_config = 0u8;
        if self.config.invert_x {
            xy_config |= registers::XY_CONFIG_FLIP_X;
        }
        if self.config.invert_y {
            xy_config |= registers::XY_CONFIG_FLIP_Y;
        }
        if self.config.swap_xy {
            xy_config |= registers::XY_CONFIG_SWITCH_XY;
        }
        self.write_reg_u8(registers::XY_CONFIG_0, xy_config).await?;

        self.write_reg_u8(
            registers::SYSTEM_CONFIG_0,
            registers::SYSTEM_CONFIG_0_SETUP_COMPLETE | registers::SYSTEM_CONFIG_0_WDT,
        )
        .await?;

        self.end_session().await?;
        Ok(())
    }
}

impl Touch {
    fn from_iter<'a, I: core::iter::Iterator<Item = &'a u8>>(i: &mut I) -> Touch {
        Touch {
            abs_x: u16_be_from_iter(i),
            abs_y: u16_be_from_iter(i),
            strength: u16_be_from_iter(i),
            size: *i.next().unwrap_or(&0),
        }
    }
}

fn u16_be_from_iter<'a, I: core::iter::Iterator<Item = &'a u8>>(i: &mut I) -> u16 {
    let hi = *i.next().unwrap_or(&0) as u16;
    let lo = *i.next().unwrap_or(&0) as u16;
    (hi << 8) | lo
}

fn i16_be_from_iter<'a, I: core::iter::Iterator<Item = &'a u8>>(i: &mut I) -> i16 {
    u16_be_from_iter(i) as i16
}

#[cfg(feature = "rmk")]
pub mod rmk_support {
    use super::{Event as IqsEvent, Iqs5xx, Iqs5xxConfig};
    use ::rmk::channel::KEYBOARD_REPORT_CHANNEL;
    use ::rmk::event::{Axis, AxisEvent, AxisValType, Event};
    use ::rmk::hid::Report as RmkReport;
    use ::rmk::input_device::{InputDevice, InputProcessor, ProcessResult};
    use ::rmk::keymap::KeyMap;
    use embassy_time::{Duration, Timer};
    use embedded_hal::digital::{InputPin, OutputPin};
    use embedded_hal_async::i2c::I2c;
    use usbd_hid::descriptor::MouseReport;

    const CUSTOM_TAG_CLICK: u8 = 1;
    const CUSTOM_TAG_BUTTON: u8 = 2;
    const BUTTON_LEFT: u8 = 1;
    const BUTTON_RIGHT: u8 = 2;

    #[derive(Debug, Clone, Copy)]
    pub struct Iqs5xxProcessorConfig {
        pub scroll_divisor: i16,
        pub natural_scroll_x: bool,
        pub natural_scroll_y: bool,
    }

    impl Default for Iqs5xxProcessorConfig {
        fn default() -> Self {
            Self {
                scroll_divisor: 32,
                natural_scroll_x: false,
                natural_scroll_y: false,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum InitState {
        Pending,
        Initializing(u8),
        Ready,
        Failed,
    }

    pub struct Iqs5xxDevice<I2C, RDY, RST>
    where
        I2C: I2c,
        RDY: InputPin,
        RST: OutputPin,
    {
        driver: Iqs5xx<I2C, RDY, RST>,
        init_state: InitState,
        poll_interval: Duration,
        hold_active: bool,
    }

    impl<I2C, RDY, RST> Iqs5xxDevice<I2C, RDY, RST>
    where
        I2C: I2c,
        RDY: InputPin,
        RST: OutputPin,
    {
        const MAX_INIT_RETRIES: u8 = 3;

        pub fn new(i2c: I2C, rdy: RDY, rst: RST, config: Iqs5xxConfig) -> Self {
            Self {
                driver: Iqs5xx::new(i2c, Some(rdy), Some(rst), config),
                init_state: InitState::Pending,
                poll_interval: Duration::from_millis(5),
                hold_active: false,
            }
        }

        pub fn with_poll_interval(
            i2c: I2C,
            rdy: RDY,
            rst: RST,
            config: Iqs5xxConfig,
            poll_interval_ms: u64,
        ) -> Self {
            Self {
                driver: Iqs5xx::new(i2c, Some(rdy), Some(rst), config),
                init_state: InitState::Pending,
                poll_interval: Duration::from_millis(poll_interval_ms),
                hold_active: false,
            }
        }

        async fn try_init(&mut self) -> bool {
            match self.init_state {
                InitState::Ready => return true,
                InitState::Failed => return false,
                InitState::Pending => {
                    self.init_state = InitState::Initializing(0);
                }
                InitState::Initializing(_) => {}
            }

            if let InitState::Initializing(retry) = self.init_state {
                if self.driver.init().await.is_ok() {
                    self.init_state = InitState::Ready;
                    return true;
                }

                if retry + 1 >= Self::MAX_INIT_RETRIES {
                    self.init_state = InitState::Failed;
                    return false;
                }

                self.init_state = InitState::Initializing(retry + 1);
                Timer::after(Duration::from_millis(100)).await;
            }

            false
        }

        fn custom_click(button: u8) -> Event {
            let mut data = [0u8; 16];
            data[0] = CUSTOM_TAG_CLICK;
            data[1] = button;
            Event::Custom(data)
        }

        fn custom_button(button: u8, pressed: bool) -> Event {
            let mut data = [0u8; 16];
            data[0] = CUSTOM_TAG_BUTTON;
            data[1] = button;
            data[2] = pressed as u8;
            Event::Custom(data)
        }
    }

    impl<I2C, RDY, RST> InputDevice for Iqs5xxDevice<I2C, RDY, RST>
    where
        I2C: I2c,
        RDY: InputPin,
        RST: OutputPin,
    {
        async fn read_event(&mut self) -> Event {
            loop {
                Timer::after(self.poll_interval).await;

                if self.init_state != InitState::Ready && !self.try_init().await {
                    continue;
                }

                let report = match self.driver.try_read_report().await {
                    Ok(Some(report)) => report,
                    Ok(None) => continue,
                    Err(_) => continue,
                };

                if report.sys_info0 & super::registers::SYSTEM_INFO_0_SHOW_RESET != 0 {
                    let _ = self.driver.wait_ready().await;
                    let _ = self
                        .driver
                        .write_reg_u8(
                            super::registers::SYSTEM_CONTROL_0,
                            super::registers::SYSTEM_CONTROL_0_ACK_RESET,
                        )
                        .await;
                    let _ = self.driver.end_session().await;
                    continue;
                }

                let hold_now = (report.events0 & super::registers::GESTURE_PRESS_HOLD) != 0;
                if hold_now && !self.hold_active {
                    self.hold_active = true;
                    return Self::custom_button(BUTTON_LEFT, true);
                }
                if !hold_now && self.hold_active {
                    self.hold_active = false;
                    return Self::custom_button(BUTTON_LEFT, false);
                }

                match IqsEvent::from_report(&report) {
                    IqsEvent::None | IqsEvent::Invalid => continue,
                    IqsEvent::Move { x, y } => {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        return Event::Touchpad(::rmk::event::TouchpadEvent {
                            finger: 0,
                            axis: [
                                AxisEvent {
                                    typ: AxisValType::Rel,
                                    axis: Axis::X,
                                    value: x,
                                },
                                AxisEvent {
                                    typ: AxisValType::Rel,
                                    axis: Axis::Y,
                                    value: y,
                                },
                                AxisEvent {
                                    typ: AxisValType::Rel,
                                    axis: Axis::Z,
                                    value: 0,
                                },
                            ],
                        });
                    }
                    IqsEvent::Scroll { x, y } => {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        return Event::Touchpad(::rmk::event::TouchpadEvent {
                            finger: 0,
                            axis: [
                                AxisEvent {
                                    typ: AxisValType::Rel,
                                    axis: Axis::H,
                                    value: x,
                                },
                                AxisEvent {
                                    typ: AxisValType::Rel,
                                    axis: Axis::V,
                                    value: y,
                                },
                                AxisEvent {
                                    typ: AxisValType::Rel,
                                    axis: Axis::Z,
                                    value: 0,
                                },
                            ],
                        });
                    }
                    IqsEvent::SingleTap { .. } => return Self::custom_click(BUTTON_LEFT),
                    IqsEvent::TwoFingerTap => return Self::custom_click(BUTTON_RIGHT),
                    _ => continue,
                }
            }
        }
    }

    pub struct Iqs5xxProcessor<
        'a,
        const ROW: usize,
        const COL: usize,
        const NUM_LAYER: usize,
        const NUM_ENCODER: usize,
    > {
        keymap: &'a core::cell::RefCell<KeyMap<'a, ROW, COL, NUM_LAYER, NUM_ENCODER>>,
        config: Iqs5xxProcessorConfig,
        buttons: u8,
        scroll_x_acc: i16,
        scroll_y_acc: i16,
    }

    impl<'a, const ROW: usize, const COL: usize, const NUM_LAYER: usize, const NUM_ENCODER: usize>
        Iqs5xxProcessor<'a, ROW, COL, NUM_LAYER, NUM_ENCODER>
    {
        pub fn new(
            keymap: &'a core::cell::RefCell<KeyMap<'a, ROW, COL, NUM_LAYER, NUM_ENCODER>>,
            config: Iqs5xxProcessorConfig,
        ) -> Self {
            Self {
                keymap,
                config,
                buttons: 0,
                scroll_x_acc: 0,
                scroll_y_acc: 0,
            }
        }

        async fn send_mouse_report(&self, report: MouseReport) {
            KEYBOARD_REPORT_CHANNEL
                .send(RmkReport::MouseReport(report))
                .await;
        }

        async fn send_button_state(&self) {
            let report = MouseReport {
                buttons: self.buttons,
                x: 0,
                y: 0,
                wheel: 0,
                pan: 0,
            };
            self.send_mouse_report(report).await;
        }

        async fn handle_click(&mut self, button: u8) {
            self.buttons |= button_mask(button);
            self.send_button_state().await;
            self.buttons &= !button_mask(button);
            self.send_button_state().await;
        }

        async fn handle_button(&mut self, button: u8, pressed: bool) {
            if pressed {
                self.buttons |= button_mask(button);
            } else {
                self.buttons &= !button_mask(button);
            }
            self.send_button_state().await;
        }

        async fn handle_motion(&self, x: i16, y: i16) {
            let report = MouseReport {
                buttons: self.buttons,
                x: x.clamp(i8::MIN as i16, i8::MAX as i16) as i8,
                y: y.clamp(i8::MIN as i16, i8::MAX as i16) as i8,
                wheel: 0,
                pan: 0,
            };
            self.send_mouse_report(report).await;
        }

        async fn handle_scroll(&mut self, mut x: i16, mut y: i16) {
            if self.config.natural_scroll_x {
                x = -x;
            }
            if self.config.natural_scroll_y {
                y = -y;
            }

            self.scroll_x_acc = self.scroll_x_acc.saturating_add(x);
            self.scroll_y_acc = self.scroll_y_acc.saturating_add(y);

            let mut pan = 0i16;
            let mut wheel = 0i16;
            let div = self.config.scroll_divisor.max(1);

            if self.scroll_x_acc.abs() >= div {
                pan = self.scroll_x_acc / div;
                self.scroll_x_acc %= div;
            }
            if self.scroll_y_acc.abs() >= div {
                wheel = self.scroll_y_acc / div;
                self.scroll_y_acc %= div;
            }

            if pan != 0 || wheel != 0 {
                let report = MouseReport {
                    buttons: self.buttons,
                    x: 0,
                    y: 0,
                    wheel: wheel.clamp(i8::MIN as i16, i8::MAX as i16) as i8,
                    pan: pan.clamp(i8::MIN as i16, i8::MAX as i16) as i8,
                };
                self.send_mouse_report(report).await;
            }
        }
    }

    impl<'a, const ROW: usize, const COL: usize, const NUM_LAYER: usize, const NUM_ENCODER: usize>
        InputProcessor<'a, ROW, COL, NUM_LAYER, NUM_ENCODER>
        for Iqs5xxProcessor<'a, ROW, COL, NUM_LAYER, NUM_ENCODER>
    {
        async fn process(&mut self, event: Event) -> ProcessResult {
            match event {
                Event::Touchpad(tp) => {
                    let mut x = 0i16;
                    let mut y = 0i16;
                    let mut h = 0i16;
                    let mut v = 0i16;

                    for axis_event in tp.axis.iter() {
                        match axis_event.axis {
                            Axis::X => x = axis_event.value,
                            Axis::Y => y = axis_event.value,
                            Axis::H => h = axis_event.value,
                            Axis::V => v = axis_event.value,
                            _ => {}
                        }
                    }

                    if h != 0 || v != 0 {
                        self.handle_scroll(h, v).await;
                        return ProcessResult::Stop;
                    }

                    if x != 0 || y != 0 {
                        self.handle_motion(x, y).await;
                        return ProcessResult::Stop;
                    }

                    ProcessResult::Stop
                }
                Event::Custom(data) => match data[0] {
                    CUSTOM_TAG_CLICK => {
                        self.handle_click(data[1]).await;
                        ProcessResult::Stop
                    }
                    CUSTOM_TAG_BUTTON => {
                        self.handle_button(data[1], data[2] != 0).await;
                        ProcessResult::Stop
                    }
                    _ => ProcessResult::Continue(event),
                },
                _ => ProcessResult::Continue(event),
            }
        }

        async fn send_report(&self, report: RmkReport) {
            KEYBOARD_REPORT_CHANNEL.send(report).await;
        }

        fn get_keymap(&self) -> &core::cell::RefCell<KeyMap<'a, ROW, COL, NUM_LAYER, NUM_ENCODER>> {
            self.keymap
        }
    }

    fn button_mask(button: u8) -> u8 {
        match button {
            BUTTON_LEFT => 1 << 0,
            BUTTON_RIGHT => 1 << 1,
            _ => 0,
        }
    }
}
