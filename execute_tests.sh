#!/bin/bash

TEMPLATES_DIR="$HOME/.mkt-dev";
TEST_DIR="tests/";
DIR_TEMPLATE_TEST="test_dir_template.sh";
GIT_TEMPLATE_TEST="test_git_template.sh";
TXML_TEMPLATE_TEST="test_txml_template.sh";
RESOURCES_DIR="resources";

reset_app_dir()
{
  rm -rf "$TEMPLATES_DIR";

  for archivo in ./*; do
    if [[ "$archivo" == "./$DIR_TEMPLATE_TEST" || \
          "$archivo" == "./$GIT_TEMPLATE_TEST" || \
          "$archivo" == "./$TXML_TEMPLATE_TEST" || \
          "$archivo" == "./$RESOURCES_DIR" ]]; then
        continue # Don't remove files needed for testing
    fi

    rm -r "$archivo";
  done
}

if [ ! -d "$TEST_DIR" ]; then
  printf "tests dir does not exist"
  exit 1;
fi

# cargo test
cargo test

cargo build -p bin_app
cp target/debug/bin_app tests/mkt_dev

cd tests || exit

### Execute Dir Template test

if ! bash "$DIR_TEMPLATE_TEST"; then
  echo "=> FAILED: Directory templates didn't pass the tests"
else
  echo "=> SUCCESS: Directory templates passed the tests"
fi

reset_app_dir;

### Execute Git Template test

if ! bash "$GIT_TEMPLATE_TEST"; then
  echo "=> FAILED: Git templates didn't pass the tests"
else
  echo "=> SUCCESS: Git templates passed the tests"
fi

reset_app_dir;

### Execute Txml Template test

if ! bash "$TXML_TEMPLATE_TEST"; then
  echo "=> FAILED: Txml templates didn't pass the tests"
else
  echo "=> SUCCESS: Txml templates passed the tests"
fi

reset_app_dir;