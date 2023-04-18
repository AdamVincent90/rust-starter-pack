// Where we register our dependency, and business modules for our src.
// This allows our App and Business layers to call modules and use them for
// Our tools, services, and workers.

// Your core modules here.
pub mod core {
    pub mod user;
}

// Your domain modules here.
pub mod domain {

    pub mod system {
        pub mod auth;
        pub mod error;
    }

    pub mod web {
        pub mod middleware;
        pub mod state;
    }
}

// Your lib modules here.
pub mod lib {
    pub mod database;
    pub mod logger;
    pub mod server;
}
