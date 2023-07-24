use crate::cmd::prelude::*;
use crate::paper::ReadingProgress;
use crate::utils::confirm;

pub static MAN: &str = include_str!("../../man/mark.md");

/// Mark an article as finished and highlight it as Red.
pub mod mark {
    use super::*;

    pub fn execute(
        input: CommandInput,
        state: &mut State,
        config: &Config,
    ) -> Result<CommandOutput, Fallacy> {
        let paper_list = match input.papers {
            // Papers are given through pipe.
            Some(list) => list,
            // Papers are specified as filter.
            None => {
                match crate::cmd::ls::execute(input, state, config)? {
                    CommandOutput::Papers(paper_list) => paper_list,
                    // `ls` always returns CommandOutput::Papers.
                    _ => panic!(),
                }
            }
        };

        // Ask for confirmation.
        let num_paper = paper_list.0.len();
        if num_paper > 1 {
            confirm(format!("Mark {} papers as read?", num_paper), false)?;
        }

        for ind in paper_list.0 {
            state.papers[ind].progress = ReadingProgress::Read;
        }

        Ok(CommandOutput::Message(format!(
            "Marked {} {} as finished.\n",
            num_paper,
            if num_paper != 1 { "papers" } else { "paper" },
        )))
    }
}

/// Mark papers as unread.
pub mod unmark {
    use super::*;

    pub fn execute(
        input: CommandInput,
        state: &mut State,
        config: &Config,
    ) -> Result<CommandOutput, Fallacy> {
        let paper_list = match input.papers {
            // Papers are given through pipe.
            Some(list) => list,
            // Papers are specified as filter.
            None => {
                match crate::cmd::ls::execute(input, state, config)? {
                    CommandOutput::Papers(paper_list) => paper_list,
                    // `ls` always returns CommandOutput::Papers.
                    _ => panic!(),
                }
            }
        };

        // Ask for confirmation.
        let num_paper = paper_list.0.len();
        if num_paper > 1 {
            confirm(format!("Mark {} papers as unread?", num_paper), false)?;
        }

        for ind in paper_list.0 {
            state.papers[ind].progress = ReadingProgress::Unread;
        }

        Ok(CommandOutput::Message(format!(
            "Marked {} {} as unread.\n",
            num_paper,
            if num_paper != 1 { "papers" } else { "paper" },
        )))
    }
}

/// Mark papers as currently reading.
pub mod current {
    use super::*;

    pub fn execute(
        input: CommandInput,
        state: &mut State,
        config: &Config,
    ) -> Result<CommandOutput, Fallacy> {
        let paper_list = match input.papers {
            // Papers are given through pipe.
            Some(list) => list,
            // Papers are specified as filter.
            None => {
                match crate::cmd::ls::execute(input, state, config)? {
                    CommandOutput::Papers(paper_list) => paper_list,
                    // `ls` always returns CommandOutput::Papers.
                    _ => panic!(),
                }
            }
        };

        // Ask for confirmation.
        let num_paper = paper_list.0.len();
        if num_paper > 1 {
            confirm(
                format!("Mark {} papers as currently reading?", num_paper),
                false,
            )?;
        }

        for ind in paper_list.0 {
            state.papers[ind].progress = ReadingProgress::InProgress;
        }

        Ok(CommandOutput::Message(format!(
            "Marked {} {} as currently reading.\n",
            num_paper,
            if num_paper != 1 { "papers" } else { "paper" },
        )))
    }
}
