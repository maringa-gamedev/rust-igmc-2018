# This script takes care of testing your crate

set -ex

main() {
    export PKG_CONFIG_ALLOW_CROSS=1

    cross build --bin nk_tool --target $TARGET
    cross build --bin nk_tool --target $TARGET --release

    cross build --bin nk_game --target $TARGET
    cross build --bin nk_game --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --bin nk_tool --target $TARGET
    cross test --bin nk_tool --target $TARGET --release

    cross test --bin nk_game --target $TARGET
    cross test --bin nk_game --target $TARGET --release

    cross run --bin nk_tool --target $TARGET
    cross run --bin nk_tool --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
