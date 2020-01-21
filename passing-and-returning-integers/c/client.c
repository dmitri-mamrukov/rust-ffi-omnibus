#include <assert.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <limits.h>

typedef struct
{
    uint32_t sum;
    uint32_t err_code;
} tuple_t;

extern tuple_t
    addition(uint32_t, uint32_t);

void assert_addition(uint32_t a, uint32_t b, tuple_t expected_result)
{
    tuple_t result = addition(a, b);

    assert(expected_result.sum == result.sum);
    assert(expected_result.err_code == result.err_code);
}

void test_api()
{
    {
        tuple_t expected_result = {.sum = 0, .err_code = 0};
        assert_addition(0, 0, expected_result);
    }
    {
        tuple_t expected_result = {.sum = 1, .err_code = 0};
        assert_addition(0, 1, expected_result);
    }
    {
        tuple_t expected_result = {.sum = UINT_MAX, .err_code = 0};
        assert_addition(0, UINT_MAX, expected_result);
    }
    {
        tuple_t expected_result = {.sum = 1, .err_code = 0};
        assert_addition(1, 0, expected_result);
    }
    {
        tuple_t expected_result = {.sum = 2, .err_code = 0};
        assert_addition(1, 1, expected_result);
    }
    {
        tuple_t expected_result = {.sum = UINT_MAX, .err_code = 0};
        assert_addition(1, UINT_MAX - 1, expected_result);
    }
    {
        tuple_t expected_result = {.sum = 0, .err_code = 1};
        assert_addition(1, UINT_MAX, expected_result);
    }
}

int main(void)
{
    tuple_t result = addition(1, 2);
    printf("(sum: %" PRIu32 ", err_code: %" PRIu32 ")\n",
           result.sum,
           result.err_code);

    test_api();

    return 0;
}
