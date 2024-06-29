#include <iostream>
#include <ctime>
#include <map>
#include <set>
#include <vector>
#include <fstream>
#include <string>
#include <cmath>
#include <sys/time.h>

using namespace std;

double get_time() {timeval tv; gettimeofday(&tv, NULL); return tv.tv_sec + tv.tv_usec * 1e-6;}
double start_time = get_time();
double elapsed() {return get_time() - start_time;}

struct Point {
    int x, y;
};

long long distance(int x1, int y1, int x2, int y2) {
    return (long long)(x2 - x1) * (x2 - x1) + (long long)(y2 - y1) * (y2 - y1);
}

int kmin = 1;

#define INLINE   inline __attribute__ ((always_inline))
#define FOR(i,a,b)  for(int i=(a);i<(b);++i)
#define REP(i,a)    FOR(i,0,a)

struct RNG {
    unsigned int MT[624];
    int index;
    RNG(int seed = 1) {init(seed);}
    void init(int seed = 1) {MT[0] = seed; FOR(i, 1, 624) MT[i] = (1812433253UL * (MT[i-1] ^ (MT[i-1] >> 30)) + i); index = 0; }
    void generate() {
        const unsigned int MULT[] = {0, 2567483615UL};
        REP(i, 227) {unsigned int y = (MT[i] & 0x8000000UL) + (MT[i+1] & 0x7FFFFFFFUL); MT[i] = MT[i+397] ^ (y >> 1); MT[i] ^= MULT[y&1]; }
        FOR(i, 227, 623) {unsigned int y = (MT[i] & 0x8000000UL) + (MT[i+1] & 0x7FFFFFFFUL); MT[i] = MT[i-227] ^ (y >> 1); MT[i] ^= MULT[y&1]; }
        unsigned int y = (MT[623] & 0x8000000UL) + (MT[0] & 0x7FFFFFFFUL); MT[623] = MT[623-227] ^ (y >> 1); MT[623] ^= MULT[y&1];
    }
    unsigned int rand() { if (index == 0) generate(); unsigned int y = MT[index]; y ^= y >> 11; y ^= y << 7  & 2636928640UL; y ^= y << 15 & 4022730752UL; y ^= y >> 18; index = index == 623 ? 0 : index + 1; return y;}
    INLINE int next() {return rand(); }
    INLINE int next(int x) {return ((long long)rand() * x) >> 32; }
    INLINE int next(int a, int b) {return a + next(b - a); }
    INLINE double next_double() {return (rand() + 0.5) * (1.0 / 4294967296.0); }
    INLINE double next_double(double a, double b) {return a + next_double() * (b - a); }
};
 
static RNG rng;

// Function to parse input file and get target points
vector<Point> readPoints(const string &filename) {
    vector<Point> points;
    ifstream file(filename);
    int x, y;
    while (file >> x >> y) {
        points.push_back({x, y});
    }
    return points;
}

// Function to determine the move direction based on velocity change
char getMove(int dx, int dy) {
    if (dx == -1 && dy == 1) return '7';
    if (dx == 0 && dy == 1) return '8';
    if (dx == 1 && dy == 1) return '9';
    if (dx == -1 && dy == 0) return '4';
    if (dx == 0 && dy == 0) return '5'; // This case should not normally happen
    if (dx == 1 && dy == 0) return '6';
    if (dx == -1 && dy == -1) return '1';
    if (dx == 0 && dy == -1) return '2';
    if (dx == 1 && dy == -1) return '3';
    return '5'; // Default case, should not normally happen
}

int getTurns(int px, int py, int vx, int vy, Point point) {
    for (int K = 1; ; K++) {
        int maxx = px + vx * K + (K * (K + 1) / 2);
        int minx = px + vx * K - (K * (K + 1) / 2);
        int maxy = py + vy * K + (K * (K + 1) / 2);
        int miny = py + vy * K - (K * (K + 1) / 2);

        if (minx <= point.x && point.x <= maxx && miny <= point.y && point.y <= maxy)
            return K;
    }
}

