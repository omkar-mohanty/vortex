
# vortex

vortex is a simple tool to extract images from pdf files 

## Installation

### 1. Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
````

### 2. Clone the repo
```
git clone https://github.com/omkar-mohanty/unpdf.git && cd unpdf
```
### 3. Install using cargo
```
cargo install --path .
```

## Usage

```bash
vortex pdf_file -o output_folder
```

### Example
From the crate root

```bash
vortex resources/sample.pdf -o sample 
```
