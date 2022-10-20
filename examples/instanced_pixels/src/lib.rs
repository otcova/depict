use depict::*;

start_loop!(draw);

fn draw(app: &Depict, draw: &mut Draw) {
    log!("dt: {}ms", app.average_delta_seconds() * 1000.);

    const N: f32 = 500.;
    for x in (-N as i32)..=(N as i32) {
        for y in (-N as i32)..=(N as i32) {
            draw.rect()
                .xy([x as f32 / N, y as f32 / N])
                .scale(1. / N)
                .rgb([
                    1. * (app.seconds() * 10. + x as f32 / 10.).sin(),
                    0.2 * (0.5 + app.seconds() * 10. + y as f32 / 10.).sin(),
                    0.,
                ]);
        }
    }
}
