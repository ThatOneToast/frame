use zed_extension_api::{self as zed, Result};

struct FrameExtension;

impl zed::Extension for FrameExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let command = worktree.which("frame_lsp").ok_or_else(|| {
            "Could not find `frame_lsp` on PATH. Build it with `cargo build -p frame_lsp` and add `target/debug` to the environment Zed uses.".to_string()
        })?;

        Ok(zed::Command {
            command,
            args: Vec::new(),
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(FrameExtension);
