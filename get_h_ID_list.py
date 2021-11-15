def get_h_ID_list():
    while True:
        print("Enter the holy numbers: ")
        h_ID_list=input().split()   #user input seperated at whitespace
        
        for i in range(len(h_ID_list)):   #convert all ID to int
            try:
                h_ID_list[i]=int(h_ID_list[i], base=10)
            except ValueError:  #if input invalid: discard whole user input, try again
                print("Error: Could not convert input \""+h_ID_list[i]+"\" to int.")
                break
        else:   #if all ID converted without failure: break out of while, return desired ID
            break

    return h_ID_list