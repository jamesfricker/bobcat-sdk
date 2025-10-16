#!/bin/sh -e

usage() {
	>&2 echo $@
	exit 2
}

project_name="$1"

[ -z "$project_name" ] && usage $(basename $0) "<project/folder name>"

case "$project_name" in
	*_*) usage "_ disallowed from project name. Use hyphens" ;;
	*[[:space:]]*) usage "spaces disallowed from project name" ;;
esac

if [ -d "$project_name" ]; then
	>&2 echo "$project_name already exists! Aborting."
	exit 1
fi

mkdir -p "$project_name/src"

cd "$project_name"

cat >Cargo.toml <<EOF
[package]
name = "$project_name"
version = "0.1.0"
edition = "2024"

[dependencies]
bobcat-sdk = { version = "0.4.2", features = ["panic"] }
EOF

mkdir .cargo

cat >.cargo/config.toml <<EOF
[build]
target = "wasm32-unknown-unknown"
EOF

cat >wasm-post.sh <<EOF
#!/bin/sh -eu

# We make a pass similar to the way the node operates (it does a
# wasm2wat->wat2wasm pass). We also do this for arbos-foundry.

f="\$(mktemp)"

wasm-opt \\
	--dce \\
	--rse \\
	--signature-pruning \\
	--strip-debug \\
	--enable-bulk-memory \\
	--strip \\
	-Oz \\
	"\$1" \\
	-o "\$f.wasm1"

wasm2wat -o \$f.wat \$f.wasm1

wat2wasm -o $project_name.wasm \$f.wat
EOF

chmod +x wasm-post.sh

cat >deploy.sh <<EOF
#!/bin/sh

url=\${ENDPOINT:-https://testnet-rpc.superposition.so}

if [ -z "\$PRIVATE_KEY" ]; then
	>&2 echo "PRIVATE_KEY unset"
	exit 2
fi

cargo stylus deploy \\
	--wasm-file "$project_name.wasm" \\
	--private-key "\$PRIVATE_KEY" \\
	--endpoint "\$url" \\
	--no-verify \\
	        | sed -nr 's/.*deployed code at address: +.*(0x.{40}).*\$/\\1/p'
EOF

chmod +x deploy.sh

cat >Makefile <<EOF

$project_name.wasm: \$(shell find src -type f)
	@cargo build --release
	@./wasm-post.sh \\
		target/wasm32-unknown-unknown/release/$project_name.wasm \\
		$project_name.wasm
EOF

cat >check-codesize.sh <<EOF
#!/bin/sh

size="\$(du "\$1" | cut -f1)"

if [ "\$size" -gt 60 ]; then
	>& echo "\$1 too large"
	exit 1
fi

exit 0
EOF

cat >src/main.rs <<EOF
#![no_main]
#![no_std]

use bobcat_sdk::{cd::const_keccak_sel, entry::read_args_safe};

const SEL_HELLO: [u8; 4] = const_keccak_sel(b"hello()");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { 32 + 4 });
    let sel: [u8; 4] = args[..4].try_into().unwrap();
    match sel {
        SEL_HELLO => 0,
        _ => 1,
    }
}
EOF

cat >.gitignore <<EOF
$project_name.wasm
*.wat
*.wasm1
target
EOF

cat >rust-toolchain.toml <<EOF
[toolchain]
channel = "stable"
components = [ "rust-src" ]
EOF

[ -z "$EDITOR" ] || $EDITOR src/main.rs &
