# Lox language as a kernel module!

This is a kernel module that implements the Lox language from the book [Crafting Interpreters](https://craftinginterpreters.com/). Main features are done.

## How to use

Clone the repo. Then run `make` in src directory to build the module. Then do `insmod kmain_lox.ko` to load the module. To run a script do `cat <pathToScript> | sudo tee /dev/LoxIO` and then do `cat /dev/LoxIO` to see the output. To unload the module do `rmmod kmain_lox`.
