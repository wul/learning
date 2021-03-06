from collections import namedtuple
import graphviz
import pprint
productions = '''                       
E  -> T E'      
E' -> + T E' | ε
T  -> F T'
T' -> * F T' | ε
F  -> ( E ) | id
'''


#Item = namedtuple('Item', "head body dot lookahead", defaults=('', tuple(), 0, ''))
class Stack:

    def __init__(self):
        self.lst = []
        
    def push(self, o):
        self.lst.append(o)

    def pop(self):
        return self.lst.pop()

    def gettop(self):
        return self.lst[-1]

    def repr_right(self):
        return "| {}".format(" ".join([str(x) for x in self.lst]))

    def repr_left(self):
        return "{} |".format(" ".join([str(x) for x in reversed(self.lst)]))
    
class Item:
    def __init__(self, head, body, dot=0, lookahead = ''):
        self.head = head
        self.body = body
        self.dot  = dot
        self.lookahead = ''
        
    def __eq__(self, other):
        return self.head == other.head and self.body == other.body and self.dot == other.dot and self.lookahead == other.lookahead

    def __hash__(self):
        return (self.head + str(self.body)).__hash__()


    def __str__(self):
        lst = [x for x in self.body]
        lst.insert(self.dot, '.')
        return "{} -> {}, {}".format(self.head, " ".join(lst), self.lookahead)

    def __expr__(self):
        lst = [x for x in self.body]
        lst.insert(self.dot, '.')
        return "{} -> {}, {}".format(self.head, " ".join(lst), self.lookahead)
    
    
class Production:
    def __init__(self, str_exp, rank):
        self.rank = rank
        self.head = None
        self.bodies = None
        
        self.production_str = str_exp.strip()
        self.parse_production_str(str_exp)
        
    def deconstruct(self):
        return [Item(head=self.head, body=tuple(body), dot=0) for body in self.bodies]

        
    def parse_production_str(self, s):
        left, bodies = s.split("->")
        left = left.strip()
        self.head = left

        # Turn bodies from strings to lists
        self.bodies = tuple([body.strip().split() for body in bodies.strip().split("|")])


    def __eq__(self, other):
        return self.rank == other.rank and self.head == other.head

    def __hash__(self):
        return self.production_str.__hash__()

EPSILON = 'ε'
ENDMARKER = '$'

