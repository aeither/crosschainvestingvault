# Tutorial 4: Your First Steps with ink! Storage and State Management

**Welcome to the world of ink! smart contracts!** If you've never written a smart contract before, don't worry – this tutorial will guide you through everything step by step. Think of this as your friendly introduction to how data works in ink! contracts.

## What You'll Learn Today

By the end of this tutorial, you'll understand:

- What ink! storage actually is (and why it's different from regular programming)
- How to store simple data like numbers and text
- How to avoid the most common beginner mistakes
- How to build a real contact book contract from scratch


## Before We Start

**Don't panic if you're new to this!** This tutorial assumes you know:

- Basic Rust syntax (variables, functions, structs)
- What a smart contract is (a program that runs on a blockchain)

If you're completely new to Rust, that's okay too – we'll explain everything as we go.

## What is ink! and Why Should You Care?

**ink!** is like a special version of Rust designed for writing smart contracts. Think of it as Rust's blockchain-savvy cousin. Here's what makes it special:

- **Safe by default**: Built on Rust's safety guarantees
- **Familiar syntax**: If you know Rust, you're already halfway there
- **Polkadot ecosystem**: Works with dozens of blockchains
- **Gas efficient**: Optimized for low transaction costs

The key difference from regular programming? **Everything costs money**. Every piece of data you store, every calculation you make – it all costs "gas" (transaction fees). That's why understanding storage is crucial!

## Understanding Storage: Your Contract's Memory

In regular programming, you can create variables and they just... exist. In smart contracts, it's different. **Storage is permanent data that lives on the blockchain forever**.

Think of contract storage like a filing cabinet:

- **Expensive to use**: Every file you store costs money
- **Permanent**: Once stored, it stays there forever
- **Public**: Anyone can read what's in there (unless encrypted)
- **Organized**: You need to plan how to organize your data


## The Building Blocks: Storage Types for Beginners

Let's start with the simplest types and build up from there.

![ink! Storage Types: A beginner-friendly comparison of different storage options in ink! smart contracts](https://ppl-ai-code-interpreter-files.s3.amazonaws.com/web/direct-files/82c455f3cf9dbad946cf99216e08e938/939f980a-9af8-4c1d-9ab4-d8837559b524/891bd28e.png)

ink! Storage Types: A beginner-friendly comparison of different storage options in ink! smart contracts

### 1. Simple Values: Your Best Friends

These are the easiest and most efficient storage types:

```rust
#[ink(storage)]
pub struct SimpleContract {
    // Store a number (like a counter)
    count: u32,
    // Store true/false values  
    is_active: bool,
    // Store the contract owner
    owner: AccountId,
}
```

**Why these are great for beginners:**

- **Low cost**: Very cheap to store and read
- **Easy to understand**: They work just like regular variables
- **Fast**: No complex operations needed


### 2. Text and Lists: When You Need More

Sometimes you need to store text or lists:

```rust
#[ink(storage)]
pub struct TextContract {
    // Store text (like a name or message)
    message: String,
    // Store a list of numbers
    numbers: Vec<u32>,
}
```

**Important beginner tip:** Be careful with `Vec`! If it grows too large, it can break your contract. We'll show you better alternatives later.

### 3. Mappings: The Smart Contract Database

This is where ink! gets really powerful. Think of `Mapping` like a dictionary or lookup table:

```rust
#[ink(storage)]
pub struct BalanceContract {
    // Map each user to their balance
    balances: Mapping<AccountId, u128>,
}
```

**Why mappings are amazing:**

- **Scalable**: Can store millions of entries efficiently
- **Fast lookups**: Finding data is quick and cheap
- **Perfect for user data**: Ideal for balances, permissions, etc.


## Storage Layouts: Packed vs Unpacked (The Simple Version)

Don't let the technical terms scare you! This is actually simple:

![Visual comparison of packed vs unpacked storage layouts in ink! smart contracts](https://user-gen-media-assets.s3.amazonaws.com/gpt4o_images/017ac7d7-f901-4898-b86d-649fe5629616.png)

Visual comparison of packed vs unpacked storage layouts in ink! smart contracts

**Packed Storage (Default):**

- All your data is stored together in one "box"
- Cheaper for small amounts of data
- Every function call loads ALL your data

**Unpacked Storage:**

- Each piece of data gets its own "box"
- More expensive but more efficient for large data
- Functions only load the data they need

**For beginners:** Start with packed storage (the default). Only worry about unpacked storage when your contracts get complex.

## Your First Real Contract: A Contact Book

Let's build something practical! We'll create a contact book that stores names and phone numbers.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod contact_book {
    use ink::storage::Mapping;
    use ink::prelude::string::String;

    /// A contact with name and phone
    #[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Contact {
        name: String,
        phone: String,
    }

    /// Our contact book storage
    #[ink(storage)]
    pub struct ContactBook {
        /// Who owns this contact book
        owner: AccountId,
        /// All contacts stored here
        contacts: Mapping<u32, Contact>,
        /// Counter for contact IDs
        next_id: u32,
    }

    impl ContactBook {
        /// Create a new contact book
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                contacts: Mapping::default(),
                next_id: 1,
            }
        }

        /// Add a new contact
        #[ink(message)]
        pub fn add_contact(&mut self, name: String, phone: String) -> Result<u32, String> {
            // Only the owner can add contacts
            if self.env().caller() != self.owner {
                return Err("Only owner can add contacts".to_string());
            }

            // Create the contact
            let contact = Contact { name, phone };
            let id = self.next_id;
            
            // Store it
            self.contacts.insert(id, &contact);
            self.next_id += 1;
            
            Ok(id)
        }

        /// Get a contact by ID
        #[ink(message)]
        pub fn get_contact(&self, id: u32) -> Option<Contact> {
            self.contacts.get(&id)
        }

        /// Count total contacts
        #[ink(message)]
        pub fn contact_count(&self) -> u32 {
            self.next_id - 1
        }
    }
}
```

**What's happening here?**

1. We define a `Contact` struct to hold name and phone
2. Our storage has an owner, a mapping of contacts, and an ID counter
3. We can add contacts (with permission check)
4. We can retrieve contacts by ID
5. We can count total contacts

## Beginner Mistakes to Avoid

Everyone makes mistakes when learning. Here are the most common ones:

### The \#1 Mistake: Forgetting Error Handling

**Never do this:**

```rust
#[ink(message)]
pub fn bad_function(&self, user: AccountId) -> u128 {
    self.balances.get(&user).unwrap() // ❌ This can crash!
}
```

**Always do this:**

```rust
#[ink(message)]
pub fn good_function(&self, user: AccountId) -> u128 {
    self.balances.get(&user).unwrap_or(0) // ✅ Safe default
}
```


### The \#2 Mistake: Growing Vectors Forever

**Never do this:**

```rust
#[ink(storage)]
pub struct BadContract {
    all_users: Vec<AccountId>, // ❌ Can grow too large
}
```

**Always do this:**

```rust
#[ink(storage)]
pub struct GoodContract {
    users: Mapping<AccountId, bool>, // ✅ Scalable
    user_count: u32,
}
```


## Gas Optimization: Making Your Contract Efficient

**Gas** is the fuel that powers your contract. Here's how to use less:

### 1. Minimize Storage Reads

```rust
// ❌ Bad: Multiple reads
let balance1 = self.balances.get(&user).unwrap_or(0);
let balance2 = self.balances.get(&user).unwrap_or(0);

