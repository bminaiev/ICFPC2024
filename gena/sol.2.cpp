/**
 *    author:  tourist
 *    created: 30.06.2024 00:45:37
**/
#undef _GLIBCXX_DEBUG

#include <bits/stdc++.h>

using namespace std;

#ifdef LOCAL
#include "algo/debug.h"
#else
#define debug(...) 42
#endif

template <typename T>
struct TPoint {
  T x;
  T y;
  int id;

  TPoint() : x(0), y(0), id(-1) {}
  TPoint(const T& x_, const T& y_) : x(x_), y(y_), id(-1) {}
  TPoint(const T& x_, const T& y_, int id_) : x(x_), y(y_), id(id_) {}

  static constexpr T eps = static_cast<T>(1e-9);

  inline TPoint operator+(const TPoint& rhs) const { return TPoint(x + rhs.x, y + rhs.y); }
  inline TPoint operator-(const TPoint& rhs) const { return TPoint(x - rhs.x, y - rhs.y); }
  inline TPoint operator*(const T& rhs) const { return TPoint(x * rhs, y * rhs); }
  inline TPoint operator/(const T& rhs) const { return TPoint(x / rhs, y / rhs); }

  friend T smul(const TPoint& a, const TPoint& b) {
    return a.x * b.x + a.y * b.y;
  }

  friend T vmul(const TPoint& a, const TPoint& b) {
    return a.x * b.y - a.y * b.x;
  }

  inline T abs2() const {
    return x * x + y * y;
  }

  inline bool operator<(const TPoint& rhs) const {
    return (y < rhs.y || (y == rhs.y && x < rhs.x));
  }

  inline bool is_upper() const {
    return (y > eps || (abs(y) <= eps && x > eps));
  }

  inline int cmp_polar(const TPoint& rhs) const {
    assert(abs(x) > eps || abs(y) > eps);
    assert(abs(rhs.x) > eps || abs(rhs.y) > eps);
    bool a = is_upper();
    bool b = rhs.is_upper();
    if (a != b) {
      return (a ? -1 : 1);
    }
    long long v = x * rhs.y - y * rhs.x;
    return (v > eps ? -1 : (v < -eps ? 1 : 0));
  }
};

using Point = TPoint<int>;
// using Point = TPoint<long long>;
//using Point = TPoint<long double>;

template <typename T>
string to_string(const TPoint<T>& p) {
  return "(" + to_string(p.x) + ", " + to_string(p.y) + ")";
}

