# react-sys

[![Crates.io](https://img.shields.io/crates/v/react-sys?style=for-the-badge)](https://crates.io/crates/react-sys)
[![docs.rs](https://img.shields.io/docsrs/react-sys/latest?style=for-the-badge)](https://docs.rs/react-sys)
[![GitHub license](https://img.shields.io/github/license/frender-rs/react-sys?style=for-the-badge)](https://github.com/frender-rs/react-sys/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/frender-rs/react-sys?style=for-the-badge)](https://github.com/frender-rs/react-sys/stargazers)

Rust bindings for `React`.

# Install

`Cargo.toml`

```toml
react-sys = "1.0.0-alpha.8"
```

Though the version is at `1.x-alpha`,
this crate is NOT ready for production
and under heavy development.
This crate is used by [`frender`](https://github.com/frender-rs/frender), which provides a friendly and safe api to use React in rust.

# TODO

- Hooks

  - [x] `React.useState`
  - [x] `React.useRef`
  - [x] `React.useEffect` `unsafe`
  - [ ] `React.useContext`
  - [ ] `useLayoutEffect`

  - [ ] Can we implement the following hooks in rust ?

    - [ ] `React.useMemo`
    - [ ] `React.useCallback`

      Maybe we should implement a `use_closure` hook in rust.

  - [ ] Do we need the following hooks in rust ?

    - [ ] `React.useReducer`
    - [ ] `React.useImperativeHandle`
    - [ ] `useDebugValue`

- [ ] `ErrorBoundary`
- [ ] `React.memo`

- [ ] `Component`

  - [ ] html
  - [ ] functional component with hooks

- [ ] `Element`

  - [x] `React.createElement`
  - [ ] `React.Element` cast between js and rust wasm
  - [ ] `jsx` like syntax to create element
