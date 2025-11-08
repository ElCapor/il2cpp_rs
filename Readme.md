# Note

This repo is currently in development and is not ready for production use.

I wrote it in a single day with 15 hours of work.
# il2cpp_rs

A lightweight Rust library for discovering and navigating IL2CPP metadata at runtime. It provides a safe-ish Rust façade over the IL2CPP C API, and builds a cache of assemblies, classes, fields, and methods with a modern ownership model for convenient querying and printing.

> Note: This repo targets Windows and uses an injected DLL entry (`DllMain`) to attach to a running IL2CPP process (e.g., a Unity game), then prints metadata to a console.

---

## Features

- IL2CPP API bindings (`il2cpp::il2cpp_sys`) and ergonomic wrappers (`il2cpp` module)
- Discovery of:
  - Assemblies → Classes → Fields → Methods
  - Method signatures: name, flags, parameters, return type
  - Field metadata: name, offset, static-ness, type
- Modern ownership model for metadata graph:
  - `Arc` shared handles for nodes (`Assembly`, `Class`, `Field`, `Method`, `Type`)
  - `Weak` back-references (e.g., `Field`/`Method` → `Class`) to avoid cycles
  - Thread-safe collections via `RwLock<Vec<...>>` in `Class`
- Minimal profiling utilities to time code paths (`profile_scope!`, `profile_call!`)

---

## Architecture Overview

- `src/il2cpp/il2cpp_sys`: raw FFI to IL2CPP exports (pointers, C-strings). All low-level `Il2Cpp*` are represented as `*mut u8` handles.
- `src/il2cpp/mod.rs`: safe-ish wrappers around the FFI that return Rust types (e.g., `String`, `Vec<...>`), and helper functions like:
  - `get_domain`, `thread_attach`/`thread_detach`
  - `domain_get_assemblies`, `assembly_get_image`
  - `image_get_class_count`, `image_get_class`
  - `class_get_name`/`namespace`/`parent`
  - `class_get_fields`, `field_get_name`/`offset`/`type`
  - `class_get_methods`, `method_get_name`/`flags`/`params`/`return_type`
- `src/il2cpp/classes`: high-level Rust model types used in the cache
  - `Class = Arc<ClassInner>`
    - `fields: RwLock<Vec<Field>>`
    - `methods: RwLock<Vec<Method>>`
  - `Field = Arc<FieldInner>`
    - `class: Weak<ClassInner>` backref
  - `Method = Arc<MethodInner>`
    - `class: Weak<ClassInner>` backref, `return_type: Type`
  - `Type = Arc<TypeInner>` (cacheable handle with `address`, `name`, `size`)
- `src/il2cpp_cache.rs`: metadata discovery and hydration into the high-level types
  - `Cache::parse_assemblies(domain)`
  - `Cache::parse_class(&mut Assembly, image)`
  - `Cache::parse_fields(&Class)` (populates `fields`)
  - `Cache::parse_methods(&Class)` (populates `methods`)

---

## Ownership Model

- Strong edges (Arc):
  - `Assembly` → `Vec<Class>`
  - `Class` → `RwLock<Vec<Field>>`, `RwLock<Vec<Method>>`
- Weak edges (Weak):
  - `Field.class`, `Method.class` → `Weak<ClassInner>`
- Benefits:
  - Avoid cycles (`Class ↔ Field/Method`)
  - Safe cloning of handles (cheap `Arc` clones)
  - Thread-safe reads and targeted writes (`RwLock`)

---

## Profiling

The crate exposes a tiny profiling module for quick ad-hoc timing in dev builds.

- Scope-based timing:

```rust
profile_scope!("Cache::new");
// code to profile...
```

- Single expression timing with result preserved:

```rust
let cache = profile_call!("Cache::new", Cache::new(domain));
```

Printed output example:

```
Cache::new took 1.23ms
```

Implementation: see `src/prof.rs` (`ScopeTimer`) and macros exported at crate root.

---

## Building

- Install and build:

```bash
rustup toolchain install stable
cargo build
```

- Standard dev build:

```bash
cargo build
```

The library compiles as a DLL (due to `DllMain`). You’ll typically inject it into a running IL2CPP process on Windows.

---

## Running / Entry Point

- On DLL load, `DllMain` spawns a Rust thread and calls `entry_point()`.
- `entry_point()`:
  - Allocates a console and initializes the IL2CPP API
  - Attaches the thread to the IL2CPP domain
  - Builds the `Cache` by walking assemblies → classes → fields/methods
  - Prints debug info (you can comment/uncomment the debug prints)

---

## Example: Discover and print classes

```rust
use il2cpp_rs::il2cpp;
use il2cpp_rs::il2cpp_cache::Cache;

fn example() -> Result<(), String> {
    let domain = il2cpp::get_domain()?;
    il2cpp::thread_attach(domain)?;

    let cache = Cache::new(domain)?;
    // Debug printing is available via Debug impls
    // println!("{:?}", cache);
    Ok(())
}
```

---

## Safety Notes

- The FFI layer manipulates raw pointers (`*mut u8`) from IL2CPP. Access patterns assume the underlying engine keeps these pointers valid while attached to the domain.
- Do not send handles across threads unless you’ve attached those threads to the IL2CPP domain (`thread_attach`).
- Avoid storing borrowed C-string pointers; convert to Rust `String` immediately (already handled by wrappers).
- All `Arc`/`Weak` handles are Send/Sync only insofar as the contained data is. The raw pointer addresses are opaque and not dereferenced in safe code.

---

## Contributing

- Run `cargo fmt` and `cargo clippy` on changes
- Keep FFI wrappers minimal and well-commented
- Preserve the Arc/Weak ownership model and avoid re-introducing cycles

---

## License

MIT for now, licence is subject to change any time soon
