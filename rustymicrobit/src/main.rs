#![no_std]
#![no_main]

mod temperature;
mod timing;
mod flash;

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

fn initialize() {
    init_heap();
    rtt_init_print!();
}

fn flash_test(flash_memory: &mut flash::Flash) {
    flash_memory.erase_if_needed(flash::LAST_PAGE);

    let mut write_buf: [u8; 32] = [0; 32];
    for (i, x) in write_buf.iter_mut().enumerate() {
        *x = i as u8;
    }
    rprintln!("Writing at {:#x}: {:02x?}", flash::LAST_PAGE, write_buf);
    flash_memory.write(flash::LAST_PAGE, &write_buf);

    for i in 0..4 {
        flash_memory.compare_read::<11>(flash::LAST_PAGE + i);
        flash_memory.compare_read::<10>(flash::LAST_PAGE + i);
        flash_memory.compare_read::<9>(flash::LAST_PAGE + i + 4);
        flash_memory.compare_read::<8>(flash::LAST_PAGE + i + 4);
        flash_memory.compare_read::<7>(flash::LAST_PAGE + i + 8);
        flash_memory.compare_read::<6>(flash::LAST_PAGE + i + 8);
        flash_memory.compare_read::<5>(flash::LAST_PAGE + i + 16);
        flash_memory.compare_read::<4>(flash::LAST_PAGE + i + 16);
        flash_memory.compare_read::<3>(flash::LAST_PAGE + i + 20);
        flash_memory.compare_read::<2>(flash::LAST_PAGE + i + 20);
        flash_memory.compare_read::<1>(flash::LAST_PAGE + i + 24);
        rprintln!("loop complete");
    }
}

#[entry]
fn main() -> ! {
    initialize();

    let mut flash_memory = flash::Flash::new();
    flash_test(&mut flash_memory);

    rprintln!("Hello my chickens! Lets get the temperature");
    delay_ms(500);

    loop {
        delay_ms(300);
        rprintln!("infinitely looping");
    }

    let peripherals = hal::pac::Peripherals::take().unwrap();
    let mut temp = hal::Temp::new(peripherals.TEMP);
    let mut temperature_storage = temperature::TemperatureStore::new();

    let mut count = 0;
    loop {
        temperature_storage.add(temperature::Temperature::new(temp.measure().to_num(), 0));
        delay_ms(10);
        rprintln!("{:?} degrees C (sample number {})", temperature_storage.get_last_added().unwrap_or(&temperature::Temperature{temperature_c: 0f32, timestamp_ms: 0}).temperature_c, count);
        count += 1;
    };

}
