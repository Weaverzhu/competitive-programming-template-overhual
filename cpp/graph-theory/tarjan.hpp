#include <bits/stdc++.h>
using namespace std;

struct Tarjan {
    Tarjan(int n) : n(n) {
        bl = vector<int>(n + 1, 0);
        g = vector<vector<int>>(n + 1);
        in = vector<bool>(n + 1, false);
        c = 0;
    }

    void dfs(int u) {
        dfn[u] = low[u] = ++c;
        tmp.push_back(u);
        in[u] = true;
        for (int v : g[u]) {
            if (dfn[v] == 0) {
                dfs(v);
            }
            if (in[v]) low[u] = min(low[u], low[v]);
        }
        if (low[u] == dfn[u]) {
            for (int x : tmp) {
                bl[x] = scc.size();
            }
            scc.push_back(tmp);
            tmp.clear();
        }
    }

    void add(int u, int v) { g[u].push_back(v); }

    int n, c;

    vector<vector<int>> scc;
    vector<int> tmp;
    vector<int> bl, dfn, low;
    vector<bool> in;
    vector<vector<int>> g;
};