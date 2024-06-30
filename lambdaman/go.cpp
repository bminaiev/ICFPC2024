/**
 *    author:  tourist
 *    created: 28.06.2024 20:51:09
**/
#include <bits/stdc++.h>

using namespace std;

#ifdef LOCAL
#include "algo/debug.h"
#else
#define debug(...) 42
#endif

const int dx[4] = {1, 0, -1, 0};
const int dy[4] = {0, 1, 0, -1};
const string let = "DRUL";

int main(int argc, char** argv) {
  if (argc < 2) {
    cerr << "usage: go [test_id]" << '\n';
    return 0;
  }
  int test_id = atoi(argv[1]);
  ios::sync_with_stdio(false);
  cin.tie(0);
  ifstream in(string("lambdaman") + char('0' + test_id / 10) + char('0' + test_id % 10) + ".in");
  vector<string> grid;
  string foo;
  while (in >> foo) {
    grid.push_back(foo);
  }
  int h = int(grid.size());
  int w = int(grid[0].size());
  string res = "";
  vector go(h, vector<vector<array<int, 3>>>(w));
  vector pr(h, vector<pair<int, int>>(w));
  vector depth(h, vector<int>(w, -1));
  int visited = 0;
  auto Dfs = [&](auto&& self, int x, int y) -> void {
    grid[x][y] = '!';
    for (int dir = 0; dir < 4; dir++) {
      int nx = x + dx[dir];
      int ny = y + dy[dir];
      if (nx >= 0 && ny >= 0 && nx < h && ny < w && grid[nx][ny] == '.') {
        visited += 1;
        depth[nx][ny] = depth[x][y] + 1;
        go[x][y].push_back({nx, ny, dir});
        pr[nx][ny] = {x, y};
        self(self, nx, ny);
      }
    }
  };
  int li = -1, lj = -1;
  for (int i = 0; i < h; i++) {
    for (int j = 0; j < w; j++) {
      if (grid[i][j] == 'L') {
        depth[i][j] = 0;
        Dfs(Dfs, i, j);
        li = i, lj = j;
      }
    }
  }
  int fi = -1, fj = -1;
  for (int i = 0; i < h; i++) {
    for (int j = 0; j < w; j++) {
      if (fi == -1 || depth[i][j] > depth[fi][fj]) {
        fi = i;
        fj = j;
      }
    }
  }
  debug(li, lj, fi, fj, visited, depth[fi][fj]);
  vector go_last(h, vector<bool>(w, false));
  {
    int ai = fi, aj = fj;
    while (ai != li || aj != lj) {
      go_last[ai][aj] = true;
      auto [bi, bj] = pr[ai][aj];
      ai = bi;
      aj = bj;
    }
  }
  // for (int i = 0; i < h; i++) debug(go_last[i], go[i]);
  auto Build = [&](auto&& self, int x, int y) -> void {
    grid[x][y] = '!';
    for (auto [nx, ny, dir] : go[x][y]) {
      if (go_last[nx][ny]) continue;
      res += let[dir];
      self(self, nx, ny);
      res += let[dir ^ 2];
    }
    for (auto [nx, ny, dir] : go[x][y]) {
      if (!go_last[nx][ny]) continue;
      res += let[dir];
      self(self, nx, ny);
    }
  };
  Build(Build, li, lj);
  // debug(res);
  if (test_id == 5) {
    res = "RDLLLULURURDURRRRDLRRDRDLLUDDRLLDRLLLLLLLURLLURLLURULRURURURRRRRRRRURRDLDLLRDRRUDDLRDDDLDRDDLULUDLDRLLLLLLLLLLULDLLURURLLURLUUUURURURURLLLLDRDL";
  }
  // debug(res);
  debug(res.size(), 2 * visited - depth[fi][fj]);
  ofstream out(string("") + char('0' + test_id / 10) + char('0' + test_id % 10) + ".out");
  if (test_id == 16) {
    string me = "a";
    for (int iter = 0; iter < 6; iter++) {
      string new_me = "";
      for (char c : me) {
        if (c == 'a') {
          new_me += "bDaRaUd";
          continue;
        }
        if (c == 'b') {
          new_me += "aRbDbLc";
          continue;
        }
        if (c == 'c') {
          new_me += "dUcLcDb";
          continue;
        }
        if (c == 'd') {
          new_me += "cLdUdRa";
          continue;
        }
        new_me += c;
      }
      me = new_me;
    }
    me.pop_back();
    for (int i = 0; i < int(me.size()); i += 2) me[i] = me[i + 1];
    out << me << '\n';
  }
  reverse(res.begin(), res.end());
  out << res << '\n';
  // debug("hi", res);
  vector<int> num = {1};
  int shift = 1;
  if (11 <= test_id && test_id <= 15) shift = 2;
  for (int id = 0; id < int(res.size()); id += shift) {
    char c = res[id];
    int val = int(let.find(c));
    int carry = val;
    for (int i = 0; i < int(num.size()); i++) {
      num[i] = num[i] * 4 + carry;
      carry = num[i] / 94;
      num[i] %= 94;
    }
    while (carry > 0) {
      num.push_back(carry % 94);
      carry /= 94;
    }
    // if (c == 'L' && id + shift < int(res.size()) && res[id + shift] == 'D') {
    //   id += shift;
    // } else {
    //   res[id + shift] = 
    // }
  }
  string encoder = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";
  string ans = "";
  for (int i = int(num.size()) - 1; i >= 0; i--) {
    ans += char(33 + num[i]);
  }
  string encoded_test_id = "";
  int tmp = test_id;
  while (tmp > 0) {
    encoded_test_id += char(33 + int(encoder.find(char('0' + tmp % 10))));
    tmp /= 10;
  }
  out << test_id << '\n';
  out << encoded_test_id << '\n';
  reverse(encoded_test_id.begin(), encoded_test_id.end());
  string final = "";
  if (11 <= test_id && test_id <= 15) {
    final += "B. S3/,6%},!-\"$!-!." + encoded_test_id + "} B$ B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L\" L# ? B= v# I\" S B. BT I# BD B* I# B% v# I% S>>LLOOFF B$ v\" B/ v# I% U# S";
  } else {
    final += "B. S3/,6%},!-\"$!-!." + encoded_test_id + "} B$ B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L\" L# ? B= v# I\" S B. BT I\" BD B% v# I% S>LOF B$ v\" B/ v# I% U# S";
  }
  final += ans;
  debug(final.size());
  out << final << '\n';
  return 0;
}
