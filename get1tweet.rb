#!/usr/bin/env ruby
# -*- coding: utf-8 -*-

require 'rubygems'
require 'twitter'

TmpFilePath = "/.timeline"

def set_timeline page = 1
    Twitter::configure do |config|
        config.consumer_key = 'your_consumer_key'
        config.consumer_secret = 'your_consumer_secret'
        config.oauth_token = 'your_auth_token'
        config.oauth_token_secret = 'your_auth_secret'
    end

    begin
        File.open(File.expand_path("~")+TmpFilePath, mode="w") do  |file|
            file.puts page
            Twitter.home_timeline(:page => page).each do |t|
                file.puts "@" + t.user.screen_name + ": " + t.text
            end
        end
    rescue 
        exit if page == 1
        page = 1
        retry
    end
end

#
# main
#
if __FILE__ == $0 then

    if ARGV[0] == "init" then
        set_timeline
        exit
    end

    exit unless File.exist?(File.expand_path("~")+TmpFilePath)

    tweets = ""
    file_path = File.expand_path("~")+TmpFilePath
    File.open(file_path, mode="r") do |file|
        tweets = file.read.split "\n"
    end
    page = tweets[0].to_i

    if tweets.size == 1 then
        set_timeline page+1
        File.open(file_path, mode="r") do |file|
            tweets = file.read.split "\n"
        end
    end

    puts tweets[1]

    File.open(file_path, mode="w") do |file|
        tweets.each_with_index do |tweet,i|
            if i != 1
                file.puts tweet
            end
        end
    end

end

