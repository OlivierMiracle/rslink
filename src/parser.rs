use crate::flags;
use crate::messages;

pub trait Parsable {
    fn parse(&self);
}

#[derive(Clone)]
pub struct Argument {
    pub short_flag: &'static str,
    pub long_flag: &'static str,
    pub is_required: bool,
    pub arg_value: ArgumentValue,
    pub arg_type: ArgumentType,
}

impl Argument {
    pub fn path() -> Argument {
        Argument {
            short_flag: flags::PATH_SHORT_FLAG,
            long_flag: flags::PATH_LONG_FLAG,
            is_required: false,
            arg_value: ArgumentValue::String(None),
            arg_type: ArgumentType::Path,
        }
    }
    pub fn file() -> Argument {
        Argument {
            short_flag: flags::FILE_SHORT_FLAG,
            long_flag: flags::FILE_LONG_FLAG,
            is_required: true,
            arg_value: ArgumentValue::String(None),
            arg_type: ArgumentType::File,
        }
    }
    pub fn forced() -> Argument {
        Argument {
            short_flag: flags::FORCED_SHORT_FLAG,
            long_flag: flags::FORCED_LONG_FLAG,
            is_required: false,
            arg_value: ArgumentValue::Boolean(None),
            arg_type: ArgumentType::Forced,
        }
    }
    pub fn all() -> Argument {
        Argument {
            short_flag: flags::ALL_SHORT_FLAG,
            long_flag: flags::ALL_LONG_FLAG,
            is_required: false,
            arg_value: ArgumentValue::Boolean(None),
            arg_type: ArgumentType::All,
        }
    }
    pub fn destinations() -> Argument {
        Argument {
            short_flag: flags::DESTINATIONS_SHORT_FLAG,
            long_flag: flags::DESTINATIONS_LONG_FLAG,
            is_required: false,
            arg_value: ArgumentValue::Boolean(None),
            arg_type: ArgumentType::Destinations,
        }
    }
    pub fn repository() -> Argument {
        Argument {
            short_flag: flags::REPOSITORY_SHORT_FLAG,
            long_flag: flags::REPOSITORY_LONG_FLAG,
            is_required: false,
            arg_value: ArgumentValue::Boolean(None),
            arg_type: ArgumentType::Repository,
        }
    }
}

#[derive(Clone)]
pub enum ArgumentType {
    Path,
    File,
    Forced,
    All,
    Destinations,
    Repository,
}

#[derive(Clone)]
pub enum ArgumentValue {
    Boolean(Option<bool>),
    String(Option<String>),
}

pub fn parse_arguments(
    args: Vec<String>,
    flags: Vec<Argument>,
) -> Result<Vec<Argument>, &'static str> {
    let mut parsed_args: Vec<Argument> = flags.clone();
    let c_max = &args.len();

    for flag in &mut parsed_args {
        loop {
            let mut c: usize = 2usize;
            if &c >= c_max {
                break;
            }

            let arg = &args[c];

            if arg == flag.short_flag || arg == flag.long_flag {
                match flag.arg_value {
                    ArgumentValue::String(_) => {
                        flag.arg_value = ArgumentValue::String(Some(args[c + 1].clone()));
                        c += 1usize;
                    }
                    ArgumentValue::Boolean(_) => {
                        flag.arg_value = ArgumentValue::Boolean(Some(true))
                    }
                }
            } else if flag.is_required {
                return Err(messages::LACKING_PARAMETER_MESSAGE);
            } else {
                match flag.arg_value {
                    ArgumentValue::Boolean(_) => flag.arg_value = ArgumentValue::Boolean(None),
                    ArgumentValue::String(_) => flag.arg_value = ArgumentValue::String(None),
                }
            }

            c += 1usize;
        }
    }
    Ok(parsed_args)
}
