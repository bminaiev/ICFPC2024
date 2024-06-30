#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
#include <limits>
#include <queue>
#include <unordered_map>

using namespace std;

const int MAX_COORD = 101000;
const int MAX_V = 1500;
const int INF = numeric_limits<int>::max();
const int BEAM_WIDTH = 350;
const int MAX_STEPS = 1000000;  // Add a maximum step limit

struct Point {
    int x, y;
    Point(int x = 0, int y = 0) : x(x), y(y) {}
};

struct State {
    int point, x, y, vx, vy, steps, d;
    State(int p = 0, int x = 0, int y = 0, int vx = 0, int vy = 0, int s = 0, int dd = 0) 
        : point(p), x(x), y(y), vx(vx), vy(vy), steps(s), d(dd) {}

    bool operator<(const State& other) const {
        return point < other.point || (point == other.point && d > other.d); // For min-heap
    }
};

vector<Point> points;
unordered_map<long long, int> dp;
unordered_map<long long, State> prv;

int dist(const Point& a, const Point& b) {
    return abs(a.x - b.x) + abs(a.y - b.y);
}

void sortPoints() {
    points.insert(points.begin(), Point(0, 0));
    for (int i = 1; i < points.size(); ++i) {
        int bestj = i;
        for (int j = i + 1; j < points.size(); ++j) {
            if (dist(points[i-1], points[j]) < dist(points[i-1], points[bestj])) {
                bestj = j;
            }
        }
        swap(points[i], points[bestj]);
    }
}

long long encodeState(const State& s) {
    return ((long long)s.point << 48) | ((long long)(s.x + MAX_COORD) << 36) | 
           ((long long)(s.y + MAX_COORD) << 24) | ((s.vx + MAX_V) << 12) | (s.vy + MAX_V);
}

void beamSearch() {
    int n = points.size();
    priority_queue<State> pq;
    pq.push(State(0, 0, 0, 0, 0, 0));
    dp[encodeState(State(0, 0, 0, 0, 0))] = 0;
    
    while (!pq.empty()) {
        if (pq.size())
            cerr << pq.top().point << endl;
        vector<State> beam;
        for (int i = 0; i < BEAM_WIDTH && !pq.empty(); ++i) {
            beam.push_back(pq.top());
            pq.pop();
        }
        pq = priority_queue<State>();
        
        for (const auto& cur : beam) {
            if (cur.point == n - 1 && cur.x == points[n-1].x && cur.y == points[n-1].y) goto end;
            if (cur.steps >= MAX_STEPS) continue;  // Add step limit check
            
            for (int dx = -1; dx <= 1; ++dx) {
                for (int dy = -1; dy <= 1; ++dy) {
                    int nvx = cur.vx + dx, nvy = cur.vy + dy;
                    int nx = cur.x + nvx, ny = cur.y + nvy;
                    int ni = cur.point;
                    if (ni < n - 1 && nx == points[ni+1].x && ny == points[ni+1].y) ni++;
                    
                    if (abs(nvx) <= MAX_V && abs(nvy) <= MAX_V &&
                        abs(nx) <= MAX_COORD && abs(ny) <= MAX_COORD) {
                        int nd = max(abs(nx - points[ni].x), abs(ny - points[ni].y));
                        State next(ni, nx, ny, nvx, nvy, cur.steps + 1, nd);
                        // cerr << cur.x << " " << cur.y << " " << cur.vx << " " << cur.vy << " -> "
                        //      << "(" << ni << ") " << nx << " " << ny << " " << nvx << " " << nvy << endl;
                        long long encoded = encodeState(next);
                        // cerr << "enc " << encoded << endl;
                        if (dp.find(encoded) == dp.end() || dp[encoded] > next.steps) {
                            dp[encoded] = next.steps;
                            prv[encoded] = cur;
                            pq.push(next);
                            // cerr << "push\n";
                        }
                    }
                }
            }
        }
    }

    end:;
}

string getPath() {
    int n = points.size();
    string path;
    State cur(n-1, points[n-1].x, points[n-1].y, 0, 0);
    int min_steps = INF;
    
    for (int vx = -MAX_V; vx <= MAX_V; ++vx) {
        for (int vy = -MAX_V; vy <= MAX_V; ++vy) {
            long long encoded = encodeState(State(n-1, points[n-1].x, points[n-1].y, vx, vy));
            if (dp.find(encoded) != dp.end() && dp[encoded] < min_steps) {
                min_steps = dp[encoded];
                cur.vx = vx;
                cur.vy = vy;
            }
        }
    }
    
    if (min_steps == INF) {
        return "No solution found";
    }
    
    while (cur.point != 0 || cur.x != 0 || cur.y != 0 || cur.vx != 0 || cur.vy != 0) {
        long long encoded = encodeState(cur);
        if (prv.find(encoded) == prv.end()) {
            return "Path reconstruction failed";
        }
        State prv_state = prv[encoded];
        int dx = cur.vx - prv_state.vx;
        int dy = cur.vy - prv_state.vy;
        // cerr << dx << " " << dy << endl;
        path += to_string(3 * (dy + 1) + (dx + 1) + 1);
        // cerr << path << endl;
        cur = prv_state;
    }
    
    reverse(path.begin(), path.end());
    return path;
}

int main() {
    cerr << "!!\n";
    int x, y;
    while (cin >> x >> y) {
        points.emplace_back(x, y);
    }
    
    cerr << "a\n";
    sortPoints();
    cerr << "b\n";
    beamSearch();
    string path = getPath();
    
    cout << path << endl;
    
    return 0;
}