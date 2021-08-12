use <functions.scad>

difference() {
     linear_extrude(5.4) plate();
     translate([0,0,1.6]) linear_extrude(5.4) key_placement() square(15, center=true);
}
