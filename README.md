<div align="center">
  <h1>Root</h1>
  <p>a backend to manage all club information</p>
</div>

### Overview
**Root** is a backend for managing all club related info; most other projects will be getting or publishing their data to root.

### Setup Instructions

#### Prerequisites
1. Ensure you have Rust installed. Use `rustup` for easy installation.  
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
2. Install the Shuttle CLI.
   ```bash
   cargo install shuttle-cli
   ```

3. Install Docker. Check [this](https://docs.docker.com/desktop/setup/install/linux/) out for instructions.

#### Clone the Repository
```bash
git clone https://github.com/amfoss/root.git
cd root
```

#### Set up Secrets
Create a `Secrets.toml` file in the root directory with the secret key
```
ROOT_SECRET='secret_key'
```

#### Run Locally
```bash
cargo shuttle run
```

---

### Documentation

Explore the [Documentation](/docs/docs.md) for detailed information and usage guidelines.

---

### How to Contribute

1. Fork the repository and clone it to your local machine.
2. Set up the project by following the installation instructions above.
3. Identify an issue or feature you'd like to work on, and create an issue to track it.
4. Develop the patch or feature, ensuring it is thoroughly tested.
5. Submit a pull request, referencing the relevant issue number.

### License
This project is licensed under GNU General Public License V3. You are welcome to adapt it, make it yours. Just make sure that you credit us too.