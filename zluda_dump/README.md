grep -E '^cu.*' log.txt | sed 's/(.*//g' | sort | uniq > uniq_host.txt
cat *.log | grep "^Unrecognized s" | grep -Eo '`([^`]*)`' | sed -E 's/^`((@\w+ )?[^[:space:]]*).*`/\1/' | sort | uniq > uniq_statements.txt
cat *.log | grep "^Unrecognized d" | grep -Eo '`([^`]*)`' | sed -E 's/^`([^`]*)`/\1/' | sort | uniq > uniq_directives.txt