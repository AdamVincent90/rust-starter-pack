### Application Layer

Where your bins sit, designed to contain the application configuration, and setup tasks for your business level logic. Bins here can contain either tooling executables to perform jobs for context related tasks, or server executables that load up and configure web (rpc, rest, graphql) services.

(main.rs right now is the entrypoint for start up and shut down of a web service, this should be moved to app, but research needs to be done for best practices in rust)
