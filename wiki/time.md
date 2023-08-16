# Overview:

While there is technically a way to measure time using `get_millis`, using this method is limiting and hard to work with. So to fill this gap, Nest includes a built-in timer functionality that can help you measure time in your program, completely independently from graphics.

___
## **new_timer**

**usage example:**
```Python
// template
new_timer delay

// specific use
set timer1 [new_timer 500]
```


**Args:**

* `delay`: `I32`

    

**Desc:**

Creates a new timer object with `delay` as its iternal delay in milliseconds, and returns its ID to be saved.

**Return value:** `I32`.

For more complex examples check one of the graphical examples in the `examples` folder.

___
## **timer_elapsed**

**usage example:**
```Python
// template
timer_elapsed id

// specific use
singleif [timer_elapsed $timer1] = true (print "passed\n")
```


**Args:**

* `id`: `I32`

    

**Desc:**

Returns whether the time elapsed, in milliseconds, since timer object with id `id` last reset is equal to or greater than its iternal `delay`.

**Return value:** `BOOL`.

For more complex examples check one of the graphical examples in the `examples` folder.

___
## **timer_millis**

**usage example:**
```Python
// template
timer_millis id

// specific use
print "Time since reset: {}\n" (timer_millis $timer1)
```


**Args:**

* `id`: `I32`

    

**Desc:**

Returns the amount of time elapsed, in milliseconds, since timer object with id `id` last reset, regardless of `delay`.

**Return value:** `BOOL`.

For more complex examples check one of the graphical examples in the `examples` folder.

___
## **timer_reset**

**usage example:**
```Python
// template
timer_reset id

// specific use
timer_reset $timer1
```


**Args:**

* `id`: `I32`

    

**Desc:**

Sets the elapsed time of timer object with id `id` to 0.
**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.

___
## **timer_set_del**

**usage example:**
```Python
// template
timer_set_delay id delay

// specific use
timer_set_delay $timer1 40
```


**Args:**

* `id`: `I32`
* `delay`: `I32`

    

**Desc:**

Sets the iternal delay of timer object with id `id` to `delay` in milliseconds.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.

## **end_timer**

**usage example:**
```Python
// template
end_timer id

// specific use
end_timer timer1 // notice no $
// or
end_timer 0 
```


**Args:**

* `id`: `STR` / `I32`

    

**Desc:**

Deletes timer object with id `id`, **when given a variable name** sets the holding storing variable to `VOID`.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.
