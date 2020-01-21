#include <assert.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

extern int32_t
    how_many_characters(const char *text);

void assert_how_many_characters(const char *text, int32_t expected_result)
{
    int32_t result = how_many_characters(text);

    assert(expected_result == result);
}

void test_api()
{
    assert_how_many_characters(NULL, -1);
    assert_how_many_characters("", 0);
    assert_how_many_characters("rust", 4);
}

int main(void)
{
    int32_t result = how_many_characters("rust");
    printf("count: %" PRIi32 "\n", result);

    test_api();

    return 0;
}
