# MISC Protocol

Description TODO. All MISC messages are sent over the [HIDE protocol](./hide_protocol.md).

> [!NOTE]  
> "TTT" refers to "total transaction time" and is used to ensure timing functionality requirements are met.

## List Components
Description TODO.


## Attest Components
Description TODO.


## Replace Components
Description TODO.


## Boot Verification
Description TODO.

```mermaid
sequenceDiagram
  participant H as Host
  participant AP as Application Processor
  participant C1 as Component 1
  participant C2 as Component 2
  H ->> AP: "boot"
  Note over AP, C2: BOOT_PINGs are sent in order, one at a time
  AP ->> C1: BOOT_PING
  C1 ->> AP: BOOT_PONG
  alt C1 response > 1s
    AP -x H: "Boot Failed"
    AP --x H: C1 Component ID
  end
  AP ->> C2: BOOT_PING
  C2 ->> AP: BOOT_PONG
  alt C2 response > 1s
    AP -x H: "Boot Failed"
    AP --x H: C2 Component ID
  end
  Note over H, C2: Minimum 2.8s TTT elapsed
  AP ->> C1: BOOT_NOW
  C1 -->> AP: C1 Boot Message
  Note over C1: C1 enters POST_BOOT
  AP ->> C2: BOOT_NOW
  C2 -->> AP: C2 Boot Message
  Note over C2: C2 enters POST_BOOT
  AP ->> H: "Boot Success"
  AP -->> H: AP Boot Message
  AP -->> H: C1 Boot Message
  AP -->> H: C2 Boot Message
  Note over AP: AP enters POST_BOOT
  Note over H: <3s TTT on success
```

### BOOT_PING
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x80`  |
| TODO      | TODO   | TODO         | TODO    |

### BOOT_PONG
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x81`  |
| TODO      | TODO   | TODO         | TODO    |

### BOOT_NOW
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x82`  |
| TODO      | TODO   | TODO         | TODO    |

## Post-Boot Communication
Description TODO.

