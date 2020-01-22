import os

while True:
    s = input("What file to compile? ->")
    t = input("What file to which you should save it? ->")
    u = input("Where is the glslc file stored? ->").strip()

    os.system(u + "./glslc " + s + " -o " + t)

    q = input("Quit? (y / n) ->")
    if q.lower() == "y":
        break
