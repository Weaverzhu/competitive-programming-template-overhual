#include <bits/stdc++.h>
using namespace std;
using LL = long long;

struct Dijkstra {
    int n;
    using T = LL;
    vector<T> d;
    vector<vector<pair<int, T>>> g;
    Dijkstra(int n) : n(n) { g = vector<vector<pair<int, T>>>(n + 1); }

    void add(int u, int v, T w) {
        // printf("%d %d %d\n", u, v, w);
        g[u].emplace_back(v, w);
    }

    void doit(int s) {
        d = vector<T>(n + 1, 1e18);
        d[s] = 0;
        using PT = pair<T, int>;
        priority_queue<PT, vector<PT>, greater<PT>> q;
        q.emplace(0, s);
        while (!q.empty()) {
            auto uu = q.top();
            int u = uu.second;
            q.pop();

            if (uu.first > d[u]) {
                continue;
            }
            for (const auto& e : g[u]) {
                int v = e.first;
                T w = e.second;
                if (d[v] > d[u] + w) {
                    d[v] = d[u] + w;
                    q.emplace(d[v], v);
                }
            }
        }
    }
};