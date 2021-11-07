use wasm_bindgen::prelude::*;

use super::macro_helpers::define_react_use_ref_helpers;

define_react_use_ref_helpers! {
    {
        value: usize,
        object: MutableRefObjectUsize,
        use_ref_usize,
        use_ref_usize_with,
    },
    {
        value: bool,
        object: MutableRefObjectBool,
        use_ref_bool,
        use_ref_bool_with,
    },
}
