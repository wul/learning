from collections import namedtuple
import pprint
productions = '''                       
E  -> T E'      
E' -> + T E' | ε
T  -> F T'
T' -> * F T' | ε
F  -> ( E ) | id
'''

CFG = namedtuple('CFG', "NT T P S")

# The format of CFG
# T   : set of terminals
# NT  : set of non-terminals
# P   : dict of non-terminals with key as non-terminal
#     : value is dictionary of production body and derivated terminals pair
# S   : start symbol

def normalize(productions):

    T  = set()
    NT = set()
    P  = {}
    S  = None
    
    prod = {}
    for line in productions.splitlines():
        line = line.strip()
        if line:
            left, bodies = line.split("->")
            left = left.strip()
            if S is None:
                S = left

            # Turn bodies from strings to lists
            bodies = [body.strip().split() for body in bodies.strip().split("|")]
            for body in bodies:
                if left not in prod:
                    prod[left] = {}

                #the set will save all derivated terminals later
                prod[left][tuple(body)] = set()

            for body in bodies:
                for symbol in body:
                    if symbol.isupper():
                        NT.add(symbol)
                    else:
                        T.add(symbol)
            
    return CFG(T=T, NT=NT, S=S, P=prod)



def FIRST(nt, cfg):
    productions = cfg.P
    chr_set = set()
    bodies_for_nt = productions[nt]
    for body, derivated_terminals in bodies_for_nt.items():
        for symbol in body:
            if is_non_terminal(symbol, cfg):
                #It's a non-terminal, we need calculate the first
                #set of it and check if epsilon in it. If epsilon is in it,
                #we need check consequent symbol too
                chrs = FIRST(symbol, cfg)
                chr_set |= chrs
                derivated_terminals |= chrs
                if 'ε' not in chrs:
                    break
            else:
                #it is a terminal symbol, we just add it and break loop
                chr_set.add(symbol)
                derivated_terminals.add(symbol)
                #skip out the loop
                break
                
    return chr_set

def is_terminal(symbol, cfg):
    return symbol not in cfg.P.keys()

def is_non_terminal(symbol, cfg):
    return symbol in cfg.P.keys()

def is_start_symbol(symbol, cfg):
    return symbol == cfg.S


def FOLLOW(nt, cfg):
    chr_set = {'$'}
    productions = cfg.P


    rels  = {}
    cache = {}
    # Get direct follows for each non-terminals
    # and deduct relations between FOLLOW set of each non-terminals
    
    for left, bodies in productions.items():
        for right in bodies:
            check_follow = False
            last_nt      = None
            for symbol in right:
                if is_non_terminal(symbol, cfg):
                    if check_follow:
                        symbols = FIRST(symbol, cfg) - {'ε'}
                        try:
                            cache[last_nt] |= symbols
                        except KeyError:
                            cache[last_nt] = symbols

                    check_follow = True
                    last_nt = symbol
                else:
                    # It's terminal
                    if check_follow:
                        symbols = {symbol,}
                        try:
                            cache[last_nt] |= symbols
                        except KeyError:
                            cache[last_nt] = symbols

                    check_follow = False

            # Deduct the relations of FOLLOW sets
            for symbol in reversed(right):
                if is_terminal(symbol, cfg):
                    break
                else:
                    #non-terminal
                    if left != symbol:
                        # ignore self contains
                        try:
                            rels[symbol].add(left)
                        except KeyError:
                            rels[symbol] = {left,}
                    
                        
                    if 'ε' not in FIRST(symbol, cfg):
                        break

    try:
        cache[cfg.S].add('$')
    except KeyError:
        cache[cfg.S] = {'$'}
    


    # Calculate all subset of FOLLOW(nt)
    def lookup_relations(nt, rels, st):
        lst = rels.get(nt, set())
        for x in lst:
            if x not in st:
                st.add(x)
                lookup_relations(x, rels, st)


    
    chr_set |= cache.get(nt, set())

    subsets = set()
    lookup_relations(nt, rels, subsets)
    
    for symbol in subsets:
        chr_set |= cache.get(symbol, set())
            

    return chr_set        
                                    

def LL1(cfg):
    # FIRST
    productions = cfg.P
    table = {}
    for nt in cfg.NT:
        first_set = FIRST(nt, cfg)
        for symbol in first_set:
            for body, derivated_terminals in productions[nt].items():
                if symbol == 'ε':
                    for x in FOLLOW(nt, cfg):
                        table[(nt, x)] = ['ε']
                        
                else:
                    if symbol in derivated_terminals:
                        table[(nt, symbol)] = body


    return table

def print_table(table):
    pprint.pprint(table)
    rows = set()
    cols = set()
    for row, col in table:
        rows.add(row)
        cols.add(col)
    
    print("Constuction table:")
    print("_____________________________________________________")
    print("{:8s}".format(""), end="")    
    for col in cols:
        print("{:<15s}".format(col), end="")
    print()

    for row in rows:
        print("{:8s}".format(row), end='')
        for col in cols:
            try:
                body = table[(row,col)]
                s = row + " -> " + " ".join(body)
            except:
                s = ""
            
            print("{:<15s}".format(s), end="")
        print("")

if __name__ == '__main__':
    cfg = normalize(productions)
    pprint.pprint(cfg)


    print("Calculate FIRSTs")
    for nt in ['E', "E'", "T", "T'", 'F']:
        print("FIRST({})\t:\t{}".format(nt, str(FIRST(nt, cfg))))

    print("Print CFG again:")
    pprint.pprint(cfg)

    print("Calculate FOLLOWs")        
    for nt in ['E', "E'", "T", "T'", 'F']:
        print("FOLLOW({})\t:\t{}".format(nt, FOLLOW(nt, cfg)))

    r = LL1(cfg)
    print_table(r)
        
