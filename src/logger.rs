pub fn init() {
    log4rs::init_config(
        log4rs::Config::builder()
            .appender(
                log4rs::config::Appender::builder().build(
                    "stdout",
                    Box::new(
                        log4rs::append::console::ConsoleAppender::builder()
                            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                                "{d} {h({l})} {M}::{L} - {m}{n}",
                            )))
                            .build(),
                    ),
                ),
            )
            .build(
                log4rs::config::Root::builder()
                    .appender("stdout")
                    .build(log::LevelFilter::Info),
            )
            .unwrap(),
    )
    .unwrap();
}
