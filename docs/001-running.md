# Compiling and Running the Project

This guide outlines the steps to compile and run the project on your ESP32 device.

## Prerequisites

Ensure you have the necessary Rust and ESP-IDF toolchains installed.

## Compilation and Flashing Steps

1.  **Initial Build (Expect Error):**
    First, initiate a build. This will likely fail, but it's necessary to create the required build directories.
    ```bash
    cargo build
    ```

2.  **Copy Partitions File:**
    After the initial build failure, you need to manually copy the `partitions.csv` file to the build output directory. The exact path might vary slightly depending on your environment, but it generally follows this pattern:
    ```bash
    cp partitions.csv ./target/xtensa-esp32-espidf/debug/build/esp-idf-sys-ee16df87bc2e2aef/out/
    ```
    **Note:** Please verify the exact target directory path in your error message or file system before executing this command, as the hash `ee16df87bc2e2aef` might change.

3.  **Second Build:**
    Run `cargo build` again. This time, with the `partitions.csv` in place, the build should succeed.
    ```bash
    cargo build
    ```

4.  **Flash and Monitor:**
    Finally, flash the compiled application to your ESP32 device and start monitoring its serial output using `espflash`.
    ```bash
    espflash flash --monitor -S --no-verify
    ```
    This command flashes the binary, automatically selects the correct serial port, and opens a serial monitor.
