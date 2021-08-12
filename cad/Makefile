all: fat_mx_plate.stl plate.svg cutting_pattern.stl

%.stl: %.printing.scad functions.scad
	openscad -o $@ $<

%.svg: %.2d.scad functions.scad
	openscad -o $@ $<
