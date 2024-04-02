//! Sort the papers according to their title, reading progress, etc.
// use std::path::PathBuf;
// use std::process::Command;

use std::collections::BTreeSet;

use crate::cmd::prelude::*;
use crate::paper::{PaperList, ReadingProgress};

pub static MAN: &str = include_str!("../../man/sort.md");

pub fn execute(
    input: CommandInput,
    state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    if input.papers.is_none() {
        return Err(Fallacy::SetNoPapers);
    }

    let papers = input.papers.unwrap().0; // list of paper index
    let mut sorted = Vec::new();

    if input.args.len() == 1 {
        // Sort by title
        let mut keep = BTreeSet::new();
        for id in papers {
            keep.insert((&state.papers[id].title, id));
        }

        for (_, id) in keep {
            sorted.push(id);
        }
    } else if input.args.len() == 3 {
        match input.args[1].as_str() {
            "by" => {
                let status = input.args[2].parse::<ReadingProgress>().unwrap();
                // Sort by reading status
                for id in papers {
                    if state.papers[id].progress == status {
                        sorted.push(id);
                    }
                }
            }
            _ => return Ok(CommandOutput::None),
        }
    }

    Ok(CommandOutput::Papers(PaperList(sorted)))
}
