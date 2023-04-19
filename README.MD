### Rust Starter Pack (Alpha)

WIP - Please note this is not final and is very much a work in progress, with lots of things to clean and implement.

This project aims to be a a rust starter kit that will aid developers in making, strong, scalable and consistent web services or tools. This project is inspired by Ardan Labs Ultimate Service that is built using Go. https://github.com/ardanlabs/service

Although this project takes lots of inspiration from Ardan Labs Ultimate Service, it is import to understand that different programming languages contain different paradigms, therefore, this starter kit will contain considerations that further utilise Rust.

The Theory for Ardan Labs Ultimate Service is strongly applied to this project, that helps make this starter kit robust. Though language may have difference in opinion, and syntax, the theory is agnostic, which is what makes programming in different languages fun!

### Service

Although this starterpack will come with some tools, pre-made libraries, and will already be a good kit for non service related projects. This project will come with two example services (ready to be modified, or built on), either for them to be used together, or seperately. These are:

1. External API - A standard rest API powered by Axum, with middleware, error handling, auth, validation, database, state, and grpc client support. The external API service acts as the "front facing" service where users make requests and then receive a response.

2. Internal API - A backend to backend API service that uses GRPC Tonic, this will also support middleware, error handling and validation, database etc. This Internal API is more catered towards receiving requests and returning responses to another internal API.

With how this project is built, The core modules within `core` can be used in an agnostic way, regardless of API implementation used. Core functions will also come with grpc client support, with the aim that it can be code genned using lumber.

#### Quick Start

Prequisites require that you have `rustc` and `cargo` installed on your machine, as well as `docker`.

1. Clone the repo
2. Cargo Install
3. run `make run-dev`

This should build a docker network consisting of your rust container running a debug, and web server. A postgres Container for storing your data, and a postgres migrate container that will perform database migrations. The initial migrations will create a `users` table with one user record.

##### Making a RSA key pair

`make rsa-keypair`

#### Work in Porgress

This project is early in development, and will contain a steady stream of changes going forward, both in terms of Rust knowledge (as I get more accumstomed to the language), research, and overall fixes and design decisions.

#### Theory

Regardless of programming language, like constructed a building. There are 3 main stages of developing a service.
The below theory will be further discussed in detail in the future.

- The Blueprint (reading material, diagrams, research, documentations, collections)
- The Scaffold (infrastucture, migrations, metadata, scripts, agnostic tooling)
- The Src (the actual rust code that underpins your scaffold and blueprints) has 3 sub layers
  1. App (executables rust services, executable rust tooling, rust configuration to get a rust app ready to perform business tasks). This layer can also contain modules that are exclusive to this app, for example lumber uses templates that only lumber will used. This is a good use case to use as a submodule over a domain module.
  2. Core (contains all your self contained modules to successfully perform a business task, this is where the main logic for your project is fired. This can include things such as Creating a user, looking up a user, etc) The main logic and entrypoint that your app layer binary runs.
  3. Domain contains all your business/project specific modules, these can be things like web middleware and shared state, or system validation, errors and auth.
  4. Lib (independant modules that have no context for the given business, for example, database connectivity, logging, and utility modules that aid your rust app and business layer logic)

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

https://user-images.githubusercontent.com/37915853/230621229-670591aa-1ccb-494c-80e0-ed4f1fef0ec0.mp4

More to follow.

#### Feedback

Please note, the aim of this project is for it not to be a "libary" or "framework" but a starter kit for people to use that can be built upon, or customised in a way they see fit.

Any feedback is greatly appreciated.
