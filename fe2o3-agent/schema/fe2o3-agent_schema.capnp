@0x80ed5d8755d2da41;

using Rust = import "rust.capnp";
$Rust.parentModule("server");

struct Point {
    x @0 :Float32;
    y @1 :Float32;
}

interface PointTracker {
    addPoint @0 (p :Point) -> (totalPoints :Float32);
}