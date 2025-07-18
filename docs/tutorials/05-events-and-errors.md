# Tutorial 5: Events and Error Handling in ink!

## Overview

Welcome to one of the most crucial aspects of ink! smart contract development! Events and error handling are the foundation of building reliable, debuggable, and user-friendly smart contracts. Think of events as your contract's way of communicating what's happening to the outside world, while error handling ensures your contract behaves predictably when things go wrong.

In this tutorial, you'll learn how to implement robust error handling patterns and design effective event systems that make your contracts production-ready. By the end, you'll understand how to create contracts that not only work correctly but also provide excellent user experiences and are easy to debug and monitor.

![ink! Smart Contract Architecture: Events and Error Handling Flow](https://ppl-ai-code-interpreter-files.s3.amazonaws.com/web/direct-files/f37306cd4f009282d653387abe549f46/efc30a35-2d26-4202-b3f0-be331cd104e8/f312df76.png)

ink! Smart Contract Architecture: Events and Error Handling Flow

## Learning Objectives

By completing this tutorial, you will be able to:

- **Implement custom error types** that provide clear, actionable feedback to users
- **Design event structures** that efficiently communicate contract state changes
- **Handle errors gracefully** without breaking contract execution
- **Use events for contract monitoring** and debugging in production environments
- **Apply best practices** for both error handling and event design
- **Test error scenarios** thoroughly to ensure contract reliability


## Prerequisites

Before diving into this tutorial, make sure you have:

- **Completed Tutorial 4** or have equivalent experience with basic ink! contract development
- **Understanding of Rust error handling** including `Result<T, E>` types and the `?` operator
- **Familiarity with ink! basics** such as storage, constructors, and message functions
- **A development environment** set up with `cargo-contract` and a local Substrate node

If you're new to Rust error handling, consider reviewing the Rust Book's chapter on error handling before proceeding.

## Understanding ink! Architecture for Events and Errors

Before we dive into implementation details, it's important to understand how events and errors fit into the overall ink! contract architecture. Events are emitted from your contract to communicate state changes to external observers, while errors help your contract handle unexpected situations gracefully.

In ink!, both events and errors are first-class citizens that integrate seamlessly with the Substrate runtime and can be easily queried and handled by client applications.

## Error Handling Fundamentals

### What Are Custom Error Types?

Custom error types in ink! are enums that define specific failure conditions your contract might encounter. Instead of using generic error messages, custom errors provide structured, meaningful information about what went wrong and why.

```rust
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum VotingError {
    /// Voting period has ended
    VotingClosed,
    /// User has already voted
    AlreadyVoted,
    /// Invalid proposal ID provided
    InvalidProposal,
    /// Not authorized to perform this action
    NotAuthorized,
}
```


### The Result Type in ink!

The `Result<T, E>` type is your primary tool for error handling in ink!. It represents either a successful value (`Ok(T)`) or an error (`Err(E)`). Every message function that can fail should return a `Result`.

```rust
#[ink(message)]
pub fn vote(&mut self, proposal_id: u32) -> Result<(), VotingError> {
    // Implementation that can return either Ok(()) or Err(VotingError)
}
```


### Error Handling Patterns

![ink! Error Handling Patterns Comparison](https://ppl-ai-code-interpreter-files.s3.amazonaws.com/web/direct-files/f37306cd4f009282d653387abe549f46/f3bdb0b2-7b00-493f-b3ef-acec41019efc/02f257a6.png)

ink! Error Handling Patterns Comparison

Understanding different error handling patterns helps you choose the right approach for each situation in your contracts.

## Event System Fundamentals

### What Are Events in ink!?

Events in ink! are structured messages that your contract emits to communicate important state changes or actions to the outside world. They're stored on the blockchain and can be queried by client applications, making them perfect for user interfaces, monitoring systems, and audit trails.

### Defining Events

Since ink! version 5.0, events can be defined independently of contracts, making them reusable across multiple contracts. Here's how to define an event:

```rust
#[ink(event)]
pub struct VoteCast {
    #[ink(topic)]
    voter: AccountId,
    #[ink(topic)]
    proposal_id: u32,
    vote: bool,
}
```


### Event Topics and Indexing

The `#[ink(topic)]` attribute makes fields searchable. Topics are like database indexes - they allow efficient filtering and searching of events. Use topics for fields that you might want to search by, such as user addresses or specific IDs.

**Guidelines for Topics:**

- Use topics for fields you'll frequently search by
- Limit topics to essential search criteria (gas costs increase with more topics)
- AccountId fields are excellent topic candidates
- Avoid making large data fields topics


## Hands-On Example: Building a Voting Contract

Let's build a complete voting contract that demonstrates both robust error handling and effective event usage. This practical example will show you how these concepts work together in a real contract.

The voting contract demonstrates several key concepts:

### Error Handling in Action

```rust
#[ink(message)]
pub fn vote(&mut self, proposal_id: u32, vote: bool) -> Result<(), VotingError> {
    let caller = self.env().caller();

    // Multiple error checks with specific error types
    if !self.is_open {
        return Err(VotingError::VotingClosed);
    }

    if self.has_voted.get(&caller).unwrap_or(false) {
        return Err(VotingError::AlreadyVoted);
    }

    if proposal_id >= self.proposals.len() as u32 {
        return Err(VotingError::InvalidProposal);
    }

    // Process the vote and emit event
    // ... implementation
    Ok(())
}
```


### Event Emission Patterns

```rust
// Emit events after successful operations
self.env().emit_event(VoteCast {
    voter: caller,
    proposal_id,
    vote,
});
```


## Advanced Error Handling Techniques

### Error Propagation with the `?` Operator

The `?` operator provides a clean way to propagate errors up the call stack:

```rust
#[ink(message)]
pub fn complex_operation(&mut self) -> Result<u32, VotingError> {
    // If this fails, the error is automatically returned
    self.validate_caller()?;
    
    // Continue with the operation
    let result = self.perform_calculation()?;
    Ok(result)
}
```


### Error Conversion and the From Trait

When working with multiple error types, implement the `From` trait for automatic error conversion:

```rust
impl From<ArithmeticError> for VotingError {
    fn from(_: ArithmeticError) -> Self {
        VotingError::CalculationFailed
    }
}
```


### When to Panic vs Return Errors

**Use `panic!` for:**

- Programming errors that should never happen
- Invariant violations
- Critical system failures

**Use `Result<T, E>` for:**

- User input validation
- Business logic violations
- Recoverable errors
- Expected failure conditions


## Event Best Practices

### Designing Informative Events

Good events tell a complete story about what happened:

```rust
#[ink(event)]
pub struct ProposalCreated {
    #[ink(topic)]
    creator: AccountId,
    #[ink(topic)]
    proposal_id: u32,
    description: Vec<u8>,
    #[ink(topic)]
    created_at: u64,
}
```


### Gas Cost Considerations

Events consume gas, so design them efficiently:

- **Minimize event size**: Only include essential data
- **Limit topics**: Each topic increases gas costs
- **Use appropriate data types**: Smaller types cost less gas
- **Consider event frequency**: High-frequency events should be optimized


### Privacy Considerations

Remember that events are public and stored on-chain:

- **Avoid sensitive data**: Never emit private information
- **Use hashes for privacy**: Hash sensitive data before emitting
- **Consider regulatory requirements**: Some data may have legal implications


## Testing Events and Errors

### Unit Testing Error Conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn duplicate_vote_fails() {
        let mut voting = Voting::new();
        voting.add_proposal(b"Test proposal".to_vec()).unwrap();
        
        // First vote should succeed
        voting.vote(0, true).unwrap();
        
        // Second vote should fail
        let result = voting.vote(0, true);
        assert_eq!(result, Err(VotingError::AlreadyVoted));
    }
}
```


### Event Testing Patterns

```rust
#[ink::test]
fn vote_emits_event() {
    let mut voting = Voting::new();
    voting.add_proposal(b"Test".to_vec()).unwrap();
    
    // Use ink!'s test environment to capture events
    let result = voting.vote(0, true);
    assert_eq!(result, Ok(()));
    
    // In a real test, you'd check the emitted events
}
```


## Debugging and Monitoring

### Using Events for Debugging

Events provide a timeline of contract execution:

```rust
#[ink(event)]
pub struct DebugInfo {
    #[ink(topic)]
    function_name: Vec<u8>,
    parameters: Vec<u8>,
    #[ink(topic)]
    timestamp: u64,
}
```


### Error Logging Strategies

Combine events and errors for comprehensive logging:

```rust
#[ink(message)]
pub fn monitored_function(&mut self) -> Result<(), VotingError> {
    // Emit debug event at start
    self.env().emit_event(FunctionCalled {
        name: b"monitored_function".to_vec(),
        caller: self.env().caller(),
    });

    // Perform operation with error handling
    match self.risky_operation() {
        Ok(result) => {
            self.env().emit_event(OperationSucceeded { result });
            Ok(())
        },
        Err(error) => {
            self.env().emit_event(OperationFailed { 
                error: format!("{:?}", error).into_bytes() 
            });
            Err(error)
        }
    }
}
```


## Integration with Frontend Applications

### Handling Errors in Client Code

```javascript
// Example using polkadot.js
try {
    const result = await contract.tx.vote(proposalId, true)
        .signAndSend(account);
    console.log('Vote cast successfully');
} catch (error) {
    if (error.message.includes('AlreadyVoted')) {
        alert('You have already voted on this proposal');
    } else if (error.message.includes('VotingClosed')) {
        alert('Voting has ended for this proposal');
    } else {
        alert('An unexpected error occurred');
    }
}
```


### Listening for Events

```javascript
// Subscribe to events
contract.events.VoteCast((error, event) => {
    if (error) {
        console.error('Event error:', error);
        return;
    }
    
    console.log('Vote cast:', {
        voter: event.returnValues.voter,
        proposalId: event.returnValues.proposal_id,
        vote: event.returnValues.vote
    });
    
    // Update UI accordingly
    updateVoteDisplay(event.returnValues);
});
```


## Performance and Gas Optimization

### Optimizing Error Handling

- **Use specific error types**: Avoid generic errors that require additional context
- **Minimize error data**: Keep error variants simple
- **Early validation**: Check for errors before expensive operations


### Event Optimization Strategies

- **Batch related events**: Emit multiple pieces of information in one event when logical
- **Use efficient data types**: Prefer primitive types over complex structures
- **Consider event necessity**: Not every state change needs an event


## Common Pitfalls and How to Avoid Them

### Error Handling Mistakes

1. **Ignoring errors**: Always handle or propagate errors appropriately
2. **Generic error messages**: Provide specific, actionable error information
3. **Inconsistent error types**: Use a unified error handling strategy across your contract

### Event Design Mistakes

1. **Too many topics**: Limit topics to essential searchable fields
2. **Sensitive data in events**: Never emit private or sensitive information
3. **Inconsistent event naming**: Use clear, consistent naming conventions

## Real-World Applications

### Audit Trail Implementation

```rust
#[ink(event)]
pub struct AdminAction {
    #[ink(topic)]
    admin: AccountId,
    #[ink(topic)]
    action_type: Vec<u8>,
    target: Option<AccountId>,
    #[ink(topic)]
    timestamp: u64,
}
```


### User Notification Systems

Events can drive real-time notifications in your dApp:

```rust
#[ink(event)]
pub struct UserNotification {
    #[ink(topic)]
    recipient: AccountId,
    message: Vec<u8>,
    priority: u8,
    #[ink(topic)]
    category: Vec<u8>,
}
```


## Testing Strategies

### Comprehensive Error Testing

Create test cases for every possible error condition:

```rust
#[cfg(test)]
mod error_tests {
    use super::*;

