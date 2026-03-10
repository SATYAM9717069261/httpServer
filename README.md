[DATA FLOW]

TCP Listener
   ↓
TCP Stream
   ↓
Reader (read bytes)
   ↓
HTTP Parser
   ↓
Request Struct
   ↓
Response Builder
   ↓
Write back to socket



Structure
    src
    ├── main.rs
    ├── server.rs
    ├── reader.rs
    ├── request.rs
    ├── response.rs
    └── parser
        ├── mod.rs
        ├── request_line.rs
        └── headers.rs



TCP NET => https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/net/index.html

cargo build
cargo run
