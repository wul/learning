def KMP(string, needle):
    '''
    Find needle in string
    '''

    f = {}
    for x in range(len(needle), 0, -1):
        f[x] = failure_function2(needle, x)

    print("Calculate failure function for string {}".format(needle))
    pprint.pprint(f)

    length = len(string)
    needle_length = len(needle)
    idx = 0
    i = 0
    while idx < length:
        if string[idx] == needle[i]:
            idx += 1
            i += 1
            if i >= needle_length: #matched
                print("We found the need in string, end is {}".format(i))
                break

        else:
            if i == 0:
                move = 1
            else:
                move = i - f[i]

            idx -= i
            idx += move
            print("Failure at after char {}, move {}".format(i, move))
            
            i = 0


    print("No match")

KMP("ababaabaabac", "abaabac")
