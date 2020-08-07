# KeySeeBee

KeySeeBee is a split ergo keyboard. It is only 2 PCB (so the name) with (almost) only SMD components on it. It's only a keyboard, no LED, no display, nothing more than keys and USB.

The firmware is [Keyberon](https://github.com/TeXitoi/keyberon), a pure rust firmware.

Features:
 * 44 keys, using Cherry MX or Kailh choc switches, only 1U keycaps;
 * USB-C connector on the 2 sides
 * TRRS cable for connecting the 2 halves (for power and UART communication between the 2 halves);
 * 2 STM32F072 MCU;
 * Only onboard SMD component (except for the switches and TRRS connector).

Inspiration:
 * [Plaid](https://github.com/hsgw/plaid) for "show the components"
 * [GergoPlex](https://www.gboards.ca/product/gergoplex) for "just a keyboard" and "only a PCB with SMD components"
 * [Lily58](https://github.com/kata0510/Lily58) for the thumb cluster
 * [Kyria](https://blog.splitkb.com/blog/introducing-the-kyria) for "don't be affraid of pinky stagger"
