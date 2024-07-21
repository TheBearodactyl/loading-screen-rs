pub mod donut;
use {
    donut::donut,
    std::{
        sync::{
            Arc,
            Mutex,
        },
        thread::{
            sleep,
            spawn,
        },
        time::Duration,
    },
    tokio::spawn as async_spawn,
};

pub async fn with_loading_screen_async<F>(loading_fn: Option<fn()>, task_fn: F)
where
    F: FnOnce() + Send + 'static,
{
    let loading_fn = loading_fn.unwrap_or(donut);
    let loading_handle = async_spawn(async move {
        loading_fn();
    });

    let task_handle = async_spawn(async move {
        task_fn();
    });

    task_handle.await.unwrap();
    loading_handle.abort();
}

pub fn with_loading_screen<F>(loading_fn: Option<fn()>, task_fn: F)
where
    F: FnOnce() + Send + 'static,
{
    let loading_fn = loading_fn.unwrap_or(donut);
    let loading_done = Arc::new(Mutex::new(false));
    let loading_done_clone = Arc::clone(&loading_done);
    let loading_thread = spawn(move || {
        loading_fn();
        let mut done = loading_done_clone.lock().unwrap();
        *done = true;
    });

    let task_thread = spawn(move || {
        task_fn();
    });

    task_thread.join().unwrap();

    while !*loading_done.lock().unwrap() {
        sleep(Duration::from_millis(100));
    }

    loading_thread.join().unwrap();
}
