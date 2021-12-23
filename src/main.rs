use std::{collections::HashMap, vec::Vec};

sixtyfps::include_modules!();

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

    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    ui.run();
}
