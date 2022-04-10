use self::color::ColorPalette;
use crate::vga::writer::Writer;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

mod buffer;
mod character;
mod color;
mod writer;

lazy_static! {
    static ref VGA_DISPLAY: Mutex<Writer> = Mutex::new(Writer::default());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)))
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    VGA_DISPLAY.lock().write_fmt(args).unwrap()
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    use fmt::Write;
    let mut writer = VGA_DISPLAY.lock();
    writer.set_color_palette(ColorPalette::error());
    writer.write_fmt(args).unwrap();
    writer.reset_color();
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        $crate::vga::_eprint(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ({
        $crate::eprint!("{}\n", format_args!($($arg)*));
    })
}
