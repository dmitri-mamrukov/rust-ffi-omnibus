const assert = require("assert");
const ffi = require("ffi");
const ref = require("ref-napi");
const array = require("ref-array");

const U32array = array(ref.types.uint32);

const lib = ffi.Library("libapi", {
    sum_of_even: [ref.types.int32, [U32array, ref.types.size_t]]
});

const numbers = new U32array([0, 1, 2, 3, 4, 5, 6]);
const result = lib.sum_of_even(numbers, numbers.length);
console.log(result);
assert(12 === result);

testApi();

function testApi() {
    function assertSumOfEven(values, expectedResult) {
        const numbers = new U32array(values);

        const result = lib.sum_of_even(numbers, numbers.length);

        assert(expectedResult === result);
    }

    assertSumOfEven(null, -1);
    assertSumOfEven([], 0);
    assertSumOfEven([0], 0);
    assertSumOfEven([1], 0);
    assertSumOfEven([2], 2);
    assertSumOfEven([0, 1, 2, 3, 4, 5, 6], 12);
}
