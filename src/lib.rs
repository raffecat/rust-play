use std::str;

#[unsafe(no_mangle)]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn process_buffer(data: *const u8, len: usize) {
    // Safety: Turn the raw pointer into a Rust slice.
    // Borrowed from Go: Rust must not store the pointer after the function returns.
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    println!("Rust received {} bytes: {:?}", len, slice);
}

#[unsafe(no_mangle)]
pub extern "C" fn print_go_string(ptr: *const u8, len: usize) {
    // Safety: Turn the raw pointer into a Rust slice.
    // Borrowed from Go: Rust must not store the pointer after the function returns.
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    if let Ok(s) = str::from_utf8(slice) {
        println!("Rust says: {}", s);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_rust_buffer(out_len: *mut usize) -> *mut u8 {
    let mut data = vec![72, 101, 108, 108, 111]; // "Hello"
    let ptr = data.as_mut_ptr();
    let len = data.len();

    // Tell Rust to forget about this memory so it isn't freed at the end of this function
    std::mem::forget(data);

    // Give the length back to Go via the pointer
    unsafe { *out_len = len; }
    
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn free_rust_buffer(ptr: *mut u8, len: usize) {
    // Reconstruct the Vec and let it go out of scope to trigger the Drop
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, len, len);
        }
    }
}
