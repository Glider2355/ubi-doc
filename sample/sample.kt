// 1行目のコメント
fun main() {
    // 2行目のコメント
    /* ブロックコメント */
    
    println("Hello, World!")

    // Sampleクラスのインスタンス生成・呼び出し例
    val sample = Sample("Alice", 30)
    sample.sayHello()

    // Sample2クラスのインスタンス生成・呼び出し例
    val sample2 = Sample2("Bob", 25)
    sample2.sayHello2()
}

/**
 * タグ無しのコメント
 * @ubiquitous ubiquitous langage kt
 * @context context kt
 * @description description kt
 */
class Sample(
    var name: String,
    var age: Int
) {
    fun sayHello() {
        println("Hello, $name!")
    }
}

/**
 * タグ無しのコメント
 * @ubiquitous ubiquitous langage kt
 * @context context kt
 * @description description kt
 */
class Sample2(
    var name: String,
    var age: Int
) {
    fun sayHello2() {
        println("Hello, $name!")
    }
}
