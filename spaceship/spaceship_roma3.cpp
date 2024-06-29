#include <iostream>
#include <ctime>
#include <set>
#include <vector>
#include <fstream>
#include <string>
#include <cmath>

using namespace std;

struct Point {
    int x, y;
};

long long distance(int x1, int y1, int x2, int y2) {
    return (long long)(x2 - x1) * (x2 - x1) + (long long)(y2 - y1) * (y2 - y1);
}

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
            if (ss.size() > 4) ss.erase(--ss.end());
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

using SPoint = pair<Point, bool>;

pair<int, string> findPath(vector<SPoint> points, bool save=false) {
    string path;
    int vx = 0, vy = 0;
    int px = 0, py = 0;
    int res = 0;

    for (const auto& [point, ex] : points) {
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
        if (ex) K++;

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

int main(int argc, char* argv[]) {
    // Read the input file with the list of target points
    srand(time(0));
    string filename(argv[1]);
    vector<Point> points = readPoints(filename);

    // Find the path to visit all points
    points = getInitial(points);
    vector<SPoint> spts;
    for (const auto& p : points)
        spts.push_back(SPoint{p, false});
    auto [pathlen, path] = findPath(spts, true);
    cerr << "start: " << path.size() << endl;

    while (true) {
        int i = rand() % spts.size();
        int j = rand() % spts.size();
        if (abs(i - j) > 16) continue;
        if (i > j) swap(i, j);
        int r = rand() % 2;
        if (r == 0) {
            for (int qi = i, qj = j; qi < qj; qi++, qj--)
                swap(spts[qi], spts[qj]);
        } else {
            spts[i].second ^= 1; // !spts[i].second;
        }

        auto [clen, _] = findPath(spts, false);
        if (clen < path.size()) {
            path = findPath(spts, true).second;
            cerr << "found " << path.size() << endl;
            ofstream outfile(filename.substr(0, filename.size() - 3) + ".out");
            outfile << path << endl;
            cout << "Path has been saved" << endl;
        } else {
            // cerr << "bad\n";
            // cerr << "found " << clen << endl;
            if (r == 0) {
                for (int qi = i, qj = j; qi < qj; qi++, qj--)
                    swap(spts[qi], spts[qj]);
            } else {
                spts[i].second ^= 1;
            }
        }
    }

    // Output the result
    ofstream outfile(filename.substr(0, filename.size() - 3) + ".out");
    outfile << path << endl;
    cout << "Path has been saved" << endl;

    return 0;
}

