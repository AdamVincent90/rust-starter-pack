### The Src

This is where your rust codes sit, in terms of creating scalable, clean, and maintable code, src consists of three primary directories (layers).

1. Application
   Application contains all your binaries and their configuration to start either, listening to requests, or executing jobs. An `Application` can fall under four different types: 1. Services - A service that listens or responds to requests. 2. Tools - A job that when runs, performs an operation that automates tasks for the developer. 3. Workers - A worker is another tool that instead performs an operation that solves a business related task, these workers can be ad-hoc, or indeed scheduled.

---

2. Business
   Business contains the logic your directories call to solve a business task. This could be an Application level handler calling a business function to create a user, therefore, a task that is soley related to the business requirements of your project.

---

3. Dependency
   Dependency, as the name suggests is where you store all your custom or third party libraries, these have no relevance to your business, but aim to be utilities and important tools that your project requires, regardless of Application level service/worker, and business logic.

---

The above is a top overview over each directory, a README.md will be included in each directory to give you more information on the reasonings for the structure.

This structure aims to make your code more understandle for developers. This structure enforces a top to bottom rule. For example; Application can use modules from Business and Dependency. Business cannot use modules from Application, but can use modules from Dependency, Dependency cannot use other modules in the project, unless it is a sub module. Enforcing this allows you to understand the `flow` of code.

Lastly, to register all your modules to this project, the `lib.rs` file is provided, please edit this to register your new module.
