#include<iostream>
#include<vector>
using namespace std;

int maxArea(vector<int>& height) {
    int left = 0;
    int right = height.size() - 1;
    int area = 0;
    
    while(left < right){
        int lineHeight = height[left] < height[right] ? height[left] : height[right];
        int tempArea = lineHeight * (right - left);
        if(area < tempArea) area = tempArea;
        if(height[left] < height[right]){
            left++;
        }else{
            right--;
        }
    }

    return area;
}

int main() {

    vector<int> height = {1,8,6,2,5,4,8,3,7};

    int result = maxArea(height);

    cout << "Area: " << result << endl;

    return 0;
}