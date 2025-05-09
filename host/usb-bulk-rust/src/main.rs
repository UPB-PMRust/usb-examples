use anyhow::Result;
use std::thread;
use std::time::Duration;

use nusb::{
    DeviceInfo, MaybeFuture,
    transfer::{Bulk, In, Out},
};

fn connection(di: DeviceInfo) -> Result<()> {
    println!("Device connected");

    let device = di.open().wait()?;
    let interface = device.claim_interface(0).wait()?;

    // Open endpoint OUT 1 (0x0_0000001)
    let mut ep_out = interface.endpoint::<Bulk, Out>(0x01)?;

    // Open endpoint IN 1 (0x1_0000001)
    let mut ep_in = interface.endpoint::<Bulk, In>(0x81)?;

    // define an input buffer
    let buffer = ep_in.allocate(4096);
    ep_in.submit(buffer);

    // This example writes data and waits to read data
    // Reading amn writing can be done independently, most probably in
    // separate threads.

    for i in 0..100000 {
        // send bytes from a String
        ep_out.submit(format!("text {i}").into_bytes().into());
        if let Some(result) = ep_out.wait_next_complete(Duration::from_millis(1000)) {
            result.status?;
        } else {
            eprintln!("Transfer OUT timeout");
        }

        // receive bytes to an str
        if let Some(result) = ep_in.wait_next_complete(Duration::from_millis(1000)) {
            println!("{:?}", std::str::from_utf8(&*result.buffer)?);
            // resubmit the buffer to have it available for the next read
            ep_in.submit(result.buffer);
        } else {
            eprintln!("Transfer IN timeout");
        }
    }
    Ok(())
}

fn main() {
    loop {
        // Search for the device with:
        //  VendorId:  0xc0de
        //  ProductId: 0xcafe
        if let Some(di) = nusb::list_devices()
            .wait()
            .unwrap()
            .find(|d| d.vendor_id() == 0xc0de && d.product_id() == 0xcafe)
        {
            if let Err(e) = connection(di) {
                eprintln!("Error {e}");
            }
        } else {
            eprintln!("Device not found, retrying for 1 s");
            thread::sleep(Duration::from_secs(1));
        }
    }
}
