#!/usr/bin/env ruby
# encoding: utf-8

unless ARGV.size == 2
  puts "Usage: #{$0} {old-gem-command} {new-gem-command}"
  exit
end # unless ARGV.size == 2

old_gem = ARGV[0]
new_gem = ARGV[1]

system("#{new_gem} install " + `#{old_gem} list --no-details --no-versions`.gsub(/\n/, ' '))

