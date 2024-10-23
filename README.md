# Ral - Rails of Package Management

Ral is a package manager designed to streamline the installation, removal, and management of packages on your system. It supports both downloading and building packages, and aims to provide compatibility with apt, dnf, and pacman to handle dependencies. Ultimately, Ral will enable you to manage your system with just one package manager.

**Note: Ral is currently under development. Expect changes and improvements in future releases.**

## Features
- Install packages via downloading or building from source.
- Use `ral` to install missing dependencies with apt, dnf, or pacman.
- Aliases for common operations:
  - `install` -> `inst`
  - `remove` -> `rm`
  - `upgrade` -> `up`
  - `update package list` -> `up --list`
- Manage package lists in JSON format.

## Installation
Ral is not ready for general use yet. However, if you want to contribute or test the current state, you can clone the repository and build it locally.

```bash
git clone https://github.com/yourusername/ral.git
cd ral
cargo build --release
```

## Usage
Ral uses simple commands to handle package management tasks.

```bash
ral inst <package-name>     # Alias for 'install'
ral rm <package-name>       # Alias for 'remove'
ral up                      # Alias for 'upgrade'
ral up --list               # Update the package list
```

## Contributing
Contributions are welcome! If you'd like to contribute to Ral, feel free to open issues or submit pull requests on GitHub.

## License
Ral is licensed under the MIT License. See [LICENSE](LICENSE) for more details.

