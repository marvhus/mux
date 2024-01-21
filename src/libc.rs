#[no_mangle]
pub unsafe extern "C" fn memset(
    dest: *mut u8,
    val: i32,
    n: usize,
) -> *mut u8 {
    let val = val as u8;
    for i in 0..n {
        *dest.add(i) = val;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(
    dest: *mut u8,
    src: *const u8,
    n: usize,
) -> *mut u8 {
    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }
    dest
}
