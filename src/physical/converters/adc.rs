use esp_idf_hal::{
    adc::{
        attenuation::DB_12,
        oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver},
        AdcChannel, Resolution, ADC1, ADCU1,
    },
    gpio::ADCPin,
};

pub struct ADC {
    pub driver: AdcDriver<'static, ADCU1>,
}

impl ADC {
    pub fn new<'b>(adc1: ADC1<'static>) -> anyhow::Result<Self> {
        let driver = AdcDriver::new(adc1)?;

        Ok(Self { driver })
    }
}

pub struct ADCChannel<'d, C>
where
    C: AdcChannel<AdcUnit = ADCU1>,
{
    channel: AdcChannelDriver<'d, C, &'d AdcDriver<'d, ADCU1>>,
}

impl<'d, C> ADCChannel<'d, C>
where
    C: AdcChannel<AdcUnit = ADCU1>,
{
    pub fn new<P>(adc: &'d AdcDriver<'static, ADCU1>, pin: P) -> anyhow::Result<Self>
    where
        P: ADCPin<AdcChannel = C> + 'd,
        P::AdcChannel: AdcChannel<AdcUnit = ADCU1>,
    {
        let config = AdcChannelConfig {
            attenuation: DB_12,                      // Accepts voltages from 0V to 3.6V~
            resolution: Resolution::Resolution12Bit, // How many bits to represent the signal, 12-bits = 0-4095
            ..Default::default()
        };
        // TODO: Should i really be creating a new channel driver for every read ?? maybe i should
        // receive this as a parameter to the function
        let channel = AdcChannelDriver::new(adc, pin, &config)?;

        Ok(Self { channel })
    }

    pub fn read(&mut self) -> anyhow::Result<u16> {
        let value = self.channel.read()?;

        Ok(value)
    }
}
