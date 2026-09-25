use crate::ffi_guard::safe_ffi_call;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, Ordering};

pub struct MountState {
    pub is_mounted: bool,
    pub mount_vnum: u32,
    pub stamina: u16,
    pub max_stamina: u16,
}

impl MountState {
    pub fn new() -> Self {
        Self {
            is_mounted: false,
            mount_vnum: 0,
            stamina: 1000,
            max_stamina: 1000,
        }
    }

    pub fn mount(&mut self, vnum: u32) {
        self.is_mounted = true;
        self.mount_vnum = vnum;
    }

    pub fn dismount(&mut self) {
        self.is_mounted = false;
        self.mount_vnum = 0;
    }

    pub fn set_stamina(&mut self, val: u16) {
        self.stamina = val.min(self.max_stamina);
    }
}

static IS_MOUNTED: AtomicBool = AtomicBool::new(false);
static MOUNT_VNUM: AtomicU32 = AtomicU32::new(0);
static MOUNT_STAMINA: AtomicU16 = AtomicU16::new(1000);

#[no_mangle]
pub extern "C" fn m2_mount_is_mounted() -> bool {
    safe_ffi_call(false, || IS_MOUNTED.load(Ordering::Relaxed))
}

#[no_mangle]
pub extern "C" fn m2_mount_get_vnum() -> u32 {
    safe_ffi_call(0, || MOUNT_VNUM.load(Ordering::Relaxed))
}

#[no_mangle]
pub extern "C" fn m2_mount_get_stamina() -> u16 {
    safe_ffi_call(0, || MOUNT_STAMINA.load(Ordering::Relaxed))
}

#[no_mangle]
pub extern "C" fn m2_mount_set(vnum: u32, mounted: bool) {
    safe_ffi_call((), || {
        MOUNT_VNUM.store(vnum, Ordering::Relaxed);
        IS_MOUNTED.store(mounted, Ordering::Relaxed);
    });
}
