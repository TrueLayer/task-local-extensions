use crate::Extensions;
use std::cell::RefCell;
use std::future::Future;

tokio::task_local! {
    static EXTENSIONS: RefCell<Extensions>;
}

/// Sets a task local to `Extensions` before `fut` is run,
/// and fetches the contents of the task local Extensions after completion
/// and returns it.
pub async fn with_extensions<T>(
    extensions: Extensions,
    fut: impl Future<Output = T>,
) -> (Extensions, T) {
    EXTENSIONS
        .scope(RefCell::new(extensions), async move {
            let response = fut.await;
            let extensions = RefCell::new(Extensions::new());

            EXTENSIONS.with(|ext| ext.swap(&extensions));

            (extensions.into_inner(), response)
        })
        .await
}

/// Retrieve any item from task-local storage.
pub async fn get_local_item<T: Send + Sync + Clone + 'static>() -> Option<T> {
    EXTENSIONS
        .try_with(|e| e.borrow().get::<T>().cloned())
        .ok()
        .flatten()
}

/// Set an item in task-local storage.
pub async fn set_local_item<T: Send + Sync + 'static>(item: T) {
    EXTENSIONS
        .try_with(|e| e.borrow_mut().insert(item))
        .expect("Failed to set local item.");
}
