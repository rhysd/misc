trait Ord {
  def <(that: Any): Boolean
  def <=(that: Any): Boolean = (this < that) || (this == that)
  def >(that: Any): Boolean = !(this <= that)
  def >=(that: Any): Boolean = !(this < that)
}

class Date(y: Int, m: Int, d: Int) extends Ord {
  def year = y
  def month = m
  def day = d
  override def toString() = "" + year + "-" + month + "-" + day
  override def equals(that: Any): Boolean =
    that.isInstanceOf[Date] && {
      val o = that.asInstanceOf[Date]
      o.day == day && o.month == month && o.year == year
    }
  def <(that: Any): Boolean = {
    if (!that.isInstanceOf[Date])
      sys.error("cannot compare " + that + " and a Date instance")

    val o = that.asInstanceOf[Date]
    (year < o.year) || (year == o.year) && (month < o.month || month == o.month && (day < o.day))
  }
}

object Trait {
  def main(args: Array[String]): Unit = {
    val d1 = new Date(2021, 8, 22)
    val d2 = new Date(2021, 7, 21)
    println("" + d1 + " < " + d2 + " ? " + (d1 < d2))
    println("" + d1 + " > " + d2 + " ? " + (d1 > d2))
  }
}
