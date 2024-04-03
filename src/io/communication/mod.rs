use futures::StreamExt;

use tokio::net::TcpListener;
use tokio_util::codec::Framed;

pub async fn listen_for_messages(
    token: tokio_util::sync::CancellationToken, 
) {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address).await.unwrap();

    tokio::spawn(async move {
        loop {
            if token.is_cancelled() {
                break
            }

            tracing::info!("Listening for incoming connections on {}", address);
            let (socket, _) = listener.accept().await.unwrap();
            tracing::info!("Accepted connection from {:?}", socket.peer_addr().unwrap());
            tokio::spawn(listen(socket));
        }
    });
}

const BUFFER_SIZE: usize = 1024;

async fn listen(
    socket: tokio::net::TcpStream,
) {
    let mut _buffer = [0; BUFFER_SIZE];
    let serpelink_codec = serpelink::serpe::codecs::SerpeCodec;
    
    let mut framed = Framed::new(socket, serpelink_codec);


    while let Some(message) = framed.next().await {
        match message {
            Ok(message) => {
                tracing::info!("Received message: {:?}", message);
            }
            Err(e) => {
                tracing::error!("Error while receiving message: {:?}", e);
            }
        }
    }

    tracing::info!("Connection closed");
}
