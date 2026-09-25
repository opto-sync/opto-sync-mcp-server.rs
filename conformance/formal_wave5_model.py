from itertools import product

def admitted(authenticated, tenant_match, write_tool, write_scope):
    return authenticated and tenant_match and (not write_tool or write_scope)

for state in product((False, True), repeat=4):
    ok = admitted(*state)
    if ok:
        assert state[0] and state[1]
        if state[2]: assert state[3]

assert admitted(True,True,False,False)
assert not admitted(True,True,True,False)
print('formal_wave5_model: ok')
