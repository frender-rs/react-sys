#[cfg(feature = "import-react")]
macro_rules! wasm_bindgen_react_state_helpers {
    ($($b:item)+) => {
        #[wasm_bindgen(inline_js = r#"
        import * as React from "react";
        export { React };
        export function use_state_object(initial_value) {
            const [state, set_state] = React.useState(initial_value);
            return { value: state, setter: { set_state } };
        }
        export function use_state_auto_clean(initial_value, clean) {
            const obj = use_state_object(initial_value);
            const state = obj.value;
            React.useEffect(() => { clean(state) }, [state]);
        }
        "#)]
        extern "C" {
            $($b)+
        }
    };
}

#[cfg(not(feature = "import-react"))]
macro_rules! wasm_bindgen_react_state_helpers {
    ($($b:item)+) => {
        #[wasm_bindgen(inline_js = r#"
        export function use_state_object(initial_value) {
            const [state, set_state] = React.useState(initial_value);
            return { value: state, setter: { set_state } };
        }
        export function use_state_auto_clean(initial_value, clean) {
            const obj = use_state_object(initial_value);
            const state = obj.value;
            React.useEffect(() => { clean(state) }, [state]);
        }
        "#)]
        extern "C" {
            $($b)+
        }
    };
}

macro_rules! define_react_use_state_for_type {
    ( { value: $type_value:ty, $($t:tt)+ } ) => {
        define_react_use_state_for_type! {{
            input: $type_value,
            output: $type_value,
            $($t)+
        }}
    };
    ({
        input: $type_input:ty,
        output: $type_output:ty,
        object: $name_object:ident,
        setter: $name_setter:ident,
        use_state: $name_use:ident,
        use_state_with: $name_use_with:ident,
        auto_clean: $name_clean:ident,
        auto_clean_with: $name_clean_with:ident
        $(,)?
    }) => {
        wasm_bindgen_react_state_helpers! {
            #[derive(Debug, Clone)]
            pub type $name_object;
            #[wasm_bindgen(structural, method, getter)]
            fn value(this: &$name_object) -> $type_output;
            #[wasm_bindgen(structural, method, getter)]
            fn setter(this: &$name_object) -> $name_setter;

            #[derive(Debug, Clone)]
            pub type $name_setter;
            #[wasm_bindgen(structural, method)]
            fn set_state(this: &$name_setter, value: $type_input);
            /// Closure `get_value_from_old` will be called immediately,
            /// thus it is safe to use reference here
            ///
            /// Closure get_value_from_old will be called only once
            #[wasm_bindgen(structural, method, js_name = "set_state")]
            fn set_state_with(
                this: &$name_setter,
                get_value_from_old: &mut dyn FnMut($type_output) -> $type_input,
            );

            /// `React.useState<T>(initial_value)`
            #[wasm_bindgen(js_name = "use_state_object")]
            fn $name_use(initial_value: $type_input) -> $name_object;

            /// `React.useState<T>(get_initial_value)`
            #[wasm_bindgen(js_name = "use_state_object")]
            #[doc = concat!("React.useState<`", stringify!($type_input), "`>(initial_value)")]
            fn $name_use_with(
                initial_value: &mut dyn FnMut() -> $type_input,
            ) -> $name_object;

            #[wasm_bindgen(js_name = "use_state_auto_clean")]
            fn $name_clean(
                initial_value: $type_input,
                free: &Closure<dyn FnMut($type_output)>,
            ) -> $name_object;

            #[wasm_bindgen(js_name = "use_state_auto_clean")]
            fn $name_clean_with(
                get_initial_value: &mut dyn FnMut() -> $type_input,
                free: &Closure<dyn FnMut($type_output)>,
            ) -> $name_object;
        }
    };
}

macro_rules! define_react_use_state_helpers {
    ($($t:tt),+ $(,)?) => {
        $( define_react_use_state_for_type!{ $t } )+
    };
}

pub(super) use define_react_use_state_for_type;
pub(super) use define_react_use_state_helpers;
pub(super) use wasm_bindgen_react_state_helpers;
