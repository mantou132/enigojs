use neon::prelude::*;
use enigo::*;

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
        10=> Key::F11,
        11=> Key::F12,
        12=> Key::F2,
        13=> Key::F3,
        14=> Key::F4,
        15=> Key::F5,
        16=> Key::F6,
        17=> Key::F7,
        18=> Key::F8,
        19=> Key::F9,
        20=> Key::Home,
        21=> Key::LeftArrow,
        22=> Key::Meta,
        23=> Key::Option,
        24=> Key::PageDown,
        25=> Key::PageUp,
        26=> Key::Return,
        27=> Key::RightArrow,
        28=> Key::Shift,
        29=> Key::Space,
        30=> Key::Tab,
        31=> Key::UpArrow,
        32=> Key::Layout('a'),
        33=> Key::Layout('b'),
        34=> Key::Layout('c'),
        35=> Key::Layout('d'),
        36=> Key::Layout('e'),
        37=> Key::Layout('f'),
        38=> Key::Layout('g'),
        39=> Key::Layout('h'),
        40=> Key::Layout('i'),
        41=> Key::Layout('j'),
        42=> Key::Layout('k'),
        43=> Key::Layout('l'),
        44=> Key::Layout('m'),
        45=> Key::Layout('n'),
        46=> Key::Layout('o'),
        47=> Key::Layout('p'),
        48=> Key::Layout('q'),
        49=> Key::Layout('r'),
        50=> Key::Layout('s'),
        51=> Key::Layout('t'),
        52=> Key::Layout('u'),
        53=> Key::Layout('v'),
        54=> Key::Layout('w'),
        55=> Key::Layout('x'),
        56=> Key::Layout('y'),
        57=> Key::Layout('z'),
        _ => unreachable!()
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
        _ => unreachable!()
    }
}

fn mouse_move_to(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let x = cx.argument::<JsNumber>(0)?.value() as i32;
    let y = cx.argument::<JsNumber>(1)?.value() as i32;
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
    Ok(cx.undefined())
}

fn mouse_move_relative(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let x = cx.argument::<JsNumber>(0)?.value() as i32;
    let y = cx.argument::<JsNumber>(1)?.value() as i32;
    let mut enigo = Enigo::new();
    enigo.mouse_move_relative(x, y);
    Ok(cx.undefined())
}

fn key_down(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
    let mut enigo = Enigo::new();
    enigo.key_down(get_key(js_enum_value));
    Ok(cx.undefined())
}

fn key_up(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
    let mut enigo = Enigo::new();
    enigo.key_up(get_key(js_enum_value));
    Ok(cx.undefined())
}

fn key_click(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
    let mut enigo = Enigo::new();
    enigo.key_click(get_key(js_enum_value));
    Ok(cx.undefined())
}

fn key_sequence(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let s = cx.argument::<JsString>(0)?.value();
    let mut enigo = Enigo::new();
    enigo.key_sequence(&s);
    Ok(cx.undefined())
}

fn key_sequence_parse(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let s = cx.argument::<JsString>(0)?.value();
    let mut enigo = Enigo::new();
    enigo.key_sequence_parse(&s);
    Ok(cx.undefined())
}

fn mouse_down(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
    let mut enigo = Enigo::new();
    enigo.mouse_down(get_mouse_button(js_enum_value));
    Ok(cx.undefined())
}

fn mouse_up(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
    let mut enigo = Enigo::new();
    enigo.mouse_up(get_mouse_button(js_enum_value));
    Ok(cx.undefined())
}

fn mouse_click(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_enum_value = cx.argument::<JsNumber>(0)?.value() as i32;
    let mut enigo = Enigo::new();
    enigo.mouse_click(get_mouse_button(js_enum_value));
    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("mouseMoveTo", mouse_move_to)?;
    cx.export_function("mouseMoveToRelative", mouse_move_relative)?;
    cx.export_function("keyDown", key_down)?;
    cx.export_function("keyUp", key_up)?;
    cx.export_function("keyClick", key_click)?;
    cx.export_function("keySequence", key_sequence)?;
    cx.export_function("keySequenceParse", key_sequence_parse)?;
    cx.export_function("mouseDown", mouse_down)?;
    cx.export_function("mouseUp", mouse_up)?;
    cx.export_function("mouseClick", mouse_click)?;
    Ok(())
});
