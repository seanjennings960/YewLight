set -x
SOURCE=dist/*
DEST=sean@beaglebone.local:/var/www/weblights

scp $SOURCE $DEST
