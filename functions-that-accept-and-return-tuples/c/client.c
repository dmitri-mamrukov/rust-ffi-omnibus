#include <assert.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <limits.h>

typedef struct
{
    uint32_t x;
    uint32_t y;
} tuple_t;

extern tuple_t
    flip_things_around(tuple_t);

void assert_flip_things_around(tuple_t tuple, tuple_t expected_result)
{
    tuple_t result = flip_things_around(tuple);

    assert(expected_result.x == result.x);
    assert(expected_result.y == result.y);
}

void test_api()
{
    {
        tuple_t tuple = {.x = 0, .y = 0};
        tuple_t expected_result = {.x = 1, .y = UINT_MAX};
        assert_flip_things_around(tuple, expected_result);
    }
    {
        tuple_t tuple = {.x = 0, .y = 2};
        tuple_t expected_result = {.x = 3, .y = UINT_MAX};
        assert_flip_things_around(tuple, expected_result);
    }
    {
        tuple_t tuple = {.x = 0, .y = UINT_MAX};
        tuple_t expected_result = {.x = 0, .y = UINT_MAX};
        assert_flip_things_around(tuple, expected_result);
    }
    {
        tuple_t tuple = {.x = 1, .y = UINT_MAX};
        tuple_t expected_result = {.x = 0, .y = 0};
        assert_flip_things_around(tuple, expected_result);
    }
    {
        tuple_t tuple = {.x = 1, .y = 2};
        tuple_t expected_result = {.x = 3, .y = 0};
        assert_flip_things_around(tuple, expected_result);
    }
    {
        tuple_t tuple = {.x = 1, .y = UINT_MAX};
        tuple_t expected_result = {.x = 0, .y = 0};
        assert_flip_things_around(tuple, expected_result);
    }
    {
        tuple_t tuple = {.x = UINT_MAX, .y = 0};
        tuple_t expected_result = {.x = 1, .y = UINT_MAX - 1};
        assert_flip_things_around(tuple, expected_result);
    }
    {
        tuple_t tuple = {.x = UINT_MAX, .y = 2};
        tuple_t expected_result = {.x = 3, .y = UINT_MAX - 1};
        assert_flip_things_around(tuple, expected_result);
    }
    {
        tuple_t tuple = {.x = UINT_MAX, .y = UINT_MAX};
        tuple_t expected_result = {.x = 0, .y = UINT_MAX - 1};
        assert_flip_things_around(tuple, expected_result);
    }
}

int main(void)
{
    tuple_t tuple = {.x = 1, .y = 2};
    tuple_t result = flip_things_around(tuple);
    printf("(x: %" PRIu32 ", y: %" PRIu32 ")\n",
           result.x,
           result.y);

    test_api();

    return 0;
}
