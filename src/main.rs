use clap::Parser;
use std::ffi::CString;
use winapi::{
    shared::windef::{HBITMAP__, HDC},
    um::wingdi,
};

#[allow(clippy::fn_to_numeric_cast_with_truncation)]
fn screenshot() -> image::RgbaImage {
    unsafe {
        let display = CString::new("DISPLAY").unwrap();

        let h_screen_dc: HDC = wingdi::CreateDCA(
            display.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
        );
        let h_memory_dc: HDC = wingdi::CreateCompatibleDC(h_screen_dc);

        let width = wingdi::GetDeviceCaps(h_screen_dc, wingdi::HORZRES);
        let height = wingdi::GetDeviceCaps(h_screen_dc, wingdi::VERTRES);

        let mut h_bitmap = wingdi::CreateCompatibleBitmap(h_screen_dc, width, height);
        let h_old_bitmap = wingdi::SelectObject(h_memory_dc, h_bitmap as *mut std::ffi::c_void);

        wingdi::BitBlt(
            h_memory_dc,
            0,
            0,
            width,
            height,
            h_screen_dc,
            0,
            0,
            wingdi::SRCCOPY,
        );
        h_bitmap = wingdi::SelectObject(h_memory_dc, h_old_bitmap) as *mut HBITMAP__;

        let h_bitmap_info = wingdi::BITMAPINFOHEADER {
            biSize: std::mem::size_of::<wingdi::BITMAPINFOHEADER> as u32,
            biPlanes: 1,
            biBitCount: 32,
            biWidth: width,
            biHeight: -height,
            biCompression: wingdi::BI_RGB,
            biSizeImage: 0,

            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        };

        let mut screen_data = Vec::<u8>::new();
        screen_data.reserve((width * height * 4).try_into().unwrap());

        let ptr: *mut wingdi::BITMAPINFO = std::mem::transmute_copy(&h_bitmap_info);

        dbg!("a");
        wingdi::GetDIBits(
            h_memory_dc,
            h_bitmap,
            0,
            height as u32,
            screen_data.as_ptr() as *mut std::ffi::c_void,
            ptr,
            wingdi::DIB_RGB_COLORS,
        );
        dbg!("b");

        wingdi::DeleteDC(h_memory_dc);
        wingdi::DeleteDC(h_screen_dc);

        return image::RgbaImage::from_raw(width as u32, height as u32, screen_data).unwrap();
    }
}

/// Takes a screenshot
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to save the screenshot to
    #[arg(short, long, default_value_t = home::home_dir().unwrap().to_str().unwrap().to_string())]
    screenshot_dir: String,
}

fn main() {
    let args = Args::parse();
    let full_path = std::path::PathBuf::from(args.screenshot_dir)
        .join(chrono::offset::Local::now().to_string());
    println!("{}", full_path.to_str().unwrap());

    let image = screenshot();

    image.save(full_path).unwrap();
}
