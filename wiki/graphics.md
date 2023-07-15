# Overview:
SGL comes with a built in simple pixel canvas API, this chapter will explain how to use it.

**The way it works:** The canvas has 2 pixel buffers, one for `display` and one for `drawing`. 
When you're editing your canvas you're actually editing the `drawing` buffer, then, when your frame is ready to be shown to the screen you can run `apply` in order to copy the `drawing` buffer into the `display` one, meaning your changes will actually be shown.

This is done in order to reduce draw calls as much as possible to save on computing power and unwanted visual gitches.

## **init**

**usage example:**
```Python
// template
[init][w_w][w_h][c_w][c_h][title]

// specific use
[init][848][480][16][9]["An example window"]
```
    

**Args:**

* `w_w`: `I32`
* `w_h`: `I32`
* `c_w`: `I32`
* `c_h`: `I32`
* `title`: `STR` or `CHAR`


**Desc:**

Initilizes SGL's graphics context and creates a new window `w_w`x`w_h` pixels long with a `c_w`x`c_h` size pixel canvas.


**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.




## **set_clear**

**usage example:**
```Python
// template
[set_clear][r][g][b]

// specific use
[set_clear][255][19][[cal][120][/][2]]
```
    

**Args:**

* `r`: `I32` (0 - 255)
* `g`: `I32` (0 - 255)
* `b`: `I32` (0 - 255)


**Desc:**

Sets the canvas clear color to the given `r` `g` `b` values.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.

## **clear**

**usage example:**
```Python
[clear]
```
    

**Desc:**

Clears the canvas and sets it to the given `clear_color`.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


## **apply**

**usage example:**
```Python
[apply]
```
    

**Desc:**

Copies the data from the `drawing` pixel buffer into the `display` one.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.


## **display**

**usage example:**
```Python
[display]
```
    

**Desc:**

Renders the `display` pixel buffer onto the screen, allowing the image to actually show.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.

## **set_pixel**

**usage example:**
```Python
// template
[set_pixel][x][y][r][g][b]

// specific use
[set_pixel] [4][8] [[cal][1][*][255]][150][200]
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


## **set_area**

**usage example:**
```Python
// template
[set_area][x][y][w][h][r][g][b]

// specific use
[set_area] [4][3] [2][3] [50][50][50]
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


## **get_pixel**

**usage example:**
```Python
// template
[get_pixel][x][y][r][g][b]

// specific use
[set][r][]
[set][g][]
[set][b][]
[get_pixel] [1][1] [r][g][b]
```
    

**Args:**

* `x`: `I32`
* `y`: `I32`
* `r`: `STR`
* `g`: `STR`
* `b`: `STR`


**Desc:**

Opposite of `set_pixel`, retrieves the `rgb` values of a pixel at [`x`, `y`] in the `drawing` pixel buffer and stores it inside the given variable names.

**Return value:** `VOID`.

For more complex examples check one of the graphical examples in the `examples` folder.
