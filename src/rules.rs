// Copyright 2024 Symerlin Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::{fs, io};
use std::os::unix::fs as unix_fs;
use std::path::Path;

use crate::devices::UsbDevice;

pub struct UsbRule {
    pub VendorId: Option<String>,
    pub ProductId: Option<String>,

    pub Owner: u32,
    pub Group: u32,
    pub Mode: u32,
}

impl UsbRule {
    pub fn Match(&self, device: &UsbDevice) -> io::Result<bool> {
        match &self.VendorId {
            Some(v) => {
                if device.GetVendorId()?.as_str() != v.as_str() {
                    return Ok(false);
                }
            }
            None => {}
        }

        match &self.ProductId {
            Some(v) => {
                if device.GetProductId()?.as_str() != v.as_str() {
                    return Ok(false);
                }
            }
            None => {}
        }

        Ok(true)
    }

    pub fn Apply(&self, device: &UsbDevice) -> io::Result<()> {
        let path = Path::new(&device.DevicePath);

        fs::set_permissions(path, unix_fs::PermissionsExt::from_mode(self.Mode))?;
        unix_fs::chown(path, Some(self.Owner), Some(self.Group))?;

        Ok(())
    }
}
