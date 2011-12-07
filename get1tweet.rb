#!/usr/bin/env ruby
# -*- coding: utf-8 -*-

TmpFilePath = "/.timeline"
MyScreenName = "your_screen_name"

def config_twitter

    require 'rubygems' if RUBY_VERSION.to_f < 1.9
    require 'twitter'

    Twitter::configure do |config|
        config.consumer_key = 'your_consumer_key'
        config.consumer_secret = 'your_consumer_secret'
        config.oauth_token = 'your_oauth_token'
        config.oauth_token_secret = 'your_oauth_secret'
    end
end

def filtering? tweet
    return false if tweet.user.screen_name==MyScreenName || tweet.text.include?(MyScreenName)

    bl_users = %w[ user1 user2 user3 ]
    bl_regexs = [ /(?:w|ｗ){4,}/, /RT @#{tweet.user.screen_name}:/, /^\s*RT @/ ]

    return true if bl_users.include? tweet.user.screen_name

    bl_regexs.each do |regex|
        return true if tweet.text =~ regex
    end

    false
end

def set_timeline page = 1

    print "getting home timeline(page: #{page})... "

    config_twitter

    file_path = File.expand_path("~")+TmpFilePath

    begin
        File.open(file_path, mode="w") do  |file|
            file.puts page
            Twitter.home_timeline(:page => page).each do |t|
                next if filtering? t
                text = t.text.include?("\n") ? t.text.split("\n").join(" ") : t.text
                if text.include? MyScreenName
                    text = "\e[1;32m#{text}\e[0m"
                end
                begin
                    file.puts "\e[1;36m@" + t.user.screen_name + ":\e[0m " \
                        + text \
                        + " \e[33m[" + Time.parse(t.created_at).strftime("%m/%d %H:%M") + "]\e[0m"
                rescue
                    next
                end
            end
        end
    rescue => error
        if page == 1 then
            File.delete file_path
            p error
            exit
        end
        page = 1
        retry
    end

    puts "done."
end

def tweet status

    print "updating status... "
    config_twitter

    begin
        Twitter.update status if status.length <= 140
    rescue => error
        p error
    end

    puts "done."
end

#
# main
#
if __FILE__ == $0 then

    case ARGV[0]
    when "init" then
        set_timeline
        exit
    when "update" then
        tweet ARGV[1]
        exit
    end

    file_path = File.expand_path("~")+TmpFilePath
    exit unless File.exist? file_path

    tweets = ""
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

    puts
    puts tweets[1]

    File.open(file_path, mode="w") do |file|
        tweets.each_with_index do |tweet,i|
            file.puts tweet unless i == 1
        end
    end

end

