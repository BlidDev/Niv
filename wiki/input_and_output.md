# Overview:

This chapter focuses on the ways you can manage input and output in Nest.

___
## **post**

**usage example:**
```Python
post
```
    

**Desc:**

Prints out the program's variable stack.


**Return value:** `VOID`.

For more complex examples check [input_output.glg](../examples.input_output.glg)

___
## **print**

**usage example:**
```Python
// template
print format Args...

// specific use
print "This doesn't have any trailing args\n"
print "This {}\n" "does"
```
    

**Args:**

* `format`: `STR`
* `Args`:  List of arguments of any value


**Desc:**

Prints out the given `format` and replaces any `{}` with the corresponding trailing argument.

**Return value:** `VOID`.


For more complex examples check [input_output.glg](../examples.input_output.glg)


___
## **dbg**

**usage example:**
```Python
// template
dbg val

// specific use
print "dbg: {}\n" [dbg 4]
```
    

**Args:**
* `val`: Any value


**Desc:**

Returns a string containing the debug info of `val`, similar to using `"{:?}"` in `Rust`.

**Return value:** `STR` containing the debug info.


For more complex examples check [input_output.glg](../examples.input_output.glg)


___
## **prt**

**usage example:**
```Python
// template
prt val

// specific use
print "prt: {}\n" [prt 4]
```
    

**Args:**
* `val`: Any value


**Desc:**

Returns a string containing the debug info of `val` in a "pretty" format, similar to using `"{:#?}"` in `Rust`.

**Return value:** `STR` containing the pretty debug info.


For more complex examples check [input_output.glg](../examples.input_output.glg)

___
## **format**

**usage example:**
```Python
// template
format format_string Args...

// specific use
set formatted_str [format "hey {}\n" "there"]
```
    

**Args:**

* `format_string`: `STR`
* `Args`:  List of arguments of any value


**Desc:**

Does the same as `print`, but instead of printing it to the screen, it returns the formatted string as a variable.

**Return value:** `STR` containing the formated string.


For more complex examples check [input_output.glg](../examples.input_output.glg)

___
## **input**

**usage example:**
```Python
// template
input msg, Args...

// specific use
set name  [input "Enter your {}: " "name"] 
```
    

**Args:**

* `msg`: `STR`
* `Args`: List of arguments of any value


**Desc:**

Prints the formatted given message in `msg` and requests input from the user.

**Return value:** `STR` containing the user's input.


For more complex examples check [input_output.glg](../examples.input_output.glg)
___
## **inputcast**

**usage example:**
```Python
// template
inputcast type msg Args...

// specific use
set num [inputcast I32 "Enter your {}: ", "age"] 
```
    

**Args:**

* `type`: `STR`
* `msg`: `STR`
* `Args`: List of arguments of any value


**Desc:**

Prints the given formated message in `msg`, requests input from the user, and returns the input casted to the desired `type` if possible.

**Return value:** The value of the casted input.


For more complex examples check [input_output.glg](../examples.input_output.glg)

