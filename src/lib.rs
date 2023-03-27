// Where we register our foundation, and business modules for our src.
// This allows our App and Business layers to call modules and use them for
// Our tools, services, and workers.

pub mod business {
    // Your business modules here.
    pub mod core {
        pub mod user;
    }

    pub mod system {
        pub mod validation;
    }
}

pub mod foundation {
    pub mod database;
    pub mod logger;
    pub mod server;
}
