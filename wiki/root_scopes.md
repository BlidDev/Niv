# Overview:
Functions are an inseperable part of any language and Nest is not diffrent. Nest's alternative to functions is are called `root scopes`.

```Cpp

// Every program has to have a `MAIN` root scope
#MAIN |
   // Root scopes can be called by the `run` command
   run hello_there 
    
    // Or either be just called like a built in command
    set sum (add 1 2)

#MAIN

// Each roots scope is defined by a `#` followed by a name
// and a `|` character that marks the start of the arugument list
#hello_there |
    print "Hi! :D\n"
#hello_there
// ^^^ A curresponding lables is required in order to mark the end of the root scope 

#add | num1 num2
    // By default, a root scope returns `VOID` 
    // but it can be specifide otherwise using `return`
    return (cal $num1 + $num2)
#add
```

___
## **Arguments**

Root scopes aruguments, as any variable in `Nest` can be of any type. When passing values into aruguments, the program pushes new variables called the aruguments names with the curresponding values and releases them after the root scope has finished running. So calling the `add` root scope from the example would look like this behind the scenes:

```Cpp
#MAIN |
    set sum (add 1 2)
#MAIN

#add | num1 num2
    set num1 1
    set num2 2
    
    return (cal $num1 + $num2)

    release num1
    release num2
#add
```
*Important:* because of how aruguments are handled and how Nest only has one variable stack for the entirety of the program, root scopes can use/set/release variables outside of their scope, **meaning calling a variable the same name as one of the aruguments will cause it to be released after calling the root scope**. This will *(hopefully)* change in the future.


___
## **run**

**usage example:**
```Python
// template
run name Args...

// specific use
run add 1 2
```
    

**Args:**

* `name`: `STR`
* `Args`:  List of arguments of any value


**Desc:**

Runs the root scope named `name` and passes `Args` as the root scope's arguments

**Return value:** The return value of the root scope.


For more complex examples check [root_scopes.glg](../root_scopes.glg)

___
## **return**

**usage example:**
```Python
// template
return val

// specific use
set s "hello there"
return (llen s)
```
    

**Args:**

* `name`: `STR`
* `val` : Any value


**Desc:**

Exits the current root scope and sets its return value to `val`.

**Return value:** `RETURN`.


For more complex examples check [root_scopes.glg](../root_scopes.glg)
