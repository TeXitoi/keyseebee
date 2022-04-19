#![no_main]
#![no_std]

// set the panic handler
use panic_halt as _;

use core::convert::Infallible;
use embedded_hal::digital::v2::InputPin;
use hal::gpio::{gpiob, Floating, Input, Output, Pin, PullUp, PushPull};
use hal::prelude::*;
use hal::serial;
use hal::usb;
use hal::{stm32, timers};
use keyberon::debounce::Debouncer;
use keyberon::key_code::KbHidReport;
use keyberon::layout::{CustomEvent, Event, Layout};
use keyberon::matrix::Matrix;
use nb::block;
use rtic::app;
use stm32f0xx_hal as hal;
use usb_device::bus::UsbBusAllocator;
use usb_device::class::UsbClass as _;
use usb_device::device::UsbDeviceState;

mod layout;

type UsbClass = keyberon::Class<'static, usb::UsbBusType, ()>;
type UsbDevice = usb_device::device::UsbDevice<'static, usb::UsbBusType>;

trait ResultExt<T> {
    fn get(self) -> T;
}
impl<T> ResultExt<T> for Result<T, Infallible> {
    fn get(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => match e {},
        }
    }
}

#[app(device = crate::hal::pac, peripherals = true, dispatchers = [CEC_CAN])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
        #[lock_free]
        layout: Layout<12, 4, 4, ()>,
    }

    #[local]
    struct Local {
        matrix: Matrix<Pin<Input<PullUp>>, Pin<Output<PushPull>>, 6, 4>,
        debouncer: Debouncer<[[bool; 6]; 4]>,
        timer: timers::Timer<stm32::TIM3>,
        transform: fn(Event) -> Event,
        tx: serial::Tx<hal::pac::USART1>,
        rx: serial::Rx<hal::pac::USART1>,
        buf: [u8; 4],
    }

    #[init(local = [bus: Option<UsbBusAllocator<usb::UsbBusType>> = None])]
    fn init(mut c: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut rcc = c
            .device
            .RCC
            .configure()
            .hsi48()
            .enable_crs(c.device.CRS)
            .sysclk(48.mhz())
            .pclk(24.mhz())
            .freeze(&mut c.device.FLASH);

        let gpioa = c.device.GPIOA.split(&mut rcc);
        let gpiob = c.device.GPIOB.split(&mut rcc);

        let usb = usb::Peripheral {
            usb: c.device.USB,
            pin_dm: gpioa.pa11,
            pin_dp: gpioa.pa12,
        };
        *c.local.bus = Some(usb::UsbBusType::new(usb));
        let usb_bus = c.local.bus.as_ref().unwrap();

        let usb_class = keyberon::new_class(usb_bus, ());
        let usb_dev = keyberon::new_device(usb_bus);

        let mut timer = timers::Timer::tim3(c.device.TIM3, 1.khz(), &mut rcc);
        timer.listen(timers::Event::TimeOut);

        let pb12: &gpiob::PB12<Input<Floating>> = &gpiob.pb12;
        let is_left = pb12.is_low().get();
        let transform: fn(Event) -> Event = if is_left {
            |e| e
        } else {
            |e| e.transform(|i, j| (i, 11 - j))
        };

        let (pa9, pa10) = (gpioa.pa9, gpioa.pa10);
        let pins = cortex_m::interrupt::free(move |cs| {
            (pa9.into_alternate_af1(cs), pa10.into_alternate_af1(cs))
        });
        let mut serial = serial::Serial::usart1(c.device.USART1, pins, 38_400.bps(), &mut rcc);
        serial.listen(serial::Event::Rxne);
        let (tx, rx) = serial.split();

        let pa15 = gpioa.pa15;
        let matrix = cortex_m::interrupt::free(move |cs| {
            Matrix::new(
                [
                    pa15.into_pull_up_input(cs).downgrade(),
                    gpiob.pb3.into_pull_up_input(cs).downgrade(),
                    gpiob.pb4.into_pull_up_input(cs).downgrade(),
                    gpiob.pb5.into_pull_up_input(cs).downgrade(),
                    gpiob.pb8.into_pull_up_input(cs).downgrade(),
                    gpiob.pb9.into_pull_up_input(cs).downgrade(),
                ],
                [
                    gpiob.pb0.into_push_pull_output(cs).downgrade(),
                    gpiob.pb1.into_push_pull_output(cs).downgrade(),
                    gpiob.pb2.into_push_pull_output(cs).downgrade(),
                    gpiob.pb10.into_push_pull_output(cs).downgrade(),
                ],
            )
        });

        (
            Shared {
                usb_dev,
                usb_class,
                layout: Layout::new(&crate::layout::LAYERS),
            },
            Local {
                timer,
                debouncer: Debouncer::new([[false; 6]; 4], [[false; 6]; 4], 5),
                matrix: matrix.get(),
                transform,
                tx,
                rx,
                buf: [0; 4],
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USART1, priority = 4, local = [rx, buf])]
    fn rx(c: rx::Context) {
        if let Ok(b) = c.local.rx.read() {
            c.local.buf.rotate_left(1);
            c.local.buf[3] = b;

            if c.local.buf[3] == b'\n' {
                if let Ok(event) = de(&c.local.buf[..]) {
                    handle_event::spawn(event).unwrap();
                }
            }
        }
    }

    #[task(binds = USB, priority = 3, shared = [usb_dev, usb_class])]
    fn usb_rx(c: usb_rx::Context) {
        (c.shared.usb_dev, c.shared.usb_class).lock(|usb_dev, usb_class| {
            if usb_dev.poll(&mut [usb_class]) {
                usb_class.poll();
            }
        });
    }

    #[task(priority = 2, capacity = 8, shared = [layout])]
    fn handle_event(c: handle_event::Context, event: Event) {
        c.shared.layout.event(event)
    }

    #[task(priority = 2, shared = [usb_dev, usb_class, layout])]
    fn tick_keyberon(mut c: tick_keyberon::Context) {
        let tick = c.shared.layout.tick();
        if c.shared.usb_dev.lock(|d| d.state()) != UsbDeviceState::Configured {
            return;
        }
        match tick {
            CustomEvent::Release(()) => unsafe { cortex_m::asm::bootload(0x1FFFC800 as _) },
            _ => (),
        }
        let report: KbHidReport = c.shared.layout.keycodes().collect();
        if !c
            .shared
            .usb_class
            .lock(|k| k.device_mut().set_keyboard_report(report.clone()))
        {
            return;
        }
        while let Ok(0) = c.shared.usb_class.lock(|k| k.write(report.as_bytes())) {}
    }

    #[task(
        binds = TIM3,
        priority = 1,
        local = [matrix, debouncer, timer, transform, tx],
    )]
    fn tick(c: tick::Context) {
        c.local.timer.wait().ok();

        for event in c
            .local
            .debouncer
            .events(c.local.matrix.get().get())
            .map(c.local.transform)
        {
            for &b in &ser(event) {
                block!(c.local.tx.write(b)).get();
            }
            handle_event::spawn(event).unwrap();
        }
        tick_keyberon::spawn().unwrap();
    }
}

fn de(bytes: &[u8]) -> Result<Event, ()> {
    match *bytes {
        [b'P', i, j, b'\n'] => Ok(Event::Press(i, j)),
        [b'R', i, j, b'\n'] => Ok(Event::Release(i, j)),
        _ => Err(()),
    }
}
fn ser(e: Event) -> [u8; 4] {
    match e {
        Event::Press(i, j) => [b'P', i, j, b'\n'],
        Event::Release(i, j) => [b'R', i, j, b'\n'],
    }
}
