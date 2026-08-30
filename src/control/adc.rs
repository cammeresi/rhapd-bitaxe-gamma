use esp_hal::analog::adc::{Adc, AdcPin};
use heapless::Vec;

use super::CommandError;

pub struct Pins<'d> {
    pub adc: Adc<'d, esp_hal::peripherals::ADC1<'d>, esp_hal::Blocking>,
    pub vdd: AdcPin<esp_hal::peripherals::GPIO2<'d>, esp_hal::peripherals::ADC1<'d>>,
}

#[derive(defmt::Format)]
pub enum Command {
    ReadVdd, // 0x50
}

impl Command {
    pub fn from_bytes(buf: &[u8]) -> Result<Self, CommandError> {
        //defmt::println!("ADC COMMAND {:x}", buf);
        match buf {
            [0x50] => Ok(Self::ReadVdd),
            _ => Err(CommandError::Invalid),
        }
    }
}

// Keep this string in its own plain (non-async) `inline(never)`
// function to work around an Xtensa LLVM backend bug ("Cannot select:
// XtensaISD::PCREL_WRAPPER" on the constant-pool string) that triggers
// when the string is referenced from the control `usb_task` poll state
// machine under fat LTO release builds. Seen with the esp toolchain's
// LLVM 21.1.3 (Xtensa Rust 1.95.0.0). Remove when the toolchain fixes
// the bug.
#[inline(never)]
fn adc_read_error() -> CommandError {
    CommandError::Message("ADC Read Error")
}

impl super::ControllerCommand for Command {
    async fn handle(
        &self,
        controller: &mut super::Controller,
    ) -> Result<Vec<u8, 256>, CommandError> {
        let adc = &mut controller.adc.adc;

        let value = match self {
            Command::ReadVdd => adc.read_oneshot(&mut controller.adc.vdd),
        }
        .map_err(|_| adc_read_error())?;

        Ok(Vec::from_slice(&value.to_le_bytes()).unwrap())
    }
}
