# restful-climbing
Toy REST API for getting information about different climbs and recording climbs you've completed.


## API Endpoints
- /routes
    - ✔️ GET: List 5 most recently added routes
        - ✔️ Add optional arguments to query body to specify # routes
    - ✔️ POST: Add a new route
- /routes/{route_id}
    - GET: Find a route by its ID
    - DELETE: Delete a route
    - PUT: Update information about the climb
- /climbers
    - GET: Return 10 most recently added climbers
    - POST: Create a new climber
- /climbers/{climber_id}
    - GET: Return 10 most recently completed climbs
- /climbers/{climber_id}/{route_id}
    - GET: Get the climber's review of the given route
    - POST: Add a new review for the given climber/route
    - PUT: Update the review for the given climber/route
    - DELETE: Delete the review of the given climber/route

## Notes
### General plan:
- ✔️ [Make Amazon RDS PostgreSQL instance](https://aws.amazon.com/rds/postgresql/pricing/)
- Write the API with:
    - [actix-web](https://actix.rs/docs/getting-started)
    - [sqlx](https://crates.io/crates/sqlx) with `runtime-tokio-rustls` feature flag
- Containerize the application using docker
- Deploy the application on AWS Fargate

### Links to articles, tutorials, etc
- [Containerizing Rust](https://www.fpcomplete.com/blog/2018/07/deploying-rust-with-docker-and-kubernetes/) - everything up to, but not including, the section "Deploying our Docker image with Kubernetes"
- [Deploying Docker container to AWS Fargate](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/Welcome.html)

### Twelve-Factor App
Some brief notes about how I'll try to adhere to twelve-factor app design
1. Codebase: Github
2. Dependencies: Cargo
3. Config: Use dotenv if I find I need some configuration is needed, but would rather avoid it
    - Avoid `$` in values stored in `.env`; had to escape with a `\`
4. Backing services: The postgres database we'll host through AWS is an attached resource. Code should be written in a way that it could be moved to another database (i.e. self-hosted server or [Google cloud](https://cloud.google.com/sql/docs/postgres/quickstarts))
5. Build release run: The tutorial for containerizing rust uses a build stage and run stage in the dockerfile. Not sure if this is separated enough for twelve-factor app standards.
6. Processes: The API will be stateless. State will be stored in the postgres databse.
7. Port binding: REST API follows this by definition?
8. Concurrency: `actix-web` handles this by using an Application factory pattern.
9. Disposability: `actix-web` handles this by enabling graceful shutdown and responding to OS signals for shutdown.
10. Dev/prod parity: Docker will standardize the environment for the app on my local PC and on AWS Fargate; use postgres for dbms for both.
11. Logs: 
    - [Logging with actix-web](https://actix.rs/docs/middleware#logging)
    - [Logging with AWS Fargate and `awslogs`](https://docs.aws.amazon.com/prescriptive-guidance/latest/implementing-logging-monitoring-cloudwatch/ec2-fargate-logs.html)
    - [`env_logger`](https://docs.rs/env_logger/0.10.0/env_logger/) - configure to write to `stdout`
12. Admin processes: I don't think I need to worry about this for this project, unless/until I want to migrate databases. Maybe I should try that just for practice.
    - **Ok, this came up way earlier than I thought.** I should make the database schema using a database migration tool, and use git for version control of the database schema files. I'll try Liquibase. And I should use a common [`dotenv`](https://docs.rs/dotenv/0.15.0/dotenv/) for my database connection parameters, for both the migration tool, and my application code. 
        - `dotenv` is abandoned, using `dotenvy` instead.
        - using `dotenvy_macro` for compile time checking, neat
    - Liquibase doesn't work with a `.env` file so I have my `liquibase.properties` and `.env` files keeping track of some duplicate info. 