import sys
sys.setrecursionlimit(10000)
print("L" + (((lambda v1: ((lambda v1: ((lambda v2: (v1((v2(v2)))))((lambda v2: (v1((v2(v2))))))))((lambda v3: (lambda v2: (v1 if (v2 == 1) else (v1 + (v3((v2 - 1))))))))))(","))(199)))
