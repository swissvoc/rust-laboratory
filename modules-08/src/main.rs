// Modules let us organize code into groups and control the privacy of paths.
mod tool;

/*
    Here are the privacy rules:
    - All items (functions, methods, structs, enums, modules, annd constants) are private by default.
    - You can use the pub keyword to make an item public.
    - You aren’t allowed to use private code defined in modules that are children of the current module.
    - You are allowed to use any code defined in ancestor modules or the current module.
*/
fn main() {
    /*
        If we want to call a function, we need to know its path. “Path” is a synonym for “name” in a way, but it evokes that filesystem metaphor.
        Additionally, functions, structs, and other items may have multiple paths that refer to the same item, so “name” isn’t quite the right concept.

        A path can take two forms:
        - An absolute path starts from a crate root by using a crate name or a literal crate.
        - A relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.

        Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).
    */
    crate::tool::hammer::clang(); // absolute path
    tool::hammer::clang(); // relative path
}
