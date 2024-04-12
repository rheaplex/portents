BLOCK_NAMES="./block-names.csv"
SAT_NAMES="./sat-names.csv"

while read -r line
do
    name=$(echo "$line" | cut -d , -f 2)
    grep "$name" $BLOCK_NAMES
done < "$SAT_NAMES"
