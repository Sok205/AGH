class Solution {
public:
    int leastInterval(vector<char>& tasks, int n) {
        vector<int> hashAlph(26,0);

        for(auto task : tasks)
        {
            hashAlph.at(task - 'A')++;
        }

        sort(hashAlph.begin(), hashAlph.end());
        int maxFrequency = hashAlph.at(25);
        int idle = (maxFrequency - 1)*n;
        for(int i = 24; i >= 0; i--){
            idle -= min(maxFrequency - 1, hashAlph.at(i));
        }
        return max(0, idle) + tasks.size();
    }
};
