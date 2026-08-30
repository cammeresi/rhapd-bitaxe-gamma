//! TPS546 core voltage regulator.
//!
//! The regulator's NVM defaults leave the core rail on from power-up.
//! This module turns it off; the host turns it on over the control
//! serial port when it wants the ASIC.

use esp_hal::i2c::master::Error;

use crate::I2cDriver;

const ADDRESS: u8 = 0x24;

// PMBus command codes
const OPERATION: u8 = 0x01;
const ON_OFF_CONFIG: u8 = 0x02;

const OPERATION_OFF: u8 = 0x00;
// Output on only while the CONTROL pin is high and OPERATION says on.
// The NVM default, 0x17, follows the CONTROL pin alone, which the
// board ties high, and ignores OPERATION.
const ON_OFF_CONFIG_CONTROL_AND_OPERATION: u8 = 0x1f;

/// Turns the core rail off.
pub async fn turn_off(i2c: &mut I2cDriver) -> Result<(), Error> {
    i2c.write_async(ADDRESS, &[OPERATION, OPERATION_OFF])
        .await?;
    i2c.write_async(
        ADDRESS,
        &[ON_OFF_CONFIG, ON_OFF_CONFIG_CONTROL_AND_OPERATION],
    )
    .await
}
