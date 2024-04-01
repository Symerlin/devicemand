# devicemand

Rule-based device management daemon for device events.

It listens uevent to detect device changes.

## Rule

Rules are stored as key/value pairs split by lines. Other whitespaces are denied.

```
Key=Value
```

### USB Rule

/etc/deviceman/usb.conf.d/

Example:

```
vendor_id=1abc
product_id=2def
uid=1000
gid=1000
mode=0666
```

## systemd

This repo provides a systemd service unit file. Write it into ```/lib/systemd/system/devicemand.service``` and start it.

```bash
systemctl start devicemand
```

The output will be logged by systemd.

```bash
systemctl status devicemand
```
