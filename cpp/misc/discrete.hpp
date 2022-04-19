#include <bits/stdc++.h>
using namespace std;

struct Discrete {
    using T = int;

    void add(T t) { disc.push_back(t); }

    void finalize() {
        sort(disc.begin(), disc.end());
        disc.resize(unique(disc.begin(), disc.end()) - disc.begin());
    }

    int rank(T t) {
        return lower_bound(disc.begin(), disc.end(), t) - disc.begin() + 1;
    }

    vector<T> disc;
};