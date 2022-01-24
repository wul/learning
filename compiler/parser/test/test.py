import os
import sys

path = os.path.sep.join([os.path.abspath(os.path.dirname(__file__)), '..'])
sys.path.append(path)
print(path)
import unittest

from parselib import LR0, normalize, Item

productions = '''                       
E  -> T E'      
E' -> + T E' | ε
T  -> F T'
T' -> * F T' | ε
F  -> ( E ) | id
'''
class TestLR0(unittest.TestCase):
    def setUp(self):
        self.cfg = normalize(productions)
            
    def test_closure(self):
        lr0 = LR0(self.cfg)
        items = lr0.closure("E")
        self.assertEqual(items, {Item(head="E", body=("T", "E'"), dot=0),
                                 Item(head="T", body=("F", "T'"), dot=0),
                                 Item(head="F", body=("(", "E", ")"), dot=0),
                                 Item(head="F", body=("id",), dot=0),                                 
        })

    def test_get_next_goto_X(self):
        lr0 = LR0(self.cfg)
        I0 = lr0.closure("E")
        I0.add(Item(head="S'", body=(self.cfg.S,), dot=0))
        
        Xs = lr0.get_next_goto_X(I0)
        self.assertEqual(sorted(Xs), sorted(("E", "T", "F", "(", "id")))

    def test_get_state_idx(self):
        lr0 = LR0(self.cfg)
        I0 = lr0.closure("E")

        idx = lr0.get_state_idx(I0)
        self.assertEqual(idx, None)

        I0.add(Item(head="S'", body=(self.cfg.S,), dot=0))
        idx = lr0.get_state_idx(I0)
        self.assertEqual(idx, 0)
        
    def test_goto(self):
        productions = '''
        E -> E + T | T
        T -> T * F | F
        F -> ( E ) | id
        '''
        self.cfg = normalize(productions)
        lr0 = LR0(self.cfg)
        I0 = lr0.closure("E")
        I0.add(Item(head="S'", body=(self.cfg.S,), dot=0))

        I1 = lr0.goto(I0, "E")
        self.assertEqual(I1, {Item(head="S'", body=("E",), dot=1),
                              Item(head="E", body=("E", "+", "T'"), dot=0),
        })        
        pass
    
if __name__ == '__main__':
    unittest.main()
