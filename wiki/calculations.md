# Overview:

This chapter covers operations you can perform on variables in Nest.

___
## **cal**

**usage example:**
```cpp
// template
cal a op b

// specific use
set sum  [cal 2 + 2] 
```
    

**Args:**

* `a`: Any value
* `op`: `STR` or `CHAR` 
* `b`: Any value


**Desc:**

Applies the operation in `op` on `a` and `b` and returns the result.

Available operations are:
```python
+  -  add
-  -  sub
*  -  mul
/  -  div
%  -  mod
==  -  eql
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
```cpp
// template
op a op b

// specific use
op my_var + 1 // adds one to my_var
```
    

**Args:**

* `a`: `STR`
* `op`: `STR` or `CHAR`
* `b`: Any value


**Desc:**

Applies the operation in `op` on `a` and `b` and sets `a` to the result.

Available operations are:
```python
+  -  add
-  -  sub
*  -  mul
/  -  div
%  -  mod
==  -  eql
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
```cpp
// template
sqrt num

// specific use
sqrt 9
```
    

**Args:**

* `num`: `I32` or `F32`


**Desc:**

Calculates the square root of `num` and returns it.


**Return value:** The square root of `num`, always a `F32`.

For more complex examples check [calculations.glg](../examples/calculations.glg)

___
## **inv**

**usage example:**
```cpp
// template
inv flag

// specific use
set this_is_true [inv false]
```
    

**Args:**

* `flag`: `BOOL`


**Desc:**
A command that takes a bool `flag` and returns its opposite value.



**Return value:** The opposite of `flag`: `BOOL`

For more complex examples check [calculations.glg](../examples/calculations.glg)

___
## **abs**

**usage example:**
```cpp
// template
abs num

// specific use
set this_is_positive [abs -4.5]
```

**Args:**

* `num`: `I32` or `F32`

**Desc:**
A command that take a numeric value `num` and returns its absolute value.


**Return value:** The abs of `num`: `I32` or `F32`.

For more complex examples check [calculations.glg](../examples/calculations.glg)
