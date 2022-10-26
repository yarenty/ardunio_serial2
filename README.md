# Serial port collector

This is example of RIIR - Rewrite it in Rust ;-).
Initial project is here: [ardunio](https://github.com/yarenty/ardunio)

After another sytem upgrade - I decided to move to rust - also keep timing - how much it will take me .... looks like half working day - which is really amazing.


## pre

ubuntu:
```shell
sudo apt install libssl-dev libudev-dev
```

## Known issues

### UBUNTU 22.x+

On Ubuntu 22.04 it broke support for CH340 USB to serial adapter based devices. (support was native in 20.04)  

do: ` /$ ls /dev/tty*` there is no `ttyUSB0` entry in the list.

This happens because of conflict between product IDs (a Braille screen reader and my CH340 based chip)
Unless you are using a braille display this should do the trick:
```shell
sudo apt remove brltty
```


## CHANGELOG
v0.1.1:
- fixed light - it is reversed (max vol: 1024);
- small binary release (3X+); 

v0.1.0:
 - initial release;
 - port scanning ability;
 - restart ability (works on Mac);

## TODO
- restart should rescan all ports - ie. ubuntu assigns new ttyUSB{x} port
- add CLI support
- check/clean parsing errors
- clean libraries
- add TUI?