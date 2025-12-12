# What is MDTeX?
LaTeX and Markdown are two of the most popular markup languages on the market today; LaTeX is a feature-rich, high-level ecosystem which appeals to academics while Markdown is lightweight and fast to appeal to developers. MDTeX bridges the two ecosystems together, providing a common markup language which both academics and developers can take advantage of.

Users are able to create projects within the MDTeX ecosystem, add any required LaTeX dependencies, and write markup in a very lightweight Markdown-like format. Projects can then be compiled as a PDF or as an HTML site.

# Technical Architecture
![Assignment Aggregator Technical Architecture](https://github.com/user-attachments/assets/81b5ceea-5e92-426e-8279-bccea6c07a25)

The CLI and web interface allow users to seamlessly interact with the MDTeX ecosystem. The compiler converts MDTeX into an intermediate representation to be processed and rendered as a PDF or HTML script by the renderer. The project manager allows for the user to manage their project structure while also providing useful information to the compiler. The package manager allows for the user to add LaTeX dependencies to their project.

# Installation Guide

1. Install Rust:
If you don’t already have Rust installed:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Clone the repository
```
git clone https://github.com/CS222-UIUC/fa25-team079.git
cd fa25-team079
cd mdtexcli
```
3. Build the binary
```
cargo build --release
```
The compiled executable will appear in:
```
target/release/mdtexcli
```
4. Run MDTeX!

# Contributors
- **Aarnav Srivastava**: Worked on the project and package manager. 
- **Agastya Govind**: Worked on the CLI tool.
- **Mariano Rodriguez**: Worked on converting MDTeX to the IR within the compiler.
- **Tejas Raghruam**: Worked on processing the IR within the renderer.

