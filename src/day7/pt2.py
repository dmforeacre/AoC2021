f = open('input.txt')

lines = f.readlines()

class Node:
    def __init__(self) -> None:
        self.name = None
        self.size = 0
        self.children = {}
        self.parent = None
        self.isDir = True


def calculateSizes(node):
    if not node.isDir:
        return node.size
    for _, c in node.children.items():
        node.size += calculateSizes(c)
    return node.size

def findDirectorySizes(smallest, node):
    if node.isDir:
        if node.size < smallest[0] and node.size > 30000000 - (70000000 - root.size):
            smallest[0] = node.size
        for _, c in node.children.items():
            findDirectorySizes(smallest, c)

def printFiles(depth, node):
    spaces = ""
    dir = ""
    for i in range(0, depth):
        spaces = spaces + " "
    if node.isDir:
        dir = "*"
    print(spaces ,node.name, " ", node.size, dir)
    for _, c in node.children.items():
        printFiles(depth + 1, c)

root = Node()
root.name = "/"
currNode = root
for l in lines:
    tokens = l.rstrip().split(" ")
    if tokens[0] == "$":
        if tokens[1] == "cd":
            if tokens[2] == "..":
                currNode = currNode.parent
            elif tokens[2] == "/":
                currNode = root
            else:
                currNode = currNode.children[tokens[2]]
    else:
        newNode = Node()
        newNode.name = tokens[1]
        newNode.parent = currNode
        if tokens[0] != "dir":
            newNode.isDir = False
            newNode.size = (int)(tokens[0])
        currNode.children[newNode.name] = newNode

calculateSizes(root)
total = [30000000]
findDirectorySizes(total, root)
#printFiles(0, root)
print(total[0])