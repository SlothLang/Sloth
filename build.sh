# Build Sloth
cargo build
FILENAME="$1"
# Compile standard library
./target/debug/sloth std/extern.sloth std/stdio.sloth std/stdmath.sloth $FILENAME

# Generate binary
clang -lm output.o std/stdio.c std/stdlib.c std/stdmath.c -o "${FILENAME%.sloth}"

# Move file
mv "${FILENAME%.sloth}" .
