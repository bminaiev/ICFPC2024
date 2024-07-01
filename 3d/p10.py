def done(x):
  if x != 0:
    print(0)
  else:
    print(1)
  exit(0)

a0 = int(input())
b0 = 0
while True:
  a1 = a0
  a2 = a0
  a3 = a0
  b1 = b0
  b2 = b0

  a4 = a1
  a5 = a1
  a6 = a1
  ap1 = a2 % 2
  az = (a3 == 0)
  b3 = b1
  b4 = b2

  a7 = a4
  ap2 = 9 * ap1
  b5 = b3
  ap3 = a5 % 2
  ap4 = a6 % 10
  if az:
    done(b4)

  a8 = a7
  ap5 = 10 - ap2
  b6 = b5
  ap6 = 1 - ap3
  ap7 = ap4

  a9 = a8
  a10 = a8
  a11 = a8
  b7 = ap5 * b6
  d = ap6 * ap7

  a12 = a9
  a13 = a9
  b8 = d + b7
  aq1 = a10 % 2
  aq2 = a11 % 10

  a14 = a12
  b9 = b8
  b10 = b8
  aq3 = aq1
  aq4 = aq2
  aq5 = a13 % 2

  a15 = a14
  b11 = b9
  bp1 = b10 % 10
  aq6 = aq3
  aq7 = aq4 + 1
  aq9 = aq5

  a16 = a15
  a17 = a15
  b12 = b11
  bp2 = bp1 * aq6
  aq8 = aq7 * aq9

  a18 = a16 // 10
  ar1 = a17 % 2
  b13 = b12
  if aq8 - bp2 != 0:
    print(0)
    exit()

  a19 = a18
  ar2 = ar1 * 9
  b14 = b13

  a20 = a19
  ar3 = ar2 + 1
  b15 = b14

  a0 = a20
  b0 = b15 // ar3

