# terminal-graphics-renderer

A lightweight 3D wireframe renderer that runs entirely in the terminal, written in Rust. Loads 3D objects from a simple text-based format defining vertices and edges, applies perspective projection, and renders them as ASCII art directly in your terminal window. Supports real-time rotation on all three axes (X, Y, Z) via keyboard input.

```
        *-------*
       /|      /|
      * -------* |
      | |      | |
      | *------|-*
      |/       |/
      *--------*
```

---

## Features

- Custom `.txt` object format for defining 3D meshes (vertices + edges)
- Perspective projection from 3D to 2D screen space
- Real-time keyboard-driven rotation on X, Y, and Z axes
- ASCII framebuffer rendering — no GPU, no window manager needed
- Raw terminal mode via [`crossterm`](https://github.com/crossterm-rs/crossterm) for cross-platform input handling

---

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

---

## Building & Running

```bash
git clone https://github.com/RFilipovic/terminal-graphics-renderer
cd terminal-graphics-renderer
cargo build --release
cargo run --release -- test.txt
```

Pass the path to any `.txt` object file as the argument. The included `test.txt` is a ready-to-use example mesh.

---

## Object File Format

Objects are defined in plain `.txt` files using two directives:

- `v x y z` — defines a vertex at the given 3D coordinates
- `e i j` — defines an edge between vertex index `i` and vertex index `j` (zero-indexed)

Example — a cube:

```
v  1.0  1.0  1.0
v  1.0  1.0 -1.0
v  1.0 -1.0  1.0
v  1.0 -1.0 -1.0
v -1.0  1.0  1.0
v -1.0  1.0 -1.0
v -1.0 -1.0  1.0
v -1.0 -1.0 -1.0
e 0 1
e 0 2
e 1 3
e 2 3
e 4 5
e 4 6
e 5 7
e 6 7
e 0 4
e 1 5
e 2 6
e 3 7
```

The included `test.txt` defines a more complex mesh with 20 vertices and 30 edges.

---

## Controls

| Key | Action |
|-----|--------|
| `W` / `S` | Rotate around X axis |
| `A` / `D` | Rotate around Y axis |
| `Q` / `E` | Rotate around Z axis |
| `Ctrl+C` | Quit |

---

## Project Structure

```
terminal-graphics-renderer/
├── src/           # Rust source code
├── test.txt       # Example 3D mesh
├── Cargo.toml     # Project manifest (dependency: crossterm 0.27)
└── Cargo.lock
```

---

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| [`crossterm`](https://crates.io/crates/crossterm) | 0.27 | Cross-platform raw terminal input and cursor control |

---

## License

Not specified. Contact the repository owner for usage terms.
