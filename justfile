today:
    cargo run --quiet --release --features today -- today

scaffold *args:
    cargo run --quiet --release -- scaffold {{args}}

download *args:
    cargo run --quiet --release -- download {{args}}

read:
    cargo run --quiet --release -- read

solve *args:
    cargo run --quiet --release -- solve {{args}}

all:
    cargo run --quiet --release -- all

time:
    cargo run --quiet --release -- time
