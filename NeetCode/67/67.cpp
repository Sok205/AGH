class MedianFinder {
private:
    vector<int> nums;
public:
    MedianFinder() {}

    void addNum(int num) {
        nums.push_back(num);
    }

    double findMedian() {
        sort(nums.begin(), nums.end());
        int n = nums.size();
        if(n & 1)
        {
            return(nums.at(n / 2));
        }
        else
        {
            return ((nums.at(n / 2) + nums.at(n / 2 - 1))/2.0);
        }
    }
};
