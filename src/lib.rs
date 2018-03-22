//! A platform agnostic driver to interface with the PCD8544 LCD controller
//! found in commonly used displays like the Nokia 5110
//! 
//! This is work-in-progress!
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(unsize)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::spi::{Write};
use hal::spi::{Mode, Phase, Polarity};
use hal::digital::OutputPin;
use hal::blocking::delay::DelayUs;

/// SPI Mode
pub const MODE:Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};


/// PCD8544 driver struct.
pub struct Pcd8544<SPI, RES, CE, DC, DELAY> {
    spi: SPI,
    /// Reset (active low)
    res: RES,  
    /// Chip Enable (active low)
    ce: CE,
    /// Data/Command (1 = Data, 0 = Command)
    dc: DC,
    /// delay
    delay: DELAY,
}

impl <SPI, E, RES, CE, DC, DELAY> Pcd8544 <SPI, RES, CE, DC, DELAY> 
where
    SPI: Write<u8, Error = E>,
    RES: OutputPin,
    CE: OutputPin,
    DC: OutputPin,
    DELAY: DelayUs<u8>,
{
    /// Create a new driver from an SPI peripheral and
    /// digital output pins.

    pub fn new(spi: SPI, res: RES, ce: CE, dc: DC, delay: DELAY) -> Result<Self, E> {
        let mut pcd8544 = Pcd8544 { spi, res, ce, dc, delay };
        
        pcd8544.ce.set_high();
        pcd8544.res.set_low();
        pcd8544.delay.delay_us(1);
        pcd8544.res.set_high();

        let init_sequence = [0x21, 0x13, 0xc2, 0x20, 0x9, 0x8, 0xc, 0x80, 0x40];
        pcd8544.send_cmd(&init_sequence)?;

        // clear RAM
        let data = [0x0; 84*6];
        pcd8544.send_data(&data)?;

        // just for fun, draw a thickline
    
        let data = [0xff; 84];        
        pcd8544.send_data(&data)?;

        Ok(pcd8544)
    }

    /// Send a sequence of bytes. This will be interpreted
    /// as a command sequence.
    pub fn send_cmd(&mut self, bytes: &[u8]) -> Result<(), E> {
        // dc low for command
        self.dc.set_low();
        self.ce.set_low();
        let r = self.spi.write(bytes)?;
        self.ce.set_high();
     
        Ok(r)   
    } 

    /// Send a sequence of bytes. This will be interpreted
    /// as a data sequence.
    pub fn send_data(&mut self, bytes: &[u8]) -> Result<(), E> {
        // dc high for data
        self.dc.set_high();
        self.ce.set_low();
        let r = self.spi.write(bytes)?;
        self.ce.set_high();
        Ok(r)
    }

    

}


        
