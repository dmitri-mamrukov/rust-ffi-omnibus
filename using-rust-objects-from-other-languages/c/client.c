#include <assert.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

typedef struct zip_code_database zip_code_database_t;

extern zip_code_database_t *
zip_code_database_new(void);

extern void
zip_code_database_free(zip_code_database_t *);

extern void
zip_code_database_populate(zip_code_database_t *);

extern uint32_t
zip_code_database_population_of(
    const zip_code_database_t *db,
    const char *zip_code);

void test_api()
{
    {
        zip_code_database_t *database = zip_code_database_new();
        zip_code_database_populate(database);
        zip_code_database_free(database);

        assert(NULL != database);
    }
    {
        zip_code_database_t *database = zip_code_database_new();
        zip_code_database_populate(database);

        uint32_t result = zip_code_database_population_of(database, "00000");

        assert(0 == result);

        result = zip_code_database_population_of(database, "90210");

        assert(90210 == result);

        result = zip_code_database_population_of(database, "20500");

        assert(20500 == result);

        result = zip_code_database_population_of(database, "99999");

        assert(99999 == result);

        result = zip_code_database_population_of(
            database,
            "non-existing-zip-code");

        assert(0 == result);

        zip_code_database_free(database);
    }
}

int main(void)
{
    zip_code_database_t *database = zip_code_database_new();
    zip_code_database_populate(database);

    uint32_t pop1 = zip_code_database_population_of(database, "90210");
    uint32_t pop2 = zip_code_database_population_of(database, "20500");

    zip_code_database_free(database);

    printf("pop1: %" PRId32 ", pop2: %" PRId32 "\n",
           (int32_t)pop1,
           (int32_t)pop2);

    test_api();

    return 0;
}
