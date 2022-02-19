#include <bits/stdc++.h>
using namespace std;
struct DSU {
    DSU(int n_) : n(n_ + 1) {
        f.resize(n);
        for (int i = 0; i < n; ++i) f[i] = i;
    };

    int find(int x) { return x == f[x] ? x : (f[x] = find(f[x])); }

    bool test(int x, int y) { return find(x) == find(y); }

    bool connect(int x, int y) {
        x = find(x);
        y = find(y);
        if (x == y) {
            return false;
        } else {
            f[find(x)] = find(y);
            return true;
        }
    }
    int n;
    vector<int> f;
};