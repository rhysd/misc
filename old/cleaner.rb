#!/usr/bin/env ruby -rfileutils

puts "Cleaning cdr history..."
file = "#{ENV['HOME']}/.chpwd-recent-dirs"
File.write(
  file,
  File::foreach(file).select{|l| l =~ /^\$'(.*)'/ && Dir.exists?($1) }.join
)

puts "Cleaning undo history..."
Dir["#{ENV['HOME']}/.vim/undo/*"]
  .reject{|f| File.exists? File.basename(f).gsub('%', '/')}
  .each{|f| FileUtils.remove_file f}
