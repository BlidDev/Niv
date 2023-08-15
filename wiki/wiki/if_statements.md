# Overview:
There are 2 ways of implementing an if-statement in SGL, as shown below:

**Important:** Multiline ifs and elses are highly case sensitive, meaning that the each curly bracket has got to be on a line of its own and `elses` and `else ifs` have to go in between 2 scopes.

```Cpp

// using a multiline if

if [cal 5 * 5] < 0
{
    print "This won\'t run\n"
}
else if 5 = 0 
{
    print "This too\n"
}
else
{
    print "But this will\n"
}

// or using a single line if

singleif 5 > 0 [print "This will run\n"] // only single "true" node
singleif 5 < 0 [print "This won\'t\n"] [print "But worry not\n"]  // both "true" and optional "false" node

```

___
## **if**

**usage example:**
```Python
// template
if a op b

// specific use
if "hey" ! "bye"
```
    

**Args:**

* `a`: Any value
* `op`: `STR` or `CHAR`
* `b`: Any value


**Desc:**

Runs [`cal`](calculations.md) on `a` `op` and `b` and if it returns `true` the program runs the given scope, if not, the program will check wether there's an `else` statement and run it.

**Return value:** `VOID`.

For more complex examples check [if_statements.glg](../examples/if_statements.glg)


___
## **singleif**

**usage example:**
```Python
// template
singleif a op b [...] [...]?

// specific use
singleif "hey" = "bye" [print "doesn\'t work\n"] 
singleif "hey" = "bye" [print "doesn\'t work\n"] [print "works\n"] 
```
    

**Args:**

* `a`: `STR`
* `op`: `STR` or `CHAR`
* `b`: Any value
* `[...]`: Any node
* `[...]?`: (Optional) Any node


**Desc:**

Runs [`cal`](calculations.md) on `a` `op` and `b` and if it returns `true` the program traverses the given node, otherwise it'll traverse the "false" node if present.

**Return value:** The returned value of the traversed node or `VOID` if nothing is traversed.

For more complex examples check [if_statements.glg](../examples/if_statements.glg)
while 
