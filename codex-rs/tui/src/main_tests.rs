use super::TopCli;
use super::prepare_launch;
use clap::Parser;
use codex_tui::RemoteAppServerEndpoint;
use pretty_assertions::assert_eq;

#[test]
fn prepares_remote_resume_picker() -> std::io::Result<()> {
    let top_cli = TopCli::try_parse_from([
        "codex-tui",
        "--remote",
        "unix:///tmp/codex-test.sock",
        "-C",
        "/tmp/project",
        "--resume-all",
        "-m",
        "test-model",
    ])
    .expect("parse");

    let (cli, remote_endpoint) = prepare_launch(top_cli)?;

    assert!(cli.resume_picker);
    assert!(cli.resume_show_all);
    assert_eq!(cli.model.as_deref(), Some("test-model"));
    assert_eq!(
        cli.cwd.as_deref(),
        Some(std::path::Path::new("/tmp/project"))
    );
    let Some(RemoteAppServerEndpoint::UnixSocket { socket_path }) = remote_endpoint else {
        panic!("expected a Unix socket endpoint");
    };
    assert_eq!(
        socket_path.as_path(),
        std::path::Path::new("/tmp/codex-test.sock")
    );
    Ok(())
}
