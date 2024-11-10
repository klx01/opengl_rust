use std::fs;
use std::os::raw::c_void;

#[derive(Copy, Clone)]
pub(crate) enum ImageFormat {
    RGB = gl::RGB as isize,
    RGBA = gl::RGBA as isize,
}

pub(crate) struct Texture {
    id: u32,
}
impl Texture {
    pub(crate) fn new_from_jpeg(file_name: &str) -> Option<Self> {
        let file_bytes = match fs::read(file_name) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("failed to read jpeg file {file_name}: {err}");
                return None;
            }
        };
        let mut decoder = zune_jpeg::JpegDecoder::new(&file_bytes);
        let decoded_bytes = match decoder.decode() {
            Ok(x) => x,
            Err(err) => {
                eprintln!("failed to decode jpeg file {file_name}: {err}");
                return None;
            },
        };
        let info = match decoder.info(){
            Some(x) => x,
            None => {
                eprintln!("failed to get info for jpeg file {file_name}");
                return None;
            },
        };
        Some(Self::new(&decoded_bytes, info.width.into(), info.height.into(), ImageFormat::RGB))
    }
    pub(crate) fn new_from_png(file_name: &str) -> Option<Self> {
        let file_bytes = match fs::read(file_name) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("failed to read png file {file_name}: {err}");
                return None;
            }
        };
        let wrap = std::io::Cursor::new(file_bytes);
        let (info, decoded_bytes) = match spng::decode(wrap, spng::Format::Rgb8) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("failed to decode png file {file_name}: {err}");
                return None;
            },
        };
        Some(Self::new(&decoded_bytes, info.width, info.height, ImageFormat::RGB))
    }
    pub(crate) fn new(image_bytes: &[u8], width: u32, height: u32, format: ImageFormat) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            /*
            // if we want to use GL_CLAMP_TO_BORDER instead of GL_REPEAT
            float borderColor[] = { 1.0f, 1.0f, 0.0f, 1.0f };
            glTexParameterfv(GL_TEXTURE_2D, GL_TEXTURE_BORDER_COLOR, borderColor);
             */
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            /*
            0 - mipmap level, needed if you want to generate mipmap manually
             */
            gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, width as i32, height as i32, 0, format as u32, gl::UNSIGNED_BYTE, image_bytes.as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self{id}
    }
    pub(crate) fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) }
    }
}