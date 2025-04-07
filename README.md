# inachus

![inachus](assets/inachus.jpg)

## Features

- Load and manage multiple contract ABIs
- Support for both read and write operations
- Interactive command-line interface
- Configurable RPC endpoint and wallet settings
- Support for array and tuple parameters
- Transaction confirmation for write operations
- Automatic parameter validation

2. Build the project:
```bash
cargo build --release
```

## Configuration

The tool uses two configuration files:

1. `config.json`: Contains RPC URL, private key, chain ID, and wait time settings
2. `contracts.json`: Contains contract names and their deployed addresses

These files are stored in the `~/.inachus` directory.

Example `config.json`:
```json
{
  "rpc_url": "https://mainnet.infura.io/v3/your-project-id",
  "private_key": "your-private-key",
  "chain_id": "1",
  "wait_time": "5"
}
```

Example `contracts.json`:
```json
[
  {
    "name": "MyContract.abi",
    "address": "0x1234567890123456789012345678901234567890"
  }
]
```

## Usage

1. Place your contract ABI files in the `~/.inachus/abi` directory with a `.abi` extension.

2. Run the tool:
```bash
cargo run --release
```

3. Follow the interactive prompts to:
   - Select a contract
   - Set contract address
   - Choose between read and write methods
   - Input method parameters
   - Execute transactions

## License

This project is licensed under the MIT License - see the LICENSE file for details.