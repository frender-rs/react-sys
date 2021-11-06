use super::macro_helpers::*;
pub use std::{concat, stringify};

use wasm_bindgen::prelude::*;

define_react_use_state_helpers! {
    {
        value: bool,
        object: UseStateBoolObject,
        setter: UseStateBoolObjectSetter,
        use_state: use_state_bool,
        use_state_with: use_state_bool_with,
        auto_clean: use_state_bool_auto_clean,
        auto_clean_with: use_state_bool_auto_clean_with,
    },
    {
        value: usize,
        object: UseStateUsizeObject,
        setter: UseStateUsizeObjectSetter,
        use_state: use_state_usize,
        use_state_with: use_state_usize_with,
        auto_clean: use_state_usize_auto_clean,
        auto_clean_with: use_state_usize_auto_clean_with,
    },
    {
        value: i32,
        object: UseStateI32Object,
        setter: UseStateI32ObjectSetter,
        use_state: use_state_i32,
        use_state_with: use_state_i32_with,
        auto_clean: use_state_i32_auto_clean,
        auto_clean_with: use_state_i32_auto_clean_with,
    },
}
