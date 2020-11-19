# Gerber change log

## v0.2 (2020-09-05)

Same as v0.1 with some bugs fixed.

Modifications:
 * Fix LDO.
 * Fix Diodes.
 * Remove references of the MCU and the USB connector on the left to
   be consistent with the right.

Tested by eropleco, fully functionnal.

Bugs:
 * The right thumb cluster is a bit offset comparing to the left one.

## v0.1 (2020-08-02)

First version. Ordered at JLCPCB. Tested and functional with the
workaround.

This version has the placeholder `JLCJLCJLCJLC` for use with the
option "Remove Order Number -> Specify a location" at JLCPCB.

Bugs:
 * LDO is badly wired. Workaround: solder it with a roation of 120°
   clockwise.
 * Diodes are in the wrong direction. Workaround: solder them in
   reverse (line on the diode up).
 * The right thumb cluster is a bit offset comparing to the left one.
