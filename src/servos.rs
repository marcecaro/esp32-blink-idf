
use esp_idf_hal::uart::config::{Config, DataBits, StopBits};
use esp_idf_hal::units::Hertz;
use esp_idf_hal::uart::Uart;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::gpio::{OutputPin, InputPin};
use crate::lewan_bus::LewanSoulBus;


pub fn init_servos<U, UART, TX, RX, P1, P2>(uart: UART, rx: TX, tx: RX) -> anyhow::Result<LewanSoulBus<'static>> 
where
    U: Uart,
    UART: Peripheral<P = U> + 'static,
    P1: OutputPin,
    P2: InputPin,
    TX: Peripheral<P=P1> + 'static,
    RX: Peripheral<P=P2> + 'static,
{
    
   

    // UART1 with default pins (TX=GPIO32, RX=GPIO33)
    let config = Config::default().baudrate(Hertz(115_200))
    .data_bits(DataBits::DataBits8)
    .parity_none()
    .stop_bits(StopBits::STOP1);
    
    let bus = LewanSoulBus::new(
        uart,
        rx,
        tx,
        &config,
    )?;

    Ok(bus)
}