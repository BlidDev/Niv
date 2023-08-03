# Overview:
This chapter contains a list of commands that don't neccaseraly have a specific subject but are important.

___
## **sleep**


**usage example:**
```Python
sleep duration
```
    

**Args:**

* `duration`: `I32`  


**Desc:**
Halts the entire program for `duration` milliseconds.

**Return value:** `VOID`.

___
## **rng**


**usage example:**
```Python
// template
rng max min

// specific use
rng 0 [cal 200 + 55]
```
    

**Args:**

* `min`: `I32`  
* `max`: `I32`  


**Desc:**
Generates and returns a random number between `min`-`max`.

**Return value:** `I32`.


___
## **exit**


**usage example:**
```Python
exit
```


**Desc:**
Exits the program.

**Return value:** `VOID`.

___
## **cst**


**usage example:**
```Python
// template
cst dst val

// specific use
cst f32 39
```
    

**Args:**

* `dst`: `STR`  
* `val`: Any value


**Desc:**
Tries to cast `val` as `dst` and returns the casted value if possible.

**Return value:** The desired value if succesful.


___
## **err**


**usage example:**
```Python
// template
err msg Args...

// specific use
err "Error: crash code {}" 1
```
    

**Args:**

* `msg`: `STR`  
* `Args`: List of arguments of any kind


**Desc:**
Throws an error containing the formated `msg` as its body.

**Return value:** `VOID`.

___
## **typeid**


**usage example:**
```Python
// template
typeid val

// specific use
set val 3.4
print "ID is: {}\n" (typeid $val)
```

**Args:**

* `val`: Any value  

**Desc:**
Retrives the type index of `val` and returns it.

Type indecies are:
```
VOID   -> 0
I32    -> 1
F32    -> 2
BOOL   -> 3
CHAR   -> 4
STR_   -> 5
UTYPE  -> 6
LIST   -> 7
NODE   -> 8
RETURN -> 9
```

**Return value:** `I32`.

___
## **chain**


**usage example:**
```C
// template
chain (...)...

// specific use
chain (set a 5) (op a + 5) (print "a: {}\n", $a)

// or even
set results (chain (set a 5) (op a + 5) (print "a: {}\n", $a))
print "results are: {}\n" $results
```
    

**Args:** 

* `(...)` : Zero or more nodes


**Desc:**
Takes a list of nodes and exceutes them from left to right.

**Return value:** `LIST` of all the nodes return values from left to right.

