# MISC Protocol

The Medical Infrastructure Supply Chain (MISC) . All MISC messages are sent over the [HIDE protocol](./hide_protocol.md).

> [!NOTE]  
> "TTT" refers to "total transaction time" and is used to ensure timing functionality requirements are met.

## List Components
The host will ask the Application Processor to "list" its components.
The Application Processor, upon receiving the message from the host, will list its provisioned components.
It will then send a magic byte as a ping to every I2C address. For components that are present and responsive, they will send a magic byte pong as well as its component ID, which will prompt the Application Processor to send the component ID to the host.

```{.mermaid loc=img}
sequenceDiagram
  participant H as Host
  participant AP as Application Processor
  participant C as Component(s)
  H ->> AP: "list\r"
  loop For each provisioned component
    AP ->> H: Info: "P>0x" + CID + "\n"
  end
  loop For each I2C address
    AP ->> C: LIST_PING
    alt C is attached and responsive
      C ->> AP: LIST_PONG
      C -->> AP: CID
      AP ->> H: Info: "F>0x" + CID + "\n"
    else C is unresponsive
      Note over AP: No response needed, continue
    end
  end
  AP -x H: Success: "List\n"
```


### LIST_PING
This byte is sent to every I2C address. For present components, this indicates that the Application Processor is asking for its component ID.

| Name      | Offset | Size (bytes) | Content |
| --------- | ------ | ------------ | ------- |
| Magic     | `0x00` | 1            | `\x50`  |

### LIST_PONG
After being prompted by the Application Processor using LIST_PING, an active component will send a response byte and the component ID to the Application Processor.

| Name         | Offset      | Size (bytes) | Content            |
| ------------ | ----------- | ------------ | ------------------ |
| Magic        | `0x00`      | 1            | `\x51`             |
| Component ID | `0x01-0x05` | 4            | `\x??\x??\x??\x??` |


## Attest Components
Description TODO.

> [!NOTE]
> The PIN attempt and component ID need to be transmitted at the beginning in a way that the host tool can understand.

```{.mermaid loc=img}
sequenceDiagram
  Host ->> AP: "attest\r"
  Host ->> AP: PIN Attempt
  Host ->> AP: Component ID
  Note over Host, Component: Minimum 0.8 TTT elapsed
  Note over AP: Compute Argon 2 hash from salt <br />and PIN attempt, compare <br />computed hash and stored hash
  Note over Host, Component: Minimum 2.8s TTT elapsed
  alt PIN is incorrect
    Note over Host, Component: Delay for an additional 5s
    AP -x Host: Error: "Attest failed\n"
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
    AP -x Host: Success: "Attest\n"
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
The AP keeps tracks of Components which are provisioned for it by storing their Component IDs. When a Component is replaced, the AP will update its list of provisioned Components with the new Component ID.

Because this is a sensitive operation, the AP requires a Replacement Token to be sent by the host to validate the replacement. The Replacement Token is a 16-byte string that is assigned during the AP's build process. The Replacement Token is used to ensure that the host is authorized to replace the Component.

In order to protect the Replacement Token from being compromised, the AP stores the Replacement Token as three uniquely salted SHA3-512 hashes. The salt is a random 16-byte string that is generated during the AP's build process and is prepended to the Replacement Token before hashing. Each salt and hash pair is stored in the AP so that the Replacement Token is never stored in plaintext.

Additionally, a fixed transaction delay of 4.5 seconds is sent in place to prevent brute force attacks.

```{.mermaid loc=img caption="Replace components protocol"}
sequenceDiagram
  participant H as Host
  participant AP as Application Processor
  H ->> AP: "replace"
  H ->> AP: Replacement Token attempt
  H ->> AP: New Component ID
  H ->> AP: Old Component ID
  loop For each N Salt-Hash pair
    Note over AP: Compute SHA3-512(Salt N, Replacement Token attempt) <br />and compare with stored Hash N
  end
  Note over H, AP: 4.5 seconds TTT elapsed
  alt Any hash is incorrect
    Note over H, AP: Delay for an additional 5s
  AP -x H: Error: "Replace failed"
  else
    Note over AP: Update Component ID list <br/>with new Component ID
  AP ->> H: Success: "Replace"
  Note over H: <5s TTT on success
  end

```

## Boot Verification
The boot verification process is used to ensure that the Application Processor (AP) only boots if all expected Components are present and valid. The AP will verify that each Component is attached and responsive before booting. The AP will then collect each Component's boot message and send it to the host.

```{.mermaid loc=img}
sequenceDiagram
  participant H as Host
  participant AP as Application Processor
  participant C as Component
  H ->> AP: "boot"
  loop For each provisioned Component
    AP ->> C: BOOT_PING
      alt Component is attached and responsive
      C ->> AP: BOOT_PONG
      Note over AP: Continue
    else Component is unresponsive
      Note over AP: Abort boot
      AP -x H: Error: "Boot failed"
    end
  end
  Note over H, C: AP has verified each Component is attached and responsive
  loop For each provisioned Component
    AP ->> C: BOOT_NOW
    C -->> AP: Component's Boot Message
    Note over C: Component enters POST_BOOT
  end
  Note over H, C: AP has collected all Component boot messages
  loop For each provisioned Component
    AP ->> H: Info: Component's Boot Message
  end
  AP ->> H: Info: AP's Boot Message
  AP ->> H: Success: "Boot"
  Note over AP: AP enters POST_BOOT
  Note over H: <3s TTT on success
```

### BOOT_PING
Sent from the Application Processor to each Component to "ping" the Component to ensure it is attached and responsive. Expected to receive a BOOT_PONG in response.

| Name      | Offset | Size (bytes) | Content      |
| --------- | ------ | ------------ | ------------ |
| Magic     | `0x00` | 80           | `\x80` * 80  |

### BOOT_PONG
Sent from the Component to the Application Processor in response to a BOOT_PING. This indicates that the Component is attached and responsive.

| Name      | Offset | Size (bytes) | Content      |
| --------- | ------ | ------------ | ------------ |
| Magic     | `0x00` | 80           | `\x81` * 80  |

### BOOT_NOW
Sent from the Application Processor to each Component to command the Component to boot. The Component will then send its boot message (of length 64) to the Application Processor.

| Name      | Offset | Size (bytes) | Content      |
| --------- | ------ | ------------ | ------------ |
| Magic     | `0x00` | 80           | `\x82` * 80  |

> [!WARNING]  
> The component should not respond to a BOOT_NOW if it has not received a BOOT_PING beforehand.

## Post-Boot Communication
Description TODO.

\newpage