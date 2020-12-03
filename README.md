# KeySeeBee

![KeySeeBee](images/keyseebee.jpg)

KeySeeBee is a split ergo keyboard. It is only 2 PCB (so the name)
with (almost) only SMD components on it. It's only a keyboard, no LED,
no display, nothing more than keys and USB.

The firmware is [Keyberon](https://github.com/TeXitoi/keyberon), a
pure rust firmware.

## Features

 * 44 keys, using Cherry MX or Kailh choc switches, only 1U keycaps;
 * USB-C connector on the 2 sides;
 * TRRS cable for connecting the 2 halves (for power and UART communication between the 2 halves);
 * 2 STM32F072 MCU, with hardware USB DFU bootloader and crystal less USB;
 * Only onboard SMD component (except for the switches and TRRS connector).

## Inspiration

 * [Plaid](https://github.com/hsgw/plaid) for "show the components"
 * [GergoPlex](https://www.gboards.ca/product/gergoplex) for "just a keyboard" and "only a PCB with SMD components"
 * [Lily58](https://github.com/kata0510/Lily58) for the thumb cluster
 * [Kyria](https://blog.splitkb.com/blog/introducing-the-kyria) for
   "don't be affraid of pinky stagger"

## Gallery

### v0.1, build by TeXitoi

![From above with one side upside down](images/above-with-back.jpg)

![Side view](images/side-view.jpg)

### v0.1, build by TeXitoi, Gateron silent clear (MX footprint), 3D printed plate

![From above](images/mx-and-plate.jpg)

### v0.2, build by eropleco, with 1.2mm anodized aluminium plate

![Left](images/eropleco-left.jpg)

![Right](images/eropleco-right.jpg)

## Bill Of Materials

Price is for 5 keyboards including shipping.

|Item                                                                      |Package|Qty|Remarks                                |Price |
|--------------------------------------------------------------------------|-------|--:|---------------------------------------|-----:|
|[Left PCB](pcb/gerbers/)                                                  |       |  1|Ordered at [JLCPCB](https://jlcpcb.com)|      |
|[Right PCB](pcb/gerbers/)                                                 |       |  1|Ordered at [JLCPCB](https://jlcpcb.com)|33.14€|
|[USB-C connector](https://www.aliexpress.com/item/33056042016.html)       |16 pins|  2|                                       | 1.44€|
|[1N4148WS](https://www.aliexpress.com/item/32774043752.html)              |SOD-323| 44|Price is for 1000                      | 2.82€|
|[PJ320A TRRS connector](https://www.aliexpress.com/item/4000661212458.html)|      |  2|                                       | 1.01€|
|[STM32F072CBT6 MCU](https://www.aliexpress.com/item/32947645404.html)     |LQFP-48|  2|STM32F072C8T6 would also work          | 9.65€|
|[XC6206P332MR regulator](https://www.aliexpress.com/item/33015891307.html)|SOT-23 |  2|Price is for 50                        | 1.93€|
|[SMD switch](https://www.aliexpress.com/item/32914876022.html)            | 3×6mm |  4|Price is for 100                       | 1.35€|
|[5.1kΩ resistor](https://www.aliexpress.com/item/32865947306.html)        | 0805  |  6|Price is for 100                       |      |
|[1µF capacitor](https://www.aliexpress.com/item/32964553793.html)         | 0805  |  4|Price is for 100                       |      |
|[100nF capacitor](https://www.aliexpress.com/item/32964553793.html)       | 0805  | 10|Price is for 100                       | 3.46€|
|[Bumpers](https://www.aliexpress.com/item/32289191938.html)               | 5×2mm | 10|Price is for 100                       | 1.75€|
|[Cherry MX compatible](https://www.aliexpress.com/item/32797603005.html) or [Kailh Choc](https://www.aliexpress.com/item/4000907409650.html) switch|5 pins (PCB mount)|44| | |
|1U keycap compatible with the chosen switches                             |       | 44|                                       |      |
|[TRRS cable](https://www.aliexpress.com/item/32809573546.html)         |Jack 3.5mm|  1|4 contacts needed                      |      |
|[USB-C cable](https://www.aliexpress.com/item/32678087225.html)           |       |  1|USB-2 is enough                        |      |

About 60€ without switches, keycaps and cables for 5 keyboards
(12€/keyboard).

## Compiling and flashing

Install the complete toolchain and utils:

```shell
curl https://sh.rustup.rs -sSf | sh
rustup target add thumbv6m-none-eabi
rustup component add llvm-tools-preview
cargo install cargo-binutils
sudo apt-get install dfu-util
```

Compile:

```shell
cd firmware
cargo objcopy --bin keyseebee --release -- -O binary keyseebee.bin
```

To flash using dfu-util, first put the board in dfu mode by pressing
BOOT, pressing and releasing RESET and releasing BOOT. Then:

```shell
dfu-util -d 0483:df11 -a 0 -s 0x08000000:leave -D keyseebee.bin
```

The fist time, if the write fail, your flash might be protected. To
unprotect:

```shell
dfu-util -d 0483:df11 -a 0 -s 0x08000000:force:unprotect -D keyseebee.bin
```

Ideally both sides needs to be flashed, but for changes that only affect the layout it's strictly not needed as the side connected with the USB cable will determine the layout. In fact, you can have different layouts stored on each half, meaning you can switch back and forth between two layouts by moving the USB cable between the two halves.
