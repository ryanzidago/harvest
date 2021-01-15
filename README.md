# harvest

```
harvest
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
```
