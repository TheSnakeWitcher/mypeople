use clap::{Arg, ArgAction};

pub fn path() -> Arg {
    Arg::new("path").num_args(0..=1).action(ArgAction::Set)
}

pub fn name() -> Arg {
    Arg::new("name").num_args(0..).action(ArgAction::Set)
}

pub fn pic(action: ArgAction) -> Arg {
    Arg::new("pic").long("pic").action(action)
}

pub fn groups(action: ArgAction) -> Arg {
    Arg::new("groups")
        .short('g')
        .long("groups")
        .action(action)
}

pub fn phones(action: ArgAction) -> Arg {
    Arg::new("phones")
        .short('p')
        .long("phones")
        .action(action)
}

pub fn emails(action: ArgAction) -> Arg {
    Arg::new("emails")
        .short('e')
        .long("emails")
        .action(action)
}

pub fn social_nets(action: ArgAction) -> Arg {
    Arg::new("social_nets")
        .short('s')
        .long("social-nets")
        .action(action)
}

pub fn wallets(action: ArgAction) -> Arg {
    Arg::new("wallets")
        .short('w')
        .long("wallets")
        .action(action)
}

pub fn notes(action: ArgAction) -> Arg {
    Arg::new("notes")
        .short('n')
        .long("notes")
        .action(action)
}
