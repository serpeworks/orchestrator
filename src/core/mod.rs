/// Core module for the Drones Server.

mod state; 

use crate::core::state::RuntimeState;

pub async fn start_core_task(
    token: tokio_util::sync::CancellationToken
) -> Result<(), ()> {
    let _state = RuntimeState::new();    

    loop {


        if token.is_cancelled() {
            break
        }
    }

    println!("done");

    return Ok(());
}

