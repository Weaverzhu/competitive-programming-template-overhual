#include <bits/stdc++.h>
using namespace std;

struct SegmentTree {
    const int INF = 1e9;
    SegmentTree(int n) : n(n) {
        nn = 1;
        while (nn < n) nn <<= 1;
        dat = vector<int>(2 * nn + 2, -INF);
        for (int i = nn - 1; i >= 1; --i) pu(i);
    }

    void pu(int rt) { dat[rt] = max(dat[rt << 1], dat[rt << 1 | 1]); }

    int L, R, v;

    void u(int i, int x) {
        i += nn - 1;
        dat[i] = max(dat[i], x);
        while (i > 1) {
            pu(i >> 1);
            i >>= 1;
        }
    }

    int q(int l, int r, int rt) {
        if (L <= l && r <= R) {
            return dat[rt];
        }

        int m = (l + r) >> 1;
        int v = -INF;
        if (L <= m) {
            v = max(v, q(l, m, rt << 1));
        }

        if (m + 1 <= R) {
            v = max(v, q(m + 1, r, rt << 1 | 1));
        }
        return v;
    }

    int q(int l, int r) {
        if (l > r) {
            return -INF;
        }
        L = l;
        R = r;
        return q(1, nn, 1);
    }

    int n, nn;
    vector<int> dat;
};

struct SegmentTreeLazy {
    using T = int;

    int nn;
    vector<T> dat, lazy;
    int L, R;
    T v;

    SegmentTreeLazy(int n, T* a = nullptr) {
        nn = 1;
        while (nn < n) {
            nn <<= 1;
        }
        dat = lazy = vector<T>(0);
        if (a != nullptr) {
            for (int i = 1; i <= n; ++i) {
                dat[i + nn - 1] = a[i];
            }
        }
    }

    void pd(int rt, int l, int r) {
        // TOOD;
    }

    void pu(int rt) {
        // TODO;
    }
    void uu(int l, int r, int rt) {
        if (L <= l && r <= R) {
            // TODO;
        }
        int m = (l + r) >> 1;
        pd(rt, l, r);
        if (L <= m) {
            uu(l, m, rt << 1);
        }
        if (m + 1 <= R) {
            uu(m + 1, r, rt << 1 | 1);
        }
        pu(rt);
    }

    void u(int _l, int _r, T _v) {
        L = _l;
        R = _r;
        v = _v;
    }
};