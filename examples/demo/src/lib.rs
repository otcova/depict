use depict::*;

setup!();

fn draw(app: &Depict, draw: &mut Draw) {
    draw.rect()
        .rotate_deg(app.seconds() * 30.)
        .xy([0.5, 0.0])
        .rotate_deg(app.seconds() * 30.)
        .rgb([app.seconds().sin(), 0.8, 1.]);
}
