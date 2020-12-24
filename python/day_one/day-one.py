#!/usr/bin/python

print "Welcome to the Day 1 Puzzle Solver" #Welcome Message

filepath = './input'

mylines = []

with open(filepath) as fs: #Opens the File
   lines = fs.readlines()
   for line in lines:
	mylines.append(int(line))
fs.close() #Closes the file

for line1 in mylines:
   for line2 in mylines:
      for line3 in mylines:
         if((line1+line2+line3)==2020):
            print(line1,line2,line3,(line1*line2*line3))
