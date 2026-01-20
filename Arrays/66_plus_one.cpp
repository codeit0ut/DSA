#include<iostream>
#include<vector>
#include<cmath>
using namespace std;

vector<int> plusOne(vector<int>& digits){
    int n = digits.size();

    for(int i = n - 1; i >= 0; i--){
        if(digits[i] < 9){
            digits[i] += 1;
            return digits;
        }
        else{
            digits[i] = 0;
            if(i == 0){
                digits.insert(digits.begin(), 1);
                return digits;
            }  
        }
    }

    return {0};         // never runs
}

int main(){

    vector<int> digits = {9,9,9};

    vector<int> result = plusOne(digits);

    for(int x : result){
        cout << x << " ";
    }
    cout << endl;

    return 0;
}