class CFG():

    def __init__(self, productions_str):
        
        # The format of CFG
        # T   : set of terminals
        # NT  : set of non-terminals
        # P   : dict of non-terminals with key as non-terminal
        #     : value is dictionary of production body and derivated terminals pair
        # S   : start symbol
        self.T  = set()
        self.NT = set()
        self.P  = []
        self.S  = None

        # helper varaibles
        # for quick locating production according to production head
        # key:value is NT: index
        self.loc = {}
        self._FIRST = {}
        self._FOLLOW = {}

        self.normalize(productions_str)

        
    def normalize(self, productions):
        idx = 0
        symbol_set = set(productions.replace("->", "").replace("|", " ").split())
    
        for line in productions.splitlines():
            line = line.strip()
            if not line:
                continue

            production = Production(line, idx)
            self.P.append(production)
            
            if self.S is None:
                self.S = production.head
            
            self.NT.add(production.head)
            self.loc[production.head] = idx

            idx += 1

        self.T = symbol_set - self.NT

        # Have to initialize follow/first seperately, because calculation of follow need first set 
        for nt in self.NT:
            self.calculate_first_set(nt)

        for nt in self.NT:
            self.calculate_follow_set(nt)

    def get_production(self, nt):
        try:
            return self.P[self.loc[nt]]
        except KeyError:
            return None

    def __iter__(self):
        return iter(tuple(self.P))

    def is_non_terminal(self, symbol):
        return symbol in self.NT

    def is_terminal(self, symbol):
        return symbol not in self.NT


    
    def add_to_first_set(self, nt, t, body):
        d = self._FIRST.get(nt, {})
        d[t] = body
        self._FIRST[nt] = d

    def calculate_first_set(self, nt):
        idx = self.loc[nt]
        production = self.P[idx]

        
        path = []
        def walk(nt, path):
            print( "walk {} against {}".format(nt, path));
            if nt in path:
                #loop, ignore
                return set()

            index = self.loc[nt]
            production = self.P[index]
            symbol_set = set()
            for body in production.bodies:
                for symbol in body:
                    if self.is_non_terminal(symbol):
                        new_idx = self.loc[symbol]

                        path.append(nt)
                        symbols = walk(symbol, path)
                        for char in symbols:
                            self.add_to_first_set(nt, char, body)

                            
                        symbol_set |= symbols
                        if not EPSILON in symbols:
                            break
                    else:
                        self.add_to_first_set(nt, symbol, body)
                        symbol_set.add(symbol)
                        break


            return symbol_set
        

        walk(nt, path)


    def calculate_follow_set(self, nt):
        symbol_set = {'$'}
        productions = self.P


        rels  = {}
        cache = {}
        
        # Get direct follows for each non-terminals
        # and deduct relations between FOLLOW set of each non-terminals

        for production in self.P:
            left = production.head
            bodies = production.bodies

            for right in bodies:
                check_follow = False
                last_nt      = None
                for symbol in right:
                    if self.is_non_terminal(symbol):

                        if check_follow:
                            symbols = self.FIRST(symbol) - {EPSILON}
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
                    if self.is_terminal(symbol):
                        break
                    else:
                        #non-terminal
                        if left != symbol:
                            # ignore self contains
                            try:
                                rels[symbol].add(left)
                            except KeyError:
                                rels[symbol] = {left,}
                    
                        
                        if EPSILON not in self.FIRST(symbol):
                            break

                
        try:
            cache[self.S].add('$')
        except KeyError:
            cache[self.S] = {'$'}
    


        # Calculate all subset of FOLLOW(nt)
        def lookup_relations(nt, rels, st):
            lst = rels.get(nt, set())
            for x in lst:
                if x not in st:
                    st.add(x)
                    lookup_relations(x, rels, st)


    
        symbol_set |= cache.get(nt, set())

        subsets = set()
        lookup_relations(nt, rels, subsets)
    
        for symbol in subsets:
            symbol_set |= cache.get(symbol, set())
            

        self._FOLLOW[nt] = symbol_set
        
    
    def get_first_set_map(self, nt):
        return self._FIRST[nt]

    def get_first_set_of_string(self, s):
        r = set()
        for symbol in s:
            symbols = set()
            if self.is_terminal(symbol):
                symbols.add(symbol)
            else:
                symbols = self.FIRST(symbol)

            r |= symbols
            if not symbols.contains(EPSILON):
                break
        return r

    def FIRST(self, s):
        ret = set()
        for symbol in s.split():
            symbols = set()
            if self.is_terminal(symbol):
                symbols.add(symbol)
            else:
                symbols = set(self._FIRST[symbol].keys())

            ret |= symbols
            if EPSILON not in symbols:
                break
        return ret

    def FOLLOW(self, nt):
        return self._FOLLOW[nt]

                                    

class LL1():
    # FIRST
    def __init__(self, cfg):
        self.cfg = cfg
        productions = cfg.P
        self.table = {}
        for nt in cfg.NT:
            first_set = cfg.FIRST(nt)
            first_set_map = cfg.get_first_set_map(nt)
            for symbol, body in first_set_map.items():
                if symbol == EPSILON:
                    for x in cfg.FOLLOW(nt):
                        self.table[(nt, x)] = [EPSILON]
                        
                else:
                    if symbol in first_set:
                        self.table[(nt, symbol)] = body

    def print_state(self, stack, s, production=''):
        print("  {:>20}\t\t{:<40}\t{} ".format(stack.repr_left(), str(s), production))

        
    def parse(self, s):
        print("\nParsing string: {}".format(s))
        print("-"*80)
        stack = Stack()
        stack.push(ENDMARKER)
        stack.push(self.cfg.S)

        parsed = False
        tokens = s.split()
        tokens.append(ENDMARKER)
        for idx in range(len(tokens)):
            token = tokens[idx]
            self.print_state(stack, tokens[idx:])
            
            while True:
                
                top = stack.gettop()
                if cfg.is_terminal(top) and top == token:
                    stack.pop()
                    break


                try:
                    body = self.table[(top, token)]
                except KeyError:
                    print("Stack TOP {} meet token {} failed".format(top, token))
                    return

                symbol = stack.pop()
                if body != [EPSILON]:
                    for symbol in reversed(body):
                        stack.push(symbol)
                self.print_state(stack, tokens[idx:], "{} -> {}".format(top, " ".join(body)))
        else:
            parsed = True

        self.print_state(stack, tokens[idx+1:])

        if parsed:
            print("All string parsed")
            
            
    def print(self):
        table = self.table
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


