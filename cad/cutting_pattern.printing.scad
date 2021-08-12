$fs=0.5;
$fa=1;
in=25.4;
g=0.05*in;

linear_extrude(1.4) {
     difference() {
          union() {
               square(19.05, center=true);
               translate([19.05/2+3, 0]) square([10, 19.05], center=true);
          }

          // MX
          circle(d=0.157*in);
          translate([2*g, 4*g]) circle(d=0.059*in);
          translate([-3*g, 3*g]) circle(d=0.059*in);
          translate([-4*g, 0]) circle(d=0.067*in);
          translate([4*g, 0]) circle(d=0.067*in);

          // Choc
          circle(d=3.2);
          translate([0, -5.9]) circle(d=0.059*in);
          translate([-5, -3.8]) circle(d=0.059*in);
          translate([-5.22, 0]) circle(d=1.8);
          translate([5.22, 0]) circle(d=1.8);

          // PJ320A
          translate([19.05/2+3, 6]) {
               translate([0, -1.6]) circle(d=1.5);
               translate([0, -8.6]) circle(d=1.5);

               for (i=[-3.2, -6.2, -10.2]) translate([2.3, i]) square([1,2], center=true);
               translate([-2.3, -11.3]) square([1,2], center=true);
          }
     }
}
