// Imports file
mod vars;
mod print;
mod primitive_types;
mod strings;
mod greatest_common_divisor;


fn main() {
    primitive_types::run();
    vars::run();
    print::run();
    strings::run();
    greatest_common_divisor::run();
    /* C and C++ require main fn to return 0 if finished successfully.
     * Rust assumes that if main fn returns at all, the program finished
     * successfully.
     * If we need to terminate program with an error status code, it can 
     * be achieved only by explicitly calling functions like:
     * expect or std::process::exit
     */
}
