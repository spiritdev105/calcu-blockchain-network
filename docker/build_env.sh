#! /usr/bin/env bash

echo "Building calcu env image"

usage() {
    echo "Usage:"
		echo "    $0 -h                      Display this help message."
		echo "    $0 [options]"
    echo "Options:"
    echo "     -p Publish image"

	  exit 1;
}

PUBLISH=0

while getopts ":hp" opt; do
    case ${opt} in
        h )
			      usage
            ;;
        p )
            PUBLISH=1
            ;;
        \? )
            echo "Invalid options: -$OPTARG" 1>&2
            exit 1
            ;;
    esac
done

TOOLCHAIN_VER="`cat docker/rust-toolchain`"
IMAGEID="calcuio/calcu-env:${TOOLCHAIN_VER}"

docker build docker/calcu-env --build-arg TOOLCHAIN="${TOOLCHAIN_VER}" -t $IMAGEID

if [ $? -eq "0" ]; then
    echo "Done with building calcu env image, tag: $IMAGEID"
else
    echo "Failed on building calcu env image."
    exit 1
fi

echo "Build success."
if [ "$PUBLISH" -eq "1" ]; then
    echo "Publishing image to $IMAGEID"
    docker push $IMAGEID
fi

