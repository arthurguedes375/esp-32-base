use esp_idf_hal::{
    adc::{AdcChannel, ADCU1},
    gpio::ADCPin,
};

use crate::physical::converters::adc::ADC;

pub struct LightAnalogSensor {}

impl LightAnalogSensor {
    pub fn start<P: ADCPin>(adc: &mut ADC, sensor_pin: P) -> anyhow::Result<u16>
    where
        P::AdcChannel: AdcChannel<AdcUnit = ADCU1>,
    {
        /* let result = adc.read(sensor_pin)?;

        Ok(result) */
        Ok(1)
    }
}
