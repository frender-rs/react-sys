use std::stringify;

macro_rules! define_react_use_ref_for_type {
    ( { value: $type_value:ty, $($t:tt)+ } ) => {
        define_react_use_ref_for_type! {{
            input: $type_value,
            output: $type_value,
            $($t)+
        }}
    };
    ({
        input: $input_type:ty,
        output: $output_type:ty,
        object: $name_object:ident,
        $name_use:ident,
        $name_use_with:ident
        $(,)?
    }) => {
        $crate::macro_import::wasm_bindgen_react! {
            /// MutableRefObject<
            #[doc = stringify!($output_type)]
            /// >
            #[derive(Debug, Clone)]
            pub type $name_object;
            #[wasm_bindgen(structural, method, getter)]
            pub fn current(this: &$name_object) -> $output_type;
            #[wasm_bindgen(structural, method, setter)]
            pub fn set_current(this: &$name_object, val: $input_type);

            #[wasm_bindgen(js_namespace = React, js_name = useRef)]
            pub fn $name_use(initial_value: $input_type) -> $name_object;

            /// Closure `get_initial_value` will be called immediately,
            /// thus it is safe to use reference here.
            ///
            /// Closure `get_initial_value` will be called only once or never called
            #[wasm_bindgen(js_namespace = React, js_name = useRef)]
            pub fn $name_use_with(get_initial_value: &mut dyn FnMut() -> $input_type) -> $name_object;
        }
    };
}

macro_rules! define_react_use_ref_helpers {
    ($($t:tt),+ $(,)?) => {
        use $crate::use_ref::macro_helpers::define_react_use_ref_for_type;
        $( define_react_use_ref_for_type!{ $t } )+
    };
}

pub(super) use define_react_use_ref_for_type;
pub(super) use define_react_use_ref_helpers;
