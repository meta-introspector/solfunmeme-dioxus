use dioxus::{
    html::{FileEngine, HasFileData},
    prelude::{FormEvent, *},
};
use std::{pin::Pin, sync::Arc};
pub fn create_upload_handler<T>(
    read_files: impl Fn(Arc<dyn FileEngine>) -> T + 'static + Clone,
) -> impl Fn(FormEvent) -> Pin<Box<dyn std::future::Future<Output = ()>>> {
    let read_files2 = read_files.clone();
    move |evt: FormEvent| {
        Box::pin({
            let value = read_files2.clone();
            async move {
                if let Some(file_engine) = evt.files() {
                    value(file_engine);
                }
            }
        })
    }
}
pub fn create_drop_handler<T>(
    read_files: impl Fn(Arc<dyn FileEngine>) -> T + 'static + Clone,
) -> impl Fn(DragEvent) -> Pin<Box<dyn std::future::Future<Output = ()>>> {
    let read_files2 = read_files.clone();
    move |evt: DragEvent| {
        Box::pin({
            let value = read_files2.clone();
            async move {
                if let Some(file_engine) = evt.files() {
                    value(file_engine);
                }
            }
        })
    }
}

// fn create_upload_handler<T>(
//     read_files: impl Fn(Arc<dyn FileEngine>) -> T + 'static + Clone
// ) -> impl Fn(FormEvent) -> Pin<Box<dyn std::future::Future<Output = ()>>> {
//     let read_files2= read_files.clone();
//     move |evt: FormEvent| Box::pin(async move {
//         if let Some(file_engine) = evt.files() {
//             read_files2(file_engine);
//         }
//     })
// }
