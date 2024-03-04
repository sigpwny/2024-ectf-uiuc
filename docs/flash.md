# Flash State
In order to store state and sensitive data that needs to persist across reboots, we use flash memory with fixed addresses.

Our firmware is limited to flash addresses `0x1000_E000` - `0x1004_5FFF`. On the MAX78000, flash is separated into pages of 8192 bytes. We store our state in the last two flash pages available to us:

- `0x1004_2000` - `0x1004_3FFF`
- `0x1004_4000` - `0x1004_5FFF`

We use two separate flash pages since we need to update Component IDs that were replaced during the Replacement command in the Application Processor. Flash can only be written in one direction (`1` to `0`) and thus require erases followed by writes if we would like to update a specific flash address. Flash can only be erased in full pages, so we use the last page of flash as a read-write page for storing/updating Component IDs. Only the Application Processor has its last page (`0x1004_4000`) available as read-write. The same page (`0x1004_4000`) for the Component is read only and stores the Component's own ID.

## Application Processor Flash State
```
0x1000_0000 ┌───────────────────────┬─────┬────┐
0x1000_E000 │ AP Firmware           │     │ R  │
            │                       │     │    │
            │                       │     │    │
0x1004_0000 ├───────────────────────┼─────┼────┤
0x1004_0000 │ Boot Message          │ 64B │ R  │
            │                       │     │    │
0x1004_1000 │ Ascon Key (AP -> C)   │ 16B │ R  │
0x1004_1080 │ Ascon Key (C -> AP)   │ 16B │ R  │
0x1004_1400 │ Attest PIN Hash       │ 32B │ R  │
0x1004_1480 │ Attest PIN Salt       │ 16B │ R  │
0x1004_1800 │ Replace Token Hash    │ 32B │ R  │
0x1004_1880 │ Replace Token Salt    │ 16B │ R  │
0x1004_2000 ├───────────────────────┼─────┼────┤
0x1004_2000 │ Prov. Component ID 1  │  4B │ RW │
0x1004_4000 ├───────────────────────┼─────┼────┤
0x1004_4000 │ Prov. Component ID 0  │  4B │ RW │
0x1004_6000 └───────────────────────┴─────┴────┘
```

## Component Flash State
```
0x1000_0000 ┌───────────────────────┬─────┬────┐
0x1000_E000 │ Component Firmware    │     │ R  │
            │                       │     │    │
            │                       │     │    │
0x1004_2000 ├───────────────────────┼─────┼────┤
0x1004_2000 │ Boot Message          │ 64B │ R  │
0x1004_2100 │ Attest Location       │ 64B │ R  │
0x1004_2200 │ Attest Date           │ 64B │ R  │
0x1004_2300 │ Attest Customer       │ 64B │ R  │
            │                       │     │    │
0x1004_3000 │ Ascon Key (AP -> C)   │ 16B │ R  │
0x1004_3080 │ Ascon Key (C -> AP)   │ 16B │ R  │
0x1004_4000 ├───────────────────────┼─────┼────┤
0x1004_XXXX │ Component ID (Self)   │  4B │ R  │
0x1004_6000 └───────────────────────┴─────┴────┘
```
