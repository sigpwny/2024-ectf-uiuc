# HIDE Protocol Communication Layer
We implement an extra communication layer between the I2C layer and the application layer, which we refer to as the HIDE protocol. The HIDE protocol ensures that all messages maintain confidentiality, integrity, authenticity, and non-replayability. We require all messages sent between the AP and the Component to use HIDE.

HIDE effectively turns each application message into a three-way challenge-response handshake. The sender first initiates a message request. The receiver will then send a random, encrypted challenge. The sender will then decrypt the challenge, solve it, and encrypt the challenge response to be sent along with the actual message. To solve the challenge nonce, the sender must perform a bitwise XOR of `0x55` with each byte in the challenge nonce.

We use the Authenticated Encryption (AE) cipher, [Ascon-128](https://ascon.iaik.tugraz.at/), for our cryptographic scheme. We chose Ascon since it was selected in the NIST Lightweight Cryptography competition and has a masked software implementation that has been tested against various power analysis and hardware attacks.

We use Ascon's associated data feature to validate each message that uses encryption. The associated data is 8 bytes long, with the first 4 bytes being the component ID, the fifth byte being the HIDE message magic byte, and the last three bytes being null bytes.

Each direction of communication uses a different symmetric encryption key, meaning there are two encryption keys:

- $K_{AP,C}$ is the key for messages sent from the Application Processor to the Component.
- $K_{C,AP}$ is the key for messages sent from the Component to the AP.

Every AP and Component built from the same deployment will share the same keys. This allows any Component to communicate with an AP from the same deployment.

## HIDE Secure Protocol

```{.mermaid loc=img #fig:hide-secure caption="HIDE secure communications protocol"}
sequenceDiagram
	participant AP as Device A<br />Application Processor<br />(or Component)
	participant C as Device B<br />Component<br />(or Application Processor)

	AP ->> C: HIDE_PKT_REQ_SECURE

	Note over C: Generates a challenge nonce

	C ->> AP: (HIDE_PKT_CHAL_SEND)
	C -->> AP: (Unencrypted): Ascon Nonce A
	C -->> AP: (Encrypted): Challenge Nonce
	C -->> AP: (Encrypted): Random bytes
	C -->> AP: (Unencrypted): Ascon Tag

	Note over AP: Receives, decrypts, and <br/>solves the challenge nonce

	AP ->> C: (HIDE_PKT_CHAL_RESP)
	AP -->> C: (Unencrypted): Ascon Nonce B
	AP -->> C: (Encrypted): Solved Challenge Nonce
	AP -->> C: (Encrypted): Message, padded with random bytes
	AP -->> C: (Unencrypted): Ascon Tag

	alt Challenge Nonce Solve is Incorrect
		Note over C: Reset transaction
	end
```

If at any point the encryption or decryption of a HIDE message using the Ascon cipher fails (such as invalidation due to message bit-flips or incorrect associated data), our secure communication functions will return an error.

Messages sent using HIDE can have a length up to 80 bytes. When HIDE interfaces are being used in the post boot code, the message length is limited to 64 bytes and the message length is encoded as an 8-bit integer in the first byte of the message. The remaining bytes are padded with random bytes. MISC messages are also limited to 64 bytes in length.

### HIDE_PKT_REQ_SECURE
Sent by any device to initial a secure communication session with another device. This packet is unencrypted and contains the magic bytes corresponding to a secure request.

| Name      | Offset | Size (bytes) | Content     |
| --------- | ------ | ------------ | ----------- |
| Magic     | `0x00` | 10           | `\x40` * 10 |

### HIDE_PKT_CHAL_SEND
Sent by the receiver of a secure request to initiate the challenge-response handshake. This packet contains the Ascon Nonce, the encrypted challenge nonce, and random bytes for padding.

| Name                      | Offset | Size (bytes) | Content     |
| ------------------------- | ------ | ------------ | ----------- |
| Ascon Nonce               | `0x00` | 16           | `\x?? * 16` |
| Encrypted Challenge Nonce | `0x10` | 16           | `\x?? * 16` |
| Encrypted Random Bytes    | `0x20` | 80 				  | `\x?? * 80` |
| Ascon Tag                 | `0x70` | 16           | `\x?? * 16` |

> [!WARNING]
> Ascon Nonce should be randomly uniquely generated for all messages

### CHAL_RESP
Sent by the sender of a secure request to complete the challenge-response handshake. This packet contains the Ascon Nonce, the encrypted solved challenge nonce, and the message to be sent.

| Name                       | Offset | Size (bytes) | Content     |
| -------------------------- | ------ | ------------ | ----------- |
| Ascon Nonce                | `0x00` | 16           | `\x?? * 16` |
| Encrypted Solved Nonce     | `0x10` | 16           | `\x?? * 16` |
| Encrypted Message + Random | `0x20` | 80 				   | `\x?? * 80` |
| Ascon Tag                  | `0x70` | 16           | `\x?? * 16` |

> [!WARNING]
> Ascon Nonce should be randomly uniquely generated for all messages

## HIDE List Protocol

As part of the MISC system, we also must implement functionality to list all Components connected to the AP. We would use the HIDE secure protocol to enumerate I2C addresses and discover components, however, the HIDE secure protocol uses a Component's full ID as associated data during the encryption and decryption process. During this discovery process, we do not know the full ID of any Component with the exception of the Components which are provisioned for the AP. Even then, we cannot use these provisioned Component IDs in the case that a different valid Component shares the same I2C address. So, we decided to implement an extension to the HIDE protocol, which we refer to as the HIDE list protocol. This is only supported when the AP is communicating with a Component.

The AP will send an unencrypted request packet which contains the magic bytes corresponding to a list request. The Component will then respond immediately with its 4-byte Component ID.

```{.mermaid loc=img #fig:hide-list caption="HIDE list protocol"}
sequenceDiagram
	participant AP as Application Processor
	AP ->> C: (Unencrypted): HIDE_PKT_REQ_LIST
	C ->> AP: (Unencrypted): Component ID
	participant C as Component
```