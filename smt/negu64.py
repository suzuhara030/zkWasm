from z3 import *
from utils import *
from functools import reduce

s = init_z3_solver()

#define spec
field, res = Ints('field res')
s.add(is_u64(field))
s.add(is_u64(res))

wasm_neg_u64 = Function('WasmNegU64', IntSort(), IntSort())
s.add(ForAll([field], wasm_neg_u64(field) == (U64_MODULUS - field) % U64_MODULUS))

#define var

overflow = Int('overflow')

constrain = overflow == 0
constraints = [
    fr_sub(fr_sub(U64_MODULUS, field), res) == 0
]

s.add(ForAll([overflow], And(constrain == reduce(lambda x, y: And(x, y), constraints))))

s.push()
# Soundness
s.add(And(constrain, wasm_neg_u64(field) != res))

check_res = s.check()
print('--------------Soundness---------------')
if check_res.r == Z3_L_TRUE:
    print('Verify: Fail')
    print(check_res)
    print(s.model())
elif check_res.r == Z3_L_FALSE:
    print('Verify: Pass')
else:
    print('Verify: Fail')
s.pop()

s.push()
# Completeness
s.add(And(wasm_neg_u64(field) == res,  ForAll([overflow], constrain)))

check_res = s.check()
print('-------------Completeness----------------')
if check_res.r == Z3_L_TRUE:
    print('Verify: Fail')
    print(s.model())
elif check_res.r == Z3_L_FALSE:
    print('Verify: Pass')
else:
    print('Verify: Fail')
s.pop()
