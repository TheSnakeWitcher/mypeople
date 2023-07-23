use clap::ArgMatches;

pub fn get_names(sub_matches: &ArgMatches) -> Vec<&String> {
    let names = sub_matches
        .get_many::<String>("name")
        .into_iter()
        .flatten()
        .collect::<Vec<&String>>();
    return names;
}
