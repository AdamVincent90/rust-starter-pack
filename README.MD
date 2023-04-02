### Rust Starter Pack

This project aims to be a a rust starter kit that will aid developers in making, strong, scalable and consistent web services or tools. This project is inspired by Ardan Labs Ultimate Service that is built using Go. https://github.com/ardanlabs/service

Although this project takes lots of inspiration from Ardan Labs Ultimate Service, it is import to understand that different programming languages contain different paradigms, therefore, this starter kit will contain considerations that further utilise Rust.

The Theory for Ardan Labs Ultimate Service is strongly applied to this project, that helps make this starter kit robust. Though language may have difference in opinion, and syntax, the theory is agnostic, which is what makes programming in different languages fun!

#### Quick Start

Prequisites require that you have `rustc` and `cargo` installed on your machine, as well as `docker`.

1. Clone the repo
2. Cargo Install
3. run `make run-dev`

This should build a docker network consisting of your rust container running a debug, and web server. A postgres Container for storing your data, and a postgres migrate container that will perform database migrations. The initial migrations will create a `users` table with one user record.

#### Work in Porgress

This project is early in development, and will contain a steady stream of changes going forward, both in terms of Rust knowledge (as I get more accumstomed to the language), research, and overall fixes and design decisions.

#### Theory

Regardless of programming language, like constructed a building. There are 3 main stages of developing a service.
The below theory will be further discussed in detail in the future.

- The Blueprint (reading material, diagrams, research, documentations, collections)
- The Scaffold (infrastucture, migrations, metadata, scripts, agnostic tooling)
- The Src (the actual rust code that underpins your scaffold and blueprints) has 3 sub layers
  1. App (executables rust services, executable rust tooling, rust configuration to get a rust app ready to perform business tasks)
  2. Business (logic the app layer calls to perform a business task for example such as queries to the database, running middleware, performing core business logic for a handler, business models)
  3. Foundation (independant modules that have no context for the given business, for example, database connectivity, logging, and utility modules that aid your rust app and business layer logic)

#### Other Features this rust starter kit will be bundled with.

- Atlas migrations: Allows making changes to your database, keeping a source of truth for your database, and allowing this project to support multiple developers working on the same dataset.

- Docker support: The make commands, development, and deployment files, to quickly spin up your docker network, or to build your image.

- Grpc Support: All configurations and setup you need to create client and server stubs, designed for the purpose of 2 rust starter kits communicating or other grpc services.

- Custom tool: Lumber. This will create Store and Core functions on the fly, so less boilerplate to write!

- Custom worker: That will generate a report for tracking activity.

### Lumber (WIP: commands are not implemented but the end goal)

Make a core entity: `make lumber core <name> <[modules]>`
Example on a core entity: `make lumber core user store-db client-grpc client-http` (supported modules will be listed here)

Make a store entity `make lumber store <name> <[options]>`
Example on a store entity: `make lumber store user db db-cache gcs` (supported modules will be listed here)

##### Example of how lumber will work


https://user-images.githubusercontent.com/37915853/229356323-57768d61-b9fe-471c-8e89-aa341d9810d1.mp4


More to follow.

#### Feedback

Please note, the aim of this project is for it not to be a "libary" or "framework" but a starter kit for people to use that can be built upon, or customised in a way they see fit.

Any feedback is greatly appreciated.
