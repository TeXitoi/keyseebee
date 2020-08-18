# Gerber change log

## v0.2 (Unreleased)

Modifications:
 * Fix LDO.
 * Fix Diodes.
 * Remove references of the MCU and the USB connector on the left to
   be consistent with the right.

Not ordered nor tested.

## v0.1 (2020-08-02)

First version. Ordered at JLCPCB, only left tested yet.

This version has the placeholder `JLCJLCJLCJLC` for use with the
option "Remove Order Number -> Specify a location" at JLCPCB.

Bugs:
 * LDO is badly wired. Workaround: solder it with a roation of 120°
   clockwise.
 * Diodes are in the wrong direction. Workaround: solder them in
   reverse (line on the diode up).
