# MISC Protocol

Note: Please use Mermaid diagrams to visualize the protocol. Here is an example of what we expect in terms of documentation: https://github.com/sigpwny/2023-ectf-sigpwny/blob/main/docs/protocol.md

## List Components



## Attest Components

```mermaid
sequenceDiagram
  Host->>AP: send comp ID and PIN
  Note over Host,Component: AP waits for 1 second
  Note over AP: Argon 2 Compute Salt and Hash
  Note over AP: Compare PIN hash and stored hash
  Note over Host,Component: Minimum 2.8s TTT elapsed
  alt is incorrect
    Note over AP: AP waits 2 seconds
    AP->>Host: AP sends invalid pin message
  else is correct
    AP->>Component: AP requests attestation data
    Component->>AP: Component sends attestation data (encrypted?)
    AP->>Host: AP sends attestation data
  end
```



## Replace Components



## Boot Verification



## Post-Boot Communication


