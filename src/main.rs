use std::env;
use std::process::exit;
use std::process::Command;

fn main() {
    let argv: Vec<String> = env::args().collect();

    maybe_display_help(&argv);

    let app_name: String = parse_arg(&argv[1], &String::from("--app="));
    let env_name: String = parse_arg(&argv[2], &String::from("--env="));
    let local_database_name: String = parse_arg(&argv[3], &String::from("--local-db="));

    let heroku_app_name: String = build_heroku_app_name(&app_name, &env_name);
    let remote_database_name = get_remote_database_name(&heroku_app_name);
    pull_from_remote_database(
        &heroku_app_name,
        &remote_database_name,
        &local_database_name,
    );
}

fn maybe_display_help(argv: &Vec<String>) {
    if argv.len() == 1 {
        println!("harvest: try 'harvest --help' for more information");
        exit(0);
    }

    if argv[1].starts_with("--help") {
        let help = "harvest
Small command line program to pull data from a database hosted on Heroku.
Requires the heroku CLI to be installed.

USAGE:
    harvest [args]

ARGS:
    --app=<app>
    --env=<env>
    --local-db=<local database name>

EXAMPLE:
    harvest --app=myapp --env=staging --local-db-=myapp_dev

    Fetches the app's database name from Heroku and then run the `heroku pg:pull <remote database name> <local database name> --app <heroku app name>` command.
    See https://devcenter.heroku.com/articles/heroku-postgresql for more information on the `heroku pg:pull` command.
";

        println!("{}", help);
        exit(0);
    }
}

fn parse_arg(arg: &String, pattern: &String) -> String {
    arg.replace(pattern, "")
}

fn build_heroku_app_name<'a>(app_name: &'a String, env_name: &'a String) -> String {
    format!("{}-{}", app_name, env_name)
}

fn get_remote_database_name(heroku_app_name: &String) -> String {
    let output = Command::new("heroku")
        .arg("pg:info")
        .arg("--app")
        .arg(heroku_app_name)
        .output()
        .unwrap()
        .stdout;

    let output = String::from_utf8(output).unwrap();

    output.rsplit(" ")
        .collect::<Vec<&str>>()[0]
        .trim_end()
        .to_string()
}

fn pull_from_remote_database<'a>(
    heroku_app_name: &'a String,
    remote_database_name: &'a String,
    local_database_name: &'a String,
) {
    let output = Command::new("heroku")
        .arg("pg:pull")
        .arg(remote_database_name)
        .arg(local_database_name)
        .arg("--app")
        .arg(heroku_app_name)
        .output()
        .expect("Failed to execute command.");

    println!("{}", String::from_utf8(output.stdout).unwrap());
}
