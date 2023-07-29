use crate::cmd::prelude::*;

pub static MAN: &str = include_str!("../../man/man.md");

pub fn execute(
    input: CommandInput,
    _state: &mut State,
    _config: &Config,
) -> Result<CommandOutput, Fallacy> {
    // Man accepts exactly one argument.
    if input.args.len() != 2 {
        return Err(Fallacy::ManInvalidArgument);
    }

    // Fetch the man string.
    let entry = input.args[1].as_ref();
    let man_str = match entry {
        "command" => crate::cmd::MAN,
        "cd" => crate::cmd::cd::MAN,
        "curl" => crate::cmd::curl::MAN,
        "ed" => crate::cmd::ed::MAN,
        "exit" => crate::cmd::exit::MAN,
        "ls" => crate::cmd::ls::MAN,
        "man" => crate::cmd::man::MAN,
        "mark" => crate::cmd::mark::MAN,
        "open" => crate::cmd::open::MAN,
        "printf" => crate::cmd::printf::MAN,
        "pwd" => crate::cmd::pwd::MAN,
        "rm" => crate::cmd::rm::MAN,
        "set" => crate::cmd::set::MAN,
        "touch" => crate::cmd::touch::MAN,
        "vimwiki" => crate::cmd::vimwiki::MAN,
        "wc" => crate::cmd::wc::MAN,
        "config" => crate::config::MAN,
        "filter" => crate::filter::MAN,
        "paper" => crate::paper::MAN,
        _ => return Err(Fallacy::ManUnknownSubject(entry.to_owned())),
    };

    // Build CommandOutput
    Ok(CommandOutput::Message(man_str.to_owned()))
}
