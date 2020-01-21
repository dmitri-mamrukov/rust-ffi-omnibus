#include <assert.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

extern int32_t
sum_of_even(const uint32_t *numbers, size_t length);

void assert_sum_of_even(const uint32_t *numbers, size_t length, int32_t expected_result)
{
    int32_t result = sum_of_even(numbers, length);

    assert(expected_result == result);
}

void test_api()
{
    {
        assert_sum_of_even(NULL, 0, -1);
    }
    {
        uint32_t numbers[] = {};
        assert_sum_of_even(numbers, 0, 0);
    }
    {
        uint32_t numbers[] = {0};
        assert_sum_of_even(numbers, 1, 0);
    }
    {
        uint32_t numbers[] = {1};
        assert_sum_of_even(numbers, 1, 0);
    }
    {
        uint32_t numbers[] = {2};
        assert_sum_of_even(numbers, 1, 2);
    }
    {
        uint32_t numbers[] = {0, 1, 2, 3, 4, 5, 6};
        assert_sum_of_even(numbers, 7, 12);
    }
}

int main(void)
{
    uint32_t numbers[] = {0, 1, 2, 3, 4, 5, 6};
    size_t length = sizeof numbers / sizeof *numbers;
    int32_t result = sum_of_even(numbers, length);
    printf("result: %" PRIi32 "\n", result);

    test_api();

    return 0;
}
