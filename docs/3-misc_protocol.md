# MISC Protocol

The Medical Infrastructure Supply Chain (MISC) defines the functionality of our medical device design. We formerly describe protocols for each function in MISC. In order to maintain the secure operation of devices, all MISC messages are sent using the [HIDE protocol](./hide_protocol.md).

> [!NOTE]  
> "TTT" refers to "total transaction time" and is used to ensure timing functionality requirements are met.

## List Components
The AP must be able to list Components which are currently connected to it, regardless of whether a Component has been provisioned for the AP. This is done by sending a HIDE list packet request to each possible I2C address. If a Component is present, it will respond with its Component ID. The AP will then log this Component ID to the host.

Since listing a component is not considered a sensitive operation, there is no requirement to use secure communication. Additionally, performing a full HIDE challenge-response handshake for every possible I2C address would be time-consuming and unnecessary. The AP would also need to know the Component's ID in order to properly encrypt/decrypt a secure HIDE message. For these reasons, the HIDE list packet request was created as an extension to the HIDE protocol and allows the quick retrieval of a Component's ID without the need for secure communication.

Note that the listing components protocol is an exception to the rest of the MISC protocol, which use the full HIDE protocol by sending a HIDE secure packet request.

```{.mermaid loc=img #fig:list caption="List components protocol"}
sequenceDiagram
  participant H as Host
  participant AP as Application Processor
  participant C as Component(s)
  H ->> AP: "list"
  loop For each provisioned component
    AP ->> H: Info: "P>0x" + Component ID
  end
  loop For each I2C address
    AP ->> C: HIDE List Packet Request
    alt C is attached and responsive
      C ->> AP: Component ID
      AP ->> H: Info: "F>0x" + Component ID
    else C is unresponsive
      Note over AP: No response needed, continue
    end
  end
  AP -x H: Success: "List"
```

Specific message packet formats are not defined for the list components protocol, since this is defined by the HIDE protocol.

## Attest Components
Every Component stores Attestation Data which is used to verify the authenticity of the Component. This Attestation Data is protected by a 6-byte Attestation PIN, which must be stored and validated on an AP. Thus, the communications between the AP and the Component must be authentic and secure in order to prevent possible attacks to extract the Attestation Data through bypassing the AP. The HIDE protocol is used to ensure this secure communication.

In order to protect the Attestation PIN from being compromised, the AP stores the Attestation PIN as three uniquely salted SHA3-512 hashes. The salt is a random 16-byte string that is generated during the AP's build process and is prepended to the Attestation PIN before hashing. Each salt and hash pair is stored in the AP so that the Attestation PIN is never stored in plaintext.

The transfer of the Attestation Data is broken into three parts in order to meet length requirements of the HIDE protocol. The AP will first request the Component's Attestation Location, then the Attestation Date, and finally the Attestation Customer. The Component will respond with each of these pieces of information in turn. Only once the AP has received all three pieces of information will it send the Attestation Data to the host.

Additionally, a fixed transaction delay of 2.5 seconds is sent in place to prevent brute force attacks.


```{.mermaid loc=img #fig:attest caption="Attest components protocol"}
sequenceDiagram
  participant H as Host
  participant AP as Application Processor
  participant C as Component
  H ->> AP: "attest"
  H ->> AP: PIN Attempt
  H ->> AP: Component ID
  loop For each N Salt-Hash pair
    Note over AP: Compute SHA3-512(Salt N, Attest PIN attempt) <br />and compare with stored Hash N
  end
  Note over H, C: Minimum 2.5 seconds TTT elapsed
  alt Any hash is incorrect
    Note over H, C: Delay for an additional 5s
    AP -x H: Error: "Attest failed"
  else
    AP ->> C: REQUEST_LOCATION
    C ->> AP: Attestation Location
    AP ->> C: REQUEST_DATE
    C ->> AP: Attestation Date
    AP ->> C: REQUEST_CUSTOMER
    C ->> AP: Attestation Customer
    AP ->> H: Info: "C>0x" + Component ID
    AP ->> H: Info: "LOC>" + Location
    AP ->> H: Info: "DATE>" + Date
    AP ->> H: Info: "CUST>" + Customer
    AP -x H: Success: "Attest"
    Note over H: <3s TTT on success
  end
```


