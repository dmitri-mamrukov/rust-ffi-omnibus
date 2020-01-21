const assert = require("assert");
const ffi = require("ffi");

const lib = ffi.Library("libapi", {
    zip_code_database_new: ["pointer", []],
    zip_code_database_free: ["void", ["pointer"]],
    zip_code_database_populate: ["void", ["pointer"]],
    zip_code_database_population_of: ["uint32", ["pointer", "string"]]
});

const ZipCodeDatabase = function() {
    this.ptr = lib.zip_code_database_new();
};

ZipCodeDatabase.prototype.free = function() {
    lib.zip_code_database_free(this.ptr);
};

ZipCodeDatabase.prototype.populate = function() {
    lib.zip_code_database_populate(this.ptr);
};

ZipCodeDatabase.prototype.populationOf = function(zip) {
    return lib.zip_code_database_population_of(this.ptr, zip);
};

const database = new ZipCodeDatabase();
try {
    database.populate();
    const pop1 = database.populationOf("90210");
    const pop2 = database.populationOf("20500");
    console.log(`pop1: ${pop1}, pop2: ${pop2}`);
    assert(90210 === pop1);
    assert(20500 === pop2);
} finally {
    database.free();
}

testApi();

function testApi() {
    {
        const database = new ZipCodeDatabase();
        try {
            database.populate();
        } finally {
            database.free();
        }
    }
    {
        const database = new ZipCodeDatabase();
        try {
            database.populate();

            let result = database.populationOf("00000");

            assert(0 === result);

            result = database.populationOf("90210");

            assert(90210 === result);

            result = database.populationOf("20500");

            assert(20500 === result);

            result = database.populationOf("99999");

            assert(99999 === result);

            result = database.populationOf("non-existing-zip-code");

            assert(0 === result);
        } finally {
            database.free();
        }
    }
}
