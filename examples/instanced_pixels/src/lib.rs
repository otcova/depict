use depict::*;

start_loop!(draw);

fn draw(app: &Depict, draw: &mut Draw) {
    log!("dt: {}ms", app.average_delta_seconds() * 1000.);

    let time = app.seconds() * 10.;

    const N: f32 = 500.;
    for x in (-N as i32)..=(N as i32) {
        for y in (-N as i32)..=(N as i32) {
            draw.rect()
                .xy([x as f32 / N, y as f32 / N])
                .scale(1. / N)
                .rgb([
                    1. * (0.5 + time + y as f32 / 10.).sin(),
                    0.5 * (time + x as f32 / 10.).sin(),
                    0.,
                ]);
        }
    }
}
