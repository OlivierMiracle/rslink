use crate::flags;
use crate::messages;
use crate::parser::Argument;
use crate::parser::ArgumentType;
use crate::repo;
use crate::Command;

pub fn update_parse(args: Vec<String>) -> Command {
    let mut args: Vec<Argument> = vec![
        Argument::path(),
        Argument::destinations(),
        Argument::repository(),
    ];
    Command::Update(args)
}

pub fn update_repo(args: Vec<Argument>) {
    let path: String;
    let repository: bool;
    let destinations: bool;

    for arg in args {
        match arg.arg_type {
            ArgumentType::Path => path = arg.arg_value,
        }
    }
}
