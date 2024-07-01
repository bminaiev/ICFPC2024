import sys

a = []
for line in sys.stdin:
    tokens = line.strip().split()
    if not tokens:
        continue
    if a:
        assert(len(a[-1]) == len(tokens))
    a.append(tokens)

src, dst = {}, {}
n, m = len(a), len(a[0])
# print(n, m)

new_a = []
for i in range(n):
    new_row = []
    for j in range(m):
        pref, cur = "", a[i][j]
        #while cur and cur[0] != 'v' and 'a' <= cur[0] <= 'z':
        #    pref, cur = pref + cur[0], cur[1:]
        ff = -1
        for w, c in enumerate(cur):
            if c != 'v' and 'a' <= c <= 'z':
                ff = w
                break
        if ff != -1:
            pref = cur[ff:]
            cur = cur[:ff]
        if pref:
            if i + 1 < n and a[i+1][j] == '@':
                if pref in src:
                    print(f"'{pref}' used more than once near time warp")
                    sys.exit(1)
                src[pref] = (i, j)
            else:
                if pref in dst:
                    print(f"'{pref}' used more than once as target for time warp")
                    sys.exit(1)
                dst[pref] = (i, j)
        if not cur:
            cur = '.'
        new_row.append(cur)
    new_a.append(new_row)

for k in src:
    if k not in dst:
        print(f"'{k}' is in src, but not in dst")
        sys.exit(1)
    dx = src[k][1] - dst[k][1]
    dy = src[k][0] + 1 - dst[k][0]
    new_a[src[k][0] + 1][src[k][1] - 1] = str(dx)
    new_a[src[k][0] + 1][src[k][1] + 1] = str(dy)

for k in dst:
    if k not in src:
        print(f"'{k}' is in dst, but not in src")
        sys.exit(1)

for r in new_a:
    print(" ".join(r))

for r in new_a:
    for e in r:
        if e == '?':
            print('Question marks left')
            sys.exit(1)

