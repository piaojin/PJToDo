# !/bin/sh

echo "******Rust RustCoreLib RustCustomScript Start******"

currentPath=`pwd`

echo $currentPath

# cd $currentPath

buildAction () {
    echo "******Building...******"
    sh ${SRCROOT}/RustBuildScript.sh
}

cleanAction () {
    echo "******Cleaning...******"
    sh ${SRCROOT}/RustCargoCleanScript.sh
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

    "build")
        buildAction
        ;;

    "clean")
        cleanAction
        ;;
esac

# exit 0

echo "******Rust RustCoreLib RustCustomScript End******"