// Function to find a sequence of moves to visit all target points
vector<Point> getInitial(vector<Point> points) {
    string path;
    int vx = 0, vy = 0;
    int px = 0, py = 0;

    vector<Point> res;
    res.reserve(2000000);
    while (!points.empty()) {
        // Find the closest point to the current position
        int closest_index = 0;
        set<pair<long long, int>> ss;
        for (int i = 0; i < points.size(); ++i) {
            long long dist = distance(px, py, points[i].x, points[i].y);
            ss.emplace(dist, i);
            if (ss.size() > kmin) ss.erase(--ss.end());
        }

        long long min_turns = numeric_limits<long long>::max();
        for (const auto& [_, index]: ss) {
            Point np = points[index];
            int t = getTurns(px, py, vx, vy, np);
            // if (closest_index > 0 && rand() % 20 == 0) continue;
            if (t < min_turns) {
                closest_index = index;
                min_turns = t;
            }
        }
        // Get the closest point
        Point point = points[closest_index];
        res.push_back(point);
        points.erase(points.begin() + closest_index);

        int K = 1;
        for (K = 1; ; K++) {
            int maxx = px + vx * K + (K * (K + 1) / 2);
            int minx = px + vx * K - (K * (K + 1) / 2);
            int maxy = py + vy * K + (K * (K + 1) / 2);
            int miny = py + vy * K - (K * (K + 1) / 2);

            if (minx <= point.x && point.x <= maxx && miny <= point.y && point.y <= maxy)
                break;
        }
        // cerr << px << "," << py << " (" << vx << "," << vy << "): to " << point.x << "," << point.y << " K=" << K << endl;

        int overx = point.x - px - vx * K;
        int overy = point.y - py - vy * K;
        // cerr << "over: " << overx << " " << overy << endl;
        int signx = 1, signy = 1;

        if (overx < 0) {
            signx = -1;
            overx = -overx;
        }
        if (overy < 0) {
            signy = -1;
            overy = -overy;
        }

        vector<int> needx, needy;
        for (int step = K; step >= 1; step--) {
            if (overx >= step) {
                needx.push_back(1);
                overx -= step;
            } else {
                needx.push_back(0);
            }

            if (overy >= step) {
                needy.push_back(1);
                overy -= step;
            } else {
                needy.push_back(0);
            }
        }

        for (int si = 1; si <= K; si++) {
            int dx = needx[si - 1] * signx;
            int dy = needy[si - 1] * signy;

            vx += dx;
            vy += dy;
            px += vx;
            py += vy;
            // cerr << px << " " << py << endl;

            // path += getMove(dx, dy);
        }
    }

    return res;
}

pair<int, string> findPath_old(vector<Point> points, bool save=false) {
    string path;
    int vx = 0, vy = 0;
    int px = 0, py = 0;
    int res = 0;

    for (const auto& point : points) {
        int K = 1;
        for (K = 1; ; K++) {
            int maxx = px + vx * K + (K * (K + 1) / 2);
            int minx = px + vx * K - (K * (K + 1) / 2);
            int maxy = py + vy * K + (K * (K + 1) / 2);
            int miny = py + vy * K - (K * (K + 1) / 2);

            if (minx <= point.x && point.x <= maxx && miny <= point.y && point.y <= maxy)
                break;
        }
        // cerr << px << "," << py << " (" << vx << "," << vy << "): to " << point.x << "," << point.y << " K=" << K << endl;

        int overx = point.x - px - vx * K;
        int overy = point.y - py - vy * K;
        // cerr << "over: " << overx << " " << overy << endl;
        int signx = 1, signy = 1;

        if (overx < 0) {
            signx = -1;
            overx = -overx;
        }
        if (overy < 0) {
            signy = -1;
            overy = -overy;
        }

        vector<int> needx, needy;
        for (int step = K; step >= 1; step--) {
            if (overx >= step) {
                needx.push_back(1);
                overx -= step;
            } else {
                needx.push_back(0);
            }

            if (overy >= step) {
                needy.push_back(1);
                overy -= step;
            } else {
                needy.push_back(0);
            }
        }

        for (int si = 1; si <= K; si++) {
            int dx = needx[si - 1] * signx;
            int dy = needy[si - 1] * signy;

            vx += dx;
            vy += dy;
            px += vx;
            py += vy;
            // cerr << px << " " << py << endl;

            if (save)
                path += getMove(dx, dy);
            res++;
        }
    }

    return {res, path};
}

