aoc2024-run DAY KIND="":
	terra ./aoc_2024/puzzle{{DAY}}.lua aoc_2024/inputs/input{{DAY}}{{KIND}}.txt

aoc2024-pack:
	cd aoc_2024 && 7zz a inputs.7z inputs/