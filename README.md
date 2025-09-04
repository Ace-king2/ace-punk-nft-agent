Ace Punk NFT Agent
About
Autonomous AI agent built with Rust and WebAssembly using the WASP framework. Designed for the UOMI network, it generates cyberpunk-themed NFT prompts, formats metadata for ERC-721 tokens, and prepares assets for minting. The agent processes structured user input, interacts with UOMI's unified API, and supports hot-reloading for rapid iteration.
Features

Cyberpunk NFT Generation: Creates unique cyberpunk-themed NFT prompts and metadata
ERC-721 Compatibility: Formats metadata according to ERC-721 standards
UOMI Network Integration: Seamlessly connects to UOMI's unified API
Multi-Model Support: Works with both Qwen/Qwen2.5-32B-Instruct and OpenAI GPT models
IPFS Ready: Prepared for decentralized storage with IPFS gateway integration
Hot-Reloading: Supports rapid development iteration
WebAssembly Performance: Built with Rust for optimal performance

Configuration
The agent uses agent/uomi.config.json for configuration:
json{
  "local_file_path": "../agent-template/src/request_input_file_example.txt",
  "api": {
      "timeout_ms": 30000,
      "retry_attempts": 3,
      "headers": {
          "Content-Type": "application/json",
          "Accept": "application/json",
          "User-Agent": "UOMI-Client/1.0"
      }
  },
  "models": {
      "1": {
          "name": "Qwen/Qwen2.5-32B-Instruct-GPTQ-Int4"
      },
      "2": {
          "name": "gpt-3.5-turbo",
          "url": "https://api.openai.com/v1/chat/completions",
          "api_key": "YOUR_OPENAI_API_KEY"
      }
  },
  "ipfs": {
      "gateway": "https://ipfs.io/ipfs",
      "timeout_ms": 10000
  }
}
Environment Setup
For security, store your OpenAI API key as an environment variable:
bashexport OPENAI_API_KEY="your_actual_api_key_here"
Then reference it in your configuration or load it at runtime.
Installation
Prerequisites

Rust (latest stable version)
Node.js and npm
WASP framework
Access to UOMI network

Build Instructions
bash# Clone the repository
git clone https://github.com/Ace-king2/ace-punk-nft-agent.git
cd ace-punk-nft-agent

# Install dependencies
npm install

# Build the WebAssembly module
cargo build --target wasm32-unknown-unknown --release

# Run the agent
wasp-cli run
Usage

Configure the Agent: Update agent/uomi.config.json with your API keys and preferences
Prepare Input: Structure your NFT generation requests according to the input format
Run Generation: Execute the agent to generate cyberpunk NFT metadata
Process Output: Retrieve formatted ERC-721 compatible metadata ready for minting

Project Structure
ace-punk-nft-agent/
├── agent/
│   ├── uomi.config.json    # Configuration file
│   └── src/                # Agent source code
├── src/                    # Main application source
├── Cargo.toml             # Rust dependencies
├── package.json           # Node.js dependencies
└── README.md             # This file
API Models
The agent supports multiple AI models:

Qwen/Qwen2.5-32B-Instruct-GPTQ-Int4: High-performance quantized model
GPT-3.5-Turbo: OpenAI's efficient model for general tasks

IPFS Integration
Built-in IPFS support for decentralized storage:

Default gateway: https://ipfs.io/ipfs
Configurable timeout and retry mechanisms
Ready for metadata and asset storage

Development
Hot Reloading
The agent supports hot-reloading for rapid development:
bashwasp-cli dev --watch
Testing
bashcargo test
npm test
Contributing

Fork the repository
Create your feature branch (git checkout -b feature/amazing-feature)
Commit your changes (git commit -m 'Add some amazing feature')
Push to the branch (git push origin feature/amazing-feature)
Open a Pull Request

Security

Never commit API keys or sensitive credentials
Use environment variables for secret management
Regularly rotate API keys
Keep dependencies updated

License
This project is licensed under the MIT License - see the LICENSE file for details.
Acknowledgments

UOMI Network for providing the unified API framework
WASP framework for WebAssembly support
The Rust and WebAssembly communities

