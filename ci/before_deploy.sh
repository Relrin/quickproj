# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          artefact=quickproj \
          stage= \

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    if [ $TARGET = x86_64-pc-windows-gnu ]; then
        artefact=quickproj.exe
    fi

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin quickproj --target $TARGET --release -- -C lto
    cp target/$TARGET/release/$artefact $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main