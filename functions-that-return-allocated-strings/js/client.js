const assert = require("assert");
const ffi = require("ffi");
const ref = require("ref-napi");

const lib = ffi.Library("libapi", {
    generate_theme_song: ["char *", [ref.types.uint8]],
    free_theme_song: [ref.types.void, ["char *"]]
});

function generateThemeSong(len) {
    const song = lib.generate_theme_song(len);
    try {
        return song.readCString();
    } finally {
        lib.free_theme_song(song);
    }
}

const result = generateThemeSong(5);
console.log(result);
assert("💣 na na na na na Batman! 💣" === result);

testApi();

function testApi() {
    function assertGenerateThemeSong(len, expectedResult) {
        const result = generateThemeSong(len);

        assert(expectedResult === result);
    }

    assertGenerateThemeSong(0, "💣 Batman! 💣");
    assertGenerateThemeSong(1, "💣 na Batman! 💣");
    assertGenerateThemeSong(2, "💣 na na Batman! 💣");
}
