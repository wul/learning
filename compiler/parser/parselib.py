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

class Production:
    def __init__(self, str_exp, rank):
        self.rank = rank
        self.head = None
        self.bodies = None
        self.first_set = set()
        self.follow_set = set()
        self.production_str = str_exp.strip()

        self.parse_production_str(str_exp)
        
        
    def parse_production_str(self, s):
        left, bodies = s.split("->")
        left = left.strip()
        self.head = left

        # Turn bodies from strings to lists
        self.bodies = [body.strip().split() for body in bodies.strip().split("|")]

    def add_to_first_set(self, symbol, body):
        pass

    def add_to_follow_set(self, symbol, body):
        pass

    def __eq__(self, other):
        return self.rank == other.rank and self.head == other.head

    def __hash__(self):
        return self.production_str.__hash__()
    

# The format of CFG
# T   : set of terminals
# NT  : set of non-terminals
# P   : dict of non-terminals with key as non-terminal
#     : value is dictionary of production body and derivated terminals pair
# S   : start symbol
def normalize2(productions):

    T  = set()
    NT = set()
    P  = []
    S  = None
    
    idx = 0
    symbol_set = set(productions.replace("->", "").replace("|", " ").split())
    
    for line in productions.splitlines():
        idx += 1
        production = Production(line, idx)
        P.append(production) 
        if S is None:
            S = production.head
            
        NT.add(production.head)

    T = symbol_set - NT
    return CFG(T=T, NT=NT, S=S, P=P)
    

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

Item = namedtuple('Item', "head body dot")

class LR0():
    def __init__(self, cfg):
        self._ACTION = None
        self._GOTO = {}
        self.cfg = cfg

        # state is set of items
        self._state = {}
        state = self.closure(self.cfg.S)
        state.add(Item(head="S'", body=(cfg.S,), dot=0))
        self._state[0] = state
        self.state_index = 0
        #self.traversal_state()

    def traversal_state(self):
        pool = {self.state_index}
        processed = set()
        
        while True:
            new_states = set()
            for state_idx in pool:
                if state_idx in processed:
                    continue
                
                
                state = self._state[state_idx]
                for X in self.get_next_goto_X(state):
                    new_state = self.goto(state, X)
                    new_state_idx = self.get_state_idx(new_state)
                    
                    if new_state_idx is None:
                        self.state_index += 1
                        new_state_idx = self.state_index
                        self._state[new_state_idx] = new_state
                        new_states.add(new_state_idx)
                        
                    self._GOTO[state_idx, X] = new_state_idx
                    
                processed.add(state_idx)

                
            # all states are processed
            if not new_states:
                break
            else:
                pool |= new_states
        
    def get_state_idx(self, state):
        for idx, st in self._state.items():
            if st == state:
                return idx

        return None
        
    def get_next_goto_X(self, state):
        Xs = set()
        for item in state:
            if item.dot < len(item.body) and item.body[item.dot] != 'ε':
                Xs.add(item.body[item.dot])
        return Xs
    
    def goto(self, state, X):
        st = set()
        for item in state:
            if item.dot == len(item.body):
                continue
            if item.body[item.dot] == X:
                item = Item(head=item.head, body=item.body, dot=item.dot+1)
                st.add(item)

        derived_items = set()
        for X in self.get_next_goto_X(st):
            if is_non_terminal(X, self.cfg):
                derived_items |= self.closure(X)
                    
        return st | derived_items
        
    def __expr__(self):
        return str(self._state) + "\n" + str(self._GOTO)


    def closure(self, nt):
        items = set()
        bodies = self.cfg.P[nt]
        for body in bodies:
            items.add(Item(head=nt, body=body, dot=0))


        while True:
            new_items = set()
            for item in items:
                symbol = item.body[item.dot]
                if is_non_terminal(symbol, self.cfg):
                    bodies = self.cfg.P[symbol]
                    for body in bodies:
                        i = Item(head=symbol, body=body, dot=0)
                        if i not in items:
                            new_items.add(i)
            if not new_items:
                break
            else:
                items |= new_items

        return items

    def print(self):
        for idx, state in self._state.items():
            print("{}\t:".format(idx))
            for item in state:
                print("\t{}".format(item))
    
if __name__ == '__main__':
    productions = '''
    E -> E + T | T
    T -> T * F | F
    F -> ( E ) | id
    '''
    cfg = normalize(productions)
    pprint.pprint(cfg)

    '''
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
    '''
    parser = LR0(cfg)
    parser.traversal_state()
    parser.print()