using State = tuple<int, int, int, int, int, int, int>;
using pii = pair<int, int>;
map<pii, map<pii, vector<tuple<int, pii, pii>>>> mem;
map<pair<pii, pii>, int> fastest;

pair<int, string> findPath2(vector<Point> points, bool save=false) {
    string path;
    int vx = 0, vy = 0;
    int px = 0, py = 0;
    int res = 0;

    for (size_t i = 0; i < points.size(); i++) {
        while (px != points[i].x || py != points[i].y) {
            if (i + 1 == points.size()) break;
            int relx1 = points[i].x - px;
            int rely1 = points[i].y - py;
            int relx2 = points[i + 1].x - points[i].x;
            int rely2 = points[i + 1].y - points[i].y;
            if (!mem.count(pii(vx, vy))) break;
            const auto& as = mem.at(pii(vx, vy));
            if (!as.count(pii(relx1, rely1))) break;
            const auto& cand = as.at(pii(relx1, rely1));

            int bd = 1e9;
            pii first_move(-2, -2);
            const pii p2(relx2, rely2);
            for (const auto& [d, sp1, fm] : cand) {
                if (!fastest.count({p2, sp1})) continue;
                int cd = d + fastest[{p2, sp1}];
                if (cd < bd) {
                    bd = cd;
                    first_move = fm;                    
                }
            }
            if (first_move.first == -2) break;

            auto [dx, dy] = first_move;

            vx += dx;
            vy += dy;
            px += vx;
            py += vy;
            // cerr << px << " " << py << endl;

            if (save)
                path += getMove(dx, dy);
            res++;
        }
        if (px == points[i].x && py == points[i].y) continue;
        const auto& point = points[i];
        int K = 1;
        for (K = 1; ; K++) {
            int maxx = px + vx * K + (K * (K + 1) / 2);
            int minx = px + vx * K - (K * (K + 1) / 2);
            int maxy = py + vy * K + (K * (K + 1) / 2);
            int miny = py + vy * K - (K * (K + 1) / 2);

            if (minx <= point.x && point.x <= maxx && miny <= point.y && point.y <= maxy)
                break;
        }
        // cerr << px << "," << py << " (" << vx << "," << vy << "): to " << point.x << "," << point.y << " K=" << K << endl;

        int overx = point.x - px - vx * K;
        int overy = point.y - py - vy * K;
        // cerr << "over: " << overx << " " << overy << endl;
        int signx = 1, signy = 1;

        if (overx < 0) {
            signx = -1;
            overx = -overx;
        }
        if (overy < 0) {
            signy = -1;
            overy = -overy;
        }

        vector<int> needx, needy;
        for (int step = K; step >= 1; step--) {
            if (overx >= step) {
                needx.push_back(1);
                overx -= step;
            } else {
                needx.push_back(0);
            }

            if (overy >= step) {
                needy.push_back(1);
                overy -= step;
            } else {
                needy.push_back(0);
            }
        }

        for (int si = 1; si <= K; si++) {
            int dx = needx[si - 1] * signx;
            int dy = needy[si - 1] * signy;

            vx += dx;
            vy += dy;
            px += vx;
            py += vy;
            // cerr << px << " " << py << endl;

            if (save)
                path += getMove(dx, dy);
            res++;
        }
    }

    return {res, path};
}

