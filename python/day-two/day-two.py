#!/usr/bin/python

print "Welcome to the Day 2 Puzzle Solver" #Welcome Message

#Variables
filepath = './input'
strlines = []

with open(filepath) as fs: #Opens the File
   lines = fs.readlines()
   for line in lines:
      strlines.append(str(line))
fs.close() #Closes the file

alines = [('0' if x[2]!='-' else '') + x for x in strlines] #adds 0 to first number

blines = [x.rstrip("\n") for x in alines] #strips the \n

#Comment this line to toggle print output debug
print(blines)
