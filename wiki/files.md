# Overview:

Just in case, Nest comes with a simple file I/O API that allows you to create, read and write files.

_Important to note:_ Files are opened by the API **in text mode and not binary**.
It is only possible to open a file in input or output mode, but **not both**.

___
## **openfile**

**usage example:**
```Python
// template
openfile path mode

// specific use
set file [openfile "res/dummy.txt" 'r']
```
    
**Args:**

* `path`: `STR`
* `mode`: `CHAR`

**Desc:**

This command has 2 modes and behaves differently in each:

* `r` : opens an existing file at the specified `path` and adds it to the `input file` stack.
* `w` : opens an existing file in write mode, or creates a new one if the path doesn't exist, and adds it to the `output file` stack.

_Important to note:_ `path` is relative to the path of the `Nest` executable, not the `nst` file.

**Return value:** `VOID`.

For more complex examples check [files.nst](../examples/files.nst)

___
## **readbuf**

**usage example:**
```Python
// template
readbuf path

// specific use
set buffer [readbuf "res/dummy.txt" 'r']
```
    
**Args:**

* `path`: `STR`

**Desc:**

Reads the entirety of an **already opened** input file and returns it as a single `STR`.

**Return value:** `STR`.

For more complex examples check [files.nst](../examples/files.nst)

___
## **writef**

**usage example:**
```Python
// template
writef path val trunc

// specific use
writef "res/dummy.txt" (cal 1 + 1) false
```
    
**Args:**

* `path`: `STR`
* `val`: Any value
* `trunc`: `BOOL`

**Desc:**

Writes `val` to an **already opened** output file at `path`, and erases all text inside the file before writing, if `trunc` is `true`.

**Return value:** `VOID`.

For more complex examples check [files.nst](../examples/files.nst)


___
## **closef**

**usage example:**
```Python
// template
closef path

// specific use
closef "res/dummy.txt"
```
    
**Args:**

* `path`: `STR`

**Desc:**

Closes an opened file at `path` at either mode.

**Return value:** `VOID`.

For more complex examples check [files.nst](../examples/files.nst)
