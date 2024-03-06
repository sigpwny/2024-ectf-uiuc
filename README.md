# UIUC MISC Design for MITRE eCTF 2024

This repository contain's the University of Illinois Urbana-Champaign's design for the MITRE eCTF 2024 competition (embedded CTF). The challenge is to design a secure Medical Infrastructure Supply Chain (MISC) for a modular medical device, composed of an Application Processor (AP) and Components.

Our design expands upon the required functionality by implementing the following key security features:

- 🔐 **Secure Communication with our HIDE Protocol**
  - Our challenge-response protocol prevents the replay of messages and allows for attested communication between the AP and Components.
  - Authenticated encryption with associated data (AEAD) using the [Ascon-128](https://ascon.iaik.tugraz.at/) cipher ensures the confidentiality and integrity of messages.
  - All communications between the AP and Components use HIDE.
- 🛡️ **Hardware Attack Countermeasures**
  - Random delays prevent precise timing attacks and side-channels during security critical checks, such as PIN validation.
  - Repetition of critical operations mitigate fault-injection attacks.
- 🦀 **Written entirely in Rust!***
  - Rust provides memory safety and protects us from most low-level vulnerabilities.
  - Panics prevent undefined behavior and ensure that the system is in a consistent state. Panics are indicated by a flashing red LED on the boards.
  - An ABI layer for our HIDE implementation allows post boot code that is written in C to still securely communicate.

\* *This statement is approved by the White House.*

## Repository Structure

- `application_processor`: The firmware for the Application Processor.
  - `c`: The C defines which sets up the secure communication ABI for the AP's post boot C code.
  - `rust`: The Rust firmware code specific to the AP.
- `component`: The firmware for an individual Component.
  - `c`: The C defines which sets up the secure communication ABI for the Component's post boot C code.
  - `rust`: The Rust firmware code specific to the Component.
- `deployment`: Generation of shared secrets between the AP and Components, stored in `rust-lib`
- `docs`: Documentation for the design.
- `rust-lib`: Shared Rust code between the AP and Components.
  - `ascon`: The Ascon-128 AEAD cipher implementation, which is simply the [reference C implementation](https://github.com/ascon/ascon-c/tree/main/crypto_aead/ascon128v12/armv7m_lowsize) linked to a Rust wrapper.
  - `ectf-board`: Provides a board abstraction of the [MAX78000FTHR](https://www.analog.com/en/resources/evaluation-hardware-and-software/evaluation-boards-kits/max78000fthr.html) and shares functions between the AP and the Component. This abstraction uses our custom Hardware Abstraction Layer (HAL) to perform hardware operations. Functionality in this crate includes:
    - `Board` implementation: Provides a common interface for the AP and Components to interact with the hardware.
    - `secure_comms`: Implements the HIDE communication layer.
    - `ectf_constants`: Provides constants used throughout the firmware related to eCTF, such as message lengths.
    - `ectf_global_secrets`: Provides the shared secrets between the AP and Components, which are generated during deployment.
    - `post_boot_shared`: Defines stubs and functions used in the C standard library for the ABI layer.
  - `hal`: Our custom Hardware Abstraction Layer (HAL) for the MAX78000FTHR. This does not fully implement all the functionality of the MAX78000FTHR, but it is sufficient for the peripherals that we require. Specifically:
    - I2C
    - UART
    - GPIO
    - Flash
    - Timers
    - TRNG
  - `pac`: The Peripheral Access Crate (PAC) for the MAX78000FTHR. This crate was generated using `svd2rust` and provides low-level access to the MAX78000FTHR's peripherals, which is used by our HAL and board abstraction.

## Documentation

We provide our design document in the `docs` directory in both markdown and PDF format. We love Mermaid sequence diagrams and those are provided as well. Here is a quick link:

- [UIUC Design Document](docs/2024_eCTF_UIUC_Design_Document.pdf)

The code itself is also heavily documented using Rust doc comments. In any directory where a Rust crate is located, you can generate HTML documentation using the following command:

```sh
cargo doc --open
```

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE.txt) file for details.