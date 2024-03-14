To run, run `cargo run`.
If you want to specify port (default is 1337), run `cargo run -- <your port>`.
The proxy automatically blocks requests to all the addresses that contain "google" in them, as well as the addresses specified in `blacklist.txt`.