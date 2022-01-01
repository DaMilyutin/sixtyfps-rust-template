sixtyfps::include_modules!();

use sixtyfps::Model;
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::{cell::RefCell, rc::Rc};
use std::{collections::HashMap, vec::Vec};

mod id_map_data;
use id_map_data::nextId;
use id_map_data::IdMapData;

fn main() {
    let ui = AppWindow::new();

    let ui_handle = ui.as_weak();

    let mut task_data = Rc::new(RefCell::new(IdMapData::default()));
    ui.set_task_data_model(sixtyfps::ModelHandle::new(
        task_data.as_ref().borrow().data.clone(),
    ));

    ui.on_request_increase_value({
        let mut task_data = task_data.clone();
        move || {
            let ui = ui_handle.unwrap();
            let id = nextId() as i32;
            (*task_data.borrow_mut()).push(id);
            let latency = ui.get_latency();
            let period = time::Duration::from_millis(((10.0 as f32) * latency) as u64);
            ui.set_counter(ui.get_counter() + 1);
            //thread::sleep(time::Duration::from_secs(1));
            //task_data.lock().unwrap().remove_by_id(id);
        }
    });
    ui.run();
}
