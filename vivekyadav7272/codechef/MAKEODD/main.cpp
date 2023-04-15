#include <iostream>
#include <fstream>
#include <numeric>
#include <cstdint>

using namespace std;

// Worse case is 10^5 elements.
// Worse case size = 32 bits * 10^5 = 390KB, that's huge for an array!
// VLAs will cause problem hence static array is declared rather than at runtime depending on size.
static uint64_t arr[100000];

bool iseven(uint64_t n) {
    return (n & 1) == 0;
}

// void solution(uint32_t n) {
//     // uint64_t acc = 0;
//     // for (uint32_t i = 0; i < n; i++) {
//     //     if (iseven(arr[i])) {
//     //         acc++;
//     //     }
//     //     arr[i] = acc;
//     // }

//     uint64_t sum_subarrays_neg = 0;
//     uint64_t sum_subarrays_pos = 0;

//     int64_t coeff = -(((int64_t)n)-2);
//     for (uint32_t i = 0; i < (n-2)/2; i++, coeff += 2) {
//         sum_subarrays_neg += (uint64_t)(-coeff) * (uint64_t)arr[i];
//     }

//     for (uint32_t i = (n-2)/2; i < n; i++, coeff += 2) {
//         sum_subarrays_pos += coeff * arr[i];
//     }

//     cout << sum_subarrays_pos - sum_subarrays_neg << '\n';
// }

void solution(uint32_t n) {
    // Already given an accumulated array.
    using i128 = __int128_t;
    i128 subarr_sum = 0;
    for (int64_t i = 0, coeff = -(int64_t)(n-2); i < n; i++, coeff += 2) {
        subarr_sum += ((i128)coeff) * arr[i];
    }

    cout << (uint64_t)subarr_sum << '\n';
}

#define input f

int main() {
    ifstream f;
    f.open("input.txt", ios::in);
    uint16_t test_cases;
    input >> test_cases;
    while (test_cases--) {
        uint32_t n;
        input >> n;

        uint64_t acc = 0;
        for (uint32_t i = 0; i < n; i++) {
            uint32_t temp;
            input >> temp;
            if (iseven(temp)) {
                acc++;
            }
            arr[i] = acc;
        }


        solution(n);
    }
}