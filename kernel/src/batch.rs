use core::arch::asm;
use lazy_static::lazy_static;
use log::{info, trace};
use crate::{config::{APP_BASE_ADDR, APP_SIZE_LIMIT, KERNEL_STACK_SIZE, USER_STACK_SIZE}, sync::UPSafeCell};

// Before implementing file system, we use include_bytes! to load the binary of the app
const APP_NUM: usize = 1;
const TEST_PRINT: &[u8] = include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/test_print.bin");

pub fn print_app_info() {
    info!("Total app count: {}", APP_NUM);
    APP_MANAGER.exclusive_access().apps.iter().enumerate().for_each(|(i, app)| {
        trace!("App[{}]: {}, start: {:p}, len: 0x{:x}", i, app.name, app.as_ptr(), app.len());
    });
}

lazy_static! {
    pub static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        let app_arr = [
            App::new("test_print", TEST_PRINT),
        ];
        UPSafeCell::new(AppManager { apps: app_arr })
    };
}

static KERNEL_STACK: KernelStack = KernelStack([0; KERNEL_STACK_SIZE]);
static USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);

// region KernelStack begin
#[repr(align(4096))]
struct KernelStack([u8; KERNEL_STACK_SIZE]);

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.0.as_ptr() as usize + KERNEL_STACK_SIZE
    }
}
// region KernelStack end

// region UserStack begin
#[repr(align(4096))]
struct UserStack([u8; USER_STACK_SIZE]);

impl UserStack {
    fn get_sp(&self) -> usize {
        self.0.as_ptr() as usize + USER_STACK_SIZE
    }
}
// region UserStack end

// region AppManager begin
pub struct AppManager {
    apps: [App; APP_NUM],
}

impl AppManager {
    pub unsafe fn load_app(&self, app_id: usize) {
        if app_id >= APP_NUM {
            info!("All apps completed");
            return;
        }

        info!("Loading app {}", app_id);
        let app = &self.apps[app_id];
        // clean instruction cache
        asm!("fence.i");
        // clear app space
        core::slice::from_raw_parts_mut(APP_BASE_ADDR as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = app.bin;
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDR as *mut u8, app.len());
        app_dst.copy_from_slice(app_src);
    }
}
// region AppManager end

// region App begin
struct App {
    name: &'static str,
    bin: &'static [u8],
}

impl App {
    pub fn new(name: &'static str, bin: &'static [u8]) -> Self {
        Self { name, bin }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.bin.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.bin.len()
    }
}
// region App end