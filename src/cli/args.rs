use clap::{Arg, ArgAction};

pub fn path() -> Arg {
    Arg::new("path").num_args(0..=1).action(ArgAction::Set)
}

pub fn name() -> Arg {
    Arg::new("name").num_args(0..).action(ArgAction::Set)
}

pub fn pic() -> Arg {
    Arg::new("pic").long("pic").action(ArgAction::Set)
}

pub fn groups() -> Arg {
    Arg::new("groups")
        .short('g')
        .long("groups")
        .action(ArgAction::Set)
}

pub fn no_groups() -> Arg {
    Arg::new("no-groups")
        .long("no-groups")
        .action(ArgAction::Set)
}

pub fn phones() -> Arg {
    Arg::new("phones")
        .short('p')
        .long("phones")
        .action(ArgAction::Set)
}

pub fn no_phones() -> Arg {
    Arg::new("no-phones")
        .long("no-phones")
        .action(ArgAction::Set)
}

pub fn emails() -> Arg {
    Arg::new("emails")
        .short('e')
        .long("emails")
        .action(ArgAction::Set)
}

pub fn no_emails() -> Arg {
    Arg::new("no-emails")
        .long("no-emails")
        .action(ArgAction::Set)
}

pub fn social_nets() -> Arg {
    Arg::new("social_nets")
        .short('s')
        .long("social-nets")
        .action(ArgAction::Set)
}

pub fn no_social_nets() -> Arg {
    Arg::new("no-social_nets")
        .long("no-social-nets")
        .action(ArgAction::Set)
}

pub fn wallets() -> Arg {
    Arg::new("wallets")
        .short('w')
        .long("wallets")
        .action(ArgAction::Set)
}

pub fn no_wallets() -> Arg {
    Arg::new("no-wallets")
        .long("no-wallets")
        .action(ArgAction::Set)
}

pub fn notes() -> Arg {
    Arg::new("notes")
        .short('n')
        .long("notes")
        .action(ArgAction::Set)
}
