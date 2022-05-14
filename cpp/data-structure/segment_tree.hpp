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
    using T = long long;

    int nn;
    vector<T> dat, lazy;
    int L, R;
    T v;

    SegmentTreeLazy(int n, T* a = nullptr) {
        nn = 1;
        while (nn < n) {
            nn <<= 1;
        }
        dat = lazy = vector<T>(nn * 2 + 2, 0);
        if (a != nullptr) {
            for (int i = 1; i <= n; ++i) {
                dat[i + nn - 1] = a[i];
            }
            for (int i = nn - 1; i >= 1; --i) {
                pu(i);
            }
        }
    }

    void pd(int rt, int l, int r) {
        // TOOD;
        if (lazy[rt]) {
            int ls = rt << 1, rs = rt << 1 | 1;
            int len = r - l + 1, sl = len / 2;
            dat[ls] = lazy[rt] * sl;
            dat[rs] = lazy[rt] * sl;
            lazy[ls] = lazy[rt];
            lazy[rs] = lazy[rt];
            lazy[rt] = 0;
        }
    }

    void pu(int rt) {
        // TODO;
        dat[rt] = dat[rt << 1] + dat[rt << 1 | 1];
    }
    void uu(int l, int r, int rt) {
        if (L <= l && r <= R) {
            dat[rt] = (r - l + 1) * v;
            lazy[rt] = v;
            return;
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
        uu(1, nn, 1);
    }
    T q(int l, int r, int rt) {
        if (L <= l && r <= R) {
            return dat[rt];
        }
        pd(rt, l, r);
        int m = (l + r) >> 1;
        T ret = 0;
        if (L <= m) {
            ret += q(l, m, rt << 1);
        }
        if (m + 1 <= R) {
            ret += q(m + 1, r, rt << 1 | 1);
        }
        pu(rt);
        return ret;
    }

    T q(int l, int r) {
        L = l;
        R = r;
        return q(1, nn, 1);
    }
};
}
;