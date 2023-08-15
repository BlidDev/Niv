#!/bin/bash
echo "enter extension"
read old_ext

echo "new extension"
read new_ext

for file in *.$old_ext
do
    mv -v "$file" "${file%.$old_ext}.$new_ext"
done;
