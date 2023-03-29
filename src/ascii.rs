use colored::Colorize;

pub fn get_header() -> String {
    r"
    ___ ___  _  _ _  _ ___ ___ _____ ___ ___  
   / __/ _ \| \| | \| | __/ __|_   _| __|   \ 
  | (_| (_) | .` | .` | _| (__  | | | _|| |) |
   \___\___/|_|\_|_|\_|___\___| |_| |___|___/ 
                                              
 "
    .bright_green()
    .bold()
    .to_string()
}

pub fn get_summary() -> String {
    "Type 'help' or 'help <executor>' for more information\n\n"
        .dimmed()
        .bold()
        .to_string()
}
