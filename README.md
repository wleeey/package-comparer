# package-comparer

This program compares the alt packages and outputs the result, which includes:

* Packages that are in the sisyphus branch, but which are not in the p10 branch.

* Packages that are in the p10 branch, but which are not in the sisyphus branch.

* Packages whose versions are higher in sisyphus than in p10.

## installation

### 1. Install Rust

If you haven't installed Rust yet, you can do so using [rustup](https://rustup.rs/). Run the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After installation, add Rust to your PATH (this usually happens automatically). Restart your terminal if necessary.

### 2. Clone the Repository

```bash
git clone https://github.com/wleeey/package-comparer.git
```
### 3. Navigate to the Project Directory

```bash
cd package-comparer
```
### 4. Run the Project

Now you can run the project using Cargo:

```bash
cargo run
```
