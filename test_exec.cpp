#include <execution>
#include <vector>
#include <algorithm>
int main() {
    std::vector<int> v(10);
    std::for_each(std::execution::par_unseq, v.begin(), v.end(), [](int& x){ x = 1; });
    return 0;
}
