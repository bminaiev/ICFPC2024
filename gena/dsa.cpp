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

  inline int64_t abs2() const {
    return int64_t(x) * x + int64_t(y) * y;
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

mt19937 rng(58);

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
  in.close();
  int n = int(pts.size());
  vector<int> order(n);
  if (test_id >= 24 || test_id == 19) {
    debug("reading order");
    string in2_filename = string("../spaceship/spaceship") + char('0' + test_id / 10) + char('0' + test_id % 10) + "_order.txt";
    ifstream in2(in2_filename);
    int _n;
    in2 >> _n;
    debug(_n, n);
    assert(_n + 1 == n);
    order.clear();
    order.push_back(0);
    for (int i = 0; i < n; i++) {
      int x;
      in2 >> x;
      order.push_back(x + 1);
    }
    in2.close();
  } else {
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

        if (test_id == 23) {
          if (pts[order[it - 1]] == Point{58, -29} && pts[j] != Point{61, -33}) {
            continue;
          }
        }



        if (test_id == 20) {
          if (pts[order[it - 1]] == Point{0, 0} && pts[j] != Point{3, 50}) {
            continue;
          }
          if (pts[order[it - 1]] == Point{38, 4} && pts[j] != Point{220, 0}) {
            continue;
          }
          if (pts[order[it - 1]] == Point{270, 296} && pts[j] != Point{277, 278}) {
            continue;
          }
          if (pts[order[it - 1]] == Point{13, 423} && pts[j] != Point{1, 553}) {
            continue;
          }
        }


        if (test_id == 21) {
          if (pts[order[it - 1]] == Point{2, 247} && pts[j] != Point{84, 415}) {
            continue;
          }
          if (pts[order[it - 1]] == Point{270, 3} && pts[j] != Point{462, 0}) {
            continue;
          }
          if (pts[order[it - 1]] == Point{609, 480} && pts[j] != Point{614, 475}) {
            continue;
          }
          if (pts[order[it - 1]] == Point{917, 6} && pts[j] != Point{999, 368}) {
            continue;
          }
        }

        // [pts[order[i]], pts[order[i]] - pts[order[i - 1]]]: ()
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
    if (test_id == 17) {
      // order = {0, 44, 50, 92, 75, 9, 94, 57, 95, 58, 93, 56, 69, 91, 2, 61, 38, 14, 15, 47, 97, 85, 29, 27, 35, 80, 48, 4, 21, 64, 42, 26, 98, 12, 68, 52, 10, 37, 6, 99, 79, 72, 54, 96, 30, 76, 43, 66, 28, 73, 81, 49, 34, 13, 86, 77, 70, 82, 78, 8, 19, 62, 25, 18, 46, 40, 60, 63, 45, 59, 1, 20, 22, 31, 55, 16, 88, 100, 67, 89, 3, 24, 32, 41, 39, 87, 51, 83, 36, 90, 11, 65, 17, 74, 5, 33, 23, 71, 7, 84, 53};
      // order = {0, 44, 50, 92, 75, 9, 94, 57, 95, 58, 93, 56, 69, 91, 2, 61, 38, 14, 15, 47, 97, 85, 29, 27, 35, 80, 48, 4, 21, 99, 6, 37, 10, 52, 68, 12, 98, 26, 42, 64, 79, 72, 54, 96, 30, 76, 43, 66, 28, 73, 81, 49, 34, 13, 22, 1, 20, 59, 45, 63, 60, 40, 46, 18, 25, 62, 19, 8, 78, 82, 70, 77, 86, 31, 55, 16, 88, 100, 67, 89, 3, 24, 32, 41, 39, 87, 51, 83, 36, 90, 11, 65, 5, 33, 23, 71, 7, 84, 53, 74, 17};
      // order = {0, 44, 50, 92, 75, 9, 94, 57, 95, 58, 93, 56, 69, 91, 2, 61, 38, 14, 15, 47, 97, 85, 29, 27, 35, 80, 48, 4, 99, 6, 37, 10, 52, 68, 12, 98, 26, 42, 64, 21, 79, 72, 54, 96, 30, 76, 43, 66, 28, 73, 81, 49, 34, 13, 22, 1, 20, 59, 45, 63, 60, 40, 46, 18, 25, 62, 19, 8, 78, 82, 88, 16, 55, 31, 86, 77, 70, 100, 67, 89, 3, 24, 32, 41, 39, 87, 51, 83, 36, 90, 11, 65, 5, 33, 23, 71, 7, 84, 53, 74, 17};
      // order = {0, 44, 50, 92, 75, 9, 94, 57, 95, 93, 25, 18, 46, 40, 60, 63, 45, 59, 20, 1, 22, 13, 34, 58, 49, 81, 73, 28, 66, 43, 76, 30, 96, 54, 72, 79, 21, 64, 42, 26, 98, 12, 68, 52, 10, 37, 6, 99, 4, 48, 80, 35, 27, 29, 85, 97, 47, 15, 14, 38, 61, 2, 91, 69, 56, 62, 19, 8, 78, 82, 88, 16, 55, 31, 86, 77, 70, 100, 67, 89, 3, 24, 32, 41, 39, 87, 51, 83, 36, 90, 11, 65, 5, 33, 23, 71, 7, 84, 53, 74, 17};
      // order = {0, 44, 50, 91, 2, 61, 38, 14, 15, 47, 97, 85, 29, 27, 35, 80, 48, 4, 79, 99, 6, 37, 10, 52, 68, 12, 98, 26, 42, 64, 21, 72, 54, 96, 30, 76, 43, 66, 28, 62, 93, 95, 57, 94, 9, 75, 92, 69, 56, 25, 18, 46, 40, 60, 63, 45, 59, 20, 1, 22, 13, 34, 58, 49, 81, 73, 19, 8, 82, 88, 16, 55, 31, 86, 77, 70, 78, 100, 67, 89, 3, 24, 32, 41, 39, 87, 51, 83, 36, 90, 11, 65, 5, 33, 23, 71, 7, 84, 53, 74, 17};
      // order = {0, 44, 50, 91, 2, 61, 57, 38, 14, 15, 47, 97, 26, 42, 64, 21, 79, 72, 54, 96, 30, 76, 43, 66, 28, 62, 93, 95, 94, 9, 75, 92, 69, 56, 25, 18, 46, 40, 60, 63, 45, 59, 31, 55, 16, 88, 8, 19, 73, 81, 49, 58, 34, 13, 22, 20, 1, 86, 77, 70, 78, 82, 100, 67, 89, 3, 24, 32, 41, 39, 87, 51, 83, 36, 90, 11, 65, 5, 33, 23, 71, 7, 84, 53, 74, 17, 37, 10, 52, 68, 12, 98, 85, 29, 27, 35, 80, 48, 4, 99, 6};
      order = {0, 44, 50, 91, 2, 61, 57, 38, 15, 37, 17, 74, 53, 84, 7, 71, 23, 33, 5, 65, 11, 90, 36, 83, 52, 51, 87, 39, 41, 32, 24, 3, 89, 67, 100, 82, 78, 70, 77, 86, 1, 20, 22, 13, 34, 58, 49, 81, 73, 25, 18, 46, 40, 60, 63, 45, 59, 31, 55, 16, 88, 8, 19, 56, 69, 92, 10, 47, 97, 26, 42, 64, 21, 79, 72, 54, 96, 30, 76, 43, 66, 28, 62, 93, 95, 94, 9, 75, 14, 68, 12, 98, 85, 29, 27, 35, 80, 48, 4, 99, 6};
    }
    if (test_id == 16) {
      order = {0, 1, 79, 364, 102, 3, 170, 406, 346, 162, 218, 323, 293, 402, 141, 8, 161, 30, 483, 159, 414, 416, 370, 69, 213, 166, 156, 152, 35, 32, 417, 446, 488, 188, 196, 240, 107, 193, 442, 303, 12, 440, 158, 26, 463, 228, 275, 180, 114, 380, 343, 194, 187, 257, 363, 6, 112, 487, 485, 64, 254, 301, 472, 441, 47, 420, 117, 309, 451, 369, 129, 259, 273, 143, 311, 315, 96, 307, 486, 413, 328, 167, 53, 286, 267, 484, 109, 115, 140, 148, 269, 467, 230, 163, 203, 349, 396, 181, 178, 50, 492, 383, 434, 340, 43, 236, 169, 280, 242, 20, 232, 373, 55, 399, 482, 264, 266, 418, 219, 212, 291, 31, 326, 118, 209, 305, 97, 19, 135, 462, 271, 94, 214, 268, 233, 375, 253, 292, 190, 355, 199, 120, 72, 145, 436, 229, 89, 270, 221, 131, 306, 171, 258, 227, 358, 75, 201, 216, 421, 255, 211, 21, 149, 341, 84, 265, 77, 177, 387, 256, 136, 411, 324, 336, 36, 289, 404, 466, 371, 70, 393, 66, 450, 68, 38, 61, 263, 302, 469, 334, 83, 425, 46, 429, 185, 238, 458, 308, 137, 353, 7, 128, 85, 106, 16, 321, 313, 88, 40, 287, 374, 175, 9, 250, 86, 246, 408, 160, 200, 174, 389, 81, 329, 331, 352, 92, 460, 426, 168, 179, 14, 454, 184, 51, 410, 134, 395, 121, 491, 350, 231, 427, 87, 225, 332, 424, 215, 461, 277, 29, 48, 165, 428, 448, 405, 477, 475, 76, 103, 359, 65, 409, 24, 191, 470, 192, 319, 93, 59, 297, 422, 322, 251, 22, 435, 283, 58, 153, 226, 310, 105, 282, 173, 423, 73, 249, 147, 345, 186, 449, 11, 464, 471, 220, 298, 401, 206, 138, 384, 123, 248, 176, 354, 385, 126, 82, 142, 433, 327, 260, 379, 41, 473, 288, 381, 223, 182, 100, 335, 431, 44, 312, 18, 42, 74, 91, 172, 456, 320, 274, 316, 361, 224, 90, 234, 407, 465, 261, 398, 382, 443, 481, 204, 124, 344, 337, 357, 262, 318, 127, 67, 437, 37, 325, 28, 198, 95, 125, 132, 133, 239, 113, 78, 390, 202, 493, 62, 290, 453, 247, 195, 130, 439, 80, 210, 304, 25, 183, 348, 372, 278, 144, 392, 330, 362, 244, 119, 296, 342, 397, 386, 151, 412, 237, 347, 54, 489, 299, 27, 17, 197, 368, 365, 388, 480, 39, 98, 108, 33, 252, 49, 217, 241, 4, 45, 438, 338, 154, 60, 207, 284, 99, 400, 403, 139, 314, 164, 479, 476, 272, 245, 235, 447, 189, 205, 366, 394, 13, 279, 111, 445, 122, 444, 356, 415, 333, 430, 116, 367, 295, 57, 71, 276, 10, 101, 391, 455, 281, 478, 222, 360, 452, 317, 52, 300, 376, 146, 468, 208, 294, 474, 34, 285, 5, 110, 2, 339, 155, 419, 490, 150, 56, 459, 104, 457, 351, 63, 15, 243, 157, 23, 432, 377, 378, 494, 495, 496, 497};
    }
    if (test_id == 22) {
      // order = {0, 1, 107, 189, 179, 138, 57, 91, 250, 48, 274, 56, 165, 170, 120, 256, 178, 241, 207, 7, 177, 140, 110, 95, 51, 41, 169, 188, 282, 37, 192, 173, 224, 10, 161, 74, 50, 160, 76, 264, 5, 293, 214, 270, 245, 6, 59, 62, 248, 24, 190, 261, 99, 268, 102, 52, 210, 133, 67, 150, 213, 103, 17, 168, 83, 29, 209, 42, 247, 220, 38, 219, 13, 198, 30, 101, 114, 285, 243, 158, 146, 39, 204, 65, 21, 272, 152, 206, 54, 143, 240, 53, 25, 217, 111, 121, 208, 291, 238, 283, 237, 134, 287, 126, 97, 199, 86, 45, 32, 80, 193, 125, 90, 92, 93, 174, 281, 64, 112, 72, 267, 196, 155, 141, 4, 255, 222, 172, 221, 265, 166, 218, 26, 186, 49, 164, 294, 36, 242, 187, 277, 23, 201, 202, 28, 249, 275, 154, 87, 162, 260, 288, 33, 142, 163, 136, 58, 223, 147, 175, 148, 195, 296, 88, 156, 109, 232, 233, 27, 63, 73, 31, 212, 231, 100, 197, 194, 153, 271, 14, 144, 234, 98, 55, 259, 15, 263, 123, 128, 12, 157, 81, 61, 16, 34, 211, 104, 70, 145, 203, 40, 159, 205, 251, 75, 82, 117, 216, 289, 181, 182, 185, 22, 35, 79, 244, 280, 2, 295, 246, 44, 236, 228, 71, 226, 77, 279, 180, 108, 269, 130, 115, 149, 286, 137, 235, 20, 227, 266, 252, 106, 151, 215, 254, 276, 96, 46, 118, 131, 284, 273, 84, 113, 18, 292, 94, 116, 89, 124, 43, 262, 253, 139, 230, 183, 239, 191, 60, 176, 132, 229, 66, 127, 184, 3, 9, 225, 78, 129, 68, 171, 105, 119, 85, 8, 122, 135, 290, 11, 167, 278, 47, 19, 200, 69, 258, 257, 297, 298, 299, 300};
      order = {0, 1, 107, 189, 179, 138, 57, 91, 250, 48, 274, 56, 165, 170, 120, 256, 178, 241, 207, 7, 177, 140, 110, 95, 51, 41, 169, 188, 282, 37, 192, 173, 224, 10, 161, 74, 50, 160, 76, 264, 5, 293, 214, 270, 245, 6, 59, 62, 248, 24, 190, 261, 99, 268, 102, 52, 210, 133, 67, 150, 213, 103, 17, 168, 83, 29, 209, 42, 247, 220, 38, 219, 13, 198, 30, 101, 114, 285, 243, 158, 146, 39, 204, 65, 21, 272, 152, 206, 54, 143, 240, 53, 25, 217, 111, 121, 208, 291, 238, 283, 237, 134, 287, 126, 97, 199, 86, 45, 32, 80, 193, 125, 90, 92, 93, 174, 281, 64, 112, 72, 267, 196, 155, 141, 4, 255, 222, 172, 221, 265, 166, 218, 26, 186, 49, 164, 294, 36, 242, 187, 277, 23, 201, 202, 28, 249, 275, 154, 87, 162, 260, 288, 33, 142, 163, 136, 58, 223, 147, 175, 148, 195, 296, 88, 156, 109, 232, 233, 27, 63, 73, 31, 212, 231, 100, 197, 194, 153, 271, 14, 144, 234, 98, 55, 259, 15, 263, 123, 128, 12, 157, 81, 61, 16, 34, 211, 104, 70, 145, 203, 40, 159, 205, 251, 75, 82, 117, 216, 289, 181, 182, 185, 22, 35, 79, 244, 280, 2, 295, 246, 44, 236, 228, 71, 226, 77, 279, 180, 108, 269, 130, 115, 149, 286, 137, 235, 20, 227, 266, 252, 106, 151, 215, 254, 276, 96, 46, 118, 131, 284, 273, 84, 113, 18, 292, 94, 116, 89, 124, 43, 262, 253, 139, 230, 183, 239, 191, 60, 176, 132, 229, 66, 127, 184, 3, 9, 225, 78, 129, 68, 171, 105, 119, 85, 8, 122, 135, 290, 11, 167, 278, 47, 299, 19, 200, 300, 69, 258, 257, 297, 298};
      reverse(order.begin() + 1, order.end());
      // order = {0, 298, 297, 257, 258, 300, 69, 200, 19, 299, 84, 113, 18, 292, 94, 116, 89, 124, 43, 262, 253, 139, 230, 183, 239, 191, 60, 176, 132, 229, 66, 127, 184, 3, 9, 225, 129, 68, 171, 105, 119, 85, 8, 122, 135, 290, 11, 167, 278, 47, 273, 284, 131, 118, 46, 96, 276, 254, 215, 151, 106, 252, 266, 227, 20, 235, 137, 286, 149, 115, 130, 269, 108, 180, 279, 77, 226, 71, 228, 236, 44, 246, 295, 2, 280, 244, 79, 35, 22, 185, 182, 181, 289, 216, 117, 82, 75, 251, 205, 159, 40, 203, 145, 70, 104, 211, 34, 16, 61, 81, 157, 12, 128, 123, 263, 15, 259, 55, 98, 234, 144, 14, 271, 153, 194, 197, 100, 231, 212, 31, 73, 63, 27, 233, 232, 109, 156, 88, 296, 195, 148, 175, 147, 223, 58, 136, 163, 142, 33, 288, 260, 162, 87, 154, 275, 28, 249, 202, 201, 23, 78, 277, 187, 242, 36, 294, 164, 49, 186, 26, 218, 166, 265, 221, 172, 222, 255, 4, 141, 155, 196, 267, 72, 112, 64, 281, 174, 93, 92, 90, 125, 193, 80, 32, 45, 86, 199, 97, 126, 287, 134, 237, 283, 238, 291, 208, 121, 111, 217, 25, 53, 240, 143, 54, 206, 152, 272, 21, 65, 204, 29, 209, 42, 247, 220, 38, 219, 13, 198, 30, 101, 114, 285, 243, 158, 146, 39, 83, 168, 17, 103, 213, 150, 67, 133, 210, 52, 102, 268, 99, 261, 190, 24, 248, 62, 59, 6, 245, 270, 214, 293, 5, 264, 76, 160, 50, 74, 161, 10, 224, 173, 192, 37, 282, 188, 169, 41, 51, 95, 110, 140, 177, 7, 207, 241, 178, 256, 120, 170, 165, 56, 274, 48, 250, 91, 57, 138, 179, 189, 107, 1};
      // order = {0, 290, 298, 297, 257, 258, 300, 69, 200, 299, 19, 84, 113, 242, 36, 294, 164, 49, 186, 26, 218, 166, 265, 106, 252, 266, 227, 20, 235, 137, 286, 115, 130, 269, 108, 180, 279, 77, 226, 71, 228, 236, 44, 246, 295, 2, 280, 244, 79, 35, 22, 185, 182, 181, 289, 216, 117, 82, 70, 145, 203, 40, 159, 205, 251, 75, 104, 211, 34, 16, 61, 81, 157, 12, 128, 123, 263, 15, 259, 55, 234, 144, 14, 271, 153, 194, 197, 100, 231, 212, 31, 73, 63, 27, 233, 232, 1, 107, 189, 179, 138, 57, 91, 250, 48, 274, 56, 165, 170, 120, 256, 178, 241, 207, 7, 177, 140, 110, 95, 51, 41, 169, 188, 282, 37, 192, 173, 224, 10, 161, 74, 50, 160, 76, 264, 5, 293, 214, 270, 245, 6, 59, 62, 248, 24, 190, 261, 99, 268, 102, 52, 210, 133, 67, 150, 213, 103, 17, 168, 83, 29, 209, 42, 247, 220, 38, 219, 13, 198, 30, 101, 114, 285, 243, 158, 146, 39, 204, 65, 21, 272, 152, 206, 54, 143, 240, 53, 25, 217, 111, 121, 208, 291, 238, 283, 237, 134, 287, 126, 97, 199, 86, 45, 32, 80, 193, 125, 90, 92, 93, 174, 64, 112, 72, 149, 267, 196, 155, 141, 4, 255, 222, 172, 221, 151, 215, 254, 276, 96, 46, 118, 131, 284, 273, 47, 124, 43, 262, 253, 139, 230, 183, 239, 191, 60, 176, 132, 229, 66, 127, 184, 3, 9, 225, 129, 68, 171, 105, 119, 85, 8, 122, 135, 11, 167, 278, 89, 116, 94, 292, 18, 187, 277, 78, 23, 201, 202, 28, 249, 275, 154, 87, 162, 260, 288, 33, 142, 281, 163, 136, 58, 223, 147, 175, 148, 195, 296, 88, 156, 109, 98};
      // order = {0, 298, 297, 257, 258, 300, 69, 200, 299, 19, 84, 113, 242, 36, 294, 164, 49, 186, 26, 218, 166, 265, 106, 252, 266, 227, 20, 235, 137, 286, 115, 130, 269, 108, 180, 279, 77, 226, 71, 228, 236, 44, 246, 295, 2, 280, 244, 79, 35, 22, 185, 182, 181, 289, 216, 117, 82, 70, 145, 203, 40, 159, 205, 251, 75, 104, 211, 34, 16, 61, 81, 157, 12, 128, 123, 263, 15, 259, 55, 234, 144, 14, 271, 153, 194, 197, 100, 231, 212, 31, 73, 63, 27, 233, 232, 1, 107, 189, 179, 138, 57, 91, 250, 48, 274, 56, 165, 170, 120, 256, 178, 241, 207, 7, 177, 140, 110, 95, 51, 41, 169, 188, 282, 37, 192, 173, 224, 10, 161, 74, 50, 160, 76, 264, 5, 293, 214, 270, 245, 6, 59, 62, 248, 24, 190, 261, 99, 268, 102, 52, 210, 133, 67, 150, 213, 103, 17, 168, 83, 29, 209, 42, 247, 220, 38, 219, 13, 198, 30, 101, 114, 285, 243, 158, 146, 39, 204, 65, 21, 272, 152, 206, 54, 143, 240, 53, 25, 217, 111, 121, 208, 291, 238, 283, 237, 134, 287, 126, 97, 199, 86, 45, 32, 80, 193, 125, 90, 92, 93, 174, 64, 112, 72, 149, 267, 196, 155, 141, 4, 255, 222, 172, 221, 151, 215, 254, 276, 96, 46, 118, 131, 284, 273, 47, 124, 43, 262, 253, 139, 230, 183, 239, 191, 60, 176, 132, 229, 66, 127, 184, 3, 9, 225, 129, 68, 171, 105, 119, 85, 8, 122, 135, 290, 11, 167, 278, 89, 116, 94, 292, 18, 187, 277, 78, 23, 201, 202, 28, 249, 275, 154, 87, 162, 260, 288, 33, 142, 281, 163, 136, 58, 223, 147, 175, 148, 195, 296, 88, 156, 109, 98};
      // order = {0, 298, 297, 257, 258, 300, 69, 200, 299, 19, 84, 113, 242, 36, 294, 164, 49, 186, 26, 218, 166, 265, 106, 252, 266, 227, 20, 235, 137, 286, 115, 130, 269, 108, 180, 279, 77, 226, 71, 228, 236, 44, 246, 295, 2, 280, 244, 79, 35, 22, 185, 182, 181, 289, 216, 117, 82, 70, 145, 203, 40, 159, 205, 251, 75, 104, 211, 34, 16, 61, 81, 157, 12, 128, 123, 263, 15, 259, 55, 234, 144, 14, 271, 153, 194, 197, 100, 231, 212, 31, 73, 63, 27, 233, 232, 1, 107, 189, 179, 138, 57, 91, 250, 48, 274, 56, 165, 170, 120, 256, 178, 241, 207, 7, 177, 140, 110, 95, 51, 41, 169, 188, 282, 37, 192, 173, 224, 10, 161, 74, 50, 160, 76, 264, 5, 293, 214, 270, 245, 6, 59, 62, 248, 24, 190, 261, 99, 268, 102, 52, 210, 133, 67, 150, 213, 103, 17, 168, 83, 29, 209, 42, 247, 220, 38, 219, 198, 13, 30, 101, 114, 285, 243, 158, 146, 39, 204, 65, 21, 272, 152, 206, 54, 143, 240, 53, 25, 217, 111, 121, 208, 291, 238, 283, 237, 134, 287, 126, 97, 199, 86, 45, 32, 80, 193, 125, 90, 92, 93, 174, 64, 112, 72, 149, 267, 196, 155, 141, 4, 255, 222, 172, 221, 151, 215, 254, 276, 96, 46, 118, 131, 284, 273, 47, 124, 43, 262, 253, 139, 230, 183, 239, 191, 60, 176, 132, 229, 66, 127, 184, 3, 9, 225, 129, 68, 171, 105, 119, 85, 8, 122, 135, 290, 11, 167, 278, 89, 116, 94, 292, 18, 187, 277, 78, 23, 201, 202, 28, 249, 275, 154, 87, 162, 260, 288, 33, 142, 281, 163, 136, 58, 223, 147, 175, 148, 195, 296, 88, 156, 109, 98};
      order = {0, 166, 218, 26, 186, 258, 69, 49, 164, 294, 242, 113, 19, 299, 200, 300, 257, 297, 265, 298, 106, 252, 266, 227, 20, 235, 137, 286, 115, 130, 269, 108, 180, 279, 77, 226, 71, 228, 236, 44, 246, 295, 2, 280, 244, 79, 35, 22, 185, 182, 181, 289, 216, 117, 82, 70, 145, 203, 40, 159, 205, 251, 75, 104, 211, 34, 16, 61, 81, 157, 12, 128, 123, 263, 15, 259, 55, 234, 144, 14, 271, 153, 194, 197, 100, 231, 212, 31, 73, 63, 27, 233, 232, 1, 107, 189, 179, 138, 57, 91, 250, 48, 274, 56, 165, 170, 120, 256, 178, 241, 207, 7, 177, 140, 110, 95, 51, 41, 169, 188, 282, 37, 192, 173, 224, 10, 161, 74, 50, 160, 76, 264, 5, 293, 214, 270, 245, 6, 59, 62, 248, 24, 190, 261, 99, 268, 102, 52, 210, 133, 67, 150, 213, 103, 17, 168, 83, 29, 209, 42, 247, 220, 38, 219, 198, 13, 30, 101, 114, 285, 243, 158, 146, 39, 204, 65, 21, 272, 152, 206, 54, 143, 240, 53, 25, 217, 111, 121, 208, 291, 238, 283, 237, 134, 287, 126, 97, 199, 86, 45, 32, 80, 193, 125, 90, 92, 93, 174, 64, 112, 72, 149, 267, 196, 155, 141, 4, 255, 222, 172, 221, 151, 215, 254, 276, 96, 46, 118, 131, 284, 273, 47, 36, 124, 43, 262, 253, 139, 230, 183, 239, 191, 60, 176, 132, 229, 66, 127, 184, 3, 9, 225, 84, 129, 68, 171, 105, 119, 85, 8, 122, 135, 290, 11, 167, 278, 89, 116, 94, 292, 18, 187, 277, 78, 23, 201, 202, 28, 249, 275, 154, 87, 162, 260, 288, 33, 142, 281, 163, 136, 58, 223, 147, 175, 148, 195, 296, 88, 156, 109, 98};
    }
    if (test_id == 18) {
      order = {0, 37, 25, 99, 49, 22, 94, 6, 43, 47, 81, 86, 9, 26, 89, 32, 96, 98, 5, 83, 4, 10, 91, 42, 85, 57, 2, 79, 56, 23, 65, 15, 74, 66, 13, 41, 76, 75, 27, 72, 87, 19, 64, 51, 54, 70, 29, 7, 52, 68, 93, 82, 1, 92, 40, 44, 80, 12, 17, 39, 78, 71, 30, 45, 20, 62, 58, 35, 18, 38, 11, 34, 8, 50, 55, 14, 31, 24, 61, 77, 84, 73, 46, 100, 33, 36, 97, 21, 63, 48, 60, 16, 69, 90, 53, 95, 28, 59, 67, 3, 88};
    }
    debug("found order", clock());
  }
  for (int i = 1; i < n; i++) {
    // debug(pts[order[i]], pts[order[i]] - pts[order[i - 1]]);
  }
  // n = min(n, 10000);
  // return 0;
  const int M = 500;
  const int inf = int(1e8);
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
  auto Eval = [&](Point a, Point b, Point sa, Point sb, int MM) {
    Point delta = sb - sa;
    for (int k = max(abs(delta.x), abs(delta.y)); k < MM; k++) {
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
    mt19937 gnr(60);
    const int LIM = 100;
    const int B = 20;
    const int D = 50;
    const int BUCKET_SIZE = 10;
    const int SKIP = 10;
    // vector dp(n, vector(2 * LIM + 1, vector<int>(2 * LIM + 1, inf)));
    // vector pr(n, vector(2 * LIM + 1, vector<Point>(2 * LIM + 1, {-1, -1})));
    // dp[0][LIM][LIM] = 0;
    vector<vector<array<int, 5>>> bests(n);
    bests[0].push_back({0, 0, 0, -1, -1});
    int last_best = 0;
    for (int i = 0; i < n - 1; i++) {
      if (test_id == 23 && i >= 26854 && i <= 26857) {
        for (auto& bb : bests[i])
          debug(i, pts[order[i]], bb);
      }
      if (i % 100 == 0 || (test_id == 23 && i > 26800 && i < 26900)) {
        int delta = bests[i][0][0] - last_best;
        last_best = bests[i][0][0];
        double R = 1.0 / max(1, i) * n * bests[i][0][0];
        auto delta_p = pts[order[i + 1]] - pts[order[i]];
        // debug(i, delta_p, clock(), bests[i].size(), bests[i][0], delta, R);
      }
      vector dp(2 * LIM + 1, vector<int>(2 * LIM + 1, inf));
      vector pr(2 * LIM + 1, vector<Point>(2 * LIM + 1, {-1, -1}));
      vector<array<int, 6>> cands;
      for (int id = 0; id < int(bests[i].size()); id++) {
        auto [ft, sx, sy, px, py] = bests[i][id];
        Point sa = {sx, sy};
        for (int nx = max(-LIM, sx - D); nx <= min(LIM, sx + D); nx += SKIP) {
          for (int ny = max(-LIM, sy - D); ny <= min(LIM, sy + D); ny += SKIP) {
            Point sb = {nx, ny};
            int param = (test_id == 23 ? (i == 26855 || (i >= 27300 && i <= 27700) || (i >= 31600 && i <= 31700) ? M : 7) : M);
            auto val = ft + Eval(pts[order[i]], pts[order[i + 1]], sa, sb, param);
            int& to = dp[nx + LIM][ny + LIM];
            if (val < to) {
              to = val;
              pr[nx + LIM][ny + LIM] = sa;
            }
          }
        }
      }
      vector best_in_bucket(2 * LIM / BUCKET_SIZE + 1, vector<array<int, 6>>(2 * LIM / BUCKET_SIZE + 1, {inf}));
      for (int sx = -LIM; sx <= LIM; sx++) {
        for (int sy = -LIM; sy <= LIM; sy++) {
          auto ft = dp[sx + LIM][sy + LIM];
          if (ft == inf) {
            continue;
          }
          auto pp = pr[sx + LIM][sy + LIM];
          array<int, 6> me = {ft, int(gnr()), sx, sy, pp.x, pp.y};
          auto& to = best_in_bucket[(sx + LIM) / BUCKET_SIZE][(sy + LIM) / BUCKET_SIZE];
          to = min(to, me);
        }
      }
      for (auto& a : best_in_bucket) {
        for (auto& b : a) {
          if (b[0] < inf) {
            cands.push_back(b);
          }
        }
      }
      if (cands.empty()) {
        debug("FAIL!", i);
        assert(false);
      }
      sort(cands.begin(), cands.end());
      if (int(cands.size()) > B) {
        cands.resize(B);
      }
      bests[i + 1].resize(cands.size());
      for (int j = 0; j < int(cands.size()); j++) {
        auto& t = cands[j];
        bests[i + 1][j] = {t[0], t[2], t[3], t[4], t[5]};
      }
    }
    auto [best, bx, by, px, py] = bests[n - 1][0];
    // int best = bests[n - 1][0][0];
    // int bx = -1, by = -1;
    // for (int sx = -LIM; sx <= LIM; sx++) {
    //   for (int sy = -LIM; sy <= LIM; sy++) {
    //     auto ft = dp[n - 1][sx + LIM][sy + LIM];
    //     if (ft < best) {
    //       best = ft;
    //       bx = sx;
    //       by = sy;
    //     }
    //   }
    // }
    debug(test_id, clock(), best);
    assert(best < inf);
    for (int i = n - 1; i > 0; i--) {
      speeds[order[i]] = {bx, by};
      bool found = false;
      for (auto [_, qx, qy, vx, vy] : bests[i - 1]) {
        if (qx == px && qy == py) {
          bx = qx;
          by = qy;
          px = vx;
          py = vy;
          found = true;
          break;
        }
      }
      if (!found) {
        debug("not found", i);
        assert(found);
      }
      // auto from = pr[i][bx + LIM][by + LIM];
      // bx = from.x;
      // by = from.y;
    }
    return best;
  };
  int score = DP();
  uniform_real_distribution<double> urd(0, 1);
  double init_temp = 1e-6;
  double final_temp = 1e-6;
  double cur_temp = init_temp;
  int best_score = score;
  auto best_order = order;
  auto best_speeds = speeds;
  const double TL = 200.0;
  const int NEAR = 1000000;
  int it = 0;
  while (1.0 * clock() / CLOCKS_PER_SEC < TL) {
    if (it % 100 == 0) {
      auto t = 1.0 * clock() / CLOCKS_PER_SEC;
      cur_temp = init_temp * pow(final_temp / init_temp, t / TL);
      debug(test_id, it, t, cur_temp, score, best_score);
    }
    it += 1;
    if (it % 2 == 0) {
      int i = rng() % (n - 1) + 1; //rng() % (n - 1) + 1;
      int j;
      do {
        j = rng() % (n - 1) + 1;
      } while (i == j);// || abs(i - j) > 5);
      int me = order[i];
      order.erase(order.begin() + i);
      order.insert(order.begin() + j, me);
      if ((pts[order[j]] - pts[order[j - 1]]).abs2() > NEAR) {
        order.erase(order.begin() + j);
        order.insert(order.begin() + i, me);
        continue;
      }
      if (j < n - 1 && (pts[order[j]] - pts[order[j + 1]]).abs2() > NEAR) {
        order.erase(order.begin() + j);
        order.insert(order.begin() + i, me);
        continue;
      }
      int delta = DP();
      delta -= score;
      if (delta <= 0 || (delta > 0 && urd(rng) < exp(-1.0 * delta / cur_temp))) {
        score += delta;
      } else {
        order.erase(order.begin() + j);
        order.insert(order.begin() + i, me);
      }
    } else {
      int i, j;
      do {
        i = rng() % (n - 1) + 1;//(n - 1) + 1;
        j = rng() % (n - 1) + 1;
      } while (i >= j);// || abs(i - j) > 5);
      if ((pts[order[j]] - pts[order[i - 1]]).abs2() > NEAR) {
        continue;
      }
      if (j < n - 1 && (pts[order[i]] - pts[order[j + 1]]).abs2() > NEAR) {
        continue;
      }
      reverse(order.begin() + i, order.begin() + j + 1);
      int delta = DP();
      delta -= score;
      if (delta <= 0 || (delta > 0 && urd(rng) < exp(-1.0 * delta / cur_temp))) {
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
  // debug(speeds);
  string res = "";
  for (int i = 1; i < n; i++) {
    Point a = pts[order[i - 1]];
    Point b = pts[order[i]];
    Point sa = speeds[order[i - 1]];
    Point sb = speeds[order[i]];
    int k = Eval(a, b, sa, sb, M);
    for (int j = k - 1; j >= 0; j--) {
      bool found = false;
      for (int dx = -1; dx <= 1; dx++) {
        for (int dy = -1; dy <= 1; dy++) {
          if (!found && Eval(a + sa + Point(dx, dy), b, sa + Point(dx, dy), sb, M) == j) {
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