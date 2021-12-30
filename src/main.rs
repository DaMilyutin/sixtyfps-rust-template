use std::sync::{Arc, Mutex};
use std::{collections::HashMap, rc::Rc, vec::Vec};
use tokio::spawn;

sixtyfps::include_modules!();
mod sync60;
use sixtyfps::Model;
use sync60::AsyncVec;

struct IdMap {
    id2row: HashMap<i16, i16>,
    row2id: Vec<i16>,
}

impl IdMap {
    fn push(&mut self, id: i16) {
        self.row2id.push(id);
        self.id2row.insert(id, self.row2id.len() as i16 - 1);
    }

    fn erase(&mut self, row: usize) {
        assert!(0 <= row && row < self.row2id.len());
        let mut r = row + 1;
        while r < self.row2id.len() {
            let id = self.row2id.get(r);
            *self.id2row.get_mut(id.unwrap()).unwrap() -= 1;
            r += 1;
        }
        self.id2row.remove(&self.row2id.get(row).unwrap());
        self.row2id.remove(row);
    }
}

fn main() {
    let ui = AppWindow::new();

    let ui_handle = ui.as_weak();

    let list_data = Arc::new(Mutex::new(Rc::new(sixtyfps::VecModel::default())));

    ui.set_task_data_model(sixtyfps::ModelHandle::new(
        list_data.lock().unwrap().clone(),
    ));

    ui.on_request_increase_value({
        let list = list_data.clone();
        move || {
            let ui = ui_handle.unwrap();
            let id = list.row_count() as i32;
            list.push(ListItemData {
                id: id,
                progress: (id * 10) as f32,
            });
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run();
}
