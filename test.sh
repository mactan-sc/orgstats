#!/bin/bash

#cd "/home/mactan/Drive/sc_ships/SC_Text/orgs/"

#echo -e "<xml> \n" > test.txt

for i in {1..30}
do

#get the thing
#remove leading and trailing quotes
#run through formatter
curl --no-progress-meter 'https://robertsspaceindustries.com/api/orgs/getOrgs?=null' --compressed -X POST \
                             --header 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0' \
                             --header 'Accept: */*' \
                             --header 'Accept-Language: en-US,en;q=0.5' \
                             --header 'Accept-Encoding: gzip, deflate, br' \
                             --header 'Content-Type: application/json' \
                             --header 'X-Requested-With: XMLHttpRequest' \
                             --header 'Origin: https://robertsspaceindustries.com' \
                             --header 'DNT: 1' \
                             --header 'Connection: keep-alive' \
                             --header 'Referer: https://robertsspaceindustries.com/community/orgs/listing?sort=size_desc&search=&pagesize=12&page=1&' \
                             --header 'Sec-Fetch-Dest: empty' \
                             --header 'Sec-Fetch-Mode: cors' \
                             --header 'Sec-Fetch-Site: same-origin' \
                             --header 'Sec-GPC: 1' \
                             --header 'TE: trailers' \
                             --data '{"sort":"size_desc","search":"","commitment":[],"roleplay":[],"size":[],"model":[],"activity":[],"language":[],"recruiting":[],"pagesize":12,"page":'$i'}' \
                             | jq '.data.html' |  sed  's/^.\(.*\).$/\1/' | xargs -0 printf '%b' >> test.txt

#website gets cranky without sleep
sleep 1

done

#echo -e "\n</xml>" >> test.txt

#test="$(cat test.txt)"

#xmllint --html test.txt  | sed --regexp-extended 's/&.*;//g' | sed '/^[[:space:]]*$/d' | sed "/'img src'/d" > test2.txt

#test=$(echo $test | sed 's/&trade;/ tm/g; s/&ntilde;/nya/g; s/&equiv;/=/g; s/&oacute;/acutie/g; s/&eacute;/ecutie/g; s/&copy;/copywrite/g; s/&Eacute;/eacute/g; s/&iacute;/iacute/g; s/&bull;/bull/g')

#test=$(echo $test | sed --regexp-extended 's/&.*;//g')

#printf "%b " $test | sed 's/\\//g' > test2.txt

filename=`date +"%Y%m%d%H%M".csv`

#get org symbol and member count and format csv
xmllint --html --xpath '*//a//span/span/span[@class="symbol"]/text() | *//span[2]/span[3]/span[2]/text()' test.txt | sed 'N;s/\n/, /;s/$/,/' > $filename

#xmlstarlet sel -t -m "*//a" -v "concat(*//span/span, ', ', *//span[2]/span[3]/span[2], ',')" -n test2.txt  > $filename

#xmllint --html --xpath '*//a//span/span/span[@class="symbol"] | *//span[2]/span[3]/span[2]' 1_original.txt > test4.txt

#rm test.txt
#rm test2.txt

head=$(head -n 1 $filename)
tail=$(tail -n 1 $filename)
linecount=$(wc -l < $filename)

message="recorded org stats \ntop org ${head} \nlast org ${tail} \n${linecount} orgs recorded"

notify-send 'Org Stats' "$message" --icon=dialog-information

#python3 -c 'import html,sys; print(html.unescape(sys.stdin.read()), end="")' < file.html

# echo $(xmlstarlet sel -t -v '//span[@class="symbol"]' -v '//span[span = "Members: "]'  test2.txt)



# #ORG
# echo $(xmlstarlet sel -t -v '//span[@class="left"]//span[@class="symbol"]' test2.txt)

# echo $(xmlstarlet sel -t -v '*//div/a/span[2]/span[2]/span' test2.txt)

# #COUNT
# echo $(xmlstarlet sel -t -v '//span[@class="right"]//span[span = "Members: "]//span[@class="value"]'  test2.txt)

# echo $(xmlstarlet sel -t -v '*//span[2]/span[3]/span[2]' test2.txt)

# #COMBINED
# xmlstarlet sel -t -v '//span[@class="left"]//span[@class="symbol"]' -n -v '//span[@class="right"]//span[span = "Members: "]//span[@class="value"]' test2.txt  > test3.txt 

# xmlstarlet sel -t -m "*//a" -v "concat(*//div/a/span[2]/span[2]/span, ', ', *//span[2]/span[3]/span[2], ',')" -n test2.txt  > test3.txt


# "concat(@id,',',span,',',span)"

# /div[1]/a/span[2]/span[2]/span

# /div[1]/a/span[3]/span[2]/span[3]/span[2]



