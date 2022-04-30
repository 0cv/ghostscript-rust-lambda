use ghostscript as gs;
use gs::callback::display;
use gs::callback::display::DisplayFormat;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;

pub struct RawImage {
    pub width: u32,
    pub height: u32,
    pub format: DisplayFormat,
    pub data: Vec<u8>,
}

impl ::std::fmt::Debug for RawImage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("RawImage")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("format", &self.format)
            .field("data(size)", &self.data.len())
            .finish()
    }
}

#[derive(Debug)]
pub struct PageGrabberDisplayCallback {
    width: usize,
    height: usize,
    raster: usize,
    format: DisplayFormat,
    pimage: *mut u8,
    pages: Vec<RawImage>,
    buf_reader: BufReader<File>,
}

impl PageGrabberDisplayCallback {
    pub fn new() -> Self {
        let f = Result::unwrap(File::open("./input.pdf"));
        let buf_reader = BufReader::new(f);

        PageGrabberDisplayCallback {
            width: 0,
            height: 0,
            raster: 0,
            format: Default::default(),
            pimage: ::std::ptr::null_mut(),
            pages: Vec::new(),
            buf_reader: buf_reader,
        }
    }

    pub fn into_pages(self) -> Vec<RawImage> {
        self.pages
    }
}

impl Default for PageGrabberDisplayCallback {
    fn default() -> Self {
        PageGrabberDisplayCallback::new()
    }
}

impl gs::callback::panic::PanicCallback for PageGrabberDisplayCallback {}

// impl gs::callback::display::DisplayCallback for PageGrabberDisplayCallback {
//     fn display_size(
//         &mut self,
//         _device: *mut gs::callback::display::DisplayRawDevice,
//         width: usize,
//         height: usize,
//         raster: usize,
//         format: DisplayFormat,
//         pimage: *mut u8,
//     ) -> gs::error::ErrCode {
//         self.width = width;
//         self.height = height;
//         self.raster = raster;
//         self.pimage = pimage;
//         self.format = format;
//         gs::GS_OK
//     }

//     fn display_page(&mut self, _device: *mut gs::callback::display::DisplayRawDevice, _copies: u32, _flush: bool) -> gs::error::ErrCode {
//         if self.format.contains(DisplayFormat::COLORS_SEPARATION) {
//             return gs::error::consts::RANGE_CHECK;
//         }

//         let bits = if let Some(bits) = display::bits_per_pixel(self.format) {
//             debug!("Page bits per pixel: {}", bits);
//             bits as usize
//         } else {
//             error!(
//                 "Unable to calculate bits per pixel for format: {:?}",
//                 self.format
//             );
//             return gs::error::consts::RANGE_CHECK;
//         };

//         let width_in_bytes = (self.width * bits + 7) / 8;
//         let size_in_bytes = self.height * width_in_bytes;
//         let mut buf = Vec::with_capacity(size_in_bytes);

//         if self.raster == width_in_bytes {
//             // Already packed. Just copy everything.
//             buf.extend_from_slice(unsafe { ::std::slice::from_raw_parts(self.pimage, size_in_bytes) });
//         } else if self.raster < width_in_bytes {
//             error!(
//                 "Bug! Invalid width_in_bytes calculation. Raster: {}, WIB: {}",
//                 self.raster, width_in_bytes
//             );
//             return gs::error::consts::RANGE_CHECK;
//         } else {
//             for row in 0..self.height {
//                 buf.extend_from_slice(unsafe {
//                     ::std::slice::from_raw_parts(
//                         self.pimage.offset((row * self.raster) as isize),
//                         width_in_bytes,
//                     )
//                 });
//             }
//         }

//         self.pages.push(RawImage {
//             width: self.width as _,
//             height: self.height as _,
//             format: self.format,
//             data: buf,
//         });

//         gs::GS_OK
//     }
// }

impl ghostscript::callback::stdio::StdioCallback for PageGrabberDisplayCallback {
    fn read_stdin(&mut self, _buf: &mut [u8]) -> Option<usize> {
        let n = Result::unwrap(self.buf_reader.read(&mut _buf[..]));
        Some(n)
    }

    fn write_stdout(&mut self, buf: &[u8]) -> usize {
        buf.len() // Silently discard everything.
    }

    fn write_stderr(&mut self, buf: &[u8]) -> usize {
        self.write_stdout(buf)
    }
}