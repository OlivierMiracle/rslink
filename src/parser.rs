use crate::flags;
use crate::messages;

pub trait Parsable {
    fn parse(&self);
}

#[derive(Debug, Default)]
pub struct ArgumentPackage {
    pub path: Option<String>,
    pub file: Option<String>,

    pub forced: bool,
    pub all: bool,
    pub destinations: bool,
    pub repository: bool,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub short_flag: &'static str,
    pub long_flag: &'static str,
    pub is_required: bool,
    pub arg_value: ArgumentValue,
    pub arg_type: ArgumentType,
}

impl Argument {
    pub fn unpack_string_value<'a>(self) -> Result<Option<String>, &'a str> {
        match self.arg_value {
            ArgumentValue::String(value) => match value {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            ArgumentValue::Boolean(_) => Err(messages::IMPOSSIBLE_ERROR_MESSAGE),
        }
    }
    pub fn unpack_bool_value<'a>(self) -> Result<bool, &'a str> {
        match self.arg_value {
            ArgumentValue::Boolean(value) => match value {
                Some(value) => Ok(value),
                None => Err(messages::LACKING_PARAMETER_MESSAGE),
            },
            ArgumentValue::String(_) => Err(messages::IMPOSSIBLE_ERROR_MESSAGE),
        }
    }
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

#[derive(Debug, Clone)]
pub enum ArgumentType {
    Path,
    File,
    Forced,
    All,
    Destinations,
    Repository,
}

#[derive(Debug, Clone)]
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

    'flags_loop: for flag in &mut parsed_args {
        let mut c: usize = 2usize;

        'args_loop: loop {
            if &c >= c_max {
                break 'args_loop;
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
                continue 'flags_loop;
            }
            c += 1usize;
        }

        if flag.is_required {
            return Err(messages::LACKING_PARAMETER_MESSAGE);
        } else {
            match flag.arg_value {
                ArgumentValue::Boolean(_) => flag.arg_value = ArgumentValue::Boolean(Some(false)),
                ArgumentValue::String(_) => flag.arg_value = ArgumentValue::String(None),
            }
        }
    }
    Ok(parsed_args)
}

pub fn assign_args<'a>(args: Vec<Argument>) -> Result<ArgumentPackage, &'a str> {
    let mut args_pack: ArgumentPackage = ArgumentPackage {
        ..Default::default()
    };
    for arg in args {
        match arg.arg_type {
            ArgumentType::Path => match arg.unpack_string_value() {
                Ok(value) => args_pack.path = value,
                Err(err) => return Err(err),
            },
            ArgumentType::File => match arg.unpack_string_value() {
                Ok(value) => args_pack.file = value,
                Err(err) => return Err(err),
            },
            ArgumentType::Forced => match arg.unpack_bool_value() {
                Ok(value) => args_pack.forced = value,
                Err(err) => return Err(err),
            },
            ArgumentType::All => match arg.unpack_bool_value() {
                Ok(value) => args_pack.all = value,
                Err(err) => return Err(err),
            },
            ArgumentType::Destinations => match arg.unpack_bool_value() {
                Ok(value) => args_pack.destinations = value,
                Err(err) => return Err(err),
            },
            ArgumentType::Repository => match arg.unpack_bool_value() {
                Ok(value) => args_pack.repository = value,
                Err(err) => return Err(err),
            },
        };
    }
    Ok(args_pack)
}
