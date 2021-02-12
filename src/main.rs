mod logger;

fn main() {
    logger::setup();
    log::info!("booting...")
}
