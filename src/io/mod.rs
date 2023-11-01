/// IO Module for the drones server.
///
///

pub async fn start_io_task(
    token: tokio_util::sync::CancellationToken
) -> Result<(), ()> {
   
    loop {

        if token.is_cancelled() {
            break;
        }
    }

    return Ok(());
}

