#include <bits/stdc++.h>

using namespace std;

struct Tree {
    int n, r, nm = 0;
    vector<vector<int>> g;
    vector<int> d, f;

    vector<vector<int>> df;

    struct M2 {
        int m1 = 0, m2 = 0;

        void update(int m) {
            if (m > m1) {
                m2 = m1;
                m1 = m;
            } else if (m > m2) {
                m2 = m;
            }
        }
    };

    vector<M2> m2;

    void dfsm2(int u) {
        m2[u].update(d[u]);
        for (int v : g[u]) {
            if (v != f[u]) {
                dfsm2(v);
                m2[u].update(m2[v].m1);
            }
        }
    }

    Tree(int n, int root = 0) : n(n), r(root) {
        g.resize(n, vector<int>());
        d = f = vector<int>(n, 0);
        read();
        d[r] = -1;
        dfs(r, r);

        nm = 20;
        df.resize(n, vector<int>(nm));

        for (int i = 0; i < n; ++i) {
            df[i][0] = f[i];
        }
        for (int i = 1; i < nm; ++i) {
            for (int j = 0; j < n; ++j) {
                df[j][i] = df[df[j][i - 1]][i - 1];
            }
        }

        m2.resize(n, M2());
        dfsm2(0);
    }

    void dfs(int u, int fa) {
        d[u] = d[fa] + 1;
        f[u] = fa;
        for (int v : g[u]) {
            if (v == fa) {
                continue;
            }
            dfs(v, u);
        }
    }

    void read() {
        for (int i = 0; i < n - 1; ++i) {
            int u, v;
            scanf("%d%d", &u, &v);
            --u;
            --v;
            g[u].push_back(v);
            g[v].push_back(u);
        }
    }

    int lca(int u, int v) const {
        if (d[u] < d[v]) {
            swap(u, v);
        }
        int y = d[u] - d[v];
        for (int i = nm - 1; i >= 0; --i) {
            if (y >= (1 << i)) {
                y -= 1 << i;
                u = df[u][i];
            }
        }
        if (u == v) {
            return u;
        }
        for (int i = nm - 1; i >= 0; --i) {
            if (df[u][i] != df[v][i]) {
                u = df[u][i];
                v = df[v][i];
            }
        }
        return df[u][0];
    }
};