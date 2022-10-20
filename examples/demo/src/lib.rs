use depict::*;

setup!();

fn draw(app: &Depict, draw: &mut Draw) {
    draw.rect()
        .translate([0.2, 0.4])
        .rgb([app.seconds().sin(), 0.8, 1.]);
}
