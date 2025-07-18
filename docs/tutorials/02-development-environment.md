
# ink! Beginner Tutorial 2: Setting Up Your Development Environment

Are you new to ink!? This guide will gently walk you through preparing your computer for smart contract development with ink!. No prior blockchain experience is required—just a willingness to learn and some basic familiarity with the command line.

## 🧭 Overview

To build ink! smart contracts, you'll need to install some tools, set up a blockchain node on your machine, and configure your code editor. This guide will help you get everything ready, step by step.

## 🎯 Learning Objectives

After completing this tutorial, you will:

- Have Rust and `cargo-contract` installed
- Run a local Substrate blockchain node for testing
- Configure your code editor for ink! development
- Successfully compile and deploy a simple contract


## ✋ Prerequisites

- Completed Tutorial 1
- Basic knowledge of how to use your computer’s terminal or command prompt


## 📚 Topics Covered

### 1. Installing Rust

- Download and install Rust using [rustup]
- Add the necessary WASM (WebAssembly) target for ink!
- Confirm Rust is installed properly


### 2. cargo-contract Tool

- Install the cargo-contract CLI tool, which helps build and interact with ink! contracts
- Learn essential commands (e.g., for building and deploying contracts)
- How to check for and update the tool when new versions are released


### 3. Local Development Node

- Download and run `substrate-contracts-node`, a blockchain node specifically for smart contract testing
- Start your own test blockchain locally
- Learn how to read basic output from the node’s logs to ensure it’s working


### 4. IDE Configuration

- Set up Visual Studio Code (VS Code) with the Rust analyzer extension for friendly syntax support
- Enable ink! highlighting and linting for better code writing
- Optional: configure debugging tools


### 5. Additional Tools

- Install Polkadot.js Apps to interact with your blockchain and contracts through your browser
- Explore other handy contract deployment tools and testing frameworks


## 🛠️ Hands-On Activities

1. **Install all required tools:** Follow the instructions for Rust, cargo-contract, and substrate-contracts-node.
2. **Start a local Substrate node:** Run the node and watch the logs to ensure it’s working.
3. **Create a new ink! project:** Use cargo-contract to scaffold a new project.
4. **Compile and deploy a basic contract:** Build your contract and try deploying it to your local node.

## ✅ Environment Checklist

| Task | Status |
| :-- | :--: |
| Rust installed and updated | [ ] |
| cargo-contract installed | [ ] |
| substrate-contracts-node running | [ ] |
| IDE configured | [ ] |
| Basic contract compiled | [ ] |

Use this checklist as you go!

## 🆘 Troubleshooting

- **Common issues**: Installation errors, missing dependencies, network problems
- **Platform notes**: Some commands are different for Windows, macOS, and Linux—double-check!
- **Getting help**: Look for answers in the ink! and Polkadot communities, Discord, or GitHub issues sections


## ⏩ Next Steps

- Move on to creating your first smart contract (see Tutorial 3)
- Dive into your project’s folder structure to learn how things fit together


## 🔗 Resources

- [cargo-contract GitHub]
- [substrate-contracts-node]
- [VS Code Rust Extension]

---

# Tutorial 2: Setting Up Your ink! Development Environment (The Simple Way)

**Ready to build your first ink! smart contract?** This tutorial will get you up and running in under 15 minutes using the easiest approach available. We'll focus on what you absolutely need to know, skipping the complexity that often overwhelms beginners.

## 🎯 What You'll Achieve

By the end of this tutorial, you'll have:

- A working ink! development environment
- Built and deployed your first smart contract
- Confidence to move on to writing your own contracts


## 🛤️ Choose Your Path

There are three main ways to set up ink! development, each suited for different needs:

![Essential ink! Development Setup Paths - Choose the approach that best fits your needs](https://ppl-ai-code-interpreter-files.s3.amazonaws.com/web/direct-files/52c76eae749ad948ad0aa8d3b69ebd67/c5d95f1f-febe-416f-ac5b-870aadfcf925/ed1cb415.png)

Essential ink! Development Setup Paths - Choose the approach that best fits your needs

**For beginners, we strongly recommend the Pop CLI approach** - it's the fastest and handles most complexity for you.

## 🚀 The Recommended Approach: Pop CLI

Pop CLI is a modern, all-in-one tool that makes ink! development much easier. Here's why it's perfect for beginners:

- **One command setup** - no need to install multiple tools separately
- **Built-in local blockchain** - no complex node setup required
- **Batteries included** - comes with everything you need out of the box
- **Great documentation** - actively maintained with clear guides


### Essential Steps (13 minutes total)

### What Just Happened?

1. **Rust** - The programming language that ink! is built on
2. **Pop CLI** - Your all-in-one development tool
3. **Contract creation** - Generated a template smart contract
4. **Build \& test** - Verified everything works
5. **Local deployment** - Put your contract on a test blockchain

## ✅ Quick Verification

Run this command to verify your setup:

```bash
pop --version
```

If you see version information, you're ready to go! 🎉

## 🔧 Alternative Approaches

If Pop CLI doesn't work for you, here are the other options:


| Approach | Best For | Time Required |
| :-- | :-- | :-- |
| **Traditional** | Standard development workflow | ~30 minutes |
| **Swanky Suite** | Advanced features and templates | ~20 minutes |

The traditional approach requires installing `cargo-contract` and `ink-node` separately, while Swanky Suite offers advanced project templates and testing tools.

## 🆘 Common Issues \& Solutions

**Pop CLI won't install?**

- Make sure Rust is properly installed: `rustc --version`
- Try: `rustup update` then reinstall Pop CLI

**Build fails?**

- Check you're in the correct directory: `ls` should show `Cargo.toml`
- Try: `pop clean` then `pop build`

**Deployment issues?**

- Ensure the contract builds successfully first
- Check your terminal for specific error messages


## 📚 What's Next?

Now that your environment is ready:

1. **Tutorial 3** - Write your first custom smart contract
2. **Explore the project** - Look at the files Pop CLI created
3. **Join the community** - Get help on [Polkadot Stack Exchange](https://polkadot.stackexchange.com/)

## 🎉 Congratulations!

You've successfully set up your ink! development environment using the most beginner-friendly approach available. The days of complex multi-tool setups are behind you - Pop CLI handles the heavy lifting so you can focus on learning smart contract development.