use nrf52833_hal as hal;

// Used to get memory address of Flash 'CONFIG' region
use core::{convert::TryInto, ptr::addr_of_mut};
// embedded_storage has required traits for flash read/write
use embedded_storage::nor_flash::NorFlash;
use embedded_storage::nor_flash::ReadNorFlash;
// HAL provides the methods for nRF flash NVMC operations
use hal::nvmc::Nvmc;
use hal::pac::NVMC;
use rtt_target::rprintln;

pub const NUM_PAGES: u32 = 4;
pub const PAGE_SIZE: u32 = 4 * 1024;
pub const LAST_PAGE: u32 = (NUM_PAGES - 1) * PAGE_SIZE;
extern "C" {
    #[link_name = "_config"]
    static mut CONFIG: [u8; (NUM_PAGES * PAGE_SIZE) as usize];
}

pub struct Flash {
    nvmc: Nvmc<NVMC>,
}

impl Flash {
    pub fn new() -> Self {
        let p = hal::pac::Peripherals::take().unwrap();
        Flash {
            nvmc: Nvmc::new(p.NVMC, unsafe { addr_of_mut!(CONFIG).as_mut().unwrap() }),
        }
    }

    pub fn write(&mut self, offset: u32, bytes: &[u8]) {
        // -> Result<(), <Nvmc<T> as ErrorType>::Error>  // TODO: handle error case!
        self.nvmc.write(offset, &bytes);
    }

    pub fn compare_read<const LENGTH: usize>(&mut self, offset: u32) {
        let actual = self.read::<LENGTH>(offset);
        let expected = unsafe { Flash::direct_read::<LENGTH>(offset) };
        if actual == expected {
            rprintln!("Read at {:#x}: {:02x?} as expected", offset, actual);
        } else {
            rprintln!(
                "Error: Read at {:#x}: {:02x?} instead of {:02x?}",
                offset,
                actual,
                expected,
            );
        }
    }

    pub fn read<const LENGTH: usize>(&mut self, offset: u32) -> [u8; LENGTH] {
        let mut buf = [0; LENGTH];
        self.nvmc.read(offset, &mut buf).unwrap();
        buf
    }

    pub unsafe fn direct_read<const LENGTH: usize>(offset: u32) -> [u8; LENGTH] {
        CONFIG[offset as usize..][..LENGTH].try_into().unwrap()
    }

    pub fn erase_if_needed(&mut self, offset: u32) {
        let mut page = [0; PAGE_SIZE as usize];
        self.nvmc.read(offset, &mut page).unwrap();
        if Flash::page_is_erased(&page) {
            return;
        }
        rprintln!("Erasing at {:#x}", offset);
        self.nvmc.erase(offset, offset + PAGE_SIZE).unwrap();
        self.nvmc.read(offset, &mut page).unwrap();
        if Flash::page_is_erased(&page) {
            rprintln!("The page was correctly erased.");
        } else {
            rprintln!("Error: The page was not correctly erased.");
        }
    }

    pub fn page_is_erased(page: &[u8; PAGE_SIZE as usize]) -> bool {
        page.iter().all(|&x| x == 0xff)
    }

}