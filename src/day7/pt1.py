f = open('input.txt')

lines = f.readlines()

class Node:
    def __init__(self) -> None:
        self.name = None
        self.size = 0
        self.children = {}
        self.parent = None

root = Node()
root.name = "/"
currNode = root;
for l in lines:
    tokens = l.split(" ")
    if tokens[0] == "$":
        if tokens[1] == "cd":
            if tokens[2] == "..":
                currNode = currNode.parent
            elif tokens[2] == "/":
                currNode = root
