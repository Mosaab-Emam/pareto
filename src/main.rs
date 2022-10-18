mod laravel;
mod zip_helper;

enum Backend {
    Laravel,
}

fn main() {
    let backend = Backend::Laravel;
    let path = ".";

    // Determine which backend initializer to use
    match backend {
        Backend::Laravel => laravel::create(&path),
        _ => panic!("This backend is not supported!"),
    }
}
