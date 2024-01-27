use crate::messages;
use crate::parser::Argument;
use crate::parser::ArgumentPackage;

pub fn update_flags() -> Vec<Argument> {
    let args: Vec<Argument> = vec![
        Argument::path(),
        Argument::destinations(),
        Argument::repository(),
    ];

    args
}

pub fn update_repo<'a>(args: ArgumentPackage) -> Result<&'a str, &'a str> {
    Err(messages::IMPOSSIBLE_ERROR_MESSAGE)
}
