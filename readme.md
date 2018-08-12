# Brawl to PF Sandbox Exporter [![dependency status](https://deps.rs/repo/github/rukai/brawl_to_pf_sandbox/status.svg)](https://deps.rs/repo/github/rukai/brawl_to_pf_sandbox)

Uses [brawllib_rs](https://github.com/rukai/brawllib_rs) to export brawl fighters to [PF Sandbox](https://pfsandbox.net).

## Steps to export

1.  install rustup https://rustup.rs/
2.  right click brawl in dolphin game list -> Properties -> Filesystem -> Disc -> Partition 1 -> right click fighter -> Extract Files... -> select the directory data/brawl/fighter
3.  copy a mod fighter directory to the directory data/MODNAMEHERE/fighter **(optional)**
4.  open a terminal and `cd` to the directory this readme is in.
5.  run the command: `cargo run --release -- all`
    or if you setup a mod directory `cargo run --release -- all --mod MODNAMEHERE`
6.  This will take a long time to complete.
7.  The package has been generated and placed in PF Sandbox's package directory for you.
8.  Open PF Sandbox and you can now select the exported package.
