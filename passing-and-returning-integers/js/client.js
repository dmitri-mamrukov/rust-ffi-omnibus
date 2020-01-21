const assert = require("assert");
const ffi = require("ffi");
const ref = require("ref-napi");
const struct = require("ref-struct-napi");

const Tuple = struct({
    sum: ref.types.uint32,
    errCode: ref.types.uint32
});

const lib = ffi.Library("libapi", {
    addition: [Tuple, [ref.types.uint32, ref.types.uint32]]
});

const result = lib.addition(1, 2);
console.log(result);
assert(3 === result.sum);
assert(0 === result.errCode);

testApi();

function testApi() {
    const UINT_MAX = 4294967295;

    function assertAddition(a, b, expectedResult) {
        const result = lib.addition(a, b);

        assert(expectedResult.sum === result.sum);
        assert(expectedResult.errCode === result.errCode);
    }

    assertAddition(0, 0, { sum: 0, errCode: 0 });
    assertAddition(0, 1, { sum: 1, errCode: 0 });
    assertAddition(0, UINT_MAX, { sum: UINT_MAX, errCode: 0 });
    assertAddition(1, 0, { sum: 1, errCode: 0 });
    assertAddition(1, 1, { sum: 2, errCode: 0 });
    assertAddition(1, UINT_MAX - 1, { sum: UINT_MAX, errCode: 0 });
    assertAddition(1, UINT_MAX, { sum: 0, errCode: 1 });
}
