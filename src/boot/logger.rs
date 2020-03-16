use log;
use simplelog;

pub fn init() {
    let log_config = simplelog::ConfigBuilder::new()
        .set_time_format_str("[%+]")
        .build();

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            log_config.clone(),
            simplelog::TerminalMode::Mixed,
        )
        .unwrap(),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            log_config,
            std::fs::OpenOptions::new()
                .append(true)
                .open("/usr/local/var/log/potato.log")
                .unwrap(),
        ),
    ])
    .unwrap();

    log::info!("~ Potato Boot Up ~");
}
