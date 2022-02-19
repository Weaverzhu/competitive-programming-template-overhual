#include <bits/stdc++.h>
using namespace std;

struct EulerGraph {
    struct E {
        int eid, u, v;
    };

    vector<vector<E>> g;
    int n, m;
    vector<bool> vise;

    EulerGraph(int n, int m) : n(n), m(m), vise(vector<bool>(m + 1, false)) {
        g.resize(n + 1);
    }

    void addEdge(int u, int v, int eid = -1) {
        static int cnt = 0;
        if (eid == -1) {
            eid = cnt++;
        }
        g[u].push_back(E{.eid = eid, .u = u, .v = v});
        g[v].push_back(E{.eid = eid, .u = v, .v = u});
    }

    void dfs(int u) {
        for (const auto& e : g[u]) {
            int v = e.v, eid = e.eid;
            if (vise[eid]) {
                continue;
            }
            vise[eid] = true;
            dfs(v);
        }
        // cautions here for the euler path + circle scenario
        path.push_back(u);
    }

    vector<int> path;
    bool suc = false;

    bool go(int u = -1) {
        int vs = 0;
        for (int i = 0; i <= n; ++i) {
            vs += g[i].size() & 1;
        }
        if (vs != 0 && vs != 2) {
            return suc = false;
        }

        if (u == -1) {
            for (int i = 0; i <= n; ++i) {
                if (g[i].size() & 1) {
                    u = i;
                    break;
                }
            }
            if (u == -1) {
                for (int i = 0; i <= n; ++i) {
                    if (!g[i].empty()) {
                        u = i;
                        break;
                    }
                }
            }
        }
        // sort the edges to get the lexicographically smallest euler path
        for (int i = 0; i <= n; ++i) {
            sort(g[i].begin(), g[i].end(),
                 [](const E& a, const E& b) { return a.v < b.v; });
        }

        dfs(u);
        reverse(path.begin(), path.end());
        suc = true;
        return suc;
    }
};
