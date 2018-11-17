# This script takes care of testing your crate

set -ex

main() {
    cross build --bin tool --target $TARGET
    cross build --bin tool --target $TARGET --release

    cross build --bin game --target $TARGET
    cross build --bin game --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --bin tool --target $TARGET
    cross test --bin tool --target $TARGET --release

    cross test --bin game --target $TARGET
    cross test --bin game --target $TARGET --release

    cross run --bin tool --target $TARGET
    cross run --bin tool --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
