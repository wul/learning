from functools import reduce

EPSILON = 'ε'
class NFA:
    def __init__(self):
        self.sset = {}
        self.start = None

    def get_or_create_state(self, label):
        state = self.sset.get(label, NFAState(label))
        self.sset[state.label] = state
        return state
        
    def set_start_state(self, label):
        state = self.get_or_create_state(label)
        state.set_start()
        self.start = state

    def set_final_state(self, label):
        state = self.get_or_create_state(label)
        state.set_final()        

    def get_start_state(self):
        return self.start

    def get_state_set(self):
        return set(self.sset.keys())

    def print(self):
        print("{", end='')
        for x in self.sset:
            print(x, end=',')
        print("}")
        
    def __eq__(self, another):
        return set(self.sset.keys()) == set(another.sset.keys())
        
    def build(self, rels, start, final_states):
        '''
        rels:
           0: (((edge1, 1), (edge2, 2), ...))
           1: ((edge11, 5), (edge12, 22), ...))
           ...
           9:  (())
           
        '''

        self.set_start_state(start)


        for label in final_states:
            sx = self.get_or_create_state(start)
            sx.set_final()

            
        for label, edges in rels.items():
            src = self.get_or_create_state(label)
            for (edge, dst_label) in edges:
                dst = self.get_or_create_state(dst_label)
                src.add_transition(edge, dst)
   


    
class NFAState:
    def __init__(self, label, final = False):
        '''
        edges: The out edges, a dict. The key is next state
        '''
        self.label = label
        self.final = final
        self.conns = {}

    def set_start(self):
        self.start = True
    
    def set_final(self):
        self.final = True
        
    def add_transition(self, edge, state):
        try:
            targets = self.conns[edge]
            targets.add(state)
        except KeyError:
            self.conns[edge] = set([state])

    def __str__(self):
        '''
        s = "{}:(".format(self.label)
        for edge, state in self.conns.items():
            s += "{}, {}".format(edge, str(state))
        s += ")"
        return s
        '''
        return str(self.label)
    
    def move(self, edge):
        '''
        Returns a set of next states
        '''
        try:
            return self.conns[edge]
        except KeyError:
            return set()

    def __hash__(self):
        return hash(self.label)

    def __eq__(self, another):
        return self.label == another.label



class DFAState(NFAState):
    def move(self, edge):
        conns = self.conns[edge]
        assert(len(conns) == 1)
        return conns[0]



#Simulating an NFA
'''
S = epsilon_closure(s0)
c = next_char()
while (c ！= eof) {
    S = epsilon_closure(move(S, c))
    c = next_char()


if S & F != 0:
    return "yes"
else:
    return "no"
'''



def subquential_char(ch: str)->str:
    return chr(ord(ch) + 1)

def print_set(sset):
    print("{", end='')
    for x in sset:
        print(x, end=',')
    print("}")

def string_encode(collection):
    return "{" + reduce(lambda x,y: str(x)+","+str(y), collection) + "}"
    

def move(sset: set, edge: str) -> set:
    '''Calculate states from sset status via edge
    '''
    
    new_sset = set()
    for state in sset:
        states = state.move(edge)
        new_sset |= states

    return new_sset

def epsilon_closure(sset: set) -> set:
    new_sset = sset.copy()
    stack = sset.copy()
    while len(stack) > 0:
        state = stack.pop()
        states = state.move(EPSILON)
        for u in states:
            if u not in new_sset:
                new_sset.add(u)
                stack.add(u)

    return new_sset


def subset_construction(nfa:NFA, edges: str)->tuple:
    '''
    nfa:    NFA
    edges:  the edges
    return:
           tuple of (DFA, DTrans)
    '''
    
    DTrans    = {}
    dfa_state = {}
    mark      = {}

    
    s0  = nfa.get_start_state()
    ns0 = epsilon_closure(set((s0,)))
    
    idx = 'A'
    dfa_state[idx] = ns0
    mark[idx] = False
    to = None
    
    while True:
        for i, ds in dfa_state.items():
            if not mark[i]:
                break
        else:
            #all marked, exit
            break

        mark[i] = True
        
        for edge in edges:
            nso = move(ds, edge)
            nns = epsilon_closure(nso)            

            #Check if the new set of NFA states already existing
            
            for target_idx, s in dfa_state.items():
                if s == nns:
                    break
            else:
                idx            = subquential_char(idx)
                dfa_state[idx] = nns
                mark[idx]      = False
                target_idx = idx
                
            DTrans[i, edge] = target_idx

    #return dfa_state
    return DTrans, dfa_state
    

n = NFA()

conns = {
    0: ((EPSILON, 1), (EPSILON, 7)),
    1: ((EPSILON, 2), (EPSILON, 4)),
    2: (('a', 3),),
    3: ((EPSILON, 6),),
    4: (('b', 5),),
    5: ((EPSILON, 6),),
    6: ((EPSILON, 7), (EPSILON, 1)),
    7: (('a', 8),),
    8: (('b', 9),),
    9: (('b', 10),),
    10: tuple(),
    }

#0 is start state, 10 is final state
n.build(conns, 0, [10])
#n.print()



edges = "ab"
dtrans, dfa_state = subset_construction(n, edges)


print("{:25s}\t{:25s}\t{}".format("NFA STATE", "DFA STATE", edges))
for idx, ss in dfa_state.items():
    s = "{:25s}\t{:25s}\t".format(string_encode(ss), idx)
    for edge in edges:
        s += dtrans[idx, edge]
    s += '\n'
    print(s)
        

'''
for key, value in dtrans.items():
    print("{} : ".format(key), end="")
    print_set(value)
''' 
#pprint.pprint(dtrans)

