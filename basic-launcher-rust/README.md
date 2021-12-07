# basic-launcher-rust

Using DOSBox, it launches GWBasic/QBasic and runs a BASIC program.

## Limitations

Getting input from stdin does not work. Programs are expected to read
input from a file, whose name is provided at the environment variable named STDIN.

