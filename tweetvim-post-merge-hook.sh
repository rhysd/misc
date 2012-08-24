# /usr/bin/zsh

set -e

HOOK_DIR=`dirname $0`
ROOT_DIR="$HOOK_DIR/../.."
TWEETVIM="$ROOT_DIR/autoload/tweetvim.vim"
TWEETVIM_TMP="$ROOT_DIR/autoload/tweetvim.vim.new"

CONSUMER_KEY="your consumer key"
CONSUMER_SECRET="your consumer secret key"

sed "s/8hht6fAi3wU47cwql0Cbkg/$CONSUMER_KEY/" $TWEETVIM | \
  sed "s/sbmqcNqlfwpBPk8QYdjwlaj0PIZFlbEXvSxxNrJDcAU/$CONSUMER_SECRET/" > $TWEETVIM_TMP

mv $TWEETVIM $ROOT_DIR/tweetvim.vim.backup
mv $TWEETVIM_TMP $TWEETVIM

# vim: ft=sh
