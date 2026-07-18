use clap::Parser;
use codex_arg0::Arg0DispatchPaths;
use codex_arg0::arg0_dispatch_or_else;
use codex_config::LoaderOverrides;
use codex_tui::AppExitInfo;
use codex_tui::Cli;
use codex_tui::ExitReason;
use codex_tui::run_main;
use codex_utils_cli::CliConfigOverrides;
use std::io::Write;
use supports_color::Stream;

fn format_exit_messages(exit_info: AppExitInfo, color_enabled: bool) -> Vec<String> {
    let is_fatal = matches!(&exit_info.exit_reason, ExitReason::Fatal(_));
    let AppExitInfo {
        token_usage,
        thread_id,
        resume_hint,
        ..
    } = exit_info;

    let mut lines = Vec::new();
    if !token_usage.is_zero() {
        lines.push(token_usage.to_string());
    }

    if let Some(resume_cmd) = resume_hint {
        let command = if color_enabled {
            format!("\u{1b}[36m{resume_cmd}\u{1b}[39m")
        } else {
            resume_cmd
        };
        lines.push(format!("To continue this session, run {command}"));
    } else if is_fatal && let Some(thread_id) = thread_id {
        lines.push(format!("Session ID: {thread_id}"));
    }

    lines
}

#[derive(Parser, Debug)]
struct TopCli {
    #[clap(flatten)]
    config_overrides: CliConfigOverrides,

    /// Connect the TUI to a remote app server endpoint.
    #[arg(long = "remote", value_name = "ADDR", hide = true)]
    remote: Option<String>,

    /// Open the resume picker without filtering sessions by working directory.
    #[arg(long = "resume-all", hide = true, default_value_t = false)]
    resume_all: bool,

    #[clap(flatten)]
    inner: Cli,
}

fn prepare_launch(
    top_cli: TopCli,
) -> std::io::Result<(Cli, Option<codex_tui::RemoteAppServerEndpoint>)> {
    let mut inner = top_cli.inner;
    if top_cli.resume_all {
        inner.resume_picker = true;
        inner.resume_show_all = true;
    }
    inner
        .config_overrides
        .raw_overrides
        .splice(0..0, top_cli.config_overrides.raw_overrides);

    let remote_endpoint = top_cli
        .remote
        .as_deref()
        .map(codex_tui::resolve_remote_addr)
        .transpose()
        .map_err(std::io::Error::other)?;
    Ok((inner, remote_endpoint))
}

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|arg0_paths: Arg0DispatchPaths| async move {
        let (inner, remote_endpoint) = prepare_launch(TopCli::parse())?;
        let exit_info = run_main(
            inner,
            arg0_paths,
            LoaderOverrides::default(),
            remote_endpoint,
        )
        .await?;
        let is_fatal = match &exit_info.exit_reason {
            ExitReason::Fatal(message) => {
                eprintln!("ERROR: {message}");
                true
            }
            ExitReason::UserRequested => false,
        };

        let color_enabled = supports_color::on(Stream::Stdout).is_some();
        for line in format_exit_messages(exit_info, color_enabled) {
            println!("{line}");
        }
        if is_fatal {
            std::io::stdout().flush()?;
            std::process::exit(1);
        }
        Ok(())
    })
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
