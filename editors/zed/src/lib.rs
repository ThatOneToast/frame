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
        let env = worktree.shell_env();
        let command = env
            .iter()
            .find(|(key, _)| key == "FRAME_LSP")
            .map(|(_, value)| value.clone())
            .or_else(|| worktree.which("frame_lsp"))
            .unwrap_or_else(|| {
                "/Users/whitebread/projects/svelte/frame/target/debug/frame_lsp".to_string()
            });

        Ok(zed::Command {
            command,
            args: Vec::new(),
            env,
        })
    }
}

zed::register_extension!(FrameExtension);
