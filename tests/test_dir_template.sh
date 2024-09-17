#!/bin/bash

source resources/utils.sh

mkdir test_dir

### Testing add command

# Adding a template without a name

$APP_BINARY add -p test_dir -as-dir

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: Dir Template add command without name failed"
  exit 1
fi

if [ ! -f "$TEMPLATES_DIR/test_dir" ]; then
  echo -e "${FAILED}: Dir Template add command without name didn't create the template data"
  exit 1
fi

if [ ! -d "$TEMPLATES_DIR/test_dir.dir" ]; then
  echo -e "${FAILED}: Dir Template add command without name didn't create the .dir directory"
  exit 1
fi

echo -e "${SUCCESS}: Dir Template add command without name passed the tests"

# Adding a template with a name

$APP_BINARY add -p test_dir -n test_dir_named -as-dir

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: Dir Template add command with name failed"
  exit 1
fi

if [ ! -f "$TEMPLATES_DIR/test_dir_named" ]; then
  echo -e "${FAILED}: Dir Template add command with name didn't create the template data"
  exit 1
fi

if [ ! -d "$TEMPLATES_DIR/test_dir_named.dir" ]; then
  echo -e "${FAILED}: Dir Template add command with name didn't create the .dir directory"
  exit 1
fi

echo -e "${SUCCESS}: Dir Template add command with name passed the tests"

### Testing spawn command

# Spawning a template with a custom name

$APP_BINARY spawn -n test_dir

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: Dir Template spawn command with out custom name failed"
  exit 1
fi

if [ ! -d "./test_dir" ]; then
  echo -e "${FAILED}: Dir Template spawn command with out custom name didn't create the directory"
  exit 1
fi

echo -e "${SUCCESS}: Dir Template spawn command with out custom name passed the tests"

# Spawning a template with a custom name

$APP_BINARY spawn -n test_dir -o test_dir_custom

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: Dir Template spawn command with custom name failed"
  exit 1
fi

if [ ! -d "./test_dir_custom" ]; then
  echo -e "${FAILED}: Dir Template spawn command with custom name didn't create the directory"
  exit 1
fi

echo -e "${SUCCESS}: Dir Template spawn command with custom name passed the tests"

### Testing remove command

$APP_BINARY rm -n test_dir

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: Dir Template remove command failed"
  exit 1
fi

if [ -f "$TEMPLATES_DIR/test_dir" ]; then
  echo -e "${FAILED}: Dir Template remove command didn't remove the template data"
  exit 1
fi

if [ -d "$TEMPLATES_DIR/test_dir.dir" ]; then
  echo -e "${FAILED}: Dir Template remove command didn't remove the .dir directory"
  exit 1
fi

echo -e "${SUCCESS}: Dir Template remove command passed the tests"
