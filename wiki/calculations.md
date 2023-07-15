# Overview:

This chapter talks about operations you can do with variables in SGL.

___
## **cal**

**usage example:**
```Python
// template
[cal][a][op][b]

// specific use
[set][sum][ [cal][2][+][2] ]
```
    

**Args:**

* `a`: Any value
* `op`: `STR` or `CHAR` 
* `b`: Any value


**Desc:**

Applies the operation in `op` on `a` and `b` and returns the result.

Available operations are:
```
+  -  add
-  -  sub
*  -  mul
/  -  div
=  -  eql
!  -  not eql
>  -  bigger than
<  -  smaller than
&  -  and
|  -  or
^  -  power
>= -  bigger or eql to
<= -  smaller or eq to
```

**Return value:** The result of the operation.

For more complex examples check [calculations.glg](../examples/calculations.glg)


___
## **op**

**usage example:**
```Python
// template
[op][a][op][b]

// specific use
[op][my_var][+][1] // adds one to my_var
```
    

**Args:**

* `a`: `STR`
* `op`: `STR` or `CHAR`
* `b`: Any value


**Desc:**

Applies the operation in `op` on `a` and `b` and sets `a` to the result.

Available operations are:
```
+  -  add
-  -  sub
*  -  mul
/  -  div
=  -  eql
!  -  not eql
>  -  bigger than
<  -  smaller than
&  -  and
|  -  or
^  -  power
>= -  bigger or eql to
<= -  smaller or eq to
```

**Return value:** The value of modified `a`.

For more complex examples check [calculations.glg](../examples/calculations.glg)

___
## **sqrt**

**usage example:**
```Python
// template
[sqrt][num]

// specific use
[sqrt][9]
```
    

**Args:**

* `num`: `I32` or `F32`


**Desc:**

Calculates the square root of `num` and returns it.


**Return value:** The square root of `num`, always a `F32`.

For more complex examples check [calculations.glg](../examples/calculations.glg)
