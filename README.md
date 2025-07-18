
# Cross-Chain Vesting Vault

A comprehensive multi-chain vesting vault implementation built with ink! smart contracts and Polkadot XCM, featuring cross-chain token transfers, asset precompile integration, and mobile-first development.

## 🏗️ Architecture

### Core Components

- **Vesting Vault Contract** (`vesting_vault/`) - ink! smart contract with XCM integration
- **Shuttle Backend** (`shuttle-backend/`) - Rust API server for XCM orchestration
- **Documentation** (`docs/`) - Comprehensive ink! tutorial series
- **Marketing** (`MARKETING.md`) - Uniswap Unicorn Hunt campaign

## 🚀 Features

- **Cross-Chain Vesting**: Lock tokens with automatic cross-chain claiming via XCM
- **Asset Precompile Integration**: Support for multiple asset types (DOT, USDT, etc.)
- **Emergency Mode**: Circuit breaker for emergency withdrawals
- **Mobile-Ready**: Flutter integration for mobile dApp development
- **Comprehensive Testing**: DRink! integration tests

## 📖 Documentation

### ink! Tutorial Series
Complete 12-part tutorial series covering:

1. **Getting Started**
   - [Introduction to ink!](docs/tutorials/01-introduction-to-ink.md)
   - [Development Environment](docs/tutorials/02-development-environment.md)
   - [First Contract](docs/tutorials/03-first-contract.md)

2. **Core Concepts**
   - [Storage and State](docs/tutorials/04-storage-and-state.md)
   - [Events and Errors](docs/tutorials/05-events-and-errors.md)
   - [Contract Interactions](docs/tutorials/06-contract-interactions.md)

3. **Advanced Features**
   - [Assets Precompile](docs/tutorials/07-assets-precompile.md)
   - [XCM Integration](docs/tutorials/08-xcm-integration.md)
   - [Account Abstraction](docs/tutorials/09-account-abstraction.md)

4. **Testing & Deployment**
   - [DRink! Testing](docs/tutorials/10-testing-with-drink.md)
   - [Shuttle Deployment](docs/tutorials/11-shuttle-deployment.md)

5. **Mobile Development**
   - [Mobile Integration](docs/tutorials/12-mobile-integration.md)

## 🛠️ Quick Start

### Prerequisites
- Rust toolchain
- cargo-contract
- Node.js and npm
- Shuttle CLI

### Deploy Shuttle Backend
```bash
cd shuttle-backend
shuttle deploy
```

### Build ink! Contract
```bash
cd vesting_vault
cargo contract build
```

### Run Tests
```bash
cargo test
```

### Test API Endpoints
```bash
curl -X POST http://localhost:8000/xcm/claim \
  -H "Content-Type: application/json" \
  -d '{"user_account":"Alice","amount":1000,"destination_parachain":"AssetHub"}'
```

## 🎯 Hackathon Tracks

### Ink Track
- **Vesting Vault Contract**: Advanced ink! implementation with XCM
- **Shuttle Backend**: Rust API for cross-chain orchestration
- **Documentation**: Complete tutorial series for ink! development

### Marketing Track
- **Uniswap Unicorn Hunt**: AR-powered street art scavenger hunt
- **Guerrilla Marketing**: Real-world DeFi engagement campaign
- **Global Reach**: Multi-city activation strategy

## 🔧 Technical Highlights

- **XCM Integration**: Native cross-chain messaging for token transfers
- **Asset Precompile**: Direct integration with Polkadot's asset system
- **Circuit Breaker**: Emergency mode for risk management
- **Mobile First**: Flutter-ready architecture
- **Production Ready**: Comprehensive testing and deployment automation

## 📱 Mobile Integration

Flutter integration enables:
- Native mobile wallet interactions
- Cross-chain transaction management
- Real-time vesting status updates
- Seamless user experience

## 🌐 Cross-Chain Features

- **Multi-Parachain Support**: Asset transfers across Polkadot ecosystem
- **XCM Messaging**: Reliable cross-chain communication
- **Asset Diversity**: Support for DOT, USDT, and custom assets
- **Automated Claiming**: Time-locked vesting with automatic releases

## 📊 API Endpoints

- `POST /xcm/claim` - Initiate cross-chain claim
- `POST /vesting/info` - Get vesting information
- `POST /simulate/deposit` - Simulate token deposit

## 🔐 Security Features

- **Time-locked Vesting**: Enforced lock periods
- **Emergency Unlock**: Admin-controlled circuit breaker
- **Cross-chain Validation**: Secure XCM message verification
- **Asset Validation**: Supported asset whitelist

## 📈 Future Roadmap

- Enhanced mobile features
- Additional parachain integrations
- Advanced governance mechanisms
- Expanded asset support

---

Built for the ink! Bounties program, demonstrating production-ready cross-chain vesting with comprehensive documentation and mobile integration.