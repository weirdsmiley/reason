//! Provides an integration with VimWiki by creating a new wiki index in the
//! default paperbase path and opening all wikis with vimwiki filetypes.
use std::path::PathBuf;
use std::process::Command;

use crate::cmd::prelude::*;
use crate::utils::confirm;

pub static MAN: &str = include_str!("../../man/vimwiki.md");

pub fn execute(
    input: CommandInput,
    state: &mut State,
    config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Build paper list from input.
    let selected = match input.papers {
        // Papers are given through pipe.
        Some(list) => list.0,
        // Papers are specified as filter.
        None => {
            match crate::cmd::ls::execute(input, state, config)? {
                CommandOutput::Papers(paper_list) => paper_list.0,
                // `ls` always returns CommandOutput::Papers.
                _ => panic!("internal ls invocation returned wrong output variant"),
            }
        }
    };

    // Build a vector of wiki paths.
    let num_papers = selected.len();
    let mut wikis = Vec::new();
    for i in selected {
        wikis.push(state.papers[i].wikipath(config, true)?.unwrap());
    }

    // Ask for confirmation.
    if num_papers > 1 {
        confirm(format!("Open wikis for {} paper?", num_papers), true)?;
    }

    // Open wikis.
    if config.output.editor_batch {
        spawn(build_editor_command(wikis.as_ref(), config), true);
    } else {
        for wiki in wikis {
            spawn(build_editor_command(&[wiki], config), false);
        }
    }

    Ok(CommandOutput::None)
}

fn spawn(mut command: Command, block: bool) {
    match command.spawn() {
        Ok(mut handle) => {
            if !block {
                return;
            }
            if let Err(e) = handle.wait() {
                println!("Failed to wait subprocess: {}", e);
            }
        }
        Err(e) => {
            if matches!(e.kind(), std::io::ErrorKind::NotFound) {
                println!("Invalid editor command: '{:?}'", e);
            } else {
                println!("Failed to spawn subprocess: '{:?}'", e);
            }
        }
    }
}

fn build_editor_command(wikis: &[PathBuf], config: &Config) -> Command {
    let command = &config.output.editor_command;
    let mut ret = Command::new(&command[0]);
    ret.args(&command[1..]).args(wikis);

    let vimwiki_opts = vec![
        String::from("-c"),
        String::from(format!(
            "\"call add(g:vimwiki_list, {{'path': \'{}\'}})\"",
            &config.storage.wiki_dir.to_str().unwrap()
        )),
    ];

    ret.args(&vimwiki_opts[0..]).args(wikis);
    ret
}
