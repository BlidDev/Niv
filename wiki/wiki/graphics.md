# Overview:
SGL comes with a built in simple pixel canvas API, this chapter will explain how to use it.

**The way it works:** The canvas has 2 pixel buffers, one for `display` and one for `drawing`. 
When you're editing your canvas you're actually editing the `drawing` buffer, then, when your frame is ready to be shown to the screen you can run `apply` in order to copy the `drawing` buffer into the `display` one, meaning your changes can actually be shown.

This is done in order to reduce SFML draw calls as much as possible to save on computing power and unwanted visual gitches.

___
## **init**

**usage example:**
```Python
// template
init w_w w_h c_w c_h title

// specific use
init 854 480 16 9 "An example window"
```
    

**Args:**

* `w_w`: `I32`
* `w_h`: `I32`
* `c_w`: `I32`
* `c_h`: `I32`
* `title`: `STR` or `CHAR`


**Desc:**

Initilizes SGL's graphics context and creates a new window `w_w`x`w_h` pixels big with a `c_w`x`c_h` size pixel canvas.


**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.




___
## **set_clear**

**usage example:**
```Python
// template
set_clear r g b

// specific use
set_clear 255 19 [cal 120 / 2]
```
    

**Args:**

* `r`: `I32` (0 - 255)
* `g`: `I32` (0 - 255)
* `b`: `I32` (0 - 255)


**Desc:**

Sets the canvas clear color to the given `r` `g` `b` values.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.

___
## **clear**

**usage example:**
```Python
clear
```
    

**Desc:**

Clears the canvas and sets it to the given `clear_color`.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


___
## **apply**

**usage example:**
```Python
apply
```
    

**Desc:**

Copies the data from the `drawing` pixel buffer into the `display` one.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


___
## **display**

**usage example:**
```Python
display
```
    

**Desc:**

Renders the `display` pixel buffer onto the screen, allowing the image to actually show.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.

___
## **set_pixel**

**usage example:**
```Python
// template
set_pixel x y r g b

// specific use
set_pixel 4 8 [cal 1 * 255] 150 200
```
    

**Args:**

* `x`: `I32`
* `y`: `I32`
* `r`: `I32` (0 - 255)
* `g`: `I32` (0 - 255)
* `b`: `I32` (0 - 255)


**Desc:**

Sets the pixel at [`x`, `y`] in the `drawing` pixel buffer to the given [`r`, `g`, `b`] values.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


___
## **set_area**

**usage example:**
```Python
// template
set_area x y w h r g b

// specific use
set_area 4 3 2 3 50 50 50
```
    

**Args:**

* `x`: `I32`
* `y`: `I32`
* `w`: `I32`
* `h`: `I32`
* `r`: `I32` (0 - 255)
* `g`: `I32` (0 - 255)
* `b`: `I32` (0 - 255)


**Desc:**

Sets a `w` by `h` area at [`x`, `y`] in the `drawing` pixel buffer to the given [`r`, `g`, `b`] values.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.

___
## **get_pixel**

**usage example:**
```Python
// template
get_pixel x y r g b

// specific use
set r  ""
set g  ""
set b  ""
get_pixel 1 1 r g b
```
    

**Args:**

* `x`: `I32`
* `y`: `I32`
* `r`: `STR` or `CHAR`
* `g`: `STR` or `CHAR`
* `b`: `STR` or `CHAR`


**Desc:**

Opposite of `set_pixel`, retrieves the `rgb` values of a pixel at [`x`, `y`] in the `drawing` pixel buffer and stores it inside the given variable names.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


___
## **get_area**

**usage example:**
```Python
// template
get_area x y w h

// specific use
set lst [get_area 1 1 3 3]
```
    


**Desc:**

Opposite of `set_area`, retrieves the `rgb` values of a `w`x`h` area at [`x`, `y`] in the `drawing` pixel buffer and returns it as a `LIST` containing all of the values from the given area in `RGBA8888` pixel format.

**Return value:** List of `I32`s.

For more complex examples check one of the graphical examples in the `examples` folder.

___
## **get_millis**

**usage example:**
```Python
get_millis
```
    

**Desc:**

Returns the amount of milliseconds that passed since the initilization of the graphical context.

**Return value:** `I32`.

For more complex examples check one of the graphical examples in the `examples` folder.


___
## **load_font**

**usage example:**
```Python
// template
load_font path

// specific use
load_font "path/to/font.ttf"
```
    

**Desc:**

Loads a font from a file to be used by all text objects in the program. 

**Return value:** `I32`.

For more complex examples check one of the graphical examples in the `examples` folder.

___
## **new_text**

**usage example:**
```Python
// template
new_text msg scale

// specific use
set text1 [new_text [format "hello {}" "world!"] 45]
```


**Args:**

* `msg`: `STR`
* `scale`: `I32`

    

**Desc:**

Creates a new text object with `msg` as its body with size `scale` (in pixels) at (0, 0) and returns its ID to be saved.

**Return value:** `I32`.

For more complex examples check one of the graphical examples in the `examples` folder.
            

___
## **set_ttext**

**usage example:**
```Python
// template
set_ttext id msg

// specific use
set_ttext $text1 "Different message\n"
```


**Args:**

* `id`: `I32`
* `msg`: `STR`


**Desc:**

Sets the text of text object with id: `id` to `msg`.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


___
## **set_tsize**

**usage example:**
```Python
// template
set_tsize id size

// specific use
set_tsize $text1 [cal 10 * 3]
```


**Args:**

* `id`: `I32`
* `size`: `I32`


**Desc:**

Sets the size of text object with id: `id` to `size` (in pixels).

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


___
## **set_tpos**

**usage example:**
```Python
// template
set_tpos id x y

// specific use
set_tpos $text1 [cal 50 / 5] 100
```


**Args:**

* `id`: `I32`
* `x`: `I32`
* `y`: `I32`


**Desc:**

Sets the position of text object with id: `id` to (`x`, `y`) (in pixels).

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


___
## **set_tclr**

**usage example:**
```Python
// template
set_tclr id r g b

// specific use
set_tclr $text1 100 100 50
```


**Args:**

* `id`: `I32`
* `r`: `I32`
* `g`: `I32`
* `b`: `I32`


**Desc:**

Sets the color of text object with id: `id` to (`r`, `g`, `b`).

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


___
## **set_tvisible**

**usage example:**
```c
// template
set_tvisible id flag

// specific use
set_tvisible $text1 false
```


**Args:**

* `id`: `I32`
* `flag`: `BOOL`


**Desc:**

Sets whether text object with id: `id` should be visible.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


___
## **end_text**

**usage example:**
```c
// template
end_text id 

// specific use
end_text text1 // notice no $
```


**Args:**

* `id`: `STR` or `CHAR`


**Desc:**

Delets the text object with id: `id` and sets the variable containing the id to `VOID`

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.

