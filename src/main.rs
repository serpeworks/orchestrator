mod core;
mod io;

#[tokio::main]
async fn main() {
     
    let _ = tokio::try_join!(
        core::start_core_task(),
        io::start_io_task()
    );
    //core::start_core_task();

}

