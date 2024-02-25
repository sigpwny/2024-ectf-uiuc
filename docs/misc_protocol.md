# MISC Protocol

Description TODO. All MISC messages are sent over the [HIDE protocol](./hide_protocol.md).

> [!NOTE]  
> "TTT" refers to "total transaction time" and is used to ensure timing functionality requirements are met.

## List Components
The host will ask the Application Processor to "list" its components.
The Application Processor, upon receiving the message from the host, has to ping
the components. It will send magic bytes recognizable to the components and waits for responses from the components. If a response is not received within a 
threshold of one second, the AP logs the component as "not found" to the Host, but if a response is received, then the AP logs the component ID to the Host.

```mermaid
sequenceDiagram
  participant H as Host
  participant AP as Application Processor
  participant C1 as Component 1
  participant C2 as Component 2
  H ->> AP: "list\r"
  loop For each provisioned component
    AP ->> H: Info: "P>0x" + CID + "\n"
  end
  Note over AP, C2: LIST_PINGs are sent in order, one at a time
  AP ->> C1: LIST_PING
  alt C1 is attached and responsive
    C1 ->> AP: LIST_PONG
    AP ->> H: Info: "F>0x" + CID + "\n"
  else C1 is unresponsive
    Note over AP: No response needed, continue
  end
  AP ->> C2: LIST_PING
  alt C2 is attached and responsive
    C2 ->> AP: LIST_PONG
    AP ->> H: Info: "F>0x" + CID + "\n"
  else C1 is unresponsive
    Note over AP: No response needed, continue
  end
  AP -x H: Success: "Listed components!\n"
```


### LIST_PING
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x50`  |

### LIST_PONG
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x51`  |


## Attest Components
Description TODO.

> [!NOTE]
> The PIN attempt and component ID need to be transmitted at the beginning in a way that the host tool can understand.

```mermaid
sequenceDiagram
  Host ->> AP: "attest\r"
  Host ->> AP: PIN Attempt
  Host ->> AP: Component ID
  Note over Host, Component: Minimum 0.8 TTT elapsed
  Note over AP: Compute Argon 2 hash from salt <br />and PIN attempt, compare <br />computed hash and stored hash
  Note over Host, Component: Minimum 2.8s TTT elapsed
  alt PIN is incorrect
    Note over Host, Component: Delay for an additional 5s
    AP -x Host: Error: "Attest failed!\n"
  else
    AP ->> Component: REQUEST_LOCATION
    Component ->> AP: SEND_LOCATION
    AP ->> Component: REQUEST_DATE
    Component ->> AP: SEND_DATE
    AP ->> Component: REQUEST_CUSTOMER
    Component ->> AP: SEND_CUSTOMER
    AP ->> Host: Info: "C>0x" + CID "\n"
    AP ->> Host: Info: "LOC>" + Location "\n"
    AP ->> Host: Info: "DATE>" + Date "\n"
    AP ->> Host: Info: "CUST>" + Customer "\n"
    AP -x Host: Success: "Retrieved attestation data!\n"
  end
```


### REQUEST_LOCATION
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x60`  |

### SEND_LOCATION
Description TODO.

| Name             | Offset | Size (bytes) | Content            |
| ---------------- | ------ | ------------ | ------------------ |
| Magic            | `0x00` | 1            | `\x61`             |
| Location         | `0x01` | 64           | `\x?? * 64`        |

### REQUEST_DATE
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x62`  |

### SEND_DATE
Description TODO.

| Name             | Offset | Size (bytes) | Content            |
| ---------------- | ------ | ------------ | ------------------ |
| Magic            | `0x00` | 1            | `\x63`             |
| Date             | `0x01` | 64           | `\x?? * 64`        |

### REQUEST_CUSTOMER
Description TODO.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x64`  |

### SEND_CUSTOMER
Description TODO.

| Name             | Offset | Size (bytes) | Content            |
| ---------------- | ------ | ------------ | ------------------ |
| Magic            | `0x00` | 1            | `\x65`             |
| Customer         | `0x01` | 64           | `\x?? * 64`        |

## Replace Components
Description TODO.

```mermaid
sequenceDiagram
  participant H as Host
  participant AP as Application Processor
  H ->> AP: Send Replacement Token Attempt, Old Component ID, and New Component ID
  Note over H, AP: AP waits for 3 seconds
  Note over AP: Append Replacement Token Salt to received Replacement Token Attempt and compute Argon2 hash
  Note over AP: Compare Token Attempt hash to stored Correct Token hash
  Note over H, AP: Wait until 4.8 seconds total time elapsed since beginning of transaction
  alt Correct Replacement Token
     Note over AP: Updates Component ID list with new Component ID
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
  H ->> AP: "boot\r"
  Note over AP, C2: BOOT_PINGs are sent in order, one at a time
  AP ->> C1: BOOT_PING
    alt C1 is attached and responsive
    C1 ->> AP: BOOT_PONG
    Note over AP: Continue
  else C1 is unresponsive
    Note over AP: Abort boot
    AP -x H: Error: "Boot failed!\n"
  end
  AP ->> C2: BOOT_PING
  alt C2 is attached and responsive
    C2 ->> AP: BOOT_PONG
    Note over AP: Continue
  else C2 is unresponsive
    Note over AP: Abort boot
    AP -x H: Error: "Boot failed!\n"
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

