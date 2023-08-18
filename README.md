# Nest
Nest is a fairly simple interpreted langauge written in Rust, being the unofficial spritiual succesor of [Glang](https://github.com/BlidDev/Glang).

The main goal behind this project was to make a langauge capeable of nested expressions (like `printf("%d\n", 1 + 1)` for example).

## Wiki
Nest holds a detailed wiki about almost every aspect of language, check these two pages out to get started:
* [Installation](wiki/installation.md)
* [Setting up and running your first project](wiki/first_project.md)

## Code example:
Here is an example code sinppet that prints your info using sturcts:

```Python

@Person
 name : ""
 age : 0
@Person

#MAIN |
    set name (input "Enter your name: ")
    set age (inputcast i32 "Enter your age: ")
    set obj (make Person $name $age)
    print "Your info is: {}\n" $obj
#MAIN
```

### Libraries used for this project
* `unescape`
* `snailquote`
* `unstringify` 
* `sfml-rs`
* `device_query`
* `clap`

___
Thanks for checking out Nest! :D
