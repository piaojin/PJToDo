# !/bin/sh

echo "******Rust RustCoreLib RustCustomScript Start******"

buildAction () {
    echo "******Building...******"
    sh RustBuildScript.sh
}

cleanAction () {
    echo "******Cleaning...******"
    sh RustCargoCleanScript.sh
}

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# MAIN

echo "Running with ACTION=${ACTION}"

case $ACTION in
    # NOTE: for some reason, it gets set to "" rather than "build" when
    # doing a build.
    "")
        buildAction
        ;;

    "clean")
        cleanAction
        ;;
esac

# exit 0

echo "******Rust RustCoreLib RustCustomScript End******"



