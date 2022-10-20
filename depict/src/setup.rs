// #[macro_export]
// macro_rules! setup {
//     ($Model: ty) => {
//         setup!({
//             let app = Depict::new()?;
//             let model = Model::new(&app);
//             app.draw_loop(move |app, draw| model.draw(app, draw))?;
//         });
//     };
//     ($draw: ident) => {
//         setup!({
//             $crate::Depict::new()?.draw_loop($draw)?;
//         });
//     };
//     ($code: expr) => {
//         use $crate::wasm_bindgen::prelude::*;

//         #[wasm_bindgen(start)]
//         pub fn start() {
//             if let Err(error) = setup() {
//                 log!("[ERROR]\n{}", error);
//             }
//         }

//         fn setup() -> Result<(), String> {
//             $code
//             Ok(())
//         }
//     };
// }

#[macro_export]
macro_rules! start_loop {
    ($draw:ident) => {
        use $crate::wasm_bindgen::prelude::*;

        #[wasm_bindgen(start)]
        pub fn start() {
            if let Err(error) = setup() {
                log!("[ERROR]\n{}", error);
            }

            fn setup() -> std::result::Result<(), String> {
                Depict::new()?.draw_loop($draw)
            }
        }
    };
}

#[macro_export]
macro_rules! start_model {
    ($model:ty) => {
        f
    };
}
