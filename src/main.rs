use std::env;
use std::process::Command;
use std::process::exit;

fn main() {
    let argv: Vec<String> = env::args().collect();

    maybe_display_help(&argv[1]);
    maybe_display_error_message();

    let app_name: String = parse_arg(&argv[1], &String::from("--app="));
    let env_name: String = parse_arg(&argv[2], &String::from("--env="));
    let local_database_name: String = parse_arg(&argv[3], &String::from("--local-db="));

    let app_name_with_env: String = build_app_name_with_env(&app_name, &env_name);
    let database_url: String = get_database_url(&app_name_with_env);

    set_database_url_env_var(database_url);
    pull_from_remote_database(&app_name_with_env, &local_database_name);
    unset_database_url_env_var();
}

fn maybe_display_help(arg: &String) {
    if arg.starts_with("--help") {
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

    The program will fetch the app's database URL with the `heroku config:get DATABASE_URL --app myapp-staging` command.
    Then, it sets the environment variable DATABASE_URL on the local computer and run the command `heroku pg:pull DATABASE_URL myapp_dev --app myapp-staging`.
    Finally, it unsets the DATABASE_URL environment variable.
";

        println!("{}", help);
        exit(0);
    }
}

fn maybe_display_error_message() {
    let key = "DATABASE_URL";
    match env::var(key) {
        Ok(val) => if !val.is_empty() {
            eprintln!("DATABASE_URL is already set.\nUnset DATABASE_URL environment variable before continuing.\n(hint: run `unset DATABASE_URL`)");
            exit(-1);
        }
        Err(e) => {
            eprintln!("Couldn't interpret {}: {}", key, e);
            exit(-1);
        }
    }
}

fn parse_arg(arg: &String, pattern: &String) -> String {
    arg.replace(pattern, "")
}

fn build_app_name_with_env<'a>(app_name: &'a String, env_name: &'a String) -> String {
    format!("{}-{}", app_name, env_name)
}

fn get_database_url(app_name_with_env: &String) -> String {
    let output = Command::new("heroku")
                    .arg("config:get")
                    .arg("DATABASE_URL")
                    .arg("--app")
                    .arg(app_name_with_env)
                    .output()
                    .expect("Failed to execute command.");

    let mut database_url: String = String::from_utf8(output.stdout).unwrap();
    database_url.pop();
    database_url
}

fn set_database_url_env_var(database_url: String) {
    let key = "DATABASE_URL";
    env::set_var(key, database_url);
}

fn pull_from_remote_database<'a>(app_name_with_env: &'a String, local_database_name: &'a String) {
    let output = Command::new("heroku")
                    .arg("pg:pull")
                    .arg("DATABASE_URL")
                    .arg(local_database_name)
                    .arg("--app")
                    .arg(app_name_with_env)
                    .output()
                    .expect("Failed to execute command.");

    println!("{}", String::from_utf8(output.stdout).unwrap());
}

fn unset_database_url_env_var() {
    env::remove_var("DATABASE_URL");
}
