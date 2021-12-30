use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::{cell::RefCell, rc::Rc};
sixtyfps::include_modules!();

// mod id_map_data;
// use id_map_data::nextId;
// use id_map_data::IdMapData;

use sixtyfps::Model;
use std::{collections::HashMap, vec::Vec};

pub fn nextId() -> i32 {
    unsafe {
        static mut n: i32 = 0 as i32;
        const period: i32 = (1 << 15) as i32;
        n = (n + 1i32) % period;
        n
    }
}

pub struct IdMapData {
    pub id2row: HashMap<i32, usize>,
    pub row2id: Vec<i32>,
    pub data: Rc<sixtyfps::VecModel<ListItemData>>,
}

impl IdMapData {
    pub fn default() -> IdMapData {
        IdMapData {
            id2row: HashMap::<i32, usize>::default(),
            row2id: Vec::<i32>::default(),
            data: Rc::new(sixtyfps::VecModel::default()),
        }
    }

    pub fn push(&mut self, id: i32) {
        self.row2id.push(id);
        self.id2row.insert(id, self.row2id.len() - 1);
        self.data.push(ListItemData {
            id: id,
            progress: 0f32,
        })
    }

    pub fn set_progress(&mut self, id: i32, progress: f32) {
        let row = *self.id2row.get(&id).unwrap();
        self.data.set_row_data(
            row,
            ListItemData {
                id: id,
                progress: progress,
            },
        );
    }

    pub fn remove_by_id(&mut self, id: i32) {
        let row = *self.id2row.get(&id).unwrap();
        self.remove_by_row(row);
    }

    pub fn remove_by_row(&mut self, row: usize) {
        assert!(0 <= row && row < self.row2id.len());
        let mut r = row + 1;
        while r < self.row2id.len() {
            let id = self.row2id.get(r);
            *self.id2row.get_mut(id.unwrap()).unwrap() -= 1;
            r += 1;
        }
        self.id2row.remove(&self.row2id.get(row).unwrap());
        self.row2id.remove(row);
        self.data.remove(row);
    }
}

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
