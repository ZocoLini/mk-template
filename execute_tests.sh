#!/bin/bash

source tests/resources/utils.sh

TEMPLATES_DIR="$HOME/.mkt-dev";
TEST_DIR="tests/";
DIR_TEMPLATE_TEST="test_dir_template.sh";
GIT_TEMPLATE_TEST="test_git_template.sh";
TXML_TEMPLATE_TEST="test_txml_template.sh";
BINARY_APP="mkt_dev";
RESOURCES_DIR="resources";

reset_app_dir()
{
  rm -rf "$TEMPLATES_DIR";

  for archivo in ./* ./.*; do
    if [[ "$archivo" == "./." || "$archivo" == "./.." || \
          "$archivo" == "./$DIR_TEMPLATE_TEST" || \
          "$archivo" == "./$GIT_TEMPLATE_TEST" || \
          "$archivo" == "./$TXML_TEMPLATE_TEST" || \
          "$archivo" == "./$BINARY_APP" || \
          "$archivo" == "./$RESOURCES_DIR" ]]; then
        continue # Don't remove files needed for testing
    fi

    rm -rf "$archivo";
  done
}

if [ ! -d "$TEST_DIR" ]; then
  printf "tests dir does not exist"
  exit 1;
fi

# cargo test
cargo test

cargo build -p bin_app
cp "target/debug/bin_app" "tests/$BINARY_APP"

cd tests || exit

### Execute Dir Template test

if ! bash "$DIR_TEMPLATE_TEST"; then
  echo -e "=> ${FAILED}: Directory templates didn't pass the tests"
else
  echo -e "=> ${SUCCESS}: Directory templates passed the tests"
fi

reset_app_dir;

### Execute Git Template test

if ! bash "$GIT_TEMPLATE_TEST"; then
  echo -e "=> ${FAILED}: Git templates didn't pass the tests"
else
  echo -e "=> ${SUCCESS}: Git templates passed the tests"
fi

reset_app_dir;

### Execute TXML Template test

if ! bash "$TXML_TEMPLATE_TEST"; then
  echo -e "=> ${FAILED}: TXML templates didn't pass the tests"
else
  echo -e "=> ${SUCCESS}: TXML templates passed the tests"
fi

reset_app_dir;

rm "$BINARY_APP"