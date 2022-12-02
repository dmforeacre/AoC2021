with open("1_1_input.txt", "r") as file:
    lines = file.readlines()
    highTotal = [0,0,0]
    subTotal = 0
    for line in lines:
        if line != '\n':
            subTotal += int(line)
        else:
            if subTotal > highTotal[0]:
                highTotal[2] = highTotal[1]
                highTotal[1] = highTotal[0]
                highTotal[0] = subTotal
            elif subTotal > highTotal[1]:
                highTotal[2] = highTotal[1]
                highTotal[1] = subTotal
            elif subTotal > highTotal[2]:
                highTotal[2] = subTotal            
            subTotal = 0
    print(sum(highTotal))
