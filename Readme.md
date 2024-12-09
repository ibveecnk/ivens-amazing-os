# RPI-kueos
## Manual compilation

### Linux
cargo rustc -- -C link-arg=-nostartfiles
### Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
### macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"

cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/target/debug/bootimage-kernel.bin