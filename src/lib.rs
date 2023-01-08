#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("./bindings.rs");

pub fn vimBufferGetAllText(buff: *mut file_buffer) -> String
{
    let mut text = String::new();
    unsafe {
        let line_count = vimBufferGetLineCount(buff);
        for line in 1 ..= line_count {
            let line_txt = vimBufferGetLine(buff, line as i64);
            let len = libc::strlen(line_txt as *mut i8);
            let line_string = String::from_raw_parts(line_txt, len, len);
            text.push_str(&line_string);
        }
    }
    text
}

pub fn vimPrintAllText(buff: *mut file_buffer)
{
    unsafe {
        let line_count = vimBufferGetLineCount(buff);
        for line in 1 ..= line_count {
            let line_txt = vimBufferGetLine(buff, line as i64) as *const i8;
            libc::puts(line_txt);
        }
    }
}
