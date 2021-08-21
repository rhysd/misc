class Reference[T] {
  private var contents: T = _
  def set(value: T): Unit = {
    contents = value
  }
  def get = contents
}

object Generics {
  def main(args: Array[String]): Unit = {
    val cell = new Reference[Int]
    cell.set(13)
    println("contents = " + cell.get)

    val cell2 = new Reference[String]
    cell2.set("hello")
    println("contents = " + cell2.get)
  }
}
