# dike/vm

The virtual machine that executes dike vm instructions.

TODO:

- Instruction enum is wrong. Updated instructions are in Notion
- Instruction implementation only implements add
- Math module is terrible. More than just integers should be able to be added, subtracted, and divided
- Sweep executor module needs to be built. It should:
  - Allocate (at the beginning) and deref (at the end) local variables
  - Map internal memory names to the allocated addresses
  - Expose a `tick` function that executes instructions
  - Starts new sweeps on the `GoTo` instruction
