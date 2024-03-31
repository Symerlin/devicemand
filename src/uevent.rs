// Copyright 2024 Symerlin Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::{fs, io, mem, process};
use std::os::raw::c_void;
use std::time::{SystemTime, UNIX_EPOCH};

use err_rs::{err, wrap_result};
use libc::{AF_NETLINK, MSG_WAITFORONE, NETLINK_KOBJECT_UEVENT, SOCK_CLOEXEC, SOCK_DGRAM};

use crate::{Error, must_get};
use crate::devices::ScanUsbDevice;
use crate::errors::MissingField;
use crate::keyvalue::ReadKeyValueMap;
use crate::rules::UsbRule;

macro_rules! log_println {
    ($str:expr, $($arg:expr), *) => { println!(concat!("{}: ", $str), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(), $($arg,)*) };
}

pub fn Serve() -> Result<(), Error> {
    let fd = unsafe { libc::socket(AF_NETLINK, SOCK_DGRAM | SOCK_CLOEXEC, NETLINK_KOBJECT_UEVENT) };
    if fd < 0 {
        err!(Error::IoError, io::Error::last_os_error());
    }

    let mut addr: libc::sockaddr_nl = unsafe { mem::zeroed() };
    addr.nl_family = AF_NETLINK as libc::sa_family_t;
    addr.nl_pid = process::id();
    addr.nl_groups = 1;

    let res = unsafe { libc::bind(fd, &addr as *const libc::sockaddr_nl as *const libc::sockaddr, mem::size_of::<libc::sockaddr_nl>() as libc::socklen_t) };
    if res != 0 {
        err!(Error::IoError, io::Error::last_os_error());
    }

    let mut buf = vec![0; 8192];

    loop {
        if unsafe { libc::recv(fd, buf.as_mut_ptr() as *mut c_void, 8192, MSG_WAITFORONE) } < 0 {
            continue;
        }

        fn act() -> Result<(), Error> {
            let devices = ScanUsbDevice()?;

            for rule in ReadUsbRules()? {
                for device in &devices {
                    if wrap_result!(Error::IoError, rule.Match(&device)) {
                        log_println!("Apply rule to {}", device.DevicePath);
                        wrap_result!(Error::IoError, rule.Apply(&device));
                    }
                }
            }
            Ok(())
        }

        match act() {
            Ok(_) => {}
            Err(e) => {
                let now = wrap_result!(
                    Error::SystemTimeError,
                    SystemTime::now().duration_since(UNIX_EPOCH)
                );
                println!("{}: {:?}", now.as_secs(), e);
            }
        }
    }
}

pub fn ReadUsbRules() -> Result<Vec<UsbRule>, Error> {
    let mut rules = vec![];
    for path in wrap_result!(Error::IoError, fs::read_dir("/etc/deviceman/usb.conf.d/")) {
        let map = wrap_result!(Error::IoError, ReadKeyValueMap(&format!("/etc/deviceman/usb.conf.d/{}", path.unwrap().file_name().into_string().unwrap())));

        rules.push(UsbRule {
            VendorId: map.get("vendor_id").cloned(),
            ProductId: map.get("product_id").cloned(),
            Owner: wrap_result!(Error::ParseIntError, must_get!(map, "uid").parse()),
            Group: wrap_result!(Error::ParseIntError, must_get!(map, "gid").parse()),
            Mode: wrap_result!(Error::ParseIntError, must_get!(map, "mode").parse()),
        });
    }

    Ok(rules)
}
