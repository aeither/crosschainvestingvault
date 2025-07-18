
# Your First ink! Contract - Super Beginner's Guide

## What is ink!?

**ink!** is a programming language for writing smart contracts that run on **Polkadot** and **Substrate-based blockchains**. Think of it as a way to write programs that live on the blockchain using **Rust** - but with special features that make it perfect for smart contracts.

![ink! Smart Contract Architecture - From Rust to Blockchain](https://user-gen-media-assets.s3.amazonaws.com/gpt4o_images/9ac929d5-e6ea-490e-8614-3fdd4019dac0.png)

ink! Smart Contract Architecture - From Rust to Blockchain

## Why Start with ink!?

- **🔒 Safe \& Secure**: Built with Rust, which prevents many common programming bugs
- **⚡ Fast \& Efficient**: Compiles to WebAssembly for optimal performance
- **🚀 Easy to Learn**: If you know Rust, you're already halfway there
- **🌐 Future-Ready**: Works with the entire Polkadot ecosystem


## The Only 3 Things You Need to Know

Every ink! contract has exactly **three core components**:

### 1. **Storage** - Where your data lives

```rust
#[ink(storage)]
pub struct Counter {
    value: i32,
}
```


### 2. **Constructor** - How your contract starts

```rust
#[ink(constructor)]
pub fn new(init_value: i32) -> Self {
    Self { value: init_value }
}
```


### 3. **Messages** - What your contract can do

```rust
#[ink(message)]
pub fn increment(&mut self) {
    self.value += 1;
}
```


## Your Complete First Contract

Here's a working counter contract that you can copy and use:

```rust
#[ink::contract]
mod counter {
    #[ink(storage)]
    pub struct Counter {
        value: i32,
    }

    impl Counter {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn increment(&mut self) {
            self.value += 1;
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
    }
}
```


## Quick Start: Build Your First Contract

![ink! Development Workflow - From Idea to Deployment](https://user-gen-media-assets.s3.amazonaws.com/gpt4o_images/e20c3cc7-b68a-4de4-a3b3-e615dd5b0908.png)

ink! Development Workflow - From Idea to Deployment

### Step 1: Install Tools

```bash
# Install Rust first (from rustup.rs)
rustup component add rust-src

# Install the ink! contract tool
cargo install --force --locked cargo-contract
```


### Step 2: Create Your Project

```bash
cargo contract new my_counter
cd my_counter
```


### Step 3: Build Your Contract

```bash
cargo contract build
```

**That's it!** You now have a `.contract` file ready to deploy.

## Key Concepts to Remember

| Concept | What it does |
| :-- | :-- |
| `#[ink::contract]` | Marks your module as an ink! contract |
| `#[ink(storage)]` | Defines what data your contract stores |
| `#[ink(constructor)]` | Creates new instances of your contract |
| `#[ink(message)]` | Functions that users can call |
| `&mut self` | Changes contract state |
| `&self` | Only reads contract state |

## Test Your Contract

Add this simple test to make sure everything works:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn it_works() {
        let mut counter = Counter::new(5);
        assert_eq!(counter.get(), 5);
        counter.increment();
        assert_eq!(counter.get(), 6);
    }
}
```

Run tests with: `cargo test`

## Deploy Locally

1. **Install local node**: `cargo install contracts-node`
2. **Start node**: `substrate-contracts-node`
3. **Deploy**: Use Contracts UI or Pop CLI

## Pro Tips for Beginners

- **Start simple** - build a counter or flipper contract first
- **Use examples** - the official ink! examples are your best friend
- **Test everything** locally before deploying
- **Join the community** - ink! Discord for help and support


## Next Steps

1. **Deploy to testnet**: Try Pop Network or Rococo
2. **Add more features**: Events, errors, and mappings
3. **Learn advanced patterns**: Storage optimization and gas efficiency

*Ready to dive deeper? The complete beginner's guide above covers everything you need to become an ink! developer. Start with the counter contract and build your way up to more complex smart contracts!*