### REQUEST_LOCATION
The Application Processor sends this message to the Component to request the Component's Attestation Location. The Component will respond with its Attestation Location.

| Name      | Offset | Size (bytes) | Content      |
| --------- | ------ | ------------ | ------------ |
| Magic     | `0x00` | 80           | `\x60` * 80  |

### SEND_LOCATION
The Component sends this message to the Application Processor in response to a REQUEST_LOCATION. This message only contains the Component's Attestation Location.

| Name             | Offset | Size (bytes) | Content            |
| ---------------- | ------ | ------------ | ------------------ |
| Location         | `0x00` | 64           | `\x?? * 64`        |

### REQUEST_DATE
The Application Processor sends this message to the Component to request the Component's Attestation Date. The Component will respond with its Attestation Date.

| Name      | Offset | Size (bytes) | Content      |
| --------- | ------ | ------------ | ------------ |
| Magic     | `0x00` | 80           | `\x62` * 80  |

> [!WARNING]  
> The Component should not respond to a REQUEST_DATE if it has not received a REQUEST_LOCATION beforehand.

### SEND_DATE
The Component sends this message to the Application Processor in response to a REQUEST_DATE. This message only contains the Component's Attestation Date.

| Name             | Offset | Size (bytes) | Content            |
| ---------------- | ------ | ------------ | ------------------ |
| Date             | `0x00` | 64           | `\x?? * 64`        |

### REQUEST_CUSTOMER
The Application Processor sends this message to the Component to request the Component's Attestation Customer. The Component will respond with its Attestation Customer.

| Name      | Offset | Size (bytes) | Content      |
| --------- | ------ | ------------ | ------------ |
| Magic     | `0x00` | 80           | `\x64` * 80  |

### SEND_CUSTOMER
The Component sends this message to the Application Processor in response to a REQUEST_CUSTOMER. This message only contains the Component's Attestation Customer.

| Name             | Offset | Size (bytes) | Content            |
| ---------------- | ------ | ------------ | ------------------ |
| Customer         | `0x00` | 64           | `\x?? * 64`        |

> [!WARNING]  
> The Component should not respond to a REQUEST_CUSTOMER if it has not received a REQUEST_DATE beforehand.

## Replace Components
The AP keeps tracks of Components which are provisioned for it by storing their Component IDs. When a Component is replaced, the AP will update its list of provisioned Components with the new Component ID.

Because this is a sensitive operation, the AP requires a Replacement Token to be sent by the host to validate the replacement. The Replacement Token is a 16-byte string that is assigned during the AP's build process. The Replacement Token is used to ensure that the host is authorized to replace the Component.

In order to protect the Replacement Token from being compromised, the AP stores the Replacement Token as three uniquely salted SHA3-512 hashes. The salt is a random 16-byte string that is generated during the AP's build process and is prepended to the Replacement Token before hashing. Each salt and hash pair is stored in the AP so that the Replacement Token is never stored in plaintext.

Additionally, a fixed transaction delay of 4.5 seconds is sent in place to prevent brute force attacks.

```{.mermaid loc=img #fig:replace caption="Replace components protocol"}
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
  Note over H, AP: Minimum 4.5 seconds TTT elapsed
  alt Any hash is incorrect
    Note over H, AP: Delay for an additional 5s
  AP -x H: Error: "Replace failed"
  else
    Note over AP: Update Component ID list <br/>with new Component ID
  AP -x H: Success: "Replace"
  Note over H: <5s TTT on success
  end

```

## Boot Verification
The boot verification process is used to ensure that the Application Processor (AP) only boots if all expected Components are present and valid. The AP will verify that each Component is attached and responsive before booting. The AP will then collect each Component's boot message and send it to the host.

```{.mermaid loc=img #fig:boot caption="Boot verification protocol"}
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
  AP -x H: Success: "Boot"
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
As part of the MISC requirements, both the AP and Component must be able to enter a POST_BOOT state, where arbitrary C code runs to support medical device operation. The AP and Component must be able to communicate with each other in this state, so interfaces for HIDE secure send and receive are defined.

For more information about the HIDE protocol, please see the [HIDE protocol documentation](./hide_protocol.md).