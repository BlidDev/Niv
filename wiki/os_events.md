# Overview:

Aside from traditional `std::cin` like user input shown in [input_and_output](input_and_output.md), SGL offers a way to check immidate keyboard key presses.

___
## **handle_input**

**usage example:**
```Python
[handle_input]
```
    

**Desc:**

_When graphics are initilized:_ Refreshes the graphics window (i.e. make it respond to OS GUI input like the X button); Refreshes the pressed-keys buffer.

_When graphics are not initilized:_ Only refreshes the key-pressed buffer;

**Return value:**  Retruns `true` if the graphics window should close, otherwise (and when graphics aren't initilized) `false`.


For more complex examples check one of the graphical examples in the `examples` folder.


___
## **key_pressed**

**usage example:**
```Python
// template
[key_pressed][key]

// specific use
[singleif][[key_pressed][Delete]][=][true][[print]["Delete\n"]]
```
    
**Args:**

* `key`: `STR` or `CHAR` 

**Desc:**

Checks if the spicified `key` is pressed, the key name has got to be listed in the [following page](https://docs.rs/device_query/0.2.8/device_query/keymap/enum.Keycode.html). Returns `true` if it's pressed.

**Return value:**  `BOOL`.


For more complex examples check one of the graphical examples in the `examples` folder.




