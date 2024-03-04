#!/bin/sh

cd '1brc'
if [ ! -d target ]; then
	./mvnw clean verify
fi
./create_measurements.sh $1
hyperfine --runs 5 --output "../data/measurements_$1.ref.out.txt" --export-csv "../data/measurements_$1.time.csv" --command-name baseline 'java --class-path target/average-1.0.0-SNAPSHOT.jar dev.morling.onebrc.CalculateAverage_baseline'
mv measurements.txt "../data/measurements_$1.txt"
