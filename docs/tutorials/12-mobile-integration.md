# Tutorial 12: Building Mobile Apps with ink! Contracts

## Overview

This tutorial demonstrates how to build mobile applications that interact with ink! smart contracts on the Polkadot ecosystem. You’ll learn both native (React Native, Flutter) and web-based (PWA) approaches, focusing on real-world integration and best practices for user experience, security, wallet interaction, and deployment.

## Learning Objectives

- Integrate ink! contracts with mobile apps
- Implement secure wallet connectivity
- Apply mobile-specific UX solutions
- Deploy applications to app stores


## Prerequisites

- Completion of previous ink! smart contract tutorials
- Basic experience with mobile development (React Native or Flutter recommended)
- Understanding of REST APIs and asynchronous programming


## Mobile Development Approaches

- **Native Apps:** Use frameworks like React Native (JavaScript/TypeScript) or Flutter (Dart) for cross-platform development.
- **Web-Based Apps (PWA):** Develop progressive web apps for broad reach.
- **Hybrid Solutions:** Combine web and native code for specific requirements.
- **Platform-Specific Considerations:** Handle differences in navigation, permissions, and UI/UX between iOS and Android.


## Wallet Integration

- **Mobile Wallet Connectivity:** Allow users to connect existing mobile wallets (e.g., Polkadot.js, Talisman).
- **WalletConnect Protocol:** Support remote signing, linking users' wallets to your app securely.
- **In-App Wallet Solutions:** Implement lightweight wallets directly in the app.
- **Biometric Authentication:** Enhance security with Face ID, Touch ID, or comparable systems.


## Contract Interaction

- **API Layer Design:** Create modular interfaces between your mobile app and smart contracts.
- **Transaction Signing:** Ensure secure signing and submission of transactions from mobile devices.
- **Event Listening:** Use WebSockets or polling to track contract events.
- **Offline Functionality:** Cache data and transactions for improved reliability when offline.


## User Experience

- **Mobile-First Design:** Optimize layouts and controls for smaller touch screens.
- **Touch-Friendly Interfaces:** Use large buttons, responsive lists, and gestures.
- **Network Handling:** Display loading states, manage connection drops, and provide retry options.
- **Error States:** Guide users with meaningful error messages and recovery paths.


## Security Considerations

- **Key Management:** Consider secure, encrypted storage or hardware-based wallets.
- **Secure Storage:** Store sensitive data in protected device storage (Keychain/iOS, Keystore/Android).
- **Network Security:** Use HTTPS and validation for server communications.
- **Privacy Protection:** Minimize personal data collection and comply with GDPR if deploying in Europe.


## Hands-On Activities

1. **Set Up the Mobile Development Environment:** Install Node.js, React Native, or Flutter SDK.
2. **Create a Base Mobile App:** Scaffold a new app using `npx react-native init` or `flutter create`.
3. **Integrate with ink! Contract:** Set up Polkadot.js or compatible libraries for blockchain interaction.
4. **Implement Wallet Connectivity:** Use WalletConnect or build in-app wallet logic.
5. **Deploy to App Stores:** Prepare app assets, conduct testing, and publish.

## Code Examples

**React Native Integration**

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';

class ContractService {
    private api: ApiPromise;
    private contract: ContractPromise;
    
    async connect() {
        const provider = new WsProvider('wss://rpc.polkadot.io');
        this.api = await ApiPromise.create({ provider });
        this.contract = new ContractPromise(this.api, abi, contractAddress);
    }
    
    async callContract(method: string, args: any[]) {
        const { result, output } = await this.contract.query[method](
            senderAddress,
            { gasLimit: -1 },
            ...args
        ); 
        return output?.toHuman();
    }
}
```

**Flutter Integration**

```dart
class ContractService {
    static const String WS_URL = 'wss://rpc.polkadot.io';
    
    Future<Map<String, dynamic>> callContract(String method, List<dynamic> args) async {
        final response = await http.post(
            Uri.parse('$WS_URL/contract/call'),
            body: jsonEncode({
                'method': method,
                'args': args,
                'contract': contractAddress,
            }),
        );
        return jsonDecode(response.body);
    }
}
```


## Mobile Architecture

- Use **MVVM** or similar patterns for maintainability.
- Employ effective **state management** (Redux, Bloc, Provider).
- Favor **offline-first design** and **data caching** for robustness.


## Platform-Specific Features

- **Push Notifications:** Inform users of contract events or actions.
- **Background Sync:** Keep data fresh with background tasks.
- **App Lifecycle Handling:** Respond to foreground/background transitions.
- **Platform APIs:** Integrate with native device features (camera, location, etc.).


## Testing Mobile Apps

- **Unit Testing:** Test logic and contract interaction modules.
- **Integration Testing:** Ensure end-to-end flows work—contract calls, wallet actions.
- **UI Testing:** Use tools like Detox (React Native) or Flutter’s test suite.
- **Device Testing:** Cover a range of devices and OS versions for compatibility.


## Real-World Applications

- **DeFi apps:** On-chain wallets, swaps, or lending platforms.
- **Gaming:** Blockchain-based rewards and item ownership.
- **Social Platforms:** Token-based communities or NFT integrations.
- **Commerce:** Decentralized marketplaces and loyalty programs.


## Deployment Strategy

- Follow **app store guidelines** (Apple, Google) for blockchain apps.
- Conduct **beta testing** with TestFlight or Google Play Beta.
- Prepare for **update mechanisms** and user communications.
- Integrate **analytics** to monitor usage and spot issues early.

By following these guidelines and examples, you can successfully build robust, user-friendly mobile applications that unlock the full potential of ink! contracts in the Polkadot ecosystem.