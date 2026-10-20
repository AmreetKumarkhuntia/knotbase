// sqlx::migrate! embeds a directory at compile time but only re-runs when an existing file changes; this makes a new migration file rebuild too.
fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
