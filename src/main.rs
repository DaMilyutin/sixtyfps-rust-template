sixtyfps::include_modules!();

use sixtyfps::{Model, VecModel};
use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
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

    let list_data = Rc::new(VecModel::<ListItemData>::default());
    let task_data = Arc::new(Mutex::new(IdMapData::new(list_data.clone())));

    ui.set_task_data_model(sixtyfps::ModelHandle::new(list_data.clone()));

    ui.on_request_increase_value({
        let task_data = task_data.clone();
        let model_handle = ui.get_task_data_model();
        move || {
            let ui = ui_handle.unwrap();
            let id = nextId() as i32;
            let model_handle = ui.get_task_data_model();
            let data = model_handle
                .as_any()
                .downcast_ref::<VecModel<ListItemData>>()
                .unwrap();
            task_data.lock().as_deref_mut().unwrap().push(data, id);
            let latency = ui.get_latency();
            let period = time::Duration::from_millis(10);

            let ui_handle = ui.as_weak();

            let task_data = task_data.clone();

            std::thread::spawn(move || {
                for i in 0..101 {
                    thread::sleep(period);
                    let task_data = task_data.clone();
                    let ui_handle = ui_handle.clone();
                    sixtyfps::invoke_from_event_loop(move || {
                        let ui = ui_handle.unwrap();
                        let model_handle = ui.get_task_data_model();
                        let data = model_handle
                            .as_any()
                            .downcast_ref::<VecModel<ListItemData>>()
                            .unwrap();
                        task_data
                            .lock()
                            .as_deref_mut()
                            .unwrap()
                            .set_progress(data, id, i as f32)
                    });
                }
                thread::sleep(time::Duration::from_secs(1));
                sixtyfps::invoke_from_event_loop(move || {
                    let ui = ui_handle.unwrap();
                    let model_handle = ui.get_task_data_model();
                    let data = model_handle
                        .as_any()
                        .downcast_ref::<VecModel<ListItemData>>()
                        .unwrap();
                    task_data
                        .lock()
                        .as_deref_mut()
                        .unwrap()
                        .remove_by_id(data, id);
                    ui.set_counter(ui.get_counter() + 1);
                });
            });

            //thread::sleep(time::Duration::from_secs(1));
            //task_data.lock().unwrap().remove_by_id(id);
        }
    });

    ui.run();
}
