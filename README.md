Is [enigo](https://github.com/enigo-rs/enigo)'s js binding

## Usage

```js
import { mouseMoveTo, keySequenceParse, mouseClick } from 'enigojs';

mouseMoveTo(10, 10);

keySequenceParse('{+CTRL}a{-CTRL}{+SHIFT}Hello World{-SHIFT}');

// 2 is MouseRightButton
mouseClick(2);
```

## Requirement

- [Rust & Cargo](https://www.rust-lang.org/learn/get-started)
- [Linux dependencies](https://github.com/enigo-rs/enigo#runtime-dependencies)

