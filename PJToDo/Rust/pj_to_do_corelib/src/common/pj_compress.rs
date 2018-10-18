extern crate flate2;

use std::io::prelude::*;
use flate2::Compression;
use flate2::write::{GzEncoder, GzDecoder};

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use common::pj_logger::*;

extern crate libc;
use self::libc::{c_char};
use std::ffi::CStr;

use pal::pj_to_do_pal::PJToDoPal;

// Compress file.
#[no_mangle]
pub unsafe extern "C" fn pj_compress(file_path: *const c_char) {
    let file_path = CStr::from_ptr(file_path).to_string_lossy().into_owned(); //unsafe
    let open_file_result = File::open(file_path);

    match open_file_result {
        Ok(mut f) => {
    
            let mut buffer = vec![0; 10];
            // read the whole file
            match f.read_to_end(&mut buffer) {
                Ok(_) => {
                    let mut e = GzEncoder::new(Vec::new(), Compression::default());
                    e.write_all(buffer.as_slice()).unwrap();
                    let bytes = e.finish().unwrap();

                    let gzip_file_result = File::create(PJToDoPal::db_gzip_path());
                    match gzip_file_result {
                        Ok(mut gzip_file) => {
                            gzip_file.write_all(bytes.as_slice());
                        }
                        Err(e) => {
                            error!("Compress error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Compress error: {}", e);
                }
            }

        }
        Err(e) => {
            error!("Compress error: {}", e);
        }
    }
    
}

// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read
fn decode_writer(bytes: Vec<u8>) -> io::Result<String> {
    let mut writer = Vec::new();
    let mut decoder = GzDecoder::new(writer);
    decoder.write_all(&bytes[..])?;
    decoder.try_finish()?;
    writer = decoder.finish()?;
    let return_string = String::from_utf8(writer).expect("String parsing error");
    Ok(return_string)
}
