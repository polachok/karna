{
  description = "Dev environment for AArch64 kernel in Rust (QEMU virt)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "llvm-tools" "rustfmt" "clippy" ];
          targets = [ "aarch64-unknown-none" "aarch64-unknown-none-softfloat" ];
        };

        crossBinutils = pkgs.pkgsCross.aarch64-embedded.buildPackages.binutils;
      in
      {
        devShells.default = pkgs.mkShell {
          name = "aarch64-kernel";

          packages = with pkgs; [
            rustToolchain
            cargo-binutils

            crossBinutils # aarch64-none-elf-{ld,objcopy,objdump,readelf,size}
            qemu          # provides qemu-system-aarch64
            dtc           # device tree compiler, for inspecting QEMU's generated dtb
            gdb           # talks to QEMU's gdbstub over TCP, no ptrace needed

            just
          ];

          shellHook = ''
            echo "aarch64-kernel dev shell"
            echo "  $(rustc --version)"
            echo "  targets: aarch64-unknown-none, aarch64-unknown-none-softfloat"
            echo "  $(qemu-system-aarch64 --version | head -n1)"
            echo "  linker:  $(aarch64-none-elf-ld --version | head -n1)"
            echo
            echo "Run a kernel (GICv2 pinned explicitly):"
            echo "  qemu-system-aarch64 -M virt,gic-version=2 -cpu cortex-a72 -m 512M \\"
            echo "    -nographic -kernel target/aarch64-unknown-none/debug/<kernel>"
            echo
            echo "Dump this QEMU version's device tree:"
            echo "  qemu-system-aarch64 -M virt,gic-version=2 -machine dumpdtb=virt.dtb"
            echo "  dtc -I dtb -O dts virt.dtb | less"
            echo
            echo "Debug (add -S -s to the qemu invocation, then in another shell):"
            echo "  gdb-multiarch -ex 'target remote :1234' target/aarch64-unknown-none/debug/<kernel>"
          '';
        };
      });
}
