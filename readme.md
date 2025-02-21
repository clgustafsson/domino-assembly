# Domino assembly

### 16-bit assembly language using only domino symbols

### 4 Registers (2 bits)
🁤 🁥 🁦 🁧, (32 bit)
### 8 Operations (3 bits)
🁣 🁪 🁱 🁸 🁿 🂆 🂍 🁢



| **Instructions**                    ||||**Description**|
| - | - |- |- |-|
| `🁣` _(op, 3 bits)_| `R1` _(reg, 2 bits)_.|`R2` _(reg, 2 bits)_.|`I` _(imm, 9 bits)_.|`R1` = `R2` + `I`|
| `🁪` _(op, 3 bits)_| `R1` _(reg, 2 bits)_.|`R2` _(reg, 2 bits)_.|`R3` _(reg, 2 bits)_.|`R1` = `R2` + `R3`|
| `🁱` _(op, 3 bits)_|`I` _(imm, 13 bits)_.|||JUMP `I` lines|
| `🁸` _(op, 3 bits)_| `R1` _(reg, 2 bits)_.|`R2` _(reg, 2 bits)_.|`I` _(imm, 9 bits)_.|JUMP `I` lines if `R1` == `R2`|
| `🁿` _(op, 3 bits)_| `R1` _(reg, 2 bits)_.|`I` _(imm, 11 bits)_.||`R1` = `I`|
| `🂆` _(op, 3 bits)_||||Input int to `🁤`|
| `🂍` _(op, 3 bits)_||||Output int from `🁤`|
| `🁢` _(op, 3 bits)_||||Exit program|


### Immediates
###### Syntax
Flip the dominos to the side, the left side represents 1s and the right side 0s
Then build a series of 1s and 0s using the dominos. The series will be sign extended.
###### Examples
🀸 = -1,
🀲🀸 = 1,
🀲🀹 = 2,
🀾🀵🀹 = -3,
🀲🁆 = 7,
🀲 = 0,🀾🀳🀹🀹 = -11, As you can see, it is very intuitive and will probably soon replace numbers in all languages

### Comments
Although it is not recommended to add comments due to the self-explainable nature of the language, It is possible to add comments by simply writing anything after each instruction as anything after the dominos will be ignored

### Example code
Please see [faculty.domino](faculty.domino)

## Interpreter

Complete interpreter written in rust with error handling

### How to use
```zsh
cargo run <filepath>
```
##### Note: Only files ending with .domino can be used

#### Run the faculty example included in this repo
```zsh
cargo run faculty.domino
```
