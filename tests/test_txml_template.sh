#!/bin/bash

source resources/utils.sh

TXML_0="resources/template_example_0.xml"
TXML_1="resources/template_example_1.xml"
TXML_2="resources/template_example_2.xml"
TXML_3="resources/template_example_3.xml"
TXML_4="resources/template_example_4.xml"

TXML_0_NAME="template_example_0"
TXML_1_NAME="template_example_1"
TXML_2_NAME="template_example_2"
TXML_3_NAME="template_example_3"
TXML_4_NAME="template_example_4"

### Testing add command

# Adding a template without a name

$APP_BINARY add -p "$TXML_0"

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template add command with out name failed with example 0"
  exit 1
fi

if [ ! -f "$TEMPLATES_DIR/$TXML_0_NAME" ]; then
  echo -e "${FAILED}: TXML Template add command with out name didn't create the template data"
  exit 1
fi

$APP_BINARY add -p "$TXML_1"

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template add command with out name failed with example 1"
  exit 1
fi

if [ ! -f "$TEMPLATES_DIR/$TXML_1_NAME" ]; then
  echo -e "${FAILED}: TXML Template add command with out name didn't create the template data"
  exit 1
fi

$APP_BINARY add -p "$TXML_2"

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template add command with out name failed with example 2"
  exit 1
fi

if [ ! -f "$TEMPLATES_DIR/$TXML_2_NAME" ]; then
  echo -e "${FAILED}: TXML Template add command with out name didn't create the template data"
  exit 1
fi

$APP_BINARY add -p "$TXML_3"

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template add command with out name failed with example 2"
  exit 1
fi

if [ -f "$TEMPLATES_DIR/$TXML_3_NAME" ]; then
  echo -e "${FAILED}: TXML Template add command with out name created the template data with an unexpected name"
  exit 1
fi

echo -e "${SUCCESS}: TXML Template add command with out name passed the tests"

# Adding a template with a name

$APP_BINARY add -p "$TXML_1" -n 1

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template add command with name failed with example 1"
  exit 1
fi

if [ ! -f "$TEMPLATES_DIR/1" ]; then
  echo -e "${FAILED}: TXML Template add command with name didn't create the template data"
  exit 1
fi

echo -e "${SUCCESS}: TXML Template add command with name passed the tests"

### Testing spawn command

# Spawning a template with out a custom name

$APP_BINARY spawn -n "$TXML_1_NAME"

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template spawn command with out custom name failed"
  exit 1
fi

if [[ ! -f "./Cargo.toml" || \
      ! -f "./rustfmt.toml" || \
      ! -f "./.gitignore" || \
      ! -d "./crates/.git" || \
      ! -d "./crates/pepe" || \
      ! -f "./crates/Hola.rs" || \
      ! -f "./crates/crate/sin_titulo.txt"
   ]] ; then
  echo -e "${FAILED}: TXML Template spawn command with out custom name didn't all the expected files and dirs"
  exit 1
fi

echo -e "${SUCCESS}: TXML Template spawn command with out custom name passed the tests"

# Spawning a template with a custom name

$APP_BINARY spawn -n "$TXML_0_NAME" -o 0

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template spawn command with custom name failed"
  exit 1
fi

if [ ! -f "./0.xml" ]; then
  echo -e "${FAILED}: TXML Template spawn command with custom name didn't create the template 0 (Single File)"
  exit 1
fi

$APP_BINARY spawn -n "$TXML_2_NAME" -o 2

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template spawn command with custom name failed"
  exit 1
fi

if [ ! -d "./2" ]; then
  echo -e "${FAILED}: TXML Template spawn command with custom name didn't create the template 2 (Single Dir)"
  exit 1
fi

echo -e "${SUCCESS}: TXML Template spawn command with custom name passed the tests"

### Testing remove command

$APP_BINARY rm -n 0

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template remove command failed"
  exit 1
fi

if [ -f "$TEMPLATES_DIR/0" ]; then
  echo -e "${FAILED}: TXML Template remove command didn't remove the template data"
  exit 1
fi

if [ -d "$TEMPLATES_DIR/0.txml" ]; then
  echo -e "${FAILED}: TXML Template remove command didn't remove the .txml file"
  exit 1
fi

echo -e "${SUCCESS}: TXML Template remove command passed the tests"

### Testing example with Variable Tags

$APP_BINARY add -p "$TXML_4"

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template add command failed with example 4"
  exit 1
fi

if [ ! -f "$TEMPLATES_DIR/$TXML_4_NAME" ]; then
  echo -e "${FAILED}: TXML Template add command didn't create the template data whe using variable tags"
  exit 1
fi
#####
$APP_BINARY spawn -n "$TXML_4_NAME" -o 4

if [ $? -ne 0 ]; then
  echo -e "${FAILED}: TXML Template spawn command failed with example 4"
  exit 1
fi

if [ ! -d "./folder1" ]; then
  echo -e "${FAILED}: TXML Template spawn command didn't create the template 4 folder with variable tags as expected"
  exit 1
fi

if [ ! -f "./folder1/file1.txt" ]; then
  echo -e "${FAILED}: TXML Template spawn command didn't create the template 4 file with variable tags as expected"
  exit 1
fi

EXPECTED_CONTENT=$(cat folder1/file1.txt)

if [ ! "$EXPECTED_CONTENT" = "file1 content" ]; then
  echo -e "${FAILED}: TXML Template spawn command didn't create the template 4 file content with variable tags as expected"
  exit 1
fi

echo -e "${SUCCESS}: TXML Template spawn command with variable tags passed the tests"
