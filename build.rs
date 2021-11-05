extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut cfg = cc::Build::new();

    cfg
        .cpp(true)
        .flag("-std=c++17")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-variable")
        .flag("-Wno-missing-field-initializers")
        .flag("-Wno-implicit-fallthrough")
        .flag("-Wno-unused-but-set-variable")
        
        .include("luau/Analysis/include")
        .include("luau/Ast/include");

    cfg.clone()
        .file("luau/Ast/src/Ast.cpp")
        .file("luau/Ast/src/Confusables.cpp")
        .file("luau/Ast/src/Lexer.cpp")
        .file("luau/Ast/src/Location.cpp")
        .file("luau/Ast/src/Parser.cpp")
        .file("luau/Ast/src/StringUtils.cpp")
        
        .compile("Luau.Ast");
        
    cfg.clone()
        .file("luau/Analysis/src/AstQuery.cpp")
        .file("luau/Analysis/src/Autocomplete.cpp")
        .file("luau/Analysis/src/BuiltinDefinitions.cpp")
        .file("luau/Analysis/src/Config.cpp")
        .file("luau/Analysis/src/Error.cpp")
        .file("luau/Analysis/src/Frontend.cpp")
        .file("luau/Analysis/src/IostreamHelpers.cpp")
        .file("luau/Analysis/src/JsonEncoder.cpp")
        .file("luau/Analysis/src/Linter.cpp")
        .file("luau/Analysis/src/Module.cpp")
        .file("luau/Analysis/src/Predicate.cpp")
        .file("luau/Analysis/src/RequireTracer.cpp")
        .file("luau/Analysis/src/Scope.cpp")
        .file("luau/Analysis/src/Substitution.cpp")
        .file("luau/Analysis/src/Symbol.cpp")
        .file("luau/Analysis/src/TopoSortStatements.cpp")
        .file("luau/Analysis/src/ToString.cpp")
        .file("luau/Analysis/src/Transpiler.cpp")
        .file("luau/Analysis/src/TxnLog.cpp")
        .file("luau/Analysis/src/TypeAttach.cpp")
        .file("luau/Analysis/src/TypedAllocator.cpp")
        .file("luau/Analysis/src/TypeInfer.cpp")
        .file("luau/Analysis/src/TypePack.cpp")
        .file("luau/Analysis/src/TypeUtils.cpp")
        .file("luau/Analysis/src/TypeVar.cpp")
        .file("luau/Analysis/src/Unifiable.cpp")
        .file("luau/Analysis/src/Unifier.cpp")
        .file("luau/Analysis/src/EmbeddedBuiltinDefinitions.cpp")

        .compile("Luau.Analysis");

    cfg.clone()
        .file("wrapper.cpp")
        .cpp_link_stdlib("stdc++")
        .compile("wrapper");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .enable_cxx_namespaces()
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
