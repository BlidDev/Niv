# How it works
Before starting to code in `Nest` it is really important you know how some core language aspects work.

___
## The node system

By defenition, everything in a `Nest` script is a node, nodes are sperarated by whitespaces and can be divided into 2 types:

* **Value**: A simple value such as a number or a string.

* **Nested**: A node that holds other nodes, marked by surrounding `()` or `[]`, most often a command such as `(cal 1 + 1)`. 

All nested nodes can and will be traversed util they return a **Value** node, for example:

```Python
// The following line
// (The reason `set` is not surrounded by `()` is because 
// the first node is always considered a nested one)

set my_variable (cal 1 + (cal 3 * 5))

// Will convert into

set my_variable (cal 1 + 15)
// |
// V
set my_variable 16
// The `set` command returns the variable value so:

16
```


___
## How Nest destiguishes between data types

By default, Nest's parser consideres every value node to be a string data type (even values that are not surrounded by quotes). Then the parser goes over the value nodes and checks if it can be determined as another data type such as `I32`. 
It is also important to note that the only way to define a `CHAR` rather than a single character `STR` is by surrounding your character with `''`.

