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

  inline bool operator==(const TPoint& rhs) const {
    return (y == rhs.y && x == rhs.x);
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
  vector<bool> used(n, false);
  used[0] = true;
  Point sp = {0, 0};
  for (int it = 1; it < n; it++) {
    int min_d = int(1e9);
    int idx = -1;
    for (int j = 1; j < n; j++) {
      if (used[j]) {
        continue;
      }
      // if (pts[order[it - 1]] == Point{-165, -173} && pts[j] == Point{-167, -172}) {
      //   continue;
      // }
      auto delta = pts[j] - pts[order[it - 1]];
      // if ((delta - sp).abs2() > 2) {
      //   continue;
      // }
      int d = abs(delta.x) + abs(delta.y);
      if (d < min_d) {
        min_d = d;
        idx = j;
      }
    }
    used[idx] = true;
    order[it] = idx;
    sp = pts[order[it]] - pts[order[it - 1]];
  }
  if (test_id == 9) {
    // order = {0, 2, 64, 69, 72, 68, 100, 55, 51, 90, 5, 38, 93, 46, 75, 1, 95, 81, 3, 62, 66, 61, 91, 54, 59, 29, 74, 27, 40, 43, 22, 65, 16, 84, 50, 71, 85, 34, 35, 52, 33, 57, 76, 13, 20, 80, 45, 47, 60, 12, 14, 7, 31, 98, 87, 73, 21, 30, 23, 99, 94, 49, 4, 83, 11, 88, 53, 79, 82, 77, 89, 97, 44, 18, 96, 67, 36, 6, 17, 28, 63, 70, 42, 78, 56, 37, 24, 10, 19, 92, 32, 9, 15, 26, 39, 58, 8, 48, 86, 41, 25};
    order = {0, 2, 64, 69, 72, 68, 100, 55, 51, 90, 5, 38, 93, 46, 75, 1, 95, 81, 3, 62, 66, 61, 91, 54, 59, 74, 29, 27, 40, 43, 22, 65, 16, 84, 50, 71, 85, 34, 35, 52, 33, 57, 76, 13, 20, 80, 45, 47, 60, 12, 14, 7, 31, 98, 87, 73, 21, 30, 23, 99, 94, 11, 83, 4, 88, 53, 49, 79, 82, 77, 89, 97, 44, 18, 96, 67, 36, 6, 17, 28, 63, 70, 42, 78, 56, 37, 24, 10, 19, 92, 32, 9, 15, 26, 39, 58, 8, 48, 86, 41, 25};
  }
  // if (test_id == 10) {
  //   order = {0, 24, 74, 51, 45, 62, 18, 12, 5, 37, 3, 40, 46, 92, 88, 6, 90, 4, 30, 36, 97, 73, 47, 63, 22, 32, 53, 8, 65, 15, 71, 89, 77, 11, 56, 16, 57, 64, 84, 19, 85, 42, 49, 17, 7, 81, 9, 20, 14, 28, 72, 93, 66, 2, 98, 87, 58, 61, 33, 13, 35, 79, 54, 44, 1, 48, 96, 80, 29, 83, 67, 26, 39, 43, 55, 100, 52, 25, 78, 68, 60, 21, 95, 86, 75, 59, 70, 50, 38, 41, 94, 99, 82, 91, 34, 76, 69, 31, 23, 10, 27};
  // }
  for (int i = 1; i < n; i++) {
    // debug(pts[order[i]], pts[order[i]] - pts[order[i - 1]]);
  }
  // return 0;
  const int M = 200;
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
  auto DP = [&]() {
    const int LIM = 10;
    vector dp(n, vector(2 * LIM + 1, vector<int>(2 * LIM + 1, inf)));
    vector pr(n, vector(2 * LIM + 1, vector<Point>(2 * LIM + 1, {-1, -1})));
    dp[0][LIM][LIM] = 0;
    for (int i = 0; i < n - 1; i++) {
      vector<array<int, 3>> bests;
      for (int sx = -LIM; sx <= LIM; sx++) {
        for (int sy = -LIM; sy <= LIM; sy++) {
          auto ft = dp[i][sx + LIM][sy + LIM];
          if (ft == inf) {
            continue;
          }
          bests.push_back({ft, sx, sy});
        }
      }
      sort(bests.begin(), bests.end());
      for (int id = 0; id < min(int(bests.size()), 20); id++) {
        auto [ft, sx, sy] = bests[id];
        Point sa = {sx, sy};
        for (int nx = -LIM; nx <= LIM; nx++) {
          for (int ny = -LIM; ny <= LIM; ny++) {
            Point sb = {nx, ny};
            auto val = ft + Eval(pts[order[i]], pts[order[i + 1]], sa, sb);
            int& to = dp[i + 1][nx + LIM][ny + LIM];
            if (val < to) {
              to = val;
              pr[i + 1][nx + LIM][ny + LIM] = sa;
            }
          }
        }
      }
    }
    int best = inf;
    int bx = -1, by = -1;
    for (int sx = -LIM; sx <= LIM; sx++) {
      for (int sy = -LIM; sy <= LIM; sy++) {
        auto ft = dp[n - 1][sx + LIM][sy + LIM];
        if (ft < best) {
          best = ft;
          bx = sx;
          by = sy;
        }
      }
    }
    assert(best < inf);
    debug(test_id, clock(), best);
    for (int i = n - 1; i > 0; i--) {
      speeds[order[i]] = {bx, by};
      auto from = pr[i][bx + LIM][by + LIM];
      bx = from.x;
      by = from.y;
    }
    return best;
  };
  int score = DP();
  mt19937 rng(58);
  uniform_real_distribution<double> urd(0, 1);
  double init_temp = 0.1;
  double final_temp = 0.1;
  double cur_temp = init_temp;
  int best_score = score;
  auto best_order = order;
  auto best_speeds = speeds;
  const double TL = 0.0;
  int it = 0;
  while (1.0 * clock() / CLOCKS_PER_SEC < TL) {
    if (it % 100 == 0) {
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
      } while (i == j);// || abs(i - j) > 5);
      int me = order[i];
      order.erase(order.begin() + i);
      order.insert(order.begin() + j, me);
      if ((pts[order[j]] - pts[order[j - 1]]).abs2() > 2500) {
        order.erase(order.begin() + j);
        order.insert(order.begin() + i, me);
        continue;
      }
      int delta = DP();
      delta -= score;
      if (delta <= 0) {// || (delta > 0 && urd(rng) < exp(-1.0 * delta / cur_temp))) {
        score += delta;
      } else {
        order.erase(order.begin() + j);
        order.insert(order.begin() + i, me);
      }
    } else {
      int i, j;
      do {
        i = rng() % (n - 1) + 1;
        j = rng() % (n - 1) + 1;
      } while (i >= j);// || abs(i - j) > 5);
      if ((pts[order[j]] - pts[order[i - 1]]).abs2() > 2500) {
        continue;
      }
      reverse(order.begin() + i, order.begin() + j + 1);
      int delta = DP();
      delta -= score;
      if (delta <= 0) {// || (delta > 0 && urd(rng) < exp(-1.0 * delta / cur_temp))) {
        score += delta;
      } else {
        reverse(order.begin() + i, order.begin() + j + 1);
      }
    }
    if (score < best_score) {
      best_score = score;
      best_order = order;
      best_speeds = speeds;
      debug(test_id, it, clock(), best_score);
    }
  }
  debug(best_score);
  order = best_order;
  // debug(order);
  speeds = best_speeds;
  string res = "";
  for (int i = 1; i < n; i++) {
    Point a = pts[order[i - 1]];
    Point b = pts[order[i]];
    Point sa = speeds[order[i - 1]];
    Point sb = speeds[order[i]];
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
  debug("hi, done");
  string out_filename = string("") + char('0' + test_id / 10) + char('0' + test_id % 10) + ".out";
  ofstream out(out_filename);
  out << "solve spaceship" << test_id << " " << res << '\n';
  out.close();
  return 0;
}
