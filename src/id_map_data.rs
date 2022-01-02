use super::*;
use std::rc::Rc;
use std::{collections::HashMap, vec::Vec};

use futures::future::{Fuse, FusedFuture, FutureExt};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use sixtyfps::{ComponentHandle, Model, ModelHandle, SharedString, VecModel};

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
    //pub data: Rc<sixtyfps::VecModel<ListItemData>>,
}

impl IdMapData {
    pub fn new(data: Rc<sixtyfps::VecModel<ListItemData>>) -> IdMapData {
        IdMapData {
            id2row: HashMap::<i32, usize>::default(),
            row2id: Vec::<i32>::default(),
            //data: data,
        }
    }

    pub fn push(&mut self, data: &sixtyfps::VecModel<ListItemData>, id: i32) {
        self.row2id.push(id);
        self.id2row.insert(id, self.row2id.len() - 1);
        data.push(ListItemData {
            id: id,
            progress: 0f32,
        })
    }

    pub fn set_progress(&self, data: &sixtyfps::VecModel<ListItemData>, id: i32, progress: f32) {
        let row = *self.id2row.get(&id).unwrap();
        data.set_row_data(
            row,
            ListItemData {
                id: id,
                progress: progress,
            },
        );
    }

    pub fn remove_by_id(&mut self, data: &sixtyfps::VecModel<ListItemData>, id: i32) {
        let row = *self.id2row.get(&id).unwrap();
        self.remove_by_row(data, row);
    }

    pub fn remove_by_row(&mut self, data: &sixtyfps::VecModel<ListItemData>, row: usize) {
        assert!(0 <= row && row < self.row2id.len());
        let mut r = row + 1;
        while r < self.row2id.len() {
            let id = self.row2id.get(r);
            *self.id2row.get_mut(id.unwrap()).unwrap() -= 1;
            r += 1;
        }
        self.id2row.remove(&self.row2id.get(row).unwrap());
        self.row2id.remove(row);
        data.remove(row);
    }

    pub fn quit(&mut self, data: &sixtyfps::VecModel<ListItemData>) {
        let mut r = self.row2id.len();
        while r != 0 {
            self.remove_by_row(data, r - 1);
        }
    }
}

pub enum IdMapMessage {
    PushId { id: i32 },
    SetProgress { id: i32, progress: f32 },
    RemoveId { id: i32 },
    CancelId { id: i32 },
    Quit,
}

// pub struct IdMapWorker {
//     pub channel: UnboundedSender<IdMapMessage>,
//     id_map_data: Arc<Mutex<IdMapData>>,
//     worker_thread: std::thread::JoinHandle<()>,
// }

// impl IdMapWorker {
//     pub fn new(cargo_ui: &AppWindow, data: Arc<Mutex<IdMapData>>) -> Self {
//         let (channel, r) = tokio::sync::mpsc::unbounded_channel();
//         let worker_thread = std::thread::spawn({
//             let handle_weak = cargo_ui.as_weak();
//             move || {
//                 tokio::runtime::Runtime::new()
//                     .unwrap()
//                     .block_on(progress_worker_loop(r, handle_weak, data.clone()))
//                     .unwrap()
//             }
//         });
//         Self {
//             channel,
//             id_map_data: data.clone(),
//             worker_thread,
//         }
//     }

//     pub fn join(self) -> std::thread::Result<()> {
//         let _ = self.channel.send(IdMapMessage::Quit);
//         self.worker_thread.join()
//     }
// }

// async fn progress_worker_loop(
//     mut r: UnboundedReceiver<IdMapMessage>,
//     handle: sixtyfps::Weak<AppWindow>,
//     data: Arc<Mutex<IdMapData>>,
// ) -> tokio::io::Result<()> {
//     //let run_app = Fuse::terminated();
//     //futures::pin_mut!(run_app);

//     loop {
//         let m = futures::select! {
//             m = r.recv().fuse() => {
//                 match m {
//                     None => return Ok(()),
//                     Some(m) => m,
//                 }
//             }
//         };

//         match m {
//             IdMapMessage::PushId { id } => sixtyfps::invoke_from_event_loop({
//                 let wh = handle.clone();
//                 let wd = data.clone();
//                 move || {
//                     wd.get_mut().push(id);
//                 }
//             }),
//             IdMapMessage::Quit => return Ok(()),
//             _ => return Ok(()),
//         };
//     }
// }
