set -e

g++ dp.cpp -o dp -O2 -Wall -Wextra -std=c++20 -pedantic -Wshadow -Wformat=2 -Wfloat-equal -Wconversion -Wlogical-op -Wshift-overflow=2 -Wduplicated-cond -Wcast-qual -Wcast-align -D_GLIBCXX_DEBUG -fmax-errors=1 -DLOCAL
echo "Compiled!"
./dp "$1"
