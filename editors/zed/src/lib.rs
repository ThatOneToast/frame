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
        let command = worktree
            .which("frame_lsp")
            .unwrap_or_else(|| format!("{}/target/debug/frame_lsp", worktree.root_path()));

        Ok(zed::Command {
            command,
            args: Vec::new(),
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(FrameExtension);
