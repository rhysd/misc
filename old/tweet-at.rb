#!/usr/bin/env ruby

date, tweet = ARGV

if !date || !text || text == ''
  puts <<~USAGE
    USAGE: tweet-at {datetime} {text}

    (e.g. tweet-at "2018/5/25 22:5:53 +09:00")
  USAGE
  exit 1
end

require 'time'
require 'twitter'

$tw = Twitter::REST::Client.new do |config|
  config.consumer_key = ENV['CONSUMER_KEY']
  config.consumer_secret = ENV['CONSUMER_SECRET']
  config.access_token = ENV['ACCESS_TOKEN']
  config.access_token_secret = ENV['ACCESS_SECRET']
end

puts "Twitter client configured: #{$tw}"

TheTime = Time.parse(date).to_i
Within10Sec = TheTime - 10
Within2Min = TheTime - 120

puts "Datetime configured: #{Time.at(TheTime)}"

def tweet(text, now)
  $tw.update(text)
  puts "Tweeted: #{text} at #{Time.at(now)}"
end

puts "Started at #{Time.now}"

def run()
  loop do
    now = Time.now.to_i
    if now > Within2Min
      loop do
        sleep 1
        now = Time.now.to_i
        if now > Within10Sec
          loop do
            now =  Time.now.to_i
            if now >= TheTime
              tweet(text, now)
              exit 0
            end
            sleep 0.1
          end
        end
        puts "Checked at #{now}"
      end
    end
    puts "Checked at #{Time.at(now)}"
    sleep 30
  end
end

run
