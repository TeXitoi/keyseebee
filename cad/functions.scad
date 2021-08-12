// You can change the parameters in the header in openscad, then
// render, and then export in the format you need.
//
// You can also include this file in your own scad file, and use the
// modules to create your own case:
//  - outline module (2D outline of the plate),
//  - plate module (2D draw of the plate, with switch holes),
//  - key_placement module (2D placement of the keys centered, to be
//    used with children) to create your own case.

switch_hole=13.8;// by spec should be 14, can be adjusted for printer imprecision

// Code here, not to be modified if you want keyseebee compatibility
inter_switch=19.05;
d=2.54;
delta=[-d,0,d,0,-4*d,-5*d];// column stagger

// this is a bug in v0.1 and v0.2, not needed after
right_thumb_offset=0.635;
function thumb_offset(is_right) = (is_right?1:0)*right_thumb_offset;

module key_placement_without_extreme(is_right=false) {
     for (i=[0:5]) for (j=[0:2]) translate([(i-1)*inter_switch, -(j-1)*inter_switch+delta[i]]) children();
     for (i=[0:2]) translate([(i-0.5)*inter_switch+thumb_offset(is_right), -2*inter_switch+min(delta[i], delta[i+1])]) children();
}

module extreme_key_placement(is_right=false) {
     translate([-32.131+thumb_offset(is_right), -45.720]) rotate(26.5) children();
}

module key_placement(is_right=false) {
     key_placement_without_extreme(is_right) children();
     extreme_key_placement(is_right) children();
}

module outline(is_right=false) {
     union() {
          hull() key_placement_without_extreme(is_right) square(inter_switch, center=true);
          translate([-inter_switch, -2*inter_switch+delta[0]]) square(inter_switch, center=true);
          hull() {
               extreme_key_placement(is_right) square(inter_switch, center=true);
               translate([-0.5*inter_switch+thumb_offset(is_right), -2*inter_switch+delta[0]]) square(inter_switch, center=true);
          }
     }
}

module plate(is_right=false) {
     difference() {
          outline(is_right);
          key_placement(is_right) square([switch_hole, switch_hole], center=true);
     }
}

