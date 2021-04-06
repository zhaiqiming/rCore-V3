const FD_STDOUT: usize = 1;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            if ((buf as usize) < APP_BASE_ADDRESS) || ((buf as usize) > APP_SIZE_LIMIT + APP_BASE_ADDRESS) {
                -1 as isize
            }else{
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            }
        },
        _ => {
            -1 as isize
            //panic!("Unsupported fd in sys_write!");
        }
    }
}