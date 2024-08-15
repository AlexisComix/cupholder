//! Nothing malicious swear down

use windows::{core::PCSTR, Win32::{Foundation::HWND, Media::Multimedia::mciSendStringA}};
use viuer::{print_from_file, Config};

fn main() {
    welcome();

    let command = PCSTR::from_raw("set cdaudio door open".as_ptr());

    // oooh so scary oooooooooh
    unsafe { mciSendStringA(command, None, HWND::default()) };
}

fn welcome() {
    print!("\x1b[2J");

    let conf = Config {
        x: 0,
        y: 0,
        width: Some(40),
        height: Some(30),
        ..Default::default()
    };

    print_from_file("./assets/cupholder.jpg", &conf).expect("image0.jpg");

    // How courteous :)
    println!("Enjoy your wonderful beverage :)");
}
