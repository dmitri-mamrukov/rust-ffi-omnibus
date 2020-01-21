const assert = require("assert");
const ffi = require("ffi");
const ref = require("ref-napi");

const lib = ffi.Library("libapi", {
    how_many_characters: [ref.types.int32, [ref.types.CString]]
});

const result = lib.how_many_characters("rust");
console.log(result);
assert(4 === result);

testApi();

function testApi() {
    function assertHowManyCharacters(text, expectedResult) {
        const result = lib.how_many_characters(text);

        assert(expectedResult === result);
    }

    assertHowManyCharacters(null, -1);
    assertHowManyCharacters("", 0);
    assertHowManyCharacters("rust", 4);
}