    #[ink::test]
    fn test_all_error_conditions() {
        let mut contract = MyContract::new();
        
        // Test each error variant
        assert_eq!(
            contract.invalid_operation(),
            Err(MyError::InvalidInput)
        );
        
        // Test error propagation
        assert_eq!(
            contract.complex_operation(),
            Err(MyError::PrerequisiteNotMet)
        );
    }
}
```


### Event Testing Framework

```rust
#[ink::test]
fn events_are_emitted_correctly() {
    let mut contract = MyContract::new();
    
    // Perform operation that should emit events
    contract.do_something().unwrap();
    
    // In a complete test framework, you would:
    // 1. Capture emitted events
    // 2. Verify event data
    // 3. Check event topics
    // 4. Validate event ordering
}
```


## Next Steps and Advanced Topics

Congratulations! You now have a solid foundation in ink! events and error handling. Here are some advanced topics to explore next:

### Cross-Contract Communication

- Learn how to handle errors from other contracts
- Understand event coordination between contracts
- Implement error conversion for cross-contract calls


### Advanced Event Patterns

- Event versioning strategies
- Complex event filtering
- Event-driven architecture patterns


### Production Considerations

- Monitoring and alerting systems
- Error analytics and reporting
- Performance optimization techniques


### Recommended Learning Path

1. **Tutorial 6: Contract Interactions** - Learn how to handle errors and events when contracts interact with each other
2. **Advanced Error Handling Patterns** - Explore complex error scenarios and advanced propagation techniques
3. **Event-Driven Architecture** - Design systems that respond to contract events
4. **Production Deployment** - Learn monitoring and debugging strategies for live contracts

## Summary

Events and error handling are fundamental to building professional-grade ink! smart contracts. Events provide transparency and enable rich user experiences, while proper error handling ensures your contracts behave predictably and provide clear feedback when things go wrong.

The key takeaways from this tutorial:

- **Custom error types** provide better user experience than generic errors
- **Events should be designed for efficiency** and searchability
- **The `?` operator** simplifies error propagation
- **Testing both success and failure paths** is crucial for reliable contracts
- **Events and errors work together** to create observable, debuggable contracts

By applying these patterns and best practices, you'll create contracts that are not only functional but also maintainable, debuggable, and user-friendly. The voting contract example demonstrates how these concepts work together in a real-world application, providing a template you can adapt for your own projects.

Remember that good error handling and event design are investments in your contract's long-term success. They make debugging easier, user experiences better, and maintenance more manageable as your contracts evolve and scale.
