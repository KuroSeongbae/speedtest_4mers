import java.time.Duration
import java.time.LocalDateTime

fun main(args: Array<String>) {
    println("Start")

    val start_time = LocalDateTime.now()

    var opt = "ACGT"
    var s = ""
    var s_last = ""
    var len_str = 12
    var change_next = true

    for (i in 0..len_str) {
        s += opt[0]
    }

    for (i in 0..len_str){
        s_last +=opt[opt.length - 1]
    }

    var counter = 1
    while (s != s_last){
        counter++
        // You can uncomment the next line to see all k-mers.
        //println("${s}")
        //println("${counter}")
        change_next = true

        for (i in 0..len_str){
            if(change_next){
                if (s[i] == opt[opt.length - 1]){
                    var string_builder = StringBuilder(s)
                    string_builder.setCharAt(i, converter(s[i]))
                    s = string_builder.toString()
                    change_next = true
                }
                else {
                    var string_builder = StringBuilder(s)
                    string_builder.setCharAt(i, converter(s[i]))
                    s = string_builder.toString()
                    break
                }
            }
        }
    }
    val end_time = LocalDateTime.now()
    println("Number of generated k-mers: ${counter}")
    println("Finish! Needed ${Duration.between(start_time, end_time).toSeconds()}")
}

fun converter(c: Char): Char {
    if (c == 'A') return 'C'
    if (c == 'C') return 'G'
    if (c == 'G') return 'T'
    if (c == 'T') return 'A'
    return ' '
}