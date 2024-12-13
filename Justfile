aoc2022-run DAY KIND="":
	cd aoc_2022 && cargo run --bin puzzle{{DAY}} -- inputs/input{{DAY}}{{KIND}}.txt

aoc2024-run DAY KIND="":
	terra ./aoc_2024/puzzle{{DAY}}.lua aoc_2024/inputs/input{{DAY}}{{KIND}}.txt

aoc-pack YEAR:
	cd aoc_{{YEAR}} && 7zz -p a inputs.7z inputs/
