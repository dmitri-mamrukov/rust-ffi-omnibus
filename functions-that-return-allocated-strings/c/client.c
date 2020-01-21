#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

extern char *
generate_theme_song(uint8_t length);

extern void
free_theme_song(char *);

void assert_generate_theme_song(uint8_t length, const char *expected_result)
{
    char *result = generate_theme_song(length);

    assert(0 == strcmp(expected_result, result));

    free_theme_song(result);
}

void test_api()
{
    assert_generate_theme_song(0, "💣 Batman! 💣");
    assert_generate_theme_song(1, "💣 na Batman! 💣");
    assert_generate_theme_song(2, "💣 na na Batman! 💣");
}

int main(void)
{
    char *song = generate_theme_song(5);
    printf("%s\n", song);
    free_theme_song(song);

    test_api();

    return 0;
}
