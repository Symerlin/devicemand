// Copyright 2024 Symerlin Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::devices;

#[test]
fn TestScanUSBDevices() {
    for device in devices::ScanUsbDevice().unwrap() {
        println!(
            "{} {}:{} {}",
            device.DevicePath,
            device.GetVendorId().unwrap(),
            device.GetProductId().unwrap(),
            device.GetProductName().unwrap(),
        );
    }
}
