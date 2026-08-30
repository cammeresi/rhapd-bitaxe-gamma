//! EMC2101 fan controller.

use esp_hal::i2c::master::Error;

use crate::I2cDriver;

const ADDRESS: u8 = 0x4c;

const FAN_CONFIG: u8 = 0x4a;
const FAN_SETTING: u8 = 0x4c;

// PWM duty from FAN_SETTING, normal polarity.
const FAN_CONFIG_MANUAL: u8 = 0x20;
const DUTY_OFF: u8 = 0x00;
const DUTY_FULL: u8 = 0x3f;

/// Turns the fan off.
pub async fn turn_off_fan(i2c: &mut I2cDriver) -> Result<(), Error> {
    set_fan(i2c, DUTY_OFF).await
}

/// Runs the fan at full speed.
pub async fn turn_on_fan(i2c: &mut I2cDriver) -> Result<(), Error> {
    set_fan(i2c, DUTY_FULL).await
}

async fn set_fan(i2c: &mut I2cDriver, duty: u8) -> Result<(), Error> {
    i2c.write_async(ADDRESS, &[FAN_CONFIG, FAN_CONFIG_MANUAL])
        .await?;
    i2c.write_async(ADDRESS, &[FAN_SETTING, duty]).await
}