void precalc() {
    const int MV = 10;
    const int MT = 6;
    for (int vx = -MV; vx <= MV; vx++) {
        cerr << vx << endl;
        for (int vy = -MV; vy <= MV; vy++) {
            set<State> cur_states;
            map<pii, vector<tuple<int, pii, pii>>> all_states;
            cur_states.insert({0, 0, vx, vy, 0, -2, -2});
            auto push = [&](int px, int py, int vx, int vy, int d, int fx, int fy) {
                tuple<int, int, int, int, int, int, int> key(px, py, vx, vy, d, fx, fy);
                cur_states.insert(key);
            };

            for (int t = 1; t <= MT; t++) {
                auto cs = cur_states;
                cur_states.clear();
                for (const auto& [px, py, vx, vy, d, fx, fy] : cs) {
                    for (int dx = -1; dx <= 1; dx++)
                        for (int dy = -1; dy <= 1; dy++) {
                            if (vx + dx > MV || vx + dx < -MV) continue;
                            if (vy + dy > MV || vy + dy < -MV) continue;
                            push(px + vx + dx, py + vy + dy, vx + dx, vy + dy, d + 1, fx == -2 ? dx : fx, fy == -2 ? dy : fy);
                        }
                }
                for (const auto& state : cur_states) {
                    int d = get<4>(state);
                    pii np{get<0>(state), get<1>(state)};
                    all_states[np].emplace_back(d, pii{get<2>(state), get<3>(state)}, pii{get<5>(state), get<6>(state)});
                    pair<pii, pii> fkey(np, {vx, vy});
                    if (fastest.count(fkey) == 0 || fastest[fkey] > d)
                        fastest[fkey] = d;
                }
            }
            mem[{vx, vy}] = std::move(all_states);
        }
    } 
}

int main(int argc, char* argv[]) {
    // Read the input file with the list of target points
    srand(time(0));
    string filename(argv[1]);
    vector<Point> points = readPoints(filename);
    kmin = stoi(argv[2]);

    precalc();
    cerr << "precalc done\n";

    // Find the path to visit all points
    points = getInitial(points);
    auto [pathlen, path] = findPath2(points, true);
    cerr << "start: " << path.size() << endl;
    ofstream ou2tfile(filename.substr(0, filename.size() - 3) + ".out");
    ou2tfile << path << endl;
    cout << "Path has been saved" << endl;

    double t0 = 5; // path.size() / 100.0;
    double tn = 0.1;
    double sa_start = elapsed();
    double t = t0;
    int SA_STEPS = 100000;
    string bpath = path;
    for (int sa_step = 0; sa_step < SA_STEPS; sa_step++) {
        if (sa_step % 100 == 0) cerr << sa_step << endl;
        int i = rand() % points.size();
        int j = rand() % points.size();
        if (abs(i - j) > 32) continue;
        if (i > j) swap(i, j);
        if (i == j && j + 1 < points.size()) j++;
        for (int qi = i, qj = j; qi < qj; qi++, qj--)
            swap(points[qi], points[qj]);

        auto [clen, _] = findPath2(points, false);
        double t = t0 * pow(tn/t0, sa_step * 1.0 / SA_STEPS);
        double wd = rng.next_double();
        if (clen <= path.size() || wd < exp(((int)path.size() - clen) / t)) {
        // if (clen < path.size()) {
            cerr << "go " << clen << ", t = " << t << " " << path.size() << endl;
            // cerr << wd << " < " << exp(((int)path.size() - clen) / t) << endl;
            // cerr << path.size() - clen << " | " << ((int)path.size() - clen) / t << endl;
            path = findPath2(points, true).second;
            if (path.size() < bpath.size()) {
                bpath = path;
                cerr << "found " << bpath.size() << endl;
                ofstream outfile(filename.substr(0, filename.size() - 3) + ".out");
                outfile << bpath << endl;
                cout << "Better Path has been saved" << endl;
            }
        } else {
            // cerr << "bad\n";
            // cerr << "found " << clen << endl;
            for (int qi = i, qj = j; qi < qj; qi++, qj--)
                swap(points[qi], points[qj]);
        }
    }

    // Output the result
    // ofstream outfile(filename.substr(0, filename.size() - 3) + ".out");
    // outfile << path << endl;
    // cout << "Path has been saved" << endl;
    cerr << "best: " << bpath.size() << endl;

    return 0;
}

