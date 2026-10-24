nusb
----

A new pure-Rust library for cross-platform low-level access to USB devices.

* [Documentation](https://docs.rs/nusb)
* [Changelog](https://github.com/kevinmehall/nusb/releases)
* [Issues](https://github.com/kevinmehall/nusb/issues)
* [Discussions](https://github.com/kevinmehall/nusb/discussions)

`nusb` supports Windows, macOS, Linux, and WebUSB and provides both async and
blocking APIs for listing and watching USB devices, reading descriptor
details, opening and managing devices and interfaces, and performing
transfers on control, bulk, and interrupt endpoints.

### Compared to [rusb](https://docs.rs/rusb/latest/rusb/) and [libusb](https://libusb.info/)

* Pure Rust, no dependency on libusb or any other C library.
* Async-first, while not requiring an async runtime.
* No context object. You just open a device. There is a global event loop thread
  that is started when opening the first device.
* Thinner layer over OS APIs, with less internal state.

### macOS device capture in this fork

`Device::capture()` explicitly detaches kernel drivers for the whole device
using `USBDeviceReEnumerate(kUSBReEnumerateCaptureDeviceMask)`, then reopens
the device and restores its configuration. This requires root privileges;
the fork does not request entitlement-based authorization.

Capture consumes an unshared device handle. Call it before cloning the handle,
claiming interfaces, or submitting control transfers. Release capture explicitly
with `Device::release_capture()`, or drop all device handles, interfaces, and
endpoints. Release re-enumerates the device so kernel drivers can attach again.
Any release failure during drop is logged. Normal interface claiming and
`detach_and_claim_interface()` retain their existing behavior.

The capture-only example sends no adapter protocol commands:

```sh
cargo build --example capture
sudo ./target/debug/examples/capture 0403 cc4d 1
```

The example captures an OpenPort, claims interface 1, and releases it.
Hardware verification is still needed. API behavior follows Apple's
[capture option](https://developer.apple.com/documentation/kernel/usbreenumerateoptions/kusbreenumeratecapturedevicemask)
and the capture/reopen lifecycle in
[libusb's Darwin backend](https://github.com/libusb/libusb/blob/master/libusb/os/darwin_usb.c).

### License

MIT or Apache 2.0, at your option
