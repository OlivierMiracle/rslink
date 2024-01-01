pub const HELP_MESSAGE: &str = r" ________   ________   ___        ___   ________    ___  __       
|\   __  \ |\   ____\ |\  \      |\  \ |\   ___  \ |\  \|\  \     
\ \  \|\  \\ \  \___|_\ \  \     \ \  \\ \  \\ \  \\ \  \/  /|_   
 \ \   _  _\\ \_____  \\ \  \     \ \  \\ \  \\ \  \\ \   ___  \  
  \ \  \\  \|\|____|\  \\ \  \____ \ \  \\ \  \\ \  \\ \  \\ \  \ 
   \ \__\\ _\  ____\_\  \\ \_______\\ \__\\ \__\\ \__\\ \__\\ \__\
    \|__|\|__||\_________\\|_______| \|__| \|__| \|__| \|__| \|__|
              \|_________|                                        
                                                                  

rslink - a friendly helper for linking multiple files within directory.

Usage:
    rslink [command] [flags]

Available commands:
    (blank), help    - d``isplays this help message
    create           - creates new repo for linking
    status           - lists detected changes; addition and deletion of files

    link             - Commands involved with creating a link
";

pub fn get_help_message() -> &'static str {
    HELP_MESSAGE
}
