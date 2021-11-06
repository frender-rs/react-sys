#[cfg(feature = "import-react")]
macro_rules! wasm_bindgen_react {
    ($($b:item)+) => {
        #[wasm_bindgen(module = "/helpers/react-import.js")]
        extern "C" {
            $($b)+
        }
    };
}

#[cfg(not(feature = "import-react"))]
macro_rules! wasm_bindgen_react {
    ($($b:item)+) => {
        #[wasm_bindgen]
        extern "C" {
            $($b)+
        }
    };
}

pub(crate) use wasm_bindgen_react;
