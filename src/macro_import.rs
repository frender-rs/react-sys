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

#[cfg(feature = "import-react")]
macro_rules! wasm_bindgen_helpers {
    ($($b:item)+) => {
        #[wasm_bindgen(module = "/helpers/helpers-import.js")]
        extern "C" {
            $($b)+
        }
    };
}

#[cfg(not(feature = "import-react"))]
macro_rules! wasm_bindgen_helpers {
    ($($b:item)+) => {
        #[wasm_bindgen(module = "/helpers/helpers.js")]
        extern "C" {
            $($b)+
        }
    };
}

pub(crate) use wasm_bindgen_helpers;
pub(crate) use wasm_bindgen_react;
