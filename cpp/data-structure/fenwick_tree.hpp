#include <bits/stdc++.h>
using namespace std;

struct FenwickTree {
    using T = int;
    FenwickTree(int n) { dat = vector<T>(n + 1, 0); }

    static inline int lowbit(int x) { return x & (-x); }

    void add(int i, T x) {
        for (; i < dat.size(); i += lowbit(i)) {
            dat[i] += x;
        }
    }

    T sum(int i) {
        T res = 0;
        for (; i; i -= lowbit(i)) {
            res += i;
        }
        return res;
    }

    vector<T> dat;
};