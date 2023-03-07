# MCFuncLang
## Overview

MCFuncLang (MCFL for short) is a programming language that compiles to Minecraft commands, outputting a datapack with `.mcfunction` files.

Compatible with Minecraft Java Edition 1.13.2.

## Language Features

### Variables

MCFL integers are stored by dummy players on scoreboard objectives. Most variables use a dummy objective, but variables can also be associated with a scoreboard criterion. In this case, they can sometimes be read-only, depending on the criterion.

MCFL floats are stored as two integers, a significand/mantissa and an exponent.

Booleans are stored as integers, where values of 0 are false and all others are true.

MCFL has the following basic data types:

* signed ints
* signed floats
* strings*
* booleans

Strings are special because they must be statically compiled.

Variables in MCFL are statically typed and have function-level scope.

### Functions

MCFL is a functional programming language and has no classes or objects. All code must be contained in a function.

In general, there are two types of functions: MCfunctions and static functions. MCfunctions are compiled to their own `.mcfunction` files, meaning they can be called on their own in-game. Static functions are not and cannot be called directly outside the MCFL program.

The `tick()` and `startup()` functions are special. A program must define at least one of these to have any effect outside of manually calling functions from in-game. Both are MCfunctions that take no arguments. The `tick()` function is called every game tick, while the `startup()` function is called once when the datapack is loaded.

## Syntax

Comments can be written with the `//` or `/* ... */` syntax.

Types are specified with the keywords `int`, `float`, `string`, and `bool`. Additionally, `void` can be used in function declarations in place of a type keyword to indicate that the function has no return value.

Variables are declared and initialized like the following:

```
int a = 7;
float b = 5.2;
string c = "hello";
bool d = false;
```

Variables can also be declared without being initialized, in which case they take on their type's default value:

```
int a; // 0
float b; // 0.0
string c; // ""
bool d; // false
```

Functions are defined as follows:

```
[function|mcfunction] name(<type1> <arg1>,...) -> <return type> {
  // Body
}
```
