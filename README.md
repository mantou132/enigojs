Is [enigo](https://github.com/enigo-rs/enigo)'s nodejs binding

## Usage

```js
const enigojs = require('enigojs');

enigojs.mouseMoveTo(10, 10);
enigojs.keySequenceParse('{+CTRL}a{-CTRL}{+SHIFT}Hello World{-SHIFT}');
enigojs.mouseClick(enigojs.MouseButton.Right);
```

When the electron ABI version and the node version do not match,
you need to:

```bash
npm install electron-build-env neon-cli --save-dev
electron-build-env neon build neon-hello --release
```
see: [neon docs](https://neon-bindings.com/docs/electron-apps/)

## Requirement

- [Rust & Cargo](https://www.rust-lang.org/learn/get-started)
- [Linux dependencies](https://github.com/enigo-rs/enigo#runtime-dependencies)

