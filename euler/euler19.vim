function! s:is_leap_year(year)
    return a:year % 4 == 0 && !(a:year % 100 == 0 && a:year % 400) ? 1 : 0
endfunction

function! s:count_20th_century_mondays()
    let l:day_count = 366 " 1900年の365日分 + 1
    let l:monday = 0
                " 1  2  3  4  5  6  7  8  9 10 11 12
    let month = [31,28,31,30,31,30,31,31,30,31,30,31]
    for y in range(1901,2000)
        let month[1] = s:is_leap_year(y) ?  29 : 28
        for m in month
            if l:day_count % 7 == 0
                let l:monday = l:monday + 1
            endif
            let l:day_count += m
        endfor
    endfor

    return l:monday

endfunction

echo s:count_20th_century_mondays()
" 171
