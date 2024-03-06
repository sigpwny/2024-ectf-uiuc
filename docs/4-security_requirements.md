# Security Requirements

Below we summarize how we achieve the outlined security requirements with our design.

## Security Requirement 1

> *"The Application Processor (AP) should only boot if all expected Components are present and valid."*

Because of the integrity, authenticity, and non-replayability properties of the HIDE protocol, we ensure that the AP is actively communicating with valid, provisioned Components from the same deployment. A counterfeit Component will not be built on the same deployment and thus will not have access to the Ascon-128 keys required for the encryption layer.

## Security Requirement 2

> *"Components should only boot after being commanded to by a valid AP that has confirmed the integrity of the device."*

After the AP verifies that each Component is connected and valid, the AP will command each Component to boot. Each Component takes advantage of the integrity, authenticity, and non-replayability properties of the HIDE protocol to confirm that the AP has truly commanded the Component to boot.

## Security Requirement 3

> *"The Attestation PIN and Replacement Token should be kept confidential."*

We use multiple SHA3-512 hashes to validate the Attestation PIN and Replacement Token, preventing compromise of the real PIN and token through side-channel analysis. We also use timers and random delays to effectively prevent brute force and further side-channel attacks.

## Security Requirement 4

> *"Component Attestation Data should be kept confidential."*

Attestation Data is only provided to the AP if the Component receives a command from the AP instructing it to provide Attestation Data. This command is only sent if the AP successfully validates the Attestation PIN. This command cannot be forged due to the properties of the HIDE protocol.

## Security Requiremenet 5

> *"The integrity and authenticity of messages sent and received using the post-boot MISC secure communications functionality should be ensured."*

The HIDE communication layer uses authenticated encryption with Ascon-128 to verify the authenticity of the sender and receiver. The challenge-response nature of the HIDE protocol and the use of nonces prevents the replay of messages.
