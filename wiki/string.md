# Overview:

Out of the gate, Nest comes with some basic string manipulation command.


___
## **stolist**

**usage example:**
```Python
// template
stolist val

// specific use
print "{}\n" [stolist "hello mother"]
```
    
**Args:**

* `val`: `STR` or `CHAR`

**Desc:**

Takes a `STR`/`CHAR` `val` and converts it into a list of `CHAR`s.


**Return value:** `LIST`.

For more complex examples check [str.nst](../examples/str.nst)

___
## **ltostr**

**usage example:**
```Python
// template
ltostr lst

// specific use
print "{}\n" [ltostr [stolist "hello mother"]]
```
    
**Args:**

* `lst`: `LIST` of `CHAR`s

**Desc:**

Takes the `LIST` value contained inside `lst` and converts it into a string.


**Return value:** `STR`.

For more complex examples check [str.nst](../examples/str.nst)

___
## **lines**

**usage example:**
```Python
// template
lines val

// specific use
print "{}\n" [lines "hello   \n there"]
```
    
**Args:**

* `val`: `STR`

**Desc:**

Splits a given `STR` `val` by newline characters and returns it as a `LIST` of `STR`s.


**Return value:** `LIST`.

For more complex examples check [str.nst](../examples/str.nst)


___
## **words**

**usage example:**
```Python
// template
words val

// specific use
print "{}\n" [words "hello   \"th ere\""]
```
    
**Args:**

* `val`: `STR`

**Desc:**

Splits a given `STR` `val` by whitespace characters (but also preserving text in quates) and returns it as a `LIST` of `STR`s.


**Return value:** `LIST`.

For more complex examples check [str.nst](../examples/str.nst)

___
## **s_words**

**usage example:**
```Python
// template
s_words val

// specific use
print "{}\n" [s_words "hello  (fj  fd) \"th ere\""]
```
    
**Args:**

* `val`: `STR`

**Desc:**

Same as `words` but also preserves text inside brackets.


**Return value:** `LIST`.

For more complex examples check [str.nst](../examples/str.nst)
___
## **trim**

**usage example:**
```Python
// template
trim val

// specific use
print "{}\n" [dbg [trim "\thello there    \n  "]]
```
    
**Args:**

* `val`: `STR`

**Desc:**

Trims whitespace characters off start and end of `STR` conatined inside `val` and returns the trimmed `STR`.


**Return value:** `STR`.

For more complex examples check [str.nst](../examples/str.nst)
