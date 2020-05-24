""" 
exec.py executes external scripts, 
which are in the same directory.
It is run by the OS import module.
"""
# SSFTP EXTERNAL SCRIPTS EXECUTION BEGINS!

# imports (the modules)
import os

# functions
def windows():
    if input("have you run with administrator?") is True:
        print("Alright, continuing")
        os.system()

# main function
def main():
    print()

# if main clause
if __name__ == "__main__":
    main()

# EOF