'''
S -> + S S | - S S | a
'''

class Stream:
    def __init__(self, s):
        self.idx = 0
        self.s = s
        self.lookahead = self.s[self.idx]
    def lookahead(self):
        return self.s[self.idx]
    
    def match(self, terminal):
        if terminal == self.lookahead():
            print("Termianl matched: {}".format(terminal))
            self.idx += 1
            
    def next_token(self):
        if self.idx + 1 = len(self.s):
            return None
        else:
            self.idx += 1
            return self.s[self.idx]
          
s = Stream("+a-aa")

def S():
    if s.lookahead() in ('+', '-'):
        S()
        S()
    elif s.lookahead() == 'a':
        match(a)
    print("Parse done!")
    
S()
