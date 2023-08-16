# Overview:

Being able to create a variable containing a list of values (i.e array) is a super important part of any language. Nest's answer to that is the `LIST` data type. A `LIST` is essentially a vector of values of any kind.

It is also important to mention that every command below (except for list definition) works on strings too and would just treat them as a `LIST` of `CHAR`s.


___
## **Defining a new list**

The easiest way to define a list is by writing a list literal like so:


```Cpp

set lst {1 "2" [cal true & false] {"another" "list"}}
//             ^                  ^
//             |                  or even be another list
//             an element can be a result of a node
```

___
## **lempty**

**usage example:**
```Python
set lst [lempty]
```
    

**Desc:**

Returns an empty list.
**Return value:** `LIST`.


For more complex examples check [lists.nst](../examples/lists.nst)

___
## **llen**

**usage example:**
```Python
// template
llen name

// specific use
set lst {"hello" 1 2 3}
print "{}\n" [llen lst]
```
    
**Args:**

* `name`: `STR` or `CHAR`

**Desc:**

Returns the length of a list called `name`.

**Return value:** `I32`.

___
## **repeat**

**usage example:**
```Python
// template
repeat val times

// specific use
set lst [repeat "a" 1024]
```
    
**Args:**

* `val`: Any value
* `times`: `I32`

**Desc:**

Creates a list `times` long and sets every element to `val`.

**Return value:** `LIST`.

For more complex examples check [lists.nst](../examples/lists.nst)


___
## **repeatl**

**usage example:**
```Python
// template
repeatl val times

// specific use
set lst [repeat {1 2 3} 3]
// creates {1 2 3 1 2 3 1 2 3}
```
    
**Args:**

* `val`: `LIST`
* `times`: `I32`

**Desc:**

Repeats the elements inside `val` `times` times and returns that as a `LIST`.

**Return value:** `LIST`.

For more complex examples check [lists.nst](../examples/lists.nst)

___
## **gete**

**usage example:**
```Python
// template
gete name index

// specific use
set lst {1 2 3}
print "{}\n" [gete lst 2]
```
    
**Args:**

* `name`: `STR` or `CHAR`
* `index`: `I32`

**Desc:**

Returns the value of the element at `index` in a given list called `name`.

**Return value:** Any value.

For more complex examples check [lists.nst](../examples/lists.nst)

___
## **sete**

**usage example:**
```Python
// template
gete name index val

// specific use
set lst {1 2 3}
print "{}\n" [sete lst 2 "a"]
```
    
**Args:**

* `name`: `STR` or `CHAR`
* `index`: `I32`
* `val`: Any value

**Desc:**

Sets the value of the element at `index` in a given list called `name` to the value of `val`.

**Return value:** Value of `val`.

For more complex examples check [lists.nst](../examples/lists.nst)

___
## **lpush**

**usage example:**
```Python
// template
lpush name val

// specific use
set lst {1 2 3}
lpush lst "4"
```
    
**Args:**

* `name`: `STR` or `CHAR`
* `val`: Any value

**Desc:**

Appends a new element with value of `val` to the end a a given list called `name`.

**Return value:** Value of `val`.

For more complex examples check [lists.nst](../examples/lists.nst)

___
## **lpop**

**usage example:**
```Python
// template
lpush name

// specific use
set lst {1 2 3}
set last (lpop lst)
```
    
**Args:**

* `name`: `STR` or `CHAR`

**Desc:**

Removes the last element of a given list called `name` and returns its value.

**Return value:** Value of former last element.

For more complex examples check [lists.nst](../examples/lists.nst)


___
## **lremove**

**usage example:**
```Python
// template
lpush name index

// specific use
set lst {1 2 3}
set last (lremove lst 1)
```
    
**Args:**

* `name`: `STR` or `CHAR`
* `index`: `I32`

**Desc:**

Removes the element at `index` in a given list called `name` and returns its value.

**Return value:** Value of the removed element.

For more complex examples check [lists.nst](../examples/lists.nst)


___
## **lclear**

**usage example:**
```Python
// template
lclear name
```
    
**Args:**

* `name`: `STR` or `CHAR`

**Desc:**

Clears all of the elements in a given list called `name`.

**Return value:** `VOID`.

For more complex examples check [lists.nst](../examples/lists.nst)
