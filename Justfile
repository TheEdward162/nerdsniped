aoc2024-run DAY:
	terra ./aoc_2024/puzzle{{DAY}}.lua aoc_2024/inputs/input{{DAY}}.txt

aoc2024-pack:
	cd aoc_2024 && 7zz a inputs.7z inputs/