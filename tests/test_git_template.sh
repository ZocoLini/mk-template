#!/bin/bash

source resources/utils.sh

GIT_REPO_ONLINE="https://github.com/ZocoLini/mk-template.git"

### Testing add command

# Adding a template without a name

$APP_BINARY add -p "$GIT_REPO_ONLINE"

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: Git Template add command without name failed"
  exit 1
fi

if [ ! -f "$TEMPLATES_DIR/mk-template" ]; then
  echo -e "${FAILED}: Git Template add command without name didn't create the template data"
  exit 1
fi

echo -e "${SUCCESS}: Git Template add command without name passed the tests"

# Adding a template with a name

$APP_BINARY add -p "$GIT_REPO_ONLINE" -n git_named

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: Git Template add command with name failed"
  exit 1
fi

if [ ! -f "$TEMPLATES_DIR/git_named" ]; then
  echo -e "${FAILED}: Git Template add command with name didn't create the template data"
  exit 1
fi

echo -e "${SUCCESS}: Git Template add command with name passed the tests"

### Testing spawn command

# Spawning a template with a custom name

$APP_BINARY spawn -n mk-template

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: Git Template spawn command with out custom name failed"
  exit 1
fi

if [ ! -d "./mk-template" ]; then
  echo -e "${FAILED}: Git Template spawn command with out custom name didn't create the directory"
  exit 1
fi

echo -e "${SUCCESS}: Git Template spawn command with out custom name passed the tests"

# Spawning a template with a custom name

$APP_BINARY spawn -n mk-template -o mk-template_custom

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: Git Template spawn command with custom name failed"
  exit 1
fi

if [ ! -d "./mk-template_custom" ]; then
  echo -e "${FAILED}: Git Template spawn command with custom name didn't create the directory"
  exit 1
fi

echo -e "${SUCCESS}: Git Template spawn command with custom name passed the tests"

### Testing remove command

$APP_BINARY rm -n test_dir

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: Git Template remove command failed"
  exit 1
fi

if [ -f "$TEMPLATES_DIR/test_dir" ]; then
  echo -e "${FAILED}: Git Template remove command didn't remove the template data"
  exit 1
fi

if [ -d "$TEMPLATES_DIR/test_dir.dir" ]; then
  echo -e "${FAILED}: Git Template remove command didn't remove the .dir directory"
  exit 1
fi

echo -e "${SUCCESS}: Git Template remove command passed the tests"
