# Brawl to PF Sandbox Exporter [![dependency status](https://deps.rs/repo/github/rukai/brawl_to_pf_sandbox/status.svg)](https://deps.rs/repo/github/rukai/brawl_to_pf_sandbox)

Uses [brawllib_rs](https://github.com/rukai/brawllib_rs) to export brawl fighters to [PF Sandbox](https://pfsandbox.net).

## Steps to export

1.  Install rustup https://rustup.rs/
2.  Right click brawl in dolphin game list -> Properties -> Filesystem -> Disc -> right click Partition 1 -> Extract Files... -> select the directory `data/Brawl`
3.  Copy the entire contents of a brawl mod sd card to the directories `data/MODNAMEHERE` **(optional)**
4.  Open a terminal and `cd` to the directory this readme is in.
5.  Run the command: `cargo run --release -- all`
    or if you setup a mod directory `cargo run --release -- all --mod MODNAMEHERE`
6.  The package has been generated and placed in PF Sandbox's package directory for you.
7.  Open PF Sandbox and you can now select the exported package.
