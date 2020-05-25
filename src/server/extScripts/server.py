""" 
exec.py executes external scripts, 
which are in the same directory.
It is run by the OS import module.
"""
# SSFTP EXTERNAL SCRIPTS EXECUTION BEGINS!

# imports (the modules)
import os

# classes + functions

# server class
#runs the gui application
class server:
    def windows(self):
        if input("have you run with administrator? (True/False): ") is True:
            print("Alright, continuing")
            os.system("py server/index.py") # run 
        if input("have you run with administrator?") is not True:
            print("Please give SSFTP admin. It needs it. >:(")
        else:
            print("smthing happened. raising exception!")
            raise Exception("Unknown admin choice.")
    def mac(self):
        if input("sudo'ed SSFTP (True/False)? ") is True:
            print("Beginning gui application. (the server)!")
            try:
                os.system("python server/index.py")
            except:
                try:
                    os.system("python3 server/index.py")
                except:
                    try:
                        os.system("py server/index.py")
                    except:
                        print("idk lol")
        if input("sudo'ed SSFTP (True/False)? ") is not True:
            print("give ssftp sudo. you greedy little-")
        else:
            print("smthing happened. raising exception!")
            raise Exception("Unknown sudo choice.")

# main function
def main():
    # change this to os of choice
    server.windows()

# if main clause
if __name__ == "__main__":
    main()

# EOF