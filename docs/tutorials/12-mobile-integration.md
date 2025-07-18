# Tutorial 12: Building Mobile Apps with ink! Contracts

## Overview
Create mobile applications that interact with ink! smart contracts, covering both native mobile development and web-based approaches. This tutorial targets the 

## Learning Objectives
- Integrate ink! contracts with mobile apps
- Implement wallet connectivity
- Handle mobile-specific UX patterns
- Deploy mobile applications

## Prerequisites
- Completed Tutorial 11
- Basic mobile development knowledge
- Understanding of REST APIs

## Topics Covered

### Mobile Development Approaches
- Native apps (React Native, Flutter)
- Web-based apps (PWA)
- Hybrid solutions
- Platform-specific considerations

### Wallet Integration
- Mobile wallet connectivity
- WalletConnect protocol
- In-app wallet solutions
- Biometric authentication

### Contract Interaction
- API layer design
- Transaction signing
- Event listening
- Offline functionality

### User Experience
- Mobile-first design
- Touch-friendly interfaces
- Network handling
- Error states

### Security Considerations
- Key management
- Secure storage
- Network security
- Privacy protection

## Hands-On Activities
1. Set up mobile development environment
2. Create React Native app
3. Integrate with ink! contract
4. Implement wallet connectivity
5. Deploy to app stores

## Code Examples
```typescript
// React Native integration
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

// Flutter integration
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

### Mobile Architecture
- MVVM pattern
- State management
- Offline-first design
- Caching strategies

### Platform-Specific Features
- Push notifications
- Background sync
- App lifecycle handling
- Platform APIs

### Testing Mobile Apps
- Unit testing
- Integration testing
- UI testing
- Device testing

## Real-World Applications
- DeFi mobile apps
- Gaming applications
- Social platforms
- Commerce solutions

### Deployment Strategy
- App store guidelines
- Beta testing
- Update mechanisms
- Analytics integration

### Performance Optimization
- Network optimization
- Battery efficiency
- Memory management
- Loading strategies

## Bounty Submission Guidelines
- Innovative mobile features
- User experience excellence
- Performance optimization
- Cross-platform compatibility

## Advanced Topics
- Cross-platform development
- Offline synchronization
- Push notification integration
- Advanced security features
