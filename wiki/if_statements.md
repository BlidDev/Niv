# Overview:
There are 2 ways of implementing an if-statement in SGL, as shown below:

```Cpp

// using a multiline if

[if][5][>][0] 
{
    [print]["This will run\n"]

[if][[cal][5][*][5]][<][0] 
{
    [print]["This won't\n"]
}

// or using a single line if

[singleif][5][>][0][[print]["This will run\n"]]
[singleif][5][<][0][[print]["This won't\n"]]

```

___
## **if**

**usage example:**
```Python
// template
[if][a][op][b]

// specific use
[if]["hey"][!]["bye"]
```
    

**Args:**

* `a`: `STR`
* `op`: `STR` or `CHAR`
* `b`: Any value


**Desc:**

Runs [`cal`](calculations.md) on `a` `op` and `b` and if it returns `true` the program runs the given scope.

**Return value:** `VOID`.

For more complex examples check [if_statements.glg](../examples/if_statements.glg)


___
## **singleif**

**usage example:**
```Python
// template
[singleif][a][op][b][[...]]

// specific use
[singleif]["hey"][=]["bye"][[print]["doesn't work\n"]]
```
    

**Args:**

* `a`: `STR`
* `op`: `STR` or `CHAR`
* `b`: Any value
* `[...]`: Any node


**Desc:**

Runs [`cal`](calculations.md) on `a` `op` and `b` and if it returns `true` the program traverses the given node.

**Return value:** The returned value of the optional node.

For more complex examples check [if_statements.glg](../examples/if_statements.glg)
while 
