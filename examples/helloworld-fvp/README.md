# <Hello, world!> example for Morello FVP

This example includes a minimal UART<to_be_decided> driver for the Morello FVP platform and prints a short
message to the console, using the `aarch64-purecap-rt` runtime. The program boots in EL2 hybrid mode and goes
into full capability EL1 mode to print the message.

## Running the example

For running this example, make sure you have used the `cheribuild` tool to install the FVP model.

### Prerequisites

<todo>
