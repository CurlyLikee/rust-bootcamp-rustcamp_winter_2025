use task_1_8::{OnlySync, OnlySend,SyncAndSend ,NotSyncNotSend};
use std::thread;


fn main() {
    let only_sync = OnlySync::new();
    let only_sync_clone = only_sync.clone();
    let thread1 = thread::spawn(move || {
        OnlySync::use_in_thread(only_sync_clone);
    });
    thread1.join().unwrap();
    let only_send = OnlySend::new();
    let thread2 = thread::spawn(move || {
        OnlySend::use_in_thread(only_send);
    });
    thread2.join().unwrap();
    let sync_and_send = SyncAndSend::new();
    let sync_and_send_clone = sync_and_send.clone();
    let thread3 = thread::spawn(move || {
        SyncAndSend::use_in_thread(sync_and_send_clone);
    });
    thread3.join().unwrap();

    let not_sync_not_send = NotSyncNotSend::new();
    NotSyncNotSend::use_in_main_thread(not_sync_not_send);

    println!("All examples completed successfully!");
}
