use std::{fs::File, io::Read};

use kml::Kml;
use tokio_util::sync::CancellationToken;
use tracing::info;

pub fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_line_number(true)
        .with_thread_ids(true)
        .compact()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");
}

pub async fn setup_signal_handlers(token: CancellationToken) {
    // for ctrl-c
    #[cfg(unix)]
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Ctrl-C Signal received! Gracefully shutting down.");
        token.cancel();
    });
}

pub fn load_perimeter(filepath: String) -> Result<Kml, Box<dyn std::error::Error>> {
    let mut file = File::open(filepath)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let kml: Kml = contents.parse()?;

    if let Kml::Polygon(_) = kml {
        Ok(kml)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Expected a Polygon KML",
        )))
    }
}
