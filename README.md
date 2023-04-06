The program uses a system level language, Rust, to get data about the network interfaces available on a device and send the result over an application Binary interface to be consumed by another programming language. Swift is the language of choice in this example.

The rust binaries were complied for an arm64 and amd64 architectures. The resulting fat file was tested with the XCode i-Phone simulator. This work serves to demonstrate how to pass a collection of various datatypes over an FFI and read the data from the other side.

More work could be done on the project to create a more functional UI and UX.
