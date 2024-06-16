use tracing::{debug, info, warn, span, Level};
use tracing_subscriber::FmtSubscriber;

pub fn example() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let number_of_yaks = 3;
    info!(number_of_yaks, "preparing to shave yaks");
    info!(a = 1, b = 1);
    debug!("debug message");
    warn!("warning!");

    let mut span = span!(Level::INFO, "my span");
    span.in_scope(|| {
        println!("I'm in span!")
    })
}
