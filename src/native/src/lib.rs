use neon::prelude::*;
use enigo::*;

fn mouse_move_to(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let x = cx.argument::<JsNumber>(0)?.value() as i32;
    let y = cx.argument::<JsNumber>(1)?.value() as i32;
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
    Ok(cx.undefined())
}

fn key_sequence_parse(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let s = cx.argument::<JsString>(0)?.value();
    let mut enigo = Enigo::new();
    enigo.key_sequence_parse(&s);
    Ok(cx.undefined())
}

fn mouse_click(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let button = cx.argument::<JsNumber>(0)?.value() as i32;
    let mut enigo = Enigo::new();
    enigo.mouse_click(match button {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        3 => MouseButton::ScrollUp,
        4 => MouseButton::ScrollDown,
        5 => MouseButton::ScrollLeft,
        6 => MouseButton::ScrollRight,
        _ => unreachable!()
    });
    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("mouseMoveTo", mouse_move_to)?;
    cx.export_function("keySequenceParse", key_sequence_parse)?;
    cx.export_function("mouseClick", mouse_click)?;
    Ok(())
});
