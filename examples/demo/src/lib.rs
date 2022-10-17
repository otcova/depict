use depict::*;

setup!();

fn draw(app: &Depict, draw: &Draw) {
    draw.rect(0.6).translate([0.2, 0.4]).color([0., 0.8, 1.]);
}
