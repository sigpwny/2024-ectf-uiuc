\title{Medical Device (MISC) Design for MITRE eCTF 2024}
\author{SIGPwny at the University of Illinois Urbana-Champaign (UIUC)}

\maketitle

# Overview
This Design Document describes UIUC's implementation of the MISC system as defined by MITRE for eCTF 2024. The MISC system builds the platform for secure communication between medical devices through the use of a central Application Processor (AP) and various Components. While the reference design is considered insecure, our implementation aims to provide a secure and robust system that meets the security requirements outlined by MITRE.

We started our design by analyzing the security requirements and creating the conceptual protocols and cryptographic schemes that would meet these requirements.

## Rust

Our design is built using Rust, a systems programming language that provides memory safety, a strong type system, and a minimal runtime. Crucially, Rust protects us from common security vulnerabilities such as buffer overflows. That being said, Rust is not a silver bullet, and we must still be careful to avoid other security pitfalls.

There were quite a few challenges related to using Rust. In particular, the embedded platform that we are building on (Analog Devices MAX78000FTHR) did not have a stable, public Rust toolchain available online. To solve this, we implemented the peripheral drivers and built our own Hardware Abstraction Layer (HAL) from scratch.

Another challenge was that we were required to support the execution of post boot code which is written in C. Not only that, but the C code needed to be able to call back into our Rust code in order to use our secure communication layer. This post boot code also requires the availability of functions from both the standard C library and the MSDK. We rewrote these functions in Rust and provided a C ABI for them so the linker could provide them to the post boot C code.