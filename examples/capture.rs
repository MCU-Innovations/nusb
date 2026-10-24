//! Capture a macOS USB device, claim an interface, then release capture.
//! Usage: sudo ./target/debug/examples/capture 0403 cc4d 1

#[cfg(target_os = "macos")]
use nusb::MaybeFuture;

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: capture <vendor-id-hex> <product-id-hex> <interface-number>".into());
    }
    let vendor = u16::from_str_radix(&args[1], 16)?;
    let product = u16::from_str_radix(&args[2], 16)?;
    let interface_number = args[3].parse()?;
    let info = nusb::list_devices()
        .wait()?
        .find(|d| d.vendor_id() == vendor && d.product_id() == product)
        .ok_or("device not found")?;
    let device = info.open().wait()?.capture().wait()?;
    let interface = device.claim_interface(interface_number).wait()?;
    println!("Captured device and claimed interface {interface_number}");
    interface.release().wait()?;
    device.release_capture().wait()?;
    println!("Released capture; kernel drivers may attach again");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("Device capture is only available on macOS");
    std::process::exit(1);
}
