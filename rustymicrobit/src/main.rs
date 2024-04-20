#![no_std]
#![no_main]

mod temperature;
mod timing;

extern crate alloc;

use cortex_m_rt::entry;
use embedded_alloc::Heap;
use panic_halt as _;
use nrf52833_hal as hal;
use rtt_target::{rtt_init_print, rprintln};
use temperature as temp;
use timing::*;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn init_heap() {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
}

#[entry]
fn main() -> ! {
    init_heap();

    rtt_init_print!();
    rprintln!("Hello my chickens! Lets get the temperature");

    let peripherals = hal::pac::Peripherals::take().unwrap();
    let mut temp = hal::Temp::new(peripherals.TEMP);
    let mut temperature_storage = temperature::TemperatureStore::new();
    loop {
        temperature_storage.add(temperature::Temperature::new(temp.measure().to_num(), 0));
        delay_ms(100);
        rprintln!("{:?} degrees C", temperature_storage.getLastAdded().unwrap_or(&temperature::Temperature{temperature_c: 0f32, timestamp_ms: 0}).temperature_c);
    };

}
