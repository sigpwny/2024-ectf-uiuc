# :runner: Attest Subteam Survival Guide

Hi, I'm Emma, and welcome to the attest subteam :tada:! I want to set you up for success as much as I can, so here is a guide that hopefully will help you do so. This document is a perpetual work in progress so feel free to recommend additional resources and sections.

## Table of Contents

I wasn't expecting to have so much to say, but I think everything here is important enough to keep around. Here's a table of contents to help you navigate.

- [Important Links](#important-links)
- [Functional Stuff](#functional-stuff)
  - [Attestation](#attestation)
  - [Replace Component](#replace-component)
  - [List Components](#list-components)
  - [Security Requirements](#security-requirements)
- [Development Stuff](#development-stuff)
  - [Documentation](#documentation)
  - [Rust (and Systems Programming Generally)](#rust)
  - [Nix](#nix)
  - [Embedded Systems](#embed)
- [Security Stuff](#security-stuff)
  - [CIA Triad](#cia)
  - [Timing Attacks (and Side Channels Generally)](#timing)
  - [Hashing and Argon2](#hash)
  - [Salting](#salt)

## Important Links

I hate hunting down links as much as you probably do, so here's the important ones.

- [GitHub issues](https://github.com/sigpwny/2024-ectf-uiuc/issues)
- [Project tracker](https://github.com/orgs/sigpwny/projects/3)
- [Full eCTF guide/rulebook](https://ectfmitre.gitlab.io/ectf-website/2024/index.html)
  - [System architecture](https://ectfmitre.gitlab.io/ectf-website/2024/specs/system_architecture.html)
  - [Functional requirements](https://ectfmitre.gitlab.io/ectf-website/2024/specs/functional_reqs.html)
  - [Detailed specs](https://ectfmitre.gitlab.io/ectf-website/2024/specs/detailed_specs.html)
  - [Security requirements](https://ectfmitre.gitlab.io/ectf-website/2024/specs/security_reqs.html)
  - [Attack flags](https://ectfmitre.gitlab.io/ectf-website/2024/flags/attack_flags.html)
- [Insecure example code](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example)
  - [Application processor](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/application_processor/src/application_processor.c)
  - [Component](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/application_processor/src/component.c)

## Functional Stuff

There are four pre-boot functional elements that we as a larger team need to support for the board. The attestation subteam will be in charge of three of those commands (because we're just that cool :sunglasses:). Each element has timing requirements and message output requirements we need to adhere to.

### Attestation

- [High-level description](https://ectfmitre.gitlab.io/ectf-website/2024/specs/functional_reqs.html#attest)
- [Detailed description](https://ectfmitre.gitlab.io/ectf-website/2024/specs/detailed_specs.html#attestation)
- Insecure Example
  - [application_processor.c lines 300-323](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/application_processor/src/application_processor.c#L300)
  - [application_processor.c lines 386-395](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/application_processor/src/application_processor.c#L386)
  - [applicaton_processor.c lines 474-486](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/application_processor/src/application_processor.c#L474)
  - [component.c lines 198-203](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/component/src/component.c#L198)

We need to be able to...

- Print all fields of a component's attestation data
- Only print the attestation data if the Attestation PIN is valid

Functionally, we need to provide these outputs:

|Level|Message format|Example|
|-----|--------------|-------|
|Info| `C>{Component ID prefixed by 0x}\n` |`C>0x02\n` |
|Info| `LOC>{Attestation location}\n` | `LOC>Boston\n` |
|Info| `DATE>{Attestation date}\n` | `DATE>01/01/1970\n` |
|Info| `CUST>{Attestation customer}\n` | `CUST>MITRE\n` |
|Success| `Attest\n` | `Attest\n`|
|Error| Any error message | `Attest failed\n` |


### Replace Component

- [High-level description](https://ectfmitre.gitlab.io/ectf-website/2024/specs/functional_reqs.html#replace)
- [Detailed description](https://ectfmitre.gitlab.io/ectf-website/2024/specs/detailed_specs.html#replace-component)
- Insecure example
  - [application_processor.c lines 398-407](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/component/src/appliction_processor.c#L398)
  - [application_processor.c lines 437-471](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/application_processor/src/application_processor.c#L437)
  - [component.c lines 198-203](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/component/src/component.c#L198)

We need to be able to...

- Replace a component (see [components](https://ectfmitre.gitlab.io/ectf-website/2024/specs/system_architecture.html#components)) currently connected to the system (see [MISC](https://ectfmitre.gitlab.io/ectf-website/2024/specs/system_architecture.html#medical-device))
- Only replace a component if the replacement token is valid

Functionally, we need to provide these outputs:

|Level|Message format|Example|
|-----|--------------|-------|
|Success| `Replace\n` | `Replace\n`|
|Error| Any error message | `Replace failed\n` |

### List Components

- [High-level description](https://ectfmitre.gitlab.io/ectf-website/2024/specs/functional_reqs.html#list-components)
- [Detailed descripton](https://ectfmitre.gitlab.io/ectf-website/2024/specs/detailed_specs.html#list-components)
- Insecure example
  - [application_processor.c lines 207-240](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/application_processor/src/application_processor.c#L207)
  - [component.c lines 184-189](https://github.com/mitre-cyber-academy/2024-ectf-insecure-example/blob/release/component/src/component.c#L184)

We need to be able to...

- Print all the components (see [components](https://ectfmitre.gitlab.io/ectf-website/2024/specs/system_architecture.html#components)) currently connected to the system (see [MISC](https://ectfmitre.gitlab.io/ectf-website/2024/specs/system_architecture.html#medical-device))
- Print all the components the system was provisioned for

Functionally, we need to provide these outputs:

|Level|Message format|Example|
|-----|--------------|-------|
|Info| `P>{Provisioned Component ID prefixed by 0x}\n` |`P>0x02\n` |
|Info| `F>{Found Component ID prefixed by 0x}\n` | `F>0x02\n` |
|Success| `List\n` | `List\n`|
|Error| Any error message | `Internal error\n` |

### Security Requirements

- [Security requirement 3](https://ectfmitre.gitlab.io/ectf-website/2024/specs/security_reqs.html#security-requirement-3)
- [Security requirement 4](https://ectfmitre.gitlab.io/ectf-website/2024/specs/security_reqs.html#security-requirement-4)

## Development Stuff

<div id='documentation'/>

### :pencil: Documentation

Perhaps a surprising place to start for an embedded capture-the-flag competition, but this is probably the most important thing we will do. Even in computing, clear communication is king for everything from debugging to knowledge transfer. The same goes for eCTF, where documentation plays a big part in scoring for finalists. 

If you want an idea of what documentation for this type of project looks like, this is what our [final README](https://github.com/sigpwny/2023-ectf-sigpwny/blob/main/README.md) from last year looked like. I'm not expecting you to be super polished, but make sure you're keeping track of what you're doing and why you're doing it within the codebase and keep your commits somewhat descriptive (I won't be strict about it just make sure I could figure out what you did if I had to).

As you might have noticed in the README, we also like to work with mermaid diagrams for visualizing protocols. If you're not familiar with mermaid, it's a way to make diagrams using text. It's a bit like markdown, but for diagrams. GitHub has integrated Mermaid support, so you can make diagrams in your markdown files and they'll render in the browser.

Here's some resources to get you started with Mermaid:

- [Mermaid live editor](https://mermaid-js.github.io/mermaid-live-editor/)
- [Mermaid documentation](https://mermaid-js.github.io/mermaid/#/)

<div id='rust'/>

### :crab: Rust (and Systems Programming Generally)

Based on your survey responses, many of you don't have experience writing Rust code. I'm excited to introduce you to the language! I personally think Rust is more analogous to C++ than C, so if you've taken CS 128 or CS 225 or learned C++ some other way the skillset should translate. If you've taken ECE 220 or learned C some other way I don't think you'll run into too much trouble either. We will be installing Rust using the Nix environment proviced, so you don't need to worry about installing it.

Here are some resources depending on what you're looking for!

- [Official documentation book](https://doc.rust-lang.org/stable/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

<div id='nix'/>

### :snowflake: Nix

We will be using Nix to manage our development environment. Nix (among other things) is a package manager that allows us to create isolated environments for our projects. For eCTF, it makes it easier for the organizers to ensure that everyone is working with the same environment and that we don't have to worry about dependencies conflicting with each other. Think about it kind of like Docker if you're familiar with that, or like a virtual environment in Python. Most of the Nix we do will be provided by the eCTF organizers, but we will need to install it and use it to manage our environment.

- [Install Nix (just install the package manager, not the whole NixOS)](https://nixos.org/download.html)
- [Nix manual](https://nixos.org/manual/nix/stable/)

<div id='embed'/>

### :bed: Embedded Systems

Embedded systems are new to a lot of people just getting started with eCTF, and to be honest they're not exactly my area of expertise as a CS major. The ECE majors in the room probably have a more solid background for how the hardware side of things works. 

If you want a virtual way to play with stuff like Arduinos, I've heard good things about [Wokwi](https://wokwi.com/), which lets you use Rust or Python to program virtual microcontrollers and wire projects. We probably won't be thinking a ton about embedded systems for our subteam, but it's good to have a general idea of how they work.

## Security Stuff

<div id='cia'/>

### :lock: CIA Triad

If you're not familiar with the CIA triad, it's a good place to start for understanding the security requirements of the eCTF. CIA stands for Confidentiality, Integrity, and Availability.

__Confidentiality__ is about ensuring that only authorized users can access data. This is a big part of what we'll be doing with the attestation subteam! We need to make sure that the attestation data can only be accessed with the Attestation PIN and can't be brute forced or otherwise bypassed.

__Integrity__ is about ensuring that data is not tampered with. If we can't trust that the data we're getting is accurate, we can't trust the system, which in a medical context is a big problem. We need to make sure that the attestation data can't be directly retrieved from the component, bypassing the application processor. 

__Availability__ is about ensuring that data is available when it's needed. There's no point in having a secure system if it's not usable! We need to make sure that the attestation data can be received by an authorized user with the Attestation PIN and that the replacement mechanism works when it's needed.

When gaining an intuition for why vulnerabilites and security measures work, it's helpful to think about them in terms of the CIA triad. Make it a habit to look at a piece of code and think about its implications for confidentiality, integrity, and availability.

<div id='timing'/>

### :hourglass: Timing Attacks (and Side Channels Generally)

Okay so let's say I have this function that checks for non-zero elements in an array. It looks like this:

```c
int array_checker(int array[]) {
  for (int i = 0; i < 100; i++) {
    if (array[i] != 0) {
      return 1;
    }
  }
  return 0;
} 
```

Let's say that given an arbitrary input, an array of 0s with a single non-zero element was generated. It's important that the attacker doesn't know where the non-zero element is. How could an attacker figure out where the element is anyways?

Really think about it before you read on. A hint is that each line of code will take a certain amount of time to execute, let's say one unit of time per line to keep it simple (just know that this wil vary in practice).

In this case, the attacker could see how long it takes for the function to return. The longer it takes, the farther into the array the first non-zero element is. This is a timing attack, which is a type of side channel attack. Another side channel attack is a power analysis attack, where an attacker can figure out what a device is doing based on how much power it's using.

What might a more secure version of this function look like? A good place to start is making sure that the amount of time the function takes to execute doesn't give the attacker any information about the input. You could make the function take a constant amount of time to execute, or you could make it so that the time it takes to execute is random.

<div id='hash'/>

### :potato: Hashing and Argon2

If you aren't familiar with __hashing functions__, they're a way to take a piece of data and turn it into a fixed length string of characters. The inner workings are a bit outside of the scope of this competition, but if you're really interested in the math behind it, I can point you to some resources.

We use hashing functions so that we don't have to store sensitive data like PINs and tokens in plaintext. A good hashing function should be hard to restore to the original data and should be hard to find duplicate inputs for (though because there are infinite strings of any length and limited strings of a fixed length, there will always be some). __Argon2__ is a password hashing function that's resistant to brute force and side channel attacks, so it's a good choice for hashing the PINs and tokens we'll be working with.

One important aspect of hashes is that if you change a single character in the input, the hash will be completely different:

```md
Input: "radsecret"
Argon2 hash: "bc7702f551781c9e7576898bcbeee3bf"

Input: "ratsecret"
Argon2 hash: "05402972bc79f150f9cd0c9a1e72b32f"
```

This is called the __avalanche effect__. This is important because it means that the attacker will need to know the exact input to get the correct hash, and so protecting against brute force attacks becomes the priority. 

<div id='salt'/>

### :salt: Salting

We can make a hash even more robust using a technique called __salting__. Just for a bit of background, a common attack on hashed passwords is to use a precomputed table of hashes to find the original input. This is called a __rainbow table__, and it saves time for the attacker because they don't have to hash every possible input to find the original input.

A salt is a random string we can add to an input before hashing it. This makes it harder for an attacker to use a precomputed table of hashes to find the original input. This works even if the attacker knows the salt, because they still cannot save time by using precomputed hashes to find the original input. This is all as simple as generating a random string and adding it to the input before hashing it!

In another example of the avalanche effect, here are two hashes of the same input with different salts:

```md
Input: "radsecret"
Salt: "mycoolsalt"
Argon2 hash: "bc7702f551781c9e7576898bcbeee3bf"

Input: "radsecret"
Salt: "peppered"
Argon2 hash: "933b04ceeb18d8ed7eb28944cef2eab5"
```