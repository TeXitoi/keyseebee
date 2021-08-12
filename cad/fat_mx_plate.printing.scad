use <functions.scad>

difference() {
     linear_extrude(5) plate();
     translate([0,0,1.6]) linear_extrude(5) key_placement() square(15, center=true);
}
