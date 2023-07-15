# Overview:
It is possible to create both `for` style and `while` loops in SGL using these 2 methods:

```Cpp

// either create a normal while loop

[while][true]
{
    // runs forever
}

[set][ans][]
[while][[cal][$ans][!]["exit"]]
{
    [set][ans][[input]["Enter your input:> "]]
}


// or create a for loop

[set][i][0]
[while][[cal][$i][<][10]]
{
    // runs 10 times
    [print]["i - {}\n"][$i]
    [op][i][+][1]
}


```

___
## **while**

**usage example:**
```Python
// template
[while][statement]

// specific use
[while][[cal][5][>][10]]
```
    

**Args:**

* `statement`: `BOOL`


**Desc:**

Loops the provided scope as long as `statement` is `true` 

**Return value:** `VOID`.

