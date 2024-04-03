use tokio::{io::AsyncReadExt, net::TcpListener};

pub async fn _listen_for_messages(
    token: tokio_util::sync::CancellationToken, 
) {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address).await.unwrap();

    tokio::spawn(async move {
        loop {
            if token.is_cancelled() {
                break
            }

            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(listen(socket));
        }
    });
}

const BUFFER_SIZE: usize = 1024;

async fn listen(
    _socket: tokio::net::TcpStream,
) {
    let mut _buffer = [0; BUFFER_SIZE];
    loop {
    }
}
