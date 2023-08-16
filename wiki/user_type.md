# Overview:

UserType is Nest's equivilant of a struct; it basically acts as a `typedef struct` would in `C`. You can define a new UserType in your program like so:

```C
@NAME
    field_name : default_value
    ...
@NAME

// so an example UserType will look like something like this
@PERSON
  name : "Cred"  
  age  : 17
@PERSON

// fields are often just regular data type but can also 
// contain other UserTypes:

@WORKER
    person : ~*PERSON // the ~* operator will be explained later
    job    : "Protecting the king"
@WORKER

```
Fields are, as any variable in SGL, type-dynamic.




___
## **Default constructor/ `~*` operator**

**usage example:**
```Python
// template
set myobj ~*NAME

// specific use
set myobj ~*PERSON
```
    

**Args:**

* `NAME`: A string literal containing a registered UserType name, for example `~*PERSON`


**Desc:**

Takes a name of a UserType type and returns an object of that given type containing its default values defined in the UserType decleration (see above).

**Return value:** A UserType with default values.
___
## **make**

**usage example:**
```Python
// template
make NAME ARGS...

// specific use
make WORKER ~*PERSON "fighting the king"
```
    

**Args:**

* `NAME`: `STR` or `CHAR`
* `ARGS...`: List of arguments of any value


**Desc:**

Creates and returns a new UserType with custom values, given in order to the command (see specific example).

**Return value:** The constructed UserType object.

For more complex examples check [user_type.glg](../examples/user_type.glg)

___
## **setf**

**usage example:**
```Python
// template
setf name field_name val

// specific use
setf ojb1 job "Running away from the king because hes winning"
```
    

**Args:**

* `name`: `STR` or `CHAR`
* `field_name`: `STR` or `CHAR`
* `val`: Any value


**Desc:**

Sets the field `field_name` of UserType `name` to `val`.

**Return value:** The value of `val`.

For more complex examples check [user_type.glg](../examples/user_type.glg)

___
## **getf**

**usage example:**
```Python
// template
getf name field_name

// specific use
set reason_of_death [getf ojb1 job] 
```
    

**Args:**

* `name`: `STR` or `CHAR`
* `field_name`: `STR` or `CHAR`


**Desc:**

Retrieves the value of UserType object `name`'s field called `field_name`.

**Return value:** The value of the field.

For more complex examples check [user_type.glg](../examples/user_type.glg)
