use clap::{Arg, ArgAction};

pub fn name() -> Arg {
    Arg::new("name")
        .num_args(0..)
        .action(ArgAction::Set)
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
    Arg::new("phone")
        .short('p')
        .long("phone")
        .action(ArgAction::Set)
}

pub fn no_phones() -> Arg {
    Arg::new("no-phone").long("no-phone").action(ArgAction::Set)
}

pub fn email() -> Arg {
    Arg::new("email")
        .short('e')
        .long("email")
        .action(ArgAction::Set)
}

pub fn no_email() -> Arg {
    Arg::new("no-email").long("no-email").action(ArgAction::Set)
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
