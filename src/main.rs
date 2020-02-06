#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f0xx_hal as hal;

use cortex_m_rt::entry;
use ssd1306::{prelude::*, Builder as SSD1306Builder};
use ssd1306::prelude::DisplaySize::Display128x32 as DisplaySize;
use ssd1306::properties::DisplayProperties;
use core::fmt;
use core::fmt::Write;
use arrayvec::ArrayString;

use crate::hal::{
    prelude::*,
    stm32,
    i2c::I2c,
    delay::Delay
};

#[entry]
fn main() -> ! {

    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(),cortex_m::peripheral::Peripherals::take()) {
        
        cortex_m::interrupt::free(move |cs| {

        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);
        
        let gpioa = p.GPIOA.split(&mut rcc);
        let scl = gpioa.pa9.into_alternate_af4(cs);
        let sda = gpioa.pa10.into_alternate_af4(cs);
        let mut led = gpioa.pa1.into_push_pull_output(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), &mut rcc);
        
        // Get delay provider
        let mut delay = Delay::new(cp.SYST, &rcc);

        // Set up the display
        
        let mut disp: TerminalMode<_> = SSD1306Builder::new().with_size(DisplaySize).connect_i2c(i2c).into();
        
        disp.init().unwrap();

        disp.clear().unwrap();

        let mut time: u8 = 180;
        
        for a in 0..time+1 {

            let mut output = ArrayString::<[u8; 64]>::new();
            
            format(&mut output, time/60, time%60);
            
            disp.write_str(output.as_str());
            time = time - 1;
            delay.delay_ms(1000_u16);


        }

        led.toggle();

    });
    
}

    loop {continue;}
    
}


fn format(buf: &mut ArrayString<[u8; 64]>, minutes: u8, seconds: u8) {
    fmt::write(buf, format_args!("time left: {:02}:{:02}                                                ", minutes, seconds)).unwrap();
}