// ✅ Good: Read once
let balance = self.balances.get(&user).unwrap_or(0);
let doubled = balance * 2;
```


### 2. Use Appropriate Data Types

```rust
// ✅ Good choices for common use cases
count: u32,           // For counters
is_active: bool,      // For flags
owner: AccountId,     // For addresses
balances: Mapping<AccountId, u128>, // For token balances
```


### 3. Batch Operations When Possible

Instead of multiple function calls, try to do multiple operations in one call.

## Testing Your Contract

Always test your contract! Here's a simple test for our contact book:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn adding_contact_works() {
        let mut contact_book = ContactBook::new();
        
        let result = contact_book.add_contact(
            "Alice".to_string(),
            "123-456-7890".to_string()
        );
        
        assert!(result.is_ok());
        assert_eq!(contact_book.contact_count(), 1);
    }

    #[ink::test]
    fn getting_contact_works() {
        let mut contact_book = ContactBook::new();
        
        let id = contact_book.add_contact(
            "Bob".to_string(),
            "987-654-3210".to_string()
        ).unwrap();
        
        let contact = contact_book.get_contact(id).unwrap();
        assert_eq!(contact.name, "Bob");
    }
}
```


## State Management: Keeping Your Data Consistent

**State management** is a fancy term for "making sure your data makes sense." Here are the basics:

### 1. Validate Input

```rust
#[ink(message)]
pub fn set_age(&mut self, age: u32) -> Result<(), String> {
    if age > 150 {
        return Err("Age seems too high".to_string());
    }
    self.age = age;
    Ok(())
}
```


### 2. Check Permissions

```rust
#[ink(message)]
pub fn admin_function(&mut self) -> Result<(), String> {
    if self.env().caller() != self.owner {
        return Err("Only owner can do this".to_string());
    }
    // ... do admin stuff
    Ok(())
}
```


### 3. Use Events for Important Changes

```rust
#[ink(event)]
pub struct ContactAdded {
    id: u32,
    name: String,
}

#[ink(message)]
pub fn add_contact(&mut self, name: String, phone: String) -> Result<u32, String> {
    // ... add contact logic ...
    
    // Emit event
    self.env().emit_event(ContactAdded {
        id: contact_id,
        name: name.clone(),
    });
    
    Ok(contact_id)
}
```


## Common Patterns You'll Use

### 1. Owner-Only Functions

```rust
fn only_owner(&self) -> Result<(), String> {
    if self.env().caller() != self.owner {
        return Err("Only owner allowed".to_string());
    }
    Ok(())
}
```


### 2. Safe Math Operations

```rust
fn safe_add(&self, a: u128, b: u128) -> Result<u128, String> {
    a.checked_add(b).ok_or("Overflow error".to_string())
}
```


### 3. Existence Checks

```rust
#[ink(message)]
pub fn update_contact(&mut self, id: u32, name: String) -> Result<(), String> {
    if self.contacts.get(&id).is_none() {
        return Err("Contact not found".to_string());
    }
    // ... update logic
    Ok(())
}
```


## Next Steps: Where to Go From Here

Congratulations! You now understand the basics of ink! storage and state management. Here's what to explore next:

### Immediate Next Steps:

1. **Practice**: Build your own version of the contact book
2. **Experiment**: Try adding update and delete functions
3. **Test**: Write more comprehensive tests

### Advanced Topics (For Later):

- Events and error handling
- Cross-contract calls
- Upgradeability patterns
- Security best practices


### Resources:

- [Official ink! Documentation](https://use.ink/)
- [Polkadot Developer Hub](https://developers.polkadot.network/)
- [ink! Examples Repository](https://github.com/use-ink/ink-examples)


## Summary: Key Takeaways

- **Storage costs money**: Every piece of data you store has a cost
- **Start simple**: Use basic types first, then move to complex ones
- **Use Mapping for user data**: It's scalable and efficient
- **Always handle errors**: Use `Result` types instead of panics
- **Test everything**: Write tests for both success and failure cases
- **Keep learning**: ink! is powerful, but start with the basics

Remember: **Every expert was once a beginner**. Don't be afraid to make mistakes – that's how you learn! The ink! community is friendly and helpful, so don't hesitate to ask questions.

Happy coding, and welcome to the world of ink! smart contracts! 🦑