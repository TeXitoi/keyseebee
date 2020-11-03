switch_hole=14.0;// by spec should be 14, can be adjusted for printer imprecision
inter_switch=19.05;
thickness=1.6;// plate thinkness
d=2.54;
delta=[-d,0,d,0,-4*d,-5*d];// column stagger

module key_placement_without_extreme() {
     for (i=[0:5]) for (j=[0:2]) translate([(i-1)*inter_switch, -(j-1)*inter_switch+delta[i]]) children();
     for (i=[0:2]) translate([(i-0.5)*inter_switch, -2*inter_switch+min(delta[i], delta[i+1])]) children();
}

module extreme_key_placement() {
     translate([-31.496, -45.720]) rotate(26.5) children();
}

module key_placement() {
     key_placement_without_extreme() children();
     extreme_key_placement() children();
}

module outline() {
     union() {
          hull() key_placement_without_extreme() square(inter_switch, center=true);
          translate([-inter_switch, -2*inter_switch+delta[0]]) square(inter_switch, center=true);
          hull() {
               extreme_key_placement() square(inter_switch, center=true);
               translate([-0.5*inter_switch, -2*inter_switch+delta[0]]) square(inter_switch, center=true);
          }
     }
}

module plate() {
     difference() {
          outline();
          key_placement() square([switch_hole, switch_hole], center=true);
     }
}

//linear_extrude(thickness)
plate();
