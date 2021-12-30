use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::{collections::HashMap, rc::Rc, vec::Vec};
sixtyfps::include_modules!();
mod sync60;
use sixtyfps::Model;
use sync60::AsyncVec;

fn nextId() -> i32 {
    unsafe {
        static mut n: i32 = 0 as i32;
        const period: i32 = (1 << 15) as i32;
        n = (n + 1i32) % period;
        n
    }
}
struct IdMapData {
    id2row: HashMap<i32, usize>,
    row2id: Vec<i32>,
    data: Rc<sixtyfps::VecModel<ListItemData>>,
}

impl IdMapData {
    fn default() -> IdMapData {
        IdMapData {
            id2row: HashMap::<i32, usize>::default(),
            row2id: Vec::<i32>::default(),
            data: Rc::new(sixtyfps::VecModel::default()),
        }
    }

    fn push(&mut self, id: i32) {
        self.row2id.push(id);
        self.id2row.insert(id, self.row2id.len() - 1);
        self.data.push(ListItemData {
            id: id,
            progress: 0f32,
        })
    }

    fn set_progress(&mut self, id: i32, progress: f32) {
        let row = *self.id2row.get(&id).unwrap();
        self.data.set_row_data(
            row,
            ListItemData {
                id: id,
                progress: progress,
            },
        );
    }

    fn remove_by_id(&mut self, id: i32) {
        let row = *self.id2row.get(&id).unwrap();
        self.remove_by_row(row);
    }

    fn remove_by_row(&mut self, row: usize) {
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

    let task_data = Arc::new(Mutex::new(IdMapData::default()));
    ui.set_task_data_model(sixtyfps::ModelHandle::new(
        task_data.lock().unwrap().data.clone(),
    ));

    ui.on_request_increase_value({
        let task_data = task_data.clone();
        move || {
            let task_data = task_data.clone();
            thread::spawn(move || {
                let ui = ui_handle.unwrap();
                let id = nextId() as i32;
                task_data.lock().unwrap().push(id);
                let latency = ui.get_latency();
                let period = time::Duration::from_millis(((10.0 as f32) * latency) as u64);
                ui.set_counter(ui.get_counter() + 1);
                thread::sleep(time::Duration::from_secs(1));
                task_data.lock().unwrap().remove_by_id(id);
            });
        }
    });
    ui.run();
}
