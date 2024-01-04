pub const HELP_MESSAGE: &str = r" ________   ________   ___        ___   ________    ___  __       
|\   __  \ |\   ____\ |\  \      |\  \ |\   ___  \ |\  \|\  \     
\ \  \|\  \\ \  \___|_\ \  \     \ \  \\ \  \\ \  \\ \  \/  /|_   
 \ \   _  _\\ \_____  \\ \  \     \ \  \\ \  \\ \  \\ \   ___  \  
  \ \  \\  \|\|____|\  \\ \  \____ \ \  \\ \  \\ \  \\ \  \\ \  \ 
   \ \__\\ _\  ____\_\  \\ \_______\\ \__\\ \__\\ \__\\ \__\\ \__\
    \|__|\|__||\_________\\|_______| \|__| \|__| \|__| \|__| \|__|
              \|_________|                                        
                                                                  

rslink - a friendly helper for linking multiple files within directory.

Version: 0.0.1

Usage:
    rslink [command] [flags]

Available commands:
    (blank), help           - d``isplays this help message
    create                  - creates new repo for linking
        -f | --force        - creates new repo even in place of older one
        -i | --initialize   - adds files inside repository to tracking
        -p | --path [path]  - path to repository. Your location as default
    delete                  - remove repo from location
        -f | --force        - force deletion, even when its no repository
        -p | --path [path]  - once again, just a path
    status                  - lists detected changes; addition and deletion of files

    link                    - Commands involved with creating a link
    update                  - Updates the repository and destinations
        -d | --destination  - Updates destinations only
        -r | --repository   _ Updated tracked files in repository
";

pub const UNKNOWN_COMMAND_MESSAGE: &str =
    "Unknown command. Please refer to 'rslink help' for further guidance.";

pub const INVALID_PARAMETER_MESSAGE: &str =
    "Invalid parameter. Please check your spelling or sanity. Thanks :3";

pub const REPO_ALREADY_EXISTS_MESSAGE: &str =
    "You are trying to create a repo where it already exists. Delete it or force creation of new one.";

pub const REPO_DOES_NOT_EXIST_MESSAGE: &str =
    "Repo does not exist at that location. Try your luck somewhere else :D";
