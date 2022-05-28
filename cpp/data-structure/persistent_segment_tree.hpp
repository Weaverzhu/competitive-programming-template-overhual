#include <bits/stdc++.h>
using namespace std;

// likely to MLE if the testcases are strong enough
// code style NG yet

namespace persistent_segment_tree {
typedef long long ll;
const int N = 2e5 + 7;
int n, m, stamp;
ll t[N * 80], lz[N * 80];
int ls[N * 80], rs[N * 80];
int rt[N], tot;
ll a[N];
void bd(int& now, int l, int r) {
    now = ++tot;
    lz[now] = 0;
    if (l == r) {
        t[now] = a[l];
        return;
    }
    int m = (l + r) >> 1;
    bd(ls[now], l, m);
    bd(rs[now], m + 1, r);
    t[now] = t[ls[now]] + t[rs[now]];
}
void upd(int& now, int pre, int l, int r, int L, int R, ll v) {
    now = ++tot;
    t[now] = t[pre];
    lz[now] = lz[pre];
    ls[now] = ls[pre];
    rs[now] = rs[pre];
    t[now] += (min(r, R) - max(l, L) + 1) * v;
    if (L <= l && r <= R) {
        lz[now] += v;
        return;
    }
    int m = (l + r) >> 1;
    if (L <= m) upd(ls[now], ls[pre], l, m, L, R, v);
    if (m < R) upd(rs[now], rs[pre], m + 1, r, L, R, v);
}
ll que(int now, int l, int r, int L, int R) {
    if (L <= l && r <= R) return t[now];
    ll res = (min(r, R) - max(L, l) + 1) * lz[now];
    int m = (l + r) >> 1;
    if (L <= m) res += que(ls[now], l, m, L, R);
    if (m < R) res += que(rs[now], m + 1, r, L, R);
    return res;
}
// init n -> bd(rt[0], 1, n)
// update l, r, x -> upd(now, pre, l, r, 1, n, x);
// query l, r -> query(now, l, r, 1, n);
}  // namespace persistent_segment_tree