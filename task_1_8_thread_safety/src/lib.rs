use std::sync::{Arc, Mutex};
use std::cell::Cell;
use std::rc::Rc;


pub struct OnlySync {
    data: Mutex<()>,
}

impl OnlySync {
    pub fn new() -> Arc<Self> {
        Arc::new(Self { data: Mutex::new(()) })
    }

    pub fn use_in_thread(arc_instance: Arc<Self>) {
        let _lock = arc_instance.data.lock().unwrap();
        println!("OnlySync: Locked and used in a thread!");
    }
}


pub struct OnlySend {
    data: Cell<()>,
}

impl OnlySend {
    pub fn new() -> Self {
        Self { data: Cell::new(()) }
    }

    pub fn use_in_thread(instance: Self) {
        println!("OnlySend: Used in a thread!");
    }
}

pub struct SyncAndSend {
    data: Mutex<()>,
}

impl SyncAndSend {
    pub fn new() -> Arc<Self> {
        Arc::new(Self { data: Mutex::new(()) })
    }

    pub fn use_in_thread(arc_instance: Arc<Self>) {
        let _lock = arc_instance.data.lock().unwrap();
        println!("SyncAndSend: Locked and used in a thread!");
    }
}

pub struct NotSyncNotSend {
    data: Rc<()>,
}

impl NotSyncNotSend {
    pub fn new() -> Self {
        Self { data: Rc::new(()) }
    }

    pub fn use_in_main_thread(instance: Self) {
        println!("NotSyncNotSend: Used in the main thread!");
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_only_sync_is_sync_but_not_send() {
        let only_sync = OnlySync::new();
        let only_sync_clone = only_sync.clone();
        thread::spawn(move || {
            OnlySync::use_in_thread(only_sync_clone);
        })
        .join()
        .unwrap();
        fn assert_sync<T: Sync>() {}
        assert_sync::<OnlySync>();
    }



    #[test]
    fn test_only_send_is_send_but_not_sync() {
        let only_send = OnlySend::new();
        thread::spawn(move || {
            OnlySend::use_in_thread(only_send);
        })
        .join()
        .unwrap();
        fn assert_send<T: Send>() {}
        assert_send::<OnlySend>();
    }



    #[test]
    fn test_sync_and_send_is_both_sync_and_send() {
        let sync_and_send = SyncAndSend::new();
        let sync_and_send_clone = sync_and_send.clone();
        thread::spawn(move || {
            SyncAndSend::use_in_thread(sync_and_send_clone);
        })
        .join()
        .unwrap();
        fn assert_sync_and_send<T: Sync + Send>() {}
        assert_sync_and_send::<SyncAndSend>();
    }

    

    #[test]
    fn test_not_sync_not_send_is_not_sync_and_not_send() {
        let not_sync_not_send = NotSyncNotSend::new();
        NotSyncNotSend::use_in_main_thread(not_sync_not_send);
    }
}
