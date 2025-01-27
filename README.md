This is a cli interface for the Corsair Void device ABI introduced on linux 6.13.

---

## Installation and Usage

Assuming you have rust toolchain installed and cargo in PATH

`cargo install corsair-void-cli`

`corsair-void-cli <subcommand>`

---

## Subcommands

- `info` Shows the device id, firmware versions, max sidetone value and microphone position. Use -j to print in json format.
- `send-alert <alert>` Plays a built-in notification from the headset where `<alert>` is 0 or 1. Requires write privileges on `/sys/bus/hid/drivers/hid-corsair-void/<dev_id>/send_alert` file (Run with sudo).
- `set-sidetone <sidetone>` Sets the sidetone value, where 0 < `<sidetone>` < max_sidetone. Requires write privileges on `/sys/bus/hid/drivers/hid-corsair-void/<dev_id>/set_sidetone` file (Run with sudo).
- `battery` Prints out battery information. Use -j to print in json format.

---

### Used resources

[kernel.org documentation](https://docs.kernel.org/admin-guide/abi-testing.html#abi-sys-bus-hid-drivers-hid-corsair-void-dev-fw-version-headset)
[driver commit](https://git.kernel.org/pub/scm/linux/kernel/git/hid/hid.git/commit/?h=for-next&id=6ea2a6fd3872e60a4d500b548ad65ed94e459ddd)
