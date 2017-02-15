# Attributes

Any item declaration may have an _attribute_ applied to it. Attributes in Rust
are modeled on Attributes in ECMA-335, with the syntax coming from ECMA-334
(C#). An attribute is a general, free-form metadatum that is interpreted
according to name, convention, and language and compiler version. Attributes
may appear as any of:

* A single identifier, the attribute name
* An identifier followed by the equals sign '=' and a literal, providing a
  key/value pair
* An identifier followed by a parenthesized list of sub-attribute arguments

Attributes with a bang ("!") after the hash ("#") apply to the item that the
attribute is declared within. Attributes that do not have a bang after the hash
apply to the item that follows the attribute.

An example of attributes:

```{.rust}
// General metadata applied to the enclosing module or crate.
#![crate_type = "lib"]

// A function marked as a unit test
#[test]
fn test_foo() {
    /* ... */
}

// A conditionally-compiled module
#[cfg(target_os="linux")]
mod bar {
    /* ... */
}

// A lint attribute used to suppress a warning/error
#[allow(non_camel_case_types)]
type int8_t = i8;
```

> **Note:** At some point in the future, the compiler will distinguish between
> language-reserved and user-available attributes. Until then, there is
> effectively no difference between an attribute handled by a loadable syntax
> extension and the compiler.

## Crate-only attributes

- `crate_name` - specify the crate's crate name.
- `crate_type` - see [linkage](#linkage).
- `feature` - see [compiler features](#compiler-features).
- `no_builtins` - disable optimizing certain code patterns to invocations of
                  library functions that are assumed to exist
- `no_main` - disable emitting the `main` symbol. Useful when some other
   object being linked to defines `main`.
- `no_start` - disable linking to the `native` crate, which specifies the
  "start" language item.
- `no_std` - disable linking to the `std` crate.
- `plugin` - load a list of named crates as compiler plugins, e.g.
             `#![plugin(foo, bar)]`. Optional arguments for each plugin,
             i.e. `#![plugin(foo(... args ...))]`, are provided to the plugin's
             registrar function.  The `plugin` feature gate is required to use
             this attribute.
- `recursion_limit` - Sets the maximum depth for potentially
                      infinitely-recursive compile-time operations like
                      auto-dereference or macro expansion. The default is
                      `#![recursion_limit="64"]`.

### Module-only attributes

- `no_implicit_prelude` - disable injecting `use std::prelude::*` in this
  module.
- `path` - specifies the file to load the module from. `#[path="foo.rs"] mod
  bar;` is equivalent to `mod bar { /* contents of foo.rs */ }`. The path is
  taken relative to the directory that the current module is in.

## Function-only attributes

- `main` - indicates that this function should be passed to the entry point,
  rather than the function in the crate root named `main`.
- `plugin_registrar` - mark this function as the registration point for
  [compiler plugins][plugin], such as loadable syntax extensions.
- `start` - indicates that this function should be used as the entry point,
  overriding the "start" language item. See the "start" [language
  item](#language-items) for more details.
- `test` - indicates that this function is a test function, to only be compiled
  in case of `--test`.
- `should_panic` - indicates that this test function should panic, inverting the success condition.
- `cold` - The function is unlikely to be executed, so optimize it (and calls
  to it) differently.
- `naked` - The function utilizes a custom ABI or custom inline ASM that requires
  epilogue and prologue to be skipped.

## Static-only attributes

- `thread_local` - on a `static mut`, this signals that the value of this
  static may change depending on the current thread. The exact consequences of
  this are implementation-defined.

## FFI attributes

On an `extern` block, the following attributes are interpreted:

- `link_args` - specify arguments to the linker, rather than just the library
  name and type. This is feature gated and the exact behavior is
  implementation-defined (due to variety of linker invocation syntax).
- `link` - indicate that a native library should be linked to for the
  declarations in this block to be linked correctly. `link` supports an optional
  `kind` key with three possible values: `dylib`, `static`, and `framework`. See
  [external blocks](#external-blocks) for more about external blocks. Two
  examples: `#[link(name = "readline")]` and
  `#[link(name = "CoreFoundation", kind = "framework")]`.
- `linked_from` - indicates what native library this block of FFI items is
  coming from. This attribute is of the form `#[linked_from = "foo"]` where
  `foo` is the name of a library in either `#[link]` or a `-l` flag. This
  attribute is currently required to export symbols from a Rust dynamic library
  on Windows, and it is feature gated behind the `linked_from` feature.

On declarations inside an `extern` block, the following attributes are
interpreted:

- `link_name` - the name of the symbol that this function or static should be
  imported as.
- `linkage` - on a static, this specifies the [linkage
  type](http://llvm.org/docs/LangRef.html#linkage-types).

On `enum`s:

- `repr` - on C-like enums, this sets the underlying type used for
  representation. Takes one argument, which is the primitive
  type this enum should be represented for, or `C`, which specifies that it
  should be the default `enum` size of the C ABI for that platform. Note that
  enum representation in C is undefined, and this may be incorrect when the C
  code is compiled with certain flags.

On `struct`s:

- `repr` - specifies the representation to use for this struct. Takes a list
  of options. The currently accepted ones are `C` and `packed`, which may be
  combined. `C` will use a C ABI compatible struct layout, and `packed` will
  remove any padding between fields (note that this is very fragile and may
  break platforms which require aligned access).

## Macro-related attributes

- `macro_use` on a `mod` — macros defined in this module will be visible in the
  module's parent, after this module has been included.

- `macro_use` on an `extern crate` — load macros from this crate.  An optional
  list of names `#[macro_use(foo, bar)]` restricts the import to just those
  macros named.  The `extern crate` must appear at the crate root, not inside
  `mod`, which ensures proper function of the [`$crate` macro
  variable](book/macros.html#The%20variable%20%24crate).

- `macro_reexport` on an `extern crate` — re-export the named macros.

- `macro_export` - export a macro for cross-crate usage.

- `no_link` on an `extern crate` — even if we load this crate for macros, don't
  link it into the output.

See the [macros section of the
book](book/macros.html#Scoping%20and%20macro%20import%2Fexport) for more information on
macro scope.

## Miscellaneous attributes

- `deprecated` - mark the item as deprecated; the full attribute is 
  `#[deprecated(since = "crate version", note = "...")`, where both arguments 
  are optional.
- `export_name` - on statics and functions, this determines the name of the
  exported symbol.
- `link_section` - on statics and functions, this specifies the section of the
  object file that this item's contents will be placed into.
- `no_mangle` - on any item, do not apply the standard name mangling. Set the
  symbol for this item to its identifier.
- `simd` - on certain tuple structs, derive the arithmetic operators, which
  lower to the target's SIMD instructions, if any; the `simd` feature gate
  is necessary to use this attribute.
- `unsafe_destructor_blind_to_params` - on `Drop::drop` method, asserts that the
  destructor code (and all potential specializations of that code) will
  never attempt to read from nor write to any references with lifetimes
  that come in via generic parameters. This is a constraint we cannot
  currently express via the type system, and therefore we rely on the
  programmer to assert that it holds. Adding this to a Drop impl causes
  the associated destructor to be considered "uninteresting" by the
  Drop-Check rule, and thus it can help sidestep data ordering
  constraints that would otherwise be introduced by the Drop-Check
  rule. Such sidestepping of the constraints, if done incorrectly, can
  lead to undefined behavior (in the form of reading or writing to data
  outside of its dynamic extent), and thus this attribute has the word
  "unsafe" in its name. To use this, the
  `unsafe_destructor_blind_to_params` feature gate must be enabled.
- `doc` - Doc comments such as `/// foo` are equivalent to `#[doc = "foo"]`.
- `rustc_on_unimplemented` - Write a custom note to be shown along with the error
   when the trait is found to be unimplemented on a type.
   You may use format arguments like `{T}`, `{A}` to correspond to the
   types at the point of use corresponding to the type parameters of the
   trait of the same name. `{Self}` will be replaced with the type that is supposed
   to implement the trait but doesn't. To use this, the `on_unimplemented` feature gate
   must be enabled.
- `must_use` - on structs and enums, will warn if a value of this type isn't used or
   assigned to a variable. You may also include an optional message by using
   `#[must_use = "message"]` which will be given alongside the warning.
