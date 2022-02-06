#include <bits/stdc++.h>
using namespace std;
using LL = long long;
struct Prime {
    int n;
    vector<int> minp;
    vector<int> prime;
    Prime(int n) : n(n) {
        minp.resize(n, 0);
        prime.clear();
        prime.reserve(n);
        for (int i = 2; i < n; ++i) {
            if (minp[i] == 0) {
                minp[i] = i;
                prime.push_back(i);
            }
            for (int j = 0; j < (int)prime.size(); ++j) {
                LL np = 1LL * i * prime[j];
                if (np >= n) {
                    break;
                }
                if (!minp[np]) {
                    minp[np] = prime[j];
                }
                if (i % prime[j] == 0) {
                    break;
                }
            }
        }
    }

    vector<int> pfac(int x) {
        vector<int> ret;
        while (x > 1) {
            int mp = minp[x];
            while (x % mp == 0) {
                ret.push_back(mp);
                x /= mp;
            }
        }
        return ret;
    }

    vector<pair<int, int>> pfac2(int x) {
        vector<pair<int, int>> ret;
        while (x > 1) {
            int mp = minp[x];
            ret.emplace_back(mp, 0);
            while (x % mp == 0) {
                ++ret.back().second;
                x /= mp;
            }
        }
        return ret;
    }
};
