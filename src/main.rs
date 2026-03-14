use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let mut cli = agcli::cli::Cli::parse();

    // Load config file and apply defaults (CLI flags take precedence)
    let cfg = agcli::Config::load();
    cli.apply_config(&cfg);

    agcli::cli::commands::execute(cli).await
}
