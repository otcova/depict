#[macro_export]
macro_rules! setup {
    () => {
        setup!({
            $crate::Depict::new()?.draw_loop(draw)?;
        });
    };
    ($code: expr) => {
        use $crate::wasm_bindgen::prelude::*;

        #[wasm_bindgen(start)]
        pub fn start() {
            if let Err(error) = setup() {
                log!("[ERROR]\n{}", error);
            }
        }

        fn setup() -> Result<(), String> {
            $code
            Ok(())
        }
    };
}
