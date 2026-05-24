#![no_std]
#![no_main]

use core::panic::PanicInfo;

use framebuffer_draw::{Color, Framebuffer};
use roxy_loader_api::{bootinfo::BootInfo, kernel_entry};

kernel_entry!(kernel_main);

fn kernel_main(bootinfo: &BootInfo) -> ! {
    let framebuffer: Framebuffer = bootinfo.framebuffer.into();

    let size = core::cmp::min(framebuffer.width, framebuffer.height);

    for i in 0..size {
        framebuffer.draw_pixel(i, i, Color::rgb(255, 255, 255));
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
