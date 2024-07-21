/// A Rust library providing functionality to display a loading screen while running tasks.
///
/// This library offers two functions:
///
/// - `with_loading_screen`: Runs a loading screen animation on a separate thread while executing a provided task synchronously.
/// - `with_loading_screen_async`: Runs a loading screen animation asynchronously while executing a provided task asynchronously.
///
/// The default loading screen animation is a donut. add your own if you're lame
///
/// # Examples
///
/// ```rust
/// use loading_screen::{with_loading_screen, with_loading_screen_async};
///
/// // Synchronous usage
/// with_loading_screen(None, || {
///     // Your task here
/// });
///
/// // Asynchronous usage
/// #[tokio::main]
/// async fn main() {
///     with_loading_screen_async(None, || {
///         // Your asynchronous task here
///     }).await;
/// }
/// ```
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

/// Runs a loading screen animation while executing a task synchronously.
///
/// # Parameters
///
/// - `loading_fn`: An optional function to display the loading screen. If `None`, the default donut spinner is used.
/// - `task_fn`: The task to be executed synchronously.
///
/// # Examples
///
/// ```rust
/// use loading_screen::{with_loading_screen, donut};
///
/// with_loading_screen(Some(donut), || {
///     // Your task here
/// });
/// ```
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

/// Runs a loading screen animation while executing a task synchronously.
///
/// # Parameters
///
/// - `loading_fn`: An optional function to display the loading screen. If `None`, the default donut spinner is used.
/// - `task_fn`: The task to be executed synchronously.
///
/// # Examples
///
/// ```rust
/// use loading_screen::{with_loading_screen, donut};
///
/// with_loading_screen(Some(donut), || {
///     // Your task here
/// });
/// ```
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
