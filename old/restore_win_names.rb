#!/usr/bin/env ruby
# encoding: utf-8

require 'pathname'

Dir.glob('*').each do |p|
  path = Pathname.new p
  next if path.directory?

  if p =~ /¥([^¥]+)$/
    path.rename $1
  end # if

end # do
