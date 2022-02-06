#include <bits/stdc++.h>
using namespace std;
// 1-indexed
struct Mo {
    int l, r, n, B;
    using P = pair<int, int>;

    struct Q {
        P p;
        int id;
        bool ans;
    };

    struct Context {
        int& n;
        vector<int>& a;
        vector<vector<P>>& p;
        Context(int& n, vector<int>& a, vector<vector<P>>& p)
            : n(n), a(a), p(p) {}
    };

    Context& c;

    vector<Q> q;
    // function<void(int)> cob;
    Mo(int n, Context& c) : n(n), c(c) {
        B = int(ceil(pow(n, 0.5)));
        mp.resize(1000001, 0);
        cnt = 0;
    }

    void init() {
        sort(q.begin(), q.end(), [this](const Q& qa, const Q& qb) {
            const P &a = qa.p, &b = qb.p;
            int ba = a.first / B, bb = b.first / B;
            if (ba != bb) {
                return ba < bb;
            }
            if (ba & 1)
                return a.second > b.second;
            else
                return a.second < b.second;
        });
    }

    void go() {
        l = 1;
        r = 0;

        for (auto& q : this->q) {
            const auto& p = q.p;
            while (r < p.second) {
                cob(1);
            }
            while (l > p.first) {
                cob(0);
            }
            while (l < p.first) {
                cob(2);
            }
            while (r > p.second) {
                cob(3);
            }
            q.ans = query();
        }
    }

    vector<int> mp;
    int cnt;
    bool query() { return cnt == 0; }

    void cob(int opt) {
        auto dec = [this](const vector<P>& p) {
            for (const auto& x : p) {
                int o = mp[x.first];
                mp[x.first] = ((mp[x.first] - x.second) % 3 + 3) % 3;
                cnt += (mp[x.first] != 0) - (o != 0);
            }
        };

        auto inc = [this](const vector<P>& p) {
            for (const auto& x : p) {
                int o = mp[x.first];
                mp[x.first] = ((mp[x.first] + x.second) % 3 + 3) % 3;
                cnt += (mp[x.first] != 0) - (o != 0);
            }
        };

        if (opt == 0) {
            inc(c.p[--l]);
        } else if (opt == 1) {
            inc(c.p[++r]);
        } else if (opt == 2) {
            dec(c.p[l++]);
        } else {
            dec(c.p[r--]);
        }
    }

    void addQ(Q q) { this->q.push_back(q); }
};
