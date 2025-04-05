# Sonic-migrate

`sonic-migrate` is a CLI tool that helps developers migrate their Solana Anchor projects to the Sonic Network. It modifies the `Anchor.toml` configuration file and updates the cluster RPC URL to point to the Sonic Network, among other tasks. This tool is designed to simplify the process of upgrading and migrating existing Solana Anchor projects.

## Features

-  **Automatic Migration**: Updates the `Anchor.toml` file to migrate from standard Solana clusters to Sonic Network.
-  **Display available sonic's network endpoint**: List available networks and their RPC URLs.
-  **Migrate to sonic's target network endpoint**: Target Sonic network (testnet, mainnet-alpha).
-  **Backup & Restore**: Automatically backs up the existing `Anchor.toml` to ensure you can restore it if needed.
-  **Dry Run Option**: See what changes would be made without applying them.
-  **Verbose Logging**: Provides detailed output to help you understand the migration process.


## Installation

You can install `sonic-migrate` using Cargo:

```bash
cargo install sonic-migrate
```

## Usage

### Basic Usage

Navigate to the root directory of your Anchor project and run:

```bash
sonic-migrate
```

This will migrate your project by modifying `Anchor.toml` and updating the cluster RPC URL to the Sonic Testnet Network. The tool will also create a backup (`Anchor.toml.bak`) before making any changes.

```bash
sonic-migrate --networks
```
This will List available networks and their RPC URLs for sonic SVM.

```bash
sonic-migrate --network <network>
```
This will migrate your project by modifying `Anchor.toml` and updating the cluster RPC URL to provided network endpoint.

### Running with a Specific Path

You can specify the path to your Anchor project explicitly:

```bash
sonic-migrate /path/to/your/anchor-project
```

### Dry Run Mode (Recommended First Step)

If you want to preview the changes that will be made without modifying the actual files, use the `--dry-run` flag:

```bash
sonic-migrate --dry-run
```

This will print out the changes that would be made to `Anchor.toml` without making any modifications.

### Verbose Mode

For more detailed logging about the migration process, use the `--verbose` flag:

```bash
sonic-migrate --verbose
```

### Restore from Backup

If you need to revert the changes made by `sonic-migrate`, you can restore the backup using the `--restore` flag:

```bash
sonic-migrate --restore
```

### Full Command Reference

- **Basic Migration to sonic Testnet**:
  ```bash
  sonic-migrate
  ```
- **List available networks and their RPC URLs**:
  ```bash
  sonic-migrate --networks
    ```
- **Migration to Target Sonic network (testnet, mainnet-alpha)**:
  ```bash
    sonic-migrate --network <network>
   ```
- **Specify Path**:
  ```bash
  sonic-migrate /path/to/project
  ```
- **Dry Run**:
  ```bash
  sonic-migrate --dry-run
  ```
- **Verbose Logging**:
  ```bash
  sonic-migrate --verbose
  ```
- **Restore Backup**:
  ```bash
  sonic-migrate --restore
  ```

## Example Workflow

1. **Run a Dry Run** to see what changes will be made:

   ```bash
   sonic-migrate --dry-run
   ```

2. **Run the Actual Migration** after reviewing the dry run output:

   ```bash
   sonic-migrate
   ```

3. **Verify** the updated `Anchor.toml` and run your project's tests.

4. If something went wrong, **restore from the backup**:

   ```bash
   sonic-migrate --restore
   ```

## How it Works

`sonic-migrate` performs the following tasks:

1. **Validation**: Ensures that the specified directory is a valid Anchor project with `Anchor.toml` and `Cargo.toml`.
2. **Backup**: Creates a backup of `Anchor.toml` before making changes.
3. **Modification**: Updates the RPC URL in `Anchor.toml` to point to the Sonic Testnet Network:
   ```
   https://api.testnet.sonic.game
   ```
4. **Logging**: Provides detailed progress, error messages, and final instructions.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request or open an Issue for suggestions, improvements, or bug reports.

### Development Setup

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/shivamSspirit/Sonic-migrate.git
   cd sonic-migrate
   ```

2. **Build the Project**:

   ```bash
   cargo build
   ```

3. **Run Tests**:

   ```bash
   cargo test
   ```

## Contact

- **Author**: shivam soni (shivamsoni6@gmail.com)
- **GitHub**: [shivamSspirit](https://github.com/shivamSspirit)

If you have any questions or suggestions, feel free to reach out!

---

Give `sonic-migrate` a ⭐ on GitHub if you find it useful!