const assert = require("assert");
const ffi = require("ffi");
const ref = require("ref-napi");
const struct = require("ref-struct-napi");

const Tuple = struct({
    x: ref.types.uint32,
    y: ref.types.uint32
});

const lib = ffi.Library("libapi", {
    flip_things_around: [Tuple, [Tuple]]
});

const result = lib.flip_things_around(new Tuple({ x: 1, y: 2 }));
console.log(result);
assert(3 === result.x);
assert(0 === result.y);

testApi();

function testApi() {
    const UINT_MAX = 4294967295;

    function assertFlipThingsAround(tuple, expectedResult) {
        const result = lib.flip_things_around(tuple);

        assert(expectedResult.x === result.x);
        assert(expectedResult.y === result.y);
    }

    assertFlipThingsAround(
        new Tuple({ x: 0, y: 0 }),
        new Tuple({ x: 1, y: UINT_MAX })
    );
    assertFlipThingsAround(
        new Tuple({ x: 0, y: 2 }),
        new Tuple({ x: 3, y: UINT_MAX })
    );
    assertFlipThingsAround(
        new Tuple({ x: 0, y: UINT_MAX }),
        new Tuple({ x: 0, y: UINT_MAX })
    );
    assertFlipThingsAround(
        new Tuple({ x: 1, y: UINT_MAX }),
        new Tuple({ x: 0, y: 0 })
    );
    assertFlipThingsAround(
        new Tuple({ x: 1, y: 2 }),
        new Tuple({ x: 3, y: 0 })
    );
    assertFlipThingsAround(
        new Tuple({ x: 1, y: UINT_MAX }),
        new Tuple({ x: 0, y: 0 })
    );
    assertFlipThingsAround(
        new Tuple({ x: UINT_MAX, y: 0 }),
        new Tuple({ x: 1, y: UINT_MAX - 1 })
    );
    assertFlipThingsAround(
        new Tuple({ x: UINT_MAX, y: 2 }),
        new Tuple({ x: 3, y: UINT_MAX - 1 })
    );
    assertFlipThingsAround(
        new Tuple({ x: UINT_MAX, y: UINT_MAX }),
        new Tuple({ x: 0, y: UINT_MAX - 1 })
    );
}
