# Embassy STM32F0 Blink

An asynchronous Rust "Blinky" example for the **STM32F0** series microcontrollers using the [Embassy framework](https://embassy.dev/).

Existing examples for the STM32F0 series are often outdated or use legacy HALs. This project provides a clean template using current best practices (as of 2026), including `probe-rs`, `defmt` logging, and the Embassy async executor.

## 🚀 Features
- **Async/Await**: Uses `embassy-executor` for non-blocking task management.
- **Multi-tasking**: Spawns two independent blink tasks with different frequencies.
- **Modern Tooling**: Integrated with `probe-rs` for seamless flashing and RTT debugging.
- **Efficient Logging**: Uses `defmt` for high-performance, low-overhead logging over RTT.
- **Optimized**: Configured with a `release` profile focused on small binary size (`opt-level = "z"`).

## 🛠 Hardware Supported
This project is configured for the **STM32F0DISCOVERY** board, which features the **STM32F051R8** MCU.

- **MCU**: STM32F051R8 (Cortex-M0)
- **Flash**: 64 KB
- **RAM**: 8 KB
- **LEDs used**: 
    - `PC8` (Blue LED)
    - `PC9` (Green LED)

## 📋 Prerequisites

1. **Rust Toolchain**: Install the stable Rust toolchain and the thumbv6m target:
   ```bash
   rustup target add thumbv6m-none-eabi
   ```

2. **probe-rs**: Used for flashing and debugging. Ensure you have version `0.30.0` or higher:
   ```bash
   cargo install probe-rs --features cli
   ```

3. **ST-Link**: The Discovery board has a built-in ST-Link/V2. Ensure your system recognizes it (drivers for Windows, or udev rules for Linux).

## 🏃 Getting Started

1. **Clone the repository**:
   ```bash
   git clone https://github.com/TiaTaiT/embassy_stm32f0_blink.git
   cd embassy_stm32f0_blink
   ```

2. **Connect your board** via the Mini-USB port.

### 3. Run the application
You can flash and run the application directly using `cargo run`. This will compile the code, upload it to the STM32F0, and automatically open a `defmt` terminal to show the logs.

*   **Development Profile** (Standard):
    ```bash
    cargo run
    ```
    Use this during active development. Even in the `dev` profile, this project uses `opt-level = "z"` to ensure the code fits within the STM32F0's 64KB Flash while keeping the build process relatively fast.

*   **Release Profile** (Optimized):
    ```bash
    cargo run --release
    ```
    Use this for final testing or deployment. This profile enables Link Time Optimization (LTO) and minimizes binary size for maximum efficiency.

***

## 📂 Project Structure

- **`.cargo/config.toml`**: Configures the `probe-rs` runner and necessary linker flags (`link.x` and `defmt.x`).
- **`src/main.rs`**: 
    - Initializes the Embassy HAL.
    - Defines an async `blink_task` that uses a `Peri<AnyPin>` type, allowing the same code to drive any GPIO pin.
    - Spawns two instances of the task to blink the Blue and Green LEDs independently.

## 📝 Customization

### Changing the MCU
If you are using a different STM32F0 chip:

1. `embassy-stm32` already enables the `memory-x` feature, which automatically selects a memory map. You can add a `memory.x` file if your chip is not supported by the automatic memory mapping selection.
2. Update the `embassy-stm32` feature in `Cargo.toml` (for example, change `stm32f051r8` to `stm32f030f4`).
3. Update the `--chip` flag in `.cargo/config.toml`.

### Changing Blink Rates
Modify the `spawner.spawn` calls in `src/main.rs`:
```rust
// Change 500ms to 1000ms
spawner.spawn(blink_task(p.PC8.into(), 1000)).unwrap();
```

## 📜 License
This project is licensed under the MIT License.
