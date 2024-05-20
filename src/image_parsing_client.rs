use std::ffi::{c_char, CString};
use libloading::{Library, Symbol};

#[repr(C)]
pub struct Region {
    pub color: [u8; 4],
    pub bounds: (u32, u32, u32, u32),
}

pub unsafe fn get_regions(path: &str) -> & [Region] {

    let lib = Library::new("mylib.dll").expect("Failed to load library");
    unsafe {
        let parse_image: Symbol<unsafe extern "C" fn(*const c_char, *mut usize) -> *const Region> =
            lib.get(b"parse_image").expect("Failed to load symbol");

        let path_as_c_string = CString::new(path).expect("Failed to create CString");
        let mut out_count: usize = 0;
        let raw_regions = parse_image(path_as_c_string.as_ptr(), &mut out_count);
        let regions = unsafe { std::slice::from_raw_parts(raw_regions, out_count) };
        return regions
    }
}
