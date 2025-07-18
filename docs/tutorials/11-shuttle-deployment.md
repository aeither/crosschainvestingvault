
# Zero-to-Hero Guide: Deploying ink! Smart Contracts with Shuttle.dev

New to ink! and wondering how to move from “it compiles!” to a live contract that auto-updates, scales, and tells you when something breaks? This beginner-friendly article walks you through every step—no prior Cloud or DevOps wizardry required. By the end you’ll have:

* A Shuttle.dev account linked to your local Rust toolchain.
* A fully automated pipeline that builds, deploys, and monitors an ink! contract.
* Confidence to roll back, scale out, and keep prod healthy.


## 1  Why Shuttle.dev for ink!

Shuttle.dev is a Rust-native platform that handles infrastructure from code annotations. For ink! developers this means:

* **Single CLI workflow** – deploy Wasm-based contracts and REST backends side-by-side.
* **Zero Docker, zero YAML** – infra is inferred from code and a tiny `shuttle.toml`.
* **Managed observability** – logs, metrics, and alerts are wired in automatically.

Beginner takeaway: you write Rust; Shuttle spins up the cloud bits.

![End-to-end deployment pipeline for an ink! smart contract using Shuttle.dev.](https://user-gen-media-assets.s3.amazonaws.com/gpt4o_images/a1f1e536-a132-44aa-9e96-0fb315afc748.png)

End-to-end deployment pipeline for an ink! smart contract using Shuttle.dev.

## 2  Prerequisites

1. Rust 1.76+ and `cargo` installed.
2. `cargo install --force --locked cargo-contract` to compile ink!.
3. `curl -sSfL https://www.shuttle.dev/install | bash` to grab Shuttle CLI.
4. A free Shuttle.dev account (`cargo shuttle login`).

Tip: everything runs on macOS, Linux, or WSL without extra tooling.

## 3  Project Bootstrap

### 3.1 Create the contract

```bash
cargo contract new flipper
cd flipper
cargo contract build
```

`target/ink/flipper.contract` is the bundle Shuttle will deploy.

### 3.2 Initialize Shuttle

```bash
cargo shuttle init --name flipper-service
```

This generates:

* `shuttle.toml` – minimal build/deploy config.
* `src/main.rs` – entry point annotated with `#[shuttle_runtime::main]`.

Replace the default Rocket example with the ink! helper:

```rust
use shuttle_ink::prelude::*;

#[shuttle_ink::main]
async fn main() -> Result<(), shuttle_ink::Error> {
    Contract::new()
        .with_name("flipper")
        .with_network("polkadot")
        .with_version("1.0.0")
        .build()?
        .deploy()
        .await?;
    Ok(())
}
```


## 4  Configuring shuttle.toml

```toml
[build]
runtime = "ink"
version = "latest"

[deploy]
network  = "polkadot"
contract = "target/ink/flipper.contract"

[monitoring]
enabled = true
alerts  = ["error", "performance"]
```

Key points:

* `runtime = "ink"` auto-selects Wasm toolchain.
* `network` can be `rococo`, `astar-zkevm`, or any parachain hosting `pallet-contracts`.
* Alerts are routed to the default Shuttle dashboard—no Prometheus setup needed.


## 5  First Deployment

```bash
cargo shuttle deploy
```

Output shows:

* Build logs for both your contract and the tiny runtime wrapper.
* A public HTTPS endpoint (e.g., `https://flipper-service.shuttleapp.rs`).
* The on-chain contract address.

Visit the endpoint and Shuttle streams logs plus Polkadot extrinsics in real time—handy for debugging.

## 6  Automating with GitHub Actions

Create `.github/workflows/deploy.yml`:

```yaml
name: Shuttle CI
on: [push]
jobs:
  build-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with: { toolchain: stable, override: true }
      - run: cargo install --locked cargo-shuttle
      - run: shuttle login --api-key ${{ secrets.SHUTTLE_API_KEY }}
      - run: cargo shuttle deploy --allow-dirty
```

Store the API key in repo secrets and every push ships a new contract instance. Pair with Shuttle’s built-in **version management** to roll back via one CLI:

```bash
cargo shuttle deployment rollback <deployment-id>
```


## 7  Observability Basics

1. **Logs** – `cargo shuttle logs -r flipper-service` tails stdout and on-chain events.
2. **Metrics** – default dashboards show Wasm heap, gas, and extrinsic success rate.
3. **Alerts** – enable Slack via dashboard → Integrations → Slack webhook; choose rules like _error > 5/min_.

Advanced: export metrics to Grafana with `cargo shuttle resource add grafana`.

## 8  Scaling \& Reliability

| Scenario | Shuttle feature | Command |
| :-- | :-- | :-- |
| High call volume | Horizontal scaling | `cargo shuttle scale set 3` |
| Blue-green update | Canary flag | `cargo shuttle deploy --canary` |
| Daily backup | Snapshot contract storage | `cargo shuttle backup create --dest s3://ink-backups` |

## 9  Hands-On Checklist

1. **Account** – sign up \& CLI login.
2. **Contract build** – `cargo contract build`.
3. **Init \& deploy** – `cargo shuttle init` → `cargo shuttle deploy`.
4. **Push to GitHub** – verify Action auto-deploys.
5. **Trigger flip()** – call via Contracts UI or Polkadot-JS and watch logs update.

You now have a live CI/CD loop!

## 10  Troubleshooting Tips

* **`ink metadata mismatch`** – rebuild with same `cargo-contract` version locally and in CI.
* **Deploy timeouts** – use `--network rococo` for faster test deployments.
* **Gas too low** – Shuttle estimates automatically, but custom constructors may need `--gas-limit`.

Still stuck? `cargo shuttle feedback` opens a GitHub issue prefilled with logs.

## 11  Best Practices Cheat-Sheet

* Version-lock toolchains (`rust-toolchain.toml` + `rustup override`).
* Separate dev/staging/prod via Shuttle **environments**.
* Use `openbrush` library for PSP22/34 standards to avoid reinventing security checks.
* Document every constructor arg in README—Shuttle surfaces them in dashboard UI.


## 12  Next Steps

* **Tutorial 12** – mobile dApp integration (Flutter + Polkadart).
* Explore **OpenBrush** for token standards and access control scaffolds.
* Contribute to ink! Dev Hub for more examples.
