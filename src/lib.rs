// Where we register our dependency, and business modules for our src.
// This allows our App and Business layers to call modules and use them for
// Our tools, services, and workers.

pub mod business {
    // Your business modules here.
    pub mod core {
        pub mod user;
    }

    pub mod system {
        pub mod auth;
        pub mod validation;
    }

    pub mod web {
        pub mod middleware;
    }
}

pub mod dependency {
    pub mod database;
    pub mod logger;
    pub mod server;
}
