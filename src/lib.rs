use language_server::{slang::Slang, LanguageServer};

use zed::{LanguageServerId, Worktree};
use zed_extension_api::{self as zed};

pub mod language_server;

struct VerilogExtension {
    slang: Slang,
}

impl zed::Extension for VerilogExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            slang: Default::default(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        match language_server_id.as_ref() {
            Slang::LANGUAGE_SERVER_ID => Ok(zed::Command {
                command: self.slang.get_binary(language_server_id, worktree)?,
                args: Vec::new(),
                env: Vec::new(),
            }),
            id => Err(format!("unknown language server `{id}`"))?,
        }
    }
}

zed::register_extension!(VerilogExtension);
