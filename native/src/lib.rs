use enigo::*;
use neon::borrow::*;
use neon::prelude::*;

/// https://neon-bindings.com/docs/classes
/// https://github.com/enigo-rs/enigo/issues/80#issuecomment-555815618

fn get_key(js_enum_value: i32) -> Key {
    match js_enum_value {
        0 => Key::Alt,
        1 => Key::Backspace,
        2 => Key::CapsLock,
        3 => Key::Control,
        4 => Key::Delete,
        5 => Key::DownArrow,
        6 => Key::End,
        7 => Key::Escape,
        8 => Key::F1,
        9 => Key::F10,
        10 => Key::F11,
        11 => Key::F12,
        12 => Key::F2,
        13 => Key::F3,
        14 => Key::F4,
        15 => Key::F5,
        16 => Key::F6,
        17 => Key::F7,
        18 => Key::F8,
        19 => Key::F9,
        20 => Key::Home,
        21 => Key::LeftArrow,
        22 => Key::Meta,
        23 => Key::Option,
        24 => Key::PageDown,
        25 => Key::PageUp,
        26 => Key::Return,
        27 => Key::RightArrow,
        28 => Key::Shift,
        29 => Key::Space,
        30 => Key::Tab,
        31 => Key::UpArrow,
        32 => Key::Layout('a'),
        33 => Key::Layout('b'),
        34 => Key::Layout('c'),
        35 => Key::Layout('d'),
        36 => Key::Layout('e'),
        37 => Key::Layout('f'),
        38 => Key::Layout('g'),
        39 => Key::Layout('h'),
        40 => Key::Layout('i'),
        41 => Key::Layout('j'),
        42 => Key::Layout('k'),
        43 => Key::Layout('l'),
        44 => Key::Layout('m'),
        45 => Key::Layout('n'),
        46 => Key::Layout('o'),
        47 => Key::Layout('p'),
        48 => Key::Layout('q'),
        49 => Key::Layout('r'),
        50 => Key::Layout('s'),
        51 => Key::Layout('t'),
        52 => Key::Layout('u'),
        53 => Key::Layout('v'),
        54 => Key::Layout('w'),
        55 => Key::Layout('x'),
        56 => Key::Layout('y'),
        57 => Key::Layout('z'),
        _ => unreachable!(),
    }
}

fn get_mouse_button(js_enum_value: i32) -> MouseButton {
    match js_enum_value {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        3 => MouseButton::ScrollUp,
        4 => MouseButton::ScrollDown,
        5 => MouseButton::ScrollLeft,
        6 => MouseButton::ScrollRight,
        _ => unreachable!(),
    }
}

fn run<F>(
    f: F,
    mut cx: CallContext<'_, JsEnigo>,
) -> Result<Handle<'_, JsValue>, neon::result::Throw>
where
    F: Fn(RefMut<'_, &mut Enigo>) -> (),
{
    let mut this = cx.this();
    let guard = cx.lock(); // immutable borrow
    let enigo = this.borrow_mut(&guard);
    f(enigo); // move
    Ok(cx.undefined().upcast())
}

declare_types! {
    pub class JsEnigo for Enigo {
        init(_cx) {
            Ok(Enigo::new())
        }
        method mouseMoveTo(mut cx) {
            let x = cx.argument::<JsNumber>(0)?.value() as i32;
            let y = cx.argument::<JsNumber>(1)?.value() as i32;
            run(|mut enigo| {
                enigo.mouse_move_to(x, y);
            }, cx)
        }
        method mouseMoveRelative(mut cx) {
            let x = cx.argument::<JsNumber>(0)?.value() as i32;
            let y = cx.argument::<JsNumber>(1)?.value() as i32;
            run(|mut enigo| {
                enigo.mouse_move_relative(x, y);
            }, cx)
        }
        method mouseDown(mut cx) {
            let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
            run(|mut enigo| {
                enigo.mouse_down(get_mouse_button(js_enum_value));
            }, cx)
        }
        method mouseUp(mut cx) {
            let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
            run(|mut enigo| {
                enigo.mouse_up(get_mouse_button(js_enum_value));
            }, cx)
        }
        method mouseClick(mut cx) {
            let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
            run(|mut enigo| {
                enigo.mouse_click(get_mouse_button(js_enum_value));
            }, cx)
        }
        method keyDown(mut cx) {
            let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
            run(|mut enigo| {
                enigo.key_down(get_key(js_enum_value));
            }, cx)
        }
        method keyUp(mut cx) {
            let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
            run(|mut enigo| {
                enigo.key_up(get_key(js_enum_value));
            }, cx)
        }
        method keyClick(mut cx) {
            let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
            run(|mut enigo| {
                enigo.key_click(get_key(js_enum_value));
            }, cx)
        }
        method keySequence(mut cx) {
            let s = cx.argument::<JsString>(0)?.value();
            run(|mut enigo| {
                enigo.key_sequence(&s);
            }, cx)
        }
        method keySequenceParse(mut cx) {
            let s = cx.argument::<JsString>(0)?.value();
            run(|mut enigo| {
                enigo.key_sequence_parse(&s);
            }, cx)
        }
    }
}

register_module!(mut m, { m.export_class::<JsEnigo>("Enigo") });
