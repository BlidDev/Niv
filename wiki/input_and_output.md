# Overview:

This chapter focuses on the ways you can manage input and output in SGL.

___
## **post**

**usage example:**
```Python
[post]
```
    

**Desc:**

Prints out the program's variable stack.


**Return value:** `VOID`

For more complex examples check [input_output.glg](../examples.input_output.glg)

___
## **print**

**usage example:**
```Python
// template
[print][format][Args]...

// specific use
[print]["This doesn't have any trailing args\n"]
[print]["This {}\n"]["does"]
```
    

**Args:**

* `format`: `STR`
* `Args`:  List of arguments of any value


**Desc:**

Prints out the given `format` and replaces any `{}` with the currosponding trailing argument.

**Return value:** `VOID`


For more complex examples check [input_output.glg](../examples.input_output.glg)


___
## **format**

**usage example:**
```Python
// template
[format][format_string][Args]...

// specific use
[set][formatted_str][[format]["hey {}\n"]["there"]]
```
    

**Args:**

* `format_string`: `STR`
* `Args`:  List of arguments of any value


**Desc:**

Does the same thing as `print` but instead of printing it to the screens it returns the formatted string as a variable.

**Return value:** `STR` containing the formated string.


For more complex examples check [input_output.glg](../examples.input_output.glg)

___
## **input**

**usage example:**
```Python
// template
[input][msg]

// specific use
[set][name][ [input]["Enter your name: "] ]
```
    

**Args:**

* `msg`: `STR`


**Desc:**

Prints the given message in `msg` and requests input from the user.

**Return value:** `STR` containing the user's input.


For more complex examples check [input_output.glg](../examples.input_output.glg)
___
## **inputcast**

**usage example:**
```Python
// template
[input][msg][type]

// specific use
[set][num][ [inputcast][I32]["Enter your age: "] ]
```
    

**Args:**

* `msg`: `STR`
* `type`: `STR`


**Desc:**

Prints the given message in `msg` and requests input from the user and returns the input casted to the desired `type` if possible.

**Return value:** The value of the casted input.


For more complex examples check [input_output.glg](../examples.input_output.glg)

