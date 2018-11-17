# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    export PKG_CONFIG_ALLOW_CROSS=1
    cross rustc --bin nk_tool --target $TARGET --release -- -C lto
    cross rustc --bin nk_game --target $TARGET --release -- -C lto

    cp target/$TARGET/release/nk_tool $stage/
    cp target/$TARGET/release/nk_game $stage/
    cp -r assets/ $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
