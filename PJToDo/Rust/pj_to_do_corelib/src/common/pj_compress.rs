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
                    let compress_result = compress_write_all(buffer.as_slice());
                    // let bytes = e.finish().unwrap();

                    match compress_result {
                        Ok(bytes) => {
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
        Err(e) => {
            error!("Compress error: {}", e);
        }
    }
}

fn compress_write_all(mut buf: &[u8]) -> io::Result<Vec<u8>> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(buf).unwrap();
    let bytes = e.finish();
    bytes
}

// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read
unsafe fn uncompresses_writer_all(uncompresses_file_path: *const c_char) -> io::Result<Vec<u8>> {
    let uncompresses_file_path = CStr::from_ptr(uncompresses_file_path)
        .to_string_lossy()
        .into_owned(); //unsafe
    let open_file_result = File::open(uncompresses_file_path);
    match open_file_result {
        Ok(mut f) => {
            let mut buffer: Vec<u8> = Vec::new();
            // read the whole file
            match f.read_to_end(&mut buffer) {
                Ok(_) => {
                    let mut writer: Vec<u8> = Vec::new();
                    let mut decoder = GzDecoder::new(writer);
                    decoder.write_all(&buffer[..]).unwrap();

                    match decoder.try_finish() {
                        Ok(_) => {
                            let bytes = decoder.finish();
                            bytes
                        }
                        Err(e) => {
                            error!("UnCompress error: {}", e);
                            Err(e)
                        }
                    }
                }
                Err(e) => {
                    error!("UnCompress error: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            error!("UnCompress error: {}", e);
            Err(e)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pj_uncompress(uncompresses_file_path: *const c_char) {
    let decode_result = uncompresses_writer_all(uncompresses_file_path);
    match decode_result {
        Ok(bytes) => {
            let gzip_file_result = File::create(PJToDoPal::db_uncompress_path());
            match gzip_file_result {
                Ok(mut gzip_file) => {
                    gzip_file.write(&bytes);
                }
                Err(e) => {
                    error!("UnCompress error: {}", e);
                }
            }
        }
        Err(e) => {
            error!("UnCompress error: {}", e);
        }
    }
}

// impl<'a> std::io::Write for &'a mut [u8] {}

// impl<'a> Write for &'a mut [u8] {
//     #[inline]
//     fn write(&mut self, data: &[u8]) -> io::Result<usize> {
//         let amt = cmp::min(data.len(), self.len());
//         let (a, b) = mem::replace(self, &mut []).split_at_mut(amt);
//         a.copy_from_slice(&data[..amt]);
//         *self = b;
//         Ok(amt)
//     }

//     #[inline]
//     fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
//         if self.write(data)? == data.len() {
//             Ok(())
//         } else {
//             Err(Error::new(
//                 ErrorKind::WriteZero,
//                 "failed to write whole buffer",
//             ))
//         }
//     }

//     #[inline]
//     fn flush(&mut self) -> io::Result<()> {
//         Ok(())
//     }
// }

// impl<'a> Read for &'a mut [u8] {
//     #[inline]
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         let amt = cmp::min(buf.len(), self.len());
//         let (a, b) = self.split_at(amt);

//         // First check if the amount of bytes we want to read is small:
//         // `copy_from_slice` will generally expand to a call to `memcpy`, and
//         // for a single byte the overhead is significant.
//         if amt == 1 {
//             buf[0] = a[0];
//         } else {
//             buf[..amt].copy_from_slice(a);
//         }

//         *self = b;
//         Ok(amt)
//     }

//     #[inline]
//     unsafe fn initializer(&self) -> Initializer {
//         Initializer::nop()
//     }

//     #[inline]
//     fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
//         if buf.len() > self.len() {
//             return Err(Error::new(
//                 ErrorKind::UnexpectedEof,
//                 "failed to fill whole buffer",
//             ));
//         }
//         let (a, b) = self.split_at(buf.len());

//         // First check if the amount of bytes we want to read is small:
//         // `copy_from_slice` will generally expand to a call to `memcpy`, and
//         // for a single byte the overhead is significant.
//         if buf.len() == 1 {
//             buf[0] = a[0];
//         } else {
//             buf.copy_from_slice(a);
//         }

//         *self = b;
//         Ok(())
//     }

//     #[inline]
//     fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
//         buf.extend_from_slice(*self);
//         let len = self.len();
//         *self = &self[len..];
//         Ok(len)
//     }
// }
