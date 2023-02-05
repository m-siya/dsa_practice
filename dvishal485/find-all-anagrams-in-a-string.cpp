#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    vector<int> findAnagrams(string s, string p) {
        vector<int> idx;
        if (p.size() > s.size()) return idx;
        vector<int> p_letters(26, 0), s_letters(26, 0);

        for (auto &c : p) p_letters[c - 'a']++;
        for (int i = 0; i < p.size(); i++) s_letters[s[i] - 'a']++;
        if (s_letters == p_letters) idx.push_back(0);

        for (int i = p.size(); i < s.size(); i++) {
            s_letters[s[i - p.size()] - 'a']--;
            s_letters[s[i] - 'a']++;
            if (s_letters == p_letters)
                idx.push_back(i - p.size() + 1);
        }
        return idx;
    }
};
