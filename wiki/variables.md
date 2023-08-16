# Overview:

A variable in Nest can be one of the following types:
```Cpp
VOID
I32
F32
BOOL
CHAR
STR
UTYPE  // will be explained in a later chapter
LIST
NODE   // only used for behind-the-scenes operations
RETURN   // only used for behind-the-scenes operations
```
All variables are type dynamic meaning they can change their type based on the value you set them to. 

___
### **Accessing a variable's value**

In order to access a variable's actual value in `Nest`, write **`$`** followed by your variable's name (for example `$myvar`). This will tell `Nest` to look through the stack for your variable name and return its value if it exists.

___
## **set**

**usage example:**
```Python
set name val
```
    

**Args:**

* `name`: `STR` / `CHAR`
* `value`: Any value


**Desc:**

Creates a variable called the value of `name` (or overrides an existing one) and inserts the value of `val` into it.

**Return value:** The value of `val`.

For more complex examples check [variables.glg](../examples/variables.glg)

___
## **release**


**usage example:**
```Python
release name
```
    

**Args:**

* `name`: `STR`  


**Desc:**
Releases the given variable called `name` from memory and returns its value; crashes if the variable doesn't exist.

**Return value:** The value of the released variable.

For more complex examples check [variables.glg](../examples/variables.glg)

___
## **reset**

**usage example:**
```Python
reset
```
    

**Desc:**
Releases all variables in the program.

**Return value:** `VOID`.

For more complex examples check [variables.glg](../examples/variables.glg)