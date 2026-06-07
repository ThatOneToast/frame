use crate::project;

pub fn init_svelte(dry_run: bool, force: bool, yes: bool) -> anyhow::Result<()> {
    project::init_svelte(dry_run, force, yes)
}

pub fn init_web(dry_run: bool, force: bool, yes: bool) -> anyhow::Result<()> {
    project::init_web(dry_run, force, yes)
}