class LR0():
    def __init__(self, cfg):
        self._ACTION = {}
        self._GOTO = {}
        self.cfg = cfg

        # state is set of items
        self._state = {}
        #state = self.closure(self.cfg.S)

        state = self.closure2(self.cfg.S)

        state.insert(0, (Item(head="S'", body=(cfg.S,), dot=0)))
        self._state[0] = state
        self.state_index = 0
        self.traversal_state()

    def traversal_state(self):
        pool = [self.state_index]
        processed = set()
        
        while True:
            new_states = []
            for state_idx in pool:
                if state_idx in processed:
                    continue
                
                
                state = self._state[state_idx]
                for item in state:
                    if len(item.body) == item.dot:  #already parsed to the end of the production
                        if item.head == "S'" and item.body[-1] == self.cfg.S:
                            self._ACTION[state_idx, ENDMARKER] = ('Accept',)
                            continue
                        
                        for t in self.cfg.FOLLOW(item.head):
                            self._ACTION[state_idx, t] = ('r', item)
                        
                for X in self.get_next_goto_X(state):
                    new_state = self.goto(state, X)
                    new_state_idx = self.get_state_idx(new_state)
                    
                    if new_state_idx is None:
                        self.state_index += 1
                        new_state_idx = self.state_index
                        self._state[new_state_idx] = new_state
                        new_states.append(new_state_idx)

                    if self.cfg.is_terminal(X):
                        self._ACTION[state_idx, X] = ('s', new_state_idx)
                    else:
                        self._GOTO[state_idx, X] = new_state_idx
                    
                processed.add(state_idx)

                
            # all states are processed
            if not new_states:
                break
            else:
                pool.extend(new_states)
        
    def get_state_idx(self, state):
        for idx, st in self._state.items():
            if st == state:
                return idx

        return None
        
    def get_next_goto_X(self, state):
        Xs = []
        for item in state:
            if item.dot < len(item.body) and item.body[item.dot] != EPSILON:
                t = item.body[item.dot]
                if t not in Xs:
                    Xs.append(t)
        return Xs
    
    def goto(self, state, X):
        st = []
        for item in state:
            if item.dot == len(item.body):
                continue
            if item.body[item.dot] == X:
                i = Item(head=item.head, body=item.body, dot=item.dot+1)
                st.append(i)

        return self.closure(st)
    
    def __expr__(self):
        return str(self._state) + "\n" + str(self._GOTO)


    def closure(self, items):
        # Items represents item set
        while True:
            new_items = []
            for item in items:
                if item.dot >= len(item.body):
                    continue
                
                symbol = item.body[item.dot]
                if self.cfg.is_non_terminal(symbol):
                    production = self.cfg.get_production(symbol)
                    item_lst = production.deconstruct()

                    for i in item_lst:
                        if i not in items:
                            new_items.append(i)

            if not new_items:
                break
            else:
                items.extend(new_items)

        return items
    
    def closure2(self, nt):
        
        production = self.cfg.get_production(nt)
        items = production.deconstruct()
        return self.closure(items)

        
    def print(self):
        '''
        for idx, state in self._state.items():
            print("{}\t:".format(idx))
            for item in state:
                print("\t{}".format(item))
        '''
        print("\n Print state table")
        for state_idx, items in self._state.items():
            print(state_idx)
            for item in items:
                print("\t{}".format(item))

        print("\n Print GOTO table")
        pprint.pprint(self._GOTO)
        print("\n Print ACTION table")
        pprint.pprint(self._ACTION)
        
    def view(self):
        dot = graphviz.Digraph(name="SLR", node_attr={"shape": "plaintext"}, format="png")
        dot.attr(size='18, 15', rankdir='LR')
        for idx, state in self._state.items():
            name = "I{0}".format(idx)

            content = ''
            for item in state:
                body = [x for x in item.body]
                body.insert(item.dot, '.')
                    
                content += '<tr><td align="left">{} -&gt; {}</td></tr>'.format(item.head, ' '.join(body))

            content = '<tr><td style="font-weight: bold">{}</td></tr>'.format(name) + content
            label = '''<
            <table border="1">
            {}
            </table>
            >'''.format(content)
            print(label)
            dot.node(name, label)
            
        for key, value in self._GOTO.items():
            src = "I{}".format(key[0])
            dst = "I{}".format(value)
            dot.edge(src, dst, label=key[1])

        for key, value in self._ACTION.items():
            src = "I{}".format(key[0])
            if value[0] == 's': #shift
                dst = "I{}".format(value[1])
                dot.edge(src, dst, label=key[1])

        dot.view()

    def GOTO(self, state, X):
        try:
            return self._GOTO[state, X]
        except KeyError:
            return None
        
    def ACTION(self, state, t):
        try:
            return self._ACTION[state, t]
        except KeyError:
            return None

    def get_item_of_reduce(self, state_idx):
        items = self._state[state_idx]
        for item in items:
            if len(item.body) == item.dot:
                return item
        return None

    def print_state(self, stack, stack2,  production=''):
        print("  {:<20}\t\t{:<40}\t{} ".format(stack.repr_right(), stack2.repr_right(), production))

    def parse(self, s):
        print("\nParse string: {}".format(s))
        
        s = s.split()
        
        stack = Stack()
        stack_symbol = Stack()
        
        cache = []
        # Push start state
        state = 0
        stack.push(state)
        stack_symbol.push(ENDMARKER)

        idx = 0
        s.append(ENDMARKER)
        t = s[idx]
        while True:
            state = stack.gettop()
            res = self.ACTION(state, t)
            self.print_state(stack, stack_symbol, "")            
            if res == None:
                raise Exception('impossible')
            elif res[0] == 's':
                stack_symbol.push(t)
                stack.push(res[1])
                idx += 1;
                if idx >= len(s):
                    break
                t = s[idx]
            elif res[0] == 'r':
                item = res[1]
                head = item.head
                body = item.body

                length = len(item.body)
                while length > 0:
                    stack_symbol.pop()
                    stack.pop()
                    length -= 1

                stack_symbol.push(head)
                state = stack.gettop()
                stack.push(self.GOTO(state, head))
            elif res[0] == "Accept":
                break
            else:
                raise Exception("fail")
                    

        print("All parsed")
        return 0
                

class LR1():
    pass

if __name__ == '__main__':
    productions = '''                       
    E  -> T E'      
    E' -> + T E' | ε
    T  -> F T'
    T' -> * F T' | ε
    F  -> ( E ) | id
    '''
    cfg = CFG(productions)
    ll = LL1(cfg)
    ll.print()
    ll.parse("id + id * id")



    productions = '''
    E -> E + T | T
    T -> T * F | F
    F -> ( E ) | id
    '''
    
    cfg = CFG(productions)
    parser = LR0(cfg)
    parser.print()
    parser.parse("id * id + id")
    #parser.view()


    parser.parse("id * id + id * ( id + id )")
    parser.view()    
    
    s = '''
                 E  -> T E'      
                 E' -> + T E' | ε
                 T  -> F T'
                 T' -> * F T' | ε
                 F  -> ( E ) | id
    '''
    cfg = CFG(s)
    parser = LR0(cfg)
    parser.print()
    parser.parse("id * id + id")    

    

