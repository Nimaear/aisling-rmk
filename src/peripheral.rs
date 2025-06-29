#![no_main]
#![no_std]

#[macro_use]
mod macros;

mod keymap;

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::{UART0, USB};
use embassy_rp::uart::{self, BufferedUart};
use embassy_rp::usb::InterruptHandler;
use keymap::{COL, ROW};
use rmk::channel::EVENT_CHANNEL;
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::join;
use rmk::matrix::Matrix;
use rmk::run_devices;
use rmk::split::central::run_peripheral_manager;
use rmk::split::SPLIT_MESSAGE_MAX_SIZE;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    UART0_IRQ => uart::BufferedInterruptHandler<UART0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let (input_pins, output_pins) = config_matrix_pins_rp!(peripherals: p,
        input: [PIN_23, PIN_22, PIN_21, PIN_20, PIN_19],
        output: [PIN_29, PIN_28, PIN_27, PIN_26, PIN_25, PIN_24, PIN_14, PIN_13, PIN_15, PIN_16, PIN_17, PIN_18]
    );

    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    let uart_instance = BufferedUart::new(
        p.UART0,
        Irqs,
        p.PIN_0,
        p.PIN_1,
        tx_buf,
        rx_buf,
        uart::Config::default(),
    );

    let debouncer = DefaultDebouncer::<ROW, COL>::new();
    let mut matrix = Matrix::<_, _, _, ROW, COL>::new(input_pins, output_pins, debouncer);

    join(
        run_devices!((matrix) => EVENT_CHANNEL),
        run_peripheral_manager::<ROW, COL, 0, 12, _>(0, uart_instance),
    )
    .await;
}
