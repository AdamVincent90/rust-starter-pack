### Source Code

This is where your rust codes sit, in terms of creating scalable, clean, and maintable code, src consists of four primary directories (layers).

1. App

   App contains all your binaries and their configuration to start either, listening to requests, or executing jobs. An `App` can fall under three different types:

   1. Services - A service that listens or responds to requests.

   2. Tools - A job that when runs, performs an operation that automates tasks for the developer.

   3. Workers - A worker is another tool that instead performs an operation that solves a business related task, these workers can be ad-hoc, or indeed scheduled.

---

2. Core
   Core contains all your self contained modules that are directly called by your app level binaries in order to fulfil or complete a business task. For example, creating a user, updating a user, viewing a user, would all fall under a self-contained core module. Core modules should also contain sub-modules such as a store or client that helps fulfil the business task.

---

3. Domain
   Domain contains the logic and business domain level modules (modules that are specifically targetted for this project). Modules such as auth, middleware, validation, error handling are good examples of modules that fall in this category Logic and code that is specifically targetted towards your project.

---

4. Lib
   Lib, as the name suggests is where you store all your custom or third party libraries, these have no relevance to your business, but aim to be important tools that your project requires, regardless of Application level service/worker, and business logic.

---

The above is a top overview over each directory, a README.md will be included in each directory to give you more information on the reasonings for the structure.

This structure aims to make your code more understandle for developers. This structure enforces a top to bottom rule. For example; App can use modules from Core, Domain and Lib. Core can use modules from Domain and Lib, but not App. Domain cannot use modules from App, or Core, but can use modules from Lib, Lib cannot use other modules in the project, unless it is a sub module. Enforcing this allows you to understand the `flow` of code.

Lastly, to register all your modules to this project, the `lib.rs` file is provided, please edit this to register your new module.
