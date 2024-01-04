#[macro_use]
mod browser;

use gloo_utils::format::JsValueSerdeExt;
use serde::Deserialize;
use std::{collections::HashMap, rc::Rc, sync::Mutex};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let context = browser::context().expect("Could not get browser context");

    browser::spawn_local(async move {
        let sheet: Sheet = browser::fetch_json("/static/rhb.json")
            .await
            .expect("Could not fetch rhb.json")
            .into_serde()
            .expect("Could not convert rhb.json into a Sheet structure");
        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = Rc::new(Mutex::new(Some(success_tx)));
        let error_tx = Rc::clone(&success_tx);
        let image = web_sys::HtmlImageElement::new().unwrap();

        let callback = Closure::once(move || {
            if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
                success_tx.send(Ok(()));
            }
        });

        let error_callback = Closure::once(move |err| {
            if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
                error_tx.send(Err(err));
            }
        });

        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
        image.set_src("/static/rhb.png");

        let mut frame = -1;
        let interval_callback = Closure::wrap(Box::new(move || {
            frame = (frame + 1) % 8;
            context.clear_rect(0.0, 0.0, 600.0, 600.0);
            let frame_name = format!("Run ({}).png", frame + 1);
            let stripe = sheet.frames.get(&frame_name).expect("Cell not found");
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                stripe.frame.x.into(),
                stripe.frame.y.into(),
                stripe.frame.w.into(),
                stripe.frame.h.into(),
                300.0,
                300.0,
                stripe.frame.w.into(),
                stripe.frame.h.into(),
            );
        }) as Box<dyn FnMut()>);
        browser::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                interval_callback.as_ref().unchecked_ref(),
                50,
            );
        interval_callback.forget();

        success_rx.await;
    });

    Ok(())
}
