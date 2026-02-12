# LEF/DEF Parser in Rust

A Rust library and toolset for parsing **LEF** and **DEF** files used in VLSI physical design flows.  
The parser is implemented using the **LALRPOP parser generator**, aiming to closely follow the official LEF/DEF grammar at the production-rule level.

This project targets high-performance parsing and easy integration into placement, routing, and analysis tools written in Rust.

---

## Features

- 📐 Parsing support for **LEF** and **DEF** formats
- 🧩 Grammar implemented using **LALRPOP**
- 📘 Grammar rules closely follow official LEF/DEF specifications
- 🚀 High-performance Rust implementation
- 🧵 Designed for large industrial-scale designs
- 🔍 Structured AST representation for downstream processing
- 🛠 Suitable for research and production EDA tooling

---

## Project Goals

- Maintain grammar close to official LEF/DEF syntax
- Enable reliable parsing of large industrial designs
- Provide reusable Rust data structures
- Allow easy integration into placement/routing/analysis flows

The parser favors correctness and grammar fidelity over ad-hoc parsing shortcuts.

---

## LEF/DEF Overview

- **LEF (Library Exchange Format)** describes technology and cell libraries.
- **DEF (Design Exchange Format)** describes placed and routed designs.

Together, they are core formats in modern VLSI physical design workflows.

---

## Installation

Clone the repository:

```bash
git clone https://github.com/giammirove/reda-lefdef
cd reda-lefdef
cargo build --release
```

## Testing

```bash
cd tests
./download_tests.sh
./run_tests.sh
```


## Related Links

- **LALRPOP parser generator**: https://github.com/lalrpop/lalrpop

- **DEF Syntax Reference**: https://coriolis.lip6.fr/doc/lefdef/lefdefref/DEFSyntax.html

- **LEF Syntax Reference**: https://coriolis.lip6.fr/doc/lefdef/lefdefref/LEFSyntax.html

