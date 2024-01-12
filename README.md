# Chaum-Pedersen Authentication service

## Overview
The Chaum-Pedersen Authentication service is a command-line application that facilitates a secret authentication using Chaum-Pedersen service. It offers support for both Discrete Log and Elliptic Curve implementations of the protocol.

Protocol is implemented for `pasta` (`Pallas` and `Vesta`) curves, but other elliptic curves or groups can be added easily.

One can run `Docker` containers with server and client communicating with each other. 

## Approach

This repository had in mind, that the protocol implemented here could and should be used outside of the authentication setting. Therefore strict crate differentiation approach was taken.

Wherever possible unit and functional tests were added to ensure soundness of the code.

Approach to documentation was closer to [self-documentation](https://en.wikipedia.org/wiki/Self-documenting_code), as I believe quality code and thoughtful naming can explain code better, that numerous code comments. Of course, whenever it was not obvious comments were added.

## Internal notation and algorithm

All the specific naming notation (e.g. for group element genetators, cryptographic terminology etc.) is acoorfing to ["Cryptography: An Introduction (3rd Edition) Nigel Smart"](https://www.cs.umd.edu/~waa/414-F11/IntroToCrypto.pdf) page 377 section "3. Sigma Protocols" subsection "3.2. Chaum–Pedersen Protocol.".

## Repository composition

The repository consists of 3 crate:
- `chaum-pedersen`
- `server`
- `client`

`chaum-pedersen` crate implements the samenamed protocol, and `server` and `client` are gRPC counterparts.

## Storage

Currently only in-memory storage is implemeted but, as the architecture is abstract it is very easy to add a database or file implementation.

## Code style and organisation

`Cargo fmt`, `udeps` and `taplo` were used to make sure consistent style was used in Rust, TOML files and no unused dependenices creeped into. 

Code was organised so that no hardwires existed beteween protocol implementation and gRPC level.

All critical functionality (protocol itself, storage, APIs) were abstracted so, that easy addition of new implementation was possible.

## Error handling

`anyhow` and `thiserror` were the staple of standardised error handling in the project.

## Usage

0. **Prerequesites**
    `Rust` and `protoc` have to be installed, details depend on your operating system.

1. **Building**
   Open a terminal and navigate to the root directory of the project.

   ```bash
   cd path/to/your/repo
   cargo build --release
   ```
2. **Start the server with default parameters**
   ```bash
   ❯ ./target/release/server
    Starting server 
          host: [::1]
          port: 50051
          flavor: discrete_log
   ```

3. **In another terminal send a request with the client using default parameters**
   ```bash
   ❯ ./target/release/client
   Starting client
          host: [::1]
          port: 50051
          flavor: discrete_log
          user: foo
   Authentication successful!
   Session ID: 6c98b794-7c75-4928-9409-936e9470b70a
   ```
## Functional and Unit Tests

`chaum-pedersen` crate contains both unit and functional tests for all its implementations (3 in total).

**Run the tests**
   Open a terminal and navigate to the root directory of the project.

   ```bash
   cd path/to/your/repo
   cargo test --release
   ```

# Docker

### Running the docker-compose Setup

1. **Navigate to Your Project Directory:**
   Open a terminal and navigate to the root directory of the project, where the `docker-compose.yml` file is located.

   ```bash
   cd path/to/your/directory
   ```

2. **Build the Docker Images:**
   Run the following command to build the Docker images as defined in your `docker-compose.yml` file.

   ```bash
   docker-compose build
   ```

3. **Start the Services:**
   To start the services (server and client containers) as defined in the `docker-compose.yml`, run:

   ```bash
   docker-compose up
   ```

4. **Shutting Down:**
   When you're done, you can shut down the containers by pressing `Ctrl+C` in the terminal where `docker-compose up` is running. Alternatively, you can run the following command in another terminal (still in the project root):

   ```bash
   docker-compose down
   ```
