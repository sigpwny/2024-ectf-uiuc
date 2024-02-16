# MISC Protocol

Description TODO. All MISC messages are sent over the [HIDE protocol](./hide_protocol.md).

> [!NOTE]  
> "TTT" refers to "total transaction time" and is used to ensure timing functionality requirements are met.

## List Components
Description TODO.

```mermaid
sequenceDiagram
  participant H as Host
  participant AP as Application Processor
  participant C1 as Component 1
  participant C2 as Component 2
  H ->> AP: "list component request"
  AP ->> H: "Log component IDs"
  Note over AP, C2: The request is sent to both components are the same time
  AP ->> C1: PING with component ID
  AP ->> C2: PING with component ID
  C1 ->> AP: PONG with component ID
  AP ->> H: Log Component 1 as found
  alt C1 response > 1s
    AP -x H: "Component 1 not found"
  end
  C2 ->> AP: PONG with component ID
  AP ->> H: Log Component 2 as found
  alt C2 response > 1s
    AP -x H: "Component 2 not found"
  end
```

### COMMAND_NAME
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x50`  |
| TODO      | TODO   | TODO         | TODO    |

## Attest Components
Description TODO.

```mermaid
```

### COMMAND_NAME
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x60`  |
| TODO      | TODO   | TODO         | TODO    |

## Replace Components
Description TODO.

```mermaid
sequenceDiagram
  participant H as Host
  participant AP as Application Processor
  H ->> AP: “Send Replacement Token Attempt, Old Component ID, and New Component ID”
  Note over H, AP: “AP waits for 3 seconds”
  Note over AP	: “Append Replacement Token Salt to received Replacement Token Attempt and compute Argon2 hash”
  Note over AP: “Compare Token Attempt hash to stored Correct Token hash”
  Note over H, AP: “Wait until 4.8 seconds total time elapsed since beginning of transaction”
  alt Correct Replacement Token
     Note over AP: “Updates Component ID list with new Component ID”
  end 

```

### COMMAND_NAME
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x70`  |
| TODO      | TODO   | TODO         | TODO    |

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

### BOOT_PONG
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x81`  |

### BOOT_NOW
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x82`  |

## Post-Boot Communication
Description TODO.