int main(int argc, char** argv) {
  if (argc < 2) {
    cerr << "usage: go [test_id]" << '\n';
    return 0;
  }
  int test_id = atoi(argv[1]);
  ios::sync_with_stdio(false);
  cin.tie(0);
  string in_filename = string("spaceship") + char('0' + test_id / 10) + char('0' + test_id % 10) + ".in";
  ifstream in(in_filename);
  vector<Point> pts;
  pts.push_back({0, 0});
  int foo_x, foo_y;
  while (in >> foo_x >> foo_y) {
    pts.push_back({foo_x, foo_y});
  }
  int n = int(pts.size());
  vector<int> order(n);
  iota(order.begin(), order.end(), 0);
  const int M = 100;
  const int inf = int(1e6);
  vector fmin(M + 1, vector<int>(2 * M + 1, +inf));
  vector fmax(M + 1, vector<int>(2 * M + 1, -inf));
  fmin[0][M] = fmax[0][M] = 0;
  for (int i = 1; i < M; i++) {
    for (int j = -i; j <= i; j++) {
      fmin[i][j + M] = min({fmin[i - 1][j - 1 + M], fmin[i - 1][j + M], fmin[i - 1][j + 1 + M]}) + j;
      fmax[i][j + M] = max({fmax[i - 1][j - 1 + M], fmax[i - 1][j + M], fmax[i - 1][j + 1 + M]}) + j;
    }
  }
  vector<Point> speeds(n, {0, 0});
  auto Eval = [&](Point a, Point b, Point sa, Point sb) {
    Point delta = sb - sa;
    for (int k = max(abs(delta.x), abs(delta.y)); k < M; k++) {
      int min_x = sa.x * k + fmin[k][delta.x + M];
      int max_x = sa.x * k + fmax[k][delta.x + M];
      if (b.x < a.x + min_x || b.x > a.x + max_x) {
        continue;
      }
      int min_y = sa.y * k + fmin[k][delta.y + M];
      int max_y = sa.y * k + fmax[k][delta.y + M];
      if (b.y < a.y + min_y || b.y > a.y + max_y) {
        continue;
      }
      // debug(a, b, sa, sb, min_x, max_x, min_y, max_y, k);
      return k;
    }
    return inf;
  };
  int score = 0;
  for (int i = 0; i < n - 1; i++) {
    score += Eval(pts[order[i]], pts[order[i + 1]], speeds[order[i]], speeds[order[i + 1]]);
  }
  mt19937 rng(58);
  uniform_real_distribution<double> urd(0, 1);
  double init_temp = 50;
  double final_temp = 0.1;
  double cur_temp = init_temp;
  int best_score = score;
  auto best_order = order;
  auto best_speeds = speeds;
  const double TL = 1000.0;
  int it = 0;
  while (1.0 * clock() / CLOCKS_PER_SEC < TL) {
    if (it % 100000 == 0) {
      auto t = 1.0 * clock() / CLOCKS_PER_SEC;
      cur_temp = init_temp * pow(final_temp / init_temp, t / TL);
      debug(test_id, it, t, cur_temp, score, best_score);
    }
    it += 1;
    if (it % 2 == 0) {
      int i = rng() % (n - 1) + 1;
      int j;
      do {
        j = rng() % (n - 1) + 1;
      } while (i == j);//|| abs(i - j) > 20);
      int delta = 0;
      delta -= Eval(pts[order[i - 1]], pts[order[i]], speeds[order[i - 1]], speeds[order[i]]);
      if (i < n - 1) {
        delta -= Eval(pts[order[i]], pts[order[i + 1]], speeds[order[i]], speeds[order[i + 1]]);
        delta += Eval(pts[order[i - 1]], pts[order[i + 1]], speeds[order[i - 1]], speeds[order[i + 1]]);
      }
      int pr = j - 1, ne = j;
      if (j >= i) {
        pr += 1;
        ne += 1;
      }
      if (ne < n) {
        delta -= Eval(pts[order[pr]], pts[order[ne]], speeds[order[pr]], speeds[order[ne]]);
        delta += Eval(pts[order[i]], pts[order[ne]], speeds[order[i]], speeds[order[ne]]);
      }
      delta += Eval(pts[order[pr]], pts[order[i]], speeds[order[pr]], speeds[order[i]]);
      Point ds = {0, 0};
      while (true) {
        bool found = false;
        for (int dx = -1; dx <= 1; dx++) {
          for (int dy = -1; dy <= 1; dy++) {
            if (dx == 0 && dy == 0) {
              continue;
            }
            int mini_delta = 0;
            if (ne < n) {
              mini_delta -= Eval(pts[order[i]], pts[order[ne]], speeds[order[i]] + ds, speeds[order[ne]]);
            }
            mini_delta -= Eval(pts[order[pr]], pts[order[i]], speeds[order[pr]], speeds[order[i]] + ds);
            auto new_ds = ds + Point(dx, dy);
            if (ne < n) {
              mini_delta += Eval(pts[order[i]], pts[order[ne]], speeds[order[i]] + new_ds, speeds[order[ne]]);
            }
            mini_delta += Eval(pts[order[pr]], pts[order[i]], speeds[order[pr]], speeds[order[i]] + new_ds);
            if (mini_delta <= 0) {
              delta += mini_delta;
              if (mini_delta < 0) found = true;
              ds = new_ds;
            }
          }
        }
        if (!found) {
          break;
        }
      }
      if (delta <= 0 || (delta > 0 && urd(rng) < exp(-1.0 * delta / cur_temp))) {
        score += delta;
        int me = order[i];
        speeds[me] = speeds[me] + ds;
        order.erase(order.begin() + i);
        order.insert(order.begin() + j, me);
      }
    } else {
      int i = rng() % (n - 1) + 1;
      int dx, dy;
      do {
        dx = int(rng() % 3) - 1;
        dy = int(rng() % 3) - 1;
      } while (dx == 0 && dy == 0);
      int delta = 0;
      delta -= Eval(pts[order[i - 1]], pts[order[i]], speeds[order[i - 1]], speeds[order[i]]);
      if (i < n - 1) {
        delta -= Eval(pts[order[i]], pts[order[i + 1]], speeds[order[i]], speeds[order[i + 1]]);
      }
      speeds[order[i]].x += dx;
      speeds[order[i]].y += dy;
      delta += Eval(pts[order[i - 1]], pts[order[i]], speeds[order[i - 1]], speeds[order[i]]);
      if (i < n - 1) {
        delta += Eval(pts[order[i]], pts[order[i + 1]], speeds[order[i]], speeds[order[i + 1]]);
      }
      if (delta <= 0 || (delta > 0 && urd(rng) < exp(-1.0 * delta / cur_temp))) {
        score += delta;
      } else {
        speeds[order[i]].x -= dx;
        speeds[order[i]].y -= dy;
      }
    }
    if (score < best_score) {
      best_score = score;
      best_order = order;
      best_speeds = speeds;
      // debug(it, best_score, best_order, best_speeds);
    }
  }
  debug(best_score);
  string res = "";
  for (int i = 1; i < n; i++) {
    Point a = pts[best_order[i - 1]];
    Point b = pts[best_order[i]];
    Point sa = best_speeds[best_order[i - 1]];
    Point sb = best_speeds[best_order[i]];
    int k = Eval(a, b, sa, sb);
    for (int j = k - 1; j >= 0; j--) {
      bool found = false;
      for (int dx = -1; dx <= 1; dx++) {
        for (int dy = -1; dy <= 1; dy++) {
          if (!found && Eval(a + sa + Point(dx, dy), b, sa + Point(dx, dy), sb) == j) {
            found = true;
            res += char('0' + (dy + 1) * 3 + (dx + 2));
            sa = sa + Point(dx, dy);
            a = a + sa;
            break;
          }
        }
      }
      assert(found);
    }
  }
  string out_filename = string("") + char('0' + test_id / 10) + char('0' + test_id % 10) + ".out";
  ofstream out(out_filename);
  out << "solve spaceship" << test_id << " " << res << '\n';
  out.close();
  return 0;
}
