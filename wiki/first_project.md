# Overview:

Nest supports both single-script projects and multi-scipts ones, this chapter will walk you through how to create and manage both.


___
## Single script projects

Say we have this script we want to run:

```Python
// hello_world.nst
#MAIN |
    print "Hello, Nest!\n"
#MAIN
```

The simplest way to run it would be:
```Bash
cargo run -- -f path/to/hello_world.nst
or
./Nest -f path/to/hello_world.nst
```

The `-f` parameter is a short for `--file-list`, which be explained later.

This would run our script and print `Hello, Nest!` to the screen.


___
## Multi script projects

There are two ways to structure a mutlifile porject in Nest, the first being a simple `file-list` and the second being a `project-file`.

For this examplantion we will use two demo files:

```Python
// helper.nst

#helper_func | 
    print "Hello from helper_func!\n"
#helper_func
```
```Python
// main.nst

#MAIN | 
    helper_func
#MAIN
```

___
### File list

As shown before, the simplest way to run a multi script project is using a file list, the files get loaded into the buffer according to the order the list was entered. So the line:

```Bash
cargo run -- -f path/to/helper.nst path/to/main.nst
or
./Nest -f path/to/helper.nst path/to/main.nst
```

Would run the program as expected.

___
### Project file

While a `file-list` is the most straightforward way to acheive this goal it can get tidious to enter the entire list every time you want to run the project.

The solution to that would be a `project-file` like so:

```
# project.prj
helper.nst
main.nst
# or even another project file:
# ../libs/mathlib/mathlib.prj
```

That would be called like so:

```Bash
cargo run -- -p path/to/project.prj
./Nest -p path/to/project.prj
```

A project file does exactly what a `file-list` does only it's in a single neat file instead of having to expand on the "compiliation" line every time a new file is added.
