extern crate chrono;
use chrono::{NaiveDateTime, TimeZone, Utc};
use wasi::fd_write;
use wasi::Ciovec;

#[no_mangle]
pub extern "C" fn parse_unix_timestamp(unix_timestamp: i64) {
    let datetime = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    let utc_datetime = Utc.from_utc_datetime(&datetime);
    let output = utc_datetime.to_string() + "\n";
    let data = [Ciovec {
        buf: output.as_ptr(),
        buf_len: output.len(),
    }];
    unsafe {
        fd_write(1, &data).unwrap();
    }
}

fn main() {
    let (argc, arg_buf_size) = unsafe { wasi::args_sizes_get().unwrap() };
    let mut args = Vec::with_capacity(argc);
    args.resize(argc, std::ptr::null_mut());

    let mut arg_buf = Vec::with_capacity(arg_buf_size);
    unsafe { wasi::args_get(args.as_mut_ptr(), arg_buf.as_mut_ptr()).unwrap() };
    let args: Vec<String> = args
        .iter()
        .map(|&arg_ptr| {
            let arg_len = unsafe { libc::strlen(arg_ptr as *const i8) };
            let arg_slice = unsafe { std::slice::from_raw_parts(arg_ptr, arg_len) };
            String::from_utf8_lossy(arg_slice).into_owned()
        })
        .collect();

    if args.len() > 1 {
        if let Ok(unix_timestamp) = args[1].parse::<i64>() {
            parse_unix_timestamp(unix_timestamp);
        }
    }
}
