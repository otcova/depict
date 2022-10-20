use depict::*;

start_model!(draw);

struct Model {
    cute_cat: Image,
}

impl Model {
    fn new(app: &Depict) -> Model {
        Self { cute_cat: app.load_image("./some_image.png") }
    }

    fn draw(&self, app: &Depict, draw: &mut Draw) {
        draw.image(self.cute_cat).scale(app.seconds().sin());
    }
}
