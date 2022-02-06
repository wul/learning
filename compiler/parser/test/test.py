import os
import sys
import logging
logging.basicConfig(level=logging.DEBUG)

path = os.path.sep.join([os.path.abspath(os.path.dirname(__file__)), '..'])
sys.path.append(path)
print(path)
import unittest

from parselib import LR0,  Item, CFG, EPSILON

productions = '''                       
E  -> T E'      
E' -> + T E' | ε
T  -> F T'
T' -> * F T' | ε
F  -> ( E ) | id
'''

class TestCFG(unittest.TestCase):
    def setUp(self):
        self.cfg = CFG(productions)

    def test_iter(self):
        for x in self.cfg:
            logging.info("{}:{}".format(x.head, x.bodies))
        
class TestLR0(unittest.TestCase):
    def setUp(self):
        self.cfg = CFG(productions)


    def test_first_set_map(self):
        productions = '''
        E -> E + T | T
        T -> T * F | F
        F -> ( E ) | id
        '''
        cfg = CFG(productions)

        
        nt = "E"

        s = cfg.get_first_set_map(nt)
        self.assertEqual(s, {'id': ["T"], '(': ["T"]})


        nt = "T"
        self.assertEqual(cfg.get_first_set_map(nt), {'id':["F"], '(':["F"]})


        nt = "F"
        self.assertEqual(cfg.get_first_set_map(nt), {'id':["id"], '(':["(", "E", ")"]})
        
        
    def test_first_set(self):
        self.assertEqual(self.cfg.FIRST("E"), {'id', '('})
        self.assertEqual(self.cfg.FIRST("E'"), {'+', EPSILON})        
        self.assertEqual(self.cfg.FIRST("T"), {'id', '('})
        self.assertEqual(self.cfg.FIRST("T'"), {'*', EPSILON})        
        self.assertEqual(self.cfg.FIRST("F"), {'id', '('})

    def test_first_set2(self):
        self.assertEqual(self.cfg.FIRST("E E'"), {'id', '('})
        self.assertEqual(self.cfg.FIRST("E' T' F"), {'+', '*', EPSILON, 'id', '('})        

        

    def test_follow(self):
        self.assertEqual(self.cfg.FOLLOW("E"), {')', '$'})
        self.assertEqual(self.cfg.FOLLOW("E'"), {')', '$'})
        self.assertEqual(self.cfg.FOLLOW("T"), {'+', ')', '$'})
        self.assertEqual(self.cfg.FOLLOW("T'"), {'+', ')', '$'})
        self.assertEqual(self.cfg.FOLLOW("F"), {'*', '+', ')', '$'})                                
            
    def test_closure(self):
        lr0 = LR0(self.cfg)
        items = lr0.CLOSURE2("E")
        #import pdb;pdb.set_trace()
        self.assertEqual(items, [Item(head="E", body=("T", "E'"), dot=0),
                                 Item(head="T", body=("F", "T'"), dot=0),
                                 Item(head="F", body=("(", "E", ")"), dot=0),
                                 Item(head="F", body=("id",), dot=0),                                 
                                 ])

    def test_get_next_goto_X(self):
        lr0 = LR0(self.cfg)

        I0 = self.cfg.get_production("E").deconstruct()
        I0.insert(0, Item(head="S'", body=(self.cfg.S,), dot=0))
        lr0.CLOSURE(I0)
        
        Xs = lr0.get_next_goto_X(I0)
        self.assertEqual(Xs, ["E", "T", "F", "(", "id"])

    def test_get_state_idx(self):
        lr0 = LR0(self.cfg)
        I0 = lr0.CLOSURE2("E")

        idx = lr0.get_state_idx(I0)
        self.assertEqual(idx, None)

        I0.insert(0, Item(head="S'", body=(self.cfg.S,), dot=0))
        idx = lr0.get_state_idx(I0)
        self.assertEqual(idx, 0)
        
    def test_calc_next_state(self):
        productions = '''
        E -> E + T | T
        T -> T * F | F
        F -> ( E ) | id
        '''
        self.cfg = CFG(productions)
        lr0 = LR0(self.cfg)
        I0 = lr0.CLOSURE2("E")
        I0.insert(0, Item(head="S'", body=(self.cfg.S,), dot=0))

        I1 = lr0.calc_next_state(I0, "E")

        self.assertEqual(I1, [Item(head="S'", body=("E",), dot=1),
                              Item(head="E", body=("E", "+", "T"), dot=1),
                              ])        

    
if __name__ == '__main__':
    unittest.main()
