# Square-1 Solver

This Square-1 Solver is written in Rust.
Its aim is to make table generation faster compared to the Solver written in Python.

Follow these steps to setup the robot:

Activate SPI in raspi-config for lights.

Install rust.

Clone this repository and switch to branch feature/robot.

Install arduino-cli to upload motor_controller.ino to microcontroller.
Uploading file from external PC is possible too, but requires plugging the microcontroller into the external PC which requires partial disassembly of the robot. Arduino-cli is the convenient option here.

Install needed libraries with:
```
sudo apt install libudev-dev libx11-dev libxtst-dev libinput-dev
```

Build project with:
```
cargo build --release
```
In the future I might release a built version which won't require building the robot bin from source.

Generate table with:
```
~/Square-1-Solver-Rust/target/release/generate all --limram
```
On the Rpi this takes about 2 hours. You could also generate the table on a faster PC and move the table to the rpi with scp.

While generating, adjust config_cam_*.txt and picconfig.toml with the help of:
```
~/Square-1-Solver-Rust/target/release/piccal
```

Add "sudo ~/Square-1-Solver-Rust/target/release/robot" to autostart of rpi.