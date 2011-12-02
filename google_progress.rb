#!/usr/bin/env ruby
# -*- coding: utf-8 -*-

class GoogleProgress

    def initialize m
        @max = m
        @tic = 0
    end

    def +@
        @tic += 1
        return if @tic > @max
        draw_progress_bar
        puts if @tic == @max
    end

private
    Default = "\e[0m"
    Blue = "\e[34m"
    Red = "\e[31m"
    Green = "\e[32m"
    Yellow = "\e[33m"

    def google count
        ooo = "o" * count
        "#{Blue}G#{Red}o#{Yellow}o#{ooo}#{Blue}g#{Green}l#{Red}e#{Default}"
    end

    def draw_progress_bar
        oo = @tic*20/@max
        print "\r#{google(oo)} [#{Float(@tic*100)/@max}]"
    end
end

#
# main
#
if __FILE__ == $0 then
    google_progress = GoogleProgress.new 100
    100.times do 
        +google_progress
        sleep 0.1
    end
end

