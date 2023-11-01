mod state; 

use crate::core::state::RuntimeState;

pub async fn start_core_task() -> Result<(), ()> {
    let state = RuntimeState::new();    

    loop {

    }

    return Ok(());
}

