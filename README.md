This is a cli interface for the Corsair Void device ABI introduced on linux 6.13.

---

## Installation and Usage

Assuming you have rust toolchain installed and cargo in PATH

`cargo install corsair-void-cli`

`corsair-void-cli <subcommand>`

---

## Subcommands

- `info` Shows the device id, firmware versions, max sidetone value and microphone position.
- `send-alert <alert>` Plays a built-in notification from the headset where `<alert>` is 0 or 1
- `set-sidetone <sidetone>` Sets the sidetone value, where 0 < `<sidetone>` < max_sidetone.
- `battery` Prints out battery information (Not yet implemented).

---

### Used resources

[kernel.org documentation](https://docs.kernel.org/admin-guide/abi-testing.html#abi-sys-bus-hid-drivers-hid-corsair-void-dev-fw-version-headset)
[driver commit](https://git.kernel.org/pub/scm/linux/kernel/git/hid/hid.git/commit/?h=for-next&id=6ea2a6fd3872e60a4d500b548ad65ed94e459ddd)
