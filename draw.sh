if ["$1"] 
then 
    file=image
else
    file=$1
fi

# rm -f $file.ppm

# cargo run --release

convert ./images/$file.ppm ./images/$file.png

cmd.exe /C start ./images/$file.png