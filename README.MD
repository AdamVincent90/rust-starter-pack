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
