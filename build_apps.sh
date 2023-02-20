cd src/
apps_dirs=`ls -d */`

for i in $apps_dirs
do
    echo $i
    cd $i
    cargo build --release
    cp target/release/libcalendar.* ../../apps
    cd ..
done