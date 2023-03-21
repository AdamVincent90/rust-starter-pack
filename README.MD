### Ultimate Rust Service

- This is mearly a personal project that aims to use the academical theory of web service application construction from Ardan Labs Ultimate Service to be applied into the Rust programming language.

- For reference to the Ardan Labs Ultimate service that is built using Golang -> https://github.com/ardanlabs/service

#### Work In progress

- This is very much early doors, lots of research needs to go into identifying the best crates and logic. Other branches will work that will be pushed into main once comfortable and tested.

#### Theory

Regardless of programming language, like constructed a building. There are 3 main stages of developing a service.
The below theory will be further discussed in detail in the future.

- The Blueprint (reading, diagramming, discussing, researching technology, documentations)
- The Scaffold (infrastucture, migrations, metadata, scripts, tooling)
- The Src (the actual rust code that underpins your scaffold and blueprints) has 3 sub layers
  - App (executables rust services, executable rust tooling, rust configuration to get a rust app ready to perform business tasks)
  - Business (logic the app layer calls to perform a business task for example such as queries to the database, running middleware, performing core business logic for a handler, business models)
  - Foundation (independant modules that have no context for the given business, for example, database connectivity, logging, and utility modules that aid your rust app and business layer logic)

#### Features this rust starter kit is bundled with

- Atlas migrations that allows making changes to your database, migrating on new docker containers, that make your migrations suitable for ephemeral containers.
- Docker development file that uses cargo watch (this is a WIP in order to decrease build times)
- Docker deployment file that can be used to push your image to a container registry, however this can be edited in a way you see fit.
- Rust binary service that acts as the main web service, will come with audit logging, middleware, auth, database functionality
- Location to add tooling and worker binaries that can be used as seperate executables
- Custom logger, database, web server modules that are ready to be used. (The only thing you would need to do is add your handler and for it to call your custom functions)
- Make commands to run migrations, spin up docker containers, and other helpers.
- In the future i want to include a tool binary that creates `core` templates and handlers with a run of a command.

Please note, the aim of this project is for it not to be a "libary" or "framework" but a starter kit for people to use that can be built upon, or customised in a way they see fit.
