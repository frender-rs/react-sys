use wasm_bindgen::prelude::*;

macro_rules! define_react_use_state_for_type {
    ( value: $type_value:ty, $($t:tt)+ ) => {
        define_react_use_state_for_type!(
            input: $type_value,
            output: $type_value,
            $($t)+
        );
    };
    (
        input: $type_input:ty,
        output: $type_output:ty,
        object: $name_object:ident,
        setter: $name_setter:ident,
        use_state: $name_use:ident,
        use_state_with: $name_use_with:ident,
        auto_clean: $name_clean:ident,
        auto_clean_with: $name_clean_with:ident
        $(,)?
    ) => {
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

        #[wasm_bindgen(js_name = "use_state_object")]
        fn $name_use(initial_value: $type_input) -> $name_object;

        #[wasm_bindgen(js_name = "use_state_object")]
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
    };
}

// #[cfg_attr(
//     feature = "import-react",
//     wasm_bindgen(inline_js = r#"
// import * as React from "react";
// export function use_state_object(initial_value) {
//     const [state, set_state] = React.useState(initial_value);
//     return { value: state, setter: { set_state } };
// }
// export function use_state_auto_clean(initial_value, clean) {
//     const obj = use_state_object(initial_value);
//     const state = obj.value;
//     React.useEffect(() => { clean(state) }, [state]);
// }
// "#)
// )]
// #[cfg_attr(
//     not(feature = "import-react"),
//     wasm_bindgen(inline_js = r#"
// export function use_state_object(initial_value) {
//     const [state, set_state] = React.useState(initial_value);
//     return { value: state, setter: { set_state } };
// }
// export function use_state_auto_clean(initial_value, clean) {
//     const obj = use_state_object(initial_value);
//     const state = obj.value;
//     React.useEffect(() => { clean(state) }, [state]);
// }
// "#)
// )]

macro_rules! ttt {
    () => {
        type Element;
    };
}

#[wasm_bindgen]
extern "C" {
    define_react_use_state_for_type! {
        value: usize,
        object: UseStateUsizeObject,
        setter: UseStateUsizeObjectSetter,
        use_state: use_state_object_usize,
        use_state_with: use_state_object_usize_with,
        auto_clean: use_state_usize_auto_clean,
        auto_clean_with: use_state_usize_auto_clean_with,
    }
}
